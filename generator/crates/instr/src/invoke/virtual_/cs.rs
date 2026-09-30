//! @CallerSensitive 虚调用的包装（← `invoke.py` 的 `caller_sensitive_wrap` /
//! `_is_caller_sensitive`，与 `invoke_virtual._CS_CTX` 的判定上下文）。
//!
//! Python 以模块级 `_CS_CTX` 在 `_gen_invokevirtual` 入口设置；这里是显式传递的 [`CsCtx`]。

use crate::env::InstrEnv;
use crate::invoke::CallRef;

/// 当前虚调用的 @CallerSensitive 判定上下文：常量池声明类 / 方法名 / 描述符 / 调用处类
pub struct CsCtx<'c> {
    owner: &'c str,
    mname: &'c str,
    desc: &'c str,
    caller: &'c str,
}

impl<'c> CsCtx<'c> {
    pub fn new(call: &'c CallRef, caller: &'c str) -> CsCtx<'c> {
        CsCtx { owner: &call.owner, mname: &call.name, desc: &call.desc, caller }
    }

    /// 包装是否生效（`caller_sensitive_wrap('\0', ..) != '\0'`）
    pub fn applies(&self, env: &InstrEnv) -> bool {
        // Python 对 `getCallerClass` 的不包装例外只作用于静态调用（该方法是 static native），
        // 虚调用路径不可达，不移植
        is_caller_sensitive(env, self.owner, self.mname, self.desc)
    }

    /// `recv.m(args)`，生效时包为 `__caller_sensitive("调用处类", || call)`（`_build_call`）
    pub fn build_call(&self, env: &InstrEnv, rust_mname: &str, recv: &str, args: &str) -> String {
        let call = format!("{recv}.{rust_mname}({args})");
        if self.applies(env) {
            format!("__caller_sensitive(\"{}\", || {call})", self.caller)
        } else {
            call
        }
    }
}

/// 被调方法（沿超类链解析声明处）是否标注清单登记的 caller-sensitive 注解
fn is_caller_sensitive(env: &InstrEnv, owner: &str, mname: &str, desc: &str) -> bool {
    crate::invoke::bind::caller_sensitive_decl(env, owner, mname, desc).0
}
