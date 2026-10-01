//! `rava build --stop-after emit` 端到端：夹具（tests/fixtures/*.java）经 javac → 闭包分析 → 发射，
//! 断言生成文本形态与命令行输出（不编译生成物）。找不到 JDK 21 时跳过。

use std::path::{Path, PathBuf};
use std::process::Command;

fn runtime_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../runtime/java_runtime")
}

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

/// 一次 `rava build`：返回 (stdout, scratch 目录)；缺 JDK → None
fn build(java: &str, tag: &str, extra: &[&str]) -> Option<(String, PathBuf)> {
    let out = std::env::temp_dir().join(format!("rava-it-{tag}-{}", std::process::id()));
    let o = Command::new(env!("CARGO_BIN_EXE_rava"))
        .arg("build")
        .arg(fixture(java))
        .args(["--jdk", "21", "--stop-after", "emit", "--clean", "--runtime"])
        .arg(runtime_dir())
        .arg("--out")
        .arg(&out)
        .args(extra)
        .output()
        .expect("启动 rava");
    let stderr = String::from_utf8_lossy(&o.stderr).to_string();
    if !o.status.success() {
        if stderr.contains("未找到 JDK") {
            eprintln!("[build_cli] 跳过：无 JDK 21");
            return None;
        }
        panic!("rava build 失败：\n{stderr}");
    }
    Some((String::from_utf8_lossy(&o.stdout).to_string(), out))
}

/// 目录下全部 .rs 文本（路径序）
fn rs_text(dir: &Path) -> String {
    let mut files = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).into_iter().flatten().flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "rs") {
                files.push(p);
            }
        }
    }
    files.sort();
    files.iter().map(|f| std::fs::read_to_string(f).unwrap_or_default()).collect::<Vec<_>>().join("\n")
}

/// record 的 ObjectMethods、typeSwitch（含限定枚举常量）、enumSwitch 全部翻译，不落存根 / 占位
#[test]
fn record_and_switch_bootstraps_translate() {
    let Some((_, out)) = build("RecordSwitch.java", "record-switch", &[]) else { return };
    let st: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(out.join("build_status.json")).unwrap()).unwrap();
    assert_eq!((st["stage"].as_str(), st["ok"].as_bool()), (Some("emit"), Some(true)), "build_status.json：{st}");
    assert_eq!(st["jdk"]["major"], 21);
    let user = rs_text(&out.join("user/src"));
    for bad in ["panic!(\"stub:", "TODO", "Object::from_any", ".downcast::<", "Default::default() /*"] {
        assert!(!user.contains(bad), "用户类生成文本含 {bad}");
    }
    // toString：简单名 + 分量模板；hashCode：31 累乘；equals：instanceof 后逐分量比较
    assert!(
        user.contains("(String::of(\"Point[x=\") + this.__get_x() + \", y=\" + this.__get_y() + \", z=\"")
            && user.contains("\", name=\" + &this.__get_name() + \"]\")"),
        "record toString 模板（UTF-16 层拼接）"
    );
    assert!(user.contains("wrapping_mul(31)"), "record hashCode");
    assert!(user.contains("o.is_instance_of(\"RecordSwitch$Point\") && {"), "record equals");
    // 首分量为引用（块表达式开头）时整体加括号，否则语句位置的 `{ .. } && ..` 被解析成块语句
    let label_eq = user.lines().find(|l| l.contains("o.is_instance_of(\"RecordSwitch$Label\") && {")).expect("Label equals");
    assert!(label_eq.contains("); ({ let x"), "引用首分量的比较整体加括号：{label_eq}");
    // typeSwitch 的限定枚举常量标签按身份比较常量；enumSwitch 同一翻译
    assert!(user.contains("__ts_sel1 == Object::from(Clone::clone(&RecordSwitch_Color::RED()?))"), "枚举常量标签");
    // 拼接实参的 toString 物化按 Java 求值序（从左到右）
    let main_rs = std::fs::read_to_string(out.join("user/src/record_switch.rs")).unwrap();
    let lines: Vec<&str> = main_rs.lines().collect();
    let at = |needle: &str| lines.iter().position(|l| l.contains(needle)).unwrap_or_else(|| panic!("缺 {needle}"));
    let (vp, vq) = (at("valueOf_obj(Object::from(Clone::clone(&p)))"), at("valueOf_obj(Object::from(Clone::clone(&q)))"));
    assert!(vp < vq, "拼接实参字符串化次序");
    // `System.out` 的 getstatic 先于拼接实参求值（JVM 栈序），物化在 toString 之前
    assert!(at("= System::out()?;") < vp, "System.out 读取先于实参 toString");
    let name = |i: usize| lines[i].trim_start().trim_start_matches("let ").split(':').next().unwrap().to_string();
    assert!(main_rs.contains(&format!("(String::of(\"\") + &{} + \" / \" + &{})", name(vp), name(vq))), "拼接模板");
    std::fs::remove_dir_all(&out).ok();
}

