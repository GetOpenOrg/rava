/// build.rs — 自动扫描生成的 .rs 文件，维护 native_status.toml 与类层次表
///
/// 职责：
///   1. 扫描 src/ 下所有 .rs 文件，提取 #[java_class] / #[java_native] 属性
///   2. 扫描 src/ 下 *_impl.rs 文件，对照已实现的方法（K-4 共置结构）
///   3. 更新 native_status.toml：implemented / needed / stub / not-needed
///   4. 打印 "needed" 状态的 native 方法清单（警告，不阻断构建）
///   5. 从 java_class! 块的 all_supertypes 属性生成类层次表（OUT_DIR/
///      hierarchy_table.rs），供 Class.isAssignableFrom 等运行时查询——
///      层次数据只在 Rust 侧表达一份（来自 class 元数据），Python 侧不再推导
///   6. 从 java_class! 块的 java_field 属性行生成字段元数据表（OUT_DIR/
///      field_table.rs：binary name → 名/描述符/修饰位/static/ConstantValue），
///      供 Class.getDeclaredField / Field.get/set 反射查询
///   7. 从 java_class! 块的 java_method / java_native 属性行生成方法元数据表
///      （OUT_DIR/method_table.rs：binary name → 名/描述符/修饰位/static/
///      native/abstract，声明序保留），供 Class.getDeclaredMethod 与
///      MethodHandleNatives.resolve（MemberName 解析内核）查询。方法身份键
///      是 (name, descriptor) 二元组（重载语义），与字段表同源同协议。

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let src_dir     = Path::new("src");
    let status_file = Path::new("../native_status.toml");

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=../user/src/");
    println!("cargo:rerun-if-env=changed=JAVA_RTA_STRICT");
    // 类宇宙 = 运行时 crate 树 + 用户 crate 树（../user/src）+ lib crate 树
    //（jar 输入模式的兄弟 crate，如 junit4/hamcrest——2026-09-23 用户树扩展
    // 的延续，元数据表（层次/直接父类/字段/方法/注解）描述整个 workspace 的
    // 类）。用户类与 lib 类与生成 JDK 类同一属性协议（java_class!）。
    // native 状态与 _impl 共置是运行时自身的覆盖面概念，仍只扫运行时树。
    let mut meta_roots: Vec<PathBuf> = vec![src_dir.to_path_buf(), PathBuf::from("../user/src")];
    for lib_root in discover_lib_crate_roots() {
        println!("cargo:rerun-if-changed={}", lib_root.display());
        meta_roots.push(lib_root);
    }

    let native_methods = scan_native_methods(src_dir);
    let status: BTreeMap<String, BTreeMap<String, String>> = load_status(status_file);
    // K-4: _impl.rs 文件与生成 stub 共置于 src/，scan_impls 只处理 *_impl.rs
    let implemented = scan_impls(src_dir);

    let mut new_status: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    for nm in &native_methods {
        let class_key  = nm.class.replace('/', ".");
        let method_key = nm.method.clone();
        if status.get(&class_key)
            .and_then(|c| c.get(&method_key))
            .map(|s| s.as_str()) == Some("not-needed")
        {
            new_status.entry(class_key).or_default()
                .insert(method_key, "not-needed".to_owned());
            continue;
        }
        let impl_key   = format!("{}.{}", nm.class, nm.method);
        let entry_stat = if implemented.contains(&impl_key) { "implemented" } else { "needed" };
        new_status.entry(class_key).or_default().insert(method_key, entry_stat.to_owned());
    }

    write_status(status_file, &new_status);

    let meta_roots: Vec<&Path> = meta_roots.iter()
        .filter(|p| p.is_dir())
        .map(|p| p.as_path())
        .collect();
    let hierarchy = scan_class_hierarchy(&meta_roots);
    write_hierarchy_table(&hierarchy);
    write_direct_super_table(&scan_direct_super(&meta_roots));
    write_field_table(&scan_class_fields(&meta_roots));
    write_method_table(&scan_class_methods(&meta_roots));
    write_modifiers_table(&scan_class_modifiers(&meta_roots));
    write_record_table(&scan_record_classes(&meta_roots));
    write_annotation_table(&scan_annotations(&meta_roots));

    let strict = std::env::var("JAVA_RTA_STRICT").unwrap_or_default() == "1";
    let needed: Vec<_> = new_status.iter()
        .flat_map(|(cls, methods)| {
            methods.iter()
                .filter(|(_, s)| s.as_str() == "needed")
                .map(move |(m, _)| format!("  → {}.{}", cls, m))
        })
        .collect();

    if !needed.is_empty() {
        if strict {
            for line in &needed {
                let method = line.trim_start_matches("  → ");
                println!("cargo::error=native method not implemented: {}", method);
            }
        } else {
            println!("cargo:warning=");
            println!("cargo:warning=─── native methods needing implementation ───");
            for line in &needed { println!("cargo:warning={}", line); }
            println!("cargo:warning=Add implementations to src/<package>/<class>_impl.rs");
            println!("cargo:warning=");
        }
    }
}

