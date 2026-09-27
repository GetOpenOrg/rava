//! `sun/invoke/util/Wrapper` 手写伴生：内部边界类（JDK 的「基本类型分类」
//! 枚举——LambdaForm / MethodTypeForm 的 basic type 体系），按调用链按需
//! 实现，其余保持 panic 存根。
//!
//! 枚举常量以线程内单例承载（JDK 枚举的 identity 语义：`w == Wrapper.INT`
//! 的比较在翻译层是 Object 恒等比较，单例保证命中）。字段按 JDK 实值填充
//! （basicTypeChar / primitiveType / wrapperType / simple 名）；format 等
//! 未消费字段以默认值承载。

use crate::prelude::*;
use super::wrapper::Wrapper;
use crate::java::lang::Class;
use crate::sync_model::__RefSlot as RefCell;

/// 常量表条目：(getter 名, basicTypeChar, primitive 名, wrapper binary name,
/// wrapperSimpleName, primitiveSimpleName)。JDK VALUES 顺序。
const CONSTANTS: &[(&str, u16, &str, &str, &str, &str)] = &[
    ("BOOLEAN", b'Z' as u16, "boolean", "java/lang/Boolean", "Boolean", "boolean"),
    ("BYTE",    b'B' as u16, "byte",    "java/lang/Byte",    "Byte",    "byte"),
    ("SHORT",   b'S' as u16, "short",   "java/lang/Short",   "Short",   "short"),
    ("CHAR",    b'C' as u16, "char",    "java/lang/Character", "Character", "char"),
    ("INT",     b'I' as u16, "int",     "java/lang/Integer", "Integer", "int"),
    ("LONG",    b'J' as u16, "long",    "java/lang/Long",    "Long",    "long"),
    ("FLOAT",   b'F' as u16, "float",   "java/lang/Float",   "Float",   "float"),
    ("DOUBLE",  b'D' as u16, "double",  "java/lang/Double",  "Double",  "double"),
    ("OBJECT",  b'L' as u16, "java.lang.Object", "java/lang/Object", "Object", "Object"),
    ("VOID",    b'V' as u16, "void",    "java/lang/Void",    "Void",    "void"),
];

crate::__process_static! {
    /// 常量单例池（与 CONSTANTS 同序）。
    static SINGLETONS: RefCell<Vec<Wrapper>> = const { RefCell::new(Vec::new()) };
}

/// 第 idx 个常量单例（惰性构建，恒同实例——clone 的是 Rc 包装）。
fn _constant(idx: usize) -> Wrapper {
    SINGLETONS.with(|s| {
        if s.borrow().len() < CONSTANTS.len() {
            let mut v = s.borrow_mut();
            for (ordinal, (getter, ch, prim, wrap, wsn, psn)) in CONSTANTS.iter().enumerate() {
                let mut w = Wrapper::default();
                w._init_not_null();
                // java/lang/Enum 的 name / ordinal（JDK 枚举常量构造器 super(name, ordinal)）：
                // switch-on-enum 的 $SwitchMap 以 ordinal() 为下标、toString 返回 name——缺省 0/null
                // 会把全部常量归到同一分支（ValueConversions.unbox → "unbox null"）
                w.__set_name(String::from(*getter));
                w.__set_ordinal(ordinal as i32);
                w.__set_basicTypeChar(*ch);
                w.__set_basicTypeString(String::from(
                    char::from_u32(*ch as u32).unwrap_or('?').to_string().as_str()));
                w.__set_primitiveType(Class::getPrimitiveClass(String::from(*prim))
                    .unwrap_or_else(|_| Class::for_class(String::from(*prim))));
                w.__set_wrapperType(Class::for_class(String::from(*wrap)));
                w.__set_wrapperSimpleName(String::from(*wsn));
                w.__set_primitiveSimpleName(String::from(*psn));
                v.push(w);
            }
        }
        Clone::clone(&s.borrow()[idx])
    })
}

