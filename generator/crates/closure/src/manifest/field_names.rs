//! 按名取字段身份的入口（`[facts.field_writes.name_resolvers]`）。字段常量折叠的写入来源之一：
//! 名字 → 字段偏移 / setter / VarHandle / 更新器 / 字段句柄 / 字段 MemberName。本表即按名取字段身份的全部入口，
//! 分析器（engine/field_names.rs）按名字值集放开字段，名字推不出时保守回退。

use std::collections::HashMap;

/// 一个按名取字段身份的入口
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NameResolver {
    /// 字段所属类的 Class 实参（形参序号，不含接收者）；None = 接收者本身是 Class
    pub class: Option<usize>,
    /// 字段名实参（形参序号，不含接收者）
    pub name: usize,
    /// 返回字段句柄（Field）：只有经 `handle_writers` 才能写字段，名字未知时按字段枚举处理（写入口可达才放开）；
    /// false = 直接取得写入能力（偏移 / setter / VarHandle / 更新器），名字未知时立即放开
    pub handle: bool,
    /// 返回字段偏移（long）：偏移即字段身份，分析器把常量类 + 常量名的调用折叠为符号偏移，
    /// 按偏移读写的手写调用点（`[facts.array_writes]` / `[facts.memory_reads]` 的 offset）据此只触及该字段
    pub offset: bool,
    /// 方法句柄引用种类实参（JVMS §5.4.3.5 reference_kind，形参序号，不含接收者）：实参是常量且属 `read_kinds`
    /// 时只取读取能力，不放开字段（名字也不配对上推）
    pub kind: Option<usize>,
    /// 只读的引用种类（位集：第 k 位 = reference_kind k）
    pub read_kinds: u16,
}

impl NameResolver {
    /// 调用点的引用种类实参（常量）只取读取能力
    pub fn read_only(&self, kind: Option<i64>) -> bool {
        kind.is_some_and(|k| (0..16).contains(&k) && self.read_kinds & (1 << k) != 0)
    }
}

pub fn parse(t: Option<&toml::Value>) -> Result<HashMap<String, NameResolver>, String> {
    let mut out = HashMap::new();
    for (k, v) in t.and_then(|v| v.as_table()).into_iter().flatten() {
        let e = v.as_table();
        let int = |key: &str| e.and_then(|e| e.get(key)).and_then(|x| x.as_integer());
        let class = match e.and_then(|e| e.get("class")) {
            Some(toml::Value::String(s)) if s == "receiver" => None,
            Some(toml::Value::Integer(i)) if *i >= 0 => Some(*i as usize),
            _ => return Err(format!("vm_intrinsics.toml [facts.field_writes.name_resolvers]：{k} 的 class 须为形参序号或 \"receiver\"")),
        };
        let Some(name) = int("name").filter(|i| *i >= 0) else {
            return Err(format!("vm_intrinsics.toml [facts.field_writes.name_resolvers]：{k} 缺 name（字段名形参序号）"));
        };
        let flag = |key: &str| e.and_then(|e| e.get(key)).and_then(|x| x.as_bool()).unwrap_or(false);
        let kind = int("kind").filter(|i| *i >= 0).map(|i| i as usize);
        let mut read_kinds = 0u16;
        for x in e.and_then(|e| e.get("read_kinds")).and_then(|x| x.as_array()).into_iter().flatten() {
            match x.as_integer() {
                Some(i) if (0..16).contains(&i) => read_kinds |= 1 << i,
                _ => return Err(format!("vm_intrinsics.toml [facts.field_writes.name_resolvers]：{k} 的 read_kinds 须为 0..16 的整数")),
            }
        }
        if read_kinds != 0 && kind.is_none() {
            return Err(format!("vm_intrinsics.toml [facts.field_writes.name_resolvers]：{k} 有 read_kinds 缺 kind（引用种类形参序号）"));
        }
        out.insert(k.clone(), NameResolver { class, name: name as usize, handle: flag("handle"), offset: flag("offset"), kind, read_kinds });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_receiver_and_index_forms() {
        let t: toml::Value = toml::from_str(
            r#"
"a/B.f:(Ljava/lang/String;)V" = { class = "receiver", name = 0, handle = true }
"a/B.g:(Ljava/lang/Class;Ljava/lang/String;)J" = { class = 0, name = 1, offset = true }
"#,
        )
        .unwrap();
        let m = parse(Some(&t)).unwrap();
        assert_eq!(m["a/B.f:(Ljava/lang/String;)V"], NameResolver { class: None, name: 0, handle: true, offset: false, kind: None, read_kinds: 0 });
        assert_eq!(
            m["a/B.g:(Ljava/lang/Class;Ljava/lang/String;)J"],
            NameResolver { class: Some(0), name: 1, handle: false, offset: true, kind: None, read_kinds: 0 }
        );
    }

    #[test]
    fn reference_kind_filters_read_only_sites() {
        let t: toml::Value =
            toml::from_str(r#""a/L.r:(BLjava/lang/Class;Ljava/lang/String;)V" = { kind = 0, class = 1, name = 2, read_kinds = [1, 2] }"#).unwrap();
        let r = parse(Some(&t)).unwrap()["a/L.r:(BLjava/lang/Class;Ljava/lang/String;)V"];
        assert!(r.read_only(Some(1)) && r.read_only(Some(2)));
        assert!(!r.read_only(Some(3)) && !r.read_only(Some(4)));
        // 引用种类推不出：按可写处理
        assert!(!r.read_only(None));
    }

    #[test]
    fn read_kinds_require_kind() {
        let t: toml::Value = toml::from_str(r#""a/L.r:()V" = { class = 0, name = 1, read_kinds = [1] }"#).unwrap();
        assert!(parse(Some(&t)).is_err());
    }

    #[test]
    fn rejects_missing_name() {
        let t: toml::Value = toml::from_str(r#""a/B.g:()V" = { class = 0 }"#).unwrap();
        assert!(parse(Some(&t)).is_err());
    }
}
