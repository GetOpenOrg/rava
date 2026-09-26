use crate::prelude::*;
use super::Field;
use crate::java::lang::Class;
use crate::java::lang::Object;

/// `Field.get/set` 的最小真实化（反射族静态注册表路线，与
/// `Class.getDeclaredField`（class_impl.rs）共用 build.rs 字段元数据表）。
///
/// 路由按元数据分派：
///   - 实例 int/long 字段：经 ObjectVTable 的 `__unsafe_int_cell` /
///     `__unsafe_long_cell` 按字段名协议（宏为平铺 int/long 字段生成臂，与
///     Unsafe 族原子同一存储单元——写入对直接字段读取可见）；
///   - 带 ConstantValue 的静态整型常量：值直接来自 class 文件（build.rs 收录）。
///
/// 边界（如实报告，待按需扩展）：
///   - 引用字段读写：无「按名取/存引用字段」的 vtable 协议，需宏侧为平铺
///     引用字段生成 `__field_get/__field_set` 臂（codegen+macros 域，本批不碰）；
///   - 无 ConstantValue 的静态字段：静态字存储无按名协议；
///   - float/double 的 ConstantValue 位形未分辨（class 文件存原始位）；
///   - 包装类对象值的拆箱（`set(p, Integer.valueOf(x))` 显式形态）：运行时
///     无到翻译类 intValue 的无条件调用边，只接受站点装箱的基本值盒
///     （`f.set(p, 99)` 的 codegen 形态）。
/// 数值拆箱（运行时侧最小形态）：值盒的运行时类是 Integer/Long（翻译包装与
/// 基本值盒的 vtable 类名同为 `java/lang/Integer`|`java/lang/Long`）→ 按其
/// toString 解析。直接调用翻译类 intValue 需要无条件 BFS 种子（会把 Integer
/// 强加进所有含 Field 的闭包），此处以类名门 + 解析承载——数值语义精确
/// （Integer.toString 即十进制整数字面量），类型不匹配（Long→int、String 等）
/// → None → IllegalArgumentException，与 JDK 同型。
fn __unbox_int(v: &Object) -> Option<i32> {
    if v.0.is_jvm_null() || v.0.__class_name() != "java/lang/Integer" { return None; }
    v.0.__obj_str().parse::<i32>().ok()
}

fn __unbox_long(v: &Object) -> Option<i64> {
    if v.0.is_jvm_null() || v.0.__class_name() != "java/lang/Long" { return None; }
    v.0.__obj_str().parse::<i64>().ok()
}

impl Field {
    // ── 注解元数据查询族（反射 L3 段 1）────────────────────────────────────
    //
    // Field 是 final 类（调用侧接收者静态类型恒为 Field wrapper），查询族以
    // wrapper 固有方法承载（impl_methods 协议；继承成员登记命中固有名时不再
    // 生成转发，E0592 防撞）。挂载键 (clazz, 字段名)；数据面是 build.rs 注解
    // 表，实例面是注解工厂（method_impl.rs 同款）。

