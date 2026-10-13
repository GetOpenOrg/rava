//! 引擎：增长链追踪（`--lambda-prof` 随 dump 打印，只读诊断；优化方案 2026-10-13 S1 埋点）。
//!
//! S2 无限链排查与 S4 收敛改写的观测底座：把「分析器里还有什么在长」按**实体族**与**关系族**逐 dump 给出
//! 当前规模、较上段的增量与速率——判据（方案 S1）：
//! - 方法数停增而某族仍线性 / 加速增长 ⇒ 该族无界（或收敛中未到平台）；
//! - 实体名按「数字段坍缩」模板分组后单段新增多实例 ⇒ 实体名混入计数器（运行时生成类 / 动态代理
//!   建模路径的实锤信号）；模板数大但一次性出齐（如 indy 站点 λ 类）为合法有限族。
//!
//! 结构上分四段：
//! - `growth fam` / `growth rel`：实体族（names 细分到 ctx@/obj/array/mirror/lambda/hwobj/class/bare）
//!   与关系族（g_sub / sub 查询位图 / mirror_open / hub_sites / pstr succ / dispatch / 图节点边）
//!   的水位 + 增量；
//! - `growth names`：本段新登记名字的分类计数 + 反例计数器模板前列（S2 实锤检测）；
//! - `growth site`：本段新 ctx 名按链首段（`@成员:偏移`）、新对象名按 `类@成员:偏移` 的来源站点前列；
//! - `growth depth` / `growth rows`：上下文与对象的堆链深度直方图、g_sub / mirror_open 行规模前列。
//!
//! 全部在 dump 时从既有表扫描（与 census 同法），不落热路径；剖析关闭时不运行（缺省零开销）。
//! 快照存于 [`super::lambda_prof::LambdaProf::growth`]（剖析开启才存在）。

use super::*;

/// 逐 dump 水位快照（增量 = 当前 − 快照；dump 后覆写）
#[derive(Default)]
pub(super) struct GrowthSnap {
    names: u64,
    methods: u64,
    members: u64,
    ctx: u64,
    objs: u64,
    arrays: u64,
    mirrors: u64,
    lambdas: u64,
    hwobjs: u64,
    classes: u64,
    g: u64,
    gsub_rows: u64,
    gsub_el: u64,
    subq_rows: u64,
    subq_bits: u64,
    mop_rows: u64,
    mop_el: u64,
    hsites: u64,
    hedges: u64,
    hubs: u64,
    pstr_rows: u64,
    pstr_slots: u64,
    gathers: u64,
    an: u64,
    aev: u64,
    disp_rows: u64,
    disp_el: u64,
    gnodes: u64,
    gedges: u64,
    /// `graph.adds[1]`（集合并入且真实增长次数）/ `graph.adds[2]`（并入元素数）
    adds: u64,
    addsel: u64,
}

/// 名字的数字段坍缩模板（`Foo$$_javassist_12` → `Foo$$_javassist_#`）：同一模板一段内多个实例
/// ⇒ 名字里混入了计数器 / 序号（S2 无限实体链的实锤判据）
fn tmpl(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_digit = false;
    for c in s.chars() {
        if c.is_ascii_digit() {
            if !in_digit {
                out.push('#');
                in_digit = true;
            }
        } else {
            out.push(c);
            in_digit = false;
        }
    }
    out
}

/// 链的段数（`#` 分隔，空段不计）
fn chain_depth(c: &str) -> usize {
    c.split('#').filter(|g| !g.is_empty()).count()
}

