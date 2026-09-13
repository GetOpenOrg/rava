// Ergonomic API for PrintStream (T38)
// println_v<T: Printable> 统一派发，避免与无参 println() 命名冲突

impl PrintStream {
    /// Java: System.out.println(x) — 统一 Printable 派发
    pub fn println_v<T: Printable>(&self, v: T) -> Result<()> {
        println!("{}", v.to_print_string());
        Ok(())
    }
}
