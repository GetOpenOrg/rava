//! 手写层三清单（runtime/java_runtime/{closure,seeds,vm_intrinsics}.toml）的读取与域判定。
//!
//! 发射层的清单读取见 `input::manifest`（同一数据源、同一语义）；库知识（类名）只出现在清单里。

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// 类所属的分析域
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Domain {
    /// 用户类：字节码翻译
    User,
    /// JDK 类（VM 契约类之外）：字节码翻译
    Translate,
    /// VM 契约边界类（closure.toml [vm_boundary]）：手写 + 按方法字节码
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
    /// 字段偏移形参：调用点上该实参是符号偏移（见 `NameResolver::offset`）时只写入所指字段
    pub offset: Option<usize>,
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

/// 成员链接路径（成员声明类初始化点按路径接收声明类，`[facts.reflect] *_owner_initializers`）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LinkRoute {
    /// 方法句柄 / VarHandle（DirectMethodHandle、VarHandles 的链接）
    Handle,
    /// 核心反射（Method.invoke / Constructor.newInstance / Field 访问器工厂）
    Reflect,
}

mod sysprops;
mod empty;
pub use empty::EmptyCollections;
mod names;
mod concrete;
mod field_names;
pub use concrete::ConcreteCfg;
pub use field_names::NameResolver;
mod indy_helpers;
mod keyed;
pub use keyed::{KeyedLookup, KeyedLookups};
mod vm_state;
mod boot_phases;
pub use boot_phases::BootPhase;
pub use vm_state::{FieldHook, LoaderMapSrc, VmState};
pub use indy_helpers::IndyHelpers;
pub use names::{NameFacts, ValueMaps};
pub use sysprops::{PropRead, PropValue, PropWrite, SysProps};

/// 方法返回值事实（[vm_constants] / [facts]）
#[derive(Debug, Clone, PartialEq)]
pub enum Fact {
    Null,
    Int(i32),
}

/// 字符 / 字符串纯函数（[facts.string_ops]）：接收者与实参都是常量时结果即常量
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrOp {
    /// 忽略大小写相等（实参 null 为 false）
    EqualsIgnoreCase,
    /// UTF-16 长度
    Length,
    IsEmpty,
    /// `s[0]*31^(n-1) + … + s[n-1]`（UTF-16 码元，int 回绕）；字符串 switch 的分派键
    HashCode,
    /// 下标处的 UTF-16 码元（越界不折叠：运行期抛异常）
    CharAt,
    /// 字符转小写（仅 ASCII 实参折叠；其余取决于 Unicode 数据表，不折叠）
    CharToLowerCase,
    /// 字符串转小写（`toLowerCase()` / `toLowerCase(Locale)`，不看语言实参）：仅接收者为 ASCII 且不含 `I` 时折叠——
    /// 此时各语言结果相同（语言相关的规则只涉及 `I` 与非 ASCII 字符：tr / az 的 `I` → `ı`，lt 的带附加符号的 `I` / `J` / `Į`）
    ToLowerCase,
}

