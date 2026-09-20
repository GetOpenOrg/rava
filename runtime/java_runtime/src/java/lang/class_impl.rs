use crate::prelude::*;
use super::*;
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

/// build.rs 生成的类层次表（OUT_DIR/hierarchy_table.rs，含模块级 static）。
mod __hierarchy {
    include!(concat!(env!("OUT_DIR"), "/hierarchy_table.rs"));
}

/// build.rs 生成的直接父类表（OUT_DIR/direct_super_table.rs）。
mod __direct_super {
    include!(concat!(env!("OUT_DIR"), "/direct_super_table.rs"));
}
