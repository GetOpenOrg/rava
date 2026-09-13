# Java → Rust 转译器：任务执行清单

**日期**：2026-09-12  
**目标**：建立完整的 Java → Rust 转译流水线，支持 Java 25 字节码，生成结构化 Rust IR（非字符串拼接），通过四层漏斗裁剪 JDK 依赖。

**本地 JDK 环境**：
| JDK 版本 | class file version | 路径 |
|----------|-------------------|------|
| 11 | 55 | `/opt/homebrew/Cellar/openjdk@11/11.0.32.1/libexec/openjdk.jdk/Contents/Home` |
| 17 | 61 | `/opt/homebrew/Cellar/openjdk@17/17.0.20.1/libexec/openjdk.jdk/Contents/Home` |
| 21 | 65 | `/opt/homebrew/Cellar/openjdk@21/21.0.11/libexec/openjdk.jdk/Contents/Home` |
| 25 | 69 | `/opt/homebrew/Cellar/openjdk@25/25.0.4.1/libexec/openjdk.jdk/Contents/Home` |
| 26 | 70 | `/opt/homebrew/Cellar/openjdk/26.0.2.1/libexec/openjdk.jdk/Contents/Home` |

默认编译使用 JDK 21（已配置为系统默认），解析 JDK 类文件时按目标版本选择对应 jmods。

---

## 完成状态图例

- `[x]` 已完成
- `[ ]` 待执行
- `[~]` 进行中

---

## 阶段一：基础设施（Phase 1 — Infrastructure）

### T01 · 二进制 `.class` 文件解析器
**状态**：`[x]`  
**文件**：`scripts/codegen/classfile.py`（新建），替换 `scripts/codegen/javap.py`

**目标**：直接解析 `.class` 二进制格式（JVMS Chapter 4），不再依赖 `javap -verbose` 文本输出。

**实现要点**：
- 解析 magic、major/minor version（支持 v45～v69，即 Java 1～25）
- 完整常量池解析（Class、Utf8、Methodref、Fieldref、NameAndType、String、Integer、Long、Float、Double、InvokeDynamic 等）
- Access flags、this_class、super_class、interfaces
- Fields：name、descriptor、access flags
- Methods：name、descriptor、access flags + Code attribute（字节码 + exception table + LocalVariableTable）
- 关键 attributes：Code、ConstantValue、Exceptions、InnerClasses、Record（Java 16+）、PermittedSubclasses（Java 17+）
- 输出：与现有 `ClassInfo` / `ParsedMethod` / `Instr` 兼容的数据结构（或直接替换这些结构）

**验收**：`python3 -c "from codegen.classfile import parse_class; c = parse_class('tests/classes/TestP0.class'); print(c)"` 输出正确类信息。

---

### T02 · Rust IR 数据结构
**状态**：`[x]`  
**文件**：`scripts/codegen/rs_ir.py`（新建）  
**依赖**：无（独立设计）

**目标**：用 Python dataclass 定义 Rust 语法树节点，替代当前在 `stack.py` / `instr.py` 中直接 f-string 拼接字符串的方式。

**节点分类**：

```
# 类型
RsType: Primitive | Named | Ref | Slice | Generic | Tuple
  Primitive → i32 / i64 / f64 / bool / ()
  Named     → struct/enum 名，带可选 path（java_runtime::...）
  Ref       → &T 或 &mut T，可选 lifetime
  Slice     → &[T]
  Generic   → Rc<RefCell<T>>、Vec<T>、Option<T>

# 表达式
RsExpr:
  Lit       → 整数/浮点/bool/string 字面量
  Var       → 变量名引用
  BinOp     → l op r（+/-/*/% 等）
  UnOp      → -/! 等
  Call      → 函数调用（path + args）
  MethodCall→ recv.method(args)
  FieldAccess → recv.field
  Index     → recv[idx]
  Cast      → val as ty
  Ref       → &expr / &mut expr
  Block     → 内联块表达式 { stmts; tail }
  If        → if cond { then } [else { else_ }]（表达式形式）
  Macro     → println!/todo!/compile_error! 等

# 语句
RsStmt:
  Let       → let [mut] name[: ty] = val;
  Assign    → target = val;
  ExprStmt  → expr;
  Return    → return [val];
  LoopStmt  → loop { body }（含 break/continue）
  IfStmt    → if/else if/else 语句形式
  RawLine   → 逃生舱：直接插入一行原始字符串（过渡期使用）

# 条目（顶层）
RsItem:
  Fn        → [pub] [unsafe] fn name(params) [-> ret] { body }
  Struct    → [pub] struct Name { fields }
  Impl      → impl [Trait for] Type { items }
  Use       → use path::...;
  Mod       → mod name;
  TypeAlias → type Name = Type;
```

**验收**：能用 IR 节点手工构造一个 `pub fn add(a: i32, b: i32) -> i32 { let r = a + b; r }` 并 render 出正确文本。

---

### T03 · Rust IR 渲染器
**状态**：`[x]`  
**文件**：`scripts/codegen/render.py`（新建）  
**依赖**：T02

**目标**：`render(node: RsNode, indent: int = 0) -> str`，将 IR 树递归转换为格式化 Rust 文本。

**实现要点**：
- 缩进用 4 空格，块结构自动增减
- 表达式优先级括号（BinOp 嵌套时正确加括号）
- 函数体最后一个语句若为 `ExprStmt` 且是函数返回值，去掉分号（Rust 尾表达式语法）
- `RawLine` 节点直接原样输出（兼容过渡期）

**验收**：T02 的手工节点经 render 后与预期文本完全一致（含换行缩进）。

---

## 阶段二：代码生成重构（Phase 2 — Codegen Rewrite）

### T04 · StackSim 改为操作 Expr 节点
**状态**：`[x]`  
**文件**：`scripts/codegen/stack.py`（修改）  
**依赖**：T02、T03