/// `--batch`：入口写 `user/src/bin/<bin>.rs`（`#[path]` 引用同级类文件），user/Cargo.toml 追加 `[[bin]]`；
/// `--trace-class` 打印 provenance 链；`--debug` 时审计照常输出（Python 序八行）；`--raw-sites` 追加位点剖面
#[test]
fn batch_trace_and_debug() {
    let sites = std::env::temp_dir().join(format!("rava-it-raw-sites-{}.tsv", std::process::id()));
    let _ = std::fs::remove_file(&sites);
    let sites_arg = sites.to_string_lossy().to_string();
    let Some((stdout, out)) = build(
        "RecordSwitch.java",
        "batch",
        &["--batch", "--debug", "--trace-class", "RecordSwitch$Point", "--raw-sites", &sites_arg],
    ) else {
        return;
    };
    let bin = std::fs::read_to_string(out.join("user/src/bin/record_switch.rs")).expect("批量入口");
    assert!(bin.contains("#[path = \"../record_switch.rs\"]"), "批量入口 #[path]：{bin}");
    let cargo = std::fs::read_to_string(out.join("user/Cargo.toml")).unwrap();
    assert!(cargo.contains("[[bin]]\nname = \"record_switch\"\npath = \"src/bin/record_switch.rs\""), "[[bin]] 段：{cargo}");
    assert!(stdout.contains("      [why] RecordSwitch$Point"), "provenance 链：{stdout}");
    assert!(stdout.contains("[precheck] native-missing="), "预检行");
    let heads = ["[cfg-audit] ", "[readability-audit] ", "[equiv-audit] ", "[fallback-audit] ", "[shortname-audit] ", "[raw-audit] "];
    let pos: Vec<usize> = heads.iter().map(|h| stdout.find(h).unwrap_or_else(|| panic!("缺审计行 {h}：{stdout}"))).collect();
    assert!(pos.windows(2).all(|w| w[0] < w[1]), "审计行序：{pos:?}");
    let rd = stdout.lines().find(|l| l.starts_with("[readability-audit] ")).unwrap();
    let keys: Vec<&str> = rd["[readability-audit] ".len()..].split(' ').map(|kv| kv.split('=').next().unwrap()).collect();
    assert_eq!(keys, ["from_any", "downcast", "downcast_ref", "rc_new", "borrow"], "{rd}");
    let raw = stdout.lines().find(|l| l.starts_with("[raw-audit] ")).unwrap();
    assert!(raw.starts_with("[raw-audit] raw_expr=") && raw.contains(" raw_stmt=") && raw.contains(" non_native_overrides="), "{raw}");
    // 剖面行 `{n}\t{kind}\t{site}`，次数降序；总数与审计行一致
    let prof = std::fs::read_to_string(&sites).unwrap_or_default();
    let rows: Vec<(usize, &str)> = prof
        .lines()
        .map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            assert_eq!(f.len(), 3, "剖面行：{l}");
            assert!(f[2].contains(".rs:"), "位点：{l}");
            (f[0].parse().unwrap(), f[1])
        })
        .collect();
    assert!(rows.windows(2).all(|w| w[0].0 >= w[1].0), "降序");
    let total = |k: &str| rows.iter().filter(|r| r.1 == k).map(|r| r.0).sum::<usize>();
    assert!(raw.contains(&format!("raw_expr={} raw_stmt={}", total("raw_expr"), total("raw_stmt"))), "{raw} vs 剖面");
    std::fs::remove_file(&sites).ok();
    std::fs::remove_dir_all(&out).ok();
}

