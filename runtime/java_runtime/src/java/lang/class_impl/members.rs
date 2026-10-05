//! Class 的成员查询：元数据表构造 Field / Method / Constructor（宿主 class_impl.rs 的私有辅助模块）

use super::*;

impl Class {
    /// `getDeclaredFields()`：本类全部声明字段的构造序列（字段表驱动，
    /// getDeclaredField 的复数形态——同一张 java_meta 字段表循环输出）。
    pub(crate) fn __table_declared_fields(&self) -> Result<JArray<Field>> {
        Field::__class_init()?;
        let cls_key = format!("{}", self.__get_name()).replace('.', "/");
        let mut out: Vec<Field> = Vec::new();
        if let Some((_, fs)) = crate::meta::class_fields().iter().find(|(n, _)| *n == cls_key) {
            for (slot, meta) in fs.iter().enumerate() {
                let mut f = Field::default();
                f._init_not_null();
                f.__set_clazz(Clone::clone(self));
                f.__set_name(String::from(meta.name));
                f.__set_modifiers(meta.modifiers);
                f.__set_slot(slot as i32);
                f.__set_type_(class_for_descriptor(meta.descriptor));
                f.__set_annotations(__anno_bytes(meta.annotations));
                f.__set_signature(__signature(meta.signature));
                out.push(f);
            }
        }
        Ok(JArray::from(out))
    }


    /// `getDeclaredConstructors()`：本类全部声明构造器（方法表 `<init>` 行；
    /// 构造器身份键 = (类, 描述符)——参数还原同 __method_from_meta）。
    pub(crate) fn __table_declared_ctors(&self) -> Result<JArray<crate::java::lang::reflect::Constructor<Object>>> {
        crate::java::lang::reflect::Constructor::<Object>::__class_init()?;
        let cls_key = format!("{}", self.__get_name()).replace('.', "/");
        let mut out: Vec<crate::java::lang::reflect::Constructor<Object>> = Vec::new();
        if let Some((_, ms)) = crate::meta::class_methods().iter().find(|(n, _)| *n == cls_key) {
            for (slot, meta) in ms.iter().enumerate() {
                if meta.name != "<init>" {
                    continue;
                }
                let mut c = crate::java::lang::reflect::Constructor::<Object>::default();
                c._init_not_null();
                c.__set_clazz(Clone::clone(self));
                c.__set_modifiers(meta.modifiers);
                c.__set_slot(slot as i32);
                let params: Vec<Class> = descriptor_params(meta.descriptor).into_iter()
                    .map(|p| class_for_descriptor(&p)).collect();
                c.__set_parameterTypes(JArray::from(params));
                let excs: Vec<Class> = meta.exceptions.iter()
                    .map(|e| Class::for_class(String::from(*e))).collect();
                c.__set_exceptionTypes(JArray::from(excs));
                c.__set_annotations(__anno_bytes(meta.annotations));
                c.__set_parameterAnnotations(__anno_bytes(meta.param_annotations));
                c.__set_signature(__signature(meta.signature));
                out.push(c);
            }
        }
        Ok(JArray::from(out))
    }

}

// ── FS-R R2：成员查询 native（元数据表构造 Field / Method / Constructor，JDK 查询族回到字节码）──
impl Class {
    /// native `getDeclaredFields0(boolean publicOnly)`：本类声明字段（声明序 = slot）。
    /// trustedFinal 与 HotSpot 同判定：static final，或 record 类的 final 实例字段。
    #[jvm_native]
    pub fn getDeclaredFields0(&self, public_only: bool) -> Result<JArray<Field>> {
        let all = self.__table_declared_fields()?;
        let is_record = self.isRecord0()?;
        let mut out: Vec<Field> = Vec::new();
        for i in 0..all.len()? {
            let mut f = all.get(i)?;
            let mods = f.__get_modifiers();
            if public_only && mods & 0x0001 == 0 {
                continue;
            }
            let fin = mods & 0x0010 != 0;
            f.__set_trustedFinal(fin && (mods & 0x0008 != 0 || is_record));
            out.push(f);
        }
        Ok(JArray::from(out))
    }

