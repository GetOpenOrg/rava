//! 引擎：引导档位上下文（计划 2026-10-05-boot-image-evaluator §5.5.2 D7）。
//!
//! 映像的残差步骤（残差调用、残差区段）在运行期按构建期的引导档位重放（`VM.initLevel`，清单
//! `[concrete.boot] level`）：initPhase1 的残差在档位 0 执行，此时 `isBooted()` 为 false。全局的
//! `[facts.returns]` 按「进入 main 时已引导完成」折叠这些查询，对残差不成立。
//!
//! 档位上下文是一种克隆上下文：残差步骤的被调方法在档位上下文中建节点，档位上下文中的方法发出的同步调用
//! （调用 / 派发 / lambda / 反射 / 方法句柄……）其目标一律进入同一档位上下文（不再按接收者克隆）；档位上下文
//! 中触发的类初始化另在档位上下文中登记 `<clinit>`（运行期由重放首次触发时在该档位执行）。档位上下文中的
//! 方法按 `[concrete.boot.level_queries]` 折叠引导查询（档位 ≥ 门限为 true），摘要不与本体共享；
//! 派发枢纽按档位分族。手写方法不克隆：经手写方法回调的字节码回到本体（已知局限）。

use super::*;
use crate::image::IVal;

/// 目标与调用方同步执行的溯源类别（调用方的档位传到目标）
const SYNC_KINDS: &[&str] = &["invoke", "dispatch", "lambda", "lambda-adapt", "indy", "method-handle", "reflect", "concrete", "service-provider"];

/// 进入调用方档位上下文的溯源类别：只有直接调用（静态 / 特殊 / 构造），且目标不另按接收者 / 选择子克隆
const LEVEL_KINDS: &[&str] = &["invoke"];

impl Engine<'_> {
    /// 方法节点所在的档位（None = 非档位上下文）
    pub(super) fn level_of(&self, m: usize) -> Option<i32> {
        if self.level_ctxs.is_empty() {
            return None;
        }
        self.level_ctxs.get(&self.methods[m].ctx).copied()
    }

    /// 档位 l 的上下文
    pub(super) fn level_ctx(&mut self, l: i32) -> u32 {
        let chain = format!("@level:{l}");
        if let Some(&id) = self.ids.get(chain.as_str()) {
            self.level_ctxs.insert(id, l);
            return id;
        }
        let id = self.id(&chain);
        self.obj_chain.insert(id, Rc::from(chain));
        self.level_ctxs.insert(id, l);
        id
    }

    /// 档位 l 需要独立上下文：有引导查询在该档位与全局事实不同（门限高于档位）
    pub(super) fn level_needed(&self, l: i32) -> bool {
        self.man.concrete.boot.level_queries.values().any(|&t| i64::from(l) < t)
    }

    /// 同步调用溯源的调用方档位上下文（NOCTX = 无）
    pub(super) fn via_level(&self, via: &Via) -> u32 {
        if self.level_ctxs.is_empty() || !SYNC_KINDS.contains(&via.kind) {
            return NOCTX;
        }
        self.via_level_any(via)
    }

    /// 溯源方法（任意类别）的档位上下文（NOCTX = 无）
    pub(super) fn via_level_any(&self, via: &Via) -> u32 {
        match via.from {
            From::Method(m) if self.level_ctxs.contains_key(&self.methods[m].ctx) => self.methods[m].ctx,
            _ => NOCTX,
        }
    }

    /// 方法节点的上下文：档位上下文中的同步调用进入调用方档位；非档位调用方不进入档位上下文
    /// （如档位上下文中创建、在本体中调用的 lambda）
    pub(super) fn level_override(&self, ctx: u32, via: &Via) -> u32 {
        if self.level_ctxs.is_empty() {
            return ctx;
        }
        let lc = self.via_level(via);
        if lc != NOCTX && (ctx == NOCTX || self.level_ctxs.contains_key(&ctx)) && LEVEL_KINDS.contains(&via.kind) {
            return lc;
        }
        if matches!(via.from, From::Method(_)) && SYNC_KINDS.contains(&via.kind) && self.level_ctxs.contains_key(&ctx) {
            return NOCTX;
        }
        ctx
    }

    /// 档位上下文中触发的类初始化：超类链、带默认方法的超接口、`<clinit>` 在档位上下文中登记
    /// （映像中已初始化的类运行期不再初始化）
    pub(super) fn level_init(&mut self, cls: &str, lc: u32) {
        if cls.starts_with('[') || self.img.as_ref().is_some_and(|s| s.is_build_time(cls)) {
            return;
        }
        if !self.level_inited.insert((cls.to_string(), lc)) {
            return;
        }
        let Some(cf) = self.h.class(cls) else { return };
        if !cf.is_interface() {
            if let Some(s) = &cf.super_name {
                self.level_init(&s.clone(), lc);
            }
            for i in self.h.all_superinterfaces(&cf) {
                if i.methods.iter().any(|m| !m.is_static() && !m.is_abstract() && !m.is_private()) {
                    self.level_init(&i.name.clone(), lc);
                }
            }
        }
        if cf.method("<clinit>", "()V").is_some() {
            let k = MemberRef { owner: cls.to_string(), name: "<clinit>".into(), desc: "()V".into() };
            self.method_ctx(k, lc, Via::class("clinit", cls));
        }
    }

    /// 残差步骤的被调方法作根（ctx = 档位上下文或 NOCTX；形参取构建期记录的实参，无记录时 open；
    /// 返回值交给运行期重放）
    pub(super) fn root_in(&mut self, key: MemberRef, kind: &'static str, ctx: u32, args: Option<&[IVal]>) -> usize {
        let via = Via::root(kind, &key.to_string());
        self.init(&key.owner.clone(), via.clone());
        if ctx != NOCTX {
            self.level_init(&key.owner.clone(), ctx);
        }
        let m = self.method_ctx(key, ctx, via);
        match args {
            Some(a) if a.len() == self.methods[m].ptypes.len() => self.image_args(m, a),
            _ => self.open_params(m),
        }
        self.returns_to_vm(m);
        m
    }
}
