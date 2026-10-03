//! x86_64（System V）上下文切换。
//!
//! 切换帧（自低地址起，64 字节）：`MXCSR(4) x87CW(2) pad(2) | r15 | r14 | r13 | r12 | rbx | rbp | 返回地址`。
//! 切换函数是普通 `extern "C"` 调用：调用方保存寄存器由编译器在调用点处理，这里只存取被调用方保存寄存器
//! （rbx、rbp、r12–r15、RSP、RIP）以及 SysV 规定由被调用方维持的 MXCSR 控制位与 x87 控制字。

use core::arch::naked_asm;

/// 切换帧字节数（含返回地址）
pub const FRAME: usize = 64;
/// 帧内 rbx 槽（新栈：入口数据指针）
pub const SLOT_DATA: usize = 40;
/// 帧内 r12 槽（新栈：入口函数）
pub const SLOT_ENTRY: usize = 32;
/// 帧内返回地址槽（新栈：蹦床地址）
pub const SLOT_RET: usize = 56;

/// 新栈初始帧中控制寄存器的取值：MXCSR 缺省 0x1F80（全部异常屏蔽、就近舍入），x87 控制字缺省 0x037F
pub const CONTROL_SLOTS: &[(usize, u64)] = &[(0, 0x1F80 | (0x037F << 32))];

/// 保存当前上下文（RSP 写入 `*save`），切到 `load` 所指栈顶的切换帧；`arg` 作为对端 `raw_switch`
/// 的返回值（新栈首次进入时作为入口函数第一个参数）
///
/// # Safety
/// `load` 必须是先前由本函数保存或由 [`crate::Context::new`] 准备的栈指针，且其所在栈仍然有效、
/// 不在任何线程上运行。
#[unsafe(naked)]
pub unsafe extern "C" fn raw_switch(save: *mut usize, load: usize, arg: usize) -> usize {
    naked_asm!(
        "push rbp",
        "push rbx",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        "sub rsp, 8",
        "stmxcsr [rsp]",
        "fnstcw [rsp + 4]",
        "mov [rdi], rsp",
        "mov rsp, rsi",
        "ldmxcsr [rsp]",
        "fldcw [rsp + 4]",
        "add rsp, 8",
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop rbx",
        "pop rbp",
        "mov rax, rdx",
        "ret",
    )
}

/// 新栈第一帧（入口蹦床）：由首次 `raw_switch` 的 `ret` 进入，此时 RSP = 栈顶（16 字节对齐）、
/// rax = 切入参数、rbx = 入口数据、r12 = 入口函数、RBP = 0。帧链在此终止：RBP = 0，
/// CFI 声明返回地址未定义，回溯与栈遍历在此干净停止，不走进载体栈。入口函数不得返回。
#[unsafe(naked)]
pub unsafe extern "C" fn trampoline() -> ! {
    naked_asm!(
        ".cfi_startproc",
        ".cfi_undefined rip",
        "xor ebp, ebp",
        "mov rdi, rax",
        "mov rsi, rbx",
        "call r12",
        "ud2",
        ".cfi_endproc",
    )
}
