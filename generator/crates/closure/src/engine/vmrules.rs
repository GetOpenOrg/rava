//! 引擎：VM 规则——VM 自身（而非字节码调用点）驱动的运行时 fn 入口。
//!
//! 每条规则：规则 ID、依据（JVMS / JLS / JNI 条款）、触发指令、运行时实现 fn 名。可达字节码方法里出现
//! 触发指令（无触发指令的规则在分析开始时触发）即把对应运行时 fn 作为手写 fn 节点入图（见 `rtfn.rs`），
//! 其手写体（构造 VM 异常对象、调用 shutdown 序列等）照常建模。运行时 fn 按名在模块单元里定位，
//! 不按文件特判。

use super::*;
use classfile::op;

pub(super) struct VmRule {
    pub id: &'static str,
    /// 规范依据
    pub basis: &'static str,
    /// 触发指令（空 = 无条件：VM 启动 / 退出序列）
    pub ops: &'static [u8],
    /// 运行时实现 fn 名
    pub fns: &'static [&'static str],
}

/// 数组元素存取指令（`iaload`..`saload`、`iastore`..`sastore`）
const fn is_array_access(o: u8) -> bool {
    (o >= op::IALOAD && o <= op::SALOAD) || (o >= op::IASTORE && o <= op::SASTORE)
}

const NULL_CHECKED: &[u8] = &[
    op::GETFIELD,
    op::PUTFIELD,
    op::INVOKEVIRTUAL,
    op::INVOKESPECIAL,
    op::INVOKEINTERFACE,
    op::ATHROW,
    op::ARRAYLENGTH,
    op::MONITORENTER,
    op::MONITOREXIT,
];
const ALLOCS: &[u8] = &[op::NEW, op::NEWARRAY, op::ANEWARRAY, op::MULTIANEWARRAY];
const NEW_ARRAYS: &[u8] = &[op::NEWARRAY, op::ANEWARRAY, op::MULTIANEWARRAY];
/// 类初始化的触发指令（JVMS §5.5）
const INIT_TRIGGERS: &[u8] = &[op::NEW, op::GETSTATIC, op::PUTSTATIC, op::INVOKESTATIC];
/// 数组存取规则的占位触发（按 [`is_array_access`] 判定）
const ARRAY_ACCESS: &[u8] = &[op::IALOAD];

pub(super) const VM_RULES: &[VmRule] = &[
    VmRule { id: "vm-entry", basis: "JNI Invocation API CreateJavaVM / DestroyJavaVM；JLS §12.1 启动、§12.8 退出（shutdown 序列）", ops: &[], fns: &["create_java_vm", "destroy_java_vm"] },
    // StackOverflowError：宏在每个生成方法入口注入 __stack_check（lib.rs），耗尽时经 JvmError::stack_overflow 构造；
    // 注入调用对分析器不可见，规则落到构造入口（与 null_pointer / out_of_memory 同口径，a3-T1b）
    VmRule { id: "stack-check", basis: "JVMS §2.5.2 / §2.5.6：方法调用入口 Java 虚拟机栈耗尽", ops: &[], fns: &["stack_overflow"] },
    // 线程未捕获异常的默认报告（输出异常描述）
    VmRule { id: "uncaught", basis: "JLS §11.3：未捕获异常终结线程，由默认未捕获异常处理报告", ops: &[], fns: &["report_uncaught_in"] },
    // NullPointerException
    VmRule { id: "null-check", basis: "JVMS §6.5 getfield / putfield / invoke* / athrow / arraylength / monitor* / 数组存取：objectref 为 null", ops: NULL_CHECKED, fns: &["null_pointer"] },
    VmRule { id: "null-check-array", basis: "JVMS §6.5 *aload / *astore：arrayref 为 null", ops: ARRAY_ACCESS, fns: &["null_pointer"] },
    // ArrayIndexOutOfBoundsException
    VmRule { id: "array-bounds", basis: "JVMS §6.5 *aload / *astore：index 越界", ops: ARRAY_ACCESS, fns: &["array_index_out_of_bounds"] },
    // ArrayStoreException
    VmRule { id: "array-store", basis: "JVMS §6.5 aastore：值的运行时类型与分量类型不兼容", ops: &[op::AASTORE], fns: &["array_store"] },
    // NegativeArraySizeException
    VmRule { id: "array-size", basis: "JVMS §6.5 newarray / anewarray / multianewarray：count < 0", ops: NEW_ARRAYS, fns: &["negative_array_size"] },
    // ArithmeticException
    VmRule { id: "div-zero", basis: "JVMS §6.5 idiv / irem / ldiv / lrem：除数为 0", ops: &[op::IDIV, op::IREM, op::LDIV, op::LREM], fns: &["arithmetic"] },
    // ClassCastException
    VmRule { id: "checkcast", basis: "JVMS §6.5 checkcast：objectref 不能转换为目标类型", ops: &[op::CHECKCAST], fns: &["class_cast"] },
    // IllegalMonitorStateException
    VmRule { id: "monitor-owner", basis: "JVMS §6.5 monitorexit：当前线程不持有监视器", ops: &[op::MONITOREXIT], fns: &["illegal_monitor_state"] },
    // ExceptionInInitializerError / NoClassDefFoundError
    VmRule { id: "class-init", basis: "JVMS §5.5 步骤 5 / 11：erroneous 类的再次主动使用、<clinit> 异常收尾", ops: INIT_TRIGGERS, fns: &["in_initializer", "no_class_def_found"] },
    // OutOfMemoryError
    VmRule { id: "heap-exhausted", basis: "JVMS §6.3 / §6.5 new 与数组分配：堆不足", ops: ALLOCS, fns: &["out_of_memory"] },
];

