/// java/io/PrintStream.println:(Ljava/lang/String;)V
pub fn println__str(_this: &PrintStream, x: String) -> Result<()> {
    println!("{}", x);
    Ok(())
}

/// java/io/PrintStream.println:(I)V
pub fn println__i(_this: &PrintStream, v: i32) -> Result<()> {
    println!("{}", v);
    Ok(())
}

/// java/io/PrintStream.println:()V
pub fn println(_this: &PrintStream) -> Result<()> {
    println!();
    Ok(())
}

/// java/io/PrintStream.flush:()V
pub fn flush(_this: &PrintStream) -> Result<()> {
    use std::io::Write;
    let _ = std::io::stdout().flush();
    Ok(())
}

/// java/io/PrintStream.print:(Ljava/lang/String;)V
pub fn print__str(_this: &PrintStream, x: String) -> Result<()> {
    print!("{}", x);
    Ok(())
}