/// 常量 getter 名 → 下标（生成本体的 pub fn 名与 JDK 常量名一致）。
fn _idx_of(getter: &str) -> Option<usize> {
    CONSTANTS.iter().position(|(g, ..)| *g == getter)
}

/// 装箱值的基本类型字符（原生值盒 / 翻译包装类两形态；非包装 → None）。
fn _box_char_of(x: &Object) -> Option<u8> {
    if x.0.is_jvm_null() {
        return None;
    }
    let any = x.0.as_any();
    if any.is::<i32>() { return Some(b'I'); }
    if any.is::<i64>() { return Some(b'J'); }
    if any.is::<f32>() { return Some(b'F'); }
    if any.is::<f64>() { return Some(b'D'); }
    if any.is::<i16>() { return Some(b'S'); }
    if any.is::<i8>() { return Some(b'B'); }
    if any.is::<u16>() { return Some(b'C'); }
    if any.is::<bool>() { return Some(b'Z'); }
    let name = x.0.__class_name();
    CONSTANTS[..8].iter().find(|(_, _, _, wrap, ..)| *wrap == name).map(|(_, c, ..)| *c as u8)
}

/// basicTypeChar → 常量下标（VALUES 序位）。
fn _idx_of_char(ch: u16) -> Option<usize> {
    CONSTANTS.iter().position(|(_, c, ..)| *c == ch)
}

/// 类型名（getPrimitiveClass 的点形态 / for_class 的点形态）→ 常量下标。
/// 覆盖 primitive 名与 wrapper binary 名两形态（JDK forPrimitiveType /
/// forWrapperType 共用的归一查询面）。
fn _idx_by_type_name(name: &str) -> Option<usize> {
    CONSTANTS.iter().position(|(_, _, prim, wrap, ..)| name == *prim || name == wrap.replace('/', "."))
}

impl Wrapper {
    pub fn BOOLEAN() -> Result<Wrapper> { Ok(_constant(0)) }
    pub fn BYTE() -> Result<Wrapper> { Ok(_constant(1)) }
    pub fn SHORT() -> Result<Wrapper> { Ok(_constant(2)) }
    pub fn CHAR() -> Result<Wrapper> { Ok(_constant(3)) }
    pub fn INT() -> Result<Wrapper> { Ok(_constant(4)) }
    pub fn LONG() -> Result<Wrapper> { Ok(_constant(5)) }
    pub fn FLOAT() -> Result<Wrapper> { Ok(_constant(6)) }
    pub fn DOUBLE() -> Result<Wrapper> { Ok(_constant(7)) }
    pub fn OBJECT() -> Result<Wrapper> { Ok(_constant(8)) }
    pub fn VOID() -> Result<Wrapper> { Ok(_constant(9)) }

    /// `values()`：常量序列（JDK VALUES 顺序）。
    pub fn values() -> Result<JArray<Wrapper>> {
        Ok(JArray::from((0..CONSTANTS.len()).map(_constant).collect::<Vec<_>>()))
    }

