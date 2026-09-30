//! 手写层三清单（runtime/java_runtime/{closure,seeds,vm_intrinsics}.toml）的读取与域判定。
//!
//! 与 `codegen/runtime_manifest.py` 同一数据源、同一语义；库知识（类名）只出现在清单里。

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// 类所属的分析域
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Domain {
    /// 用户类：字节码翻译
    User,
    /// 公开 API（java/、javax/）与放行条目：字节码翻译
    Translate,
    /// 内部边界 / VM 耦合边界 / 翻译域外：整体手写，BFS 截断
    Boundary,
    /// 根类（java/lang/Object）：手写 ObjectVTable
    Root,
}

/// invokedynamic 引导方法分类（vm_intrinsics.toml [indy]）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndyKind {
    Lambda,
    Concat,
    Native,
    /// record 的 equals / hashCode / toString（native 的细分）：各引用分量派发同名 Object 方法
    ObjectMethods,
}

/// 手写方法写入实参数组的元素（`[facts.array_writes]`；形参序号按描述符，不含接收者）
#[derive(Debug, Clone, Default)]
pub struct ArrayWrite {
    /// 被写入元素的数组形参；None = 不写任何实参数组
    pub dst: Option<usize>,
    /// 写入值取自这些形参的值
    pub values: Vec<usize>,
    /// 写入值取自这些形参数组的元素
    pub elements: Vec<usize>,
    /// 写入值含手写体产出
    pub produced: bool,
    /// dst 为对象（非数组）时写入其引用实例字段（Unsafe 按偏移写入）
    pub fields: bool,
    /// 写入值取自调用点最后一个实参（签名多态方法：实参个数随调用点变化）
    pub last: bool,
}

/// 反射成员对象所表示的成员类别（`[facts.reflect]`）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Members {
    /// 声明方法（不含构造器 / 类初始化）
    Methods,
    /// 声明构造器
    Constructors,
    /// record 组件的访问器方法
    RecordAccessors,
}

mod sysprops;
mod names;
pub use names::NameFacts;
pub use sysprops::{PropRead, PropValue, SysProps};

/// 方法返回值事实（[vm_constants] / [facts]）
#[derive(Debug, Clone, PartialEq)]
pub enum Fact {
    Null,
    Int(i32),
}

pub struct Manifest {
    pub runtime_dir: PathBuf,
    boundary_pkgs: Vec<String>,
    vm_boundary: HashSet<String>,
    release: Vec<String>,
    /// 模拟删除共置手写的放行条目（`rava closure --release-bytecode`）：前缀内按精确名提供的手写不再取手写
    hw_dropped: Vec<String>,
    intrinsics: HashSet<String>,
    null_to_false: HashSet<String>,
    returns: HashMap<String, Fact>,
    receiver_returns: HashSet<String>,
    field_enumerators: HashSet<String>,
    field_handle_writers: HashSet<String>,
    field_handle_bridges: HashSet<String>,
    deserializers: HashSet<String>,
    array_writes: HashMap<String, ArrayWrite>,
    memory_reads: HashMap<String, usize>,
    array_returns: HashMap<String, Vec<String>>,
    mirror_returns: HashSet<String>,
    member_enumerators: HashMap<String, Members>,
    member_invokers: HashMap<String, Vec<Members>>,
    method_lookups: HashSet<String>,
    pub boot_init: Vec<String>,
    /// seeds.toml 反射种子配置（注解 / locale / JCA / 纯数据束载体）
    pub seeds: crate::seeds::SeedCfg,
    indy: HashMap<String, IndyKind>,
    /// 基本类型描述符字符 → 装箱类（`[boxing]`；lambda 装箱 / 拆箱适配）
    boxing: HashMap<u8, String>,
    /// 按值比较的纯函数（接收者与实参都是常量时结果即常量）
    value_equals: HashSet<String>,
    /// VM 初始系统属性表与读写锚点
    pub sysprops: SysProps,
    /// 按名取类与字符串拼接
    pub names: NameFacts,
}

const OBJECT: &str = "java/lang/Object";
const PUBLIC_API: [&str; 2] = ["java/", "javax/"];

