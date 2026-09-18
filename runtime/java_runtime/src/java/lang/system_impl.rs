use crate::prelude::*;
use super::*;
use crate::java::io::print_stream::PrintStream;

impl System {
    #[jvm_native]
    pub fn arraycopy(src: Object, src_pos: i32, dest: Object, dest_pos: i32, length: i32) -> Result<()> {
        macro_rules! try_copy {
            ($t:ty) => {
                if let Some(s) = src.0.as_any().downcast_ref::<JArray<$t>>() {
                    let d = dest.downcast::<JArray<$t>>();
                    for i in 0..length {
                        d.set(dest_pos + i, s.get(src_pos + i));
                    }
                    return Ok(());
                }
            };
        }
        try_copy!(i8);
        try_copy!(i32);
        try_copy!(i64);
        try_copy!(u16);
        try_copy!(f32);
        try_copy!(f64);
        try_copy!(bool);
        try_copy!(i16);
        try_copy!(Object);
        panic!("stub: System.arraycopy: unsupported array element type")
    }

    #[jvm_native]
    pub fn currentTimeMillis() -> Result<i64> {
        use std::time::{SystemTime, UNIX_EPOCH};
        Ok(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as i64)
    }

    #[jvm_native]
    pub fn nanoTime() -> Result<i64> {
        use std::time::{SystemTime, UNIX_EPOCH};
        Ok(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos() as i64)
    }

    /// static 字段 `out` 的读取访问器。HotSpot 中该字段由 VM 引导阶段（initPhase1 →
    /// 本地方法 setOut0/setErr0）直接写入，不经 `<clinit>`，故由手写层提供；
    /// 签名与宏生成的 static 访问器一致（`Result<T>`）。
    #[jvm_native]
    pub fn out() -> Result<PrintStream> {
        let mut ps = PrintStream::default();
        ps._init_not_null();
        Ok(ps)
    }

    /// static 字段 `err` 的读取访问器。HotSpot 中该字段由 VM 引导阶段（initPhase1 →
    /// 本地方法 setOut0/setErr0）直接写入，不经 `<clinit>`，故由手写层提供；
    /// 签名与宏生成的 static 访问器一致（`Result<T>`）。
    #[jvm_native]
    pub fn err() -> Result<PrintStream> {
        let mut ps = PrintStream::default();
        ps._init_not_null();
        Ok(ps)
    }
}
