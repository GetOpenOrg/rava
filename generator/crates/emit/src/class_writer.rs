//! 单类文件文本（← `emitter/class_writer._gen_class_rs`）。
//!
//! 步骤 (a)：仅骨架占位（带生成标记，mod 树据此识别生成文件）；类块头 / 字段 / 导入
//! 在步骤 (b)，方法块在步骤 (c)，vtable / 继承段在步骤 (d) 填充。

use ty::ClassInfo;

use crate::body::MethodBodyEmitter;
use crate::ctx::{EmitCtx, ProjectState};
use crate::error::Result;

/// 类所在 crate 的发射参数
pub enum ClassSite<'l> {
    /// java_runtime 内的 JDK 类
    Jdk(&'l crate::project::layout::JdkLayout),
    /// user crate 类：同 crate 兄弟类导入行
    User { sibling_imports: Vec<String> },
}

/// 生成单类文件文本
pub fn gen_class_rs(
    ctx: &EmitCtx<'_>,
    _state: &mut ProjectState,
    _bodies: &mut dyn MethodBodyEmitter,
    ci: &ClassInfo,
    _site: &ClassSite<'_>,
) -> Result<String> {
    Ok(format!("// {}：rava_macros::java_class! 骨架占位（步骤 (b) 起填充）\n", ctx.short(ci.name())))
}