**目标**：`StackSim` 的操作数栈改为存储 `RsExpr` 节点，`emit()` 改为追加 `RsStmt` 节点到当前块，`fresh()` 生成 `Let` 节点并返回 `Var` 节点。

**关键接口变更**：
```python
# 旧
sim.push("local_0")
sim.emit(f"let {v} = {a} + {b};")

# 新
sim.push(Var("local_0"))
sim.emit(Let(v, None, False, BinOp("+", a, b)))
```

**验收**：运行 `python3 scripts/main.py tests/TestP0.java` 仍输出正确 Rust 代码（行为等价，格式可不同）。

---

### T05 · instr.py 生成 IR 节点
**状态**：`[~]`（过渡：已接入 IR 基础设施，各指令仍通过 RawExpr/RawStmt 包装字符串；逐条迁移为真正 IR 节点是后续优化）  
**文件**：`scripts/codegen/instr.py`（修改）  
**依赖**：T04

**目标**：所有 `sim.emit(f"...")` 改为 `sim.emit(RsStmt节点)`，所有 `sim.push(f"...")` 改为 `sim.push(RsExpr节点)`。

**范围**：
- 算术指令（iadd/isub/imul/idiv/irem/ineg）→ `BinOp` / `UnOp`
- 比较 + 跳转 → `IfStmt` + `Break`/`Continue`
- invokestatic / invokevirtual / invokespecial → `Call` / `MethodCall`
- 字段访问（getfield/putfield）→ `FieldAccess` / `Assign`
- 数组操作（newarray/iaload/iastore/arraylength）→ IR 节点

**验收**：P0–P3 四个测试的 Rust 输出 cargo check 通过，Java 与 Rust 输出一致。

---

### T06 · method.py 生成 Fn/Impl IR
**状态**：`[~]`（过渡：已接入 render_stmt；方法体用 str 列表，后续迁移为 RsFn/RsImpl 节点）  
**文件**：`scripts/codegen/method.py`（修改）  
**依赖**：T05

**目标**：`gen_method_body()` 返回 `RsItem`（`Fn` 节点），外层 `gen_class()` 返回 `Impl` 节点，全部由 render 统一输出。

**验收**：同 T05 验收条件。

---

### T07 · emitter.py 使用 render 输出
**状态**：`[ ]`（暂缓：method.py 当前仍输出字符串，emitter.py 无需改动；待 T06 完全迁移后再推进）  
**文件**：`scripts/codegen/emitter.py`（修改）  
**依赖**：T06、T03

**目标**：`write_cargo_project()` 调用 `render()` 而不是直接字符串拼接生成 `.rs` 文件。

**验收**：全量回归（P0–P3）通过。

---

## 阶段三：流水线改进（Phase 3 — Pipeline）

### T08 · RTA 改进：`<clinit>` 隔离 + cutoff 截断
**状态**：`[x]`  
**文件**：`scripts/java_rta.py`（修改），新增 `config/cutoff.toml`

**目标**：
- `<clinit>` 方法只在类被 `new` 实例化时才跟随，不无条件追踪
- 支持 `cutoff.toml` 截断规则：进入 ServiceLoader、反射、安全框架等类时停止追踪，记录为边界

**cutoff.toml 初始内容**：
```toml
[cutoff]
classes = [
    "java/util/ServiceLoader",
    "java/lang/reflect/Method",
    "java/lang/reflect/Constructor",
    "java/security/AccessController",
    "sun/misc/Unsafe",
    "java/lang/ClassLoader",
    "java/lang/invoke/MethodHandle",
    "java/lang/invoke/MethodHandles",
]
```

**验收**：对 `tests/HelloWorld.java` 运行 RTA，可达类数量从失控降到 30 以内。

---

### T09 · 方法分类器
**状态**：`[x]`  
**文件**：`scripts/codegen/classify.py`（新建）

**目标**：对 RTA 可达方法集里每个方法打标签：

```python
class MethodCategory(Enum):
    AUTO_TRANSPILE = "auto_transpile"   # 可自动转译
    NATIVE_STUB    = "native_stub"      # ACC_NATIVE，需手填
    UNSUPPORTED    = "unsupported"      # invokedynamic/MethodHandle 等
    MANUAL_OVERRIDE = "manual_override" # java_runtime 手写层已覆盖
```

分类逻辑优先级：手写层覆盖 > native flag > 不支持指令扫描 > 默认 AUTO_TRANSPILE。

**验收**：对 `tests/TestP1.java` 的可达方法集输出分类表，格式正确。

---

### T10 · 清单生成器
**状态**：`[x]`  
**文件**：`scripts/gen_manifest.py`（新建），输出 `manifest.toml`

**目标**：读取 RTA 分析结果 + 方法分类结果，生成 `manifest.toml`（格式见设计文档第 8 章）。

**字段**：`id`、`category`、`class_file`、`jni_name`（native 时）、`suggested_impl`、`status`（todo/done/wont_implement）。

**验收**：`python3 scripts/gen_manifest.py tests/HelloWorld.java` 生成合法 TOML，包含 meta 摘要和每个方法条目。

---

### T11 · 空壳生成器
**状态**：`[x]`  
**文件**：`scripts/gen_stubs.py`（新建）

**目标**：读取 `manifest.toml`，对每个 `status = "todo"` 的条目生成带 `compile_error!` / `todo!()` 的 Rust 空壳文件；对 `status = "done"` 的条目生成 `pub use` 转发。

**输出目录**：`output/src/java_runtime/`（按 JDK 包路径）。

**验收**：`cargo check` 输出的 `compile_error!` 列表即为 native stub TODO 清单。

---

## 阶段四：JDK 自动转译（Phase 4 — JDK Transpilation）

### T12 · JDK 类文件定位 + 元数据自动发现
**状态**：`[x]`  
**文件**：`scripts/codegen/transpile.py`（扩展），`scripts/codegen/emitter.py`（扩展）

