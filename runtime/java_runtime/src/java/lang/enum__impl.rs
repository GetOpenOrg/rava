//! `java/lang/Enum` 手写实现（仅当 `enum_.rs` 进入闭包生成时编译，见 K-2 规则）。

use crate::prelude::*;
use super::*;

impl<E: Clone + Default + 'static + From<Object> + Into<Object>> Enum<E> {
    /// `Enum.valueOf(Class&lt;T&gt;, String)`（javac 为每个 enum 合成的 `valueOf` 的落点）。
    ///
    /// JDK 字节码经 `Class.enumConstantDirectory()` 的反射目录按名查找；原生侧由
    /// 运时常量目录等价承载（`java_class!` 宏在类初始化后按「自身类型 static 字段」
    /// 形态登记，字段名即常量名）。语义与 JDK 一致：命中即返回；未命中时 name 为
    /// null 抛 NullPointerException，否则抛 IllegalArgumentException。
    ///
    /// E 由调用点按「常量池类是当前类的祖先」从子类泛型签名精化（enum 子类内
    /// E=子类自身）；泛型上下文里未精化的 `Enum::<Object>::valueOf` 无法构造
    /// 擦除实参的祖先视图（依赖存储层擦除，见计划 A-1），命中路径会抛
    /// ClassCastException。
    ///
    /// 错误路径构造的异常（Rust→Java 反向边）经 upcalls 属性声明，BFS 触达
    /// 本方法时入队其翻译（原 vm_roots.txt 的无条件种子收窄为按需——valueOf
    /// 未被调用的构建无需翻译这两个构造器）。
    #[jvm_native(upcalls = "java/lang/NullPointerException.<init>:(Ljava/lang/String;)V java/lang/IllegalArgumentException.<init>:(Ljava/lang/String;)V")]
    pub fn valueOf(enumClass: Class, name: String) -> Result<Enum<E>> {
        let cls_name = format!("{}", enumClass.__get_name());
        // JVM 反射路径语义：按名查常量前强制目标类初始化（常量目录在
        // `<clinit>` 之后登记——见 lib.rs 钩子表）
        crate::ensure_class_initialized(&cls_name)?;
        if let Some(found) = lookup_constant(&cls_name, &format!("{}", name)) {
            return Ok(Enum::<E>::from(found));
        }
        // name 为 String wrapper：null 判定走 vtable 钩子（`_is_jnull` 只对
        // `Object` 载体生效、对 wrapper 恒 false——原 NPE 检查死，null 名会
        // 误走 IllegalArgumentException 分支）
        if name.is_jvm_null() {
            return Err(JvmError::from(
                NullPointerException::new_str(String::from("Name is null"))?));
        }
        Err(JvmError::from(IllegalArgumentException::new_str(String::from(
            format!("No enum constant {}.{}", cls_name, name)))?))
    }
}
