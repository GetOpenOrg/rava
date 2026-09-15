#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types)]
mod test_stream_collectors;
mod test_stream_collectors_person;
use test_stream_collectors::TestStreamCollectors;

fn main() {
    TestStreamCollectors::main().unwrap_or_else(|e| eprintln!("JVM Error: {:?}", e));
}