**实现内容**：
- `_collect_jdk_refs(class_infos)`：扫描用户类所有方法指令注释，提取 JDK 类 binary name（invoke*, getstatic, new 等指令）
- `_discover_jdk_classes(class_infos)`：通过 `JdkResolver` 从 jmods 解析每个引用的 JDK `.class` 文件，返回 `ClassInfo` 列表
- `_gen_jdk_class_rs(ci)`：为 JDK 类生成 `// @java_class(...)` + `// @java_native(...)` 元数据注释文件
- `_jdk_class_file_path()`：将 binary name 映射到 `src/java/.../*.rs` 路径
- `write_cargo_project()` 新增 `jdk_class_infos` 参数，写入 `src/java/` 目录下的元数据文件

**HelloWorld 发现的 JDK 类**（直接引用）：
- `java/io/PrintStream`（0 native）
- `java/lang/Object`（6 native：getClass, hashCode, clone, notify, notifyAll, wait0）
- `java/lang/StringBuilder`（0 native）
- `java/lang/System`（9 native：registerNatives, setIn/Out/Err0, currentTimeMillis, nanoTime, arraycopy, identityHashCode, mapLibraryName）
- `java/util/ArrayList`（0 native）
- `java/util/List`（interface，无 native）

**build.rs 同步改进**：状态重建从 `@java_native` 注释全新构建（不保留旧的 needed/implemented 条目），只继承人工设置的 `not-needed` 标记，避免删除注释后旧条目残留。

**验收**：
- `native_status.toml` 自动填充 `java.lang.Object` 和 `java.lang.System` 的 native 方法状态
- `cargo run` 输出 `Hello, World`，无编译错误

---

### T13 · Hello World 端到端验证
**状态**：`[x]`  
**文件**：`tests/HelloWorld.java`

**目标**：`tests/HelloWorld.java` 经完整流水线（RTA → manifest → stub gen → auto-transpile → cargo build → 运行）输出 `Hello, World!`，与 `java HelloWorld` 一致。

**流程**：
```bash
python3 scripts/gen_manifest.py tests/HelloWorld.java
python3 scripts/gen_stubs.py manifest.toml
python3 scripts/main.py tests/HelloWorld.java
cd output && cargo run --release
```

**验收**：输出为 `Hello, World!`，`cargo check` 无 `compile_error!`（所有 native stub 已实现或标记 wont_implement）。

---

## 完成回归保护

每个任务完成后必须验证：

```bash
# P0–P3 Java vs Rust 输出对比
for f in TestP0 TestP1 TestP2 TestP3; do
  java_out=$(java -cp tests/classes $f)
  python3 scripts/main.py tests/$f.java > /dev/null 2>&1
  rust_out=$(cd output && cargo run --release 2>/dev/null)
  [ "$java_out" = "$rust_out" ] && echo "✓ $f" || echo "✗ $f"
done
```

---

## 任务依赖图

```
T01 (classfile parser)
T02 (Rust IR)
T03 (renderer) ─── 依赖 T02
T04 (StackSim) ─── 依赖 T02, T03
T05 (instr.py) ─── 依赖 T04
T06 (method.py)─── 依赖 T05
T07 (emitter)  ─── 依赖 T06, T03
T08 (RTA 改进) ─── 独立
T09 (分类器)   ─── 依赖 T08
T10 (清单生成) ─── 依赖 T09
T11 (空壳生成) ─── 依赖 T10
T12 (JDK转译)  ─── 依赖 T01, T07, T11
T13 (Hello World) ─ 依赖 T12
```

**最优执行序**：T01 → T02 → T03 → T04 → T05 → T06 → T07（先完成代码生成链）→ T08 → T09 → T10 → T11 → T12 → T13

---

## 阶段五：字节码元信息属性化 + build.rs 自动化（Phase 5 — Metadata & Build Automation）

> 详细设计见：`docs/plans/2026-09-12-jdk-bytecode-translation.md`

### T14 · classfile.py 完整元信息扩展
**状态**：`[x]`  
**文件**：`scripts/codegen/classfile.py`（修改），`scripts/codegen/types.py`（修改）  
**依赖**：T01（已完成）

**目标**：`ParsedClass` / `ParsedMethod` / `ParsedField` 补充完整字节码元信息：

- `ParsedClass`：`super_class`、`interfaces[]`、`generic_signature`（Signature 属性）、`source_file`、`access_flags`、`is_interface`、`is_abstract`、`is_enum`
- `ParsedMethod`：`access_flags`、`is_native`（ACC_NATIVE）、`is_abstract`（ACC_ABSTRACT）、`exceptions[]`（Exceptions 属性）、`generic_signature`
- `ParsedField`：`access_flags`、`is_static`、`generic_signature`

**验收**：`parse_class('tests/classes/HelloWorld.class')` 返回包含 `super_class='java/lang/Object'`、`interfaces=[]` 的结构；`parse_class` 对某个已知含泛型签名的类（如 ArrayList.class）能正确提取 `generic_signature`。

---

### T15 · JDK Class 解析器
**状态**：`[x]`  
**文件**：`scripts/codegen/jdk_resolver.py`（新建）  
**依赖**：T14

**目标**：实现 `JdkResolver` 类，给定 binary name（如 `java/util/ArrayList`）返回 `.class` 字节流。

**实现要点**：
- 优先从 `$JAVA_HOME/jmods/java.base.jmod`（zip 格式）读取
- 不在 java.base 中的类，遍历其他 `*.jmod` 文件
- `is_jdk_class(binary_name)` 判断是否为 JDK 内置类（`java/`、`javax/`、`sun/` 等前缀）
- 缓存已读取的 ZipFile 对象

**验收**：
```python
r = JdkResolver()
data = r.resolve('java/util/ArrayList')
assert data is not None and data[:4] == b'\xca\xfe\xba\xbe'
cls = parse_class_bytes(data)
assert cls.class_name == 'java/util/ArrayList'
assert 'java/util/List' in cls.interfaces
```

---