    /// 本字段挂载点的注解条目（空 = 无注解 / 非表构造形态）。
    fn __anno_entries(&self) -> &'static [crate::annotation_meta::__anno_table::AnnotationEntry] {
        let clazz = self.__get_clazz();
        if Object::from(Clone::clone(&clazz)).0.is_jvm_null() {
            return &[];
        }
        let cls_key = format!("{}", clazz.__get_name()).replace('.', "/");
        let name = format!("{}", self.__get_name());
        crate::annotation_meta::field_annotation_entries(&cls_key, &name)
    }

    /// `getAnnotation(Class)`：命中 → 注解代理实例（未命中 → null）。
    pub fn getAnnotation(&self, annotationClass: Class) -> Result<Object> {
        let anno = format!("{}", annotationClass.__get_name()).replace('.', "/");
        let Some(hit) = crate::annotation_meta::find_annotation(self.__anno_entries(), &anno)
        else { return Ok(Object::default()) };
        crate::annotation_meta::annotation_instance(hit.anno, hit.elements)
    }

    /// `isAnnotationPresent(Class)`：纯名匹配。
    pub fn isAnnotationPresent(&self, annotationClass: Class) -> Result<bool> {
        let anno = format!("{}", annotationClass.__get_name()).replace('.', "/");
        Ok(crate::annotation_meta::has_annotation(self.__anno_entries(), &anno))
    }

    /// `getAnnotations()`：全部注解实例（声明序）。
    pub fn getAnnotations(&self) -> Result<JArray<Object>> {
        let mut out: Vec<Object> = Vec::new();
        for e in self.__anno_entries() {
            out.push(crate::annotation_meta::annotation_instance(e.anno, e.elements)?);
        }
        Ok(JArray::from(out))
    }

    /// `getDeclaredAnnotations()`：RuntimeVisibleAnnotations 即声明面。
    pub fn getDeclaredAnnotations(&self) -> Result<JArray<Object>> {
        self.getAnnotations()
    }

    /// 字段元数据路由（get/set 共用）：返回 (描述符, static, 修饰位, ConstantValue)。
    /// 元数据缺席（非 getDeclaredField 构造的 Field）→ IllegalArgumentException。
    fn __meta(&self) -> Result<(&'static str, bool, i32, Option<i64>)> {
        let name = format!("{}", self.__get_name());
        match self.__get_clazz().__declared_field_meta(&name) {
            Some(meta) => Ok(meta),
            None => Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                String::from(format!("Not a declared field: {}", name)))?)),
        }
    }

    /// `Field.get(Object)`：静态字段 obj 忽略；实例字段要求接收者非 null 且
    /// 与声明类赋值兼容（否则 NPE / IllegalArgumentException，与 JDK 同序）。
    /// 访问检查（无调用方建模的近似）：`setAccessible(true)`（override_）或
    /// 字段 public 之外，非 public 字段一律 IllegalAccessException——与 JDK
    /// 对 java.base 私有字段的可达行为同型（真 Java 从未命名模块不可访问）。
    #[jvm_native(upcalls = "java/lang/IllegalAccessException.<init>:(Ljava/lang/String;)V")]
    pub fn __impl_get(&self, obj: Object) -> Result<Object> {
        let name = format!("{}", self.__get_name());
        let (descriptor, is_static, mods, constant) = self.__meta()?;
        let __decl = format!("{}", self.__get_clazz().__get_name()).replace('.', "/");
        if !crate::reflect_dispatch::member_accessible(&__decl, mods, self.__get_override_()) {
            return Err(JvmError::from(crate::java::lang::IllegalAccessException::new_str(
                String::from(format!("Class can not access a member with modifiers {}", mods)))?));
        }
        if is_static {
            if let Some(r) = crate::reflect_dispatch::reflect_field(&__decl, &name, Object::default(), None) {
                return r;
            }
            return match (descriptor, constant) {
                // ConstantValue 整型常量：值即 class 文件常量（按描述符装箱）
                ("J", Some(v)) => Ok(Object::from(v)),
                ("I", Some(v)) => Ok(Object::from(v as i32)),
                ("S", Some(v)) => Ok(Object::from(v as i16)),
                ("B", Some(v)) => Ok(Object::from(v as i8)),
                ("C", Some(v)) => Ok(Object::from(v as u16)),
                ("Z", Some(v)) => Ok(Object::from(v != 0)),
                _ => panic!("stub: Field.get 静态字段无按名协议（无 ConstantValue 的静态字 / F/D 位形未分辨）: {}",
                            name),
            };
        }
        if obj.0.is_jvm_null() {
            // JVMS：实例字段读取的隐式 null 检查
            return Err(JvmError::null_pointer());
        }
        // vtable 的 is_instance_of 按 binary name（斜线形态）比较
        let cls_key = format!("{}", self.__get_clazz().__get_name()).replace('.', "/");
        if !obj.0.is_instance_of(&cls_key) {
            return Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                String::from(format!("Not a field of class {}", cls_key.replace('/', "."))))?));
        }
        if let Some(r) = crate::reflect_dispatch::reflect_field(&__decl, &name, Clone::clone(&obj), None) {
            return r;
        }
        match descriptor {
            "J" => match obj.0.__unsafe_long_cell(&name) {
                Some(cell) => Ok(Object::from(cell.get())),
                None => Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                    String::from(format!("Not a flat long field: {}", name)))?)),
            },
            "I" => match obj.0.__unsafe_int_cell(&name) {
                Some(cell) => Ok(Object::from(cell.get())),
                None => Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                    String::from(format!("Not a flat int field: {}", name)))?)),
            },
            "Z" => match obj.0.__unsafe_bool_cell(&name) {
                Some(cell) => Ok(Object::from(cell.get())),
                None => Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                    String::from(format!("Not a flat boolean field: {}", name)))?)),
            },
            d if d.starts_with('L') || d.starts_with('[') => match obj.0.__unsafe_ref_get(&name) {
                Some(v) => Ok(v),
                None => Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                    String::from(format!("Not a reference field: {}", name)))?)),
            },
            _ => panic!("stub: Field.get 其余基本类型（S/B/C/F/D 实例字段）无按名协议: {}", name),
        }
    }

    /// 基本类型读取族的公共底座：经 `__impl_get` 取值盒（访问检查 / 接收者
    /// 检查 / 元数据路由同源），按字段描述符做 JLS §5.1.2 拓宽到目标整型。
    /// 描述符不在 `widen_from` 名单 → IllegalArgumentException（JDK
    /// `UnsafeFieldAccessorImpl.newGetIllegalArgumentException` 同型）。
    fn __get_widened_integral(&self, obj: Object, widen_from: &str, target: &str) -> Result<i64> {
        let (descriptor, _, _, _) = self.__meta()?;
        if descriptor.len() != 1 || !widen_from.contains(descriptor) {
            return Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                String::from(format!("Attempt to get {} field \"{}\" with illegal data type conversion to {}",
                                     descriptor, self.__get_name(), target)))?));
        }
        let boxed = self.__impl_get(obj)?;
        // 值盒的 toString 即十进制字面量（Character 盒按码点，Boolean 不在拓宽名单）
        let text = boxed.0.__obj_str();
        let v = if descriptor == "C" {
            text.chars().next().map(|c| c as i64)
        } else {
            text.parse::<i64>().ok()
        };
        match v {
            Some(v) => Ok(v),
            None => Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                String::from(format!("Not an integral value for field {}", self.__get_name())))?)),
        }
    }

    /// `Field.getLong(Object)`：long 及可拓宽到 long 的 int/short/byte/char 字段
    /// （ObjectStreamClass.getDeclaredSUID 的 `serialVersionUID` 静态常量读取即此路径）。
    #[jvm_native(upcalls = "java/lang/IllegalAccessException.<init>:(Ljava/lang/String;)V")]
    pub fn __impl_getLong(&self, obj: Object) -> Result<i64> {
        self.__get_widened_integral(obj, "JISBC", "long")
    }

    /// `Field.getInt(Object)`：int 及可拓宽到 int 的 short/byte/char 字段。
    #[jvm_native(upcalls = "java/lang/IllegalAccessException.<init>:(Ljava/lang/String;)V")]
    pub fn __impl_getInt(&self, obj: Object) -> Result<i32> {
        Ok(self.__get_widened_integral(obj, "ISBC", "int")? as i32)
    }

    /// `Field.set(Object, Object)`：接收者/访问检查与 get 同序；值按描述符拆箱
    /// （只接受站点装箱的基本值盒，包装类对象形态见文件头边界注记）。
    #[jvm_native(upcalls = "java/lang/IllegalAccessException.<init>:(Ljava/lang/String;)V")]
    pub fn __impl_set(&self, obj: Object, value: Object) -> Result<()> {
        let name = format!("{}", self.__get_name());
        let (descriptor, is_static, mods, _constant) = self.__meta()?;
        let __decl = format!("{}", self.__get_clazz().__get_name()).replace('.', "/");
        if !crate::reflect_dispatch::member_accessible(&__decl, mods, self.__get_override_()) {
            return Err(JvmError::from(crate::java::lang::IllegalAccessException::new_str(
                String::from(format!("Class can not access a member with modifiers {}", mods)))?));
        }
        // final 字段：静态 final 恒不可写；实例 final 需 setAccessible(true)（JDK Field.set 语义）
        if (mods & 0x0010) != 0 && (is_static || !self.__get_override_()) {
            return Err(JvmError::from(crate::java::lang::IllegalAccessException::new_str(
                String::from(format!("Can not set {}final field {}.{}",
                                     if is_static { "static " } else { "" },
                                     __decl.replace('/', "."), name)))?));
        }
        // 引用赋值兼容检查（JDK：值非 null 且不可赋给字段类型 → IllegalArgumentException）
        if descriptor.starts_with('L') && !value.0.is_jvm_null() {
            let want = &descriptor[1..descriptor.len() - 1];
            if want != "java/lang/Object" && !value.0.is_instance_of(want) {
                return Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                    String::from(format!("Can not set {} field {}.{} to {}",
                                         want.replace('/', "."), __decl.replace('/', "."), name,
                                         value.0.__class_name().replace('/', "."))))?));
            }
        }
        if is_static {
            if let Some(r) = crate::reflect_dispatch::reflect_field(&__decl, &name, Object::default(), Some(value)) {
                return r.map(|_| ());
            }
            panic!("stub: Field.set 静态字段无按名协议（字段闭包缺席）: {}", name);
        }
        if obj.0.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        // vtable 的 is_instance_of 按 binary name（斜线形态）比较
        let cls_key = format!("{}", self.__get_clazz().__get_name()).replace('.', "/");
        if !obj.0.is_instance_of(&cls_key) {
            return Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                String::from(format!("Not a field of class {}", cls_key.replace('/', "."))))?));
        }
        if let Some(r) = crate::reflect_dispatch::reflect_field(&__decl, &name, Clone::clone(&obj), Some(Clone::clone(&value))) {
            return r.map(|_| ());
        }
        match descriptor {
            "J" => match (obj.0.__unsafe_long_cell(&name), __unbox_long(&value)) {
                (Some(cell), Some(v)) => { cell.set(v); Ok(()) }
                (_, None) => Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                    String::from(format!("Not a long value for field {}", name)))?)),
                (None, _) => Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                    String::from(format!("Not a flat long field: {}", name)))?)),
            },
            "I" => match (obj.0.__unsafe_int_cell(&name), __unbox_int(&value)) {
                (Some(cell), Some(v)) => { cell.set(v); Ok(()) }
                (_, None) => Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                    String::from(format!("Not an int value for field {}", name)))?)),
                (None, _) => Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                    String::from(format!("Not a flat int field: {}", name)))?)),
            },
            "Z" => match (obj.0.__unsafe_bool_cell(&name), crate::reflect_dispatch::unbox_bool(&value)) {
                (Some(cell), Some(v)) => { cell.set(v); Ok(()) }
                (_, None) => Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                    String::from(format!("Not a boolean value for field {}", name)))?)),
                (None, _) => Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                    String::from(format!("Not a flat boolean field: {}", name)))?)),
            },
            d if d.starts_with('L') || d.starts_with('[') => {
                if obj.0.__unsafe_ref_set(&name, value) {
                    Ok(())
                } else {
                    Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                        String::from(format!("Not a reference field: {}", name)))?))
                }
            }
            _ => panic!("stub: Field.set 其余基本类型（S/B/C/F/D 实例字段）无按名协议: {}", name),
        }
    }
}
