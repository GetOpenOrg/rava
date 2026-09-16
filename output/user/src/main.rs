#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types)]
mod test_lambda;
mod test_lambda_transformer;
use test_lambda::TestLambda;

fn main() {
    TestLambda::main().unwrap_or_else(|e| eprintln!("JVM Error: {:?}", e));
}
