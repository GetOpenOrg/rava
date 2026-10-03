//! 按名取类与字符串拼接的清单事实（`[facts.reflect]` 的 `class_lookups` / `instantiators` / `constant_tables`，
//! `value_maps`，`[facts.string_concat]`）。分析器据此把「常量前缀 + 常量表取值」拼出的类名解析成具体类（engine/class_lookup.rs）。

use std::collections::{BTreeMap, BTreeSet, HashSet};

#[derive(Debug, Default)]
pub struct NameFacts {
    /// 按名取类：静态方法，第 0 个实参是类的 binary name（`.` 分隔），返回该类的类镜像
    class_lookups: HashSet<String>,
    /// 按名加载类（不初始化）：静态方法，第 0 个实参是类的 binary name，返回该类的类镜像或 null
    class_loads: HashSet<String>,
    /// 手写类镜像构造 fn（`宿主类.fn 名`）：第 0 个实参是类的 binary name（`/` 分隔），返回该类的类镜像
    hw_mirrors: HashSet<String>,
    /// 实例化：接收者类镜像所指类的新实例（无参构造）
    instantiators: HashSet<String>,
    /// 常量表基类 → 读取入口（`名字:描述符`）：基类的具体子类是生成的常量表，内容即子类自身代码里的字符串常量
    tables: BTreeMap<String, BTreeSet<String>>,
    /// 字符串构建器：新建（构造器，可带初始内容实参）/ 追加（返回接收者本身）/ 取结果
    builders: HashSet<String>,
    appends: HashSet<String>,
    results: HashSet<String>,
    /// 构建器清空（实参为常量 0 时内容变空）：循环复用的构建器在追加前清空时拆段仍成立
    resets: HashSet<String>,
    /// 值映射：新建即空、写入入口按（键, 值）存入、读取入口只返回已存入的值或 null 的映射实现类
    value_maps: ValueMaps,
    /// 返回接收者镜像所指类的 binary name（`.` 分隔，数组为描述符形式）/ 简单名：拼接段可由镜像值集确定
    name_of: HashSet<String>,
    simple_name_of: HashSet<String>,
}

/// `[facts.reflect.value_maps]`：`classes` 映射实现类；`writers` / `readers` 为写入 / 读取入口（`名字:描述符`）
#[derive(Debug, Default)]
pub struct ValueMaps {
    pub classes: HashSet<String>,
    pub writers: HashSet<String>,
    pub readers: HashSet<String>,
}

fn list(t: Option<&toml::Value>, key: &str) -> Vec<String> {
    t.and_then(|s| s.get(key))
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
        .unwrap_or_default()
}

impl NameFacts {
    /// `reflect` = `[facts.reflect]`，`concat` = `[facts.string_concat]`
    pub fn from_toml(reflect: Option<&toml::Value>, concat: Option<&toml::Value>) -> Result<Self, String> {
        let mut tables = BTreeMap::new();
        for (base, v) in reflect.and_then(|r| r.get("constant_tables")).and_then(|v| v.as_table()).into_iter().flatten() {
            let readers: BTreeSet<String> = v
                .as_table()
                .and_then(|t| t.get("readers"))
                .and_then(|r| r.as_array())
                .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
                .unwrap_or_default();
            if readers.is_empty() {
                return Err(format!("vm_intrinsics.toml [facts.reflect.constant_tables]：{base} 须为 {{ readers = [\"名字:描述符\"…] }}"));
            }
            tables.insert(base.clone(), readers);
        }
        Ok(NameFacts {
            class_lookups: list(reflect, "class_lookups").into_iter().collect(),
            class_loads: list(reflect, "class_loads").into_iter().collect(),
            hw_mirrors: list(reflect, "hw_mirror_by_name").into_iter().collect(),
            instantiators: list(reflect, "instantiators").into_iter().collect(),
            tables,
            builders: list(concat, "builders").into_iter().collect(),
            appends: list(concat, "appends").into_iter().collect(),
            results: list(concat, "results").into_iter().collect(),
            resets: list(concat, "resets").into_iter().collect(),
            name_of: list(reflect, "name_of_receiver").into_iter().collect(),
            simple_name_of: list(reflect, "simple_name_of_receiver").into_iter().collect(),
            value_maps: {
                let vm = reflect.and_then(|r| r.get("value_maps"));
                ValueMaps {
                    classes: list(vm, "classes").into_iter().collect(),
                    writers: list(vm, "writers").into_iter().collect(),
                    readers: list(vm, "readers").into_iter().collect(),
                }
            },
        })
    }

