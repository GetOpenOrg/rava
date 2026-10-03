//! 引擎：按名查方法的包装方法与调用点配对。
//!
//! 形如 `getPrivateMethod(Class cl, String name, ..)` 的辅助方法：内部的按名查找（清单 `method_lookups`）
//! 以本方法形参为查找类、另一形参为名字。名字与类各自汇合全部调用点再相乘会失真（任意类 × 任意名），
//! 只取汇合格的中间态常量又随调用点接入先后而变。终态口径：包装方法登记为「查找类形参 × 名字形参」，
//! 各调用点按本点的名字实参（字面量）× 本点的类实参值集（类镜像）点名——结果只并不减，与处理顺序无关。
//! 调用点的两个实参又来自其形参时，调用方同样登记为包装方法（逐层上推）。

use super::*;

/// 包装方法上的一组配对：查找类形参 × 名字形参（序号含接收者，同 `Src::Param`）
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct LookupWrap {
    cls: u16,
    name: u16,
    /// 查找类在包装方法内的非形参来源（如沿 `getSuperclass` 上溯的结果），配对时并入调用点的类值集
    extra: Rc<[Node]>,
    /// 查找结果的反射调用通道
    ch: u8,
}

impl<'a> Engine<'a> {
    /// 方法 m 的按名查找点：查找类值 cv 与名字值 nv 都含本方法形参时登记 m 为包装方法；返回是否登记了配对
    /// （登记了的形参名字不再记为反射缺口：各调用点按配对点名）
    pub(super) fn lookup_wrap_site(&mut self, m: usize, cv: &V, nv: &V, extra: &[Node], ch: u8) -> bool {
        let cps = param_srcs(cv);
        let nps = param_srcs(nv);
        if cps.is_empty() || nps.is_empty() {
            return false;
        }
        let mut ex: Vec<Node> = extra.to_vec();
        let class = self.id(CLASS);
        for f in self.feeds(m, cv, class) {
            if let Feed::N(n) = f {
                if !matches!(n, Node::P(mm, _) if mm == m) {
                    ex.push(n);
                }
            }
        }
        ex.sort();
        ex.dedup();
        let extra: Rc<[Node]> = ex.into();
        let key = self.methods[m].key.clone();
        let mut added = false;
        // 同一 (类形参, 名字形参, 通道) 只留一项，非形参来源取并（逐层上推时来源只增，登记次数有界）
        let ws = self.lwraps.entry(key).or_default();
        for &cls in &cps {
            for &name in &nps {
                match ws.iter_mut().find(|w| w.cls == cls && w.name == name && w.ch == ch) {
                    Some(w) => {
                        if extra.iter().any(|n| !w.extra.contains(n)) {
                            let mut u: Vec<Node> = w.extra.iter().chain(extra.iter()).copied().collect();
                            u.sort();
                            u.dedup();
                            w.extra = u.into();
                            added = true;
                        }
                    }
                    None => {
                        ws.push(LookupWrap { cls, name, extra: extra.clone(), ch });
                        added = true;
                    }
                }
            }
        }
        if added {
            // 已处理过的调用点须按新登记的配对重放
            for c in self.callers.get(&m).cloned().unwrap_or_default() {
                self.methods[c].applied = None;
                self.push_m(c);
            }
        }
        true
    }

    /// 方法 m 的按名查找调用（实参 args 含接收者）：查找类取 Class 接收者，名字取 String 形参；
    /// 任一组合登记了配对即返回 true。查找类经 Class 形参传入的查找（`findStatic` 等）不在此列：
    /// 其形参名字与 Class 常量 / 本点字面量的组合另有口径（见 `reflective_writes`）
    pub(super) fn lookup_wraps(&mut self, m: usize, mref: &MemberRef, opcode: u8, args: &[V], ch: u8) -> bool {
        let Some(md) = parse_method(&mref.desc) else { return false };
        let skip = usize::from(opcode != classfile::op::INVOKESTATIC);
        let Some(cv) = args.first().filter(|_| skip == 1) else { return false };
        let mut hit = false;
        for (p, nv) in md.params.iter().zip(args.iter().skip(skip)) {
            if matches!(p, FieldType::Object(c) if c == STRING) {
                hit |= self.lookup_wrap_site(m, cv, nv, &[], ch);
            }
        }
        hit
    }

    /// 调用点 (m, off)（实参 args 含接收者）：被调方法是包装方法时按本点实参配对点名
    pub(super) fn lookup_wrap_call(&mut self, m: usize, off: u32, args: &[V]) {
        if self.lwraps.is_empty() {
            return;
        }
        let Some(ts) = self.dispatch.get(&(m, off)) else { return };
        let mut ws: Vec<LookupWrap> = vec![];
        for &t in ts {
            if let Some(w) = self.lwraps.get(&self.methods[t].key) {
                ws.extend(w.iter().cloned());
            }
        }
        for w in ws {
            let (Some(cv), Some(nv)) = (args.get(w.cls as usize), args.get(w.name as usize)) else { continue };
            let (cv, nv) = (cv.clone(), nv.clone());
            let mut names: BTreeSet<Rc<str>> = nv.lits().into_iter().collect();
            // 名字与类都来自本方法形参：本方法同样是包装方法（其调用点再配对）；
            // 只有名字来自形参时，类已在本点确定，名字取各调用点在该形参上的字符串常量
            if !self.lookup_wrap_site(m, &cv, &nv, &w.extra, w.ch) {
                names.extend(self.param_strs(m, off, &nv));
            }
            if names.is_empty() {
                continue;
            }
            let class = self.id(CLASS);
            let mut fs = self.feeds(m, &cv, class);
            fs.extend(w.extra.iter().map(|&n| Feed::N(n)));
            for c in self.feed_mirror_classes(m, &fs) {
                for n in &names {
                    self.reflect_name(&c, n, w.ch);
                }
            }
        }
    }

    /// 值集 fs 中类镜像所指的类（读者登记同 `value_set`）；所指未知的值记为方法 m 的反射缺口
    pub(super) fn feed_mirror_classes(&mut self, m: usize, fs: &[Feed]) -> Vec<String> {
        let s = self.value_set(fs);
        let mut out = vec![];
        for x in s.classes.iter() {
            match self.mirrors.get(&x) {
                Some(&c) => out.push(self.names[c as usize].to_string()),
                None => {
                    let what = self.names[x as usize].to_string();
                    self.reflect_gaps.insert(format!("{} <- recv({what})", self.methods[m].key));
                }
            }
        }
        for o in &s.open {
            self.reflect_gaps.insert(format!("{} <- recv(open({}))", self.methods[m].key, self.names[o as usize]));
        }
        out
    }
}

/// 值的形参来源序号（含接收者）
fn param_srcs(v: &V) -> Vec<u16> {
    v.srcs()
        .iter()
        .filter_map(|s| match *s {
            Src::Param(i) => Some(i),
            _ => None,
        })
        .collect()
}
