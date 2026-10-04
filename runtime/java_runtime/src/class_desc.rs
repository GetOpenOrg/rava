//! 类静态描述符（S7，docs/plans/2026-10-04-s7-object-handle-descriptor.md §3.2）。
//!
//! `java_class!` 宏为每个生成类发射一个 `static X__DESC: __ClassDesc`，wrapper 上以固有常量
//! `X::__DESC` 指向它。类型判定（instanceof / checkcast / aastore / catch / 擦除重建）与按类
//! 无关的逻辑从这里读数据，不再按类展开一份比较代码：
//! - 类祖先判定 O(1)：`display[t.depth]` 与目标描述符同址即是 `t` 的子类（含自身）；
//! - 全超类型（本类、父类链、闭包内外的全部接口，含 `java/lang/Object`）按名字有序，二分查找。
//!
//! 名字取 `__ClassDesc` 而非 `ClassDesc`：`java.lang.constant.ClassDesc` 是 Java 类，生成文件
//! 显式 `use` 它时会遮蔽 prelude 通配引入（与 `__Shared` 同一约定）。
//!
//! 接口没有类描述符：接口载体不是运行时类，接口判定按名字查 `supertypes`（闭包外接口没有
//! Rust 类型，只能按名字表达）。

/// 一个 Java 类的静态描述符。地址即类标识（同一类恒为同一 `static`）。
#[derive(Debug)]
pub struct __ClassDesc {
    /// binary name（`java/lang/NullPointerException`）。
    pub binary_name: &'static str,
    /// 父类链深度：`java/lang/Object` 的直接子类为 0（Object 本身没有描述符）。
    pub depth: u16,
    /// 祖先描述符，最深祖先（Object 的直接子类）在前、本类在末：`display[depth]` 是自身。
    pub display: &'static [&'static __ClassDesc],
    /// 全部超类型的 binary name（含自身与 `java/lang/Object`），按字节序升序。
    pub supertypes: &'static [&'static str],
}

impl __ClassDesc {
    /// 直接父类的描述符（父类是 Object 时为 None）。
    #[inline]
    pub fn super_desc(&self) -> Option<&'static __ClassDesc> {
        match self.depth {
            0 => None,
            d => self.display.get(d as usize - 1).copied(),
        }
    }

    /// 本类是否是 `target` 或其子类（O(1)）。
    #[inline]
    pub fn is_subclass_of(&self, target: &__ClassDesc) -> bool {
        self.display
            .get(target.depth as usize)
            .is_some_and(|a| std::ptr::eq(*a, target))
    }

    /// 本类是否是名为 `name` 的类型（类或接口，含 `java/lang/Object`）的子类型。
    #[inline]
    pub fn is_subtype_name(&self, name: &str) -> bool {
        self.supertypes.binary_search(&name).is_ok()
    }
}