struct NativeMethod { class: String, method: String, #[allow(dead_code)] descriptor: String }

fn scan_native_methods(src_dir: &Path) -> Vec<NativeMethod> {
    let mut result = Vec::new();
    if !src_dir.exists() { return result; }
    for path in walk_rs_files(src_dir) {
        let content = fs::read_to_string(&path).unwrap_or_default();
        parse_native_comments(&content, &mut result);
    }
    result
}

fn parse_native_comments(content: &str, out: &mut Vec<NativeMethod>) {
    let mut current_class = String::new();
    let mut in_class_attr = false;
    let mut class_attr_buf = String::new();
    for line in content.lines() {
        let trimmed = line.trim();
        let is_class_attr_start = trimmed.starts_with("#[java_class(")
            || trimmed.starts_with("#[cfg_attr(any(), java_class(");
        if is_class_attr_start {
            in_class_attr = true;
            class_attr_buf = trimmed.to_owned();
        } else if in_class_attr {
            class_attr_buf.push(' ');
            class_attr_buf.push_str(trimmed);
        }
        if in_class_attr && class_attr_buf.contains(")]") {
            if let Some(name) = extract_attr(&class_attr_buf, "binary_name") {
                current_class = name;
            }
            in_class_attr = false;
            class_attr_buf.clear();
        }
        let is_native_attr = trimmed.starts_with("#[java_native(")
            || trimmed.starts_with("#[cfg_attr(any(), java_native(");
        if is_native_attr {
            if let Some(method) = extract_attr(trimmed, "name") {
                let descriptor = extract_attr(trimmed, "descriptor").unwrap_or_default();
                if !current_class.is_empty() {
                    out.push(NativeMethod { class: current_class.clone(), method, descriptor });
                }
            }
        }
    }
}

/// 生成属性的键与 '=' 之间有对齐填充空格（`#[binary_name       = "..."]`），
/// 此提取器容忍空白；键须以 `#[` 前缀出现，避免子串误配。
fn extract_attr_padded(s: &str, key: &str) -> Option<String> {
    let start = s.find(&format!("#[{}", key))?;
    let rest = s[start + key.len() + 2..].trim_start();
    let rest = rest.strip_prefix('=')?.trim_start();
    let rest = rest.strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(rest[..end].to_owned())
}

fn extract_attr(s: &str, key: &str) -> Option<String> {
    let pattern = format!("{} = \"", key);
    let start = s.find(&pattern)? + pattern.len();
    let end = s[start..].find('"')? + start;
    Some(s[start..end].to_owned())
}

/// 类层次表扫描：java_class! 块内的裸属性行（`#[binary_name = "..."]` 与
/// `#[all_supertypes = "..."]` 各自独立成行，同一块内 binary_name 在前）。
/// all_supertypes 以 ';' 分隔、含类自身（attrs._compute_all_supertypes）。
fn scan_class_hierarchy(roots: &[&Path]) -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();
    for path in roots.iter().flat_map(|r| walk_rs_files(r)) {
        let content = fs::read_to_string(&path).unwrap_or_default();
        let mut current = String::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(name) = extract_attr_padded(trimmed, "binary_name") {
                current = name;
            }
            if let Some(supers) = extract_attr_padded(trimmed, "all_supertypes") {
                if !current.is_empty() {
                    result.insert(current.clone(), supers);
                }
            }
        }
    }
    result
}

/// 类 → 直接父类（java_class! 块的 super_class 属性；接口无 super_class 属性）。
/// 消费方：Class.getSuperclass（class_impl.rs）。
fn scan_direct_super(roots: &[&Path]) -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();
    for path in roots.iter().flat_map(|r| walk_rs_files(r)) {
        let content = fs::read_to_string(&path).unwrap_or_default();
        let mut current = String::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(name) = extract_attr_padded(trimmed, "binary_name") {
                current = name;
            }
            if let Some(sup) = extract_attr_padded(trimmed, "super_class") {
                if !current.is_empty() && !sup.is_empty() {
                    result.insert(current.clone(), sup);
                }
            }
        }
    }
    result
}