impl Engine<'_> {
    /// 增长链一段（随 `--lambda-prof` dump 调用；`p.growth` 为上段快照，段末覆写）
    pub(super) fn growth_dump(&self, tag: &str) {
        let Some(p) = self.lprof.as_ref() else { return };
        let last = p.growth.borrow();
        let d = |cur: u64, old: u64| format!("+{}/{}", cur.wrapping_sub(old), cur);

        // ── 实体族水位（方法数停增后仍长的族即无界嫌疑；G 为全局已实例化类型）──
        let names_n = self.names.len() as u64;
        let methods_n = self.methods.len() as u64;
        let members_n = self.mbase.len() as u64;
        let ctx_n = self.methods.values().filter(|m| m.ctx != NOCTX).count() as u64;
        let objs_n = self.objs.len() as u64;
        eprintln!(
            "{tag} growth fam {}s names={} methods={} members={} ctx={} objs={} arrays={} mirrors={} lambdas={} hwobjs={} classes={} G={}",
            p.elapsed_s(),
            d(names_n, last.names),
            d(methods_n, last.methods),
            d(members_n, last.members),
            d(ctx_n, last.ctx),
            d(objs_n, last.objs),
            d(self.arrays.len() as u64, last.arrays),
            d(self.mirrors.len() as u64, last.mirrors),
            d(self.lambdas.len() as u64, last.lambdas),
            d(self.hwobjs.len() as u64, last.hwobjs),
            d(self.classes.len() as u64, last.classes),
            d(self.g.len() as u64, last.g),
        );

        // ── 关系族水位（g_sub 行 = 查过的过滤类型数；hub_sites 边 = 调用点接入枢纽数）──
        let (gsub_rows, gsub_el) = (self.g_sub.len() as u64, self.g_sub.values().map(|v| v.len() as u64).sum::<u64>());
        let subq_rows = self.sub_rows.iter().filter(|r| r.has_any()).count() as u64;
        let subq_bits = self.sub_rows.iter().map(|r| r.known_bits()).sum::<u64>();
        let (mop_rows, mop_el) =
            (self.mirror_open.len() as u64, self.mirror_open.values().map(|v| v.len() as u64).sum::<u64>());
        let (hsites, hedges) =
            (self.hub_sites.len() as u64, self.hub_sites.values().map(|s| s.len() as u64).sum::<u64>());
        let (pstr_rows, pstr_slots) = self.pstr.growth_stats();
        let (an, aev) = {
            let mut an = 0u64;
            let mut aev = 0u64;
            for m in self.methods.values() {
                if let Some(a) = &m.analysis {
                    an += 1;
                    aev += a.events.len() as u64;
                }
            }
            (an, aev)
        };
        let (disp_rows, disp_el) =
            (self.dispatch.len() as u64, self.dispatch.values().map(|s| s.len() as u64).sum::<u64>());
        let gedges = self.graph.edges.iter().map(|e| e.len() as u64).sum::<u64>();
        eprintln!(
            "{tag} growth rel gsub={} subq={} mop={} hub_sites={} hubs={} pstr={} gathers={} an={} aev={} dispatch={} graph=(nodes={} edges={} adds={})",
            format!("{} {}", d(gsub_rows, last.gsub_rows), d(gsub_el, last.gsub_el)),
            format!("{} {}", d(subq_rows, last.subq_rows), d(subq_bits, last.subq_bits)),
            format!("{} {}", d(mop_rows, last.mop_rows), d(mop_el, last.mop_el)),
            format!("{} {}", d(hsites, last.hsites), d(hedges, last.hedges)),
            d(self.hubs.len() as u64, last.hubs),
            format!("{} {}", d(pstr_rows, last.pstr_rows), d(pstr_slots, last.pstr_slots)),
            d(self.gathers.len() as u64, last.gathers),
            d(an, last.an),
            d(aev, last.aev),
            format!("{} {}", d(disp_rows, last.disp_rows), d(disp_el, last.disp_el)),
            d(self.graph.node_count() as u64, last.gnodes),
            d(gedges, last.gedges),
            format!("+{}/{}", self.graph.adds[1].wrapping_sub(last.adds), self.graph.adds[2].wrapping_sub(last.addsel)),
        );

        // ── 本段新登记名字：分类计数 + 计数器模板前列 + 来源站点前列 ──
        let new_from = (last.names as usize).min(self.names.len());
        if self.names.len() > new_from {
            let mut cat = [0u64; 8]; // ctx@ / obj / array / mirror / lambda / hwobj / class / bare
            let mut tmpls: HashMap<String, (u64, String)> = HashMap::default();
            let mut ctxsites: HashMap<String, u64> = HashMap::default();
            let mut objsites: HashMap<String, u64> = HashMap::default();
            for ix in new_from..self.names.len() {
                let idx = ix as u32;
                let n: &str = &self.names[ix];
                let k = if self.objs.contains_key(&idx) {
                    1
                } else if self.arrays.contains_key(&idx) {
                    2
                } else if self.mirrors.contains_key(&idx) {
                    3
                } else if self.lambdas.contains_key(&idx) {
                    4
                } else if self.hwobjs.contains_key(&idx) {
                    5
                } else if n.starts_with('@') {
                    0
                } else if self.classes.contains_key(n) {
                    6
                } else {
                    7
                };
                cat[k] += 1;
                let t = tmpl(n);
                let e = tmpls.entry(t).or_insert_with(|| (0, n.to_string()));
                e.0 += 1;
                // 来源站点：ctx 名取首个 `@成员:偏移`（剥掉可能的 `堆上下文~` 前缀），对象名取首段
                // `类@成员:偏移`（类随站点记，同站点多类不混）
                if k == 0 || k == 1 {
                    let first = n.split('#').next().unwrap_or(n);
                    let site = if k == 0 { first.rfind('@').map_or(first, |a| &first[a..]) } else { first };
                    let m = if k == 0 { &mut ctxsites } else { &mut objsites };
                    *m.entry(site.to_string()).or_default() += 1;
                }
            }
            let total_new = (self.names.len() - new_from) as u64;
            eprintln!(
                "{tag} growth names new={} ctx@={} obj={} array={} mirror={} lambda={} hwobj={} class={} bare={}",
                total_new,
                cat[0],
                cat[1],
                cat[2],
                cat[3],
                cat[4],
                cat[5],
                cat[6],
                cat[7],
            );
            // 计数器模板前列（同模板多实例 = 名字混入序号；一次出齐的有限族——如 λ 类按 indy 站点——
            // 只在首段集中出现，后续段归零；持续多段增长的是 S2 追查对象）
            let clip = |s: &str| {
                if s.chars().count() > 120 {
                    let mut v: String = s.chars().take(117).collect();
                    v.push('…');
                    v
                } else {
                    s.to_string()
                }
            };
            let mut tv: Vec<(String, u64, String)> = tmpls.into_iter().map(|(t, (n, e))| (t, n, e)).collect();
            tv.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
            let tops: Vec<String> =
                tv.into_iter().take(12).map(|(t, n, e)| format!("×{n} {}（例 {}）", clip(&t), clip(&e))).collect();
            if !tops.is_empty() {
                eprintln!("{tag} growth tmpl {}", tops.join(" | "));
            }
            let top_sites = |m: &HashMap<String, u64>, what: &str| {
                let mut v: Vec<(&String, &u64)> = m.iter().collect();
                v.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
                let s: Vec<String> = v.into_iter().take(8).map(|(k, n)| format!("×{n} {k}")).collect();
                if !s.is_empty() {
                    eprintln!("{tag} growth site {what} {}", s.join(" | "));
                }
            };
            top_sites(&ctxsites, "ctx");
            top_sites(&objsites, "obj");
        }

        // ── 深度分布：上下文与对象的堆链段数（含截断口径 HEAP_DEPTH 的实际落点）──
        let mut ctx_d = [0u64; 4];
        for m in self.methods.values() {
            if m.ctx == NOCTX {
                continue;
            }
            let d = self.obj_chain.get(&m.ctx).map_or(0, |c| chain_depth(c));
            ctx_d[d.min(3)] += 1;
        }
        let mut obj_d = [0u64; 4];
        for &id in self.objs.keys() {
            let d = self.obj_chain.get(&id).map_or(0, |c| chain_depth(c));
            obj_d[d.min(3)] += 1;
        }
        eprintln!(
            "{tag} growth depth ctx[0/1/2/3+]={}/{}/{}/{} objs[0/1/2/3+]={}/{}/{}/{}",
            ctx_d[0],
            ctx_d[1],
            ctx_d[2],
            ctx_d[3],
            obj_d[0],
            obj_d[1],
            obj_d[2],
            obj_d[3],
        );

        // ── 行规模前列（g_sub 行 = 过滤类型；mirror_open 行 = getClass 作用的 open 类型）──
        let mut gr: Vec<(u32, usize)> = self.g_sub.iter().map(|(t, v)| (*t, v.len())).collect();
        gr.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let gs: Vec<String> = gr
            .into_iter()
            .take(8)
            .map(|(t, n)| format!("×{n} {}", self.names[t as usize]))
            .collect();
        if !gs.is_empty() {
            eprintln!("{tag} growth rows gsub {}", gs.join(" | "));
        }
        let mut mr: Vec<(u32, usize)> = self.mirror_open.iter().map(|(t, v)| (*t, v.len())).collect();
        mr.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let ms: Vec<String> = mr
            .into_iter()
            .take(8)
            .map(|(t, n)| format!("×{n} {}", self.names[t as usize]))
            .collect();
        if !ms.is_empty() {
            eprintln!("{tag} growth rows mop {}", ms.join(" | "));
        }
        // hub_sites 前列（调用点接入的枢纽数——同站点多枢纽 / 多克隆接入的浓缩点）
        let mut hr: Vec<(usize, u32, usize)> = self.hub_sites.iter().map(|((m, o), s)| (*m, *o, s.len())).collect();
        hr.sort_by(|a, b| b.2.cmp(&a.2).then((a.0, a.1).cmp(&(b.0, b.1))));
        let hs: Vec<String> = hr
            .into_iter()
            .take(8)
            .map(|(m, o, n)| format!("×{n} {}@{o}", self.methods[m].key))
            .collect();
        if !hs.is_empty() {
            eprintln!("{tag} growth rows hub {}", hs.join(" | "));
        }

        // 段末覆写快照（下一段的增量基点；先放掉 `last` 的共享借用再取独占借用）
        drop(last);
        let mut l = p.growth.borrow_mut();
        *l = GrowthSnap {
            names: names_n,
            methods: methods_n,
            members: members_n,
            ctx: ctx_n,
            objs: objs_n,
            arrays: self.arrays.len() as u64,
            mirrors: self.mirrors.len() as u64,
            lambdas: self.lambdas.len() as u64,
            hwobjs: self.hwobjs.len() as u64,
            classes: self.classes.len() as u64,
            g: self.g.len() as u64,
            gsub_rows,
            gsub_el,
            subq_rows,
            subq_bits,
            mop_rows,
            mop_el,
            hsites,
            hedges,
            hubs: self.hubs.len() as u64,
            pstr_rows,
            pstr_slots,
            gathers: self.gathers.len() as u64,
            an,
            aev,
            disp_rows,
            disp_el,
            gnodes: self.graph.node_count() as u64,
            gedges,
            adds: self.graph.adds[1],
            addsel: self.graph.adds[2],
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 数字段坍缩模板：连续数字段坍缩为单个 `#`，非数字段原样
    #[test]
    fn growth_template() {
        assert_eq!(tmpl("Foo$$_javassist_12"), "Foo$$_javassist_#");
        assert_eq!(tmpl("X$$Lambda$17/0x123"), "X$$Lambda$#/#x#");
        assert_eq!(tmpl("com.foo.Bar"), "com.foo.Bar");
        // 描述符内不含数字；链分隔符 `#` 原样保留、各段的偏移数字各自坍缩
        assert_eq!(tmpl("@com.foo.Bar.f:(I)V:17#cls@a:1"), "@com.foo.Bar.f:(I)V:##cls@a:#");
        // 链段按 `#` 分隔、空段不计
        assert_eq!(chain_depth("a#b#c"), 3);
        assert_eq!(chain_depth("a"), 1);
        assert_eq!(chain_depth(""), 0);
    }
}
