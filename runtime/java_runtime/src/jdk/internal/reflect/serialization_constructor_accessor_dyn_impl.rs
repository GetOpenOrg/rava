//! VM 支持类 `jdk/internal/reflect/SerializationConstructorAccessorDyn` 的 native（FS-R R3）。

use crate::prelude::*;
use super::serialization_constructor_accessor_dyn::SerializationConstructorAccessorDyn;
use crate::java::lang::Class;

impl SerializationConstructorAccessorDyn {
    /// native `allocateAndInit(Class target, Class initCl)`：分配 target 实例（不运行其构造器），
    /// 在其上运行 initCl 的无参构造体——L3 分派闭包的 `<alloc>` / `<init_on>` 伪成员（N2），
    /// 即 JDK 生成访问器的 `new target` + `invokespecial initCl.<init>()V` 两步。
    #[jvm_native]
    pub fn allocateAndInit(target: Class, init_cl: Class) -> Result<Object> {
        let t = format!("{}", target.__get_name()).replace('.', "/");
        let i = format!("{}", init_cl.__get_name()).replace('.', "/");
        let empty: JArray<Object> = JArray::from(Vec::<Object>::new());
        let obj = crate::reflect_dispatch::reflect_invoke(&t, "<alloc>", "()V", Object::default(), &empty)?;
        crate::reflect_dispatch::reflect_invoke(&i, "<init_on>", "()V", Clone::clone(&obj), &empty)?;
        Ok(obj)
    }
}
