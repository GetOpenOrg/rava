/// java/lang/Throwable.fillInStackTrace:(I)Ljava/lang/Throwable;
/// 单线程转译环境无栈帧信息；返回 self 引用（no-op 实现）
pub fn fillInStackTrace(_this: &Throwable, _depth: i32) -> Result<Throwable> {
    Ok(Throwable::default())
}