    /// native `getDeclaredMethods0(boolean publicOnly)`：本类声明方法（不含 `<init>` / `<clinit>`）。
    #[jvm_native]
    pub fn getDeclaredMethods0(&self, public_only: bool) -> Result<JArray<crate::java::lang::reflect::Method>> {
        let all = self.__table_declared_methods()?;
        let mut out = Vec::new();
        for i in 0..all.len()? {
            let m = all.get(i)?;
            if !public_only || m.__get_modifiers() & 0x0001 != 0 {
                out.push(m);
            }
        }
        Ok(JArray::from(out))
    }

    /// native `getDeclaredConstructors0(boolean publicOnly)`：本类声明构造器。
    #[jvm_native]
    pub fn getDeclaredConstructors0(&self, public_only: bool)
        -> Result<JArray<crate::java::lang::reflect::Constructor<Object>>>
    {
        let all = self.__table_declared_ctors()?;
        let mut out = Vec::new();
        for i in 0..all.len()? {
            let c = all.get(i)?;
            if !public_only || c.__get_modifiers() & 0x0001 != 0 {
                out.push(c);
            }
        }
        Ok(JArray::from(out))
    }
}

impl Class {
    /// `enumConstantDirectory()`（包私有；`Enum.valueOf` 的查表面，FS-H8）：JDK 体经
    /// `getEnumConstantsShared()`（反射调用 `values()`）建「常量名 → 常量」映射并缓存于
    /// `enumConstantDirectory` 字段。原生侧枚举宇宙取运行时常量目录（`java_class!` 宏在类初始化
    /// 后登记，与 `JavaLangAccess.getEnumConstantsShared` 同源），其余逐句同 JDK：先查字段缓存；
    /// 非枚举类（修饰符无 ACC_ENUM）抛 `IllegalArgumentException(getName() + " is not an enum class")`。
    #[jvm_boundary]
    pub fn __impl_enumConstantDirectory(&self) -> Result<crate::java::util::Map<Object, Object>> {
        let cached = self.__get_enumConstantDirectory();
        if !cached.is_jvm_null() {
            return Ok(cached);
        }
        let cls_name = format!("{}", self.__get_name());
        // JVM 反射路径语义：读常量宇宙前强制目标类初始化（常量目录在 `<clinit>` 之后登记）
        crate::ensure_class_initialized(&cls_name)?;
        let entries = if self.getModifiers()? & 0x4000 != 0 {
            crate::constant_directory_entries(&cls_name)
        } else {
            None
        };
        let Some(entries) = entries else {
            let ex = crate::java::lang::IllegalArgumentException::new_str(
                String::from(format!("{} is not an enum class", cls_name).as_str()))?;
            return Err(ex.into());
        };
        let map = crate::java::util::HashMap::<Object, Object>::new()?;
        for (name, value) in entries {
            let _ = map.put(Object::from(String::from(name.as_str())), value)?;
        }
        let dir = <crate::java::util::Map<Object, Object> as ::std::convert::From<Object>>::from(Object::from(map));
        self.__set_enumConstantDirectory(Clone::clone(&dir));
        Ok(dir)
    }

    /// 本类按 (名字, 描述符) 声明的方法（VM 直取反射对象：动态代理的接口方法对象，
    /// HotSpot 同样经方法元数据构造）。未声明 → null。
    pub(crate) fn __table_method(&self, name: &str, descriptor: &str) -> Result<crate::java::lang::reflect::Method> {
        crate::java::lang::reflect::Method::__class_init()?;
        for (slot, meta) in self.__declared_method_rows().iter().enumerate() {
            if meta.name == name && meta.descriptor == descriptor && !meta.inherited {
                return Ok(self.__method_from_meta(meta, slot as i32));
            }
        }
        Ok(crate::java::lang::reflect::Method::default())
    }

    /// 本类声明的无参方法（record 组件访问器）：元数据表直构，不经公开查询族
    /// （getRecordComponents0 是 native，JDK 侧同样由 VM 直接取方法对象）。
    pub(crate) fn __table_method_noargs(&self, name: &str) -> Result<crate::java::lang::reflect::Method> {
        let all = self.__table_declared_methods()?;
        for i in 0..all.len()? {
            let m = all.get(i)?;
            if format!("{}", m.__get_name()) == name && m.__get_parameterTypes().len()? == 0 {
                return Ok(m);
            }
        }
        Ok(crate::java::lang::reflect::Method::default())
    }
}