    /// static `forPrimitiveType(Class)`：基本类型的 Class → 常量；非基本
    /// 类型（含 wrapper、引用类型）按 JDK 抛 IllegalArgumentException。
    pub fn forPrimitiveType_class(type_: Class) -> Result<Wrapper> {
        let name = format!("{}", type_.__get_name());
        match _idx_by_type_name(&name).filter(|&i| {
            // 只认 primitive 形态（OBJECT 的 primitive 形态是 java.lang.Object）
            name == CONSTANTS[i].2 || (i == 8 && name == "java.lang.Object")
        }) {
            Some(i) => Ok(_constant(i)),
            None => Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                String::from(format!("not a primitive type: {}", name).as_str()))?)),
        }
    }

    /// static `forPrimitiveType(Class)` 的 JDK25 名面：JDK25 删去 `forPrimitiveType(char)`
    /// 重载后改编名不再带 `_class` 后缀——同一实现。
    pub fn forPrimitiveType(type_: Class) -> Result<Wrapper> {
        Self::forPrimitiveType_class(type_)
    }

    /// static `forWrapperType(Class)`：包装类的 Class → 常量；其余抛 IAE。
    pub fn forWrapperType(type_: Class) -> Result<Wrapper> {
        let name = format!("{}", type_.__get_name());
        match _idx_by_type_name(&name).filter(|&i| name == CONSTANTS[i].3.replace('/', ".")) {
            Some(i) => Ok(_constant(i)),
            None => Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                String::from(format!("not a wrapper type: {}", name).as_str()))?)),
        }
    }

    /// static `forBasicType(char)`：basicTypeChar → 常量；未知字符抛 IAE。
    pub fn forBasicType_c(type_: u16) -> Result<Wrapper> {
        match CONSTANTS.iter().position(|&(_, ch, ..)| ch == type_) {
            Some(i) => Ok(_constant(i)),
            None => Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                String::from(format!("not a basic type char: {}", type_).as_str()))?)),
        }
    }

    /// static `forBasicType(Class)`：类型的 basic 分类（primitive → 自身；
    /// wrapper → 其 primitive；引用 → OBJECT；void → VOID）。
    pub fn forBasicType_class(type_: Class) -> Result<Wrapper> {
        let name = format!("{}", type_.__get_name());
        if let Some(i) = _idx_by_type_name(&name) {
            return Ok(_constant(i));
        }
        // 其余引用类型统一 OBJECT（basic type 体系的 L 形态）
        Ok(_constant(8))
    }

    /// 实例 `primitiveType()`。
    pub fn primitiveType(&self) -> Result<Class> {
        Ok(Clone::clone(&self.__get_primitiveType()))
    }

    /// 实例 `wrapperType()`：包装类（OBJECT → Object、VOID → Void）。消费方：
    /// MethodType.canConvert（asType 的可转换判定）。
    pub fn wrapperType(&self) -> Result<Class> {
        Ok(Clone::clone(&self.__get_wrapperType()))
    }

    /// 实例 `isConvertibleFrom(Wrapper)`：source → this 的基本类型可转换性（JDK Wrapper 的
    /// Format 位判定逐条等价）：同一常量 → true；序位（VALUES 顺序）this < source → false；
    /// 两者均为 SIGNED 格式（byte/short/int/long/float/double）→ true（加宽）；否则 this 为
    /// OTHER（Object / void）或 source 为 char → true，其余 false。
    pub fn isConvertibleFrom(&self, source: Wrapper) -> Result<bool> {
        let (Some(t), Some(s)) = (_idx_of_char(self.__get_basicTypeChar()),
                                  _idx_of_char(source.__get_basicTypeChar())) else {
            return Ok(false);
        };
        if t == s {
            return Ok(true);
        }
        if t < s {
            return Ok(false);
        }
        let signed = |i: usize| matches!(CONSTANTS[i].1 as u8, b'B' | b'S' | b'I' | b'J' | b'F' | b'D');
        if signed(t) && signed(s) {
            return Ok(true);
        }
        let other = |i: usize| matches!(CONSTANTS[i].1 as u8, b'L' | b'V');
        Ok(other(t) || CONSTANTS[s].1 == b'C' as u16)
    }

    /// 实例 `wrapperSimpleName()`：包装类简单名（ValueConversions 按 "unbox" + 名查找转换方法）。
    pub fn wrapperSimpleName(&self) -> Result<String> {
        Ok(Clone::clone(&self.__get_wrapperSimpleName()))
    }

    /// 实例 `primitiveSimpleName()`：基本类型简单名（OBJECT → "Object"）。
    pub fn primitiveSimpleName(&self) -> Result<String> {
        Ok(Clone::clone(&self.__get_primitiveSimpleName()))
    }

    /// 实例 `basicTypeString()`：basicTypeChar 的单字符串形态。
    pub fn basicTypeString(&self) -> Result<String> {
        Ok(Clone::clone(&self.__get_basicTypeString()))
    }

    /// 实例 `zero()`：该类型的零值装箱（JDK Wrapper.zero 的包装对象；OBJECT / VOID → null）。
    pub fn zero(&self) -> Result<Object> {
        Ok(match self.__get_basicTypeChar() as u8 {
            b'Z' => Object::from(false),
            b'B' => Object::from(0i8),
            b'S' => Object::from(0i16),
            b'C' => Object::from(0u16),
            b'I' => Object::from(0i32),
            b'J' => Object::from(0i64),
            b'F' => Object::from(0f32),
            b'D' => Object::from(0f64),
            _ => Object::default(),
        })
    }

    /// 实例 `convert(Object, Class<T>)`：`convert(x, type, true)`（宽松转换：null → 零值）。
    #[jvm_boundary(upcalls = "java/lang/Class.cast:(Ljava/lang/Object;)Ljava/lang/Object;")]
    pub fn convert_obj_class(&self, x: Object, type_: Class) -> Result<Object> {
        self.convert_obj_class_z(x, type_, true)
    }

    /// 实例 `cast(Object, Class<T>)`：`convert(x, type, false)`（严格：源包装须可转换，否则 CCE）。
    #[jvm_boundary(upcalls = "java/lang/Class.cast:(Ljava/lang/Object;)Ljava/lang/Object;")]
    pub fn cast(&self, x: Object, type_: Class) -> Result<Object> {
        self.convert_obj_class_z(x, type_, false)
    }

    /// 私有 `convert(Object, Class<T>, boolean isCast)`，与 JDK 逐条等价：
    /// OBJECT → type.cast（接口不检查）后原样返回；x 已是本包装类 → 原样；
    /// !isCast 时源值的包装须 isConvertibleFrom，否则 ClassCastException；isCast 且 x 为
    /// null → zero()；其余经 `wrap`：数值化（Number / Character → int / Boolean → 0|1）后按
    /// 本类型窄化 / 拓宽装箱（JLS §5.1.2 / §5.1.3 的 Java 转换语义）。
    #[jvm_boundary(upcalls = "java/lang/Class.cast:(Ljava/lang/Object;)Ljava/lang/Object;")]
    pub fn convert_obj_class_z(&self, x: Object, type_: Class, is_cast: bool) -> Result<Object> {
        let tc = self.__get_basicTypeChar() as u8;
        if tc == b'L' {
            if !type_.isInterface()? {
                type_.cast(Clone::clone(&x))?;
            }
            return Ok(x);
        }
        let wrap_bin = CONSTANTS[_idx_of_char(self.__get_basicTypeChar()).unwrap_or(8)].3;
        if !x.0.is_jvm_null() && _box_char_of(&x) == Some(tc) {
            return Ok(x);
        }
        if !is_cast {
            let convertible = match _box_char_of(&x).and_then(|c| _idx_of_char(c as u16)) {
                Some(si) => self.isConvertibleFrom(_constant(si))?,
                None => false,
            };
            if !convertible {
                return Err(JvmError::class_cast(format!(
                    "Cannot cast {} to {}",
                    if x.0.is_jvm_null() { "null".to_owned() } else { x.0.__class_name().replace('/', ".") },
                    wrap_bin.replace('/', "."))));
            }
        } else if x.0.is_jvm_null() {
            return self.zero();
        }
        self.wrap_obj(x)
    }

    /// `wrap(Object)`（重载名 wrap_obj）：数值化后按本类型装箱（'L' 原样、'V' → null）。
    pub fn wrap_obj(&self, x: Object) -> Result<Object> {
        use crate::reflect_dispatch as rd;
        let tc = self.__get_basicTypeChar() as u8;
        match tc {
            b'L' => return Ok(x),
            b'V' => return Ok(Object::default()),
            _ => {}
        }
        // numberValue：浮点源保留 f64，其余整型化为 i64（Character → 码元、Boolean → 0/1）
        let (fv, iv, floating) = if let Some(b) = rd::unbox_bool(&x) {
            (0.0, b as i64, false)
        } else if let Some(c) = rd::unbox_char(&x) {
            (0.0, c as i64, false)
        } else if matches!(_box_char_of(&x), Some(b'F') | Some(b'D')) {
            (rd::unbox_f64(&x).unwrap_or(0.0), 0, true)
        } else if let Some(v) = rd::unbox_i64(&x) {
            (0.0, v, false)
        } else {
            return Err(JvmError::class_cast(format!(
                "Cannot cast {} to java.lang.Number", x.0.__class_name().replace('/', "."))));
        };
        let int_value = || -> i32 { if floating { fv as i32 } else { iv as i32 } };
        Ok(match tc {
            b'I' => Object::from(int_value()),
            b'J' => Object::from(if floating { fv as i64 } else { iv }),
            b'F' => Object::from(if floating { fv as f32 } else { iv as f32 }),
            b'D' => Object::from(if floating { fv } else { iv as f64 }),
            b'S' => Object::from(int_value() as i16),
            b'B' => Object::from(int_value() as i8),
            b'C' => Object::from(int_value() as u16),
            b'Z' => Object::from(((int_value() as i8) & 1) != 0),
            _ => Object::default(),
        })
    }

    /// 实例 `basicTypeChar()`。
    pub fn basicTypeChar(&self) -> Result<u16> {
        Ok(self.__get_basicTypeChar())
    }

    /// 实例 `isSubwordOrInt()`：int 与窄于 int 的整型（byte/short/char/boolean）
    /// 在 MH 体系里共享 int 的栈槽形态。
    pub fn isSubwordOrInt(&self) -> Result<bool> {
        Ok(matches!(self.__get_basicTypeChar(),
            c if c == b'I' as u16 || c == b'S' as u16 || c == b'B' as u16
                || c == b'C' as u16 || c == b'Z' as u16))
    }

    /// 实例 `isDoubleWord()`：long/double 占两个栈槽。
    pub fn isDoubleWord(&self) -> Result<bool> {
        Ok(matches!(self.__get_basicTypeChar(), c if c == b'J' as u16 || c == b'D' as u16))
    }

    /// 实例 `isSingleWord()`：非 long/double 即单槽（isDoubleWord 的否定）。
    pub fn isSingleWord(&self) -> Result<bool> {
        Ok(!self.isDoubleWord()?)
    }

    /// static `asPrimitiveType(Class)`：wrapper → primitive；其余原样。
    pub fn asPrimitiveType(type_: Class) -> Result<Class> {
        let name = format!("{}", type_.__get_name());
        match _idx_by_type_name(&name).filter(|&i| name == CONSTANTS[i].3.replace('/', ".")) {
            Some(i) => Ok(Class::getPrimitiveClass(String::from(CONSTANTS[i].2))?),
            None => Ok(type_),
        }
    }

    /// static `asWrapperType(Class)`：primitive → wrapper；其余原样。
    pub fn asWrapperType(type_: Class) -> Result<Class> {
        let name = format!("{}", type_.__get_name());
        match _idx_by_type_name(&name).filter(|&i| name == CONSTANTS[i].2 && i != 8) {
            Some(i) => Ok(Class::for_class(String::from(CONSTANTS[i].3))),
            None => Ok(type_),
        }
    }

    /// static `isWrapperType(Class)`：是否包装类。
    pub fn isWrapperType(type_: Class) -> Result<bool> {
        let name = format!("{}", type_.__get_name());
        Ok(CONSTANTS.iter().any(|&(_, _, _, wrap, ..)| name == wrap.replace('/', ".")))
    }

    /// static `basicTypeChar(Class)`：类型 → basic 字符（basic 分类的字符面）。
    pub fn basicTypeChar_class(type_: Class) -> Result<u16> {
        Ok(Wrapper::forBasicType_class(type_)?.__get_basicTypeChar())
    }
}
