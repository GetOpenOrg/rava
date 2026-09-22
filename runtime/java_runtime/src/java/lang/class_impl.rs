use crate::prelude::*;
use super::*;
use super::reflect::Field;
use std::cell::RefCell;
use std::collections::HashMap;

impl Class {
    /// native registerNatives：HotSpot 绑定 JNI 入口；原生二进制无此需要。
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        Ok(())
    }

    /// native getPrimitiveClass(String)：每个基本类型名对应唯一的 Class 对象
    /// （`Integer.TYPE == int.class` 的身份语义），首次请求时创建。
    #[jvm_native]
    pub fn getPrimitiveClass(name: String) -> Result<Class> {
        thread_local! {
            static PRIMITIVES: RefCell<HashMap<std::string::String, Class>> = RefCell::new(HashMap::new());
        }
        let key = format!("{}", name);
        Ok(PRIMITIVES.with(|cache| {
            Clone::clone(cache.borrow_mut().entry(key).or_insert_with(|| {
                let mut c = Class::default();
                c._init_not_null();
                c.__set_name(Clone::clone(&name));
                c
            }))
        }))
    }

    /// `ldc` 装载的类字面量（`X.class` / `X[].class`）：每个 binary name 对应唯一
    /// 的 Class 对象，首次请求时创建并在线程内缓存 —— 保证 `X.class == X.class`
    /// 的身份语义（JVMS §5.1 运行时常量池的类引用只解析一次）。
    ///
    /// `getName()` 返回 Java 形式的二进制名：斜线换点（`java/util/List` →
    /// `java.util.List`），数组类型保持 JVM 描述符形态（`[Ljava.lang.String;`）。
    /// isAssignableFrom 的层次查询在运行时经 build.rs 生成的层次表进行，
    /// 此处不再携带/登记超类型数据。
    pub fn for_class(binary_name: String) -> Class {
        thread_local! {
            static CLASSES: RefCell<HashMap<std::string::String, Class>> =
                RefCell::new(HashMap::new());
        }
        let key = format!("{}", binary_name);
        CLASSES.with(|cache| {
            Clone::clone(cache.borrow_mut().entry(key.clone()).or_insert_with(|| {
                let mut c = Class::default();
                c._init_not_null();
                c.__set_name(String::from(key.replace('/', ".").as_str()));
                c
            }))
        })
    }

    /// `getDeclaredField(String)`：按名取本类声明字段（反射族静态注册表路线）。
    ///
    /// 字段元数据表由 build.rs 从 `java_class!` 块的 java_field 属性生成
    /// （OUT_DIR/field_table.rs，与层次表同源——字段声明元数据只在 Rust 侧
    /// 表达一份，规则四）。命中 → 构造 Field 携带完整声明元数据：clazz=本类、
    /// name、modifiers=声明修饰位、slot=声明序（getDeclaredFields0 语义）、
    /// type=描述符对应 Class。对象身份按「查询即构造」（Unsafe 族按
    /// (声明类, 字段名) 消费，见 unsafe__impl 的不透明 id 协议）。
    /// 未命中（含类不在表内：闭包外类、数组、基本类型）→ 抛
    /// NoSuchFieldException（真实异常对象、消息=字段名，与 JDK
    /// Class.getDeclaredField0 行为一致），可被 java_try 捕获。
    #[jvm_boundary(upcalls = "java/lang/NoSuchFieldException.<init>:(Ljava/lang/String;)V")]
    pub fn getDeclaredField(&self, name: String) -> Result<Field> {
        let cls_key = format!("{}", self.__get_name()).replace('.', "/");
        let query = format!("{}", name);
        let hit = __fields::CLASS_FIELDS.iter()
            .find(|(n, _)| *n == cls_key)
            .and_then(|(_, fs)| fs.iter().enumerate().find(|(_, f)| f.name == query));
        let Some((slot, meta)) = hit else {
            return match crate::java::lang::NoSuchFieldException::new_str(Clone::clone(&name)) {
                Ok(ex) => Err(ex.into()),
                // 构造器自身失败（<clinit> 等）时传播嵌套异常（与 vm_throw 同序）
                Err(nested) => Err(nested),
            };
        };
        let mut f = Field::default();
        f._init_not_null();
        f.__set_clazz(Clone::clone(self));
        f.__set_name(Clone::clone(&name));
        f.__set_modifiers(meta.modifiers);
        f.__set_slot(slot as i32);
        f.__set_type_(class_for_descriptor(&meta.descriptor));
        Ok(f)
    }

    /// native `Class.isArray()`：数组类判定。数组类的名字是 JVM 描述符形态
    /// （`[I`、`[Ljava.lang.String;`——for_class 的存储形态），首字符 `[`
    /// 即数组（JLS：数组的运行时类是 JVM 创建的 Array 类型）。
    /// 消费方：MethodHandles 链的 checkSymbolicClass / findVarHandle 类型检查。
    #[jvm_native]
    pub fn isArray(&self) -> Result<bool> {
        Ok(format!("{}", self.__get_name()).starts_with('['))
    }

    /// native `Class.isPrimitive()`：基本类型类判定。基本类型的 Class 经
    /// getPrimitiveClass 创建，名字是基本类型字面量（int / boolean / …，
    /// 无包前缀）；按显式名单判定（九种，JLS §4.2），非基本类型（含数组、
    /// void 的 Class 缺席形态）→ false。
    /// 消费方：VarHandles.makeFieldHandle 的字段类型分派链。
    #[jvm_native]
    pub fn isPrimitive(&self) -> Result<bool> {
        let name = format!("{}", self.__get_name());
        Ok(matches!(name.as_str(),
            "boolean" | "byte" | "char" | "short" | "int" | "long" | "float" | "double"))
    }

    /// native `Class.getComponentType()`：数组类返回元素 Class，非数组返回 null。
    ///
    /// 数组类的名字是 JVM 描述符形态（`[I`、`[Ljava.lang.String;`、`[[I`——
    /// for_class 的存储形态，内层点分隔）。解析：剥一层 `[` 后按首字符分派
    /// ——`L` → 引用类、`[` → 元素仍是数组（描述符形态递归）、基本类型字符 →
    /// getPrimitiveClass 的唯一实例（`int.class` 身份语义）。非数组（闭包外类
    /// / 接口 / 基本类型自身）→ null（JLS：componentType 只对数组类非 null）。
    /// 消费方：`Arrays.copyOf(orig, len, newType)` 链（`Array.newInstance(
    /// newType.getComponentType(), n)`，TestCollectionFactory 的 toArray 卡点）、
    /// MethodHandles 的数组访问器构造。
    pub fn __impl_getComponentType(&self) -> Result<Class> {
        let name = format!("{}", self.__get_name());
        let Some(rest) = name.strip_prefix('[') else {
            return Ok(Class::default());
        };
        Ok(class_for_descriptor(rest))
    }

    /// 反射族内部：按字段名查本类声明元数据（描述符 / static 标志 / 修饰位 /
    /// ConstantValue 整数值）。getDeclaredField 与 Field.get/set（reflect 邻域
    /// 伴生 field_impl.rs）共用同一张 build.rs 字段表；未声明 → None。
    pub fn __declared_field_meta(&self, name: &str) -> Option<(&'static str, bool, i32, Option<i64>)> {
        let cls_key = format!("{}", self.__get_name()).replace('.', "/");
        __fields::CLASS_FIELDS.iter()
            .find(|(n, _)| *n == cls_key)
            .and_then(|(_, fs)| fs.iter().find(|f| f.name == name))
            .map(|f| (f.descriptor, f.is_static, f.modifiers, f.constant))
    }

    /// 反射族内部：按 (name, descriptor) 二元组查本类声明方法元数据（修饰位 /
    /// static / native / abstract）。方法重载使 name 不唯一，命中判定必须
    /// 名与描述符配对（JDK getDeclaredMethod 语义——参数类型还原成描述符后
    /// 比对）。消费方：MethodHandleNatives.resolve 的方法/构造器 kind
    /// （method_handle_natives_impl.rs，MemberName 解析内核）；未声明 → None。
    pub fn __declared_method_meta(&self, name: &str, descriptor: &str) -> Option<(i32, bool, bool, bool)> {
        let cls_key = format!("{}", self.__get_name()).replace('.', "/");
        __methods::CLASS_METHODS.iter()
            .find(|(n, _)| *n == cls_key)
            .and_then(|(_, ms)| ms.iter().find(|m| m.name == name && m.descriptor == descriptor))
            .map(|m| (m.modifiers, m.is_static, m.is_native, m.is_abstract))
    }

    /// 反射族内部：描述符 → Class 对象（class_for_descriptor 的类型面）。
    /// MethodHandleNatives.resolve 的字段 kind 类型核对共用（描述符还原的
    /// Class 与 MemberName 携带的 Class 按名相等）。
    pub fn __class_for_descriptor(desc: &str) -> Class {
        class_for_descriptor(desc)
    }

    /// `Class.isAssignableFrom(Class)`：`X.isAssignableFrom(Y)` 即 Y 的类型闭包
    /// 包含 X（含 X == Y）。超类型闭包由生成器在类字面量处静态推导
    /// （父类链 + 全部接口，见 codegen ldc 类字面量的第二参数），随 `for_class`
    /// 登记到线程内侧表；未登记的目标（闭包外类、数组、基本类型）仅同名相等。
    pub fn isAssignableFrom(&self, cls: Class) -> Result<bool> {
        let self_name = format!("{}", self.__get_name()).replace('.', "/");
        let cls_name = format!("{}", cls.__get_name()).replace('.', "/");
        if self_name == cls_name {
            return Ok(true);
        }
        // cls 的超类型闭包（含自身）包含 self 即可赋值。层次表由 build.rs 从
        // java_class! 的 all_supertypes 属性生成（class 元数据的唯一表达）；
        // 未生成/接口载体的 cls 不在表中，退化为同名相等（与 JVM 语义的差异
        // 仅影响"参数侧从未进入闭包"的场景）。
        Ok(__hierarchy::CLASS_HIERARCHY.iter()
            .find(|(n, _)| *n == cls_name)
            .map(|(_, supers)| supers.iter().any(|s| *s == self_name))
            .unwrap_or(false))
    }

    /// `Class.getName()`：返回类对象的二进制名（Java 形式，点分隔）。
    ///
    /// `java/lang/Class` 只作为类型存根进入闭包，字节码版的 `getName()` 是 stub，
    /// 故在此手写。名字来源有二：类字面量经 `for_class` 写入、基本类型经
    /// `getPrimitiveClass` 写入（二者落到同一个 private name 字段）。
    pub fn __impl_getName(&self) -> Result<String> {
        Ok(Clone::clone(&self.__get_name()))
    }

    /// native `Class.getSuperclass()`：直接父类的 Class 对象。
    ///
    /// 查询经 build.rs 从 `java_class!` 的 super_class 属性生成的直接父类表
    /// （与 isAssignableFrom 的层次表同源）。Object 自身 / 接口 / 基本类型 /
    /// 未登记类（闭包外、数组）→ null（JLS 对接口与 Object 返回 null 的语义）。
    #[jvm_native]
    pub fn getSuperclass(&self) -> Result<Class> {
        let name = format!("{}", self.__get_name()).replace('.', "/");
        match __direct_super::CLASS_DIRECT_SUPER.iter().find(|(n, _)| *n == name) {
            // for_class 的缓存键是斜线形态（与 ldc 类字面量同一调用形态）——身份语义
            //（`zuper == Enum.class`）依赖同一缓存条目
            Some((_, sup)) => Ok(Class::for_class(String::from(*sup))),
            None => Ok(Class::default()),
        }
    }

    /// `Class.getSimpleName()`：简单名。顶层类取最后一个 `.` 之后的段，
    /// 嵌套类再取最后一个 `$` 之后的段（JDK getSimpleBinaryName 的常见形态）；
    /// 数组 / 匿名类等罕见形态按现状原样返回，按需再补。
    pub fn __impl_getSimpleName(&self) -> Result<String> {
        let full = format!("{}", self.__get_name());
        let simple = full.rsplit('.').next().unwrap_or("");
        let simple = simple.rsplit('$').next().unwrap_or("");
        Ok(String::from(simple))
    }

    /// `Class.desiredAssertionStatus()`：该类的断言是否启用。
    ///
    /// 原生二进制没有 `-ea` / `-da` 开关，断言恒为禁用（即 JVM 的默认行为），
    /// 故恒返回 false。JDK 大量类的 `<clinit>` 用 `!X.class.desiredAssertionStatus()`
    /// 初始化 `$assertionsDisabled`，此方法是那条路径的必经之地。
    pub fn __impl_desiredAssertionStatus(&self) -> Result<bool> {
        Ok(false)
    }
}

