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
    /// 引用数组以擦除元素形态（`JArray<Object>`，元素初值 null）承载——与
    /// javac 对 `T[]` 擦除后的运行时形态一致，arraycopy 存入具体元素后，调用
    /// 侧经既有擦除还原（array.rs `try_array_view` 的逐元素兼容）恢复具体数组
    /// 类型。基本类型 componentType（消费链：`ObjectInputStream.readArray` 按流中
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
        Ok(Object::from(JArray::<Object>::new(length)))
    }
}
