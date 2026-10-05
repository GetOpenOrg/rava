//! JArray 的 ObjectVTable 实现：数组作为 Java 对象的运行时行为（宿主 array.rs 的私有辅助模块）

use super::*;

/// null 数组装入后经 vtable 的 is_jvm_null 呈现 Java null 语义。
impl<T: Clone + Default + From<Object> + Into<Object> + 'static + crate::sync_model::__ThreadSafe> crate::java::lang::ObjectVTable for JArray<T> {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn __identity(&self) -> *const () { self.identity() }
    fn is_jvm_null(&self) -> bool { JArray::is_jvm_null(self) }
    fn __array_len(&self) -> Option<crate::error::Result<i32>> { Some(self.len()) }

    /// 数组类的 Class 对象（JLS §10.8：`new String[0].getClass()` 是
    /// `[Ljava.lang.String;`）。binary name 为 JVM 描述符形态、斜线键——与
    /// ldc 的 `X[].class`（`Class::for_class("[Ljava/lang/String;")`）落同一
    /// 缓存条目，`a.getClass() == X[].class` 的身份语义由此成立。协变视图
    /// 委托源数组（数组类由创建时的元素类型决定，与观察形态无关）；基本元素
    /// 静态取描述符字符；引用元素经元素 vtable 的 getClass 递归取得（嵌套
    /// 数组因此正确：`JArray<JArray<T>>` → `[[T`）。
    fn getClass(&self) -> crate::error::Result<crate::java::lang::Class> {
        // 协变视图：`String[]` 以 `Object[]` 形态流转时 getClass 仍是
        // `[Ljava.lang.String;`——委托源数组取运行时元素类型。
        if let Repr::Covariant(view) = &*self.0 {
            return view.origin.0.getClass();
        }
        if let Some(d) = Self::primitive_elem_descriptor() {
            return Ok(crate::java::lang::Class::for_class(
                crate::java::lang::String::from(format!("[{}", d).as_str())));
        }
        if let Repr::Own(_, Some(tag)) = &*self.0 {
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
        //   - null 数组通过任意 instanceof（JVMS aconst_null 语义）；
        //   - 协变视图按源数组（运行时元素类型）判定；
        //   - 同描述符成立；目标 "[Ljava/lang/Object;" 按数组协变恒成立
        //（JLS §4.10.4：任意引用元素数组是 Object[] 的子类型——基本元素
        //     数组不是，故只在引用元素分支放行）。
        if matches!(&*self.0, Repr::Null) {
            return true;
        }
        if let Repr::Covariant(view) = &*self.0 {
            return view.origin.0.is_instance_of(type_id);
        }
        if let Some(d) = Self::primitive_elem_descriptor() {
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
        match &*self.0 {
            Repr::Own(cells, tag) => {
                let data = cells.borrow();
                Some(Object::from(JArray(Rc::new(Repr::Own(RefCell::new(data.clone()), tag.clone())))))
            }
            Repr::Covariant(view) => view.origin.0.__shallow_copy(),
            Repr::Null => Some(Object::from(Clone::clone(self))),
        }
    }

    /// checkcast 到数组类型：同元素类型 → 自身；视图还原 → 交给源数组判定；
    /// 引用类型数组 → `Object[]`：协变视图。其余目标元素类型由
    /// `From<Object> for JArray<T>`（知道目标元素类型）经 `__array_elem_assignable`
    /// 判定。null 通过任意引用类型的 checkcast（JVMS §6.5 checkcast）。
    fn __view_into(&self, any: crate::sync_model::__AnyRef, slot: &mut dyn std::any::Any) -> bool {
        if let Some(same) = slot.downcast_mut::<Option<Self>>() {
            *same = Some(Clone::clone(self));
            return true;
        }
        if matches!(&*self.0, Repr::Null) {
            // null 通过任意数组类型的 checkcast：以目标形态的 null 还原
            if let Some(erased) = slot.downcast_mut::<Option<JArray<Object>>>() {
                *erased = Some(JArray::default());
                return true;
            }
            return false;
        }
        if let Repr::Covariant(view) = &*self.0 {
            return view.origin.0.__view_into(any, slot);
        }
        if let Some(erased) = slot.downcast_mut::<Option<JArray<Object>>>() {
            if !Self::has_primitive_elements() {
                *erased = Some(JArray(Rc::new(Repr::Covariant(covariant_view(self)))));
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
        if Self::has_primitive_elements() {
            return false;
        }
        match &*self.0 {
            Repr::Covariant(view) => view.origin.0.__array_elem_assignable(target_elem, slot),
            Repr::Own(..) => {
                let probe: Object = Into::<Object>::into(T::default());
                if let Some(d) = probe.0.__desc() {
                    return d.is_subtype_name(target_elem);
                }
                let unused = crate::sync_model::__unused_any();
                probe.0.__view_into(unused, slot)
            }
            Repr::Null => false,
        }
    }
}