### T16 · emitter.py 生成 Java 元信息属性宏
**状态**：`[x]`  
**文件**：`scripts/codegen/emitter.py`（修改）  
**依赖**：T14

**目标**：在生成的 `.rs` 文件的 struct / method / field 上附加属性宏，内嵌完整 Java 元信息：

```rust
#[java_class(
    binary_name = "java/util/ArrayList",
    super_class  = "java/util/AbstractList",
    interfaces   = "java/util/List,java/util/RandomAccess,java/lang/Cloneable,java/io/Serializable",
    generic_sig  = "<E:Ljava/lang/Object;>...",
    access       = "public",
    source_file  = "ArrayList.java",
)]
pub struct ArrayList<E> {
    #[java_field(name = "size", descriptor = "I", access = "private")]
    pub size: Field<i32>,
}

impl<E: Clone> ArrayList<E> {
    #[java_method(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public")]
    pub fn add(&self, e: E) -> Result<bool> { ... }

    #[java_native(name = "clone", descriptor = "()Ljava/lang/Object;", access = "protected native")]
    pub fn clone_java(&self) -> Result<JvmObject> {
        todo!("native java/util/ArrayList.clone:()Ljava/lang/Object;")
    }
}
```

**native 方法处理**：无字节码体时生成 `todo!` 存根；同时在 Python 侧收集待实现清单。

**验收**：`python3 scripts/main.py tests/HelloWorld.java` 后，`output/src/hello_world.rs` 开头出现 `#[java_class(binary_name = "HelloWorld", ...)]`。

---

### T17 · build.rs 属性扫描与 native_status.toml 自动维护
**状态**：`[x]`  
**文件**：`output/build.rs`（新建），`output/build_support/java_meta.rs`（新建）  
**依赖**：T16

**目标**：`build.rs` 扫描 `src/` 下所有 `.rs` 文件，提取 `#[java_class]` / `#[java_native]` 属性，对照 `native_impls/` 目录，自动生成并更新 `native_status.toml`。

**build.rs 职责**：
1. 扫描 `src/`，提取 `#[java_native(...)]` 标注的所有方法（binary_name + descriptor）
2. 检查 `native_impls/` 中是否有对应实现文件和函数签名
3. 更新 `native_status.toml`：`implemented` / `needed` / `stub` / `not-needed`
4. 若有 `needed` 状态的 native 方法，输出清单（不阻断构建，只警告；阻断策略在 P6 引入）
5. `println!("cargo:rerun-if-changed=src/");` 和 `native_impls/`

**验收**：
- 运行 `cargo build` 后 `native_status.toml` 自动出现，包含 HelloWorld 调用链涉及的 native 方法
- 修改 `native_impls/` 下某个文件后再次 `cargo build`，对应条目状态变为 `implemented`

---

### T18 · native_impls 目录建立与 HelloWorld 最小 native 实现
**状态**：`[x]`  
**文件**：`native_impls/java/lang/system.rs` 等（新建）  
**依赖**：T17

**目标**：根据 `native_status.toml` 中 `needed` 的方法，逐一手写 Rust 实现，直到 HelloWorld 端到端运行正确。

**HelloWorld 调用链预期需要的 native 方法**（待 T17 验证后确认）：
- `java/lang/System.arraycopy`
- `java/io/FileDescriptor.sync`（或类似 IO 底层）
- `java/lang/Object.hashCode`（如被调用）
- `java/lang/String.intern`（如被调用）

**验收**：`native_status.toml` 中无 `needed` 状态条目；`cargo run` 输出与 `java HelloWorld` 一致。

---

**阶段五任务依赖**：

```
T14 (classfile 元信息) ─ 依赖 T01（已完成）
T15 (JDK resolver)    ─ 依赖 T14
T16 (emitter 属性宏)  ─ 依赖 T14
T17 (build.rs)        ─ 依赖 T16
T18 (native impls)    ─ 依赖 T17
```

**最优执行序**：T14 → T15（并行 T16）→ T17 → T18

---

## 阶段六：架构重新设计（Phase 6 — Architecture Redesign）

> 详细设计见：`docs/plans/2026-09-12-architecture-redesign.md`  
> 设计规则见：`docs/rules.md`

**背景**：代码审计发现代码生成层存在 5 个根本性问题（魔法字符串、字符串类型标注、if-chain 分派、正则后处理、RawExpr 泛滥），需要从架构层面系统性修复。

### T19 · rs_ir.py 扩展新 IR 节点
**状态**：`[x]`  
**文件**：`scripts/codegen/rs_ir.py`、`scripts/codegen/render.py`  
**依赖**：T02（已完成）

**目标**：扩展 3 个新节点：
- `NewPendingExpr(class_name: str)`：对应 `new` 指令，等待 `<init>` 完成
- `StaticFieldRef(class_name: str, field_name: str, ty: RsType)`：对应 `getstatic`
- `RsInfer`：对应 Rust `_`，用于集合泛型占位

同时在 `render.py` 中补充这 3 个节点的渲染逻辑。

**验收**：3 个新节点能通过 `render_expr()` / `render_type()` 正确渲染。

---

### T20 · stack.py 删除字符串兼容路径
**状态**：`[x]`  
**文件**：`scripts/codegen/stack.py`  
**依赖**：T19

**目标**：删除所有向后兼容的字符串接受路径：
- `push()` 只接受 `(RsExpr, RsType)`，不再接受 `str`
- `pop()` 返回 `(RsExpr, RsType)`
- 删除 `pop_str()`、`load_local_str()` 方法
- `store_local()` 生成 `LetStmt(mutable=False, ...)`，不再拼字符串
- `emit()` 只接受 `RsStmt`，不再接受 `str`

**验收**：`stack.py` 中无任何 `str` 类型的 `push`/`emit` 调用。

---

### T21 · instr.py 全量 IR 化（消灭魔法字符串）
**状态**：`[x]`  
**文件**：`scripts/codegen/instr.py`  
**依赖**：T20