fn load(dir: &Path, name: &str) -> Result<toml::Table, String> {
    let p = dir.join(name);
    match std::fs::read_to_string(&p) {
        Ok(s) => s.parse::<toml::Table>().map_err(|e| format!("{}：{e}", p.display())),
        Err(_) => Ok(toml::Table::new()),
    }
}

fn strings(t: &toml::Table, sec: &str, key: &str) -> Vec<String> {
    t.get(sec)
        .and_then(|s| s.get(key))
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
        .unwrap_or_default()
}

impl Manifest {
    /// `runtime_dir` = runtime/java_runtime
    pub fn load(runtime_dir: &Path) -> Result<Self, String> {
        let closure = load(runtime_dir, "closure.toml")?;
        let seeds = load(runtime_dir, "seeds.toml")?;
        let vm = load(runtime_dir, "vm_intrinsics.toml")?;

        let mut release = strings(&closure, "release", "packages");
        release.extend(strings(&closure, "release", "classes"));
        release.extend(strings(&seeds, "jca", "release_packages"));
        release.extend(strings(&seeds, "jca", "release_classes"));

        let mut intrinsics = HashSet::new();
        if let Some(arr) = vm.get("intrinsic").and_then(|v| v.as_array()) {
            for e in arr {
                let member = e.get("member").and_then(|v| v.as_str()).unwrap_or_default();
                if e.get("kind").is_none() || e.get("reason").is_none() {
                    return Err(format!("vm_intrinsics.toml：内建条目须写明 kind 与 reason：{member}"));
                }
                intrinsics.insert(member.to_string());
            }
        }

        let mut returns = HashMap::new();
        for m in strings(&vm, "vm_constants", "null_returns") {
            returns.insert(m, Fact::Null);
        }
        if let Some(t) = vm.get("facts").and_then(|s| s.get("returns")).and_then(|v| v.as_table()) {
            for (k, v) in t {
                let f = match v {
                    toml::Value::Boolean(b) => Fact::Int(*b as i32),
                    toml::Value::Integer(i) => Fact::Int(*i as i32),
                    toml::Value::String(s) if s == "null" => Fact::Null,
                    _ => return Err(format!("vm_intrinsics.toml [facts] returns：{k} 的值须为 null / 整数 / 布尔")),
                };
                returns.insert(k.clone(), f);
            }
        }

        let facts = |sec: &str, key: &str| -> Vec<String> {
            vm.get("facts")
                .and_then(|s| s.get(sec))
                .and_then(|t| t.get(key))
                .and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
                .unwrap_or_default()
        };

        let mut array_writes = HashMap::new();
        if let Some(t) = vm.get("facts").and_then(|s| s.get("array_writes")).and_then(|v| v.as_table()) {
            let idx = |v: Option<&toml::Value>| -> Vec<usize> {
                v.and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|x| x.as_integer()).map(|x| x as usize).collect()).unwrap_or_default()
            };
            for (k, v) in t {
                let Some(e) = v.as_table() else {
                    return Err(format!("vm_intrinsics.toml [facts.array_writes]：{k} 的值须为表"));
                };
                array_writes.insert(
                    k.clone(),
                    ArrayWrite {
                        dst: e.get("dst").and_then(|x| x.as_integer()).map(|x| x as usize),
                        values: idx(e.get("values")),
                        elements: idx(e.get("elements")),
                        produced: e.get("produced").and_then(|x| x.as_bool()).unwrap_or(false),
                        fields: e.get("fields").and_then(|x| x.as_bool()).unwrap_or(false),
                        last: e.get("last").and_then(|x| x.as_bool()).unwrap_or(false),
                    },
                );
            }
        }

        let mut memory_reads = HashMap::new();
        if let Some(t) = vm.get("facts").and_then(|s| s.get("memory_reads")).and_then(|v| v.as_table()) {
            for (k, v) in t {
                let Some(src) = v.as_table().and_then(|e| e.get("src")).and_then(|x| x.as_integer()) else {
                    return Err(format!("vm_intrinsics.toml [facts.memory_reads]：{k} 须为 {{ src = 形参序号 }}"));
                };
                memory_reads.insert(k.clone(), src as usize);
            }
        }

        let mut array_returns = HashMap::new();
        if let Some(t) = vm.get("facts").and_then(|s| s.get("array_returns")).and_then(|v| v.as_table()) {
            for (k, v) in t {
                let elems: Option<Vec<String>> = v
                    .as_table()
                    .and_then(|e| e.get("elements"))
                    .and_then(|x| x.as_array())
                    .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect());
                match elems {
                    Some(es) if !es.is_empty() && k.contains(")[") => {
                        array_returns.insert(k.clone(), es);
                    }
                    _ => return Err(format!("vm_intrinsics.toml [facts.array_returns]：{k} 须为返回引用数组的方法，值为 {{ elements = [类型…] }}")),
                }
            }
        }

        let field_writes = |key: &str| facts("field_writes", key);
        let reflect = |key: &str| facts("reflect", key);
        let mut member_enumerators = HashMap::new();
        for (key, kind) in [("methods", Members::Methods), ("constructors", Members::Constructors), ("record_accessors", Members::RecordAccessors)] {
            for m in reflect(key) {
                member_enumerators.insert(m, kind);
            }
        }
        let mut member_invokers: HashMap<String, Vec<Members>> = HashMap::new();
        for (key, kind) in [("method_invokers", Members::Methods), ("constructor_invokers", Members::Constructors)] {
            for m in reflect(key) {
                member_invokers.entry(m).or_default().push(kind);
            }
        }

        let mut indy = HashMap::new();
        // 细分类别在后：同时列于 native 时取细分
        for (key, kind) in [
            ("lambda", IndyKind::Lambda),
            ("concat", IndyKind::Concat),
            ("native", IndyKind::Native),
            ("object_methods", IndyKind::ObjectMethods),
        ] {
            for m in strings(&vm, "indy", key) {
                indy.insert(m, kind);
            }
        }
        let boxing: HashMap<u8, String> = vm
            .get("boxing")
            .and_then(|s| s.as_table())
            .into_iter()
            .flatten()
            .filter_map(|(k, v)| match (k.as_bytes(), v.as_str()) {
                ([c], Some(cls)) => Some((*c, cls.to_string())),
                _ => None,
            })
            .collect();

        Ok(Manifest {
            runtime_dir: runtime_dir.to_path_buf(),
            boundary_pkgs: strings(&closure, "boundary", "packages"),
            vm_boundary: strings(&closure, "vm_boundary", "classes").into_iter().collect(),
            release,
            hw_dropped: Vec::new(),
            intrinsics,
            null_to_false: strings(&vm, "vm_constants", "null_to_false").into_iter().collect(),
            returns,
            receiver_returns: strings(&vm, "facts", "receiver_returns").into_iter().collect(),
            field_enumerators: field_writes("enumerators").into_iter().collect(),
            field_handle_writers: field_writes("handle_writers").into_iter().collect(),
            field_handle_bridges: field_writes("handle_bridges").into_iter().collect(),
            deserializers: field_writes("deserializers").into_iter().collect(),
            array_writes,
            memory_reads,
            array_returns,
            mirror_returns: reflect("mirror_of_receiver").into_iter().collect(),
            member_enumerators,
            member_invokers,
            method_lookups: reflect("method_lookups").into_iter().collect(),
            boot_init: strings(&seeds, "boot_init", "classes"),
            seeds: crate::seeds::SeedCfg::from_toml(&seeds),
            indy,
            boxing,
            value_equals: strings(&vm, "facts", "value_equals").into_iter().collect(),
            sysprops: SysProps::from_toml(vm.get("facts").and_then(|s| s.get("system_properties")))?,
            names: NameFacts::from_toml(vm.get("facts").and_then(|s| s.get("reflect")), vm.get("facts").and_then(|s| s.get("string_concat")))?,
        })
    }

    /// 分析期追加放行条目（`rava closure --release`，C1d 放行实测），不改清单文件
    pub fn release_more(&mut self, entries: impl IntoIterator<Item = String>) {
        self.release.extend(entries);
    }

    /// 放行并模拟删除前缀内的共置手写（`rava closure --release-bytecode`）：按精确名提供的手写方法改按字节码建模，
    /// native 与 VM 内建不受影响——测手写删除后的真实闭包增量
    pub fn release_bytecode(&mut self, entries: impl IntoIterator<Item = String>) {
        for e in entries {
            self.release.push(e.clone());
            self.hw_dropped.push(e);
        }
    }

    /// 放行条目：包前缀（`/` 结尾）或类（含 `$` 嵌套类）
    fn released(&self, cls: &str) -> bool {
        self.release.iter().any(|r| entry_matches(r, cls))
    }

    /// 该类的共置手写在分析期视为已删除（`--release-bytecode`）
    pub fn hw_dropped(&self, cls: &str) -> bool {
        self.hw_dropped.iter().any(|r| entry_matches(r, cls))
    }

    /// 类的分析域（`user` = 类来自用户输入）
    pub fn domain(&self, cls: &str, user: bool) -> Domain {
        if user {
            return Domain::User;
        }
        if cls == OBJECT {
            return Domain::Root;
        }
        if self.released(cls) {
            return Domain::Translate;
        }
        if self.boundary_pkgs.iter().any(|p| cls.starts_with(p.as_str())) {
            return Domain::Boundary;
        }
        let outer = cls.split('$').next().unwrap_or(cls);
        if self.vm_boundary.contains(outer) {
            return Domain::Boundary;
        }
        if PUBLIC_API.iter().any(|p| cls.starts_with(p)) {
            Domain::Translate
        } else {
            Domain::Boundary
        }
    }

    /// VM 耦合边界类（`[vm_boundary]`，含嵌套类）
    pub fn is_vm_boundary(&self, cls: &str) -> bool {
        self.vm_boundary.contains(cls.split('$').next().unwrap_or(cls))
    }

    /// VM 内建（手写承载、不分析 Java 体）
    pub fn is_intrinsic(&self, member: &str) -> bool {
        self.intrinsics.contains(member)
    }

    /// 方法返回值事实（`类.方法:描述符`）
    pub fn return_fact(&self, member: &str) -> Option<&Fact> {
        self.returns.get(member)
    }

    /// 返回值是接收者的浅拷贝（类型集 = 接收者类型集；数组共享元素节点）
    pub fn returns_receiver(&self, member: &str) -> bool {
        self.receiver_returns.contains(member)
    }

    /// 手写方法写入实参数组元素的声明（未声明 = 按手写体是否取得数组视图保守处理）
    pub fn array_writes(&self, member: &str) -> Option<&ArrayWrite> {
        self.array_writes.get(member)
    }

    /// 手写方法的返回值读自形参 src 所指对象（数组元素 / 引用字段）：返回该形参序号（按描述符，不含接收者）
    pub fn memory_read(&self, member: &str) -> Option<usize> {
        self.memory_reads.get(member).copied()
    }

    /// 手写方法返回新数组、VM 只写入所列类型的元素（`[facts.array_returns]`）：返回元素类型（binary name / 数组描述符）
    pub fn array_return(&self, member: &str) -> Option<&[String]> {
        self.array_returns.get(member).map(|v| v.as_slice())
    }

    /// 返回字段句柄数组的反射枚举（字段常量折叠的写入来源）
    pub fn is_field_enumerator(&self, member: &str) -> bool {
        self.field_enumerators.contains(member)
    }

    /// 按字段句柄写字段的入口（与字段枚举同时可达才放开被枚举的字段）
    /// 按成员引用逐项比对，不格式化（方法登记热路径，清单只有几项）
    pub fn is_field_handle_writer(&self, key: &classfile::constant::MemberRef) -> bool {
        self.field_handle_writers.iter().any(|s| member_is(s, key))
    }

    /// 句柄桥：在其内调用 handle_writers 不算写入入口（句柄只经 Field.set* 的访问器使用）
    pub fn is_field_handle_bridge(&self, member: &str) -> bool {
        self.field_handle_bridges.contains(member)
    }

    /// 反序列化入口（可达即非 static、非 transient 字段不折叠）
    pub fn is_deserializer(&self, member: &str) -> bool {
        self.deserializers.contains(member)
    }

    /// 返回接收者的类镜像（`Object.getClass` 语义）
    pub fn returns_mirror(&self, member: &str) -> bool {
        self.mirror_returns.contains(member)
    }

    /// 反射成员枚举：接收者类镜像所指类的哪类成员成为反射对象
    pub fn member_enumerator(&self, member: &str) -> Option<Members> {
        self.member_enumerators.get(member).copied()
    }

    /// 反射调用：调用哪类成员（Method / Constructor 对象所表示的成员）
    pub fn member_invoker(&self, member: &str) -> &[Members] {
        self.member_invokers.get(member).map_or(&[], |v| v.as_slice())
    }

    /// 按名查找方法（类 + 方法名常量点名反射目标）
    pub fn is_method_lookup(&self, member: &str) -> bool {
        self.method_lookups.contains(member)
    }

    /// 纯函数：null 实参 → false
    pub fn is_value_equals(&self, member: &str) -> bool {
        self.value_equals.contains(member)
    }

    pub fn is_null_to_false(&self, member: &str) -> bool {
        self.null_to_false.contains(member)
    }

    /// 引导方法（`类.方法`）的分类；未登记 = 按普通静态调用分析
    pub fn indy_kind(&self, bsm: &str) -> Option<IndyKind> {
        self.indy.get(bsm).copied()
    }

    /// 基本类型描述符字符的装箱类（`[boxing]`）
    pub fn boxed_class(&self, prim: u8) -> Option<&str> {
        self.boxing.get(&prim).map(String::as_str)
    }

    /// 类是装箱类时其基本类型描述符字符
    pub fn unboxed_prim(&self, cls: &str) -> Option<u8> {
        self.boxing.iter().find(|(_, c)| c.as_str() == cls).map(|(p, _)| *p)
    }
}