fn write_direct_super_table(entries: &BTreeMap<String, String>) {
    let Ok(out_dir) = std::env::var("OUT_DIR") else { return };
    let mut out = String::from(
        "// 由 build.rs 自动生成：类 → 直接父类表（binary name → super_class 属性）。
         // 数据源：java_class! 块的 super_class 属性（接口无该属性，天然缺席）。
         // 消费方：Class.getSuperclass（class_impl.rs）。请勿手改。

         pub static CLASS_DIRECT_SUPER: &[(&str, &str)] = &[
",
    );
    for (name, sup) in entries {
        out.push_str(&format!("    ({:?}, {:?}),\n", name, sup));
    }
    out.push_str("];
");
    let path = Path::new(&out_dir).join("direct_super_table.rs");
    if let Err(e) = fs::write(&path, &out) {
        panic!("写 direct_super_table.rs 失败: {e}");
    }
}

/// 单个声明字段的元数据（`java_field` 属性行的结构化形态）。
/// modifiers 为 java.lang.reflect.Modifier 位集（public=0x1 / private=0x2 /
/// protected=0x4 / static=0x8 / final=0x10 / volatile=0x40 / transient=0x80 …）；
/// constant 是 ConstantValue 属性的整数值（仅整型常量收录，字符串等形态缺席）。
struct FieldMeta {
    name:       String,
    descriptor: String,
    modifiers:  i32,
    is_static:  bool,
    constant:   Option<i64>,
}

/// 字段元数据扫描：java_class! 块内 java_field 属性行（每字段独立成行，
/// `#[cfg_attr(any(), java_field(name = "x", descriptor = "I", access =
/// "private", modifiers = "static final", is_static = true))]`）。类上下文
/// 与层次表同源（同块内 binary_name 在前）。声明顺序保留（Field.slot 语义）。
/// 消费方：Class.getDeclaredField / Field.get/set（class_impl.rs / field_impl.rs）。
fn scan_class_fields(roots: &[&Path]) -> BTreeMap<String, Vec<FieldMeta>> {
    let mut result: BTreeMap<String, Vec<FieldMeta>> = BTreeMap::new();
    for path in roots.iter().flat_map(|r| walk_rs_files(r)) {
        let content = fs::read_to_string(&path).unwrap_or_default();
        let mut current = String::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(name) = extract_attr_padded(trimmed, "binary_name") {
                current = name;
            }
            let Some(open) = trimmed.find("java_field(") else { continue };
            if current.is_empty() { continue; }
            // 属性行自 java_field( 起截取，键值对提取只在该窗口内进行，
            // 避免行内其他 token（如 generic_signature 的泛型描述）误配。
            let window = &trimmed[open..];
            let (Some(name), Some(descriptor)) =
                (extract_attr(window, "name"), extract_attr(window, "descriptor"))
            else { continue };
            let access    = extract_attr(window, "access").unwrap_or_default();
            let modifiers = extract_attr(window, "modifiers").unwrap_or_default();
            let bits = modifier_bits(&access) | modifier_bits(&modifiers);
            // static 判定：is_static 显式标志优先（生成器对 static 字段发出），
            // 无标志时回退到 modifiers 词面（手写形态容错）。
            let is_static = extract_flag(window, "is_static")
                .unwrap_or_else(|| modifiers.split_whitespace().any(|t| t == "static"));
            let constant = extract_attr(window, "constant_value")
                .and_then(|v| v.strip_suffix('L').unwrap_or(&v).parse::<i64>().ok());
            result.entry(current.clone()).or_default().push(FieldMeta {
                name, descriptor, modifiers: bits, is_static, constant,
            });
        }
    }
    result
}

/// 兄弟 lib crate 的 src 树发现（jar 输入模式：junit4/hamcrest 等）。判据 =
/// 兄弟目录含 Cargo.toml + src/（排除自身/用户/target）。确定序（排序）保证
/// 表生成稳定。
fn discover_lib_crate_roots() -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir("..") else { return Vec::new() };
    let mut roots: Vec<PathBuf> = entries.flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir()
            && p.join("Cargo.toml").is_file()
            && p.join("src").is_dir()
            && p.file_name().and_then(|n| n.to_str()) != Some("java_runtime")
            && p.file_name().and_then(|n| n.to_str()) != Some("user"))
        .map(|p| p.join("src"))
        .collect();
    roots.sort();
    roots
}

/// 单条注解记录（RuntimeVisibleAnnotations 的 annotation 结构）。
/// elements 的值保持「tag:载荷」编码文本透传（编码在 codegen/classfile，
/// 解码在 java_runtime::annotation——build.rs 不解释载荷）。
struct AnnoEntry {
    anno:     String,
    elements: Vec<(String, String)>,
}

/// 切分（载荷内保留字符已百分号编码——分隔符只以分隔身份出现）。
fn anno_split(s: &str, sep: char) -> Vec<&str> {
    s.split(sep).collect()
}

/// 注解载荷文本（`bin#name=tag:载荷#...;...`）→ 条目序列。
fn parse_anno_entries(text: &str) -> Vec<AnnoEntry> {
    let mut out = Vec::new();
    for entry in anno_split(text, ';') {
        if entry.is_empty() { continue; }
        let segs = anno_split(entry, '#');
        if segs.is_empty() { continue; }
        let mut e = AnnoEntry { anno: segs[0].to_owned(), elements: Vec::new() };
        for seg in &segs[1..] {
            let Some(eq) = seg.find('=') else { continue };
            e.elements.push((seg[..eq].to_owned(), seg[eq + 1..].to_owned()));
        }
        out.push(e);
    }
    out
}

