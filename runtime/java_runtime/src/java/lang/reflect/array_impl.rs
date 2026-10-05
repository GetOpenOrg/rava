use crate::prelude::*;
use super::Array;
use crate::java::lang::{
    Boolean, Byte, Character, Class, Double, Float, IllegalArgumentException, Integer, Long, Object, Short,
};
use crate::reflect_dispatch as rd;

/// 基本类型值（数组元素读出 / 写入实参），按 JVMS 基本类型种类区分。
#[derive(Clone, Copy)]
enum Prim {
    Z(bool),
    B(i8),
    C(u16),
    S(i16),
    I(i32),
    J(i64),
    F(f32),
    D(f64),
}

impl Prim {
    /// JLS §5.1.2 基本类型拓宽（含同型）：转为 `kind`（描述符字符）种类；不允许的转换 → None。
    fn widen(self, kind: u8) -> Option<Prim> {
        use Prim::*;
        Some(match (self, kind) {
            (Z(v), b'Z') => Z(v),
            (B(v), b'B') => B(v),
            (B(v), b'S') => S(v as i16),
            (B(v), b'I') => I(v as i32),
            (S(v), b'S') => S(v),
            (S(v), b'I') => I(v as i32),
            (C(v), b'C') => C(v),
            (C(v), b'I') => I(v as i32),
            (I(v), b'I') => I(v),
            (B(v), b'J') => J(v as i64),
            (S(v), b'J') => J(v as i64),
            (C(v), b'J') => J(v as i64),
            (I(v), b'J') => J(v as i64),
            (J(v), b'J') => J(v),
            (B(v), b'F') => F(v as f32),
            (S(v), b'F') => F(v as f32),
            (C(v), b'F') => F(v as f32),
            (I(v), b'F') => F(v as f32),
            (J(v), b'F') => F(v as f32),
            (F(v), b'F') => F(v),
            (B(v), b'D') => D(v as f64),
            (S(v), b'D') => D(v as f64),
            (C(v), b'D') => D(v as f64),
            (I(v), b'D') => D(v as f64),
            (J(v), b'D') => D(v as f64),
            (F(v), b'D') => D(v as f64),
            (D(v), b'D') => D(v),
            _ => return None,
        })
    }

    /// 装箱（包装类 valueOf，含缓存池身份语义——与 JDK Reflection::array_get 的 box 一致）。
    fn boxed(self) -> Result<Object> {
        Ok(match self {
            Prim::Z(v) => Object::from(Boolean::valueOf_z(v)?),
            Prim::B(v) => Object::from(Byte::valueOf_b(v)?),
            Prim::C(v) => Object::from(Character::valueOf(v)?),
            Prim::S(v) => Object::from(Short::valueOf_s(v)?),
            Prim::I(v) => Object::from(Integer::valueOf_i(v)?),
            Prim::J(v) => Object::from(Long::valueOf_l(v)?),
            Prim::F(v) => Object::from(Float::valueOf_f(v)?),
            Prim::D(v) => Object::from(Double::valueOf_d(v)?),
        })
    }

    /// 拆箱（JDK Reflection::unbox_for_primitive）：包装类实例 → 其基本值；null / 非包装类 → None。
    fn unboxed(v: &Object) -> Option<Prim> {
        if v.0.is_jvm_null() {
            return None;
        }
        let any = v.0.as_any();
        macro_rules! native_box {
            ($($t:ty => $k:ident),*) => {
                $(if let Some(b) = any.downcast_ref::<$t>() { return Some(Prim::$k(*b)); })*
            };
        }
        native_box!(bool => Z, i8 => B, u16 => C, i16 => S, i32 => I, i64 => J, f32 => F, f64 => D);
        match class_name(v)?.as_str() {
            "java/lang/Boolean" => rd::unbox_bool(v).map(Prim::Z),
            "java/lang/Byte" => rd::unbox_i32(v).map(|x| Prim::B(x as i8)),
            "java/lang/Character" => rd::unbox_char(v).map(Prim::C),
            "java/lang/Short" => rd::unbox_i32(v).map(|x| Prim::S(x as i16)),
            "java/lang/Integer" => rd::unbox_i32(v).map(Prim::I),
            "java/lang/Long" => rd::unbox_i64(v).map(Prim::J),
            "java/lang/Float" => rd::unbox_f32(v).map(Prim::F),
            "java/lang/Double" => rd::unbox_f64(v).map(Prim::D),
            _ => None,
        }
    }
}

/// 非 null 对象的运行时类名（斜线形态；数组为描述符形态）
fn class_name(v: &Object) -> Option<std::string::String> {
    let c = v.0.getClass().ok()?;
    Some(format!("{}", c.__get_name()).replace('.', "/"))
}

