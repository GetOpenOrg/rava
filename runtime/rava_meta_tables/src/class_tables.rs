//! 类级元数据表：类层次 / 直接父类 / 修饰符 / 嵌套 / 直接超接口 / record / <clinit> 与 sealed 许可子类型。

use super::*;

/// 类层次表扫描：java_class! 块内的裸属性行（`#[binary_name = "..."]` 与
/// `#[all_supertypes = "..."]` 各自独立成行，同一块内 binary_name 在前）。
/// all_supertypes 以 ';' 分隔、含类自身（attrs._compute_all_supertypes）。
pub(crate) fn scan_class_hierarchy(texts: &[&str]) -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();
    for content in texts.iter().copied() {
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

pub(crate) fn render_hierarchy_table(g: &mut Group, entries: &BTreeMap<String, String>) {
    // CLASS_HIERARCHY 行：类, [超类型]（含自身）——Class.isAssignableFrom
    let (s, p) = g.table("CLASS_HIERARCHY");
    for (name, supers) in entries {
        s.str(p, name);
        let items: Vec<&str> = supers.split(';').filter(|x| !x.is_empty()).collect();
        s.strs(p, &items);
    }
}

/// 类 → 直接父类（java_class! 块的 super_class 属性；接口无 super_class 属性）。
/// 消费方：Class.getSuperclass（class_impl.rs）。
pub(crate) fn scan_direct_super(texts: &[&str]) -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();
    for content in texts.iter().copied() {
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

pub(crate) fn render_direct_super_table(g: &mut Group, entries: &BTreeMap<String, String>) {
    // CLASS_DIRECT_SUPER 行：类, 直接父类（接口缺席）——Class.getSuperclass
    push_pairs(g, "CLASS_DIRECT_SUPER", entries);
}

/// (类, 串) 行
fn push_pairs(g: &mut Group, table: &str, entries: &BTreeMap<String, String>) {
    let (s, p) = g.table(table);
    for (k, v) in entries {
        s.str(p, k);
        s.str(p, v);
    }
}

/// 串集行
fn push_names(g: &mut Group, table: &str, names: &BTreeSet<String>) {
    let (s, p) = g.table(table);
    for n in names {
        s.str(p, n);
    }
}

/// 类 → java.lang.reflect.Modifier 位集（Class.getModifiers 的数据源）。
/// 类级属性扫描：`#[access` 含 public → PUBLIC(0x1)；无 super_class 属性 →
/// 接口（INTERFACE|ABSTRACT，JVMS 语义：接口恒 abstract）——Object 除外
///（无父类但非接口）；final 位无类级属性源，不发射（无反射消费方依赖）。
pub(crate) fn scan_class_modifiers(texts: &[&str]) -> BTreeMap<String, i32> {
    let mut result: BTreeMap<String, i32> = BTreeMap::new();
    for content in texts.iter().copied() {
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
                        class_modifier_bits(is_public && !member_restricted(&mods_str), has_super, is_object)
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
                class_modifier_bits(is_public && !member_restricted(&mods_str), has_super, is_object)
                    | modifier_bits(&mods_str));
        }
    }
    // 手写根类 java/lang/Object（object.rs，无 java_class! 块）：`public class Object`
    //（与方法表补行同源；缺行时 getModifiers 落 PUBLIC|FINAL|ABSTRACT 缺省，
    // ReflectionFactory 据 ABSTRACT 位走 InstantiationException 访问器）
    result.entry("java/lang/Object".to_owned()).or_insert(0x0001);
    result
}

/// 成员类修饰符（InnerClasses 条目）为 private / protected：顶层 access 的 public 位不作数
///（protected 成员类在类文件顶层记为 public）
fn member_restricted(mods: &str) -> bool {
    mods.split_whitespace().any(|t| t == "private" || t == "protected")
}

/// public 位 + 接口位（无父类且非 java/lang/Object → INTERFACE|ABSTRACT）。
pub(crate) fn class_modifier_bits(is_public: bool, has_super_class: bool, is_object: bool) -> i32 {    let mut bits = 0i32;
    if is_public { bits |= 0x0001; }
    if !has_super_class && !is_object { bits |= 0x0200 | 0x0400; }
    bits
}

pub(crate) fn render_modifiers_table(g: &mut Group, entries: &BTreeMap<String, i32>) {
    // CLASS_MODIFIERS 行：类, Modifier 位集（接口恒含 INTERFACE|ABSTRACT）——Class.getModifiers
    let (s, p) = g.table("CLASS_MODIFIERS");
    for (name, mods) in entries {
        s.str(p, name);
        s.i32(*mods);
    }
}