/// 三挂载点注解扫描（反射 L3 段 1）：
///   - 类级：java_class! 块内 `#[annotations = "..."]` 属性行（binary_name 上下文）；
///   - 方法级：java_method / java_native 属性行的 `annotations = "..."` 键
///     （键 = (类, 方法名, 描述符)——与 method_table 同一身份键）；
///   - 字段级：java_field 属性行的 `annotations = "..."` 键（键 = (类, 字段名)）。
/// 消费方：Class/Method/Field 的 isAnnotationPresent / getAnnotation /
/// getAnnotations（class_impl.rs / method_impl.rs / field_impl.rs）。
fn scan_annotations(roots: &[&Path]) -> AnnotationTables {
    let mut tables = AnnotationTables::default();
    for path in roots.iter().flat_map(|r| walk_rs_files(r)) {
        let content = fs::read_to_string(&path).unwrap_or_default();
        let mut current = String::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(name) = extract_attr_padded(trimmed, "binary_name") {
                current = name;
            }
            if current.is_empty() { continue; }
            // 类级属性行
            if let Some(text) = extract_attr_padded(trimmed, "annotations") {
                tables.classes.entry(current.clone()).or_default()
                    .extend(parse_anno_entries(&text));
                continue;
            }
            // 方法/字段属性行：窗口内提取
            let Some(open) = trimmed.find("java_method(").or_else(|| trimmed.find("java_native("))
                .or_else(|| trimmed.find("java_field(")) else { continue };
            let window = &trimmed[open..];
            let Some(text) = extract_attr(window, "annotations") else { continue };
            let entries = parse_anno_entries(&text);
            if entries.is_empty() { continue; }
            if window.starts_with("java_field(") {
                if let Some(fname) = extract_attr(window, "name") {
                    tables.fields.entry((current.clone(), fname)).or_default().extend(entries);
                }
            } else if let (Some(mname), Some(desc)) =
                (extract_attr(window, "name"), extract_attr(window, "descriptor"))
            {
                tables.methods.entry((current.clone(), mname, desc)).or_default().extend(entries);
            }
        }
    }
    tables
}

#[derive(Default)]
struct AnnotationTables {
    classes:  BTreeMap<String, Vec<AnnoEntry>>,
    methods:  BTreeMap<(String, String, String), Vec<AnnoEntry>>,
    fields:   BTreeMap<(String, String), Vec<AnnoEntry>>,
}

