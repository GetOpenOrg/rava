//! 反射元数据表端到端（二进制体积 B1，`docs/plans/2026-10-04-binary-size.md`）：
//! - 类级注解往返：类文件的 RuntimeVisibleAnnotations 原始字节与其稀疏常量池 → 发射的 CLASS_ANNO 字节流 →
//!   按运行时解码字形（`java_runtime::meta_codec::class_anno`）解回，与类文件逐字节一致。注解持有类只作类字面量
//!  （L1 不透明类），覆盖数组元素值与嵌套注解数组；
//! - 成员表裁剪口径：反射调用调用者敏感方法（`Method.invoke(Field.get)`）读私有静态字段时，字段声明类有字段表，
//!   运行期注解解析（判调用者敏感读方法注解、元注解定保留策略、动态代理取接口方法）涉及的注解类型有方法表。
//! 找不到 JDK 21 时跳过。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};

use classfile::extras::{parse_extras, AnnoConst};

static RAVA: Mutex<()> = Mutex::new(());

/// 见 `closure_cli.rs` 同名函数
fn manifest_dir() -> PathBuf {
    std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}

/// 一次 rava 子命令（输入源文件居首）；缺 JDK → None
fn rava(sub: &str, java: &Path, args: &[&str]) -> Option<String> {
    let _guard = RAVA.lock().unwrap_or_else(|e| e.into_inner());
    let o = Command::new(env!("CARGO_BIN_EXE_rava"))
        .arg(sub)
        .arg(java)
        .args(["--jdk", "21", "--runtime"])
        .arg(manifest_dir().join("../../../runtime/java_runtime"))
        .args(args)
        .output()
        .expect("启动 rava");
    let stderr = String::from_utf8_lossy(&o.stderr).to_string();
    if !o.status.success() {
        if stderr.contains("未找到 JDK") {
            eprintln!("[meta_reflect_cli] 跳过：无 JDK 21");
            return None;
        }
        panic!("rava {sub} 失败：\n{stderr}");
    }
    Some(stderr + &String::from_utf8_lossy(&o.stdout))
}

fn s(p: &Path) -> String {
    p.to_string_lossy().to_string()
}

/// `AnnoArrays.java` 只发射一次（各测试共用）；缺 JDK → None
fn anno_scratch() -> Option<&'static PathBuf> {
    static SCRATCH: OnceLock<Option<PathBuf>> = OnceLock::new();
    SCRATCH
        .get_or_init(|| {
            let out = std::env::temp_dir().join(format!("rava-meta-anno-{}", std::process::id()));
            let java = manifest_dir().join("tests/fixtures/AnnoArrays.java");
            rava("build", &java, &["--stop-after", "emit", "--clean", "--out", &s(&out)])?;
            Some(out)
        })
        .as_ref()
}

