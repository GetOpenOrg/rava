//! 具体求值调用点在工作队列不动点上判定（计划 2026-10-05-boot-image-evaluator §5.8.6）。
//!
//! 调用点处理时只登记最新输入并挂起（`concrete.rs::concrete_call`），工作队列排空后统一判定：
//! - 组合不可枚举 / 任一组合求值失败 / 结果引用映像容器对象 → 回退抽象调用边（站点重跑接边）；
//! - 否则应用尚未应用的组合（镜像缓存可物化的并入引导映像，按热求值入闭包）。
//!
//! 已应用的组合（闭包轨迹、映像中的镜像缓存组）撤不回：单调引擎中这些事实已被其他分析读过。判定若在处理调用点时
//! 即时作出，站点回退前应用过哪些组合取决于接收者集合增长与调用点处理的相对次序（默认次序与 `--flow-batch 1`
//! 不同），映像与闭包随之变化。不动点上的状态与处理次序无关，于是每轮应用的组合、回退的轮次都与次序无关。
//! 同一轮先对全部挂起站点作出判定、再统一应用，站点之间互不读对方本轮的结果。

use super::*;

/// 一个挂起调用点的判定
enum Plan {
    Fallback(String),
    Apply { outs: Vec<(Vec<AK>, Rc<Result<Outcome, String>>, bool)>, line: String },
}

impl<'a> Engine<'a> {
    /// 工作队列排空时判定挂起的具体求值调用点；返回是否有站点判定
    pub(in crate::engine) fn concrete_release(&mut self) -> bool {
        if self.concrete.held.is_empty() {
            return false;
        }
        let held = std::mem::take(&mut self.concrete.held);
        let plans: Vec<((usize, u32), Plan)> = held.iter().map(|(&w, h)| (w, self.concrete_decide(w.0, w.1, h))).collect();
        for (((m, off), plan), h) in plans.into_iter().zip(held.values()) {
            let site_name = format!("{}@{off}", self.methods[m].key);
            match plan {
                Plan::Fallback(why) => {
                    self.concrete_fallback(m, off, site_name, why);
                    self.push_site((m, off), site_prof::TRIG_RELEASE, None);
                }
                Plan::Apply { outs, line } => {
                    let entry = self.method_ctx(h.resolved.clone(), self.concrete.ctx, Via::method("concrete", m, Some(off)));
                    self.dispatch.entry((m, off)).or_default().insert(entry);
                    self.callers.entry(entry).or_default().insert(m);
                    for (c, r, hot) in outs {
                        if !self.concrete.applied.insert((m, off, c)) {
                            continue;
                        }
                        let Ok(o) = &*r else { continue };
                        match o.alt.as_ref().filter(|_| hot) {
                            Some(a) => {
                                self.image_memo_apply(&a.0);
                                self.concrete_apply(m, off, &h.resolved, &h.md, &a.1);
                            }
                            None => self.concrete_apply(m, off, &h.resolved, &h.md, o),
                        }
                    }
                    self.concrete.diag.entry(site_name).or_default().insert(line);
                }
            }
        }
        true
    }

    /// 按挂起时登记的输入判定（不改闭包；备好镜像缓存所需的映像镜像对象，按类名幂等）
    fn concrete_decide(&mut self, m: usize, off: u32, h: &Held) -> Plan {
        let combos = match self.combos(m, off, &h.md, h.recv.as_ref(), &h.args) {
            Ok(c) => c,
            Err(why) => return Plan::Fallback(why),
        };
        let r = &h.resolved;
        let Some(site) = self.h.resolve_method(&r.owner, &r.name, &r.desc, false) else { return Plan::Fallback("入口未解析".into()) };
        let mut outs = Vec::new();
        for c in &combos {
            let r = self.concrete_eval(&site, &h.resolved, c);
            if let Err(w) = &*r {
                return Plan::Fallback(format!("{c:?}：{w}"));
            }
            outs.push((c.clone(), r));
        }
        // 镜像缓存可物化进引导映像的组合按热求值入闭包（运行期缓存已命中）
        let mut why: Vec<Option<String>> = Vec::new();
        let mut hot: Vec<bool> = Vec::new();
        for (_, r) in &outs {
            let Ok(o) = &**r else {
                hot.push(false);
                why.push(None);
                continue;
            };
            let w = match (&o.alt, &o.alt_why) {
                (Some(a), _) => self.image_memo_prepare(&a.0).err(),
                (None, w) => w.clone(),
            };
            hot.push(w.is_none() && o.alt.is_some());
            why.push(w);
        }
        let image: Vec<Rc<str>> = outs
            .iter()
            .zip(&hot)
            .filter_map(|((_, r), &h)| r.as_ref().as_ref().ok().map(|o| pick(o, h)))
            .flat_map(|o| apply::image_types(o).cloned().collect::<Vec<_>>())
            .collect();
        if let Some(t) = image.iter().find(|t| self.container(t)) {
            return Plan::Fallback(format!("结果引用映像中的容器形态对象 {t}"));
        }
        // 诊断：缓存物化进映像的组合标「⇒映像」，未物化的附原因
        let shown: Vec<String> = combos
            .iter()
            .zip(why.iter().zip(&hot))
            .take(DIAG_COMBOS)
            .map(|(c, (w, &h))| match (h, w) {
                (true, _) => format!("{c:?}⇒映像"),
                (false, Some(w)) if w != "无缓存写入" => format!("{c:?}（并：{w}）"),
                _ => format!("{c:?}"),
            })
            .collect();
        let more = combos.len().saturating_sub(DIAG_COMBOS);
        let line = format!("具体求值 {} 组实参：{}{}", combos.len(), shown.join(" "), if more > 0 { format!(" …（另 {more} 组）") } else { String::new() });
        Plan::Apply { outs: outs.into_iter().zip(hot).map(|((c, r), h)| (c, r, h)).collect(), line }
    }
}
