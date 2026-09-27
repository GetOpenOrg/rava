//! `jdk/internal/reflect/ConstantPool` 的 native 方法（FS-R R4b）。
//!
//! HotSpot 以 constantPoolOop 指向类的常量池。原生二进制不携带完整常量池：constantPoolOop
//! 是所属 Class（Class.getConstantPool 设置），条目来自该类注解属性体引用的稀疏常量池
//! （build.rs class_anno_table，原索引）。消费方：AnnotationParser（getUTF8At / getIntAt /
//! getLongAt / getFloatAt / getDoubleAt）。表外索引 → IllegalArgumentException（HotSpot 对
//! 越界 / 错型索引同）。

use crate::prelude::*;
use super::constant_pool::ConstantPool;
use crate::java::lang::Class;
use crate::anno_pool::CpVal;

fn entry(cp_oop: &Object, index: i32) -> Result<&'static CpVal> {
    let cls = <Class as From<Object>>::from(Clone::clone(cp_oop));
    let key = format!("{}", cls.__get_name()).replace('.', "/");
    crate::anno_pool::cp_entry(&key, index)
        .ok_or_else(|| JvmError::illegal_argument(&format!("Wrong type at constant pool index {}", index)))
}

fn wrong(index: i32) -> JvmError {
    JvmError::illegal_argument(&format!("Wrong type at constant pool index {}", index))
}

impl ConstantPool {
    #[jvm_native]
    pub fn getUTF8At0(&self, cp_oop: Object, index: i32) -> Result<String> {
        match entry(&cp_oop, index)? {
            CpVal::U(s) => Ok(String::from(*s)),
            _ => Err(wrong(index)),
        }
    }

    #[jvm_native]
    pub fn getIntAt0(&self, cp_oop: Object, index: i32) -> Result<i32> {
        match entry(&cp_oop, index)? {
            CpVal::I(v) => Ok(*v),
            _ => Err(wrong(index)),
        }
    }

    #[jvm_native]
    pub fn getLongAt0(&self, cp_oop: Object, index: i32) -> Result<i64> {
        match entry(&cp_oop, index)? {
            CpVal::J(v) => Ok(*v),
            _ => Err(wrong(index)),
        }
    }

    #[jvm_native]
    pub fn getFloatAt0(&self, cp_oop: Object, index: i32) -> Result<f32> {
        match entry(&cp_oop, index)? {
            CpVal::F(v) => Ok(*v),
            _ => Err(wrong(index)),
        }
    }

    #[jvm_native]
    pub fn getDoubleAt0(&self, cp_oop: Object, index: i32) -> Result<f64> {
        match entry(&cp_oop, index)? {
            CpVal::D(v) => Ok(*v),
            _ => Err(wrong(index)),
        }
    }
}
