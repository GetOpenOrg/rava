//! 被调用方保存寄存器逐个破坏后恢复的检查（x19–x29 / d8–d15；rbx / rbp / r12–r15 / MXCSR / x87 控制字）。
//!
//! 两侧各用不同的「盐」把全部被调用方保存寄存器置为哨兵值后直接调用切换汇编，恢复后逐个比对：
//! 对端置入的不同值若泄漏过来即判失败。x86_64 另把 MXCSR 舍入位与 x87 控制字设为两侧不同的值。

use rava_coro::{raw_switch, Context, Stack};
use std::arch::naked_asm;

/// 置哨兵 → 切到 `*load` → 恢复后比对，返回不一致位图（0 = 全部恢复）
#[cfg(target_arch = "aarch64")]
#[unsafe(naked)]
unsafe extern "C" fn probe(save: *mut Context, load: *const Context, arg: usize, salt: usize) -> u64 {
    naked_asm!(
        "sub sp, sp, #176",
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
        "str x3, [sp, #160]",
        "add x19, x3, #19",
        "add x20, x3, #20",
        "add x21, x3, #21",
        "add x22, x3, #22",
        "add x23, x3, #23",
        "add x24, x3, #24",
        "add x25, x3, #25",
        "add x26, x3, #26",
        "add x27, x3, #27",
        "add x28, x3, #28",
        "add x29, x3, #29",
        "add x9, x3, #108",
        "fmov d8, x9",
        "add x9, x3, #109",
        "fmov d9, x9",
        "add x9, x3, #110",
        "fmov d10, x9",
        "add x9, x3, #111",
        "fmov d11, x9",
        "add x9, x3, #112",
        "fmov d12, x9",
        "add x9, x3, #113",
        "fmov d13, x9",
        "add x9, x3, #114",
        "fmov d14, x9",
        "add x9, x3, #115",
        "fmov d15, x9",
        "ldr x1, [x1]",
        "bl {sw}",
        "ldr x3, [sp, #160]",
        "mov x9, xzr",
        "add x10, x3, #19",
        "cmp x19, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #0",
        "add x10, x3, #20",
        "cmp x20, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #1",
        "add x10, x3, #21",
        "cmp x21, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #2",
        "add x10, x3, #22",
        "cmp x22, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #3",
        "add x10, x3, #23",
        "cmp x23, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #4",
        "add x10, x3, #24",
        "cmp x24, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #5",
        "add x10, x3, #25",
        "cmp x25, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #6",
        "add x10, x3, #26",
        "cmp x26, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #7",
        "add x10, x3, #27",
        "cmp x27, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #8",
        "add x10, x3, #28",
        "cmp x28, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #9",
        "add x10, x3, #29",
        "cmp x29, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #10",
        "fmov x12, d8",
        "add x10, x3, #108",
        "cmp x12, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #11",
        "fmov x12, d9",
        "add x10, x3, #109",
        "cmp x12, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #12",
        "fmov x12, d10",
        "add x10, x3, #110",
        "cmp x12, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #13",
        "fmov x12, d11",
        "add x10, x3, #111",
        "cmp x12, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #14",
        "fmov x12, d12",
        "add x10, x3, #112",
        "cmp x12, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #15",
        "fmov x12, d13",
        "add x10, x3, #113",
        "cmp x12, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #16",
        "fmov x12, d14",
        "add x10, x3, #114",
        "cmp x12, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #17",
        "fmov x12, d15",
        "add x10, x3, #115",
        "cmp x12, x10",
        "cset x11, ne",
        "orr x9, x9, x11, lsl #18",
        "mov x0, x9",
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
        "add sp, sp, #176",
        "ret",
        sw = sym raw_switch,
    )
}

