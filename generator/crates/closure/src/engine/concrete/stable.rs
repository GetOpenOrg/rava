//! 映像状态的稳定性：哪些映像对象字段 / 静态字段在类初始化之后不再被改写，求值时可读。
//!
//! - 类型：清单 `[concrete] stable_types` 逐类声明的不可变契约（含子类型）：实例的全部字段可读（`Vm::get_field`），
//!   声明类的静态字段所指映像数组视为冻结（如字符属性表）；
//! - 静态字段：非 final 的静态字段若全部写入点都在声明类的 `<clinit>` 中，初始化完成后即事实上不变。
//!   写入点按访问控制界定范围：private / 包私有字段只可能由同包的类写入（嵌套成员同包），扫描该包全部类的
//!   `putstatic`；public / protected 字段的写入方不可穷举，不算稳定。

use classfile::{acc, op, ClassFile, Field, Operand};
use resolve::classpath::Origin;

use super::vm::*;
use super::*;

impl Vm {
    /// 对象类型是否为发布后不再改写的类型（按类型缓存）
    pub(super) fn stable(&mut self, env: &Env, o: u32) -> bool {
        let ty = self.ty(o);
        if let Some(&b) = self.stable_ty.get(&ty) {
            return b;
        }
        let b = env.cfg().stable_types.iter().any(|s| env.h().is_subtype(&ty, s));
        self.stable_ty.insert(ty, b);
        b
    }

    /// 读自稳定类型静态字段的映像数组冻结（初始化后只读）
    pub(super) fn freeze_static(&mut self, env: &Env, decl: &str, v: CV) {
        let CV::R(a) = v else { return };
        let h = &self.heap[a as usize];
        if h.epoch != 0 || !matches!(h.body, Body::Arr(_)) || self.frozen.contains(&a) {
            return;
        }
        if env.cfg().stable_types.iter().any(|s| env.h().is_subtype(decl, s)) {
            self.frozen.insert(a);
        }
    }

    /// 静态字段的写入点是否全部在声明类的 `<clinit>` 中
    pub(super) fn clinit_only(&mut self, env: &Env, decl: &ClassFile, fd: &Field) -> bool {
        if fd.access & (acc::PUBLIC | acc::PROTECTED) != 0 {
            return false;
        }
        let pkg = resolve::hierarchy::package_of(&decl.name).to_string();
        let names = self.package(env, &pkg);
        for n in names.iter() {
            let Some(cf) = env.h().class(n) else { continue };
            for m in &cf.methods {
                let clinit = cf.name == decl.name && m.is_clinit();
                let Some(code) = m.code.as_ref() else { continue };
                for x in &code.insns {
                    let Operand::Field(f) = &x.operand else { continue };
                    if x.opcode != op::PUTSTATIC || f.name != fd.name || f.desc != fd.desc || clinit {
                        continue;
                    }
                    let hit = env.h().resolve_field(&f.owner, &f.name, &f.desc).is_none_or(|s| s.class.name == decl.name);
                    if hit {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// 包内全部类名（首次调用时按类路径建索引）
    fn package(&mut self, env: &Env, pkg: &str) -> Rc<[String]> {
        let idx = self.pkgs.get_or_insert_with(|| {
            let mut idx: HashMap<String, Vec<String>> = HashMap::default();
            for o in [Origin::User, Origin::Lib, Origin::Jdk, Origin::Image] {
                for n in env.cp.names_of(o) {
                    idx.entry(resolve::hierarchy::package_of(&n).to_string()).or_default().push(n);
                }
            }
            idx
        });
        Rc::from(idx.get(pkg).cloned().unwrap_or_default())
    }
}
