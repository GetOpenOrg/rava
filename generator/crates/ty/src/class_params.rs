//! 类级泛型形参与外围作用域（`type_map.parse_class_type_params` /
//! `outer_instance_class` / `enclosing_method_info` / `effective_class_type_params`）。

use std::collections::BTreeSet;
use std::rc::Rc;

use classfile::Method;

use crate::consts::ACC_MANDATED;
use crate::registry::{method_signature, ClassInfo};
use crate::type_map::parse_descriptor_params;
use crate::TyCtx;

/// 基本类型描述符字符（含 `V`）
pub(crate) fn is_prim_char(c: u8) -> bool {
    matches!(
        c,
        b'V' | b'I' | b'J' | b'F' | b'D' | b'Z' | b'B' | b'C' | b'S'
    )
}

/// 跳过一个 FieldTypeSignature，返回其后位置
pub fn skip_field_type_sig(sig: &str, i: usize) -> usize {
    let b = sig.as_bytes();
    let Some(&c) = b.get(i) else {
        return i;
    };
    match c {
        _ if is_prim_char(c) => i + 1,
        b'T' => sig[i + 1..]
            .find(';')
            .map(|p| i + 1 + p + 1)
            .unwrap_or(b.len()),
        b'L' => {
            let mut depth = 0i32;
            let mut j = i + 1;
            while j < b.len() {
                match b[j] {
                    b'<' => depth += 1,
                    b'>' => depth -= 1,
                    b';' if depth == 0 => return j + 1,
                    _ => {}
                }
                j += 1;
            }
            j
        }
        b'[' | b'+' | b'-' => skip_field_type_sig(sig, i + 1),
        _ => i + 1,
    }
}

/// 类级（或方法级）签名的类型形参名表：`<K:..;V:..>..` → `[K, V]`
pub fn parse_class_type_params(sig: &str) -> Vec<String> {
    let b = sig.as_bytes();
    if b.first() != Some(&b'<') {
        return Vec::new();
    }
    let mut params = Vec::new();
    let mut i = 1;
    while i < b.len() && b[i] != b'>' {
        let mut j = i;
        while j < b.len() && b[j] != b':' && b[j] != b'>' {
            j += 1;
        }
        if j >= b.len() || b[j] == b'>' {
            break;
        }
        if j > i {
            params.push(sig[i..j].to_string());
        }
        i = j + 1;
        if i < b.len() && b[i] != b':' && b[i] != b'>' {
            i = skip_field_type_sig(sig, i);
        }
        while i < b.len() && b[i] == b':' {
            i += 1;
            if i < b.len() && b[i] != b':' && b[i] != b'>' {
                i = skip_field_type_sig(sig, i);
            }
        }
    }
    params
}

/// `this$N`（N 为十进制数字）
pub(crate) fn is_outer_this_field(name: &str) -> bool {
    name.strip_prefix("this$")
        .is_some_and(|d| !d.is_empty() && d.bytes().all(|c| c.is_ascii_digit()))
}

/// 内部类实例绑定的外部实例所属类；无外部实例 → 空串。
///
/// 依据：合成字段 `this$N` 的描述符；javac 省略 `this$N` 时，以 EnclosingMethod /
/// InnerClasses 记录的直接外围类核对构造器首个 ACC_MANDATED 形参。
pub fn outer_instance_class(ci: &ClassInfo) -> String {
    for f in ci.fields() {
        if !f.is_static() && is_outer_this_field(&f.name) {
            if let Some(inner) = f.desc.strip_prefix('L').and_then(|d| d.strip_suffix(';')) {
                if !inner.is_empty() && !inner.contains(';') {
                    return inner.to_string();
                }
            }
        }
    }
    let mut enclosing = ci.enclosing_class().to_string();
    if enclosing.is_empty() {
        if let Some(ic) = ci.inner_classes().iter().find(|ic| ic.inner == ci.name()) {
            enclosing = ic.outer.clone().unwrap_or_default();
        }
    }
    if !enclosing.is_empty() {
        let want = format!("L{enclosing};");
        for m in ci.methods() {
            if m.name != "<init>" || m.parameters.is_empty() {
                continue;
            }
            let params = parse_descriptor_params(&m.desc);
            if params.first() == Some(&want) && m.parameters[0].1 & ACC_MANDATED != 0 {
                return enclosing;
            }
        }
    }
    String::new()
}

