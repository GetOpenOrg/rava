//! `sun/invoke/util/BytecodeDescriptor` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! 描述符 ↔ Class 的双向转换（MethodType.toMethodDescriptorString /
//! fromMethodDescriptorString 的底座；MH-native 的 MemberName 解析与调用点类型消费）。
//! Class 与描述符的映射唯一真源是手写 Class 层（`descriptorString` /
//! `__class_for_descriptor`），本类只做描述符文本的拼接与切分。

use crate::prelude::*;
use super::bytecode_descriptor::implref::BytecodeDescriptor;
use crate::java::lang::{Class, ClassLoader};
use crate::java::util::{ArrayList, List};

/// 方法描述符 `(P…)R` 的分量描述符表（形参在前，返回类型在末）；格式错误 → None。
fn split_method_descriptor(desc: &str) -> Option<Vec<std::string::String>> {
    let b = desc.as_bytes();
    if b.first() != Some(&b'(') {
        return None;
    }
    let mut out = Vec::new();
    let mut i = 1usize;
    let mut params_done = false;
    while i < b.len() {
        if b[i] == b')' {
            if params_done {
                return None;
            }
            params_done = true;
            i += 1;
            continue;
        }
        let start = i;
        while i < b.len() && b[i] == b'[' {
            i += 1;
        }
        match b.get(i)? {
            b'L' => {
                let semi = desc[i..].find(';')? + i;
                i = semi + 1;
            }
            b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' | b'V' => i += 1,
            _ => return None,
        }
        out.push(desc[start..i].to_owned());
    }
    if !params_done || out.is_empty() {
        return None;
    }
    Some(out)
}

impl BytecodeDescriptor {
    /// `unparse(Class)`：类型描述符（`I` / `Ljava/lang/String;` / `[J` …）。
    #[jvm_boundary]
    pub fn unparse_class(type_: Class) -> Result<String> {
        type_.descriptorString()
    }

    /// `unparse(Object)`：Class → 描述符；MethodType → 方法描述符；其余（String）原样。
    #[jvm_boundary]
    pub fn unparse_obj(type_: Object) -> Result<String> {
        if type_.0.is_instance_of("java/lang/Class") {
            return Clone::clone(&type_).try_cast::<Class>("java/lang/Class")?.descriptorString();
        }
        if type_.0.is_instance_of("java/lang/invoke/MethodType") {
            return Clone::clone(&type_)
                .try_cast::<crate::java::lang::invoke::MethodType>("java/lang/invoke/MethodType")?
                .toMethodDescriptorString();
        }
        String::valueOf_obj(type_)
    }

    /// `unparseMethod(Class rtype, Class[] ptypes)`：`(P…)R`。
    #[jvm_boundary]
    pub fn unparseMethod_class_arr_class(rtype: Class, ptypes: JArray<Class>) -> Result<String> {
        let mut s = std::string::String::from("(");
        for i in 0..ptypes.len()? {
            s.push_str(&format!("{}", ptypes.get(i)?.descriptorString()?));
        }
        s.push(')');
        s.push_str(&format!("{}", rtype.descriptorString()?));
        Ok(String::from(s.as_str()))
    }

    /// `parseMethod(String, ClassLoader)`：分量 Class 表（形参在前、返回类型在末），可变
    /// ArrayList——`MethodType.fromMethodDescriptorString` 对其 `remove(size-1)` 取返回类型。
    /// 格式错误 → IllegalArgumentException（JDK parseError 同型）。
    #[jvm_boundary(upcalls = "java/util/ArrayList.<init>:()V java/util/ArrayList.add:(Ljava/lang/Object;)Z")]
    pub fn parseMethod_str_classloader(bytecodeSignature: String, _loader: ClassLoader) -> Result<List<Object>> {
        let desc = format!("{}", bytecodeSignature);
        let Some(parts) = split_method_descriptor(&desc) else {
            return Err(JvmError::illegal_argument(&format!("bad signature: {}", desc)));
        };
        let list: ArrayList<Object> = ArrayList::new()?;
        for p in &parts {
            list.add_obj(Object::from(Class::__class_for_descriptor(p)))?;
        }
        Ok(List::from(list))
    }
}
