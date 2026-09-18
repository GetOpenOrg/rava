use crate::prelude::*;
use super::*;
use std::cell::RefCell;
use std::collections::HashMap;

impl Class {
    /// native registerNatives：HotSpot 绑定 JNI 入口；原生二进制无此需要。
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        Ok(())
    }

    /// native getPrimitiveClass(String)：每个基本类型名对应唯一的 Class 对象
    /// （`Integer.TYPE == int.class` 的身份语义），首次请求时创建。
    #[jvm_native]
    pub fn getPrimitiveClass(name: String) -> Result<Class> {
        thread_local! {
            static PRIMITIVES: RefCell<HashMap<std::string::String, Class>> = RefCell::new(HashMap::new());
        }
        let key = format!("{}", name);
        Ok(PRIMITIVES.with(|cache| {
            Clone::clone(cache.borrow_mut().entry(key).or_insert_with(|| {
                let mut c = Class::default();
                c._init_not_null();
                c.__set_name(Clone::clone(&name));
                c
            }))
        }))
    }
}
