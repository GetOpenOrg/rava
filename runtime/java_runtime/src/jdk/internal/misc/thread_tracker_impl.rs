//! `jdk/internal/misc/ThreadTracker` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! JDK 语义：跟踪「正在执行某段代码」的线程集合，用于检测同线程递归进入
//!（`Charset.lookupViaProviders` 的 provider 递归保护、`URL` 的 handler 查找等）。
//! `tryBegin()` 当前线程首次加入时返回 key，已在集合中则返回 null；`end(key)` 移除。
//!
//! 原生侧不翻译 `Set<ThreadRef>` 链（ThreadRef/ConcurrentHashMap.KeySetView 仅服务于本类），
//! 改为按 tracker 身份在线程局部表中记录线程身份集合。key 即当前线程对象。

use crate::prelude::*;
use super::thread_tracker::implref::ThreadTracker;
use crate::java::lang::Thread;
use crate::sync_model::__RefSlot as RefCell;
use std::collections::{HashMap, HashSet};

std::thread_local! {
    /// tracker 身份 → 已登记线程身份集合。
    static TRACKED: RefCell<HashMap<usize, HashSet<usize>>> = RefCell::new(HashMap::new());
}

fn identity_of(obj: Object) -> usize {
    obj.0.__identity() as usize
}

impl ThreadTracker {
    fn tracker_id(&self) -> usize {
        identity_of(Object::from(Clone::clone(self)))
    }

    /// `<init>()`：空跟踪集合。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Ok(this)
    }

    #[doc(hidden)]
    pub fn __init_on(this: Self) -> Result<Self> {
        Ok(this)
    }

    /// `tryBegin()`：当前线程未登记 → 登记并返回 key（当前线程对象）；已登记 → null。
    pub fn __impl_tryBegin(&self) -> Result<Object> {
        let thread = Object::from(Thread::currentThread()?);
        let tid = identity_of(Clone::clone(&thread));
        let tracker = self.tracker_id();
        let added = TRACKED.with(|t| t.borrow_mut().entry(tracker).or_default().insert(tid));
        Ok(if added { thread } else { Object::default() })
    }

    /// `begin()`：登记当前线程并返回 key；重复登记违反 JDK 断言（`added`），此处按幂等处理。
    pub fn __impl_begin(&self) -> Result<Object> {
        let thread = Object::from(Thread::currentThread()?);
        let tid = identity_of(Clone::clone(&thread));
        let tracker = self.tracker_id();
        TRACKED.with(|t| t.borrow_mut().entry(tracker).or_default().insert(tid));
        Ok(thread)
    }

    /// `end(Object key)`：移除 key 对应线程。
    pub fn __impl_end(&self, key: Object) -> Result<()> {
        if _is_jnull(&key) {
            return Ok(());
        }
        let tid = identity_of(key);
        let tracker = self.tracker_id();
        TRACKED.with(|t| {
            if let Some(set) = t.borrow_mut().get_mut(&tracker) {
                set.remove(&tid);
            }
        });
        Ok(())
    }

    /// `contains(Thread)`：线程是否已登记。
    pub fn __impl_contains(&self, thread: Thread) -> Result<bool> {
        let tid = identity_of(Object::from(thread));
        let tracker = self.tracker_id();
        Ok(TRACKED.with(|t| t.borrow().get(&tracker).is_some_and(|s| s.contains(&tid))))
    }
}
