#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
mod hello_world;
use hello_world::HelloWorld;

fn main() {
    HelloWorld::main().unwrap_or_else(|e| eprintln!("JVM Error: {:?}", e));
}
