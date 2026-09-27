//! `jdk/internal/reflect/ReflectionFactory` 手写伴生（FS-R 过渡）：只余序列化构造器。
//!
//! 包 `jdk/internal/reflect/` 已按字节码翻译（boundary_release，FS-R R2a）：构造器
//! （`langReflectAccess = SharedSecrets.getJavaLangReflectAccess()`）、`soleInstance` 单例、
//! 访问器工厂族均走 JDK 字节码。`newConstructorForSerialization` 的 JDK 实现经
//! `generateConstructor` 运行期生成字节码，R3 改为 VM 内建后删除本伴生
//!（docs/plans/2026-09-27-reflection-metadata-table.md §2.4）。

use crate::prelude::*;
use super::reflection_factory::ReflectionFactory;
use crate::java::lang::Class;

impl ReflectionFactory {

    /// `newConstructorForSerialization(Class)`：序列化构造器——沿超类链找首个
    /// 不可序列化超类 initCl，取其无参构造（private / 跨包包可见 → null，
    /// 无无参构造 → null，与 JDK 21 同判定序）。
    ///
    /// JDK 的 generateConstructor 返回 constructorToCall 的副本（声明类仍为
    /// initCl）并挂「分配 cl 实例 + 运行 initCl 构造体」的序列化访问器。此处
    /// 返回元数据副本并登记目标类（reflect_dispatch 序列化构造器表）——
    /// Constructor.newInstance 据此经分派闭包 `<alloc>` 分配 cl 实例、`<init_on>`
    /// 运行 initCl 构造体（N2）。
    /// JDK 的 superHasAccessibleConstructor 逐级检查简并为对 initCl 构造的
    /// 可见性检查（非 null 返回的判定面一致：链上中间类均可序列化）。
    #[jvm_boundary(upcalls = "java/lang/Class.getPackageName:()Ljava/lang/String;")]
    pub fn newConstructorForSerialization_class(&self, cl: Class)
        -> Result<crate::java::lang::reflect::Constructor<Object>>
    {
        let serializable = Class::for_class(String::from("java/io/Serializable"));
        let mut init_cl = Clone::clone(&cl);
        while serializable.isAssignableFrom(Clone::clone(&init_cl))? {
            let sup = init_cl.getSuperclass()?;
            if Object::from(Clone::clone(&sup)).0.is_jvm_null() {
                return Ok(Default::default());
            }
            init_cl = sup;
        }
        // initCl 的无参构造器（元数据表直查；缺席 → null，与 JDK 捕获 NoSuchMethodException 同型）
        let ctors = init_cl.__table_declared_ctors()?;
        let mut found = None;
        for i in 0..ctors.len()? {
            let c = ctors.get(i)?;
            if c.__get_parameterTypes().len()? == 0 {
                found = Some(c);
                break;
            }
        }
        let Some(ctor) = found else {
            return Ok(Default::default());
        };
        let mods = ctor.__get_modifiers();
        let package_private = (mods & (0x0001 | 0x0004)) == 0;
        if (mods & 0x0002) != 0
            || (package_private
                && format!("{}", cl.getPackageName()?) != format!("{}", init_cl.getPackageName()?))
        {
            return Ok(Default::default());
        }
        // 序列化访问器语义：newInstance 分配 cl 实例并运行 initCl 的无参构造体（登记目标类）
        let id = Object::from(Clone::clone(&ctor)).0.__identity() as usize;
        crate::reflect_dispatch::register_serialization_ctor(
            id, &format!("{}", cl.__get_name()).replace('.', "/"));
        Ok(ctor)
    }

}
