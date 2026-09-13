#![allow(unused_imports)]
pub mod error;
pub mod types;
pub mod java;

pub use error::{JvmError, Result};
pub use types::Field;
pub use java::lang::Object;
pub use java::util::{ArrayList, HashMap, HashSet};

/// prelude：生成代码用 `use java_runtime::prelude::*;` 引入所有必要符号。
pub mod prelude {
    #![allow(unused_imports)]
    pub use super::error::{JvmError, Result};
    pub use super::types::Field;
    pub use super::java::lang::Object;
    pub use super::java::util::{ArrayList, HashMap, HashSet};
}