impl<'a> Engine<'a> {
    /// 触发规则 `i`（每条至多一次）：其运行时 fn 入图
    fn fire_vm_rule(&mut self, i: usize) {
        if self.vm_rules_fired & (1 << i) != 0 {
            return;
        }
        self.vm_rules_fired |= 1 << i;
        let r = &VM_RULES[i];
        for f in r.fns {
            let hosts = self.hw.units_with_fn(f);
            if hosts.is_empty() {
                self.unresolved.insert(format!("vm-rule {}：{f}", r.id));
            }
            for h in hosts {
                self.rt_fn_node(&h, f, Via::root("vm-rule", &format!("{}：{}", r.id, r.basis)));
            }
        }
    }

    /// 无触发指令的规则（VM 启动 / 退出序列）
    pub fn root_vm_rules(&mut self) {
        for i in 0..VM_RULES.len() {
            if VM_RULES[i].ops.is_empty() {
                self.fire_vm_rule(i);
            }
        }
    }

    /// 字节码方法的可达指令触发规则
    pub(super) fn vm_rules_scan(&mut self, m: usize, a: &Analysis) {
        let all = (1u64 << VM_RULES.len()) - 1;
        if self.vm_rules_fired == all {
            return;
        }
        let key = &self.methods[m].key;
        let Some(cf) = self.h.class(&key.owner) else { return };
        let Some(code) = cf.method(&key.name, &key.desc).and_then(|x| x.code.as_ref()) else { return };
        let mut seen = [false; 256];
        for (x, live) in code.insns.iter().zip(a.reachable.iter()) {
            if *live {
                seen[x.opcode as usize] = true;
            }
        }
        let array = (0..=255u8).any(|o| seen[o as usize] && is_array_access(o));
        for i in 0..VM_RULES.len() {
            let r = &VM_RULES[i];
            let hit = if std::ptr::eq(r.ops, ARRAY_ACCESS) { array } else { r.ops.iter().any(|&o| seen[o as usize]) };
            if hit && !r.ops.is_empty() {
                self.fire_vm_rule(i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 栈耗尽规则：无条件触发（任何方法调用入口都可能栈耗尽），落到 JvmError 构造入口
    #[test]
    fn stack_check_rule_is_unconditional() {
        let r = VM_RULES.iter().find(|r| r.id == "stack-check").expect("缺 stack-check 规则");
        assert!(r.ops.is_empty(), "stack-check 应为无条件规则");
        assert_eq!(r.fns, &["stack_overflow"]);
    }

    /// 规则数不超出触发位图宽度
    #[test]
    fn rules_fit_fired_bitmap() {
        assert!(VM_RULES.len() < 64);
    }
}
