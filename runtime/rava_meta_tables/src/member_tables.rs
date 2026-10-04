//! 成员元数据表：字段表与方法表（声明序 = slot）。

use super::*;

/// 单个声明字段的元数据（`java_field` 属性行的结构化形态）。
/// modifiers 为 java.lang.reflect.Modifier 位集（public=0x1 / private=0x2 /
/// protected=0x4 / static=0x8 / final=0x10 / volatile=0x40 / transient=0x80 …）；
/// constant 是 ConstantValue 属性的整数值（仅整型常量收录，字符串等形态缺席）。
pub(crate) struct FieldMeta {
    name:       String,
    descriptor: String,
    modifiers:  i32,
    is_static:  bool,
    constant:   Option<i64>,
    annotations: Vec<u8>,
    /// Signature 属性（泛型签名，Field.signature / getGenericType 数据源）；无则空串
    signature:  String,
}

/// 字段元数据扫描：java_class! 块内 java_field 属性行（每字段独立成行，
/// `#[cfg_attr(any(), java_field(name = "x", descriptor = "I", access =
/// "private", modifiers = "static final", is_static = true))]`）。类上下文
/// 与层次表同源（同块内 binary_name 在前）。声明顺序保留（Field.slot 语义）。
/// 消费方：Class.getDeclaredField / Field.get/set（class_impl.rs / field_impl.rs）。
pub(crate) fn scan_class_fields(texts: &[&str]) -> BTreeMap<String, Vec<FieldMeta>> {
    let mut result: BTreeMap<String, Vec<FieldMeta>> = BTreeMap::new();
    for content in texts.iter().copied() {
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
            let annotations = extract_key(window, "raw_annotations").map(|h| hex_bytes(&h)).unwrap_or_default();
            let signature = extract_key(window, "generic_signature").unwrap_or_default();
            result.entry(current.clone()).or_default().push(FieldMeta {
                name, descriptor, modifiers: bits, is_static, constant, annotations, signature,
            });
        }
    }
    result
}

