#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
mod java_runtime;
mod test_p3;
use test_p3::TestP3;

fn main() {
    TestP3::main().unwrap_or_else(|e| eprintln!("JVM Error: {:?}", e));
}
