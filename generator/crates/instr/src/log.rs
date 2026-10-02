//! 指令翻译的对外副作用账（← Python 模块级全局：`equiv_audit.record`、
//! `cfg.STATS.record_instanceof_fold`、`inherited_calls.request`、
//! `LAMBDA_NAME_LEDGER.record_reference`、`sam_objects.record_site`）。
//!
//! 本 crate 无全局可变状态：副作用按发生顺序追加到调用方持有的 [`InstrLog`]，
//! 由方法体 / 类发射层（P4c / P5）在合适的时机汇总消费。

/// 等价性审计口径（`[equiv-audit]` 行的 ID）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Audit {
    IdentityHash,
    ClassInit,
    InternIdentity,
    ClassLiteral,
    BoxedNull,
    NegArray,
    NullArray,
    MonitorMt,
    RecordHash,
    FieldNpe,
}

impl Audit {
    /// 审计行输出序（← `equiv_audit.IDS`，compatibility.md §4 目录序）
    pub const REPORT_ORDER: [Audit; 10] = [
        Audit::IdentityHash,
        Audit::InternIdentity,
        Audit::NullArray,
        Audit::BoxedNull,
        Audit::ClassLiteral,
        Audit::RecordHash,
        Audit::NegArray,
        Audit::FieldNpe,
        Audit::ClassInit,
        Audit::MonitorMt,
    ];

    /// Python 侧的口径名
    pub fn as_str(self) -> &'static str {
        match self {
            Audit::IdentityHash => "identity-hash",
            Audit::ClassInit => "class-init",
            Audit::InternIdentity => "intern-identity",
            Audit::ClassLiteral => "class-literal",
            Audit::BoxedNull => "boxed-null",
            Audit::NegArray => "neg-array",
            Audit::NullArray => "null-array",
            Audit::MonitorMt => "monitor-mt",
            Audit::RecordHash => "record-hash",
            Audit::FieldNpe => "field-npe",
        }
    }
}

/// 一条副作用
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Effect {
    /// 等价性审计计数 +1
    Audit(Audit),
    /// instanceof 编译期折叠为 false（`STATS.record_instanceof_fold`）
    InstanceofFold,
    /// 继承成员登记：接收者类需发射该继承方法的转发成员（`inherited_calls.request`）
    Inherited { receiver: String, method: String, param_desc: String },
    /// lambda 实现方法的调用侧 Rust 名（`LAMBDA_NAME_LEDGER.record_reference`，与定义侧对账）
    LambdaRef { class: String, method: String, rust_name: String },
    /// SAM 合成对象站点（`sam_objects.record_site`：站点 samtype 须与预扫描一致）。
    /// `hidden` 为站点的 lambda 隐藏类名，`interfaces` 为其直接超接口（samtype + 标记接口 +
    /// 可序列化标志的 Serializable，LambdaMetafactory 同序）
    SamSite { iface: String, sam_desc: String, class: String, hidden: String, interfaces: Vec<String> },
}

/// 副作用账（按发生顺序）
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InstrLog {
    pub effects: Vec<Effect>,
}

impl InstrLog {
    pub fn push(&mut self, e: Effect) {
        self.effects.push(e);
    }

    pub fn audit(&mut self, a: Audit) {
        self.effects.push(Effect::Audit(a));
    }

    pub fn inherited(&mut self, receiver: &str, method: &str, param_desc: &str) {
        self.effects.push(Effect::Inherited {
            receiver: receiver.to_string(),
            method: method.to_string(),
            param_desc: param_desc.to_string(),
        });
    }
}