pub(crate) fn render_field_table(entries: &BTreeMap<String, Vec<FieldMeta>>) -> String {
    let mut out = String::from(
        "// 由生成器（rava_meta_tables）生成：字段元数据表（binary name → 声明字段序列，声明序 = slot）。
         // 数据源：java_class! 块内 java_field 属性（字段声明元数据的唯一表达，规则四）。
         // 消费方：Class.getDeclaredField / Field.get/set（class_impl.rs / field_impl.rs）。
         // modifiers 为 java.lang.reflect.Modifier 位集；constant 为 ConstantValue 整数值。
         // 请勿手改。


         #[export_name = \"__java_meta_CLASS_FIELDS\"] pub static CLASS_FIELDS: &[(&str, &[FieldMeta])] = &[
",
    );
    for (class, fields) in entries {
        if fields.is_empty() { continue; }
        out.push_str(&format!("    ({:?}, &[\n", class));
        for f in fields {
            out.push_str(&format!(
                "        FieldMeta {{ name: {:?}, descriptor: {:?}, modifiers: {:#06x}, is_static: {}, constant: {}, annotations: &{:?}, signature: {:?} }},\n",
                f.name, f.descriptor, f.modifiers, f.is_static,
                match f.constant { Some(v) => format!("Some({}i64)", v), None => "None".to_owned() },
                f.annotations, f.signature,
            ));
        }
        out.push_str("    ]),\n");
    }
    out.push_str("];
");
    out
}

/// 单个声明方法的元数据（java_method / java_native 属性行的结构化形态）。
/// modifiers 为 java.lang.reflect.Modifier 位集（方法侧：synchronized=0x20 /
/// varargs=0x80 / native=0x100 / abstract=0x400）；exceptions 为 throws 子句
/// 的 binary name 列表（Method.getExceptionTypes 的数据源）；name 含
/// `<init>` / `<clinit>` 行（构造器/类初始化器的声明记录，消费方按 JDK
/// 语义过滤——getDeclaredMethods 不见二者、getDeclaredConstructors 取
/// `<init>`）。
pub(crate) struct MethodMeta {
    name:       String,
    descriptor: String,
    modifiers:  i32,
    is_static:  bool,
    is_native:  bool,
    is_abstract: bool,
    exceptions: Vec<String>,
    /// FS-R R4b：RuntimeVisibleAnnotations / RuntimeVisibleParameterAnnotations / AnnotationDefault 原始属性体
    annotations: Vec<u8>,
    param_annotations: Vec<u8>,
    annotation_default: Vec<u8>,
    /// Signature 属性（泛型签名，Method / Constructor.signature 数据源）；无则空串
    signature: String,
    /// 继承成员行（`inherited_from`：超类型声明、展平到本类块供分派 / MethodHandle 解析；
    /// `declared_by`：复制进本类的接口 default 体 / 未覆盖的超类虚方法体，声明者为该类型）；
    /// 不属本类声明面（getDeclaredMethods 过滤）。
    inherited: bool,
    /// 复制进本类的方法体的声明类型（`declared_by`）；本类声明 / 继承转发行为空串
    declared_by: String,
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
pub(crate) fn scan_class_methods(texts: &[&str]) -> BTreeMap<String, Vec<MethodMeta>> {
    let mut result: BTreeMap<String, Vec<MethodMeta>> = BTreeMap::new();
    for content in texts.iter().copied() {
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
            let raw = |k: &str| extract_key(window, k).map(|h| hex_bytes(&h)).unwrap_or_default();
            result.entry(current.clone()).or_default().push(MethodMeta {
                name, descriptor, modifiers: bits, is_static, is_native, is_abstract, exceptions,
                annotations: raw("raw_annotations"),
                param_annotations: raw("raw_param_annotations"),
                annotation_default: raw("raw_annotation_default"),
                signature: extract_key(window, "generic_signature").unwrap_or_default(),
                // 继承转发行与复制进本类的方法体（`declared_by`）都不是本类声明
                inherited: extract_key(window, "inherited_from").is_some()
                    || extract_key(window, "declared_by").is_some(),
                declared_by: extract_key(window, "declared_by").unwrap_or_default(),
            });
        }
    }
    result
}

/// 手写 `java/lang/Object`（object.rs，Arch-4 ObjectVTable 根，无 java_class! 块）
/// 的方法表补行：JLS §4.3.2 / JVMS §2.9——Object 恰有一个 public 无参构造器，
/// 反射面（getConstructors / getDeclaredConstructors / getConstructor()）须可见。
/// 构造体由 reflect_dispatch 的 Object `<init>` 臂承载。其余成员（JLS §4.3.2 的
/// hashCode / equals / toString / getClass / clone / notify* / wait* / finalize）按
/// JDK 21 声明（修饰符 / 描述符 / throws）补行：`Object.class.getMethod` 与
/// getMethods() 的继承面（动态代理的 Object 方法转发、反射列举）由此可见。
pub(crate) fn with_object_ctor_row(mut methods: BTreeMap<String, Vec<MethodMeta>>)
    -> BTreeMap<String, Vec<MethodMeta>>
{
    let rows = methods.entry("java/lang/Object".to_owned()).or_default();
    // (name, descriptor, modifiers, is_native, throws)：PUBLIC 0x1 / PRIVATE 0x2 / PROTECTED 0x4 /
    // FINAL 0x10 / NATIVE 0x100。顺序 = HotSpot 方法表序（JDK 21 `Object.class.getDeclaredMethods()`
    // 实测：finalize, wait0, equals, toString, hashCode, getClass, clone, notify, notifyAll,
    // wait(J), wait(JI), wait()）——getMethods / getDeclaredMethods 的输出顺序与 JVM 一致
    //（ListMethods 实证）；构造器不在方法序列中，置首位不影响方法序
    const OBJECT_MEMBERS: &[(&str, &str, i32, bool, &[&str])] = &[
        ("<init>", "()V", 0x0001, false, &[]),
        ("finalize", "()V", 0x0004, false, &["java/lang/Throwable"]),
        ("wait0", "(J)V", 0x0112, true, &["java/lang/InterruptedException"]),
        ("equals", "(Ljava/lang/Object;)Z", 0x0001, false, &[]),
        ("toString", "()Ljava/lang/String;", 0x0001, false, &[]),
        ("hashCode", "()I", 0x0101, true, &[]),
        ("getClass", "()Ljava/lang/Class;", 0x0111, true, &[]),
        ("clone", "()Ljava/lang/Object;", 0x0104, true, &["java/lang/CloneNotSupportedException"]),
        ("notify", "()V", 0x0111, true, &[]),
        ("notifyAll", "()V", 0x0111, true, &[]),
        ("wait", "(J)V", 0x0011, false, &["java/lang/InterruptedException"]),
        ("wait", "(JI)V", 0x0011, false, &["java/lang/InterruptedException"]),
        ("wait", "()V", 0x0011, false, &["java/lang/InterruptedException"]),
    ];
    for (i, (name, desc, mods, native, throws)) in OBJECT_MEMBERS.iter().enumerate() {
        if rows.iter().any(|m| m.name == *name && m.descriptor == *desc) {
            continue;
        }
        rows.insert(i.min(rows.len()), MethodMeta {
            name: (*name).to_owned(), descriptor: (*desc).to_owned(),
            modifiers: *mods, is_static: false, is_native: *native, is_abstract: false,
            exceptions: throws.iter().map(|e| (*e).to_owned()).collect(),
            annotations: Vec::new(), param_annotations: Vec::new(), annotation_default: Vec::new(),
            signature: String::new(), inherited: false, declared_by: String::new(),
        });
    }
    methods
}

/// 栈帧方法的元数据（`vm_stack` 帧的修饰位与注解）：随行表方法项发射，不依赖成员表
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameMeta {
    pub modifiers: i32,
    pub is_static: bool,
    pub is_native: bool,
    /// RuntimeVisibleAnnotations 原始属性体（`@Hidden` / `@CallerSensitive` / `@LambdaForm.Compiled` 判定）
    pub annotations: Vec<u8>,
}

/// 帧方法元数据索引：全部生成文本的方法属性行（含手写根类 Object 的成员行）
pub struct FrameIndex(BTreeMap<String, Vec<MethodMeta>>);

impl FrameIndex {
    pub fn new(texts: &[&str]) -> Self {
        FrameIndex(with_object_ctor_row(scan_class_methods(texts)))
    }

    /// 帧归属类自身声明的行；方法体复制进他类（`declared_by`）而归属类无表项时取宿主类的行
    pub fn get(&self, class: &str, name: &str, descriptor: &str, host: &str) -> Option<FrameMeta> {
        let find = |c: &str, own: bool| {
            self.0.get(c)?.iter().find(|m| (!own || !m.inherited) && m.name == name && m.descriptor == descriptor)
        };
        let m = find(class, true).or_else(|| (!host.is_empty()).then(|| find(host, false)).flatten())?;
        Some(FrameMeta { modifiers: m.modifiers, is_static: m.is_static, is_native: m.is_native, annotations: m.annotations.clone() })
    }
}

pub(crate) fn render_method_table(entries: &BTreeMap<String, Vec<MethodMeta>>) -> String {
    let mut out = String::from(
        "// 由生成器（rava_meta_tables）生成：方法元数据表（binary name → 声明方法序列，声明序 = slot）。
         // 数据源：java_class! 块内 java_method / java_native 属性（方法声明元数据的唯一表达）。
         // 消费方：Class.getDeclaredMethod（class_impl.rs）、MethodHandleNatives.resolve 的
         // 方法/构造器 kind（method_handle_natives_impl.rs）。方法身份键是 (name, descriptor)
         // 二元组（重载语义）。modifiers 为 java.lang.reflect.Modifier 位集。请勿手改。


         #[export_name = \"__java_meta_CLASS_METHODS\"] pub static CLASS_METHODS: &[(&str, &[MethodMeta])] = &[
         ",
    );
    for (class, methods) in entries {
        if methods.is_empty() { continue; }
        out.push_str(&format!("    ({:?}, &[\n", class));
        for m in methods {
            let excs: Vec<String> = m.exceptions.iter().map(|e| format!("{:?}", e)).collect();
            out.push_str(&format!(
                "        MethodMeta {{ name: {:?}, descriptor: {:?}, modifiers: {:#06x}, is_static: {}, is_native: {}, is_abstract: {}, exceptions: &[{}], annotations: &{:?}, param_annotations: &{:?}, annotation_default: &{:?}, signature: {:?}, inherited: {}, declared_by: {:?} }},\n",
                m.name, m.descriptor, m.modifiers, m.is_static, m.is_native, m.is_abstract,
                excs.join(", "), m.annotations, m.param_annotations, m.annotation_default, m.signature, m.inherited, m.declared_by,
            ));
        }
        out.push_str("    ]),\n");
    }
    out.push_str("];
");
    out
}