pub struct Manifest {
    pub runtime_dir: PathBuf,
    vm_boundary: HashSet<String>,
    /// `<clinit>` 由手写层承载的 VM 边界类（`[vm_boundary] clinit_carried`，含嵌套类）；其余 VM 边界类的
    /// `<clinit>` 按字节码翻译
    vm_clinit_carried: HashSet<String>,
    /// VM 边界类中按字节码翻译的嵌套类（`[vm_boundary] translate_nested`）与分析期追加的放行条目
    release: Vec<String>,
    /// VM 注入的静态字段（`[vm_constants.injected_statics]` 的键 `类.字段`）：运行期值由 VM 给出；
    /// 取值是整数 / 布尔字面量时记其值（分析器按该值折叠）
    injected_statics: HashMap<String, Option<i64>>,
    /// 模拟删除共置手写的放行条目（`rava closure --release-bytecode`）：前缀内按精确名提供的手写不再取手写
    hw_dropped: Vec<String>,
    intrinsics: HashSet<String>,
    null_to_false: HashSet<String>,
    returns: HashMap<String, Fact>,
    receiver_returns: HashSet<String>,
    field_enumerators: HashSet<String>,
    serial_enumerators: HashSet<String>,
    static_offset_getters: Vec<String>,
    field_handle_writers: HashSet<String>,
    field_handle_bridges: HashSet<String>,
    field_name_resolvers: HashMap<String, NameResolver>,
    deserializers: HashSet<String>,
    serializable_markers: Vec<String>,
    array_writes: HashMap<String, ArrayWrite>,
    /// 方法句柄解释器（`[facts.handle_interpreters]`）：其手写体调用点上的字段写入成员只写 DMH 所指字段
    handle_interpreters: Vec<String>,
    memory_reads: HashMap<String, (usize, Option<usize>)>,
    array_returns: HashMap<String, Vec<String>>,
    mirror_returns: HashSet<String>,
    superclass_returns: HashSet<String>,
    declaring_returns: HashSet<String>,
    primitive_class_returns: HashSet<String>,
    /// `[facts.reflect.defined_classes]`：VM 承载的运行期类定义点 → 承载所定义类成员的 VM 支持类
    defined_class_returns: HashMap<String, String>,
    /// `[facts.reflect.serial_allocators]`：序列化构造器的生成点 → 分配目标 Class 形参序号（不含接收者）
    serial_allocators: HashMap<String, usize>,
    caller_class_returns: HashSet<String>,
    /// `[caller_sensitive] annotations`：标注此注解的方法是 @CallerSensitive（binary name）
    caller_sensitive: HashSet<String>,
    component_returns: HashSet<String>,
    member_enumerators: HashMap<String, Members>,
    member_invokers: HashMap<String, Vec<Members>>,
    method_lookups: HashSet<String>,
    constructor_lookups: HashSet<String>,
    class_initializers: HashSet<String>,
    mirror_subtype_tests: HashSet<String>,
    member_owner_initializers: HashMap<String, LinkRoute>,
    method_to_handle: HashSet<String>,
    pub boot_init: Vec<String>,
    /// VM 启动期调用的静态方法（seeds.toml `[boot_init] calls`，`类.方法:描述符`）
    pub boot_calls: Vec<String>,
    /// VM 引导阶段（seeds.toml `[[boot_init.phases]]`，锚点可达时作根，见 `boot_phases.rs`）
    pub boot_phases: Vec<BootPhase>,
    /// seeds.toml 反射种子配置（注解 / locale / JCA / 纯数据束载体）
    pub seeds: crate::seeds::SeedCfg,
    indy: HashMap<String, IndyKind>,
    /// 拼接 / record ObjectMethods 调用点的分量处理入口（`[indy]`，见 `indy_helpers.rs`）
    pub indy_helpers: IndyHelpers,
    /// 按键查找入口（`[facts.keyed_lookups]`，见 `keyed.rs`）
    pub keyed_lookups: KeyedLookups,
    /// 基本类型描述符字符 → 装箱类（`[boxing]`；lambda 装箱 / 拆箱适配）
    boxing: HashMap<u8, String>,
    /// 按值比较的纯函数（接收者与实参都是常量时结果即常量）
    value_equals: HashSet<String>,
    /// 字符串纯函数
    string_ops: HashMap<String, StrOp>,
    /// VM 初始系统属性表与读写锚点
    pub sysprops: SysProps,
    /// 空的不可修改集合工厂与其上的查询结果（`[facts.empty_collections]`）
    pub empty: EmptyCollections,
    /// 按名取类与字符串拼接
    pub names: NameFacts,
    /// 具体求值（`[concrete]`）
    pub concrete: ConcreteCfg,
    /// VM 注入状态的落地（字段访问钩子、模块 → 加载器映射来源）
    pub vm_state: VmState,
}

