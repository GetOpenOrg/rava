//! `[indy]` 调用点分量处理入口：拼接 / record ObjectMethods 对引用实参（分量）的语义各对应一个 JDK 静态方法，
//! 生成器在调用点发射对它的调用（方法体由字节码翻译），分析器把实参值接入它的形参。
//!
//! - `concat_stringify`：字符串化（concat 的引用实参、record toString 的引用分量）；
//! - `component_hash`：record hashCode 的引用分量；
//! - `component_equals`：record equals 的引用分量（两个实参：本方分量、对方分量）。
//!
//! 清单是唯一真源：登记了 concat / object_methods 引导方法而缺对应入口时装载报错，不做回落。

/// 分量处理入口（`类.方法:描述符`）
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct IndyHelpers {
    pub stringify: Option<String>,
    pub hash: Option<String>,
    pub equals: Option<String>,
}

impl IndyHelpers {
    /// 读 `[indy]` 节；`has_concat` / `has_object_methods` = 清单登记了对应类别的引导方法
    pub fn from_toml(sec: Option<&toml::Value>, has_concat: bool, has_object_methods: bool) -> Result<Self, String> {
        let get = |key: &str, need: bool| -> Result<Option<String>, String> {
            match sec.and_then(|s| s.get(key)) {
                Some(v) => {
                    let s = v.as_str().filter(|s| well_formed(s));
                    s.map(|s| Some(s.to_string()))
                        .ok_or_else(|| format!("vm_intrinsics.toml [indy] {key} 应为 `类.方法:描述符`：{v}"))
                }
                None if need => Err(format!("vm_intrinsics.toml [indy] 缺 {key}（已登记的 concat / object_methods 引导方法需要它）")),
                None => Ok(None),
            }
        };
        Ok(IndyHelpers {
            stringify: get("concat_stringify", has_concat || has_object_methods)?,
            hash: get("component_hash", has_object_methods)?,
            equals: get("component_equals", has_object_methods)?,
        })
    }
}

fn well_formed(s: &str) -> bool {
    s.split_once(':').is_some_and(|(head, desc)| head.contains('.') && desc.starts_with('('))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sec(s: &str) -> toml::Value {
        toml::Value::Table(s.parse().unwrap())
    }

    #[test]
    fn required_when_kinds_registered() {
        let s = sec("concat_stringify = \"a/S.v:(La/O;)La/S;\"\n");
        assert!(IndyHelpers::from_toml(Some(&s), true, false).is_ok());
        assert!(IndyHelpers::from_toml(Some(&s), true, true).is_err());
        assert!(IndyHelpers::from_toml(None, true, false).is_err());
        assert_eq!(IndyHelpers::from_toml(None, false, false).unwrap(), IndyHelpers::default());
        assert!(IndyHelpers::from_toml(Some(&sec("concat_stringify = \"bad\"\n")), false, false).is_err());
    }
}
