//! 引擎：按类镜像强制类初始化（`[facts.reflect] class_initializers`，如 `Unsafe.ensureClassInitialized`）。
//!
//! JDK 以「初始化某类以运行其 `<clinit>` 的副作用」登记跨包访问器（`SharedSecrets.javaUtilJarAccess()`：
//! `ensureClassInitialized(JarFile.class)` 后读 `javaUtilJarAccess`，由 `JarFile.<clinit>` 写入）。
//! 不建模时 `<clinit>` 的写入缺席，字段按初值 null 折叠——属未建模写入来源。Class 实参所指类
//! （类字面量、值集里的类镜像）按 JVMS §5.5 初始化；值集含推不出所指类的镜像记为反射缺口。

use super::*;

impl Engine<'_> {
    pub(super) fn mirror_init_site(&mut self, m: usize, off: u32, mref: &MemberRef, k: &str, opcode: u8, args: &[V]) {
        if !self.man.is_class_initializer(k) {
            return;
        }
        let Some(md) = parse_method(&mref.desc) else { return };
        let skip = usize::from(opcode != classfile::op::INVOKESTATIC);
        let class = self.id(CLASS);
        let site = format!("{k} @ {}@{off}", self.methods[m].key);
        let mut targets: Vec<String> = vec![];
        for (p, a) in md.params.iter().zip(args.iter().skip(skip)) {
            if !matches!(p, FieldType::Object(c) if c == CLASS) {
                continue;
            }
            if let V::Class(c, _) = a {
                targets.push(c.to_string());
                continue;
            }
            let fs = self.feeds(m, a, class);
            let s = self.value_set(&fs);
            for x in s.classes.iter() {
                match self.mirrors.get(&x) {
                    Some(&c) => targets.push(self.names[c as usize].to_string()),
                    None => {
                        let what = self.names[x as usize].to_string();
                        self.reflect_gaps.insert(format!("{site} <- {what}"));
                    }
                }
            }
            for o in &s.open {
                self.reflect_gaps.insert(format!("{site} <- open({})", self.names[o as usize]));
            }
        }
        for c in targets {
            if !c.starts_with('[') {
                self.seeds.mirror_inits.insert(c.clone());
            }
            self.init(&c, Via::method("mirror-init", m, Some(off)));
        }
    }
}
