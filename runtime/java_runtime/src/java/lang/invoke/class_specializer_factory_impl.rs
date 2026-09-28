//! `java/lang/invoke/ClassSpecializer$Factory` 的运行期类定义点（N11，
//! `docs/plans/2026-09-27-bmh-dynamic-species.md`）。
//!
//! JDK 在此以 ASM 生成物种类字节码并 `Lookup.defineClass`。原生二进制无运行期类定义：
//! 全部未预生成的物种 key 共用 VM 支持类 `BoundMethodHandle$Species_Dyn`（字节码翻译），
//! 此处登记 key → SpeciesData 并返回该类；随后 ClassSpecializer 按 key 形态查找的
//! `make` / `arg<T><i>` 由 `crate::species_dyn` 应答。vm_intrinsics.toml「运行期类定义点」登记。

use crate::prelude::*;
use super::class_specializer_factory::ClassSpecializer_Factory;
use super::ClassSpecializer_SpeciesData;
use crate::java::lang::Class;

impl<T, K, S> ClassSpecializer_Factory<T, K, S>
where
    T: Clone + Default + 'static + From<Object> + Into<Object> + crate::sync_model::__ThreadSafe,
    K: Clone + Default + 'static + From<Object> + Into<Object> + crate::sync_model::__ThreadSafe,
    S: Clone + Default + 'static + From<Object> + Into<Object> + crate::sync_model::__ThreadSafe,
{
    /// `generateConcreteSpeciesCode(String className, SpeciesData)`：登记动态物种，返回通用载体类。
    #[jvm_boundary]
    pub fn __impl_generateConcreteSpeciesCode(
        &self,
        _className: String,
        speciesData: ClassSpecializer_SpeciesData<T, K, S>,
    ) -> Result<Class> {
        let key: Object = speciesData.key()?.into();
        crate::species_dyn::register(format!("{}", key), Object::from(speciesData));
        Ok(Class::for_class(String::from(crate::species_dyn::SPECIES_DYN)))
    }
}
