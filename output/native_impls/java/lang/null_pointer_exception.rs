/// java/lang/NullPointerException.getExtendedNPEMessage:()Ljava/lang/String;
/// JVM 扩展 NPE 消息需要 HotSpot 内部支持；转译环境返回空字符串
pub fn getExtendedNPEMessage(_this: &NullPointerException) -> Result<String> {
    Ok(String::new())
}