/// JDK Reflection::array_get / array_set 的类型不符消息
fn type_mismatch() -> JvmError {
    JvmError::illegal_argument("argument type mismatch")
}

/// `getXxx` / `setXxx` 作用于引用元素数组（JDK Reflection::array_get / array_set 的 type 检查）
fn not_primitive_array() -> JvmError {
    JvmError::illegal_argument("Argument is not an array of primitive type")
}

/// 基本元素数组 `set` 的 null 值：JDK 抛无消息的 IllegalArgumentException
fn null_for_primitive() -> JvmError {
    match IllegalArgumentException::new() {
        Ok(e) => JvmError::from(e),
        Err(nested) => nested,
    }
}

/// 数组实参的元素形态：基本元素（读写闭包按种类分派）或引用元素（Object 级协变视图 + 运行时数组类名）。
enum Elems {
    Prim(u8),
    Ref(JArray<Object>, std::string::String),
}

/// 实参数组的元素形态。null → NPE；非数组 → IllegalArgumentException("Argument is not an array")。
fn elems_of(array: &Object) -> Result<Elems> {
    if array.0.is_jvm_null() {
        return Err(JvmError::null_pointer());
    }
    let any = array.0.as_any();
    macro_rules! prim_kind {
        ($($t:ty => $k:literal),*) => {
            $(if any.downcast_ref::<crate::array::__ArrayObj<$t>>().is_some() { return Ok(Elems::Prim($k)); })*
        };
    }
    prim_kind!(bool => b'Z', i8 => b'B', u16 => b'C', i16 => b'S', i32 => b'I', i64 => b'J', f32 => b'F', f64 => b'D');
    let unused = crate::sync_model::__unused_any();
    let mut view: Option<JArray<Object>> = None;
    array.0.__view_into(unused, &mut view);
    match view {
        Some(v) if array.0.__array_len().is_some() => {
            let name = format!("{}", array.0.getClass()?.__get_name()).replace('.', "/");
            Ok(Elems::Ref(v, name))
        }
        _ => Err(JvmError::illegal_argument("Argument is not an array")),
    }
}

/// 基本元素数组的第 i 个元素
fn prim_get(array: &Object, kind: u8, index: i32) -> Result<Prim> {
    let any = array.0.as_any();
    macro_rules! read {
        ($t:ty, $k:ident) => {
            match any.downcast_ref::<crate::array::__ArrayObj<$t>>() {
                Some(a) => Prim::$k(a.get(index)?),
                None => return Err(type_mismatch()),
            }
        };
    }
    Ok(match kind {
        b'Z' => read!(bool, Z),
        b'B' => read!(i8, B),
        b'C' => read!(u16, C),
        b'S' => read!(i16, S),
        b'I' => read!(i32, I),
        b'J' => read!(i64, J),
        b'F' => read!(f32, F),
        _ => read!(f64, D),
    })
}

/// 基本元素数组的第 i 个元素写入 v（v 先按元素种类拓宽，不允许 → IllegalArgumentException）
fn prim_set(array: &Object, kind: u8, index: i32, v: Prim) -> Result<()> {
    let any = array.0.as_any();
    macro_rules! write {
        ($t:ty, $k:ident) => {
            match (any.downcast_ref::<crate::array::__ArrayObj<$t>>(), v.widen(kind)) {
                (Some(a), Some(Prim::$k(x))) => a.set(index, x),
                _ => Err(type_mismatch()),
            }
        };
    }
    match kind {
        b'Z' => write!(bool, Z),
        b'B' => write!(i8, B),
        b'C' => write!(u16, C),
        b'S' => write!(i16, S),
        b'I' => write!(i32, I),
        b'J' => write!(i64, J),
        b'F' => write!(f32, F),
        _ => write!(f64, D),
    }
}

/// `getXxx`：基本元素读出并拓宽到 `kind`；引用元素数组 → IllegalArgumentException（非基本数组）。
fn get_as(array: &Object, index: i32, kind: u8) -> Result<Prim> {
    match elems_of(array)? {
        Elems::Prim(k) => prim_get(array, k, index)?.widen(kind).ok_or_else(type_mismatch),
        Elems::Ref(..) => Err(not_primitive_array()),
    }
}

