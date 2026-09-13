/// java/lang/System.currentTimeMillis:()J
pub fn currentTimeMillis() -> Result<i64> {
    use std::time::{SystemTime, UNIX_EPOCH};
    Ok(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as i64)
}

/// java/lang/System.nanoTime:()J
pub fn nanoTime() -> Result<i64> {
    use std::time::{SystemTime, UNIX_EPOCH};
    Ok(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos() as i64)
}

/// java/lang/System.arraycopy:(Ljava/lang/Object;ILjava/lang/Object;II)V
/// not-needed — HelloWorld 不调用此方法

/// java/lang/System.registerNatives:()V
/// not-needed

/// java/lang/System.setIn0:(Ljava/io/InputStream;)V
/// not-needed

/// java/lang/System.setOut0:(Ljava/io/PrintStream;)V
/// not-needed

/// java/lang/System.setErr0:(Ljava/io/PrintStream;)V
/// not-needed

/// java/lang/System.out:Ljava/io/PrintStream;
pub fn out() -> PrintStream {
    PrintStream::default()
}

/// java/lang/System.err:Ljava/io/PrintStream;
pub fn err() -> PrintStream {
    PrintStream::default()
}
