//! 字节码指令解码（JVMS §6.5）：操作数结构化，符号引用在解码期解出。

use crate::constant::{Const, ConstantPool, MemberRef};
use crate::reader::Reader;
use crate::Error;

/// 结构化操作数。分支目标均为绝对字节偏移。
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    None,
    /// bipush / sipush
    Int(i32),
    /// 局部变量槽（xload / xstore / ret，含 wide 形态）
    Local(u16),
    Iinc { index: u16, delta: i16 },
    Branch(u32),
    TableSwitch { default: u32, low: i32, high: i32, targets: Vec<u32> },
    LookupSwitch { default: u32, pairs: Vec<(i32, u32)> },
    /// getstatic / putstatic / getfield / putfield
    Field(MemberRef),
    /// invokevirtual / invokespecial / invokestatic / invokeinterface；bool = InterfaceMethodref
    Method(MemberRef, bool),
    /// invokedynamic：(常量池下标, bootstrap 索引, 名字, 描述符)。常量池下标是调用点身份
    /// （lambda 站点变量名 `__lam_{idx}` 等的来源）
    InvokeDynamic { index: u16, bsm: u16, name: String, desc: String },
    /// new / anewarray / checkcast / instanceof：binary name 或数组描述符
    Class(String),
    MultiANewArray(String, u8),
    /// newarray 基本类型代码（4..=11）
    NewArray(u8),
    Ldc(Const),
}

#[derive(Debug, Clone)]
pub struct Insn {
    pub offset: u32,
    pub opcode: u8,
    pub operand: Operand,
}

impl Insn {
    pub fn name(&self) -> &'static str {
        opcode_name(self.opcode)
    }
}

pub mod op {
    pub const LDC: u8 = 0x12;
    pub const LDC_W: u8 = 0x13;
    pub const LDC2_W: u8 = 0x14;
    pub const IINC: u8 = 0x84;
    pub const GOTO: u8 = 0xa7;
    pub const JSR: u8 = 0xa8;
    pub const RET: u8 = 0xa9;
    pub const TABLESWITCH: u8 = 0xaa;
    pub const LOOKUPSWITCH: u8 = 0xab;
    pub const IRETURN: u8 = 0xac;
    pub const RETURN: u8 = 0xb1;
    pub const GETSTATIC: u8 = 0xb2;
    pub const PUTSTATIC: u8 = 0xb3;
    pub const GETFIELD: u8 = 0xb4;
    pub const PUTFIELD: u8 = 0xb5;
    pub const INVOKEVIRTUAL: u8 = 0xb6;
    pub const INVOKESPECIAL: u8 = 0xb7;
    pub const INVOKESTATIC: u8 = 0xb8;
    pub const INVOKEINTERFACE: u8 = 0xb9;
    pub const INVOKEDYNAMIC: u8 = 0xba;
    pub const NEW: u8 = 0xbb;
    pub const NEWARRAY: u8 = 0xbc;
    pub const ANEWARRAY: u8 = 0xbd;
    pub const ATHROW: u8 = 0xbf;
    pub const CHECKCAST: u8 = 0xc0;
    pub const INSTANCEOF: u8 = 0xc1;
    pub const WIDE: u8 = 0xc4;
    pub const MULTIANEWARRAY: u8 = 0xc5;
    pub const IFNULL: u8 = 0xc6;
    pub const IFNONNULL: u8 = 0xc7;
    pub const GOTO_W: u8 = 0xc8;
    pub const JSR_W: u8 = 0xc9;
}

const NAMES: [&str; 202] = [
    "nop", "aconst_null", "iconst_m1", "iconst_0", "iconst_1", "iconst_2", "iconst_3", "iconst_4",
    "iconst_5", "lconst_0", "lconst_1", "fconst_0", "fconst_1", "fconst_2", "dconst_0", "dconst_1",
    "bipush", "sipush", "ldc", "ldc_w", "ldc2_w", "iload", "lload", "fload", "dload", "aload",
    "iload_0", "iload_1", "iload_2", "iload_3", "lload_0", "lload_1", "lload_2", "lload_3",
    "fload_0", "fload_1", "fload_2", "fload_3", "dload_0", "dload_1", "dload_2", "dload_3",
    "aload_0", "aload_1", "aload_2", "aload_3", "iaload", "laload", "faload", "daload", "aaload",
    "baload", "caload", "saload", "istore", "lstore", "fstore", "dstore", "astore", "istore_0",
    "istore_1", "istore_2", "istore_3", "lstore_0", "lstore_1", "lstore_2", "lstore_3", "fstore_0",
    "fstore_1", "fstore_2", "fstore_3", "dstore_0", "dstore_1", "dstore_2", "dstore_3", "astore_0",
    "astore_1", "astore_2", "astore_3", "iastore", "lastore", "fastore", "dastore", "aastore",
    "bastore", "castore", "sastore", "pop", "pop2", "dup", "dup_x1", "dup_x2", "dup2", "dup2_x1",
    "dup2_x2", "swap", "iadd", "ladd", "fadd", "dadd", "isub", "lsub", "fsub", "dsub", "imul",
    "lmul", "fmul", "dmul", "idiv", "ldiv", "fdiv", "ddiv", "irem", "lrem", "frem", "drem", "ineg",
    "lneg", "fneg", "dneg", "ishl", "lshl", "ishr", "lshr", "iushr", "lushr", "iand", "land", "ior",
    "lor", "ixor", "lxor", "iinc", "i2l", "i2f", "i2d", "l2i", "l2f", "l2d", "f2i", "f2l", "f2d",
    "d2i", "d2l", "d2f", "i2b", "i2c", "i2s", "lcmp", "fcmpl", "fcmpg", "dcmpl", "dcmpg", "ifeq",
    "ifne", "iflt", "ifge", "ifgt", "ifle", "if_icmpeq", "if_icmpne", "if_icmplt", "if_icmpge",
    "if_icmpgt", "if_icmple", "if_acmpeq", "if_acmpne", "goto", "jsr", "ret", "tableswitch",
    "lookupswitch", "ireturn", "lreturn", "freturn", "dreturn", "areturn", "return", "getstatic",
    "putstatic", "getfield", "putfield", "invokevirtual", "invokespecial", "invokestatic",
    "invokeinterface", "invokedynamic", "new", "newarray", "anewarray", "arraylength", "athrow",
    "checkcast", "instanceof", "monitorenter", "monitorexit", "wide", "multianewarray", "ifnull",
    "ifnonnull", "goto_w", "jsr_w",
];

