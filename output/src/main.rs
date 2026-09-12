#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
mod java_runtime;
mod hello_world;
mod test_p0;
mod test_p1;
mod test_p2;
mod test_p3;
use hello_world::HelloWorld;

fn main() {
    HelloWorld::main().unwrap_or_else(|e| eprintln!("JVM Error: {:?}", e));
}