    pub fn is_class_lookup(&self, member: &str) -> bool {
        self.class_lookups.contains(member)
    }

    pub fn is_class_load(&self, member: &str) -> bool {
        self.class_loads.contains(member)
    }

    /// 手写 fn `host.f` 是否按名构造类镜像
    pub fn is_hw_mirror(&self, host: &str, f: &str) -> bool {
        self.hw_mirrors.contains(&format!("{host}.{f}"))
    }

    pub fn is_instantiator(&self, member: &str) -> bool {
        self.instantiators.contains(member)
    }

    /// 读取入口 `名字:描述符` 所属的常量表基类
    pub fn table_bases<'s>(&'s self, sig: &'s str) -> impl Iterator<Item = &'s str> + 's {
        self.tables.iter().filter(move |(_, rs)| rs.contains(sig)).map(|(b, _)| b.as_str())
    }

    pub fn is_builder(&self, member: &str) -> bool {
        self.builders.contains(member)
    }

    pub fn is_append(&self, member: &str) -> bool {
        self.appends.contains(member)
    }

    pub fn is_result(&self, member: &str) -> bool {
        self.results.contains(member)
    }

    pub fn is_reset(&self, member: &str) -> bool {
        self.resets.contains(member)
    }

    /// 返回接收者镜像所指类的 binary name（`Class.getName` 语义）
    pub fn is_name_of(&self, member: &str) -> bool {
        self.name_of.contains(member)
    }

    /// 返回接收者镜像所指类的简单名（`Class.getSimpleName` 语义）
    pub fn is_simple_name_of(&self, member: &str) -> bool {
        self.simple_name_of.contains(member)
    }

    pub fn value_maps(&self) -> &ValueMaps {
        &self.value_maps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_facts() {
        let v: toml::Value = toml::from_str(
            r#"
            [r]
            class_lookups = ["a/C.byName:(Ljava/lang/String;)La/C;"]
            class_loads = ["a/L.load:(Ljava/lang/String;)Ljava/lang/Class;"]
            hw_mirror_by_name = ["a/K.for_class"]
            instantiators = ["a/C.make:()Ljava/lang/Object;"]
            name_of_receiver = ["a/K.name:()Ljava/lang/String;"]
            simple_name_of_receiver = ["a/K.simple:()Ljava/lang/String;"]
            [r.constant_tables."a/T"]
            readers = ["get:(Ljava/lang/Object;)Ljava/lang/Object;"]
            [r.value_maps]
            classes = ["a/M"]
            writers = ["put:(La/K;La/K;)La/K;"]
            readers = ["get:(La/K;)La/K;"]
            [s]
            builders = ["a/B.<init>:()V"]
            appends = ["a/B.add:(Ljava/lang/String;)La/B;"]
            results = ["a/B.str:()Ljava/lang/String;"]
            resets = ["a/B.clear:(I)V"]
            "#,
        )
        .unwrap();
        let f = NameFacts::from_toml(v.get("r"), v.get("s")).unwrap();
        assert!(f.is_class_lookup("a/C.byName:(Ljava/lang/String;)La/C;") && f.is_instantiator("a/C.make:()Ljava/lang/Object;"));
        assert_eq!(f.table_bases("get:(Ljava/lang/Object;)Ljava/lang/Object;").collect::<Vec<_>>(), vec!["a/T"]);
        assert!(f.is_builder("a/B.<init>:()V") && f.is_append("a/B.add:(Ljava/lang/String;)La/B;") && f.is_result("a/B.str:()Ljava/lang/String;"));
        assert!(f.is_reset("a/B.clear:(I)V"));
        assert!(f.is_class_load("a/L.load:(Ljava/lang/String;)Ljava/lang/Class;") && !f.is_class_load("a/C.byName:(Ljava/lang/String;)La/C;"));
        assert!(f.is_hw_mirror("a/K", "for_class") && !f.is_hw_mirror("a/K", "new"));
        assert!(f.is_name_of("a/K.name:()Ljava/lang/String;") && !f.is_name_of("a/K.simple:()Ljava/lang/String;"));
        assert!(f.is_simple_name_of("a/K.simple:()Ljava/lang/String;"));
        let vm = f.value_maps();
        assert!(vm.classes.contains("a/M") && vm.writers.len() == 1 && vm.readers.contains("get:(La/K;)La/K;"));
        let bad: toml::Value = toml::from_str("[r.constant_tables.\"a/T\"]\nx = 1\n").unwrap();
        assert!(NameFacts::from_toml(bad.get("r"), None).is_err());
    }
}
