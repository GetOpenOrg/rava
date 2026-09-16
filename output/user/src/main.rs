#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types)]
mod test_array_list;
mod test_collections;
use test_array_list::TestArrayList;

fn main() {
    TestArrayList::main().unwrap_or_else(|e| eprintln!("JVM Error: {:?}", e));
}