pub fn opcode_name(op: u8) -> &'static str {
    NAMES.get(op as usize).copied().unwrap_or("unknown")
}

/// 条件分支（ifXX / if_icmpXX / if_acmpXX / ifnull / ifnonnull）
pub fn is_cond_branch(op: u8) -> bool {
    (0x99..=0xa6).contains(&op) || op == op::IFNULL || op == op::IFNONNULL
}

/// 控制流不落入下一条指令
pub fn is_terminal(op: u8) -> bool {
    matches!(op, op::GOTO | op::GOTO_W | op::RET | op::TABLESWITCH | op::LOOKUPSWITCH | op::ATHROW)
        || (op::IRETURN..=op::RETURN).contains(&op)
}

fn target(pc: u32, off: i32) -> u32 {
    (pc as i64 + off as i64) as u32
}

pub fn decode(code: &[u8], pool: &ConstantPool) -> Result<Vec<Insn>, Error> {
    let mut r = Reader::new(code);
    let mut out = Vec::new();
    while r.remaining() > 0 {
        let pc = r.pos() as u32;
        let opc = r.u1()?;
        let operand = match opc {
            0x10 => Operand::Int(r.i1()? as i32),
            0x11 => Operand::Int(r.i2()? as i32),
            op::LDC => Operand::Ldc(pool.loadable(r.u1()? as u16)?),
            op::LDC_W | op::LDC2_W => Operand::Ldc(pool.loadable(r.u2()?)?),
            0x15..=0x19 | 0x36..=0x3a | op::RET => Operand::Local(r.u1()? as u16),
            op::IINC => {
                let index = r.u1()? as u16;
                Operand::Iinc { index, delta: r.i1()? as i16 }
            }
            0x99..=0xa8 | op::IFNULL | op::IFNONNULL => Operand::Branch(target(pc, r.i2()? as i32)),
            op::GOTO_W | op::JSR_W => Operand::Branch(target(pc, r.i4()?)),
            op::TABLESWITCH => {
                let pad = (4 - (r.pos() % 4)) % 4;
                r.skip(pad)?;
                let default = target(pc, r.i4()?);
                let low = r.i4()?;
                let high = r.i4()?;
                if high < low {
                    return Err(Error::BadCode(pc));
                }
                let mut targets = Vec::with_capacity((high - low + 1) as usize);
                for _ in low..=high {
                    targets.push(target(pc, r.i4()?));
                }
                Operand::TableSwitch { default, low, high, targets }
            }
            op::LOOKUPSWITCH => {
                let pad = (4 - (r.pos() % 4)) % 4;
                r.skip(pad)?;
                let default = target(pc, r.i4()?);
                let n = r.i4()?;
                if n < 0 {
                    return Err(Error::BadCode(pc));
                }
                let mut pairs = Vec::with_capacity(n as usize);
                for _ in 0..n {
                    let k = r.i4()?;
                    pairs.push((k, target(pc, r.i4()?)));
                }
                Operand::LookupSwitch { default, pairs }
            }
            op::GETSTATIC..=op::PUTFIELD => Operand::Field(pool.member_ref(r.u2()?)?.0),
            op::INVOKEVIRTUAL..=op::INVOKESTATIC => {
                let (m, iface) = pool.member_ref(r.u2()?)?;
                Operand::Method(m, iface)
            }
            op::INVOKEINTERFACE => {
                let (m, _) = pool.member_ref(r.u2()?)?;
                r.skip(2)?;
                Operand::Method(m, true)
            }
            op::INVOKEDYNAMIC => {
                let idx = r.u2()?;
                r.skip(2)?;
                match pool.get(idx)? {
                    crate::constant::CpEntry::InvokeDynamic(bsm, nt) => {
                        let (n, d) = pool.name_and_type(*nt)?;
                        Operand::InvokeDynamic { index: idx, bsm: *bsm, name: n.to_string(), desc: d.to_string() }
                    }
                    _ => return Err(Error::BadIndex(idx)),
                }
            }
            op::NEW | op::ANEWARRAY | op::CHECKCAST | op::INSTANCEOF => {
                Operand::Class(pool.class_name(r.u2()?)?.to_string())
            }
            op::NEWARRAY => Operand::NewArray(r.u1()?),
            op::MULTIANEWARRAY => {
                let c = pool.class_name(r.u2()?)?.to_string();
                Operand::MultiANewArray(c, r.u1()?)
            }
            op::WIDE => {
                // wide 只扩宽操作数：发出被修饰指令本身（语义不变）
                let inner = r.u1()?;
                let index = r.u2()?;
                let operand = if inner == op::IINC {
                    Operand::Iinc { index, delta: r.i2()? }
                } else {
                    Operand::Local(index)
                };
                out.push(Insn { offset: pc, opcode: inner, operand });
                continue;
            }
            0xca..=0xff => return Err(Error::BadOpcode(opc, pc)),
            _ => Operand::None,
        };
        out.push(Insn { offset: pc, opcode: opc, operand });
    }
    Ok(out)
}
