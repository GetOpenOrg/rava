//! 引擎：LambdaMetafactory 的装箱 / 拆箱适配边。
//!
//! SAM 签名与实现方法签名在同一位置上一边是基本类型、一边是引用类型时，LambdaMetafactory 生成的
//! 适配代码调用装箱类的 `valueOf`（基本 → 引用）或 `xxxValue`（引用 → 基本，含扩宽）。这些调用
//! 由 VM 生成、不在任何字节码里，这里按描述符差异补上调用边；装箱类取自清单 `[boxing]`。

use super::*;

/// 装箱工厂的方法名（`valueOf(基本类型)` → 装箱类）
const BOX_FACTORY: &str = "valueOf";
/// 拆箱方法名后缀（`intValue` / `longValue` …：零实参、返回目标基本类型）
const UNBOX_SUFFIX: &str = "Value";

/// 一处适配调用
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Conv {
    /// 实参位置 `pos`（捕获实参 ++ SAM 实参的下标）：基本类型 `prim` 装箱后传给实现方法的引用形参
    BoxArg { pos: usize, prim: u8 },
    /// 实参位置 `pos`：引用实参经 `owner` 的拆箱方法转成实现方法的基本类型形参 `prim`
    UnboxArg { pos: usize, owner: String, prim: u8 },
    /// 实现方法返回基本类型 `prim`，SAM 返回引用：装箱
    BoxRet { prim: u8 },
    /// 实现方法返回引用，SAM 返回基本类型 `prim`：经 `owner` 的拆箱方法
    UnboxRet { owner: String, prim: u8 },
}

/// 引用类型 `src` → 基本类型 `dst` 的拆箱方法所在类：目标基本类型的装箱类；`src` 本身是其他装箱类时
/// 另取它（`Integer` → `long` 调 `Integer.longValue`）
fn unbox_owners(man: &Manifest, src: &FieldType, dst: u8) -> Vec<String> {
    let mut out: Vec<String> = man.boxed_class(dst).map(String::from).into_iter().collect();
    if let FieldType::Object(c) = src {
        if man.unboxed_prim(c).is_some() && !out.contains(c) {
            out.push(c.clone());
        }
    }
    out
}

/// 按描述符比较 SAM（`sam`，擦除签名；`inst` 为实例化签名，给出装箱类的具体类型）与实现方法
/// （`impl_params` 含接收者 / 构造器不含，`impl_ret` 构造器为所构造类）的逐位差异。
/// 前 `ncap` 个实现方法形参由捕获实参提供（LambdaMetafactory 要求类型一致），不做适配
pub(super) fn adapt_plan(
    man: &Manifest,
    sam: &MethodDesc,
    inst: Option<&MethodDesc>,
    impl_params: &[FieldType],
    impl_ret: Option<&FieldType>,
    ncap: usize,
) -> Vec<Conv> {
    let mut out = Vec::new();
    for (i, sp) in sam.params.iter().enumerate() {
        let pos = ncap + i;
        let Some(ip) = impl_params.get(pos) else { break };
        let src = inst.and_then(|d| d.params.get(i)).unwrap_or(sp);
        match (src, ip) {
            (FieldType::Prim(p), q) if q.is_reference() => out.push(Conv::BoxArg { pos, prim: *p }),
            (r, FieldType::Prim(p)) if r.is_reference() => {
                for owner in unbox_owners(man, r, *p) {
                    out.push(Conv::UnboxArg { pos, owner, prim: *p });
                }
            }
            _ => {}
        }
    }
    let sam_ret = inst.map_or(sam.ret.as_ref(), |d| d.ret.as_ref().or(sam.ret.as_ref()));
    match (impl_ret, sam_ret) {
        (Some(FieldType::Prim(p)), Some(r)) if r.is_reference() => out.push(Conv::BoxRet { prim: *p }),
        (Some(r), Some(FieldType::Prim(p))) if r.is_reference() => {
            for owner in unbox_owners(man, r, *p) {
                out.push(Conv::UnboxRet { owner, prim: *p });
            }
        }
        _ => {}
    }
    out
}

impl<'a> Engine<'a> {
    /// lambda 创建点：引导静态实参（SAM 擦除签名、实现方法句柄、实例化签名）→ 适配表
    pub(super) fn lambda_plan(&self, bargs: &[Const], imh: &MethodHandle, ncap: usize) -> Vec<Conv> {
        let mt = |i: usize| match bargs.get(i) {
            Some(Const::MethodType(d)) => parse_method(d),
            _ => None,
        };
        let (Some(sam), Some(imd)) = (mt(0), parse_method(&imh.member.desc)) else { return vec![] };
        let inst = mt(2);
        let owner = FieldType::Object(imh.member.owner.clone());
        // 5 invokeVirtual / 7 invokeSpecial / 9 invokeInterface：接收者占首个形参；8 构造器返回所构造类
        let mut params = Vec::with_capacity(imd.params.len() + 1);
        if matches!(imh.kind, 5 | 7 | 9) {
            params.push(owner.clone());
        }
        params.extend(imd.params);
        let ret = if imh.kind == 8 { Some(owner) } else { imd.ret };
        adapt_plan(self.man, &sam, inst.as_ref(), &params, ret.as_ref(), ncap)
    }

