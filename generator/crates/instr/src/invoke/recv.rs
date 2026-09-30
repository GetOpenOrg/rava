//! 虚调用接收者解析（← `member_owner.py` 中依赖栈状态的部分：`_close_open_type_args`、
//! `_erase_slot_params`、`_resolve_virtual_sig_params`）。

use ty::{ClassInfo, RsType};

use sim::StackSim;

use crate::env::InstrEnv;
use crate::error::{unported, InstrResult};
use crate::invoke::CallRef;

/// 菱形构造结果（`X<_, A>`）作接收者：待推断实参取擦除，写回构造处 turbofish
/// （`_close_open_type_args`；`stack_idx` 为自栈底的下标）
pub fn close_open_type_args(env: &InstrEnv, sim: &mut StackSim, stack_idx: usize) -> InstrResult<()> {
    let _ = (env, sim, stack_idx);
    unported("close_open_type_args")
}

/// 祖先类声明的虚方法：发射签名字面为 Object 的位对齐为 Object（`_erase_slot_params`）
pub fn erase_slot_params(env: &InstrEnv, owner: &ClassInfo, mname: &str, desc: &str, sig_params: Vec<Option<RsType>>) -> Vec<Option<RsType>> {
    let _ = (env, owner, mname, desc);
    sig_params
}

/// 接收者解析与泛型实参映射（`_resolve_virtual_sig_params`）：会改写接收者栈位
/// （类型变量 → 上界视图、菱形 `_` 实参 → 擦除闭合）
pub fn resolve_virtual_sig_params(env: &InstrEnv, sim: &mut StackSim, call: &CallRef) -> InstrResult<Option<Vec<Option<RsType>>>> {
    let _ = (env, sim, call);
    unported("resolve_virtual_sig_params")
}
