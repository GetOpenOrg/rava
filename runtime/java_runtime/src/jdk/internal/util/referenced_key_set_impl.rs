//! `jdk/internal/util/ReferencedKeySet` 手写伴生：内部边界类，按调用链按需
//! 实现（K-2 规则），其余保持 panic 存根。
//!
//! 消费链：`MethodType.<clinit>` 的 internTable（方法类型的规范实例池）。
//! JDK 原型是弱引用集（ReferenceKey 包装 + ReferenceQueue 清扫）——Rc 运行
//! 时无 GC 弱引用形态，以「实例身份键的线程内强引用名单」承载：驻留语义
//! （同值取同一规范实例）完整保留，仅生命周期语义差异（不主动驱逐——
//! 观察面等价，JDK 的驱逐只影响内存）。元素相等按 ObjectVTable.equals
//! （MethodType.equals 即值相等）。

use crate::prelude::*;
use super::referenced_key_set::ReferencedKeySet;
use crate::sync_model::__RefSlot as RefCell;
use std::collections::HashMap;

thread_local! {
    /// 驻留名单：set 实例身份（Rc 指针）→ 已驻留元素（Object 形态）。
    static TABLES: RefCell<HashMap<usize, Vec<Object>>> = RefCell::new(HashMap::new());
}

fn _with_table<R>(key: usize, f: impl FnOnce(&mut Vec<Object>) -> R) -> R {
    TABLES.with(|t| f(t.borrow_mut().entry(key).or_default()))
}

impl<T> ReferencedKeySet<T>
where
    T: Clone + Default + 'static + From<Object> + Into<Object>,
{
    /// static `create(isSoft, useNativeQueue, supplier)`：工厂——弱引用参数与
    /// 后备 Map 供应商在强引用名单形态下无意义（不驱逐、无并发扩容钩子），
    /// 注册空名单即可。supplier 取擦除 Object 传参形态（调用点 codegen 的
    /// 泛型擦除形态），本形态下不消费。
    pub fn create_z_z_supplier(_isSoft: bool, _useNativeQueue: bool, _supplier: Object) -> Result<Self> {
        let mut s = Self::default();
        s._init_not_null();
        let key = (Rc::as_ptr(&s.vtable) as *const u8) as usize;
        _with_table(key, |v| *v = Vec::new());
        Ok(s)
    }

    /// static `create(isSoft, supplier)`：JDK25 去掉 `useNativeQueue` 参数后的
    /// 2 参工厂（JDK21 形态见上），取舍同上。supplier 以泛型接收——JDK25 调用点
    /// 传 `Supplier<Object>`，而 Supplier 不必在每个闭包里存在，本文件不点名它。
    pub fn create<S>(isSoft: bool, _supplier: S) -> Result<Self> {
        Self::create_z_z_supplier(isSoft, false, Object::default())
    }

    /// `get(e)`：取同值规范实例；缺席 → null（T::default 的 jvm-null 形态）。
    pub fn get(&self, e: T) -> Result<T> {
        let key = (Rc::as_ptr(&self.vtable) as *const u8) as usize;
        let probe: Object = e.into();
        let hit = _with_table(key, |v| {
            v.iter()
                .find(|x| x.equals(Clone::clone(&probe)).unwrap_or(false))
                .cloned()
        });
        Ok(match hit {
            Some(o) => T::from(o),
            None => T::default(),
        })
    }

    /// `intern(e)`：取同值规范实例，缺席则驻留 e 并返回 e。
    pub fn intern_obj(&self, e: T) -> Result<T> {
        let key = (Rc::as_ptr(&self.vtable) as *const u8) as usize;
        let probe: Object = Clone::clone(&e).into();
        let hit = _with_table(key, |v| {
            if let Some(x) = v
                .iter()
                .find(|x| x.equals(Clone::clone(&probe)).unwrap_or(false))
            {
                Clone::clone(x)
            } else {
                v.push(Clone::clone(&probe));
                Clone::clone(&probe)
            }
        });
        Ok(T::from(hit))
    }

    /// `size()`：驻留元素数（AbstractCollection 协议成员）。
    pub fn size(&self) -> Result<i32> {
        let key = (Rc::as_ptr(&self.vtable) as *const u8) as usize;
        Ok(_with_table(key, |v| v.len() as i32))
    }
}