    /// 按 lambda 创建时算好的适配表接边：装箱实参改由 `valueOf` 的返回值流入实现方法形参
    pub(super) fn lambda_adapt(&mut self, m: usize, off: u32, l: &Lambda, all: &mut Args, ret: Option<u32>, res: Option<Node>) {
        let via = Via::method("lambda-adapt", m, Some(off));
        for c in &l.adapt {
            match c {
                Conv::BoxArg { pos, prim } => {
                    if let Some(t) = self.box_edge(m, off, *prim, &via, None, None) {
                        if let Some(slot) = all.get_mut(*pos) {
                            *slot = Some(vec![Feed::N(Node::R(t))]);
                        }
                    }
                }
                Conv::UnboxArg { pos, owner, prim } => {
                    let fs = all.get(*pos).cloned().flatten();
                    self.unbox_edge(m, off, owner, *prim, &via, fs);
                }
                Conv::BoxRet { prim } => {
                    self.box_edge(m, off, *prim, &via, ret, res);
                }
                Conv::UnboxRet { owner, prim } => {
                    self.unbox_edge(m, off, owner, *prim, &via, None);
                }
            }
        }
    }

    /// `装箱类.valueOf(基本类型)`：返回值流入 `res`（给出时）
    pub(super) fn box_edge(&mut self, m: usize, off: u32, prim: u8, via: &Via, ret: Option<u32>, res: Option<Node>) -> Option<usize> {
        let owner = self.man.boxed_class(prim)?.to_string();
        let desc = format!("({})L{owner};", prim as char);
        let Some(site) = self.h.resolve_method(&owner, BOX_FACTORY, &desc, false) else {
            self.unresolved.insert(format!("{owner}.{BOX_FACTORY}:{desc}"));
            return None;
        };
        let (o, n, d) = site.key();
        self.init(&o, via.clone());
        let t = self.method(MemberRef { owner: o, name: n, desc: d }, via.clone());
        let wid = self.id(&owner);
        self.edge(m, off, t, Recv::None, &[None], ret.or(Some(wid)), res);
        Some(t)
    }

    /// `owner` 上返回 `prim` 的拆箱方法；接收者取实参来源（未知时按 `owner` open）
    fn unbox_edge(&mut self, m: usize, off: u32, owner: &str, prim: u8, via: &Via, recv: Option<Vec<Feed>>) {
        let Some(cf) = self.h.class(owner) else {
            self.unresolved.insert(owner.to_string());
            return;
        };
        let want = format!("(){}", prim as char);
        let Some(mt) = cf.methods.iter().find(|x| !x.is_static() && x.desc == want && x.name.ends_with(UNBOX_SUFFIX)) else {
            self.unresolved.insert(format!("{owner}.*{UNBOX_SUFFIX}:{want}"));
            return;
        };
        let key = MemberRef { owner: owner.to_string(), name: mt.name.clone(), desc: mt.desc.clone() };
        let oid = self.id(owner);
        let t = self.method(key, via.clone());
        let fs = recv.unwrap_or_else(|| vec![Feed::S(TypeSet::open(oid))]);
        self.edge(m, off, t, Recv::Feeds(fs), &[], None, None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(vm: &str) -> Manifest {
        // 各测试并行运行：目录名带进程内序号，避免互删
        static SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let seq = SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("rava-lambda-adapt-{}-{seq}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("vm_intrinsics.toml"), vm).unwrap();
        let r = Manifest::load(&dir).unwrap();
        std::fs::remove_dir_all(&dir).ok();
        r
    }

    fn md(d: &str) -> MethodDesc {
        parse_method(d).unwrap()
    }

    const VM: &str = "[boxing]\nI = \"t/BoxI\"\nJ = \"t/BoxJ\"\n";

    #[test]
    fn unbox_arg_and_box_ret() {
        // SAM (Object)Object，实例化 (t/BoxI)t/BoxI，实现方法静态 (I)I：实参拆箱、返回装箱
        let man = manifest(VM);
        let impl_d = md("(I)I");
        let plan = adapt_plan(&man, &md("(Lt/O;)Lt/O;"), Some(&md("(Lt/BoxI;)Lt/BoxI;")), &impl_d.params, impl_d.ret.as_ref(), 0);
        assert_eq!(plan, vec![Conv::UnboxArg { pos: 0, owner: "t/BoxI".into(), prim: b'I' }, Conv::BoxRet { prim: b'I' }]);
    }

    #[test]
    fn box_arg_widening_unbox_and_captures() {
        // 1 个捕获实参；SAM (I t/BoxI)V → 实现方法 (捕获, Object, J)V：基本实参装箱；BoxI → long 取两个拆箱类
        let man = manifest(VM);
        let impl_d = md("(Lt/C;Lt/O;J)V");
        let plan = adapt_plan(&man, &md("(ILt/BoxI;)V"), None, &impl_d.params, impl_d.ret.as_ref(), 1);
        assert_eq!(
            plan,
            vec![
                Conv::BoxArg { pos: 1, prim: b'I' },
                Conv::UnboxArg { pos: 2, owner: "t/BoxJ".into(), prim: b'J' },
                Conv::UnboxArg { pos: 2, owner: "t/BoxI".into(), prim: b'J' },
            ]
        );
    }

    #[test]
    fn same_shape_no_adapt() {
        let man = manifest(VM);
        let impl_d = md("(Lt/S;)I");
        let plan = adapt_plan(&man, &md("(Lt/O;)I"), None, &impl_d.params, impl_d.ret.as_ref(), 0);
        assert!(plan.is_empty());
        let plan = adapt_plan(&man, &md("()Lt/C;"), None, &[], Some(&FieldType::Object("t/C".into())), 0);
        assert!(plan.is_empty());
    }

    #[test]
    fn unbox_ret() {
        // 实现方法返回 t/BoxI、SAM 返回 int（ToIntFunction 引用返回 Integer 的方法）
        let man = manifest(VM);
        let plan = adapt_plan(&man, &md("(Lt/O;)I"), None, &[FieldType::Object("t/C".into())], Some(&FieldType::Object("t/BoxI".into())), 0);
        assert_eq!(plan, vec![Conv::UnboxRet { owner: "t/BoxI".into(), prim: b'I' }]);
    }
}