const OBJECT: &str = "java/lang/Object";

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

        let release = strings(&closure, "vm_boundary", "translate_nested");

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

        let mut string_ops = HashMap::new();
        if let Some(t) = vm.get("facts").and_then(|s| s.get("string_ops")).and_then(|v| v.as_table()) {
            for (k, v) in t {
                let op = match v.as_str() {
                    Some("equals_ignore_case") => StrOp::EqualsIgnoreCase,
                    Some("length") => StrOp::Length,
                    Some("is_empty") => StrOp::IsEmpty,
                    Some("hash_code") => StrOp::HashCode,
                    Some("char_at") => StrOp::CharAt,
                    Some("char_to_lower_case") => StrOp::CharToLowerCase,
                    Some("to_lower_case") => StrOp::ToLowerCase,
                    _ => {
                        return Err(format!(
                            "vm_intrinsics.toml [facts.string_ops]：{k} 的值须为 equals_ignore_case / length / is_empty / hash_code / char_at / char_to_lower_case / to_lower_case"
                        ))
                    }
                };
                string_ops.insert(k.clone(), op);
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
                        offset: e.get("offset").and_then(|x| x.as_integer()).map(|x| x as usize),
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
                let offset = v.get("offset").and_then(|x| x.as_integer()).map(|x| x as usize);
                memory_reads.insert(k.clone(), (src as usize, offset));
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

        let mut defined_class_returns = HashMap::new();
        if let Some(t) = vm.get("facts").and_then(|s| s.get("reflect")).and_then(|s| s.get("defined_classes")).and_then(|v| v.as_table()) {
            for (k, v) in t {
                let Some(c) = v.as_str() else {
                    return Err(format!("vm_intrinsics.toml [facts.reflect.defined_classes]：{k} 须为 VM 支持类 binary name"));
                };
                defined_class_returns.insert(k.clone(), c.to_string());
            }
        }

        let mut serial_allocators = HashMap::new();
        if let Some(t) = vm.get("facts").and_then(|s| s.get("reflect")).and_then(|s| s.get("serial_allocators")).and_then(|v| v.as_table()) {
            for (k, v) in t {
                let Some(i) = v.as_integer().filter(|i| *i >= 0) else {
                    return Err(format!("vm_intrinsics.toml [facts.reflect.serial_allocators]：{k} 须为 Class 形参序号"));
                };
                serial_allocators.insert(k.clone(), i as usize);
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
            vm_boundary: strings(&closure, "vm_boundary", "classes").into_iter().collect(),
            vm_clinit_carried: strings(&closure, "vm_boundary", "clinit_carried").into_iter().collect(),
            release,
            injected_statics: vm
                .get("vm_constants")
                .and_then(|s| s.get("injected_statics"))
                .and_then(|v| v.as_table())
                .map(|t| t.iter().map(|(k, v)| (k.clone(), literal_value(v))).collect())
                .unwrap_or_default(),
            hw_dropped: Vec::new(),
            intrinsics,
            null_to_false: strings(&vm, "vm_constants", "null_to_false").into_iter().collect(),
            returns,
            receiver_returns: strings(&vm, "facts", "receiver_returns").into_iter().collect(),
            field_enumerators: field_writes("enumerators").into_iter().collect(),
            serial_enumerators: field_writes("serial_enumerators").into_iter().collect(),
            static_offset_getters: field_writes("static_offset_getters"),
            field_handle_writers: field_writes("handle_writers").into_iter().collect(),
            field_handle_bridges: field_writes("handle_bridges").into_iter().collect(),
            field_name_resolvers: field_names::parse(vm.get("facts").and_then(|s| s.get("field_writes")).and_then(|s| s.get("name_resolvers")))?,
            deserializers: field_writes("deserializers").into_iter().collect(),
            serializable_markers: field_writes("serializable_markers"),
            array_writes,
            handle_interpreters: facts("handle_interpreters", "members"),
            memory_reads,
            array_returns,
            mirror_returns: reflect("mirror_of_receiver").into_iter().collect(),
            superclass_returns: reflect("superclass_of_receiver").into_iter().collect(),
            declaring_returns: reflect("declaring_of_receiver").into_iter().collect(),
            primitive_class_returns: reflect("primitive_class").into_iter().collect(),
            defined_class_returns,
            serial_allocators,
            caller_class_returns: reflect("caller_class").into_iter().collect(),
            caller_sensitive: strings(&vm, "caller_sensitive", "annotations").into_iter().collect(),
            component_returns: reflect("component_of_receiver").into_iter().collect(),
            member_enumerators,
            member_invokers,
            method_lookups: reflect("method_lookups").into_iter().collect(),
            constructor_lookups: reflect("constructor_lookups").into_iter().collect(),
            class_initializers: reflect("class_initializers").into_iter().collect(),
            mirror_subtype_tests: reflect("mirror_subtype_tests").into_iter().collect(),
            member_owner_initializers: reflect("handle_owner_initializers")
                .into_iter()
                .map(|c| (c, LinkRoute::Handle))
                .chain(reflect("reflect_owner_initializers").into_iter().map(|c| (c, LinkRoute::Reflect)))
                .collect(),
            method_to_handle: reflect("method_to_handle").into_iter().collect(),
            boot_init: strings(&seeds, "boot_init", "classes"),
            boot_calls: strings(&seeds, "boot_init", "calls"),
            boot_phases: boot_phases::parse(&seeds)?,
            seeds: crate::seeds::SeedCfg::from_toml(&seeds),
            indy,
            indy_helpers: IndyHelpers::from_toml(
                vm.get("indy"),
                !strings(&vm, "indy", "concat").is_empty(),
                !strings(&vm, "indy", "object_methods").is_empty(),
            )?,
            keyed_lookups: KeyedLookups::from_toml(vm.get("facts").and_then(|s| s.get("keyed_lookups")))?,
            boxing,
            value_equals: strings(&vm, "facts", "value_equals").into_iter().collect(),
            string_ops,
            sysprops: SysProps::from_toml(vm.get("facts").and_then(|s| s.get("system_properties")))?,
            empty: EmptyCollections::from_toml(vm.get("facts").and_then(|s| s.get("empty_collections")))?,
            names: NameFacts::from_toml(vm.get("facts").and_then(|s| s.get("reflect")), vm.get("facts").and_then(|s| s.get("string_concat")))?,
            concrete: concrete::parse(vm.get("concrete"))?,
            vm_state: VmState::from_toml(&vm)?,
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
    /// 命令行追加的放行项（`--release`）与按字节码建模的手写项（`--release-bytecode`）：跨运行缓存键的一项
    pub fn cli_overrides(&self) -> (&[String], &[String]) {
        (&self.release, &self.hw_dropped)
    }

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
        if self.is_vm_boundary(cls) {
            Domain::Boundary
        } else {
            Domain::Translate
        }
    }

    /// VM 耦合边界类（`[vm_boundary]`，含嵌套类）
    pub fn is_vm_boundary(&self, cls: &str) -> bool {
        self.vm_boundary.contains(cls.split('$').next().unwrap_or(cls))
    }

    /// VM 边界类的 `<clinit>` 由手写层承载（`[vm_boundary] clinit_carried`，按最外层类匹配）
    pub fn is_vm_clinit_carried(&self, cls: &str) -> bool {
        self.vm_clinit_carried.contains(cls.split('$').next().unwrap_or(cls))
    }

    /// VM 注入的静态字段（`[vm_constants.injected_statics]`）：值不来自字节码，读取不折叠
    pub fn is_injected_static(&self, owner: &str, name: &str) -> bool {
        !self.injected_statics.is_empty() && self.injected_statics.contains_key(&format!("{owner}.{name}"))
    }

    /// VM 注入的静态字段的字面量取值（取值为整数 / 布尔字面量时）：读取恒为该值
    pub fn injected_literal(&self, owner: &str, name: &str) -> Option<i64> {
        if self.injected_statics.is_empty() {
            return None;
        }
        self.injected_statics.get(&format!("{owner}.{name}")).copied().flatten()
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

    /// 方法句柄解释器：手写体经 LambdaForm 调用的内存读写成员只作用于 DMH 所指字段（不读写数组元素）。
    /// 按成员引用逐项比对，不格式化（手写调用点增长热路径，清单只有几项）
    pub fn is_handle_interpreter(&self, key: &classfile::constant::MemberRef) -> bool {
        self.handle_interpreters.iter().any(|s| member_is(s, key))
    }

    /// 手写方法的返回值读自形参 src 所指对象（数组元素 / 引用字段）：返回该形参序号（按描述符，不含接收者）
    pub fn memory_read(&self, member: &str) -> Option<usize> {
        self.memory_reads.get(member).map(|x| x.0)
    }

    /// 读内存手写方法的字段偏移形参（序号不含接收者）：调用点上为符号偏移时只读所指字段
    pub fn memory_read_offset(&self, member: &str) -> Option<usize> {
        self.memory_reads.get(member).and_then(|x| x.1)
    }

    /// 手写方法返回新数组、VM 只写入所列类型的元素（`[facts.array_returns]`）：返回元素类型（binary name / 数组描述符）
    pub fn array_return(&self, member: &str) -> Option<&[String]> {
        self.array_returns.get(member).map(|v| v.as_slice())
    }

    /// 返回字段句柄数组的反射枚举（字段常量折叠的写入来源）
    pub fn is_field_enumerator(&self, member: &str) -> bool {
        self.field_enumerators.contains(member)
    }

    /// 只对可序列化类调用字段枚举、只取其可序列化字段（非 static、非 transient）的调用方：接收者推不出时
    /// 按可序列化字段口径放开，不按全部字段
    pub fn is_serial_enumerator(&self, member: &str) -> bool {
        self.serial_enumerators.contains(member)
    }

    /// 静态字段基址 / 偏移的取法：可达前类镜像不作静态字段基址按偏移读取
    /// 按成员引用逐项比对，不格式化（方法登记热路径，清单只有几项）
    pub fn is_static_offset_getter(&self, key: &classfile::constant::MemberRef) -> bool {
        self.static_offset_getters.iter().any(|s| member_is(s, key))
    }

    /// 按字段句柄写字段的入口（与字段枚举同时可达才放开被枚举的字段）
    /// 按成员引用逐项比对，不格式化（方法登记热路径，清单只有几项）
    pub fn is_field_handle_writer(&self, key: &classfile::constant::MemberRef) -> bool {
        self.field_handle_writers.iter().any(|s| member_is(s, key))
    }

    /// 字段句柄类型：字段枚举返回数组的分量类型、返回字段句柄（`handle = true`）的按名入口的返回类型
    pub fn field_handle_types(&self) -> Vec<String> {
        let ret = |s: &str| s.rsplit_once(')').map(|(_, r)| r.to_string()).unwrap_or_default();
        let obj = |r: &str| r.strip_prefix('L').and_then(|c| c.strip_suffix(';')).map(str::to_string);
        let mut out: Vec<String> = self.field_enumerators.iter().filter_map(|e| ret(e).strip_prefix('[').and_then(obj)).collect();
        out.extend(self.field_name_resolvers.iter().filter(|(_, r)| r.handle).filter_map(|(k, _)| obj(&ret(k))));
        out.sort();
        out.dedup();
        out
    }

    /// 句柄桥：在其内调用 handle_writers 不算写入入口（句柄只经 Field.set* 的访问器使用）
    pub fn is_field_handle_bridge(&self, member: &str) -> bool {
        self.field_handle_bridges.contains(member)
    }

    /// 按名取字段身份的入口（`[facts.field_writes.name_resolvers]`）
    pub fn field_name_resolver(&self, member: &str) -> Option<NameResolver> {
        self.field_name_resolvers.get(member).copied()
    }

    /// 按类镜像强制类初始化（`[facts.reflect] class_initializers`）：Class 实参所指类初始化
    pub fn is_class_initializer(&self, member: &str) -> bool {
        self.class_initializers.contains(member)
    }

    /// 类镜像子类型判定（`[facts.reflect] mirror_subtype_tests`）：接收者镜像所指类是实参镜像所指类的超类型时为真，
    /// 条件分支上按此收窄实参（absint/narrow.rs）
    pub fn is_mirror_subtype_test(&self, member: &str) -> bool {
        self.mirror_subtype_tests.contains(member)
    }

    /// 调用方的 Class 实参恒为「经该路径正被链接 / 访问的静态成员或构造器的声明类」（`[facts.reflect]
    /// handle_owner_initializers / reflect_owner_initializers`）：不按值集求目标，由「成员可达即声明类初始化」的
    /// 结构不变量覆盖（engine/mirror_init.rs）
    pub fn member_owner_route(&self, caller: &str) -> Option<LinkRoute> {
        self.member_owner_initializers.get(caller).copied()
    }

    /// 反序列化入口（可达即非 static、非 transient 字段不折叠）
    pub fn is_deserializer(&self, member: &str) -> bool {
        self.deserializers.contains(member)
    }

    /// 可序列化标记接口：反序列化只写实现者（声明类是其子类型）的字段；空 = 不区分（全部字段）
    pub fn serializable_markers(&self) -> &[String] {
        &self.serializable_markers
    }

    /// 返回接收者的类镜像（`Object.getClass` 语义）
    pub fn returns_mirror(&self, member: &str) -> bool {
        self.mirror_returns.contains(member)
    }

    /// 返回接收者镜像所指类的直接超类镜像（`Class.getSuperclass` 语义：接口 / 根类 / 基本类型为 null，数组为根类）
    pub fn returns_superclass(&self, member: &str) -> bool {
        self.superclass_returns.contains(member)
    }

    /// 返回接收者镜像所指类的声明类镜像（`[facts.reflect] declaring_of_receiver`，按 InnerClasses）
    pub fn returns_declaring_class(&self, member: &str) -> bool {
        self.declaring_returns.contains(member)
    }

    /// 返回基本类型（含 void）的类镜像（`Class.getPrimitiveClass` 语义）：所指类不是字节码类，无初始化、无成员
    pub fn returns_primitive_class(&self, member: &str) -> bool {
        self.primitive_class_returns.contains(member)
    }

    /// VM 承载的运行期类定义点所返回类的成员承载类（VM 支持类）：返回值即其类镜像
    pub fn defined_class(&self, member: &str) -> Option<&str> {
        self.defined_class_returns.get(member).map(String::as_str)
    }

    /// 返回调用它的 @CallerSensitive 方法的调用者类镜像（`Reflection.getCallerClass` 语义）
    pub fn returns_caller_class(&self, member: &str) -> bool {
        self.caller_class_returns.contains(member)
    }

    /// 注解（字段描述符形态 `Lx/Y;`）是否为 @CallerSensitive 注解
    pub fn is_caller_sensitive_annotation(&self, type_desc: &str) -> bool {
        let bin = type_desc.strip_prefix('L').and_then(|s| s.strip_suffix(';')).unwrap_or(type_desc);
        self.caller_sensitive.contains(bin)
    }

    /// 返回接收者镜像所指数组类的元素类型镜像（`getComponentType` 语义）
    pub fn returns_component_class(&self, member: &str) -> bool {
        self.component_returns.contains(member)
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

    /// 序列化构造器的生成点（`[facts.reflect] serial_allocators`）：返回分配目标的 Class 形参序号（不含接收者）
    pub fn serial_allocator(&self, member: &str) -> Option<usize> {
        self.serial_allocators.get(member).copied()
    }

    /// 查找构造器（Class 实参 / 接收者所指类的构造器成为反射构造目标）
    pub fn is_constructor_lookup(&self, member: &str) -> bool {
        self.constructor_lookups.contains(member)
    }

    /// 反射对象（Method）转成方法句柄（`[facts.reflect] method_to_handle`）
    pub fn is_method_to_handle(&self, member: &str) -> bool {
        self.method_to_handle.contains(member)
    }

    /// 方法反射调用入口：是否是方法句柄解释器以外、按反射对象调用的入口由描述符判定（见引擎 `reflect_call.rs`）
    pub fn method_invoker_keys(&self) -> impl Iterator<Item = &str> {
        self.member_invokers.iter().filter(|(_, ks)| ks.contains(&Members::Methods)).map(|(k, _)| k.as_str())
    }

    /// 纯函数：null 实参 → false
    pub fn is_value_equals(&self, member: &str) -> bool {
        self.value_equals.contains(member)
    }

    pub fn string_op(&self, member: &str) -> Option<StrOp> {
        self.string_ops.get(member).copied()
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

/// `s` 是否恰为 `key` 的「类.名:描述符」形式（与 `MemberRef` 的 Display 同式，免分配）
fn member_is(s: &str, key: &classfile::constant::MemberRef) -> bool {
    let rest = s.strip_prefix(key.owner.as_str()).and_then(|r| r.strip_prefix('.'));
    let rest = rest.and_then(|r| r.strip_prefix(key.name.as_str())).and_then(|r| r.strip_prefix(':'));
    rest == Some(key.desc.as_str())
}

#[cfg(test)]
mod tests;

/// 取值的字面量值：整数 / 布尔（取值表达式字符串不是字面量）
fn literal_value(v: &toml::Value) -> Option<i64> {
    match v {
        toml::Value::Integer(n) => Some(*n),
        toml::Value::Boolean(b) => Some(i64::from(*b)),
        _ => None,
    }
}
