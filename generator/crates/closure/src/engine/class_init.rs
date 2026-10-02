//! 引擎：类初始化事实（vm_intrinsics.toml `[facts.class_init]`）。
//!
//! `initializers` 成员（`Unsafe.ensureClassInitialized` 等）由 VM 初始化以实参传入的类：`ldc` 类字面量不触发初始化
//! （JVMS §5.5），初始化只发生在这次调用里，没有字节码层面的 `<clinit>` 调用边。调用点上 Class 实参值集里的类镜像
//! 即被初始化的类（值集增长时站点重跑）：该类进入初始化层（`<clinit>` 入链），并按调用点记入输出，供生成器登记
//! 初始化钩子。基本类型类镜像（[`Engine::primitive_mirror`]）与数组类没有初始化，跳过。值集含所指未知的 Class（open / 非镜像值）时记 `unknown`——无法枚举，由生成器对闭包内全部带
//! `<clinit>` 的类登记钩子兜底。

use super::*;

#[derive(Default)]
pub struct ClassInitFacts {
    /// 调用点（`调用方成员@偏移`）→ 被初始化的类
    pub sites: BTreeMap<String, BTreeSet<String>>,
    /// 出现过所指未知的 Class 实参
    pub unknown: bool,
}

impl ClassInitFacts {
    /// 全部被初始化的类（各调用点并集）
    pub fn targets(&self) -> BTreeSet<&str> {
        self.sites.values().flatten().map(String::as_str).collect()
    }
}

impl<'a> Engine<'a> {
    /// 调用点（方法 m、偏移 off）若是类初始化入口，按 Class 实参初始化所指类
    pub(super) fn class_init_site(&mut self, m: usize, off: u32, opcode: u8, mref: &MemberRef, args: &[V]) {
        let Some(j) = self.man.class_initializer(&self.mref_key(mref)) else { return };
        let skip = usize::from(opcode != classfile::op::INVOKESTATIC);
        let Some(a) = args.get(skip + j) else { return };
        let mut known: Vec<String> = vec![];
        match a {
            V::Class(c, _) => known.push(c.to_string()),
            V::Null => {}
            V::Ref { .. } => {
                let class = self.id(CLASS);
                let fs = self.feeds(m, a, class);
                let s = self.value_set(&fs);
                self.class_init.unknown |= !s.open.is_empty();
                for x in s.classes.iter() {
                    match self.mirrors.get(&x) {
                        Some(&c) => known.push(self.names[c as usize].to_string()),
                        // 基本类型类没有初始化
                        None if Some(x) == self.prim_mirror => {}
                        None => self.class_init.unknown = true,
                    }
                }
            }
            _ => self.class_init.unknown = true,
        }
        let site = format!("{}@{off}", self.methods[m].key);
        // 数组类没有初始化（JVMS §5.5）
        for c in known.into_iter().filter(|c| !c.starts_with('[')) {
            if !self.class_init.sites.get(&site).is_some_and(|s| s.contains(&c)) {
                self.init(&c, Via::method("class-init", m, Some(off)));
                self.class_init.sites.entry(site.clone()).or_default().insert(c);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 各调用点的被初始化类取并集输出
    #[test]
    fn targets_union_sites() {
        let mut f = ClassInitFacts::default();
        f.sites.entry("a/A.m:()V@3".into()).or_default().extend(["a/X".to_string(), "a/Y".to_string()]);
        f.sites.entry("a/B.n:()V@7".into()).or_default().insert("a/X".into());
        assert_eq!(f.targets().into_iter().collect::<Vec<_>>(), vec!["a/X", "a/Y"]);
    }
}