/// 描述符 → Class 对象（getDeclaredField 的 type 填充与 getComponentType 的
/// 元素解析共用）：`L<类>;` / `[<描述符>` 经 for_class（斜线键与 ldc 类字面量
/// 同一缓存条目，身份一致）；基本类型描述符经 getPrimitiveClass 的唯一实例。
/// 点形式（Class 名存储形态）先归一为斜线。无法识别 → null Class。
fn class_for_descriptor(desc: &str) -> Class {
    let slashes: std::string::String = desc.replace('.', "/");
    let prim = |java_name: &str| {
        Class::getPrimitiveClass(String::from(java_name)).unwrap_or_default()
    };
    match slashes.as_bytes().first() {
        Some(b'L') if slashes.ends_with(';') =>
            Class::for_class(String::from(&slashes[1..slashes.len() - 1])),
        Some(b'[') => Class::for_class(String::from(slashes.as_str())),
        Some(b'B') => prim("byte"),
        Some(b'C') => prim("char"),
        Some(b'D') => prim("double"),
        Some(b'F') => prim("float"),
        Some(b'I') => prim("int"),
        Some(b'J') => prim("long"),
        Some(b'S') => prim("short"),
        Some(b'Z') => prim("boolean"),
        _ => Class::default(),
    }
}

/// build.rs 生成的类层次表（OUT_DIR/hierarchy_table.rs，含模块级 static）。
mod __hierarchy {
    include!(concat!(env!("OUT_DIR"), "/hierarchy_table.rs"));
}

/// build.rs 生成的直接父类表（OUT_DIR/direct_super_table.rs）。
mod __direct_super {
    include!(concat!(env!("OUT_DIR"), "/direct_super_table.rs"));
}

/// build.rs 生成的字段元数据表（OUT_DIR/field_table.rs，含 FieldMeta 与
/// CLASS_FIELDS static）。
mod __fields {
    include!(concat!(env!("OUT_DIR"), "/field_table.rs"));
}

/// build.rs 生成的方法元数据表（OUT_DIR/method_table.rs，含 MethodMeta 与
/// CLASS_METHODS static）。
mod __methods {
    include!(concat!(env!("OUT_DIR"), "/method_table.rs"));
}