fn write_annotation_table(t: &AnnotationTables) {
    let Ok(out_dir) = std::env::var("OUT_DIR") else { return };
    let mut out = String::from(
        "// 由 build.rs 自动生成：注解元数据表（反射 L3 段 1）。
         // 数据源：java_class! 块的 annotations 属性（类级）与 java_method /
         // java_native / java_field 属性行的 annotations 键——文本来自
         // codegen 对 RuntimeVisibleAnnotations 的解析编码，元素值保持
         // 「tag:载荷」编码透传，解码在 java_runtime::annotation。
         // 消费方：Class/Method/Field 的 isAnnotationPresent / getAnnotation /
         // getAnnotations。请勿手改。

         pub struct AnnotationEntry {
             pub anno:     &'static str,
             pub elements: &'static [(&'static str, &'static str)],
         }

         pub static CLASS_ANNOTATIONS: &[(&str, &[AnnotationEntry])] = &[
         ",
    );
    for (class, entries) in &t.classes {
        if entries.is_empty() { continue; }
        out.push_str(&format!("    ({:?}, &[\n", class));
        out.push_str(&anno_entries_text(entries));
        out.push_str("    ]),\n");
    }
    out.push_str("];

pub static METHOD_ANNOTATIONS: &[(&str, &str, &str, &[AnnotationEntry])] = &[
");
    for ((class, name, desc), entries) in &t.methods {
        if entries.is_empty() { continue; }
        out.push_str(&format!("    ({:?}, {:?}, {:?}, &[\n", class, name, desc));
        out.push_str(&anno_entries_text(entries));
        out.push_str("    ]),\n");
    }
    out.push_str("];

pub static FIELD_ANNOTATIONS: &[(&str, &str, &[AnnotationEntry])] = &[
");
    for ((class, name), entries) in &t.fields {
        if entries.is_empty() { continue; }
        out.push_str(&format!("    ({:?}, {:?}, &[\n", class, name));
        out.push_str(&anno_entries_text(entries));
        out.push_str("    ]),\n");
    }
    out.push_str("];
");
    let path = Path::new(&out_dir).join("annotation_table.rs");
    if let Err(e) = fs::write(&path, &out) {
        panic!("写 annotation_table.rs 失败: {e}");
    }
}

fn anno_entries_text(entries: &[AnnoEntry]) -> String {
    let mut out = String::new();
    for e in entries {
        let elems: Vec<String> = e.elements.iter()
            .map(|(n, v)| format!("({:?}, {:?})", n, v))
            .collect();
        out.push_str(&format!(
            "        AnnotationEntry {{ anno: {:?}, elements: &[{}] }},\n",
            e.anno, elems.join(", "),
        ));
    }
    out
}

/// 访问标志 / 修饰符词串 → java.lang.reflect.Modifier 位集。
/// 未知 token（varargs 等）忽略。
fn modifier_bits(s: &str) -> i32 {
    let mut bits = 0i32;
    for tok in s.split_whitespace() {
        bits |= match tok {
            "public"       => 0x0001,
            "private"      => 0x0002,
            "protected"    => 0x0004,
            "static"       => 0x0008,
            "final"        => 0x0010,
            "synchronized" => 0x0020,
            "volatile"     => 0x0040,
            "transient"    => 0x0080,
            // 方法侧同位异名（JVMS access_flags：字段 volatile=0x40/方法
            // bridge=0x40、字段 transient=0x80/方法 varargs=0x80——
            // java.lang.reflect.Modifier 对方法读 varargs 位）
            "varargs"      => 0x0080,
            "native"       => 0x0100,
            // 类侧专有（java_class! 块 modifiers 属性，attrs._class_modifiers_str）：
            // 生成的接口块带 super_class=Object，class_modifier_bits 的「无父类即
            // 接口」推断对其不成立——INTERFACE/ANNOTATION 位以修饰词为准
            "interface"    => 0x0200,
            "abstract"     => 0x0400,
            "annotation"   => 0x2000,
            "strictfp"     => 0x0800,
            "synthetic"    => 0x1000,
            "enum"         => 0x4000,
            _ => 0,
        };
    }
    bits
}

/// 布尔属性提取（`key = true/false`，容忍对齐空格）；缺席 → None。
fn extract_flag(s: &str, key: &str) -> Option<bool> {
    let start = s.find(&format!("{} ", key))?;
    let rest = s[start + key.len()..].trim_start();
    let rest = rest.strip_prefix('=')?.trim_start();
    if rest.starts_with("true") { Some(true) }
    else if rest.starts_with("false") { Some(false) }
    else { None }
}

fn write_field_table(entries: &BTreeMap<String, Vec<FieldMeta>>) {    let Ok(out_dir) = std::env::var("OUT_DIR") else { return };
    let mut out = String::from(
        "// 由 build.rs 自动生成：字段元数据表（binary name → 声明字段序列，声明序 = slot）。
         // 数据源：java_class! 块内 java_field 属性（字段声明元数据的唯一表达，规则四）。
         // 消费方：Class.getDeclaredField / Field.get/set（class_impl.rs / field_impl.rs）。
         // modifiers 为 java.lang.reflect.Modifier 位集；constant 为 ConstantValue 整数值。
         // 请勿手改。

         pub struct FieldMeta {
             pub name:       &'static str,
             pub descriptor: &'static str,
             pub modifiers:  i32,
             pub is_static:  bool,
             pub constant:   Option<i64>,
         }

         pub static CLASS_FIELDS: &[(&str, &[FieldMeta])] = &[
",
    );
    for (class, fields) in entries {
        if fields.is_empty() { continue; }
        out.push_str(&format!("    ({:?}, &[\n", class));
        for f in fields {
            out.push_str(&format!(
                "        FieldMeta {{ name: {:?}, descriptor: {:?}, modifiers: {:#06x}, is_static: {}, constant: {} }},\n",
                f.name, f.descriptor, f.modifiers, f.is_static,
                match f.constant { Some(v) => format!("Some({}i64)", v), None => "None".to_owned() },
            ));
        }
        out.push_str("    ]),\n");
    }
    out.push_str("];
");
    let path = Path::new(&out_dir).join("field_table.rs");
    if let Err(e) = fs::write(&path, &out) {
        panic!("写 field_table.rs 失败: {e}");
    }
}

/// 单个声明方法的元数据（java_method / java_native 属性行的结构化形态）。
/// modifiers 为 java.lang.reflect.Modifier 位集（方法侧：synchronized=0x20 /
/// varargs=0x80 / native=0x100 / abstract=0x400）；exceptions 为 throws 子句
/// 的 binary name 列表（Method.getExceptionTypes 的数据源）；name 含
/// `<init>` / `<clinit>` 行（构造器/类初始化器的声明记录，消费方按 JDK
/// 语义过滤——getDeclaredMethods 不见二者、getDeclaredConstructors 取
/// `<init>`）。
struct MethodMeta {
    name:       String,
    descriptor: String,
    modifiers:  i32,
    is_static:  bool,
    is_native:  bool,
    is_abstract: bool,
    exceptions: Vec<String>,
}

/// 方法元数据扫描：java_class! 块内 java_method / java_native 属性行。
/// 形态（生成树实测，2026-09-22 TestAtomics scratch 27,080 + 290 行）：
/// - 单段路径 `#[java_method(` / `#[java_native(`（无 cfg_attr 包裹形态）；
/// - 身份键 `name = "` / `descriptor = "` 单空格；布尔标志对齐填充
///   （`is_static    = true` 4 空格），须用 extract_flag 容忍式提取；
/// - 无 name/descriptor 的 `target = "...”` / `result = "..."` 行是 codegen
///   提示属性（接口适配器/checkcast 标记），不是方法声明行，按身份键缺席
///   排除。native 方法的元数据也是方法表行（JDK 反射对 native 一视同仁）。
/// 声明顺序保留（getDeclaredMethods0 的 slot 语义）。
/// 消费方：Class.getDeclaredMethod（class_impl.rs）、MethodHandleNatives.
/// resolve 的方法/构造器 kind（method_handle_natives_impl.rs）。
fn scan_class_methods(roots: &[&Path]) -> BTreeMap<String, Vec<MethodMeta>> {
    let mut result: BTreeMap<String, Vec<MethodMeta>> = BTreeMap::new();
    for path in roots.iter().flat_map(|r| walk_rs_files(r)) {
        let content = fs::read_to_string(&path).unwrap_or_default();
        let mut current = String::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(name) = extract_attr_padded(trimmed, "binary_name") {
                current = name;
            }
            let Some(open) = trimmed.find("java_method(").or_else(|| trimmed.find("java_native(")) else { continue };
            if current.is_empty() { continue; }
            let window = &trimmed[open..];
            let (Some(name), Some(descriptor)) =
                (extract_attr(window, "name"), extract_attr(window, "descriptor"))
            else { continue };
            let access    = extract_attr(window, "access").unwrap_or_default();
            let modifiers = extract_attr(window, "modifiers").unwrap_or_default();
            let bits = modifier_bits(&access) | modifier_bits(&modifiers);
            let is_static = extract_flag(window, "is_static")
                .unwrap_or_else(|| modifiers.split_whitespace().any(|t| t == "static"));
            let is_native = extract_flag(window, "is_native")
                .unwrap_or_else(|| modifiers.split_whitespace().any(|t| t == "native"));
            let is_abstract = extract_flag(window, "is_abstract")
                .unwrap_or_else(|| modifiers.split_whitespace().any(|t| t == "abstract"));
            let exceptions = extract_attr(window, "exceptions")
                .map(|s| s.split(',').filter(|t| !t.is_empty()).map(str::to_owned).collect())
                .unwrap_or_default();
            result.entry(current.clone()).or_default().push(MethodMeta {
                name, descriptor, modifiers: bits, is_static, is_native, is_abstract, exceptions,
            });
        }
    }
    result
}

fn write_method_table(entries: &BTreeMap<String, Vec<MethodMeta>>) {
    let Ok(out_dir) = std::env::var("OUT_DIR") else { return };
    let mut out = String::from(
        "// 由 build.rs 自动生成：方法元数据表（binary name → 声明方法序列，声明序 = slot）。
         // 数据源：java_class! 块内 java_method / java_native 属性（方法声明元数据的唯一表达）。
         // 消费方：Class.getDeclaredMethod（class_impl.rs）、MethodHandleNatives.resolve 的
         // 方法/构造器 kind（method_handle_natives_impl.rs）。方法身份键是 (name, descriptor)
         // 二元组（重载语义）。modifiers 为 java.lang.reflect.Modifier 位集。请勿手改。

         pub struct MethodMeta {
             pub name:        &'static str,
             pub descriptor:  &'static str,
             pub modifiers:   i32,
             pub is_static:   bool,
             pub is_native:   bool,
             pub is_abstract: bool,
             pub exceptions:  &'static [&'static str],
         }

         pub static CLASS_METHODS: &[(&str, &[MethodMeta])] = &[
         ",
    );
    for (class, methods) in entries {
        if methods.is_empty() { continue; }
        out.push_str(&format!("    ({:?}, &[\n", class));
        for m in methods {
            let excs: Vec<String> = m.exceptions.iter().map(|e| format!("{:?}", e)).collect();
            out.push_str(&format!(
                "        MethodMeta {{ name: {:?}, descriptor: {:?}, modifiers: {:#06x}, is_static: {}, is_native: {}, is_abstract: {}, exceptions: &[{}] }},\n",
                m.name, m.descriptor, m.modifiers, m.is_static, m.is_native, m.is_abstract,
                excs.join(", "),
            ));
        }
        out.push_str("    ]),\n");
    }
    out.push_str("];
");
    let path = Path::new(&out_dir).join("method_table.rs");
    if let Err(e) = fs::write(&path, &out) {
        panic!("写 method_table.rs 失败: {e}");
    }
}

/// 类 → java.lang.reflect.Modifier 位集（Class.getModifiers 的数据源）。
/// 类级属性扫描：`#[access` 含 public → PUBLIC(0x1)；无 super_class 属性 →
/// 接口（INTERFACE|ABSTRACT，JVMS 语义：接口恒 abstract）——Object 除外
///（无父类但非接口）；final 位无类级属性源，不发射（无反射消费方依赖）。
fn scan_class_modifiers(roots: &[&Path]) -> BTreeMap<String, i32> {
    let mut result: BTreeMap<String, i32> = BTreeMap::new();
    for path in roots.iter().flat_map(|r| walk_rs_files(r)) {
        let content = fs::read_to_string(&path).unwrap_or_default();
        let mut current = String::new();
        let mut is_public = false;
        let mut has_super = false;
        // 类级 modifiers 属性词面（static 位只经此路径进表——成员嵌套类的
        // ACC_STATIC 在 InnerClasses 条目，发射侧已并入 modifiers 串）
        let mut mods_str = String::new();
        let mut touched = false;
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(name) = extract_attr_padded(trimmed, "binary_name") {
                if !current.is_empty() && touched {
                    let name = std::mem::take(&mut current);
                    let is_object = name == "java/lang/Object";
                    result.insert(name,
                        class_modifier_bits(is_public, has_super, is_object)
                            | modifier_bits(&mods_str));
                }
                current = name;
                is_public = false;
                has_super = false;
                mods_str.clear();
                touched = true;
                continue;
            }
            if current.is_empty() { continue; }
            if trimmed.starts_with("#[access") && trimmed.contains("public") {
                is_public = true;
            }
            if trimmed.starts_with("#[super_class") && !trimmed.contains("\"\"") {
                has_super = true;
            }
            if let Some(m) = extract_attr_padded(trimmed, "modifiers") {
                mods_str = m;
            }
        }
        if !current.is_empty() && touched {
            let is_object = current == "java/lang/Object";
            result.insert(current,
                class_modifier_bits(is_public, has_super, is_object)
                    | modifier_bits(&mods_str));
        }
    }
    result
}

/// public 位 + 接口位（无父类且非 java/lang/Object → INTERFACE|ABSTRACT）。
fn class_modifier_bits(is_public: bool, has_super_class: bool, is_object: bool) -> i32 {    let mut bits = 0i32;
    if is_public { bits |= 0x0001; }
    if !has_super_class && !is_object { bits |= 0x0200 | 0x0400; }
    bits
}

fn write_modifiers_table(entries: &BTreeMap<String, i32>) {
    let Ok(out_dir) = std::env::var("OUT_DIR") else { return };
    let mut out = String::from(
        "// 由 build.rs 自动生成：类修饰符表（binary name → Modifier 位集）。
         // 数据源：java_class! 块的 access / super_class 属性。接口（无 super_class，
         // Object 除外）恒含 INTERFACE|ABSTRACT。final 位无属性源不发射。
         // 消费方：Class.getModifiers（class_impl.rs）。请勿手改。

         pub static CLASS_MODIFIERS: &[(&str, i32)] = &[
",
    );
    for (name, mods) in entries {
        out.push_str(&format!("    ({:?}, {:#06x}),\n", name, mods));
    }
    out.push_str("];
");
    let path = Path::new(&out_dir).join("modifiers_table.rs");
    if let Err(e) = fs::write(&path, &out) {
        panic!("写 modifiers_table.rs 失败: {e}");
    }
}

/// record 类集（is_record 属性在场 = Record 属性在场，Class.isRecord 判据）。
fn scan_record_classes(roots: &[&Path]) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    for path in roots.iter().flat_map(|r| walk_rs_files(r)) {
        let content = fs::read_to_string(&path).unwrap_or_default();
        let mut current = String::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(name) = extract_attr_padded(trimmed, "binary_name") {
                current = name;
                continue;
            }
            if !current.is_empty()
                && trimmed.starts_with("#[is_record")
                && trimmed.contains("true")
            {
                result.insert(current.clone());
            }
        }
    }
    result
}

fn write_record_table(entries: &BTreeSet<String>) {
    let Ok(out_dir) = std::env::var("OUT_DIR") else { return };
    let mut out = String::from(
        "// 由 build.rs 自动生成：record 类集（binary name；Record 属性在场）。
         // 数据源：java_class! 块的 is_record 属性（classfile 的 Record 属性判定）。
         // 消费方：Class.isRecord（class_impl.rs）。请勿手改。

         pub static RECORD_CLASSES: &[&str] = &[
",
    );
    for name in entries {
        out.push_str(&format!("    {:?},\n", name));
    }
    out.push_str("];
");
    let path = Path::new(&out_dir).join("record_table.rs");
    if let Err(e) = fs::write(&path, &out) {
        panic!("写 record_table.rs 失败: {e}");
    }
}

fn write_hierarchy_table(entries: &BTreeMap<String, String>) {
    let Ok(out_dir) = std::env::var("OUT_DIR") else { return };
    let mut out = String::from(
        "// 由 build.rs 自动生成：类层次表（binary name → 全部超类型，含自身）。
         // 数据源：java_class! 块的 all_supertypes 属性（class 元数据推导）。
         // 消费方：Class.isAssignableFrom（class_impl.rs）。请勿手改。

         pub static CLASS_HIERARCHY: &[(&str, &[&str])] = &[
",
    );
    for (name, supers) in entries {
        let items: Vec<String> = supers.split(';')
            .filter(|s| !s.is_empty())
            .map(|s| format!("{:?}", s))
            .collect();
        out.push_str(&format!("    ({:?}, &[{}]),
", name, items.join(", ")));
    }
    out.push_str("];
");
    let path = Path::new(&out_dir).join("hierarchy_table.rs");
    if let Err(e) = fs::write(&path, out) {
        eprintln!("build.rs: cannot write hierarchy table: {}", e);
    }
}

fn scan_impls(src_dir: &Path) -> HashSet<String> {
    let mut result = HashSet::new();
    if !src_dir.exists() { return result; }
    for path in walk_rs_files(src_dir) {
        // K-4: 只处理 *_impl.rs 文件（共置的手写 impl 文件）
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if !stem.ends_with("_impl") { continue; }
        let content = fs::read_to_string(&path).unwrap_or_default();
        // 去掉 _impl 后缀还原为对应类的路径
        let impl_path = path.with_file_name(format!("{}.rs", &stem[..stem.len()-5]));
        let class = path_to_class(src_dir, &impl_path);
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with("pub fn ") || t.starts_with("pub unsafe fn ") {
                if let Some(name) = extract_fn_name(t) {
                    result.insert(format!("{}.{}", class, name));
                    result.insert(format!("{}.{}", class, to_camel(&name)));
                }
            }
            if t.starts_with("/// ") {
                if let Some(rest) = t.strip_prefix("/// ") {
                    if let Some(pos) = rest.find('.') {
                        let m_part = &rest[pos+1..];
                        if let Some(colon) = m_part.find(':') {
                            let mname = m_part[..colon].to_owned();
                            let cls   = rest[..pos].replace('/', ".");
                            result.insert(format!("{}.{}", cls, mname));
                            result.insert(format!("{}.{}", rest[..pos].to_owned(), mname));
                        }
                    }
                }
            }
        }
    }
    result
}

fn path_to_class(base: &Path, path: &Path) -> String {
    let rel = path.strip_prefix(base).unwrap_or(path);
    rel.with_extension("").to_string_lossy().replace('\\', "/")
}

fn extract_fn_name(line: &str) -> Option<String> {
    let after_fn = line.split("fn ").nth(1)?;
    let name: String = after_fn.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
    if name.is_empty() { None } else { Some(name) }
}

fn to_camel(snake: &str) -> String {
    let mut result = String::new();
    let mut cap_next = false;
    for ch in snake.chars() {
        if ch == '_' { cap_next = true; }
        else if cap_next { result.extend(ch.to_uppercase()); cap_next = false; }
        else { result.push(ch); }
    }
    result
}

fn load_status(path: &Path) -> BTreeMap<String, BTreeMap<String, String>> {
    if !path.exists() { return BTreeMap::new(); }
    parse_toml_status(&fs::read_to_string(path).unwrap_or_default())
}

fn parse_toml_status(s: &str) -> BTreeMap<String, BTreeMap<String, String>> {
    let mut result: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    let mut current = String::new();
    for line in s.lines() {
        let t = line.trim();
        if t.starts_with('[') && t.ends_with(']') && !t.starts_with("[[") {
            current = t[1..t.len()-1].to_owned();
        } else if !current.is_empty() {
            if let Some(eq) = t.find('=') {
                let key = t[..eq].trim().to_owned();
                let val = t[eq+1..].trim().trim_matches('"').to_owned();
                if !key.is_empty() && !val.is_empty() {
                    result.entry(current.clone()).or_default().insert(key, val);
                }
            }
        }
    }
    result
}

fn write_status(path: &Path, status: &BTreeMap<String, BTreeMap<String, String>>) {
    let mut out = String::new();
    out.push_str("# native_status.toml — 由 build.rs 自动维护\n");
    out.push_str("# status: \"implemented\" | \"needed\" | \"stub\" | \"not-needed\"\n\n");
    for (class, methods) in status {
        if methods.is_empty() { continue; }
        out.push_str(&format!("[{}]\n", class));
        for (method, s) in methods { out.push_str(&format!("{} = \"{}\",\n", method, s)); }
        out.push('\n');
    }
    fs::write(path, out).unwrap_or_else(|e| eprintln!("build.rs: cannot write native_status.toml: {}", e));
}

fn walk_rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    walk_dir(dir, &mut result);
    result
}

fn walk_dir(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() { walk_dir(&path, out); }
        else if path.extension().map(|e| e == "rs").unwrap_or(false) { out.push(path); }
    }
}