/// `setXxx`：基本值写入基本元素数组（拓宽）；引用元素数组 → IllegalArgumentException。
fn set_as(array: &Object, index: i32, v: Prim) -> Result<()> {
    match elems_of(array)? {
        Elems::Prim(k) => prim_set(array, k, index, v),
        Elems::Ref(..) => Err(not_primitive_array()),
    }
}

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

    /// native `Array.get(Object, int)`（JDK Reflection::array_get）：引用元素原样返回，基本元素装箱。
    /// null 数组 → NPE；非数组 → IllegalArgumentException；越界 → ArrayIndexOutOfBoundsException。
    #[jvm_native]
    pub fn get(array: Object, index: i32) -> Result<Object> {
        match elems_of(&array)? {
            Elems::Prim(k) => prim_get(&array, k, index)?.boxed(),
            Elems::Ref(v, _) => v.get(index),
        }
    }

    /// native `Array.set(Object, int, Object)`（JDK Reflection::array_set）：引用元素数组按组件类型
    /// 检查值（null 恒可存；不可赋值 → IllegalArgumentException("array element type mismatch")）；
    /// 基本元素数组先拆箱（null / 非包装类 → IllegalArgumentException）再按 JLS §5.1.2 拓宽写入。
    #[jvm_native]
    pub fn set(array: Object, index: i32, value: Object) -> Result<()> {
        match elems_of(&array)? {
            Elems::Prim(k) => {
                if value.0.is_jvm_null() {
                    return Err(null_for_primitive());
                }
                let v = Prim::unboxed(&value).ok_or_else(type_mismatch)?;
                prim_set(&array, k, index, v)
            }
            Elems::Ref(v, name) => {
                if !value.0.is_jvm_null() {
                    let component = name.strip_prefix('[').unwrap_or(&name);
                    let component = component.strip_prefix('L').and_then(|c| c.strip_suffix(';')).unwrap_or(component);
                    let value_class = class_name(&value).unwrap_or_default();
                    if !Class::__name_assignable(component, &value_class) {
                        return Err(JvmError::illegal_argument("array element type mismatch"));
                    }
                }
                v.set(index, value)
            }
        }
    }

    #[jvm_native]
    pub fn getBoolean(array: Object, index: i32) -> Result<bool> {
        match get_as(&array, index, b'Z')? { Prim::Z(v) => Ok(v), _ => Err(type_mismatch()) }
    }

    #[jvm_native]
    pub fn getByte(array: Object, index: i32) -> Result<i8> {
        match get_as(&array, index, b'B')? { Prim::B(v) => Ok(v), _ => Err(type_mismatch()) }
    }

    #[jvm_native]
    pub fn getChar(array: Object, index: i32) -> Result<u16> {
        match get_as(&array, index, b'C')? { Prim::C(v) => Ok(v), _ => Err(type_mismatch()) }
    }

    #[jvm_native]
    pub fn getShort(array: Object, index: i32) -> Result<i16> {
        match get_as(&array, index, b'S')? { Prim::S(v) => Ok(v), _ => Err(type_mismatch()) }
    }

    #[jvm_native]
    pub fn getInt(array: Object, index: i32) -> Result<i32> {
        match get_as(&array, index, b'I')? { Prim::I(v) => Ok(v), _ => Err(type_mismatch()) }
    }

    #[jvm_native]
    pub fn getLong(array: Object, index: i32) -> Result<i64> {
        match get_as(&array, index, b'J')? { Prim::J(v) => Ok(v), _ => Err(type_mismatch()) }
    }

    #[jvm_native]
    pub fn getFloat(array: Object, index: i32) -> Result<f32> {
        match get_as(&array, index, b'F')? { Prim::F(v) => Ok(v), _ => Err(type_mismatch()) }
    }

    #[jvm_native]
    pub fn getDouble(array: Object, index: i32) -> Result<f64> {
        match get_as(&array, index, b'D')? { Prim::D(v) => Ok(v), _ => Err(type_mismatch()) }
    }

    #[jvm_native]
    pub fn setBoolean(array: Object, index: i32, z: bool) -> Result<()> {
        set_as(&array, index, Prim::Z(z))
    }

    #[jvm_native]
    pub fn setByte(array: Object, index: i32, b: i8) -> Result<()> {
        set_as(&array, index, Prim::B(b))
    }

    #[jvm_native]
    pub fn setChar(array: Object, index: i32, c: u16) -> Result<()> {
        set_as(&array, index, Prim::C(c))
    }

    #[jvm_native]
    pub fn setShort(array: Object, index: i32, s: i16) -> Result<()> {
        set_as(&array, index, Prim::S(s))
    }

    #[jvm_native]
    pub fn setInt(array: Object, index: i32, i: i32) -> Result<()> {
        set_as(&array, index, Prim::I(i))
    }

    #[jvm_native]
    pub fn setLong(array: Object, index: i32, l: i64) -> Result<()> {
        set_as(&array, index, Prim::J(l))
    }

    #[jvm_native]
    pub fn setFloat(array: Object, index: i32, f: f32) -> Result<()> {
        set_as(&array, index, Prim::F(f))
    }

    #[jvm_native]
    pub fn setDouble(array: Object, index: i32, d: f64) -> Result<()> {
        set_as(&array, index, Prim::D(d))
    }
}
