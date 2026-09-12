# JDK 字节码全量翻译实现计划

**日期**：2026-09-12  
**状态**：待实现  
**目标**：从 HelloWorld 出发，沿调用链自动翻译所有可达 JDK 字节码为 Rust，native 方法统一追踪，build.rs 自动维护类层级与实现状态。

---

## 目录

1. [架构总览](#1-架构总览)
2. [P0：JDK Class 解析器](#2-p0jdk-class-解析器)
3. [P1：生成带属性注解的 Rust 文件](#3-p1生成带属性注解的-rust-文件)
4. [P2：build.rs 自动处理](#4-p2buildrs-自动处理)
5. [P3：native_impls 目录与实现追踪](#5-p3native_impls-目录与实现追踪)
6. [P4：删除硬编码 JDK 分派](#6-p4删除硬编码-jdk-分派)
7. [数据流图](#7-数据流图)
8. [阶段里程碑](#8-阶段里程碑)

---

## 1. 架构总览

### 1.1 设计原则

1. **字节码是唯一数据源**：JDK 类的类型信息、方法签名、继承关系全部从 `.class` 文件读取，Python 代码中不维护任何 JDK 类的硬编码常量。
2. **生成代码自描述**：Rust 属性宏（`#[java_class(...)]`、`#[java_method(...)]` 等）把字节码元信息内嵌到生成的 `.rs` 文件中，代码即文档。
3. **build.rs 是自动化入口**：类层级验证、接口实现检查、native 方法追踪，全部在 `build.rs` 里扫描属性宏自动完成，不手工维护。
4. **native 方法是唯一手写点**：只有标记为 `ACC_NATIVE` 的方法才需要手写 Rust 实现；其余方法（含 JDK 内部实现）一律从字节码自动翻译。

### 1.2 目录结构（目标态）

```
java_rta/
├── scripts/
│   ├── main.py
│   └── codegen/
│       ├── classfile.py        # 扩展：输出完整元信息
│       ├── jdk_resolver.py     # 新增：定位并读取 JDK .class 文件
│       ├── emitter.py          # 修改：生成带 #[java_*] 属性的 .rs 文件
│       ├── instr.py            # 修改：删除所有 JDK 特殊分派
│       ├── type_map.py         # 修改：从 ClassFile 推导类型，删硬编码
│       └── ...
│
├── output/                     # 生成的 Cargo 项目
│   ├── Cargo.toml
│   ├── build.rs                # 新增：扫描属性、维护 native_status.toml
│   ├── native_status.toml      # 自动维护：native 方法实现状态
│   ├── native_impls/           # 手写 native Rust 实现
│   │   └── java/
│   │       ├── lang/
│   │       │   ├── object.rs
│   │       │   ├── system.rs
│   │       │   └── string.rs
│   │       └── io/
│   │           └── ...
│   └── src/
│       ├── main.rs
│       ├── hello_world.rs      # 用户代码（自动翻译）
│       └── java/               # JDK 代码（自动翻译）
│           ├── lang/
│           │   ├── mod.rs
│           │   ├── object.rs
│           │   ├── string.rs
│           │   └── string_builder.rs
│           └── util/
│               ├── mod.rs
│               ├── array_list.rs
│               └── ...
```

---

## 2. P0：JDK Class 解析器

### 2.1 目标

新增 `scripts/codegen/jdk_resolver.py`，给定 Java 类的 binary name（如 `java/util/ArrayList`），返回其 `.class` 文件的二进制内容。

### 2.2 JDK 类文件位置

| Java 版本 | 类文件位置 |
|-----------|-----------|
| Java 8    | `$JAVA_HOME/jre/lib/rt.jar` |
| Java 9-17 | `$JAVA_HOME/lib/modules`（jimage 格式）|
| Java 17+  | `$JAVA_HOME/jmods/java.base.jmod`（zip 格式）|

优先用 `javac --release` 编译，再用 `java -Xbootclasspath` 或直接读 jmod：

```python
# jdk_resolver.py

import os
import zipfile
import subprocess
from pathlib import Path
from functools import lru_cache


def _find_java_home() -> Path:
    """从环境变量或 java 命令定位 JAVA_HOME。"""
    if home := os.environ.get('JAVA_HOME'):
        return Path(home)
    result = subprocess.run(['java', '-XshowSettings:all', '-version'],
                            capture_output=True, text=True)
    for line in result.stderr.splitlines():
        if 'java.home' in line:
            return Path(line.split('=')[-1].strip())
    raise RuntimeError('Cannot locate JAVA_HOME')


class JdkResolver:
    """从 JDK 安装目录解析 .class 文件。"""

    def __init__(self, java_home: Path | None = None):
        self._home = java_home or _find_java_home()
        self._jmod_cache: dict[str, zipfile.ZipFile] = {}

    def resolve(self, binary_name: str) -> bytes | None:
        """返回 class 文件字节，找不到返回 None。
        binary_name 格式：java/util/ArrayList
        """
        class_path = binary_name + '.class'

        # Java 17+：从 jmod 文件读
        jmod = self._home / 'jmods' / 'java.base.jmod'
        if jmod.exists():
            return self._read_from_jmod(jmod, 'classes/' + class_path)

        # Java 9-16：javac 提取（fallback）
        return self._extract_via_javac(binary_name)

    def _read_from_jmod(self, jmod_path: Path, entry: str) -> bytes | None:
        key = str(jmod_path)
        if key not in self._jmod_cache:
            self._jmod_cache[key] = zipfile.ZipFile(jmod_path)
        zf = self._jmod_cache[key]
        try:
            return zf.read(entry)
        except KeyError:
            # 尝试其他模块（java.sql、java.xml 等）
            for jmod in self._home.glob('jmods/*.jmod'):
                if jmod == jmod_path:
                    continue
                zf2 = zipfile.ZipFile(jmod)
                try:
                    return zf2.read(entry)
                except KeyError:
                    continue
        return None

    def _extract_via_javac(self, binary_name: str) -> bytes | None:
        """通过 javac -verbose 定位 class 文件路径（兜底方案）。"""
        # 用临时 java 文件触发类加载，从输出中提取路径
        ...

    def is_jdk_class(self, binary_name: str) -> bool:
        """判断是否是 JDK 内置类（不是用户代码）。"""
        return any(binary_name.startswith(pkg) for pkg in (
            'java/', 'javax/', 'sun/', 'com/sun/', 'jdk/', 'org/ietf/',
        ))
```

### 2.3 classfile.py 扩展

在 `ParsedClass` 数据类中补充完整元信息：

```python
# types.py — ParsedClass 扩展

@dataclass
class ParsedField:
    name: str
    descriptor: str
    access_flags: int          # ACC_PUBLIC/STATIC/FINAL 等
    is_static: bool
    generic_signature: str = ''

@dataclass
class ParsedMethod:
    ...                        # 已有字段
    access_flags: int          # 新增
    is_native: bool            # 新增：ACC_NATIVE
    is_abstract: bool          # 新增：ACC_ABSTRACT
    exceptions: list[str]      # 新增：Exceptions 属性（throws 声明）
    generic_signature: str = ''  # 新增：方法泛型签名

@dataclass
class ParsedClass:
    class_name: str
    super_class: str           # 新增
    interfaces: list[str]      # 新增
    fields: list[ParsedField]
    methods: list[ParsedMethod]
    access_flags: int          # 新增
    generic_signature: str = ''  # 新增：类泛型签名（Signature 属性）
    source_file: str = ''      # 新增：SourceFile 属性
    is_interface: bool = False   # 新增：ACC_INTERFACE
    is_abstract: bool = False    # 新增：ACC_ABSTRACT
    is_enum: bool = False        # 新增：ACC_ENUM
```

`parse_class()` 已经读取了 constant pool，补充解析以下属性：
- 类级别：`super_class`（cp 引用）、`interfaces[]`（cp 引用数组）
- 类属性：`Signature`（泛型）、`SourceFile`
- 方法属性：`Exceptions`（throws 声明）、`Signature`（方法泛型）
- 方法访问标志：`ACC_NATIVE (0x0100)`、`ACC_ABSTRACT (0x0400)`

---

## 3. P1：生成带属性注解的 Rust 文件

### 3.1 属性宏格式定义

生成的 `.rs` 文件在结构体和方法声明上附加属性，作为元信息载体：

```rust
// 类属性
#[java_class(
    binary_name = "java/util/ArrayList",
    super_class  = "java/util/AbstractList",
    interfaces   = "java/util/List,java/util/RandomAccess,java/lang/Cloneable,java/io/Serializable",
    generic_sig  = "<E:Ljava/lang/Object;>Ljava/util/AbstractList<TE;>;Ljava/util/List<TE;>;...",
    access       = "public",
    source_file  = "ArrayList.java",
)]
pub struct ArrayList<E> { ... }

// 字段属性
#[java_field(name = "size", descriptor = "I", access = "private")]
pub size: Field<i32>,

// 普通方法属性
#[java_method(
    name       = "add",
    descriptor = "(Ljava/lang/Object;)Z",
    access     = "public",
    throws     = "",
)]
pub fn add(&self, e: E) -> Result<bool> { ... }

// native 方法属性
#[java_native(
    name       = "arraycopy",
    descriptor = "(Ljava/lang/Object;ILjava/lang/Object;II)V",
    access     = "public static native",
)]
pub fn arraycopy(...) -> Result<()> {
    todo!("native java/lang/System.arraycopy:(Ljava/lang/Object;ILjava/lang/Object;II)V")
}

// 接口翻译为 trait
#[java_interface(
    binary_name = "java/util/List",
    super_interfaces = "java/util/Collection",
    generic_sig  = "<E:Ljava/lang/Object;>...",
)]
pub trait List<E> {
    #[java_method(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public abstract")]
    fn add(&self, e: E) -> Result<bool>;
    ...
}
```

### 3.2 emitter.py 修改

在 `_gen_class_rs()` 中，把 `ParsedClass` 的所有元信息输出为属性字符串：

```python
def _java_class_attr(cls: ParsedClass) -> str:
    parts = [f'binary_name = "{cls.class_name}"']
    if cls.super_class and cls.super_class != 'java/lang/Object':
        parts.append(f'super_class = "{cls.super_class}"')
    if cls.interfaces:
        ifaces = ','.join(cls.interfaces)
        parts.append(f'interfaces = "{ifaces}"')
    if cls.generic_signature:
        parts.append(f'generic_sig = "{cls.generic_signature}"')
    if cls.source_file:
        parts.append(f'source_file = "{cls.source_file}"')
    parts.append(f'access = "{_access_str(cls.access_flags)}"')
    inner = ',\n    '.join(parts)
    return f'#[java_class(\n    {inner},\n)]'

def _java_method_attr(m: ParsedMethod) -> str:
    tag = 'java_native' if m.is_native else 'java_method'
    parts = [
        f'name = "{m.name}"',
        f'descriptor = "{m.descriptor}"',
        f'access = "{_access_str(m.access_flags)}"',
    ]
    if m.exceptions:
        parts.append(f'throws = "{",".join(m.exceptions)}"')
    inner = ', '.join(parts)
    return f'#[{tag}({inner})]'
```

### 3.3 native 方法处理

当 `m.is_native == True` 时，生成器**不翻译字节码**（因为没有），而是：

1. 检查 `native_impls/{class_path}.rs` 中是否存在对应实现
2. 有实现 → 在生成文件中用 `include!` 或 `mod` 引用
3. 无实现 → 生成 `todo!` 存根，并记录进待实现清单

```rust
// native 方法，无实现时生成的存根
#[java_native(name = "arraycopy", descriptor = "(...)", access = "public static native")]
pub fn arraycopy(src: JObject, src_pos: i32, dest: JObject, dest_pos: i32, length: i32) -> Result<()> {
    todo!("native java/lang/System.arraycopy:(Ljava/lang/Object;ILjava/lang/Object;II)V")
}
```

---

## 4. P2：build.rs 自动处理

### 4.1 build.rs 职责

```rust
// output/build.rs

fn main() {
    // 1. 扫描 src/ 提取所有 java_* 属性
    let registry = java_meta::scan_sources("src/");

    // 2. 验证接口实现完整性
    //    对每个 #[java_class(interfaces="...")] 声明的接口，
    //    检查对应 #[java_interface] trait 的所有方法是否都有实现
    registry.check_interface_impls().report();

    // 3. 扫描 native_impls/，与 #[java_native] 方法对照
    //    生成/更新 native_status.toml
    registry.update_native_status("native_impls/", "native_status.toml");

    // 4. （未来）生成接口调度表，用于虚方法分派
    // registry.emit_dispatch_table("src/java_runtime/dispatch.rs");

    // 重新触发构建的条件
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=native_impls/");
}
```

### 4.2 属性扫描实现

`build.rs` 不依赖 proc-macro；用简单的正则或 `syn` 解析源文件提取属性内容：

```rust
// output/build_support/java_meta.rs（被 build.rs 引用）

pub struct JavaMethodMeta {
    pub class: String,
    pub name: String,
    pub descriptor: String,
    pub is_native: bool,
    pub source_file: PathBuf,
}

pub struct ClassRegistry {
    pub classes: HashMap<String, ClassMeta>,   // binary_name → ClassMeta
    pub methods: Vec<JavaMethodMeta>,
}

impl ClassRegistry {
    /// 扫描 src/ 下所有 .rs 文件，提取 #[java_class] / #[java_native] 等属性
    pub fn scan_sources(src_dir: &str) -> Self {
        let mut registry = ClassRegistry::default();
        for entry in walkdir::WalkDir::new(src_dir) {
            let path = entry.path();
            if path.extension().map(|e| e == "rs").unwrap_or(false) {
                registry.parse_file(path);
            }
        }
        registry
    }

    fn parse_file(&mut self, path: &Path) {
        let content = std::fs::read_to_string(path).unwrap_or_default();
        // 提取 #[java_class(...)] 块
        // 提取 #[java_native(...)] 块
        // 用简单的正则匹配属性内容，不依赖完整 Rust 语法解析
        for cap in JAVA_CLASS_RE.captures_iter(&content) {
            let attrs = parse_attr_map(cap.get(1).unwrap().as_str());
            self.classes.insert(attrs["binary_name"].clone(), ClassMeta {
                binary_name: attrs["binary_name"].clone(),
                super_class: attrs.get("super_class").cloned().unwrap_or_default(),
                interfaces: attrs.get("interfaces")
                    .map(|s| s.split(',').map(str::to_owned).collect())
                    .unwrap_or_default(),
                ...
            });
        }
        for cap in JAVA_NATIVE_RE.captures_iter(&content) {
            // 同理提取 native 方法信息
        }
    }

    pub fn update_native_status(&self, impls_dir: &str, status_file: &str) {
        let mut status: HashMap<String, HashMap<String, String>> = load_toml(status_file);
        for m in &self.methods {
            if !m.is_native { continue; }
            let impl_file = format!("{}/{}.rs",
                impls_dir, m.class.to_lowercase().replace('/', "_"));
            let current = if Path::new(&impl_file).exists() {
                // 检查文件中是否有对应方法签名
                if impl_contains_method(&impl_file, &m.name, &m.descriptor) {
                    "implemented"
                } else {
                    "stub"
                }
            } else {
                "needed"
            };
            status.entry(m.class.replace('/', "."))
                  .or_default()
                  .entry(m.name.clone())
                  .and_modify(|v| {
                      // 不覆盖已有的 "not-needed" 标记
                      if v != "not-needed" { *v = current.to_owned(); }
                  })
                  .or_insert(current.to_owned());
        }
        write_toml(status_file, &status);
    }
}
```

---

## 5. P3：native_impls 目录与实现追踪

### 5.1 目录规范

```
native_impls/
  java/
    lang/
      object.rs      # java.lang.Object 的 native 方法
      system.rs      # java.lang.System 的 native 方法
      string.rs      # java.lang.String 的 native 方法
    io/
      file_output_stream.rs
```

每个文件内的函数签名必须与对应 `#[java_native]` 声明的签名一致，build.rs 靠此匹配。

### 5.2 native_impls 文件格式

```rust
// native_impls/java/lang/system.rs
// 手写 native 实现，可自由使用 Rust 标准库和第三方 crate

/// java/lang/System.arraycopy:(Ljava/lang/Object;ILjava/lang/Object;II)V
pub fn arraycopy(src: &[u8], src_pos: i32, dest: &mut [u8], dest_pos: i32, length: i32) {
    let s = src_pos as usize;
    let d = dest_pos as usize;
    let n = length as usize;
    dest[d..d+n].copy_from_slice(&src[s..s+n]);
}

/// java/lang/System.currentTimeMillis:()J
pub fn current_time_millis() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
```

### 5.3 native_status.toml 格式

```toml
# native_status.toml
# 由 build.rs 自动维护，手动修改会在下次构建时被覆盖（not-needed 除外）
# status 值：
#   "implemented" — native_impls/ 中有完整实现
#   "stub"        — 有文件但方法签名未匹配，语义不完整
#   "needed"      — HelloWorld 调用链触达，尚无实现，阻断编译
#   "not-needed"  — RTA 裁剪后不可达，暂不需要实现

[java.lang.Object]
hashCode             = "implemented"
getClass             = "stub"
clone                = "needed"
registerNatives      = "not-needed"

[java.lang.System]
arraycopy            = "needed"
currentTimeMillis    = "implemented"
nanoTime             = "not-needed"

[java.lang.String]
intern               = "not-needed"
charAt               = "implemented"
```

### 5.4 构建阻断策略

当存在 `status = "needed"` 的方法，且该方法在当前 RTA 可达集合中时，build.rs 输出错误并终止构建：

```rust
// build.rs
let blockers: Vec<_> = registry.native_methods()
    .filter(|m| m.is_reachable && m.status == "needed")
    .collect();

if !blockers.is_empty() {
    eprintln!("error: {} native method(s) need implementation:", blockers.len());
    for m in &blockers {
        eprintln!("  → {}.{}:{}", m.class, m.name, m.descriptor);
        eprintln!("    add impl to native_impls/{}.rs", m.class.replace('/', "/"));
    }
    std::process::exit(1);
}
```

---

## 6. P4：删除硬编码 JDK 分派

完成 P0–P3 后，以下代码可以全部删除：

### 6.1 `instr.py` 删除内容

```python
# 删除这些函数（每个 JDK 类一个分派函数的模式）
def _dispatch_list(sim, mname, obj, args): ...      # 删
def _dispatch_map(sim, mname, obj, args): ...       # 删
def _dispatch_set(sim, mname, obj, args): ...       # 删
def _gen_string_concat(sim, comment): ...           # 保留（invokedynamic 特有）

# _gen_invokevirtual 中删除所有 JDK 特殊分支
if obj_e in ('__stdout__', '__stderr__'): ...       # 删（JDK 字节码翻译后自然产生）
if cls in ('StringBuilder', 'StringBuffer'): ...   # 删
```

### 6.2 `type_map.py` 删除内容

```python
# 删除手工维护的 JDK 类型表
JDK_COLL_TYPES: dict = { 'ArrayList': ..., 'HashMap': ... }  # 删

# jvm_to_rust() 对 JDK 类的特殊分支改为：
# 从 ClassRegistry 查询已翻译类的 Rust 类型，找不到返回 JvmObject（占位）
def jvm_to_rust(t: str, registry=None) -> str:
    if t.startswith('L') and t.endswith(';'):
        binary_name = t[1:-1]
        if registry:
            if cls := registry.get(binary_name):
                return cls.rust_type  # 从已解析的 ParsedClass 获取
        return 'JvmObject'  # 未翻译的 JDK 类占位
    ...
```

### 6.3 `runtime.py` 删除内容

```python
# 删除手写的 Rust 存根字符串（这些内容由字节码翻译自动生成）
RUNTIME_FILES = {
    'java/util/array_list.rs': '...',  # 删，改为翻译 ArrayList.class 自动生成
    'java/lang/system.rs': '...',      # 删，native 方法由 native_impls/ 提供
    ...
}
```

---

## 7. 数据流图

```
HelloWorld.java
  │ javac -g
  ▼
HelloWorld.class
  │ RTA（java_rta.py）
  ▼
可达集合：{HelloWorld, java/lang/System, java/util/ArrayList, ...}
  │
  ├─ 用户类（HelloWorld）
  │    └─ classfile.py → parse_class()
  │         └─ emitter.py → hello_world.rs（带 #[java_class]）
  │
  └─ JDK 类（java/util/ArrayList 等）
       └─ jdk_resolver.py → .class 字节流
            └─ classfile.py → parse_class()
                 └─ emitter.py → java/util/array_list.rs（带 #[java_class]）
                      ├─ 普通方法：字节码 → Rust（同用户类流程）
                      └─ native 方法：生成 todo! 存根 + 记录

cargo build
  └─ build.rs
       ├─ 扫描 src/ 所有 .rs → 提取 #[java_class] / #[java_native]
       ├─ 扫描 native_impls/ → 对照已实现方法
       ├─ 更新 native_status.toml
       ├─ 检查接口实现完整性
       └─ 如有 "needed" native 方法 → 报错，列出需要手写的清单
```

---

## 8. 阶段里程碑

| 阶段 | 任务 | 完成标志 |
|------|------|---------|
| **P0** | 实现 `jdk_resolver.py`，能从 JDK jmod 读取 `java/lang/String.class` | `resolver.resolve('java/lang/String')` 返回非空字节 |
| **P0** | 扩展 `classfile.py`：解析 super_class、interfaces、Signature、Exceptions、ACC_NATIVE | `ParsedClass` 包含完整元信息 |
| **P1** | 修改 `emitter.py`：生成 `#[java_class]` / `#[java_method]` / `#[java_native]` 属性 | 生成的 `.rs` 文件包含完整 Java 元信息属性 |
| **P1** | native 方法生成 `todo!` 存根，并输出初始 `native_status.toml` | `native_status.toml` 自动生成 |
| **P2** | 实现 `build.rs` 属性扫描逻辑，自动更新 `native_status.toml` | 每次 `cargo build` 后 toml 自动同步 |
| **P2** | `build.rs` 检测 `needed` 方法，阻断构建并打印清单 | 有未实现 native 方法时 `cargo build` 报清晰错误 |
| **P3** | 手写 HelloWorld 链路最小 native 集合（`System.arraycopy`、`String` 基础方法等） | `cargo run` 输出正确 |
| **P4** | 删除 `instr.py` 中所有 JDK 特殊分派函数 | `instr.py` 无任何 JDK 类名常量 |
| **P4** | 删除 `type_map.py` 中 `JDK_COLL_TYPES` 等硬编码表 | `type_map.py` 无任何 JDK 类名 |
| **P4** | 删除 `runtime.py` 中手写 Rust 存根 | `runtime.py` 不再包含 Rust 代码字符串 |
