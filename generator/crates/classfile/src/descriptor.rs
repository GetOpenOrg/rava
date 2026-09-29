//! 字段 / 方法描述符（JVMS §4.3）。

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldType {
    /// B C D F I J S Z
    Prim(u8),
    Object(String),
    Array(Box<FieldType>),
}

impl FieldType {
    pub fn is_reference(&self) -> bool {
        !matches!(self, FieldType::Prim(_))
    }

    /// 类型里出现的类（数组取元素类）
    pub fn class_ref(&self) -> Option<&str> {
        match self {
            FieldType::Prim(_) => None,
            FieldType::Object(c) => Some(c),
            FieldType::Array(e) => e.class_ref(),
        }
    }

    /// 占用的局部变量槽数（long / double 为 2）
    pub fn slots(&self) -> u16 {
        match self {
            FieldType::Prim(b'J') | FieldType::Prim(b'D') => 2,
            _ => 1,
        }
    }

    pub fn descriptor(&self) -> String {
        match self {
            FieldType::Prim(c) => (*c as char).to_string(),
            FieldType::Object(c) => format!("L{c};"),
            FieldType::Array(e) => format!("[{}", e.descriptor()),
        }
    }
}

fn parse_at(s: &[u8], i: &mut usize) -> Option<FieldType> {
    let c = *s.get(*i)?;
    *i += 1;
    match c {
        b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' => Some(FieldType::Prim(c)),
        b'L' => {
            let start = *i;
            while *s.get(*i)? != b';' {
                *i += 1;
            }
            let name = std::str::from_utf8(&s[start..*i]).ok()?.to_string();
            *i += 1;
            Some(FieldType::Object(name))
        }
        b'[' => Some(FieldType::Array(Box::new(parse_at(s, i)?))),
        _ => None,
    }
}

pub fn parse_field(desc: &str) -> Option<FieldType> {
    let mut i = 0;
    let t = parse_at(desc.as_bytes(), &mut i)?;
    (i == desc.len()).then_some(t)
}

/// CONSTANT_Class 名字（binary name 或数组描述符）→ 类型
pub fn class_operand_type(name: &str) -> FieldType {
    if name.starts_with('[') {
        parse_field(name).unwrap_or_else(|| FieldType::Object(name.to_string()))
    } else {
        FieldType::Object(name.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodDesc {
    pub params: Vec<FieldType>,
    /// None = void
    pub ret: Option<FieldType>,
}

impl MethodDesc {
    pub fn param_slots(&self) -> u16 {
        self.params.iter().map(|p| p.slots()).sum()
    }
}

pub fn parse_method(desc: &str) -> Option<MethodDesc> {
    let s = desc.as_bytes();
    if s.first() != Some(&b'(') {
        return None;
    }
    let mut i = 1;
    let mut params = Vec::new();
    while *s.get(i)? != b')' {
        params.push(parse_at(s, &mut i)?);
    }
    i += 1;
    let ret = if s.get(i) == Some(&b'V') {
        i += 1;
        None
    } else {
        Some(parse_at(s, &mut i)?)
    };
    (i == s.len()).then_some(MethodDesc { params, ret })
}

/// 描述符里出现的全部类名（按出现序，含数组元素类）
pub fn class_refs(desc: &str) -> Vec<String> {
    let mut out = Vec::new();
    let s = desc.as_bytes();
    let mut i = 0;
    while i < s.len() {
        if s[i] == b'L' {
            let start = i + 1;
            let mut j = start;
            while j < s.len() && s[j] != b';' {
                j += 1;
            }
            out.push(String::from_utf8_lossy(&s[start..j]).into_owned());
            i = j + 1;
        } else {
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_desc() {
        let m = parse_method("(I[Ljava/lang/String;J)Ljava/util/List;").unwrap();
        assert_eq!(m.params.len(), 3);
        assert_eq!(m.param_slots(), 4);
        assert_eq!(m.ret.unwrap().class_ref(), Some("java/util/List"));
        assert!(parse_method("()V").unwrap().ret.is_none());
        assert_eq!(class_refs("([[Ljava/lang/Object;Ljava/lang/String;)V"), ["java/lang/Object", "java/lang/String"]);
    }
}