/// 嵌套元数据（FS-R R1）：类 → (外层类, 简单名, 本类 InnerClasses 条目在场, 封闭方法三元组)。
/// 数据源：`inner_classes`（本类自身条目：`inner:outer:simple:flags`）与 `enclosing_method`
/// （`class:name:desc`）属性。消费方：Class.getDeclaringClass0 / getSimpleBinaryName0 /
/// getEnclosingMethod0（HotSpot 同源：InnerClasses / EnclosingMethod 属性）。
pub(crate) struct NestMeta {
    outer: String,
    simple: String,
    self_entry: bool,
    enclosing: Option<(String, String, String)>,
    /// 本类声明的成员类：InnerClasses 中 outer 为本类、inner 非本类的条目（属性序）
    members: Vec<String>,
}

pub(crate) fn scan_nest_meta(texts: &[&str]) -> BTreeMap<String, NestMeta> {
    let mut result: BTreeMap<String, NestMeta> = BTreeMap::new();
    for content in texts.iter().copied() {
        let mut current = String::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(name) = extract_attr_padded(trimmed, "binary_name") {
                current = name;
                continue;
            }
            if current.is_empty() { continue; }
            if let Some(v) = extract_attr_padded(trimmed, "inner_classes") {
                for ent in v.split(';') {
                    let parts: Vec<&str> = ent.split(':').collect();
                    if parts.len() >= 3 && parts[1] == current && parts[0] != current {
                        result.entry(current.clone()).or_insert(NestMeta {
                            outer: String::new(), simple: String::new(), self_entry: false, enclosing: None, members: vec![],
                        }).members.push(parts[0].to_owned());
                    }
                    if parts.len() >= 3 && parts[0] == current {
                        let e = result.entry(current.clone()).or_insert(NestMeta {
                            outer: String::new(), simple: String::new(), self_entry: false, enclosing: None, members: vec![],
                        });
                        e.outer = parts[1].to_owned();
                        e.simple = parts[2].to_owned();
                        e.self_entry = true;
                    }
                }
            }
            if let Some(v) = extract_attr_padded(trimmed, "enclosing_method") {
                let parts: Vec<&str> = v.splitn(3, ':').collect();
                if parts.len() == 3 {
                    let e = result.entry(current.clone()).or_insert(NestMeta {
                        outer: String::new(), simple: String::new(), self_entry: false, enclosing: None, members: vec![],
                    });
                    e.enclosing = Some((parts[0].to_owned(), parts[1].to_owned(), parts[2].to_owned()));
                }
            }
        }
    }
    result
}

pub(crate) fn render_nest_table(g: &mut Group, entries: &BTreeMap<String, NestMeta>) {
    // CLASS_NEST 行：类, 外层类, 简单名, 标志（bit0 本类条目在场 / bit1 有封闭方法）, [封闭方法 类, 名, 描述符],
    // [成员类]——Class.getDeclaringClass0 / getSimpleBinaryName0 / getEnclosingMethod0
    let (s, p) = g.table("CLASS_NEST");
    for (name, m) in entries {
        s.str(p, name);
        s.str(p, &m.outer);
        s.str(p, &m.simple);
        s.u32(m.self_entry as u32 | (m.enclosing.is_some() as u32) << 1);
        if let Some((c, n, d)) = &m.enclosing {
            s.str(p, c);
            s.str(p, n);
            s.str(p, d);
        }
        s.strs(p, &m.members);
    }
}

/// 直接超接口表（class 文件 interfaces 项，声明序）：数据源 `interfaces` 属性。
/// 消费方：Class.getInterfaces0（HotSpot 同源）。
pub(crate) fn scan_class_interfaces(texts: &[&str]) -> BTreeMap<String, Vec<String>> {
    let mut result: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for content in texts.iter().copied() {
        let mut current = String::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(name) = extract_attr_padded(trimmed, "binary_name") {
                current = name;
                continue;
            }
            if current.is_empty() { continue; }
            if let Some(v) = extract_attr_padded(trimmed, "interfaces") {
                let list: Vec<String> = v.split(',').map(str::trim)
                    .filter(|x| !x.is_empty()).map(str::to_owned).collect();
                if !list.is_empty() {
                    result.insert(current.clone(), list);
                }
            }
        }
    }
    result
}

pub(crate) fn render_interfaces_table(g: &mut Group, entries: &BTreeMap<String, Vec<String>>) {
    // CLASS_INTERFACES 行：类, [直接超接口]（声明序）——Class.getInterfaces0
    let (s, p) = g.table("CLASS_INTERFACES");
    for (name, list) in entries {
        s.str(p, name);
        s.strs(p, list);
    }
}

/// record 类集（is_record 属性在场 = Record 属性在场，Class.isRecord 判据）。
pub(crate) fn scan_record_classes(texts: &[&str]) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    for content in texts.iter().copied() {
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

/// record 组件表（record_components 属性：`名:描述符:Signature`，`|` 分隔，声明序）。
pub(crate) fn scan_record_components(texts: &[&str]) -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();
    for content in texts.iter().copied() {
        let mut current = String::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(name) = extract_attr_padded(trimmed, "binary_name") {
                current = name;
                continue;
            }
            if !current.is_empty() {
                if let Some(v) = extract_attr_padded(trimmed, "record_components") {
                    result.insert(current.clone(), v);
                }
            }
        }
    }
    result
}

