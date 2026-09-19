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

    /// `Class.getName()`：返回类对象的二进制名（Java 形式，点分隔）。
    ///
    /// `java/lang/Class` 只作为类型存根进入闭包，字节码版的 `getName()` 是 stub，
    /// 故在此手写。名字来源有二：类字面量经 `for_class` 写入、基本类型经
    /// `getPrimitiveClass` 写入（二者落到同一个 private name 字段）。
    pub fn __impl_getName(&self) -> Result<String> {
        Ok(Clone::clone(&self.__get_name()))
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
