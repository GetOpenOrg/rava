//! 数组的 ObjectVTable 实现：数组作为 Java 对象的运行时行为（宿主 array.rs 的私有辅助模块）
//!
//! 非 null 数组是 `__ArrayObj<T>`；null 数组装入 Object 是 `__ArrayNull<T>` 静态哨兵，以元素类型
//! 应答数组类与 checkcast 探针（null 的身份与任意 null 相同）。

use super::*;

/// 数组类（JLS §10.8）：反射创建数组按组件标签，其余按元素类型 T。
fn array_class<T: Default + Into<Object> + 'static>(tag: Option<&Rc<str>>) -> crate::error::Result<crate::java::lang::Class> {
    if let Some(d) = JArray::<T>::primitive_elem_descriptor() {
        return Ok(crate::java::lang::Class::for_class(
            crate::java::lang::String::from(format!("[{}", d).as_str())));
    }
    if let Some(tag) = tag {
        let binary = if tag.starts_with('[') { format!("[{}", tag) } else { format!("[L{};", tag) };
        return Ok(crate::java::lang::Class::for_class(
            crate::java::lang::String::from(binary.as_str())));
    }
    let elem_name = format!("{}", Into::<Object>::into(T::default())
        .0.getClass()?
        .__get_name())
        .replace('.', "/");
    let binary = if elem_name.starts_with('[') {
        format!("[{}", elem_name)
    } else {
        format!("[L{};", elem_name)
    };
    Ok(crate::java::lang::Class::for_class(
        crate::java::lang::String::from(binary.as_str())))
}

