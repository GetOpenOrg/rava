use crate::prelude::*;
use super::print_stream::PrintStream;

impl PrintStream {
    /// println_v：T38 统一入口，接受任何可转为 Object 的值并打印一行。
    /// 调用方由 invoke.py 生成，实参类型可以是 String、i32 等。
    #[jvm_ext]
    pub fn println_v<T: Into<Object>>(&self, x: T) -> Result<()> {
        println!("{}", x.into());
        Ok(())
    }
}