/// 源文件里 `NAME: &[u8] = b"..."` 字面量的字节（`render_bytes` 的转义：`\"` `\\` `\xNN` 与 `\` + 换行续行）
fn byte_literal(src: &str, name: &str) -> Vec<u8> {
    let head = format!(" {name}: &[u8] = b\"");
    let at = src.find(&head).unwrap_or_else(|| panic!("缺表 {name}")) + head.len();
    let b = src.as_bytes();
    let (mut i, mut out) = (at, Vec::new());
    loop {
        match b[i] {
            b'"' => return out,
            b'\\' => match b[i + 1] {
                b'x' => {
                    out.push(u8::from_str_radix(&src[i + 2..i + 4], 16).unwrap());
                    i += 4;
                }
                b'\n' => {
                    i += 2;
                    while b[i].is_ascii_whitespace() {
                        i += 1;
                    }
                }
                c => {
                    out.push(c);
                    i += 2;
                }
            },
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
}

/// LEB128 字节流读取（运行时 `meta_codec::Reader` 的字形）
struct Reader<'a> {
    pool: &'a [Vec<u8>],
    b: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn u(&mut self) -> u64 {
        let (mut v, mut sh) = (0u64, 0);
        loop {
            let x = self.b[self.at];
            self.at += 1;
            v |= u64::from(x & 0x7f) << sh;
            if x & 0x80 == 0 {
                return v;
            }
            sh += 7;
        }
    }
    fn zz(&mut self) -> i64 {
        let v = self.u();
        (v >> 1) as i64 ^ -((v & 1) as i64)
    }
    fn bytes(&mut self) -> Vec<u8> {
        let i = self.u() as usize;
        self.pool[i].clone()
    }
}

/// 池：各项「LEB128 长度 + 内容」首尾相接
fn pool_items(p: &[u8]) -> Vec<Vec<u8>> {
    let mut r = Reader { pool: &[], b: p, at: 0 };
    let mut out = Vec::new();
    while r.at < p.len() {
        let n = r.u() as usize;
        out.push(p[r.at..r.at + n].to_vec());
        r.at += n;
    }
    out
}

/// 用户元数据表 CLASS_ANNO 解码：类 → (注解原始字节, 稀疏常量池)
fn user_class_annos(scratch: &Path) -> BTreeMap<String, (Vec<u8>, BTreeMap<u16, AnnoConst>)> {
    let src = std::fs::read_to_string(scratch.join("user/src/rava_user_meta.rs")).expect("读用户元数据表");
    let pool = pool_items(&byte_literal(&src, "META_POOL"));
    let table = byte_literal(&src, "CLASS_ANNO");
    let mut r = Reader { pool: &pool, b: &table, at: 0 };
    let mut out = BTreeMap::new();
    while r.at < table.len() {
        let class = String::from_utf8(r.bytes()).unwrap();
        let raw = r.bytes();
        let mut cp = BTreeMap::new();
        for _ in 0..r.u() {
            let idx = r.zz() as u16;
            let v = match r.u() {
                0 => AnnoConst::Utf8(String::from_utf8(r.bytes()).unwrap()),
                1 => AnnoConst::Utf16((0..r.u()).map(|_| r.u() as u16).collect()),
                2 => AnnoConst::Int(r.zz() as i32),
                3 => AnnoConst::Long(r.zz()),
                4 => AnnoConst::Float(r.u() as u32),
                5 => AnnoConst::Double(r.u()),
                t => panic!("未知常量标签 {t}"),
            };
            cp.insert(idx, v);
        }
        out.insert(class, (raw, cp));
    }
    out
}

/// 类文件里类级注解的原始字节，与类级注解引用的常量池条目（类文件的稀疏池含字段 / 方法注解，按类级引用取子集）
fn class_file_annos(scratch: &Path, class: &str) -> (Vec<u8>, BTreeMap<u16, AnnoConst>) {
    let data = std::fs::read(scratch.join("closure_input/classes").join(format!("{class}.class"))).expect("读类文件");
    let x = parse_extras(&data).expect("解析类文件");
    (x.raw_annotations, x.anno_cpool)
}

/// 持有类 class 的类级注解经 CLASS_ANNO 往返后与类文件一致；返回解回的常量池
fn assert_round_trip(class: &str) -> Option<BTreeMap<u16, AnnoConst>> {
    let scratch = anno_scratch()?;
    let rows = user_class_annos(scratch);
    let (raw, cp) = rows.get(class).unwrap_or_else(|| panic!("CLASS_ANNO 缺 L1 注解持有类 {class}：{:?}", rows.keys().collect::<Vec<_>>()));
    let (want_raw, want_cp) = class_file_annos(scratch, class);
    assert!(!want_raw.is_empty(), "{class} 类文件无类级注解");
    assert_eq!(raw, &want_raw, "{class} 注解原始字节往返不一致");
    for (i, v) in &want_cp {
        assert_eq!(cp.get(i), Some(v), "{class} 常量池 #{i} 往返不一致");
    }
    Some(cp.clone())
}

fn utf8s(cp: &BTreeMap<u16, AnnoConst>) -> Vec<&str> {
    cp.values().filter_map(|v| if let AnnoConst::Utf8(s) = v { Some(s.as_str()) } else { None }).collect()
}

/// 数组元素值（字符串数组、超出单字节 LEB128 的 int 数组）往返
#[test]
fn array_annotation_values_round_trip() {
    let Some(cp) = assert_round_trip("AnnoArrays$ArrHolder") else { return };
    let strs = utf8s(&cp);
    assert!(strs.contains(&"alpha") && strs.contains(&"beta"), "{cp:?}");
    assert!(cp.values().any(|v| *v == AnnoConst::Int(70000)), "{cp:?}");
}

/// 嵌套注解数组（`@Outer({@Inner(..), @Inner(..)})`）往返
#[test]
fn nested_array_annotation_round_trip() {
    let Some(cp) = assert_round_trip("AnnoArrays$NestedHolder") else { return };
    let strs = utf8s(&cp);
    assert!(strs.contains(&"x1") && strs.contains(&"y2") && strs.contains(&"LAnnoArrays$Inner;"), "{cp:?}");
}

/// 反射调用调用者敏感方法读私有静态字段：字段声明类有字段表；注解类型（含元注解与超接口）有方法表
#[test]
fn reflective_private_static_field_member_tables() {
    let out = std::env::temp_dir().join(format!("rava-meta-cs-{}.json", std::process::id()));
    let java = manifest_dir().join("tests/fixtures/ReflectCsField.java");
    if rava("closure", &java, &["-o", &s(&out)]).is_none() {
        return;
    }
    let d: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&out).expect("读闭包 JSON")).expect("闭包 JSON");
    let _ = std::fs::remove_file(&out);
    let set = |k: &str| -> Vec<String> {
        d["reflect"][k].as_array().expect("数组").iter().map(|x| x.as_str().unwrap().to_string()).collect()
    };
    let (methods, fields) = (set("meta_methods"), set("meta_fields"));
    assert!(fields.iter().any(|c| c == "ReflectCsField"), "字段表缺 ReflectCsField：{fields:?}");
    for c in [
        "java/lang/reflect/Field",
        "jdk/internal/reflect/CallerSensitive",
        "java/lang/annotation/Retention",
        "java/lang/annotation/Annotation",
    ] {
        assert!(methods.iter().any(|m| m == c), "方法表缺 {c}");
    }
}
