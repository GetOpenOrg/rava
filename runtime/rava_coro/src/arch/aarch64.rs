//! aarch64（AAPCS64）上下文切换。
//!
//! 切换帧（自低地址起，160 字节，16 字节对齐）：
//! `x19 x20 | x21 x22 | x23 x24 | x25 x26 | x27 x28 | x29(FP) x30(LR) | d8 d9 | d10 d11 | d12 d13 | d14 d15`。
//! 切换函数是普通 `extern "C"` 调用：调用方保存寄存器由编译器在调用点处理，这里只存取被调用方保存寄存器
//! （x19–x28、FP、LR、SP、d8–d15 的低 64 位——AAPCS64 只要求 v8–v15 的低 64 位被保存）。

use core::arch::naked_asm;

/// 切换帧字节数
pub const FRAME: usize = 160;
/// 帧内 x19 槽（新栈：入口数据指针）
pub const SLOT_DATA: usize = 0;
/// 帧内 x20 槽（新栈：入口函数）
pub const SLOT_ENTRY: usize = 8;
/// 帧内 x30 槽（新栈：蹦床地址）
pub const SLOT_RET: usize = 88;

/// 新栈初始帧中控制寄存器的槽位与取值（aarch64 不在切换帧内保存 FPCR，无此类槽位）
pub const CONTROL_SLOTS: &[(usize, u64)] = &[];

/// 保存当前上下文（SP 写入 `*save`），切到 `load` 所指栈顶的切换帧；`arg` 作为对端 `raw_switch`
/// 的返回值（新栈首次进入时作为入口函数第一个参数）
///
/// # Safety
/// `load` 必须是先前由本函数保存或由 [`crate::Context::new`] 准备的栈指针，且其所在栈仍然有效、
/// 不在任何线程上运行。
#[unsafe(naked)]
pub unsafe extern "C" fn raw_switch(save: *mut usize, load: usize, arg: usize) -> usize {
    naked_asm!(
        "sub sp, sp, #160",
        "stp x19, x20, [sp, #0]",
        "stp x21, x22, [sp, #16]",
        "stp x23, x24, [sp, #32]",
        "stp x25, x26, [sp, #48]",
        "stp x27, x28, [sp, #64]",
        "stp x29, x30, [sp, #80]",
        "stp d8, d9, [sp, #96]",
        "stp d10, d11, [sp, #112]",
        "stp d12, d13, [sp, #128]",
        "stp d14, d15, [sp, #144]",
        "mov x9, sp",
        "str x9, [x0]",
        "mov sp, x1",
        "ldp x19, x20, [sp, #0]",
        "ldp x21, x22, [sp, #16]",
        "ldp x23, x24, [sp, #32]",
        "ldp x25, x26, [sp, #48]",
        "ldp x27, x28, [sp, #64]",
        "ldp x29, x30, [sp, #80]",
        "ldp d8, d9, [sp, #96]",
        "ldp d10, d11, [sp, #112]",
        "ldp d12, d13, [sp, #128]",
        "ldp d14, d15, [sp, #144]",
        "add sp, sp, #160",
        "mov x0, x2",
        "ret",
    )
}

/// 新栈第一帧（入口蹦床）：由首次 `raw_switch` 的 `ret` 进入，此时 SP = 栈顶（16 字节对齐）、
/// x0 = 切入参数、x19 = 入口数据、x20 = 入口函数、FP = 0。帧链在此终止：FP = 0、LR 置 0，
/// CFI 声明返回地址未定义，回溯与栈遍历在此干净停止，不走进载体栈。入口函数不得返回。
#[unsafe(naked)]
pub unsafe extern "C" fn trampoline() -> ! {
    naked_asm!(
        ".cfi_startproc",
        ".cfi_undefined x30",
        "mov x29, xzr",
        "mov x30, xzr",
        "mov x1, x19",
        "blr x20",
        "brk #0x1",
        ".cfi_endproc",
    )
}