**目标**：全部 JVM 指令改为产生 IR 节点，消灭所有 f-string 拼接：
- 整型/浮点常量 → `Lit(val, ty)`
- 算术运算 → `BinOp`
- `new` 指令 → `NewPendingExpr(class_binary_name)`（从注释通用解析，无类名硬编码）
- `getstatic/putstatic` → `StaticFieldRef` / `StaticFieldAssign`（从注释通用解析）
- `getfield/putfield` → `FieldAccess` / `AssignStmt`（从注释通用解析）
- `invokevirtual`（JDK）→ 保留 if-chain 暂时不动（T22 处理）
- `invokevirtual`（用户类）→ `MethodCall`
- store/load → `LetStmt` / `Var`

**验收**：`instr.py` 中无 `f"..."` 字符串拼接，无 `__new__`/`__stdout__` 魔法字符串。

---

### T22 · method.py 删除正则后处理，改为 IR mutation 分析
**状态**：`[x]`  
**文件**：`scripts/codegen/method.py`  
**依赖**：T21

**目标**：删除 4 个正则后处理函数，替换为 IR 级分析：
- 删 `_fix_coll_types`（类型由 IR 构建时确定）
- 删 `_remove_unnecessary_mut`（替换为 `_analyze_mutation()` 扫描 `AssignStmt`）
- 删 `_merge_aliases`（`store_local` 直接使用目标变量名）
- 删 `_simplify_wrapping_add`（`iinc` 改为输出 `AssignStmt(BinOp("+="))`）

新增 `_analyze_mutation(stmts) -> set[str]`，在渲染前遍历 IR 设置 `LetStmt.mutable`。

**验收**：`method.py` 中无 `import re`，无任何正则操作。

---

### T23 · 阶段 A 回归验证
**状态**：`[x]`  
**依赖**：T22

**目标**：确认 A1–A4 改造后全部测试仍正确。

**验收**：
```bash
cargo check   # 0 errors
# Java vs Rust 输出对比
for f in TestP0 TestP1 TestP2 TestP3; do
  java_out=$(java -cp tests/classes $f)
  rust_out=$(cd output && cargo run --release 2>/dev/null)
  [ "$java_out" = "$rust_out" ] && echo "✓ $f" || echo "✗ $f"
done
# HelloWorld 端到端
cd output && cargo run --release  # 输出 Hello, World!
```

---

### T24 · jdk_dispatch.py 新建（过渡注册表）
**状态**：`[x]`  
**文件**：`scripts/codegen/jdk_dispatch.py`（新建）  
**依赖**：T21

**目标**：用注册表替代 `instr.py` 中的 if-chain：
- `JDK_INTERFACE_GROUPS`：接口 binary name → 组 ID（不枚举实现类）
- `get_dispatch_group(class_binary_name, registry)`：从 `ClassInfo.all_interfaces()` 动态推导组 ID
- `JDK_VIRTUAL_METHODS`：`(组ID, 方法名)` → handler 函数（返回 IR 节点）
- `instr.py` 的 `_gen_invokevirtual` 改为查注册表，删除所有 `_dispatch_list/map/set` 函数

**注意**：`jdk_dispatch.py` 是过渡产物，Phase D（T27）完成后整个文件删除。

**验收**：`instr.py` 中无 `if cls in (...)` JDK 分派 if-chain。

---

### T25 · 泛型类型实时确定（删除 JDK_COLL_TYPES）
**状态**：`[x]`  
**文件**：`scripts/codegen/jdk_dispatch.py`、`scripts/codegen/type_map.py`  
**依赖**：T24

**目标**：
- `new ArrayList` 时压 `RsGeneric('ArrayList', [RsInfer()])`
- 第一次 `list.add(elem)` 时，handler 调用 `_resolve_coll_type(sim, obj, elem_ty)` 将 `RsInfer` 替换为实际类型
- 删除 `type_map.py` 中的 `JDK_COLL_TYPES`

**验收**：`type_map.py` 中无 `JDK_COLL_TYPES`，`ArrayList<i32>` 类型由 IR 自动推导，无正则修补。

---

### T26 · 阶段 B 回归验证
**状态**：`[x]`  
**依赖**：T25

**目标**：确认 B1–B3 改造后全部测试仍正确，重点验证 TestP3（集合泛型）。

**验收**：P0–P3 + HelloWorld 全部通过，TestP3 的 `ArrayList<i32>`/`HashMap<String,i32>` 类型正确。

---

### T27 · emitter.py 升级属性格式，build.rs 同步更新
**状态**：`[x]`  
**文件**：`scripts/codegen/emitter.py`、`output/build.rs`  
**依赖**：T17（已完成）

**目标**：
- `emitter.py`：JDK 元数据文件从 `// @java_class(...)` 注释格式升级为 `#[java_class(...)]` 属性格式，嵌入完整元信息（`binary_name`、`super_class`、`interfaces`、`generic_sig`、`access`、`source`）
- 接口类生成 `#[java_interface(...)]`，方法生成 `#[java_method(...)]` / `#[java_native(...)]`
- `build.rs`：扫描正则从 `// @java_*` 改为 `#[java_*]`，从属性自动重建 `ClassRegistry`（含完整类层次图）
- `build.rs` 中无任何手工维护的类名/接口名常量

**验收**：生成的 `.rs` 文件使用 `#[java_class(...)]` 语法；`build.rs` 能从属性中正确重建继承链；`native_status.toml` 内容与之前一致。

---

### T28 · 完整 JDK 字节码翻译
**状态**：`[x]`  
**文件**：`scripts/codegen/emitter.py`  
**依赖**：T23、T27

**目标**：
- `_gen_jdk_class_rs()` 从"生成元数据存根"改为调用与用户类相同的完整翻译流程
- `write_cargo_project()` 中用户类和 JDK 类使用统一翻译入口 `write_class()`
- 删除 `jdk_dispatch.py`（整个文件）
- 删除 `type_map.py` 中剩余的任何 JDK 类名常量
- `instr.py` 的 `_gen_invokevirtual` 退化为全类统一处理，无任何 JDK 特判