impl<'a> TyCtx<'a> {
    /// 局部 / 匿名类的外围方法（EnclosingMethod）
    pub fn enclosing_method_info(&self, ci: &ClassInfo) -> Option<&'a Method> {
        let enclosing = ci.enclosing_class();
        let (name, desc) = ci.enclosing_method()?;
        if enclosing.is_empty() || self.reg.is_empty() {
            return None;
        }
        let outer = self.reg.get(enclosing)?;
        outer
            .methods()
            .iter()
            .find(|m| m.name == name && m.desc == desc)
    }

    /// 类在 Rust 侧的有效类型形参 = 外围作用域的类型变量（被自身同名形参遮蔽者除外）+ 自身形参
    pub fn effective_class_type_params(&self, ci: &ClassInfo) -> Rc<Vec<String>> {
        let cacheable = self.reg.get(ci.name()).is_some_and(|r| std::ptr::eq(r, ci));
        if cacheable {
            if let Some(hit) = self.reg.caches.effective_params.borrow().get(ci.name()) {
                return hit.clone();
            }
        }
        let mut visiting = BTreeSet::new();
        let result = Rc::new(self.effective_params_uncached(ci, &mut visiting));
        if cacheable {
            self.reg
                .caches
                .effective_params
                .borrow_mut()
                .insert(ci.name().to_string(), result.clone());
        }
        result
    }

    fn effective_params_uncached(
        &self,
        ci: &ClassInfo,
        visiting: &mut BTreeSet<String>,
    ) -> Vec<String> {
        let own = parse_class_type_params(ci.generic_signature());
        if self.reg.is_empty() {
            return own;
        }
        visiting.insert(ci.name().to_string());
        let is_local = !ci.enclosing_class().is_empty();
        let mut inherited: Vec<String> = Vec::new();
        let outer_bin = outer_instance_class(ci);
        if let Some(outer) = self.reg.get(&outer_bin) {
            // 外部类链成环时截断（Python 侧无此防护，合法字节码不成环）
            if !std::ptr::eq(outer, ci) && !visiting.contains(outer.name()) {
                inherited = self.effective_params_uncached(outer, visiting);
            }
        }
        if is_local {
            if let Some(em) = self.enclosing_method_info(ci) {
                let method_params = parse_class_type_params(method_signature(em));
                if !method_params.is_empty() {
                    inherited.retain(|p| !method_params.contains(p));
                    inherited.extend(method_params);
                }
            }
        }
        visiting.remove(ci.name());
        inherited.retain(|p| !own.contains(p));
        inherited.extend(own);
        inherited
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_type_params() {
        assert_eq!(
            parse_class_type_params("<E:Ljava/lang/Object;>Ljava/lang/Object;"),
            ["E"]
        );
        assert_eq!(
            parse_class_type_params("<K:Ljava/lang/Object;V:Ljava/lang/Object;>Lp/A<TK;>;"),
            ["K", "V"]
        );
        assert_eq!(
            parse_class_type_params("<T::Lp/I<TT;>;:Lp/J;>Ljava/lang/Object;"),
            ["T"]
        );
        assert!(parse_class_type_params("Ljava/lang/Object;").is_empty());
        assert!(parse_class_type_params("").is_empty());
    }

    #[test]
    fn skip_sig() {
        let s = "Lp/A<TK;>.B<[I>;X";
        assert_eq!(skip_field_type_sig(s, 0), s.len() - 1);
        assert_eq!(skip_field_type_sig("TT;I", 0), 3);
        assert_eq!(skip_field_type_sig("[[I", 0), 3);
    }
}
