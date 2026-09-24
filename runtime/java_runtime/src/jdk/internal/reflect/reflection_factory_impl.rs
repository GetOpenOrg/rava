//! `jdk/internal/reflect/ReflectionFactory` 手写伴生：JUnit Runner 路径的
//! `AccessibleObject` 类初始化触达（`setAccessible` → clinit 的
//! `doPrivileged(GetReflectionFactoryAction)` 链）。
//!
//! JDK 21 语义面：`<init>()V` 为私有空体；`soleInstance` 由 clinit 内联初始化的
//! 单例；`getReflectionFactory()` 即返回该单例。本伴生按需实现这三点——
//! `newMethodAccessor` / `newFieldAccessor` 等 accessor 工厂族不在翻译侧
//! 反射协议（L3 `__reflect_dispatch`）的路径上，保持 panic 存根。

use crate::prelude::*;
use super::reflection_factory::ReflectionFactory;
use crate::java::lang::Class;

impl ReflectionFactory {
    /// `<init>()V`：私有空构造（JDK 21 `private ReflectionFactory() {}`）。
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Ok(this)
    }

    /// `newConstructorForSerialization(Class)`：序列化构造器——沿超类链找首个
    /// 不可序列化超类 initCl，取其无参构造（private / 跨包包可见 → null，
    /// 无无参构造 → null，与 JDK 21 同判定序）。
    ///
    /// JDK 的 generateConstructor 返回 constructorToCall 的副本（声明类仍为
    /// initCl）并挂「分配 cl 实例 + 运行 initCl 构造体」的序列化访问器。此处
    /// 返回同一元数据副本；序列化访问器语义（反序列化实例化）未建模——调用它
    /// 的 ObjectInputStream.readObject 链不在翻译面上（M3 路径只到
    /// ObjectStreamClass 元数据：Result.<clinit> 的 lookup(...).getFields()）。
    /// JDK 的 superHasAccessibleConstructor 逐级检查简并为对 initCl 构造的
    /// 可见性检查（非 null 返回的判定面一致：链上中间类均可序列化）。
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
        let ctor = match init_cl.getDeclaredConstructor(JArray::from(Vec::<Class>::new())) {
            Ok(c) => c,
            // NoSuchMethodException → null（JDK catch 同型）；其余异常传播
            Err(e) if e.is_instance_of("java/lang/NoSuchMethodException") => {
                return Ok(Default::default());
            }
            Err(e) => return Err(e),
        };
        let mods = ctor.__get_modifiers();
        let package_private = (mods & (0x0001 | 0x0004)) == 0;
        if (mods & 0x0002) != 0
            || (package_private
                && format!("{}", cl.getPackageName()?) != format!("{}", init_cl.getPackageName()?))
        {
            return Ok(Default::default());
        }
        Ok(ctor)
    }

    /// static `getReflectionFactory()`：返回 clinit 单例 `soleInstance`。
    /// 生成侧 clinit 不在本闭包调用链上（静态字段初始化未发射），单例在此
    /// 惰性落置——`thread_local` 持有底层 `Object`（运行时单线程模型，
    /// Object 非 Sync），跨调用同一对象身份（JVM `soleInstance` 的可观测语义）。
    pub fn getReflectionFactory() -> Result<ReflectionFactory> {
        thread_local! {
            static SOLE_INSTANCE: RefCell<Option<Object>> = const { RefCell::new(None) };
        }
        let obj = SOLE_INSTANCE.with(|slot| {
            let mut slot = slot.borrow_mut();
            slot.get_or_insert_with(|| {
                // 构造为空体（不可失败）；Err 分支退回 null 单例仅为闭合类型
                match ReflectionFactory::new() {
                    Ok(rf) => Object::from(rf),
                    Err(_) => Object::default(),
                }
            })
            .clone()
        });
        Ok(ReflectionFactory::from(obj))
    }
}