**验收**：
- `jdk_dispatch.py` 文件不存在
- Python 代码中无任何 JDK 类名字符串常量
- JDK 类的 `.rs` 文件有完整 Rust 方法体（非 `todo!` 存根）
- 全部测试通过

---

### T29 · build.rs 构建阻断升级
**状态**：`[x]`  
**文件**：`output/build.rs`  
**依赖**：T27

**目标**：将 `native_status.toml` 中 `needed` 状态的 native 方法从 `cargo:warning=` 升级为 `cargo:error=`，阻断构建并打印清晰错误信息。

**验收**：当 `native_status.toml` 存在 `needed` 条目时，`cargo build` 报错并列出需要实现的方法；无 `needed` 条目时构建成功。

---

**阶段六任务依赖**：

```
T19 (rs_ir 扩展)      ─ 依赖 T02
T20 (stack.py 严格化) ─ 依赖 T19
T21 (instr.py IR化)   ─ 依赖 T20
T22 (method.py 清理)  ─ 依赖 T21
T23 (阶段A验证)       ─ 依赖 T22
T24 (jdk_dispatch)    ─ 依赖 T21
T25 (泛型推导)        ─ 依赖 T24
T26 (阶段B验证)       ─ 依赖 T25
T27 (属性格式升级)    ─ 依赖 T17
T28 (JDK完整翻译)     ─ 依赖 T23, T27（删除 jdk_dispatch.py）
T29 (构建阻断)        ─ 依赖 T27
```

**最优执行序**：T19 → T20 → T21 → T22 → T23（阶段A）→ T24 → T25 → T26（阶段B）→ T27（阶段C）→ T28（阶段D）→ T29（阶段E）

---

## 阶段七：泛型支持 + 字节码属性完善（Phase 7 — Generics & Attributes）

> 覆盖 T29 之后的全部工作：方法级 BFS、checkcast 驱动的类型擦除方案、字段 Signature 使用。

### T30 · 方法级 BFS（879 类完整 JDK 依赖图）
**状态**：`[x]`  
**文件**：`scripts/codegen/transpile.py`

**目标**：将 BFS 深度从"类级引用"改为"方法级调用图"，精确追踪实际调用链上的类，不添加多余类。

**结果**：BFS 发现 879 个 JDK 类，生成 `jdk_classes` crate；JField 由 `Field` 重命名为 `JField` 以避免语义歧义；脚本清理删除废弃代码。

**验收**：`cargo check` 零错误，HelloWorld 和 TestP0–P3 全部通过。

---

### T31 · checkcast 驱动的泛型类型擦除方案
**状态**：`[x]`  
**文件**：`output/java_runtime/src/java/lang/object.rs`、`scripts/codegen/instr.py`、`scripts/codegen/emitter.py`

**背景**：Java 泛型在字节码层面被类型擦除（`ArrayList<String>.get()` 返回 `Object`）。JVM 在赋值给具体类型变量时插入 `checkcast` 指令做运行时类型收窄。

**实现**：
- `Object` 新增 `from_any<T>()` 工厂方法和 `downcast<T>()` 方法（内部存 `Rc<dyn Any>`）
- `checkcast` 指令处理器：当栈顶是 `Object` 类型时，自动包装 `.downcast::<T>()` 表达式
- `emitter.py` 为每个非 Object 类自动生成 `impl Into<Object>` 和 `impl From<Object>` trait

**验收**：`tests/TestGenerics.java`（ArrayList<String>）输出 `Alice / Bob / Charlie / 3`，与 Java 运行结果一致。

---

### T32 · 字段级 Signature 支持
**状态**：`[x]`  
**文件**：`scripts/codegen/sig_parser.py`、`scripts/codegen/emitter.py`

**背景**：`FieldInfo.generic_signature` 已被解析并存储（如 `elementData` 的 `[TE;`），但 emitter 生成字段时仍用裸描述符，导致 `JField<Object>` 而非 `JField<E>`。

**实现**：
- `sig_parser.py` 新增 `parse_field_type(sig, class_type_params) -> str`：解析字段 Signature 为 Rust 类型
- `emitter.py`：字段类型优先用 `generic_signature`，回退到裸描述符
- **关键修复**：泛型 struct bounds 从 `E: Clone + Default + 'static` 改为 `E: Clone + 'static`，让 `#[derive(Default)]` 在 impl Default 的 where 子句中自动添加 `Default`，避免 native_impls 函数需要 `E: Default`

**验收**：`cargo check` 零错误；TestGenerics 仍输出正确结果；`jdk_classes` 中泛型字段类型更精确。

---

### T33 · 单 crate 架构迁移
**状态**：`[ ]`  
**文件**：`output/Cargo.toml`、`scripts/codegen/emitter.py`

**背景**：当前 workspace 有 `java_runtime`、`jdk_classes`、`user` 三个 crate，目标态是合并为单 crate（只保留 `user`）。

**目标**：
- `jdk_classes` 内容内联到 `user/src/java/`
- `java_runtime` 的 `error.rs`、`types.rs` 内联到 `user/src/`
- `java_runtime/src/java/` 临时手写层由字节码翻译替换后删除
- Cargo.toml 只剩一个 `[[bin]]` 条目

**验收**：`output/` 下无 `jdk_classes`、`java_runtime` 子目录；`cargo run` 仍输出正确结果。

---

### T34 · LocalVariableTypeTable 解析（泛型局部变量）
**状态**：`[x]` 已完成（commit `223a54c`）  
**文件**：`scripts/codegen/classfile.py`、`scripts/codegen/types.py`、`scripts/codegen/stack.py`、`scripts/codegen/method.py`、`scripts/codegen/instr.py`

**背景**：`LocalVariableTable` 给出局部变量名，但对泛型变量只有裸描述符（如 `Ljava/lang/Object;`）。`LocalVariableTypeTable` 保存带泛型签名的类型（如 `TE;` 表示类型变量 `E`）。

