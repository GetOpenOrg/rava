//! `java.lang.ProcessEnvironment`（Unix）的 native 层。

use crate::prelude::*;
use super::*;

impl ProcessEnvironment {
    /// native `environ()`：进程环境的字节形态（`[name0, value0, name1, value1, ...]`，
    /// 与 JDK ProcessEnvironment_md.c 同布局；类体据此构建不可修改的 String 视图）。
    #[jvm_native]
    pub fn environ() -> Result<JArray<JArray<i8>>> {
        use std::os::unix::ffi::OsStrExt;
        let to_bytes = |s: &std::ffi::OsStr| -> JArray<i8> {
            JArray::from(s.as_bytes().iter().map(|b| *b as i8).collect::<Vec<i8>>())
        };
        let mut out: Vec<JArray<i8>> = Vec::new();
        for (k, v) in std::env::vars_os() {
            out.push(to_bytes(&k));
            out.push(to_bytes(&v));
        }
        Ok(JArray::from(out))
    }
}