/// 清单条目匹配：包前缀（`/` 结尾）或类（含 `$` 嵌套类）
fn entry_matches(entry: &str, cls: &str) -> bool {
    if entry.ends_with('/') {
        cls.starts_with(entry)
    } else {
        cls == entry || cls.strip_prefix(entry).is_some_and(|rest| rest.starts_with('$'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_vm(vm: &str) -> Result<Manifest, String> {
        let dir = std::env::temp_dir().join(format!("rava-manifest-{}-{}", std::process::id(), vm.len()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("vm_intrinsics.toml"), vm).unwrap();
        let r = Manifest::load(&dir);
        std::fs::remove_dir_all(&dir).ok();
        r
    }

    #[test]
    fn array_returns_parse() {
        let m = with_vm("[facts.array_returns]\n\"a/B.f:()[Ljava/lang/Object;\" = { elements = [\"a/C\", \"a/D\"] }\n").unwrap();
        assert_eq!(m.array_return("a/B.f:()[Ljava/lang/Object;"), Some(&["a/C".to_string(), "a/D".to_string()][..]));
        assert_eq!(m.array_return("a/B.g:()[Ljava/lang/Object;"), None);
    }

    #[test]
    fn array_returns_reject_non_array() {
        assert!(with_vm("[facts.array_returns]\n\"a/B.f:()Ljava/lang/Object;\" = { elements = [\"a/C\"] }\n").is_err());
        assert!(with_vm("[facts.array_returns]\n\"a/B.f:()[Ljava/lang/Object;\" = { elements = [] }\n").is_err());
    }

    #[test]
    fn indy_object_methods_refines_native_and_boxing() {
        let m = with_vm("[indy]\nnative = [\"a/B.boot\", \"a/C.boot\"]\nobject_methods = [\"a/B.boot\"]\n[boxing]\nI = \"a/BoxI\"\n").unwrap();
        assert_eq!(m.indy_kind("a/B.boot"), Some(IndyKind::ObjectMethods));
        assert_eq!(m.indy_kind("a/C.boot"), Some(IndyKind::Native));
        assert_eq!(m.boxed_class(b'I'), Some("a/BoxI"));
        assert_eq!(m.unboxed_prim("a/BoxI"), Some(b'I'));
        assert_eq!(m.boxed_class(b'J'), None);
    }
}

/// `s` 是否恰为 `key` 的「类.名:描述符」形式（与 `MemberRef` 的 Display 同式，免分配）
fn member_is(s: &str, key: &classfile::constant::MemberRef) -> bool {
    let rest = s.strip_prefix(key.owner.as_str()).and_then(|r| r.strip_prefix('.'));
    let rest = rest.and_then(|r| r.strip_prefix(key.name.as_str())).and_then(|r| r.strip_prefix(':'));
    rest == Some(key.desc.as_str())
}
