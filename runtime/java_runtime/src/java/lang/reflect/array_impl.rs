use crate::prelude::*;
use super::Array;
use crate::java::lang::Class;
use crate::java::lang::Object;

impl Array {
    /// native `Array.newArray(Class, int)`：反射创建一维数组（消费链：
    /// `Arrays.copyOf(orig, len, newType)` → `Array.newInstance(
    /// newType.getComponentType(), len)`，即 `Collection.toArray(T[] a)` 的
    /// "按运行时类型新建数组" 分支）。
    ///
    /// 原生二进制没有按运行时 Class 动态选定 Rust 元素类型的机制；反射创建的
    /// 引用数组以擦除载体（`JArray<Object>`，元素初值 null）+ 组件类型标签承载
    /// （FS-R6）：getClass 为 `[L<component>;`，checkcast / instanceof 按组件类型
    /// 可赋值精确判定，aastore 按标签做存储检查（ArrayStoreException）。
    /// 基本类型 componentType（消费链：`ObjectInputStream.readArray` 按流中
    /// 描述符 `Array.newInstance(int.class, n)` 后 `(int[]) array` 批量读入）按
    /// `getPrimitiveClass` 名字分派到对应的基本元素载体（`[I` 等真实数组，
    /// 零初值）；`void` → IllegalArgumentException（JDK 同）。null componentType
    /// → NPE、负长度 → NegativeArraySizeException（与 VM 行为一致）。
    #[jvm_native]
    pub fn newArray(componentType: Class, length: i32) -> Result<Object> {
        if Object::from(Clone::clone(&componentType)).0.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        if length < 0 {
            return Err(JvmError::negative_array_size(length));
        }
        if componentType.isPrimitive()? {
            let name = format!("{}", componentType.__get_name());
            return Ok(match name.as_str() {
                "int" => Object::from(JArray::<i32>::new(length)),
                "long" => Object::from(JArray::<i64>::new(length)),
                "short" => Object::from(JArray::<i16>::new(length)),
                "byte" => Object::from(JArray::<i8>::new(length)),
                "char" => Object::from(JArray::<u16>::new(length)),
                "float" => Object::from(JArray::<f32>::new(length)),
                "double" => Object::from(JArray::<f64>::new(length)),
                "boolean" => Object::from(JArray::<bool>::new(length)),
                _ => return Err(JvmError::illegal_argument("")),
            });
        }
        // 引用组件：擦除载体 + 组件类型标签（FS-R6），运行时数组类即 `[L<component>;`
        let component = format!("{}", componentType.__get_name()).replace('.', "/");
        Ok(Object::from(JArray::<Object>::__new_component_tagged(length, &component)))
    }

    /// native `Array.getLength(Object)`：任意元素类型数组的长度（与 arraylength 同源，
    /// 经 vtable 钩子 `__array_len`，与元素类型无关）。null → NPE；非数组 →
    /// IllegalArgumentException("Argument is not an array")（HotSpot Reflection::array_get_length 同）。
    #[jvm_native]
    pub fn getLength(array: Object) -> Result<i32> {
        if array.0.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        match array.0.__array_len() {
            Some(len) => len,
            None => Err(JvmError::illegal_argument("Argument is not an array")),
        }
    }

    /// native `multiNewArray(Class componentType, int[] dimensions)`：多维数组（JVMS multianewarray
    /// 语义）。componentType 为最内层元素类型；外层逐维建 `[..[L<component>;` 标签的引用数组，
    /// 最内一维按 newArray 建元素数组（基本类型载体或带组件标签的引用数组）。
    /// 维度为空 / 超过 255 → IllegalArgumentException，任一维为负 → NegativeArraySizeException。
    #[jvm_native]
    pub fn multiNewArray(componentType: Class, dimensions: JArray<i32>) -> Result<Object> {
        if Object::from(Clone::clone(&componentType)).0.is_jvm_null() || dimensions.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let n = dimensions.len()?;
        if n == 0 || n > 255 {
            return Err(JvmError::illegal_argument("Wrong number of dimensions"));
        }
        let mut dims = Vec::with_capacity(n as usize);
        for i in 0..n {
            let d = dimensions.get(i)?;
            if d < 0 {
                return Err(JvmError::negative_array_size(d));
            }
            dims.push(d);
        }
        Self::__multi_new(&componentType, &dims)
    }

    fn __multi_new(component: &Class, dims: &[i32]) -> Result<Object> {
        if dims.len() == 1 {
            return Self::newArray(Clone::clone(component), dims[0]);
        }
        // 外层元素类型描述符：(dims.len()-1) 层 '[' + 最内层组件描述符
        let inner = format!("{}", component.__get_name()).replace('.', "/");
        let leaf = if component.isPrimitive()? {
            match inner.as_str() {
                "int" => "I", "long" => "J", "short" => "S", "byte" => "B", "char" => "C",
                "float" => "F", "double" => "D", "boolean" => "Z",
                _ => return Err(JvmError::illegal_argument("")),
            }.to_owned()
        } else {
            format!("L{};", inner)
        };
        let elem_tag = format!("{}{}", "[".repeat(dims.len() - 1), leaf);
        let outer = JArray::<Object>::__new_component_tagged(dims[0], &elem_tag);
        for i in 0..dims[0] {
            outer.set(i, Self::__multi_new(component, &dims[1..])?)?;
        }
        Ok(Object::from(outer))
    }
}