impl<T: Clone + Default + From<Object> + Into<Object> + 'static + crate::sync_model::__ThreadSafe> crate::java::lang::ObjectVTable for __ArrayObj<T> {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn __identity(&self) -> *const () { self.identity() }
    /// 根类字节码方法体（equals / toString / wait）的 `this`：数组对象本身
    fn __object(&self) -> Option<Object> { Some(Object::from(self.handle())) }
    fn __array_len(&self) -> Option<crate::error::Result<i32>> { Some(Ok(self.len())) }

    /// 数组类的 Class 对象（JLS §10.8：`new String[0].getClass()` 是
    /// `[Ljava.lang.String;`）。binary name 为 JVM 描述符形态、斜线键——与
    /// ldc 的 `X[].class`（`Class::for_class("[Ljava/lang/String;")`）落同一
    /// 缓存条目，`a.getClass() == X[].class` 的身份语义由此成立。协变视图
    /// 委托源数组（数组类由创建时的元素类型决定，与观察形态无关）；基本元素
    /// 静态取描述符字符；引用元素经元素 vtable 的 getClass 递归取得（嵌套
    /// 数组因此正确：`JArray<JArray<T>>` → `[[T`）。
    fn getClass(&self) -> crate::error::Result<crate::java::lang::Class> {
        match self.form() {
            // 协变视图：`String[]` 以 `Object[]` 形态流转时 getClass 仍是
            // `[Ljava.lang.String;`——委托源数组取运行时元素类型。
            Form::Covariant(view) => view.origin.0.getClass(),
            Form::Own(_, tag) => array_class::<T>(tag),
        }
    }

    /// JLS §10.8 / §4.10.4：数组的直接超类型是 Object、Cloneable、Serializable。
    /// 有意不按 "java/lang/Object" 匹配——aastore_storable 的元素类型名探针对
    /// 多维数组退化为 "java/lang/Object"（数组的类名探针无元素信息），按名放行
    /// 会使异构数组元素的存储检查失效（元素是数组的情形走 `__view_into` 臂）。
    fn is_instance_of(&self, type_id: &str) -> bool {
        if matches!(type_id, "java/lang/Cloneable" | "java/io/Serializable") {
            return true;
        }
        if !type_id.starts_with('[') {
            return false;
        }
        // instanceof 数组目标（instanceof byte[] 等，描述符形态）：
        //   - 协变视图按源数组（运行时元素类型）判定；
        //   - 同描述符成立；目标 "[Ljava/lang/Object;" 按数组协变恒成立
        //（JLS §4.10.4：任意引用元素数组是 Object[] 的子类型——基本元素
        //     数组不是，故只在引用元素分支放行）。
        if let Form::Covariant(view) = self.form() {
            return view.origin.0.is_instance_of(type_id);
        }
        if let Some(d) = JArray::<T>::primitive_elem_descriptor() {
            return type_id == format!("[{}", d);
        }
        let target = type_id.replace('.', "/");
        if target == "[Ljava/lang/Object;" {
            return true;
        }
        // 引用元素 / 嵌套数组：按数组协变（JLS §4.10.3，组件类型可赋值）判定
        match self.getClass() {
            Ok(c) => crate::java::lang::Class::__name_assignable(
                &target, &format!("{}", c.__get_name()).replace('.', "/")),
            Err(_) => false,
        }
    }

    /// `Object.clone()`（invokevirtual）在数组上的语义（JLS §10.7）：浅拷贝——
    /// 新数组对象、逐元素共享引用（元素自身的 Clone 即引用共享）。Own 形态按
    /// 当前元素类型重建；协变视图委托源数组（克隆保持运行时元素类型，Java 的
    /// clone 不改变数组的具体类型）。
    fn __shallow_copy(&self) -> Option<Object> {
        match self.form() {
            Form::Own(store, tag) => {
                let elems = store.to_vec();
                Some(Object::from(JArray::own(elems.len(), tag.cloned(), elems)))
            }
            Form::Covariant(view) => view.origin.0.__shallow_copy(),
        }
    }

    /// checkcast 到数组类型：同元素类型 → 自身；视图还原 → 交给源数组判定；
    /// 引用类型数组 → `Object[]`：协变视图。其余目标元素类型由
    /// `From<Object> for JArray<T>`（知道目标元素类型）经 `__array_elem_assignable`
    /// 判定。null 通过任意引用类型的 checkcast（JVMS §6.5 checkcast）。
    fn __view_into(&self, any: crate::sync_model::__AnyRef, slot: &mut dyn std::any::Any) -> bool {
        if let Some(same) = slot.downcast_mut::<Option<JArray<T>>>() {
            *same = Some(self.handle());
            return true;
        }
        if let Form::Covariant(view) = self.form() {
            return view.origin.0.__view_into(any, slot);
        }
        if let Some(erased) = slot.downcast_mut::<Option<JArray<Object>>>() {
            if !JArray::<T>::has_primitive_elements() {
                *erased = Some(JArray::covariant(covariant_view_of(self)));
                return true;
            }
        }
        false
    }

    /// 数组协变的元素赋值兼容探针（S-4）：调用方（`From<Object> for JArray<T>`，
    /// 知道目标元素类型）构造 `Option<T>` slot；本钩子以「源元素类型的 null 探针」
    /// view_into 该 slot——wrapper 的祖先名单是静态生成的（与元素值无关），填充成功
    /// ⇔ 目标元素类型是源元素类型自身或其祖先（JLS §4.10.3 数组子类型条件）。
    /// 已是视图的数组按其源数组（运行时元素类型）判定；基本元素数组无协变视图。
    fn __array_accepts(&self, candidate: &Object) -> bool {
        try_array_view::<T>(candidate).is_some()
    }

    fn __array_elem_assignable(&self, target_elem: &str, slot: &mut dyn std::any::Any) -> bool {
        if JArray::<T>::has_primitive_elements() {
            return false;
        }
        match self.form() {
            Form::Covariant(view) => view.origin.0.__array_elem_assignable(target_elem, slot),
            Form::Own(..) => {
                let probe: Object = Into::<Object>::into(T::default());
                if let Some(d) = probe.0.__desc() {
                    return d.is_subtype_name(target_elem);
                }
                let unused = crate::sync_model::__unused_any();
                probe.0.__view_into(unused, slot)
            }
        }
    }
}

