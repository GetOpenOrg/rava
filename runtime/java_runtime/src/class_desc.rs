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
//!
//! 实例字段（S7-3）：存储 `X__inner` 内联平铺全部实例字段单元（继承字段在前、自有字段在后），
//! 字段的平铺下标即 `field_base + 自有序号`，运行时类的 `offsets()` 按平铺下标给出字段在存储中的
//! 字节偏移。浅拷贝、Unsafe 按名字段协议沿 `display` 读各类的 `fields`，不再按类展开方法
//! （见 `field_desc.rs`）。

use crate::field_desc::__FieldDesc;
use crate::java::lang::Object;

/// 一个 Java 类的静态描述符。地址即类标识（同一类恒为同一 `static`）。
pub struct __ClassDesc {
    /// binary name（`java/lang/NullPointerException`）。
    pub binary_name: &'static str,
    /// 父类链深度：`java/lang/Object` 的直接子类为 0（Object 本身没有描述符）。
    pub depth: u16,
    /// 祖先描述符，最深祖先（Object 的直接子类）在前、本类在末：`display[depth]` 是自身。
    pub display: &'static [&'static __ClassDesc],
    /// 全部超类型的 binary name（含自身与 `java/lang/Object`），按字节序升序。
    pub supertypes: &'static [&'static str],
    /// 本类自有实例字段（声明序，与存储布局一致）。
    pub fields: &'static [__FieldDesc],
    /// 继承实例字段数：本类第 i 个自有字段的平铺下标是 `field_base + i`。
    pub field_base: u16,
    /// 新建本类的默认存储（全部字段取缺省值、新标识），装入 Object。
    pub alloc: fn() -> Object,
    /// 本类存储中平铺实例字段（按平铺下标）的字节偏移（实现层导出的偏移表）。
    pub offsets: fn() -> &'static [u32],
}

impl std::fmt::Debug for __ClassDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("__ClassDesc")
            .field("binary_name", &self.binary_name)
            .field("depth", &self.depth)
            .field("fields", &self.fields)
            .field("field_base", &self.field_base)
            .finish_non_exhaustive()
    }
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