**实现**：
- `classfile.py`：解析 LVTT 子属性，`slot → Signature` 映射
- `types.py`：`ParsedMethod.local_types` 字段存储 LVTT 映射
- `method.py`：预计算 `slot_hint_types`（调用 `parse_field_type` 转换），传入所有 `StackSim` 构造器
- `stack.py`：`store_local` 在栈顶类型为 `Object` 时用 LVTT hint 覆盖；`Var+RsGeneric` 时 `let_ty=None` 避免 `newarray` 类型不匹配
- `instr.py`：Vec 类型参数从 `&arr` 改为 `arr.clone()`，修复 `Rc<RefCell<Vec<T>>>` 传参

**验收结果**：TestGenerics `first/second/third` 已通过 `checkcast` 得到 `String` 类型（LVTT 与 checkcast 结果一致）；类型变量场景（`TE;` → `E`）在泛型类方法（如 ArrayList.set 的 `oldValue: E`）中生效，但这些方法目前是 stub 所以效果不可见。

---

### T35 · Autoboxing + HashMap/HashSet native 实现
**状态**：`[x]` 已完成（commit `2f692f1`）  
**文件**：`scripts/codegen/instr.py`、`output/java_runtime/src/java/lang/object.rs`、`output/native_impls/java/io/print_stream.rs`、`output/native_impls/java/util/hash_map.rs`、`output/native_impls/java/util/hash_set.rs`

**背景**：Java primitive → Object 参数传递时需要自动装箱；生成代码对 `ArrayList<Integer>` 等集合传入 `i32` 但方法期望 `Object`，导致 10 处编译错误。HashMap/HashSet 的 put/get/containsKey/add/contains 均为 stub。

**实现**：
- `instr.py`：`_gen_invokevirtual` 中对所有传入 Object 参数的值统一插入 `.into()`（含 primitive）
- `object.rs`：`impl From<i32/i64/...> for Object`；`PartialEq`（primitive 值比较 + ptr_eq）；`Display` 展示 primitive 值；`fmt_primitive()` 供 jdk_classes 扩展
- `print_stream.rs`：新增 `println__z`、`println__j`、`println__obj`；后者在 jdk_classes 上下文 downcast String 正确显示
- `hash_map.rs`：put/get/containsKey/size/isEmpty；`Vec<(Object,Object)>` 存入 `table` 字段；`_obj_eq` 支持 String 内容比较
- `hash_set.rs`：add/contains/size/isEmpty；`Vec<Object>` 存入 `map` 字段

**验收**：TestP3 输出 `3 / 20 / 3 / 2 / true / 2 / true` ✓

---

## 阶段八：Java 风格 Ergonomic API（Phase 8 — Java-like API）

> 目标：用户用 Rust 调用生成 API 时，代码写法与 Java 高度相似。差异通过宏/VM 层吸收，对用户透明。
> 完整设计见：`docs/plans/2026-09-13-java-like-api-roadmap.md`

### T36 · Unboxing：Object → primitive 反向转换
**状态**：`[x]` 已完成（本 session）  
**文件**：`output/java_runtime/src/java/lang/object.rs`

**目标**：添加 `impl From<Object> for i32/i64/f32/f64/bool`，使 `ArrayList<i32>` 的 `get(0) -> i32` 可直接工作，不需要用户手动 downcast。

**实现**：
- `Object` 中添加 `downcast::<T>()` 辅助方法（panic on wrong type）或返回 `Option`
- `impl From<Object> for i32 { fn from(o: Object) -> i32 { *o.0.downcast_ref::<i32>().unwrap() } }`
- 同理 i64、f32、f64、bool、i8、i16、u16

**验收**：`let n: i32 = ArrayList::<i32>::new_default()?.get(0)?;` 编译通过

---

### T37 · 方法名去 mangle + 构造器 `new()`
**状态**：`[ ]`  
**文件**：`scripts/codegen/emitter.py`、`scripts/codegen/instr.py`

**目标**：
1. 每组重载中参数最少（JVM 擦除后的 Object 参数）的版本保留 Java 原始名（`add`、`get`、`put`）；同时生成带 `_obj` 后缀的别名供字节码翻译代码调用
2. `new_default()` → `new()`（无参 `<init>` 的 synthetic 构造器）

**规则**：
- `add__obj(Object)` → 保留为 `add(Object)`，方法签名不变
- `println__i(i32)` / `println__str(String)` / `println__obj(Object)` → 由 T38 的 trait 统一，不需要这些重载了
- `new_default()` → `new()`

**验收**：`ArrayList::new()?`、`list.add(x)?` 编译通过

---

### T38 · println 统一 trait 派发
**状态**：`[~]` 部分完成（Printable trait 已定义，instr.py 统一生成待实现）  
**文件**：`output/java_runtime/src/java/lang/object.rs`（定义 Printable）、`output/native_impls/java/io/print_stream.rs`（统一 println）、`scripts/codegen/instr.py`（生成 println(x)）

**目标**：所有 `println__*` 重载统一为 `println<T: Printable>(v: T)`，代码生成器生成统一调用 `println(x)`

**实现**：
- `java_runtime` 中定义 `Printable` trait，i32/i64/bool/f32/f64/Object 均实现
- `PrintStream::println<T: Printable>` 替换 `println__i/println__str/println__obj`
- `instr.py` 中 `invokevirtual println:*` 统一生成 `println(x)`

**验收**：`System::out().println(42)?`、`System::out().println(true)?`、`System::out().println(s)?` 均编译通过

---

### T39 · 集合 ergonomic 泛型方法层
**状态**：`[~]` 部分完成（ergonomic 方法已实现，命名待 T37 统一）  
**文件**：`output/native_impls/java/util/array_list_ergonomic.rs`、`output/native_impls/java/util/hash_map_ergonomic.rs`、`output/native_impls/java/util/hash_set_ergonomic.rs`（新建）；`scripts/codegen/emitter.py`（添加 _ergonomic.rs 自动 include 机制）