/// null 数组装入 Object：Java null 语义（身份同任意 null），数组类与 checkcast 按元素类型 T 应答。
impl<T: Clone + Default + From<Object> + Into<Object> + 'static + crate::sync_model::__ThreadSafe> crate::java::lang::ObjectVTable for __ArrayNull<T> {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn __obj_str(&self) -> std::string::String { "null".to_owned() }
    fn is_jvm_null(&self) -> bool { true }
    fn __identity(&self) -> *const () { Object::default().0.__identity() }
    fn __array_len(&self) -> Option<crate::error::Result<i32>> {
        Some(Err(crate::error::JvmError::null_pointer()))
    }
    fn getClass(&self) -> crate::error::Result<crate::java::lang::Class> { array_class::<T>(None) }

    /// null 通过任意引用类型的 instanceof 数组目标探针（JVMS aconst_null 语义，同非 null 数组的超类型）
    fn is_instance_of(&self, type_id: &str) -> bool {
        matches!(type_id, "java/lang/Cloneable" | "java/io/Serializable") || type_id.starts_with('[')
    }

    fn __shallow_copy(&self) -> Option<Object> { Some(Object::from(JArray::<T>::default())) }

    /// null 通过任意数组类型的 checkcast：以目标形态的 null 还原
    fn __view_into(&self, _any: crate::sync_model::__AnyRef, slot: &mut dyn std::any::Any) -> bool {
        if let Some(same) = slot.downcast_mut::<Option<JArray<T>>>() {
            *same = Some(JArray::default());
            return true;
        }
        if let Some(erased) = slot.downcast_mut::<Option<JArray<Object>>>() {
            *erased = Some(JArray::default());
            return true;
        }
        false
    }

    fn __array_accepts(&self, candidate: &Object) -> bool {
        try_array_view::<T>(candidate).is_some()
    }

    fn __array_elem_assignable(&self, _target_elem: &str, _slot: &mut dyn std::any::Any) -> bool { false }
}

/// 数组引用 `JArray<T>` 作为 Java 引用应答根类方法（数组接收者上的 `hashCode` / `equals` /
/// `synchronized` / `Object__clone_base(&a)` 等静态调用）：逐方法转交所引用的数组对象，null 引用
/// 转交本元素类型的 null 哨兵。引用本身不是对象，不装入 Object（`From<JArray<T>> for Object`
/// 取出数组对象本身）。
impl<T: Clone + Default + From<Object> + Into<Object> + 'static + crate::sync_model::__ThreadSafe> JArray<T> {
    #[inline]
    fn target(&self) -> &dyn crate::java::lang::ObjectVTable {
        match self.0.as_deref() {
            Some(o) => o,
            None => const { &__ArrayNull::<T>::NULL },
        }
    }
}

impl<T: Clone + Default + From<Object> + Into<Object> + 'static + crate::sync_model::__ThreadSafe> crate::java::lang::ObjectVTable for JArray<T> {
    fn hashCode(&self) -> i32 { self.target().hashCode() }
    fn equals(&self, other: Object) -> crate::error::Result<bool> { self.target().equals(other) }
    fn __obj_str(&self) -> std::string::String { self.target().__obj_str() }
    fn __to_string(&self) -> crate::error::Result<std::string::String> { self.target().__to_string() }
    fn __proxy_invoke(&self, iface: &str, name: &str, desc: &str, args: Vec<Object>)
        -> Option<crate::error::Result<Object>> {
        self.target().__proxy_invoke(iface, name, desc, args)
    }
    fn is_instance_of(&self, type_id: &str) -> bool { self.target().is_instance_of(type_id) }
    fn __desc(&self) -> Option<&'static crate::class_desc::__ClassDesc> { self.target().__desc() }
    fn as_any(&self) -> &dyn std::any::Any { self.target().as_any() }
    fn getClass(&self) -> crate::error::Result<crate::java::lang::Class> { self.target().getClass() }
    fn compareTo(&self, other: Object) -> crate::error::Result<i32> { self.target().compareTo(other) }
    fn is_jvm_null(&self) -> bool { self.0.is_none() }
    fn __interface(&self, slot: &mut dyn std::any::Any) { self.target().__interface(slot) }
    fn __class_name(&self) -> &'static str { self.target().__class_name() }
    fn __identity(&self) -> *const () { self.target().__identity() }
    fn __object(&self) -> Option<Object> { self.target().__object() }
    fn __array_len(&self) -> Option<crate::error::Result<i32>> { self.target().__array_len() }
    fn __erased_vtable(&self, depth: u16, slot: &mut dyn std::any::Any) { self.target().__erased_vtable(depth, slot) }
    fn __array_elem_assignable(&self, target_elem: &str, slot: &mut dyn std::any::Any) -> bool {
        self.target().__array_elem_assignable(target_elem, slot)
    }
    fn __array_accepts(&self, candidate: &Object) -> bool { self.target().__array_accepts(candidate) }
    fn __view_into(&self, any: crate::sync_model::__AnyRef, slot: &mut dyn std::any::Any) -> bool {
        self.target().__view_into(any, slot)
    }
    fn __shallow_copy(&self) -> Option<Object> { self.target().__shallow_copy() }
}
