//! 引擎：宽接收者枢纽跨克隆共享，与按调用点建模目标的接收者按效果去重。
//!
//! 增长枢纽按接收值来源建立（`hub.rs::grow_hub`）：同一调用成员在不同克隆上下文里各有一族来源，各持一份精确接收者
//! 集合。框架规模的闭包里 `Object.equals` / `hashCode` / `getClass` / `toString` 这类成员的增长枢纽有数千份、
//! 每份上万接收者，接收者集合与逐（调用点 × 接收者）接边的按调用点建模目标占满内存且不收敛（计划 §4.11 / §4.12）。
//!
//! - 共享：增长枢纽的接收者超过 [`HUB_SHARE`] 即永久升级——按（调用成员, 接口调用, 只收抽象对象, 档位）取共享枢纽
//!   （[`HubSet::Shared`]），原接入记录逐条以原实参 / 结果 / 实参值 / 归属重接到共享枢纽，原接收者全部转发并入，
//!   此后的接入与接收者一律转发（`Hub::fwd`）。原枢纽已接的边保留：共享枢纽的接收者与接入都是其超集，目标按同一
//!   `recv_ctx` 选出，合流值是其上界，原边的效果被涵盖。升级条件「最终接收者数超过阈值」随值集单调，不动点与处理
//!   次序无关；阈值取在语料单例闭包的最大增长枢纽接收者数之上，语料闭包不触发升级、结果不变。
//! - 去重：按调用点建模的目标（非字节码：native、清单特判返回）对每个接收者逐接入点接边。效果只经 this 注入与
//!   返回值依赖接收者；返回值不依赖接收者（或只依赖其类镜像）时，同效果键的接收者逐调用点效果相同，只有代表逐接入点
//!   接边，其余只注入被调方 this（`P(t, 0)`，与调用点无关）。逐调用点表 `special` 只留代表。

use super::*;

/// 增长枢纽精确接收者数的升级阈值：高于语料（e2e 全集）单例闭包的最大增长枢纽接收者数（实测见计划 §4.12）
pub(super) const HUB_SHARE: usize = 4096;
/// 返回值与接收者无关的按调用点建模目标：全部接收者同一效果键
const SKEY_ANY: u32 = u32::MAX;

impl Engine<'_> {
    /// 枢纽 h 的转发终点（未升级者为自身）
    pub(super) fn hub_fwd(&self, mut h: u32) -> u32 {
        while let Some(w) = self.hubs[h as usize].fwd {
            h = w;
        }
        h
    }

    /// 增长枢纽 g 刚并入一个接收者：记峰值；超过阈值则升级并返回 true
    pub(super) fn hub_grown(&mut self, g: u32) -> bool {
        let n = self.hubs[g as usize].recvs.len();
        self.grow_max = self.grow_max.max(n);
        if n <= HUB_SHARE {
            return false;
        }
        self.hub_upgrade(g);
        true
    }

    /// 增长枢纽 g 升级为共享枢纽：接入记录重接、接收者转发；g 的接收者与逐调用点表释放（报告用的中转目标保留）
    fn hub_upgrade(&mut self, g: u32) {
        let Some((mref, iface, objs)) = self.hubs[g as usize].share.clone() else { return };
        let hub = &self.hubs[g as usize];
        let (site, owner, ptypes, ret, via) = (hub.site.clone(), hub.owner, hub.ptypes.clone(), hub.ret, hub.via.clone());
        let lc = self.via_level(&via);
        let key = (mref, iface, HubSet::Shared(objs), lc);
        let w = match self.hub_ids.get(&key) {
            Some(&w) => w,
            None => self.hub_new(key, &site, owner, ptypes, ret, via),
        };
        self.hub_upgraded += 1;
        let hub = &mut self.hubs[g as usize];
        hub.fwd = Some(w);
        let links = std::mem::take(&mut hub.links);
        let recvs = std::mem::take(&mut hub.recvs);
        hub.special = BTreeMap::new();
        hub.edged = HashSet::default();
        hub.skeys = HashSet::default();
        hub.lambdas = Rc::default();
        // 接入记录按原样重接（实参来源、结果节点、实参值、读者归属），不归当前 lambda 调用读者
        let saved = self.call_vals.take();
        let outer = self.cur_lcall.take();
        for ((m, off), l) in links {
            self.call_vals = l.cv.clone();
            self.with_reader(l.host, |e| e.link_hub_in(w, m, off, &l.a, l.res));
        }
        self.call_vals = saved;
        self.cur_lcall = outer;
        for r in recvs {
            self.hub_recv(w, r);
        }
    }

    /// 按调用点建模目标 t 上接收者 r 的效果键：None = 效果依赖接收者本身（逐个接边）。
    /// 字节码目标（透传摘要随分析推进）、lambda / 手写实现对象（合成类镜像与逐个派发）、写内存 / 反射回调的手写体、
    /// 返回接收者本身或其镜像所指的模型都逐个；类镜像返回值只依赖接收者类型；其余返回值与接收者无关
    pub(super) fn special_key(&mut self, t: usize, r: u32) -> Option<u32> {
        let kind = self.methods[t].kind;
        if kind == Kind::Bytecode || self.methods[t].is_static || self.lambdas.contains_key(&r) || self.hwobjs.contains_key(&r) {
            return None;
        }
        if matches!(kind, Kind::Handwritten(_)) && (self.hw_writes(t).iter().any(Option::is_some) || self.rcall_entry(t).is_some()) {
            return None;
        }
        match self.methods[t].ret_model {
            RetModel::Mirror => Some(self.ty(r)),
            RetModel::Plain | RetModel::Caller | RetModel::NewArray(_) | RetModel::StaticBase | RetModel::Read(_) => Some(SKEY_ANY),
            _ => None,
        }
    }
}