/// `--lib`：jar 类进独立 lib crate（Java 可见性、lib.rs 包模块、user 依赖）；`--full-precheck`
/// 只出预检，不出审计与发射汇总
#[test]
fn lib_crate_and_precheck_only() {
    let Some(home) = resolve::jdk::find_major(21) else { return };
    let work = std::env::temp_dir().join(format!("rava-it-libjar-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&work);
    let classes = work.join("classes");
    let src = fixture("lib/greet/Greeter.java");
    assert!(Command::new(home.join("bin/javac")).arg("-d").arg(&classes).arg(&src).status().unwrap().success());
    let jar = work.join("greet.jar");
    assert!(Command::new(home.join("bin/jar")).arg("cf").arg(&jar).arg("-C").arg(&classes).arg(".").status().unwrap().success());
    let spec = format!("greet={}", jar.display());
    let Some((stdout, out)) = build("LibUser.java", "lib", &["--lib", &spec, "--full-precheck"]) else { return };
    assert!(stdout.contains("[jar] greet ← greet.jar（1 类）"), "{stdout}");
    assert!(stdout.contains("[precheck] native-missing="), "预检行");
    assert!(!stdout.contains("[equiv-audit]") && !stdout.contains("[emit]"), "--full-precheck 不出审计 / 汇总：{stdout}");
    let lib_cargo = std::fs::read_to_string(out.join("greet/Cargo.toml")).unwrap();
    assert!(lib_cargo.contains("crate-type = [\"lib\"]"), "{lib_cargo}");
    assert!(std::fs::read_to_string(out.join("greet/src/lib.rs")).unwrap().contains("pub mod greet;"));
    let greeter = std::fs::read_to_string(out.join("greet/src/greet/greeter.rs")).unwrap();
    assert!(greeter.contains("pub fn greet(&self"), "public 方法跨 crate 可见");
    assert!(greeter.contains("pub(crate) fn prefix("), "包可见方法降级：{greeter}");
    let user_cargo = std::fs::read_to_string(out.join("user/Cargo.toml")).unwrap();
    assert!(user_cargo.contains("greet           = { path = \"../greet\" }"), "{user_cargo}");
    let ws = std::fs::read_to_string(out.join("Cargo.toml")).unwrap();
    assert!(ws.contains("\"greet\""), "workspace 成员：{ws}");
    assert!(rs_text(&out.join("user/src")).contains("greet::greet::Greeter"), "user 经 lib crate 名引用");
    std::fs::remove_dir_all(&out).ok();
    std::fs::remove_dir_all(&work).ok();
}

/// try/finally 内 `return e;` 的返回值暂存：各臂存储同槽、类型不一（汇合为根类）时，已是汇合类型
/// 那一臂的存储不得丢失（回归：DeepCopy `ObjectInputStream.readObject0` 的 E0381）
#[test]
fn try_finally_return_temp_kept_in_every_arm() {
    let Some((_, out)) = build("TryFinallyReturn.java", "try-finally-return", &[]) else { return };
    let rs = std::fs::read_to_string(out.join("user/src/try_finally_return.rs")).unwrap();
    let body: Vec<&str> = rs.lines().skip_while(|l| !l.contains("pub fn pick(")).take_while(|l| !l.contains("pub fn main(")).collect();
    // 终态语义与臂的字面形态无关（case 标签可能因选择子值域与 default 合臂）：
    // 返回值类型已是汇合类型（Object）的 case 1 臂，取值后紧接着存入暂存槽
    let a1 = body.iter().position(|l| l.contains("let _t1: Object = Self::a()?;")).expect("case 1 取返回值");
    assert_eq!(body[a1 + 1].trim(), "local_1 = Clone::clone(&_t1);", "{}", body.join("\n"));
    assert_eq!(body.iter().filter(|l| l.trim_start().starts_with("local_1 = ")).count(), 3, "三个臂都存储返回值");
    std::fs::remove_dir_all(&out).ok();
}

/// 祖先类由接口 default 注入的槽位，子类经更具体接口的 default 覆盖时落在祖先槽位上
/// （回归：CollectorsDemo `ListN.stream` → `AbstractCollection.spliterator` 存根）
#[test]
fn more_specific_default_overrides_ancestor_injected_slot() {
    let Some((_, out)) = build("DefaultSlot.java", "default-slot", &[]) else { return };
    let user = rs_text(&out.join("user/src"));
    let split_attrs: Vec<&str> = user.lines().filter(|l| l.contains("#[java_method(name = \"split\"")).collect();
    let list_default =
        split_attrs.iter().filter(|l| l.contains("virtual_in = ")).map(|l| l.to_string()).collect::<Vec<_>>();
    assert!(
        list_default.iter().all(|l| l.contains("virtual_in = \"DefaultSlot_AbsColl\"")),
        "split 展开全部落在 AbsColl 槽位：{list_default:#?}"
    );
    assert_eq!(list_default.len(), 2, "AbsColl 注入 + AbsList 覆盖：{list_default:#?}");
    // 叶子类的继承成员经祖先槽位派发到 AbsList 的覆盖
    assert!(
        user.contains("inherited_from = \"DefaultSlot_AbsList\", vtable_owner = \"DefaultSlot_AbsColl\")]"),
        "ListN.split 继承自 AbsList、槽位在 AbsColl"
    );
    std::fs::remove_dir_all(&out).ok();
}

/// 恒 null 接收者（字段类型无实例、读作 null）：调用点按 invokevirtual 语义抛 NullPointerException
/// （`__null_recv` 携类名 + 方法名 + 描述符，接收者非 null 时精确 panic），不翻译调用、不得静默给默认值
#[test]
fn virtual_view_null_receiver_throws_npe() {
    let Some((_, out)) = build("NullView.java", "null-view", &[]) else { return };
    let rs = std::fs::read_to_string(out.join("user/src/null_view_holder.rs")).unwrap();
    let line = rs.lines().find(|l| l.contains("__null_recv(")).expect("h.name() 导出为 null_recv");
    assert!(line.contains("\"NullView$Handler.name:()Ljava/lang/String;\""), "{line}");
    assert!(!line.contains("Default::default()") && !rs.contains("__virtual_view("), "{line}");
    std::fs::remove_dir_all(&out).ok();
}

/// 字符串拼接求值顺序（JDK 21 语义）：实参从左到右求值与转换，toString 物化依实参序；
/// 夹在两个有副作用调用之间的静态字段读取不得推迟到后一个调用之后（JVM 输出 `X-1-Y`）
#[test]
fn concat_operands_evaluate_left_to_right() {
    let Some((_, out)) = build("ConcatOrder.java", "concat-order", &[]) else { return };
    let rs = std::fs::read_to_string(out.join("user/src/concat_order.rs")).unwrap();
    let body: Vec<&str> = rs.lines().skip_while(|l| !l.contains("pub fn main(")).collect();
    let at = |needle: &str| body.iter().position(|l| l.contains(needle)).unwrap_or_else(|| panic!("缺 {needle}：\n{}", body.join("\n")));
    assert!(at("valueOf_obj(Object::from(Clone::clone(&a)))") < at("valueOf_obj(Object::from(Clone::clone(&b)))"));
    let (x, step, y) = (at("String::from(\"X\")"), at("ConcatOrder::step()?"), at("String::from(\"Y\")"));
    assert!(x < step && step < y, "step 读取在 make(X) 与 make(Y) 之间：\n{}", body.join("\n"));
    assert!(body[step].trim_start().starts_with("let "), "step 读取物化为临时量：{}", body[step]);
    std::fs::remove_dir_all(&out).ok();
}

/// `rava image-dirs`：每行一个已存在的类目录；VM 支持类按模块给出（与 `build` 缺省派生同源）
#[test]
fn image_dirs_lists_existing_class_dirs() {
    if resolve::jdk::find_major(21).is_none() {
        return;
    }
    let o = Command::new(env!("CARGO_BIN_EXE_rava"))
        .args(["image-dirs", "--jdk", "21", "--runtime"])
        .arg(runtime_dir())
        .output()
        .expect("启动 rava");
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    let text = String::from_utf8_lossy(&o.stdout).to_string();
    let dirs: Vec<&str> = text.lines().collect();
    assert!(dirs.iter().all(|d| Path::new(d).is_dir()), "{text}");
    let modules = std::fs::read_dir(runtime_dir().join("../java_support")).unwrap().flatten().filter(|e| e.path().is_dir()).count();
    assert!(dirs.iter().filter(|d| d.contains("/rava/vmsupport/")).count() == modules, "{text}");
}

/// `--api-package`：包内公开 API 为入口，`--full-precheck` 出预检明细（同 `rava audit api` 入口）
#[test]
fn api_package_precheck() {
    let Some((stdout, out)) = build("TryFinallyReturn.java", "api", &["--api-package", "java/util/function", "--full-precheck", "--closure-json"]) else {
        return;
    };
    let line = stdout.lines().find(|l| l.starts_with("[api] java/util/function（不含子包）→ ")).unwrap_or_else(|| panic!("{stdout}"));
    let n: usize = line.split("→ ").nth(1).and_then(|r| r.split(' ').next()).and_then(|n| n.parse().ok()).unwrap();
    assert!(n > 30, "{line}");
    assert!(stdout.contains("[precheck] native-missing="), "{stdout}");
    let facts = std::fs::read_to_string(out.join("closure_input/closure.json")).unwrap();
    assert!(facts.contains("java/util/function/BiFunction"), "API 入口类入闭包");
    std::fs::remove_dir_all(&out).ok();
}

/// L1（名字级）类发不透明形态：只作 instanceof / 字段类型出现、无实例的类型发 `java_class_opaque!`，
/// 已实例化类的超接口升 L2 照常发射；L1 属主上的虚调用导出为 null_recv（不翻译调用）
#[test]
fn name_level_classes_emit_opaque() {
    let Some((_, out)) = build("OpaqueLevels.java", "opaque-levels", &[]) else { return };
    let read = |f: &str| std::fs::read_to_string(out.join("user/src").join(f)).unwrap();
    let marker = read("opaque_levels_marker.rs");
    assert!(marker.contains("rava_macros::java_class_opaque!") && marker.contains("pub struct OpaqueLevels_Marker;"), "{marker}");
    let ghost = read("opaque_levels_ghost.rs");
    assert!(ghost.contains("pub struct OpaqueLevels_Ghost: OpaqueLevels_Shape;"), "{ghost}");
    let shape = read("opaque_levels_shape.rs");
    assert!(shape.contains("rava_macros::java_class!") && !shape.contains("java_class_opaque"), "已实例化类的超接口至少 L2：{shape}");
    let main = read("opaque_levels.rs");
    assert!(main.contains("__null_recv(") && main.contains("OpaqueLevels$Ghost.name:()Ljava/lang/String;"), "{main}");
    // 接收者静态类型为 L1 类（局部变量 Phantom）、成员在 L2 祖先 Base 上：先上转到属主视图
    let phantom = read("opaque_levels_phantom.rs");
    assert!(phantom.contains("rava_macros::java_class_opaque!"), "{phantom}");
    let view = "Into::<OpaqueLevels_Base>::into(Clone::clone(&p))";
    assert!(main.contains(&format!("{view}.__nn()?.__get_tag()")), "L1 接收者读字段须上转：{main}");
    assert!(main.contains(&format!("{view}.__nn()?.__set_tag(5i32)")), "L1 接收者写字段须上转：{main}");
    // 折叠出的 null 常量接收者：取声明类的 null
    assert!(main.contains("<OpaqueLevels_Base>::default().__nn()?.__get_tag()"), "{main}");
    assert!(!main.contains("Object::default().__nn()?.__get_tag()"), "{main}");
    std::fs::remove_dir_all(&out).ok();
}

/// 动态代理实现的接口（无静态实现类）：属主升 L2 照常发射，接口调用不导出 null_recv
#[test]
fn proxy_interface_owner_not_opaque() {
    let Some((_, out)) = build("ProxyIface.java", "proxy-iface", &[]) else { return };
    let read = |f: &str| std::fs::read_to_string(out.join("user/src").join(f)).unwrap();
    let greeter = read("proxy_iface_greeter.rs");
    assert!(!greeter.contains("java_class_opaque"), "代理接口须至少 L2：{greeter}");
    let main = read("proxy_iface.rs");
    assert!(!main.contains("__null_recv("), "代理对象上的接口调用不得判恒 null：{main}");
    std::fs::remove_dir_all(&out).ok();
}

/// 手写体经注册表工厂构造的资源束经 setParent 串成父链：束对象须作为值进入流图，
/// `ResourceBundle.getObject` 的 `parent.getObject(key)`（@22）不得判为接收者恒 null
#[test]
fn locale_bundle_parent_not_null_recv() {
    let Some((_, out)) = build("LocaleBundleParent.java", "rbparent", &["--full-precheck", "--closure-json"]) else {
        return;
    };
    let facts: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(out.join("closure_input/closure.json")).unwrap()).unwrap();
    let get_object = "java/util/ResourceBundle.getObject:(Ljava/lang/String;)Ljava/lang/Object;";
    let folds = facts["folds"].as_array().unwrap();
    let nr: Vec<u64> = folds
        .iter()
        .filter(|f| f["method"] == get_object)
        .flat_map(|f| f["null_recv"].as_array().cloned().unwrap_or_default())
        .filter_map(|x| x.as_u64())
        .collect();
    assert!(!nr.contains(&22), "getObject@22 误判恒 null：{nr:?}");
    // 非空断言：getObject 在闭包内（父链查找路径确实被分析）
    assert!(facts["methods"].as_array().unwrap().iter().any(|m| m["id"] == get_object), "getObject 不在闭包内");
    std::fs::remove_dir_all(&out).ok();
}
