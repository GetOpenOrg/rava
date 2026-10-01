//! `jdk/internal/reflect/MethodAccessorGenerator` 的运行期类定义点（FS-R R3，
//! `docs/plans/2026-09-27-reflection-metadata-table.md` §2.4）。
//!
//! JDK 在此以字节码汇编器生成序列化构造器访问器类。原生二进制无运行期类定义：全部
//! 序列化构造器共用 VM 支持类 `SerializationConstructorAccessorDyn`（字节码翻译），实例
//! 携带 (待实例化类, 首个不可序列化超类)。vm_intrinsics.toml「运行期类定义点」登记。

use crate::prelude::*;
use super::method_accessor_generator::MethodAccessorGenerator;
use super::{SerializationConstructorAccessorDyn, SerializationConstructorAccessorImpl};
use crate::java::lang::Class;

impl MethodAccessorGenerator {
    /// `generateSerializationConstructor(declaringClass, parameterTypes, modifiers,
    /// targetConstructorClass)`：分配 declaringClass 实例、运行 targetConstructorClass 的
    /// 无参构造体的访问器（ReflectionFactory.generateConstructor 其余步骤走字节码）。
    #[jvm_boundary]
    pub fn __impl_generateSerializationConstructor(
        &self,
        declaring_class: Class,
        _parameter_types: JArray<Class>,
        _modifiers: i32,
        target_constructor_class: Class,
    ) -> Result<SerializationConstructorAccessorImpl> {
        let acc = SerializationConstructorAccessorDyn::new(declaring_class, target_constructor_class)?;
        Ok(From::from(Object::from(acc)))
    }
}