/// 置哨兵 → 切到 `*load` → 恢复后比对，返回不一致位图（0 = 全部恢复）
#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
unsafe extern "C" fn probe(save: *mut Context, load: *const Context, arg: usize, salt: usize) -> u64 {
    naked_asm!(
        "push rbp",
        "push rbx",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        "sub rsp, 24",
        "mov [rsp], rcx",
        "stmxcsr [rsp + 8]",
        "fnstcw [rsp + 12]",
        "mov eax, ecx",
        "and eax, 3",
        "shl eax, 13",
        "or eax, 0x1F80",
        "mov [rsp + 16], eax",
        "ldmxcsr [rsp + 16]",
        "mov eax, ecx",
        "and eax, 3",
        "shl eax, 10",
        "or eax, 0x037F",
        "mov [rsp + 20], ax",
        "fldcw [rsp + 20]",
        "lea rbp, [rcx + 5]",
        "lea rbx, [rcx + 3]",
        "lea r12, [rcx + 12]",
        "lea r13, [rcx + 13]",
        "lea r14, [rcx + 14]",
        "lea r15, [rcx + 15]",
        "mov rsi, [rsi]",
        "call {sw}",
        "mov rcx, [rsp]",
        "xor eax, eax",
        "lea r8, [rcx + 5]",
        "cmp rbp, r8",
        "setne r9b",
        "movzx r9d, r9b",
        "shl r9d, 0",
        "or eax, r9d",
        "lea r8, [rcx + 3]",
        "cmp rbx, r8",
        "setne r9b",
        "movzx r9d, r9b",
        "shl r9d, 1",
        "or eax, r9d",
        "lea r8, [rcx + 12]",
        "cmp r12, r8",
        "setne r9b",
        "movzx r9d, r9b",
        "shl r9d, 2",
        "or eax, r9d",
        "lea r8, [rcx + 13]",
        "cmp r13, r8",
        "setne r9b",
        "movzx r9d, r9b",
        "shl r9d, 3",
        "or eax, r9d",
        "lea r8, [rcx + 14]",
        "cmp r14, r8",
        "setne r9b",
        "movzx r9d, r9b",
        "shl r9d, 4",
        "or eax, r9d",
        "lea r8, [rcx + 15]",
        "cmp r15, r8",
        "setne r9b",
        "movzx r9d, r9b",
        "shl r9d, 5",
        "or eax, r9d",
        "stmxcsr [rsp + 4]",
        "mov r8d, [rsp + 4]",
        "cmp r8d, [rsp + 16]",
        "setne r9b",
        "movzx r9d, r9b",
        "shl r9d, 6",
        "or eax, r9d",
        "fnstcw [rsp + 4]",
        "movzx r8d, word ptr [rsp + 4]",
        "movzx r10d, word ptr [rsp + 20]",
        "cmp r8d, r10d",
        "setne r9b",
        "movzx r9d, r9b",
        "shl r9d, 7",
        "or eax, r9d",
        "ldmxcsr [rsp + 8]",
        "fldcw [rsp + 12]",
        "add rsp, 24",
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop rbx",
        "pop rbp",
        "ret",
        sw = sym raw_switch,
    )
}

struct Shared {
    main: Context,
    co: Context,
    co_mask: u64,
    rounds: usize,
    stop: bool,
}

const MAIN_SALT: usize = 0x1000;
const CO_SALT: usize = 0x5003;

unsafe extern "C" fn entry(_: usize, data: *mut u8) -> ! {
    let s = data as *mut Shared;
    loop {
        let m = probe(&mut (*s).co, &(*s).main, 0, CO_SALT);
        (*s).co_mask |= m;
        (*s).rounds += 1;
        if (*s).stop {
            break;
        }
    }
    rava_coro::switch(&mut (*s).co, &mut (*s).main, 0);
    unreachable!();
}

#[test]
fn callee_saved_registers_survive() {
    let stack = Stack::new().expect("stack");
    let mut s = Box::new(Shared { main: Context::empty(), co: Context::empty(), co_mask: 0, rounds: 0, stop: false });
    let sp: *mut Shared = &mut *s;
    let mut main_mask = 0;
    unsafe {
        (*sp).co = Context::new(&stack, entry, sp.cast());
        for i in 0..1000 {
            (*sp).stop = i == 999;
            main_mask |= probe(&mut (*sp).main, &(*sp).co, 0, MAIN_SALT);
        }
    }
    assert_eq!(main_mask, 0, "载体侧寄存器未恢复，位图 {main_mask:#x}");
    assert_eq!(s.co_mask, 0, "协程侧寄存器未恢复，位图 {:#x}", s.co_mask);
    assert_eq!(s.rounds, 999);
    drop(stack);
}