**目标**：`ArrayList<String>` 可直接 `list.add(s)` 和 `let v: String = list.get(0)?`，不需要 `.into()`/`.downcast()`

**实现**：在 native_impls 中为集合类添加泛型 ergonomic 方法，约束 `E: Clone + Into<Object> + From<Object> + 'static`：
- `ArrayList<E>::add(e: E)` → 内部调用 `add_obj(e.into())`
- `ArrayList<E>::get(i: i32) -> Result<E>` → 内部调用 `get_obj(i)` 再 `E::from(obj)`
- `HashMap<K,V>::put(k: K, v: V)` / `get(k: K) -> Result<V>`
- `HashSet<E>::add(e: E)` / `contains(e: E) -> Result<bool>`

**依赖**：T36（From<Object> for primitive）

**验收**：`let v: String = list.get(0)?;`、`let n: i32 = nums.get(0)?;` 均无需手动类型转换，编译通过

---

### T40 · `java_rta_macros` proc-macro crate
**状态**：`[ ]`  
**文件**：新建 `java_rta_macros/` crate

**目标**：将 `cfg_attr(any(), java_class(...))` 中的死属性变为真实激活的 proc-macro，自动为所有 java class 生成：`Into<Object>`、`From<Object>`、`Display`、`Debug`；用户自定义 struct 也可使用 `#[java_class]`

**实现**：
- 新建 proc-macro crate，加入 workspace
- `#[java_class(binary_name = "...")]` 展开：`impl Into<Object>`、`impl From<Object>`、`impl Display`、`impl Debug`
- 修改 emitter.py 将 `cfg_attr(any(), java_class(...))` 改为 `#[java_class(...)]`
- 用户文档：如何在自定义 Rust struct 上使用 `#[java_class]` 接入 JDK 集合 API

**依赖**：T36、T37、T39

**验收**：
```rust
#[java_class(binary_name = "com/example/Person")]
struct Person { pub name: JField<String> }
let persons: ArrayList<Person> = ArrayList::new()?;
persons.add(Person::default())?;
let p: Person = persons.get(0)?;
```

---

**阶段七任务依赖**：

```
T30 (方法级 BFS)          ─ 依赖 T28（已完成）
T31 (checkcast 类型擦除)  ─ 依赖 T30
T32 (字段 Signature 使用) ─ 依赖 T31
T33 (单 crate 迁移)       ─ 依赖 T32
T34 (LocalVariableTypeTable) ─ 依赖 T32
T35 (Autoboxing + HashMap/HashSet) ─ 依赖 T34（已完成）
```

**阶段八任务依赖**：

```
T36 (Unboxing)             ─ 依赖 T35（已完成）
T37 (方法名去 mangle)      ─ 依赖 T35（已完成）
T38 (println trait)        ─ 独立（可并行 T36/T37）
T39 (集合 ergonomic 方法)  ─ 依赖 T36
T40 (proc-macro crate)     ─ 依赖 T36、T37、T39
```

**最优执行序**：T30 → T31 → T32（已全部完成）→ T33、T34（并行，待执行）
T36 + T37 + T38（可并行）→ T39 → T40

---

### T41 · ArrayList/HashMap/HashSet 对比测试（Java vs Rust）
**状态**：`[x]` 已完成（本 session 验证）  
**文件**：`tests/TestArrayList.java`、`tests/test_array_list_rust.rs`、`output/user/src/test_array_list_ergonomic.rs`

**目标**：证明生成的 Rust API 与 Java 代码高度相似；记录当前差距和目标写法。

**实现**：
- `tests/TestArrayList.java`：完整 Java 测试（ArrayList/HashMap/HashSet 基本操作）
- `output/user/src/test_array_list_ergonomic.rs`：可编译的 Rust 对比测试，展示当前写法
- `tests/test_array_list_rust.rs`：对比文档，Java vs 当前 Rust vs 目标 Rust 逐段注释

**验证结果**：Java 输出与生成 Rust 输出逐行一致（14 行全部匹配）：
```
3 / Alice / Bob / Charlie / 3 / 100 / 95 / 3 / 30 / 25 / true / false / 2 / true / false
```

**待修复（记录）**：
- T42：for-each 增强循环代码生成 bug（局部变量 slot 复用导致类型冲突，在 TestArrayList.java 中注释标注）

---

### T42 · for-each 增强循环代码生成 bug
**状态**：`[ ]`  
**文件**：`scripts/codegen/instr.py`（或 `emitter.py`）

**问题**：Java `for (String name : names)` 编译为 `names.iterator()` + `hasNext()` + `next()` 字节码。JVM 编译器将 for-each 的匿名迭代器存入一个局部变量 slot，该 slot 在循环结束后**被后续变量复用**（Java 编译器的 slot reuse 优化）。代码生成器按 slot 分配 Rust 变量名，导致后续变量被错误地类型声明为 `Iterator<Object>`。

**症状**：
```rust
// 生成的错误代码（slot 3 被 Iterator 和 HashMap 复用）：
let _t13 = names.iterator()?;
let mut ages: Iterator<Object> = _t13;   // ← 类型错误，应为 HashMap
ages = HashMap::<Object, Object>::new_default()?;  // ← 赋值类型不符
```

**根因**：for-each 的合成迭代器变量没有 LVT entry（编译器合成，匿名），但 `ages` 的 LVT entry start_pc 可能覆盖了迭代器的 store 指令，或者 slot 分配逻辑没有正确处理 slot 生命周期边界。

**修复方向**：
1. 在 for-each 模式检测时（`invokeinterface Iterator.hasNext + next`），将迭代器 slot 的生命周期限定在循环体内，不延续到循环后
2. 或：识别 for-each 字节码模式，生成 Rust `for v in list.iter()` 语法

**验收**：TestArrayList.java 的 for-each 段可以正确生成并运行
