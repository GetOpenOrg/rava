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
use std::cell::RefCell;

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

thread_local! {
    /// 常量单例池（与 CONSTANTS 同序）。
    static SINGLETONS: RefCell<Vec<Wrapper>> = const { RefCell::new(Vec::new()) };
}

/// 第 idx 个常量单例（惰性构建，恒同实例——clone 的是 Rc 包装）。
fn _constant(idx: usize) -> Wrapper {
    SINGLETONS.with(|s| {
        if s.borrow().len() < CONSTANTS.len() {
            let mut v = s.borrow_mut();
            for (_, ch, prim, wrap, wsn, psn) in CONSTANTS {
                let mut w = Wrapper::default();
                w._init_not_null();
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
