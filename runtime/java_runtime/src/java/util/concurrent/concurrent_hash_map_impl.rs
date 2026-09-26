use crate::prelude::*;
use super::concurrent_hash_map::ConcurrentHashMap;

// ConcurrentHashMap 伴生：无参构造的容量策略。
//
// JDK 无参构造惰性分配 16 槽表（首个 put 时 initTable）。本运行时把初始
// 容量定为 1024（sizeCtl 语义同 ConcurrentHashMap(int) 构造——首个 put
// 分配 1024 槽，扩容阈值 768）：
//   - 容量不是 CHM 契约的一部分（JDK 自身按负载/竞争动态演化），观察等价；
//   - 单线程运行时无并发扩容收益诉求，中大型表（时区 region、属性快照、
//     服务注册等系统级映射）一次到位。
// 边界：生成侧 transfer（扩容搬移）的 low/high 链拆分存在变量提升缺口
//（分支内赋值被遮蔽 let 吞失，codegen/method/vars.py 的 hoist 机制待修），
// 1024 容量使已知消费面在阈值内完成装载、不触达该路径（TzdbZoneRules
// Provider.load 实测 604 region）。缺口修复后本容量策略不变、仍然成立。
impl<K, V> ConcurrentHashMap<K, V>
where
    K: Clone + Default + 'static + From<Object> + Into<Object> + crate::sync_model::__ThreadSafe,
    V: Clone + Default + 'static + From<Object> + Into<Object> + crate::sync_model::__ThreadSafe,
{
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        // 父视图初始化与生成侧 __init_on 同链（AbstractMap 无实例初始化语义，
        // 保持同一调用形态避免载体差异）
        crate::java::util::AbstractMap::<K, V>::__init_on(
            <crate::java::util::AbstractMap<K, V> as ::std::convert::From<Self>>::from(
                ::std::clone::Clone::clone(&this)))?;
        this.__set_sizeCtl(1024i32);
        Ok(this)
    }
}
