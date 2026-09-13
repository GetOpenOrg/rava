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

/// java/io/PrintStream.println:(Z)V
pub fn println__z(_this: &PrintStream, v: bool) -> Result<()> {
    println!("{}", v);
    Ok(())
}

/// java/io/PrintStream.println:(J)V
pub fn println__j(_this: &PrintStream, v: i64) -> Result<()> {
    println!("{}", v);
    Ok(())
}

/// java/io/PrintStream.println:(Ljava/lang/Object;)V
pub fn println__obj(_this: &PrintStream, x: Object) -> Result<()> {
    // 在 jdk_classes 上下文中可直接访问 String 类型，优先按 String 显示
    if let Some(s) = x.0.downcast_ref::<String>() {
        println!("{}", s);
    } else {
        println!("{}", x);  // 回退到 Object::Display（原始类型或 "Object"）
    }
    Ok(())
}

/// java/io/PrintStream.print:(Ljava/lang/String;)V
pub fn print__str(_this: &PrintStream, x: String) -> Result<()> {
    print!("{}", x);
    Ok(())
}

// String 实现 Printable（在 jdk_classes 上下文中定义，因为 String 类型在此）
impl Printable for String {
    fn to_print_string(&self) -> std::string::String {
        format!("{}", self)
    }
}