pub(crate) fn render_record_table(g: &mut Group, entries: &BTreeSet<String>, components: &BTreeMap<String, String>) {
    // RECORD_CLASSES 行：类（Record 属性在场）——Class.isRecord
    push_names(g, "RECORD_CLASSES", entries);
    // RECORD_COMPONENTS 行：record 类, [(名, 描述符, Signature)]——Class.getRecordComponents0
    let (s, p) = g.table("RECORD_COMPONENTS");
    for (cls, spec) in components {
        s.str(p, cls);
        let comps: Vec<&str> = spec.split('|').filter(|c| !c.is_empty()).collect();
        s.len(comps.len());
        for comp in comps {
            let mut it = comp.splitn(3, ':');
            for _ in 0..3 {
                s.str(p, it.next().unwrap_or(""));
            }
        }
    }
}

/// 布尔类属性为 true 的类集：`has_clinit`（声明了 `<clinit>`）、`is_hidden`（lambda 调用点隐藏类，
/// `hidden_class!` 声明块）。
pub(crate) fn scan_flag_classes(texts: &[&str], key: &str) -> BTreeSet<String> {
    let tag = format!("#[{key}");
    let mut result = BTreeSet::new();
    for content in texts.iter().copied() {
        let mut current = String::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(name) = extract_attr_padded(trimmed, "binary_name") {
                current = name;
                continue;
            }
            if !current.is_empty() && trimmed.starts_with(&tag) && trimmed.contains("true") {
                result.insert(current.clone());
            }
        }
    }
    result
}

/// 类级字符串属性表：类 → 属性值（键须以 `#[` 前缀出现）。用于 permitted_subclasses（sealed 许可子类型，
/// `,` 分隔，声明序）、nest_members（NestMembers 属性，`,` 分隔，声明序）、class_access_flags（类文件
/// access_flags 原值，十进制）。
pub(crate) fn scan_class_attr(texts: &[&str], key: &str) -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();
    for content in texts.iter().copied() {
        let mut current = String::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(name) = extract_attr_padded(trimmed, "binary_name") {
                current = name;
                continue;
            }
            if !current.is_empty() {
                if let Some(v) = extract_attr_padded(trimmed, key) {
                    result.insert(current.clone(), v);
                }
            }
        }
    }
    result
}

/// 类 → 名单（`,` 分隔属性值）行
fn push_name_lists(g: &mut Group, table: &str, lists: &BTreeMap<String, String>) {
    let (s, p) = g.table(table);
    for (cls, list) in lists {
        s.str(p, cls);
        let items: Vec<&str> = list.split(',').filter(|c| !c.is_empty()).collect();
        s.strs(p, &items);
    }
}

/// 类文件级元数据（VM 注入的类信息）：
/// - CLINIT_CLASSES：声明了 <clinit> 的类（has_clinit 属性）——ObjectStreamClass.hasStaticInitializer；
/// - HIDDEN_CLASSES：lambda 调用点隐藏类（is_hidden 属性）——Class.isHidden / 镜像名 / forName 排除；
/// - PERMITTED_SUBCLASSES：sealed 类的许可子类型——Class.getPermittedSubclasses0；
/// - NEST_MEMBERS：嵌套宿主的 NestMembers 属性——Class.getNestMembers0；
/// - CLASS_ACCESS_FLAGS：类文件 access_flags 原值——Class.getClassAccessFlagsRaw0；
/// - CLASS_SOURCE_FILE：SourceFile 属性——StackTraceElement.initStackTraceElement 的 fileName；
/// - CLASS_DEFINING_LOADER：定义加载器（app / platform；引导加载器的类不在表中）——Class.classLoader 的读取钩子。
pub(crate) fn render_class_meta_table(g: &mut Group, clinit: &BTreeSet<String>, hidden: &BTreeSet<String>,
                                     permitted: &BTreeMap<String, String>, nest_members: &BTreeMap<String, String>,
                                     access: &BTreeMap<String, String>, source: &BTreeMap<String, String>,
                                     loaders: &BTreeMap<String, String>) {
    push_names(g, "CLINIT_CLASSES", clinit);
    push_names(g, "HIDDEN_CLASSES", hidden);
    push_name_lists(g, "PERMITTED_SUBCLASSES", permitted);
    push_name_lists(g, "NEST_MEMBERS", nest_members);
    let (s, p) = g.table("CLASS_ACCESS_FLAGS");
    for (cls, v) in access {
        if let Ok(bits) = v.trim().parse::<i32>() {
            s.str(p, cls);
            s.i32(bits);
        }
    }
    push_pairs(g, "CLASS_SOURCE_FILE", source);
    push_pairs(g, "CLASS_DEFINING_LOADER", loaders);
}
