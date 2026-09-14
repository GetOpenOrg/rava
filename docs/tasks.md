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
**状态**：`[ ]` 推迟（前置条件未满足）
**文件**：`output/Cargo.toml`、`scripts/codegen/emitter.py`

**背景**：当前 workspace 有 `java_runtime`、`java_rta_macros`、`jdk_classes`、`user` 四个 crate，目标态是合并为单 crate（只保留 `user`）。

**真正的前置条件**：`java_runtime/src/java/` 中的手写 String/ArrayList/System 等实现必须先被字节码翻译替换（T46 系列）。在此之前做 T33 只是搬移手写代码，没有实质收益。推迟到 `java_runtime/java/` 目录可以删除之后再做。

**目标**：
- `jdk_classes` 内容内联到 `user/src/java/`
- `java_runtime` 的 `error.rs`、`types.rs` 内联到 `user/src/`（永久保留的 VM 基础设施）
- `java_runtime/src/java/` 临时手写层由字节码翻译替换后删除
- `java_rta_macros` 保留为独立 crate（proc-macro 必须独立编译）
- Cargo.toml 只剩 `user` + `java_rta_macros` 两个成员

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
**状态**：`[x]` 已完成（本 session）  
**文件**：`scripts/codegen/type_map.py`、`scripts/codegen/emitter.py`、`scripts/codegen/instr.py`

**目标**：
1. 重载后缀从 `__` 改为 `_`：`add__obj` → `add_obj`，`println__i` → `println_i`
2. `@synthetic new()` 工厂：`new_default()` → `new()`
3. `emitter.py` 检测 `@synthetic new` 占用时，将 JDK `<init>` 存根改名为 `new_init` 避免冲突

**验收**：`ArrayList::<Object>::new()?`、`list.add_obj(x.into())?`、`println_v` 编译通过；`cargo check` 零错误

---

### T38 · println 统一 trait 派发
**状态**：`[x]` 已完成（本 session）  
**文件**：`output/java_runtime/src/lib.rs`（Printable trait）、`output/native_impls/java/io/print_stream_ergonomic.rs`（println_v）、`scripts/codegen/instr.py`（生成 println_v(x)）

**目标**：所有有参 println 重载通过 `Printable` trait 统一派发

**实现**：
- `java_runtime/src/lib.rs` 中定义 `Printable` trait，i32/i64/bool/f32/f64/String/Object 均实现
- 无参 `println()` 保留原名（JDK 生成）；有参版本通过 `println_v<T: Printable>` 派发
- `print_stream_ergonomic.rs`：`impl PrintStream { pub fn println_v<T: Printable>(&self, v: T) }`
- `instr.py`：有参 `invokevirtual println` → 生成 `println_v(x)` 调用

**验收**：`System::out().println_v(42)?`、`System::out().println_v(true)?`、`System::out().println_v(s)?` 均编译通过；TestArrayList 两个 binary 输出完全一致

---

### T39 · 集合 ergonomic 泛型方法层
**状态**：`[x]` 完成  
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
**状态**：`[x]` 完成  
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
**状态**：`[x]` 完成  
**文件**：`scripts/codegen/stack.py`

**问题**：Java `for (String name : names)` 编译为 `names.iterator()` + `hasNext()` + `next()` 字节码。JVM 编译器将 for-each 的匿名迭代器存入一个局部变量 slot，该 slot 在循环结束后**被后续变量复用**（Java 编译器的 slot reuse 优化）。代码生成器按 slot 分配 Rust 变量名，导致后续变量被错误地类型声明为 `Iterator<Object>`。

**症状**：
```rust
// 生成的错误代码（slot 3 被 Iterator 和 HashMap 复用）：
let _t13 = names.iterator()?;
let mut ages: Iterator<Object> = _t13;   // ← 类型错误，应为 HashMap
ages = HashMap::<Object, Object>::new_default()?;  // ← 赋值类型不符
```

**根因**：for-each 的合成迭代器变量没有 LVT entry（编译器合成，匿名），但 `ages` 的 LVT entry start_pc 可能覆盖了迭代器的 store 指令，或者 slot 分配逻辑没有正确处理 slot 生命周期边界。

**已实现修复**（`scripts/codegen/stack.py`）：
- `store_local()` 中检测 slot 类型变化，当新类型与旧类型不同时，生成 `LetStmt`（let 重新绑定/shadowing）而非 `AssignStmt`
- 这样 `ages: HashMap` 会用 `let` 重新声明，与 `Iterator` 类型的旧 slot 断开绑定

**待验证**：需要在 `TestArrayList.java` 中取消 for-each 注释，重新生成并 `cargo check` 验证

**验收**：TestArrayList.java 的 for-each 段可以正确生成并运行

---

## 阶段九：字节码翻译覆盖率扩展（Phase 9 — Bytecode Translation Coverage）

> 目标：逐步将 `java_runtime/src/java/` 中的手写实现替换为字节码翻译，消灭 `panic!("stub: ...")` 存根。每个任务对应一条完整调用链，从用户代码出发，追踪到 native 方法为止。

---

### T43 · StringBuilder 字节码翻译
**状态**：`[ ]`
**文件**：`output/native_impls/java/lang/string_builder.rs`、`scripts/codegen/transpile.py`

**背景**：Java `"Hello" + name` 在字节码层面编译为：
```java
new StringBuilder()
  .append("Hello")
  .append(name)
  .toString()
```
当前 `StringBuilder.new()` 有 native 实现，但 `append(String)`、`append(int)` 等重载和 `toString()` 均是 `panic!` 存根。

**目标**：
- 将 `java/lang/StringBuilder` 加入调用链翻译（翻译 `append`/`toString` 字节码）
- 在 `native_impls/java/lang/string_builder.rs` 中实现底层 `append`/`toString` native 方法
- 验证：`"Hello " + name` 字符串拼接能正确运行

**验收**：包含字符串拼接的 Java 程序翻译后正确输出

---

### T44 · 虚方法派发（invokevirtual 多态）
**状态**：`[ ]`
**文件**：`scripts/codegen/instr.py`、`scripts/codegen/emitter.py`

**背景**：当前 `invokevirtual` 生成的是静态直接调用（`obj.method()`），没有多态分派。若 `obj` 的运行时类型是子类，调用的仍是父类方法，行为错误。

**目标**：
- 分析 `invokevirtual` 生成逻辑，确定当前的静态调用边界
- 对需要多态的场景（接口调用、抽象类方法）生成正确的 trait 对象派发或 downcast 后调用
- 至少覆盖：父类引用调用子类覆写方法的场景

**验收**：包含单层继承+方法覆写的 Java 测试翻译后行为正确

---

### T45 · 异常处理（try/catch/finally）
**状态**：`[ ]`
**文件**：`scripts/codegen/method.py`、`scripts/codegen/stack.py`

**背景**：Java 异常在字节码层面通过异常表（exception table）表达，不是结构化控制流。当前代码生成器不处理异常表，遇到 try/catch 的字节码会生成错误代码或直接 panic。

**目标**：
- `classfile.py` 解析方法的异常表（exception handlers）
- `method.py` 根据异常表生成 Rust `match` 或 `if let Err(e) = ...` 结构
- 至少覆盖：单个 `catch` 子句、`finally` 块

**验收**：包含 try/catch 的 Java 程序翻译后正确处理异常

---

### T46 · java/lang/String 字节码翻译
**状态**：`[ ]`
**文件**：`output/native_impls/java/lang/string.rs`（新建）、`scripts/codegen/transpile.py`

**背景**：`java/lang/String` 目前由 `java_runtime/src/java/lang/string.rs` 手写实现。目标是将其替换为从 `String.class` 字节码翻译出的版本，只保留真正的 native 方法（`charAt`、`length` 等）在 `native_impls/` 中手写。

**目标**：
- 将 `java/lang/String` 从 `_JAVA_RUNTIME_CLASSES` 排除，走 codegen 路径
- 在 `native_impls/java/lang/string.rs` 实现 native 方法
- 删除 `java_runtime/src/java/lang/string.rs` 手写层

**依赖**：此任务是 T33（单 crate 迁移）的真正前置条件之一

**验收**：HelloWorld 在删除手写 String 后仍能正确运行

---

### T47 · java/lang/System + PrintStream 字节码翻译
**状态**：`[ ]`
**文件**：`output/native_impls/java/lang/system.rs`、`output/native_impls/java/io/print_stream.rs`

**背景**：`System.out.println` 是最常用的调用链起点，当前由手写实现驱动。目标是将 `java/lang/System` 和 `java/io/PrintStream` 加入字节码翻译路径，只保留真正的 native 底层（write syscall 等）在 `native_impls/` 中。

**依赖**：T46（String 需要先翻译）

**验收**：System.out.println 通过字节码翻译路径运行，`java_runtime/src/java/` 对应文件可删除

---

### T48 · 类继承与接口实现场景测试
**状态**：`[ ]`
**文件**：`tests/TestInheritance.java`（新建）

**背景**：目前所有测试都是单类场景。需要验证：子类继承父类字段/方法、接口实现、`instanceof` 检查、`super` 调用等是否能正确翻译。

**目标**：
- 编写包含继承、接口、`instanceof` 的 Java 测试
- 运行翻译器，确认生成代码编译并输出正确结果
- 记录并修复发现的 bug

**验收**：`TestInheritance.java` 翻译后与 `java TestInheritance` 输出一致

---

### T49 · Lambda 与匿名类翻译
**状态**：`[ ]`
**文件**：`scripts/codegen/instr.py`（invokedynamic 处理）

**背景**：Java lambda（`() -> ...`、`x -> x.toString()`）在字节码层面通过 `invokedynamic` + `bootstrap method` 实现。当前 `invokedynamic` 未实现，遇到 lambda 代码直接 panic 或生成错误代码。

**目标**：
- 理解 Java lambda 的 `invokedynamic` 字节码结构（LambdaMetafactory bootstrap）
- 设计 Rust 侧的表示方案（闭包 / trait object）
- 实现至少无捕获变量的简单 lambda 翻译

**验收**：`list.forEach(x -> System.out.println(x))` 能翻译并正确运行

---

### T50 · IR 对象化完成（T05/T06/T07 收尾）
**状态**：`[ ]`
**文件**：`scripts/codegen/instr.py`、`scripts/codegen/method.py`、`scripts/codegen/emitter.py`

**背景**：T05/T06 是 `[~]` 过渡状态——IR 基础设施已建立，但 `instr.py` 仍用 `RawExpr/RawStmt` 包字符串，`method.py` 仍输出字符串列表。这让代码生成逻辑难以分析和变换。

**目标**：
- `instr.py`：所有指令生成真正的 IR 节点（`CallExpr`、`BinOp`、`FieldAccess` 等），消灭 `RawExpr`
- `method.py`：方法体改为 `RsFn` IR 节点，通过 `render()` 输出
- `emitter.py`（T07）：使用 IR render 输出，删除字符串拼接残留

**依赖**：T05、T06 先各自推进，T07 等两者完成后合并

**验收**：`grep -r 'RawExpr\|RawStmt' scripts/` 无输出；所有现有测试仍通过

---

**阶段九任务依赖**：

```
T43 (StringBuilder)       ─ 独立，可立即开始
T44 (虚方法派发)          ─ 独立，可立即开始
T45 (异常处理)            ─ 独立，可立即开始
T46 (String 字节码翻译)   ─ 独立（T33 的真正前置）
T47 (System/PrintStream)  ─ 依赖 T46
T48 (继承/接口测试)       ─ 依赖 T44
T49 (Lambda/invokedynamic)─ 独立（高难度）
T50 (IR 对象化)           ─ 独立（内部质量，不阻塞功能）
T33 (单 crate 迁移)       ─ 依赖 T46、T47（手写层消灭后再做）
```

**推荐执行序**：
- 短期：T43（最小工作量，高价值）或 T44（解锁继承场景）
- 中期：T46 → T47（消灭手写层，推进目标架构）
- 长期：T49（Lambda）、T50（IR 质量）、T33（单 crate）

---

### T51 · 端到端测试自动化框架
**状态**：`[ ]`
**文件**：`scripts/run_tests.py`（新建）

**背景**：测试文件已按特性分类存放于 `tests/e2e/`，期望输出存于 `tests/expected/`。
目前每次验证需要手动对比 `java` 和 `cargo run` 的输出，无法自动化回归。

**目录结构（已建立）**：
```
tests/
├── e2e/
│   ├── 01_basics/    HelloWorld, TestArithmetic, TestArrays, TestControlFlow
│   ├── 02_oop/       TestObjects, TestInheritance, TestInterfaces
│   ├── 03_generics/  TestGenerics
│   ├── 04_collections/ TestCollections, TestArrayList
│   ├── 05_strings/   TestStringBuilder
│   ├── 06_exceptions/ TestExceptions
│   └── 07_lambdas/   TestLambda
├── expected/         *.txt  ← java 运行输出（ground truth）
└── compare/          Java vs Rust API 对比文档（手动阅读）
```

**目标**：`python3 scripts/run_tests.py` 自动完成：
1. 发现 `tests/e2e/**/*.java` 下所有测试文件
2. 对每个文件运行转译器 → 编译 Rust → 执行二进制，捕获 stdout
3. 与 `tests/expected/<ClassName>.txt` diff
4. 报告 PASS / FAIL，打印不匹配的行

**实现要点**：
- `--filter` 参数支持按目录或类名过滤（如 `--filter 01_basics`）
- `--update-expected` 参数：覆写 `expected/` 文件（更新 ground truth 时使用）
- 失败时打印 unified diff，方便定位代码生成问题
- 已知不支持的特性（T44/T45/T49 未完成时）可用 `# skip` 注释标记跳过

**验收**：
```bash
python3 scripts/run_tests.py --filter 01_basics   # 基础场景全部 PASS
python3 scripts/run_tests.py                       # 全量运行，输出每项状态
```

---

## 阶段十：代码生成器架构改进（Phase 10 — Codegen Architecture）

> 详细设计见：`docs/plans/2026-09-14-codegen-improvement-plan.md`  
> 目标：系统性修复 codegen 的结构缺陷，提升类型正确性和代码质量。

---

### T52 · 降低编译错误至 0（持续任务）
**状态**：`[~]` 进行中（2026-09-14 当前：6224 errors）  
**文件**：`codegen/instr.py`、`codegen/method.py`、`codegen/emitter.py`

**背景**：JDK 字节码翻译生成的 `jdk_classes` crate 存在大量编译错误，阻止整体流水线验证。目标是将 `cargo check` 错误降至 0。

**错误演变**：
| 时间 | 错误数 | 主要修复 |
|------|--------|---------|
| 2026-09-13 | 8251 | 起始 |
| 2026-09-14 早 | 7873 | 上一 session 结束 |
| 2026-09-14 | 7704 | clone→jvm_clone rename |
| 2026-09-14 | 7668 | bool→i32 类型转换 |
| 2026-09-14 | 7644 | 构造函数 return Ok(this) |
| 2026-09-14 | 7433 | Object::from_any 非基本类型 coerce |
| 2026-09-14 | 7275 | stub 与调用点类型一致 |
| 2026-09-14 | 7134 | invokevirtual null coerce |
| 2026-09-14 | 6928 | ldc class 常量 + 接口类型 coerce |
| 2026-09-14 | 6445 | aastore/areturn Object coerce，lcmp/ldiv 类型转换 |

**剩余主要错误类别（6445 errors）**：
- E0308 (2245)：Class<Object>/String/Object 双向、Vec<Class<Object>> vs Vec<Object>、i64/i32
- E0599 (2840)：Object 类型变量上调用具体方法（类型追踪缺失）
- E0609 (449)：字段找不到（继承字段缺失）
- E0425 (310)：变量找不到
- E0433 (152)：类型不在作用域
- E0061 (156)：参数数量错误

**下一步行动**：
1. 验证 `_coerce_icmp_operand` 修复（u16/i32 比较运算符，约 13 cases）
2. 分析 E0425 变量找不到根因（310 cases）
3. 分析 E0599 高频缺失方法（borrow/borrow_mut on Object 90 cases）

**验收**：`cd output && cargo check` 零错误。

---

### T53 · 修复 instanceof 语义错误
**状态**：`[ ]`  
**文件**：`codegen/instr.py`、`output/java_runtime/src/lib.rs`  
**优先级**：P1  
**依赖**：T52 编译错误降至合理水平（<2000）后更容易验证效果

**背景**：`instr.py` 中 `instanceof` 指令硬编码压入 `Lit("true")`，语义完全错误。任何依赖 `instanceof` 结果的分支都会产生错误的运行时行为，且无编译期警告（见计划文档 §5.1 和 §1.2）。

**当前错误代码**（instr.py）：
```python
elif op == 'instanceof':
    sim.push(Lit('true'), BOOL)
```

**实施步骤**：

1. `java_runtime/src/lib.rs`：为 `Object` 添加 `downcast_ref::<T>() -> Option<&T>` 方法：
   ```rust
   impl Object {
       pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
           self.0.downcast_ref::<T>()
       }
   }
   ```
   （`Object::from_any` 已用 `Rc<dyn Any>` 存储，可直接使用 `downcast_ref`）

2. `instr.py` 修改 `instanceof` 处理：
   ```python
   elif op == 'instanceof':
       obj_expr, obj_ty = sim.pop()
       if comment:
           check_rust = jvm_to_rust(comment.strip(), registry)
           result = RawExpr(f'({render_expr(obj_expr)}).downcast_ref::<{check_rust}>().is_some()')
       else:
           result = Lit('false')
       sim.push(result, BOOL)
   ```

3. 对接口类型的 `instanceof`，完整语义需依赖类层次图（T55），暂时返回 `false` 作为保守值。

**验收**：
- `obj instanceof String` 在 obj 确实是 String 时编译通过且运行时返回 `true`
- `instanceof` 结果驱动的分支行为正确

---

### T54 · 精确 mut 标注（扩展 _analyze_mutation 递归覆盖嵌套块）
**状态**：`[ ]`  
**文件**：`codegen/method.py`、`codegen/stack.py`  
**优先级**：P1

**背景**：`stack.py` 的 `store_local` 一律生成 `let mut`，导致所有局部变量都带无用 `mut`，Clippy 报大量 `unused_mut` 警告（当前靠全局 `#[allow(unused_mut)]` 压制）。

根本原因：`method.py` 的 `_analyze_mutation` 不递归进入 `IfStmt.then`、`IfStmt.else_`、`LoopStmt.body`，且对 `else_=None` 有 bug。

**实施步骤**：

1. 修复 `_analyze_mutation`（method.py）：
   ```python
   def _analyze_mutation(stmts):
       assigned = set()
       def collect(ss):
           for stmt in ss:
               if isinstance(stmt, AssignStmt) and isinstance(stmt.target, Var):
                   assigned.add(stmt.target.name)
               elif isinstance(stmt, IfStmt):
                   collect(stmt.then)
                   if stmt.else_:        # ← 修复 None 判断
                       collect(stmt.else_)
               elif isinstance(stmt, LoopStmt):
                   collect(stmt.body)
               # RawStmt 保守处理：不尝试解析字符串
       collect(stmts)
       # 对 first_decl 中未被赋值的 LetStmt，将 mutable 改为 False
   ```

2. 修改 `StackSim.store_local`（stack.py）：初始生成 `LetStmt(mutable=False)`，由 `_analyze_mutation` 在渲染前决定是否改为 `True`。

3. `_analyze_mutation` 在渲染前执行，根据 `assigned` 集合更新 `LetStmt.mutable`。

**验收**：
- 被二次赋值的变量仍为 `let mut`
- 未被赋值的变量降级为 `let`（不可变）
- `cargo check` 的 `unused_mut` 警告大幅减少

---

### T55 · 注解驱动的继承图：build.rs 扫描生成跨层 From impl
**状态**：`[ ]`  
**文件**：`output/jdk_classes/build.rs`（扩展）  
**优先级**：**P0（与 T78/T76 并列，依赖 T78 注解完整化先完成）**  
**方案文档**：`docs/plans/2026-09-14-annotation-driven-java-metadata.md`

**背景**：

> **设计变更（2026-09-14）**：原方案是在 Python 侧构建 ClassHierarchy 数据结构，
> 在转译时展平字段。新方案改为注解驱动：每个生成的 Rust 文件通过 `#[java_class(...)]`
> 注解承载完整 Java 元数据，build.rs 在编译时扫描这些注解来构建继承关系，
> 无需 Python 运行时数据结构。

每个 Rust 文件只标注自己直接声明的父类和接口（镜像 Java 声明），链路由遍历解析：

```
FileNotFoundException（super_class=IOException）
  → IOException（super_class=Exception）
    → Exception（super_class=Throwable）→ ...
```

**实施步骤**：

1. 依赖 T78（注解完整化）先完成，确保 `java_class` 注解包含 `super_class`/`interfaces`/`modifiers`

2. 扩展 `output/jdk_classes/build.rs`：
   - 遍历 `src/**/*.rs`，用正则提取所有 `#[java_class(...)]` 注解
   - 构建内存继承图 `HashMap<binary_name, (super_class, interfaces)>`
   - 对每对祖先-后代生成 `From<Child> for Ancestor` impl：
     ```rust
     // build.rs 写入 OUT_DIR/java_from_impls.rs
     impl From<FileNotFoundException> for Exception {
         fn from(v: FileNotFoundException) -> Exception { v._super.into() }
     }
     ```
   - `mod.rs` 中 `include!(concat!(env!("OUT_DIR"), "/java_from_impls.rs"))`

**为什么 build.rs 而非 proc-macro**：
- proc-macro 处理单个 item，无法访问其他文件的注解
- build.rs 在编译前运行，可扫描所有文件，构建全局继承图

**影响范围**：
- 跨层 From impl：`FileNotFoundException → Exception`、`→ Throwable` 等
- 消除 E0308 中"expected Ancestor, found Descendant"类型错误
- 为 T53（instanceof）提供 `is_subtype` 查询能力

**验收**：
- `FileNotFoundException` 可以直接 `.into()` 得到 `Throwable`
- E0308 中继承类型不匹配错误数量下降 50% 以上

---

### T56 · 实现 if/else 控制流结构恢复（CFG + 支配树）
**状态**：`[ ]`  
**文件**：`codegen/cfg.py`（扩展）、`codegen/method.py`（扩展）  
**优先级**：P2  
**依赖**：T52 编译错误降至 2000 以下后再实施，避免回归难以判断

**背景**：`cfg.py` 只做 back-edge goto 检测（识别 while 循环）和三指令布尔模式。`if/else` 完全没有结构化恢复，是当前所有覆盖率瓶颈的根本原因（见计划文档 §1.1 和 §四）。

**三阶段实施路线**：

**Step 1：基本块切割**（约 150 行，`cfg.py` 新增）
```python
@dataclass
class BasicBlock:
    id: int
    instrs: list[Instr]
    succs: list[int]   # 后继 BB id
    preds: list[int]   # 前驱 BB id

def build_basic_blocks(instrs: list[Instr]) -> list[BasicBlock]:
    # 识别 leader：方法入口 + 跳转目标 + 跳转后的下一条
    # 切割基本块，建立前驱/后继边
```

**Step 2：支配树计算**（约 60 行，迭代式算法）
```python
def compute_dominators(bbs: list[BasicBlock]) -> dict[int, int]:
    # 返回 {bb_id: idom_id}（immediate dominator）
    # Cooper et al. 迭代式算法
```

**Step 3：结构恢复**（约 200 行）
```python
def recover_structure(bbs, idom) -> list[StructNode]:
    # 识别自然循环（回边 n→h，h dom n）→ LoopStmt
    # 识别 if/else（条件跳转 → then_bb/else_bb → merge_bb）→ IfStmt
    # 识别 switch（tableswitch/lookupswitch）→ 新 MatchStmt IR 节点
```

每步完成后独立测试，不影响现有正常工作的代码路径。

**验收**：
- 含 if/else 的简单 Java 方法生成正确的 Rust `if/else`
- 原有 while 循环测试通过
- 嵌套 if/loop 结构不产生错误

---

### T57 · 实现 switch/tableswitch/lookupswitch 指令
**状态**：`[ ]`  
**文件**：`codegen/instr.py`、`codegen/method.py`（新 MatchStmt IR 节点）  
**优先级**：P3  
**依赖**：T56（最好在 CFG 基础上实现，正确处理 fallthrough）

**背景**：`tableswitch` 和 `lookupswitch` 指令只弹出 key，完全未生成 `match` 分支。这是语义级 bug——任何 switch 语句都静默产生错误行为（见计划文档 §5.1）。

**JVM 字节码格式**：
- `tableswitch`：连续整数范围 [low, high]，每个值对应一个跳转偏移
- `lookupswitch`：任意整数值 → 偏移的键值对列表

**目标生成结果**：
```rust
match key {
    0 => { /* case 0 */ }
    1 => { /* case 1 */ }
    _ => { /* default */ }
}
```

**实施步骤**：
1. `rs_ir.py` 新增 `MatchStmt` IR 节点：`arms: list[(pattern, body)]`，`default_body: list`
2. `classfile.py` 已解析 switch 操作数（`default +X, low Y, high Z`），在 `method.py` 中识别 switch 块边界
3. `instr.py` 中 `tableswitch`/`lookupswitch` 生成 `MatchStmt`
4. `render.py` 新增 `MatchStmt` 渲染（`match key { pattern => { ... } }`）

**验收**：
- 含 switch 语句的 Java 方法生成正确的 Rust `match`
- default 分支对应 `_` arm
- 运行结果与 Java 一致

---

### T58 · 类型系统 RsType 化（jvm_to_rust 返回 RsType 节点）
**状态**：`[ ]`  
**文件**：`codegen/type_map.py`、`codegen/instr.py`、`codegen/emitter.py`  
**优先级**：P3

**背景**：`type_map.py` 的 `jvm_to_rust` 返回 `str`，导致后续所有类型判断脆弱：

```python
# 当前脆弱写法（格式稍变即失效）
if rt.startswith('Rc<RefCell<Vec<'):
    elem = rt[len('Rc<RefCell<Vec<'):-3]  # 魔法切片

# 应改为结构化查询
if isinstance(ty, RsGeneric) and ty.outer == 'Rc':
    vec_ty = ty.params[0].params[0]  # RefCell<Vec<T>>
```

`rs_ir.py` 已定义完整的 `RsType` ADT，但 `type_map.py` 未打通。

**实施步骤（渐进替换）**：

1. `type_map.py` 新增 `jvm_to_rs_type(desc, registry) -> RsType`，与旧 `jvm_to_rust` 并行：
   ```python
   def jvm_to_rs_type(desc: str, registry=None) -> RsType:
       if desc == 'V': return RsPrimitive('()')
       if desc == 'I': return RsPrimitive('i32')
       if desc == 'J': return RsPrimitive('i64')
       if desc.startswith('['):
           elem = jvm_to_rs_type(desc[1:], registry)
           return RsGeneric('Rc', [RsGeneric('RefCell', [RsGeneric('Vec', [elem])])])
       cls = desc[1:-1].split('/')[-1].replace('$', '_') if desc.startswith('L') else desc
       return RsNamed(cls)
   ```

2. 将 `instr.py`、`emitter.py` 中高频字符串类型判断改为 `isinstance` 检查（渐进）

3. 最终弃用旧 `jvm_to_rust`（字符串版），所有调用点迁移到 `jvm_to_rs_type`

**验收**：
- 所有现有测试通过
- 生成代码与重构前一致（diff 为零）
- 消除所有 `startswith('Rc<RefCell<Vec<')` 形式的字符串比较

---

### T59 · 方法级 native 实现注入（细粒度覆盖机制）
**状态**：`[ ]`  
**文件**：`codegen/emitter.py`、`output/native_impls/`（目录约定）  
**优先级**：P3  
**来源**：参考 ruva 项目 `docs/plans/2026-09-14-codegen-improvement-plan.md` §1.3

**背景**：当前 `native_impls/` 以文件为粒度替换整个类（整类手写）。当一个类只有少数方法需要手写时，不得不维护整个类的所有方法，随着生成逻辑改进，手写版本与生成版本差距越来越大（见计划文档 §5.5）。

**目标**：引入**方法级注入协议**，允许只覆盖特定方法：

```
native_impls/
└── java/
    └── lang/
        └── string.rs   # 只包含需要手写的 java/lang/String 方法
```

`string.rs` 内容约定：
```rust
/// java/lang/String.intern:()Ljava/lang/String;
pub fn intern(&self) -> Result<String> {
    // 手写实现
}

/// @synthetic
pub fn helper_method(&self) -> String {
    // 注入 Java 中不存在的辅助方法
}
```

**实施步骤**：
1. `emitter.py` 在生成类文件时，扫描 `native_impls/{binary_name_snake}.rs`
2. 解析 `/// java/ClassName.methodName:descriptor` 注释头，识别覆盖方法
3. 对匹配方法：不生成 stub，而是生成 `// see native_impls/` 注释 + `pub use` 或直接 `include!`
4. 对 `@synthetic`：直接注入到 impl 块，不需要对应 Java 方法

**验收**：
- 只包含单个方法手写覆盖的 `native_impls/*.rs` 文件正确注入
- 其他方法仍由 codegen 自动生成
- 现有 `native_impls/java/lang/system.rs` 等整类覆盖仍向后兼容

---

### T60 · 表达式优先级括号优化
**状态**：`[ ]`  
**文件**：`codegen/render.py`、`codegen/rs_ir.py`  
**优先级**：P3  
**来源**：参考 ruva 项目 `docs/plans/2026-09-14-codegen-improvement-plan.md` §1.7

**背景**：`render.py` 生成的表达式不做优先级分析，复杂嵌套表达式可能产生语义错误代码（如 `a + b * c` 被生成为 `(a + b) * c`）。目前只靠大量保守括号规避，导致生成代码括号过多，可读性差。

**实施步骤**：
1. `rs_ir.py` 的 `BinOp` 节点添加优先级属性：
   ```python
   _BIN_OP_PREC = {
       '||': 1, '&&': 2, '|': 3, '^': 4, '&': 5,
       '==': 6, '!=': 6, '<': 7, '>': 7, '<=': 7, '>=': 7,
       '+': 10, '-': 10, '*': 11, '/': 11, '%': 11,
   }
   ```
2. `render.py` 中 `render_expr(BinOp)` 只在子表达式优先级低于父表达式时加括号
3. 同步修复 `RawExpr` 中手动拼接的过度括号（渐进）

**验收**：
- 生成代码中不出现 `(((a + b)))` 多余括号
- 不出现语义错误的优先级（如乘法被错误加括号变成加法先算）
- 现有所有测试通过（括号优化不改变语义）

---

### T61 · SSA 构建（架构级，长期目标）
**状态**：`[ ]`  
**优先级**：P4（高成本，依赖 T56 CFG）  
**依赖**：T56（需要先有完整 CFG）

**背景**：`StackSim` 在每条指令处理后立即生成 `LetStmt`，不同分支路径上相同 slot 的变量类型不一致，无合并（φ）逻辑。导致复杂分支结构下类型不稳定，产生大量编译错误。

**目标架构**：
- 引入 SSA（Static Single Assignment）构建阶段
- 每个 slot 在不同赋值点分配新的 SSA 变量（`v0`, `v1`, `v2`）
- 在控制流汇聚点插入 φ 函数确定合并类型
- 变量的最终 Rust 类型由 φ 函数操作数的公共超类型决定

**影响**：这是类型精确性的根本性改进，但工程量极大，且需要 T56（CFG）作为基础。

**验收**：
- 跨分支变量类型正确合并（`Object x = cond ? new Foo() : new Bar()` 中 x 类型为公共父类）
- 循环变量类型稳定

---

**阶段十任务依赖**：

```
T52 (降低编译错误) ─ 持续进行，独立
T53 (instanceof)   ─ 依赖 T52 (<2000 后验证更清晰)；完整语义依赖 T55
T54 (精确 mut)     ─ 独立，可立即开始
T55 (类层次图)     ─ 独立，可立即开始（先做父类字段注入的简化版）
T56 (if/else CFG)  ─ 依赖 T52 (<2000 后避免回归难判断)
T57 (switch)       ─ 依赖 T56（建议），或独立实现简化版
T58 (RsType 化)    ─ 独立，渐进替换，可与其他任务并行
T59 (方法级 native) ─ 独立，可立即开始
T60 (优先级括号)   ─ 独立，低风险，可随时处理
T61 (SSA)          ─ 依赖 T56，长期目标
```

**推荐执行序**：
- 立即：T52（持续）、T54（低成本 P1）、T59（独立 P3）
- 短期：T53（P1，依赖 T52 降至 <2000）、T55（P2，近期前置优化可立即做）
- 中期：T56（P2，CFG 结构恢复）、T58（P3，类型系统重构）
- 长期：T57（P3，switch）、T60（P3，括号）、T61（P4，SSA）

---

## 阶段十一：计划文档新增改进项（Phase 11 — Plan v2 追加）

> 本阶段任务来自 `docs/plans/2026-09-14-codegen-improvement-plan.md` 的更新版本（v2）  
> 新增内容：异常表（P0）、BFS 性能（P1）、Rust 关键字（P1）、并行处理（P3）、泛型精确化（P4）、消除 RawExpr（P5）

---

### T62 · 异常表解析与 try/catch/finally 结构恢复
**状态**：`[ ]`  
**文件**：`codegen/classfile.py`、`codegen/cfg.py`、`codegen/instr.py`、`codegen/method.py`  
**优先级**：P0（语义级 bug，任何含 try/catch 的方法均生成错误代码）

**背景**：`classfile.py` 的 `_parse_code_attribute` 解析 exception table 时完全跳过：

```python
exc_count = r.u2()
for _ in range(exc_count):
    r.skip(8)  # start_pc, end_pc, handler_pc, catch_type ← 全部丢弃
```

这导致所有含 `try/catch` 的 Java 方法的异常处理逻辑被静默丢弃，程序只走正常路径，运行时行为完全错误。JDK 大量方法（IO、反射、集合）都含 try/catch，影响范围极广。

**实施步骤**：

**Step 1：classfile.py 解析异常表**
```python
@dataclass
class ExceptionTableEntry:
    start_pc: int
    end_pc: int
    handler_pc: int
    catch_type: str | None  # None = finally（catch_type index == 0）

# ParsedMethod 新增字段
exception_table: list[ExceptionTableEntry]
```

修改 `_parse_code_attribute`，将 `r.skip(8)` 改为解析四元组并保存。

**Step 2：CFG 异常边建模**

在 `cfg.py` 的基本块构建阶段：
- 受保护区间 `[start_pc, end_pc)` 内的每个基本块，添加到 `handler_pc` 的**异常边**
- `handler_pc` 对应的基本块标记为 `is_handler = True`，记录 `catch_type`

**Step 3：结构恢复生成 try/catch**

识别受保护区间后，生成以下 IR 结构：
```python
@dataclass
class TryCatchStmt:
    try_body: list[Stmt]
    handlers: list[tuple[str | None, str, list[Stmt]]]
    # [(catch_type, var_name, handler_body), ...]
    # catch_type=None 表示 finally
```

**Step 4：render.py 渲染**

Java try/catch 在 Rust 侧映射到：
```rust
// try/catch → match 风格的错误处理（基于 Result 传播）
let result = (|| -> Result<()> {
    /* try body */
    Ok(())
})();
match result {
    Ok(_) => {}
    Err(e) if e.is::<IOException>() => { /* catch body */ }
    Err(e) => return Err(e),  // 未捕获的异常继续传播
}
```

**阶段性验收**：
- Step 1 完成后：`ParsedMethod.exception_table` 非空，日志可打印各方法的异常表
- Step 2 完成后：CFG 中异常边正确建立，handler_pc 块被标记
- Step 3 完成后：生成 TryCatchStmt IR，不再丢弃异常逻辑
- Step 4 完成后：`cargo check` 中涉及 try/catch 的 E0308/E0425 错误减少

---

### T63 · BFS 队列改用 deque（O(n²) → O(n)）
**状态**：`[ ]`  
**文件**：`codegen/transpile.py`  
**优先级**：P1（一行改动，零风险）

**背景**：`transpile.py` 的 `_discover_jdk_classes_method_level` 使用 Python list 作为 BFS 队列，每次 `pop(0)` 是 O(n) 操作，导致整体 BFS 时间复杂度为 O(n²)。当 JDK 类数量达到 500+ 时，时间从约 25ms 退化到数秒。

**当前代码**：
```python
queue: list[tuple[str, str, str]] = []
...
cls, meth, desc = queue.pop(0)  # O(n)！
```

**修复**：
```python
from collections import deque
queue: deque[tuple[str, str, str]] = deque()
...
cls, meth, desc = queue.popleft()  # O(1)
```

同时检查 `transpile.py` 中其他使用 `list.pop(0)` 的 BFS/队列模式，统一替换。

**验收**：
- `python3 scripts/main.py` 输出中，BFS 阶段耗时不随类数量平方增长
- 生成代码与修复前完全一致（diff 为零）

---

### T64 · Rust 关键字转义（safe_ident 完整覆盖）
**状态**：`[ ]`  
**文件**：`codegen/emitter.py`、`codegen/type_map.py`、`codegen/instr.py`  
**优先级**：P1（可能正在产生 E0532 编译错误）

**背景**：Java 允许使用 Rust 保留关键字作为变量名、字段名、方法名（如 `type`、`ref`、`match`、`impl`、`use`、`in`、`abstract`、`yield`）。若代码生成不处理，生成的 `.rs` 文件出现 `error[E0532]: expected identifier, found keyword`。

特殊情况：`self`/`Self` 不能用 `r#self`，必须重命名为 `self_`。

**实施步骤**：

1. 在 `emitter.py`（或 `type_map.py`）维护完整关键字集合：
   ```python
   RUST_KEYWORDS = frozenset({
       # 严格关键字
       'as', 'async', 'await', 'break', 'const', 'continue', 'crate',
       'dyn', 'else', 'enum', 'extern', 'false', 'fn', 'for', 'if',
       'impl', 'in', 'let', 'loop', 'match', 'mod', 'move', 'mut',
       'pub', 'ref', 'return', 'self', 'Self', 'static', 'struct',
       'super', 'trait', 'true', 'type', 'unsafe', 'use', 'where', 'while',
       # 保留关键字（不稳定/未来）
       'abstract', 'become', 'box', 'do', 'final', 'macro', 'override',
       'priv', 'try', 'typeof', 'unsized', 'virtual', 'yield',
   })
   
   def safe_ident(name: str) -> str:
       if name in ('self', 'Self'):
           return name + '_'
       if name in RUST_KEYWORDS:
           return 'r#' + name
       return name
   ```

2. 将 `safe_ident` 应用到所有生成标识符的位置：
   - `emitter.py`：方法名生成、字段名生成、参数名生成
   - `instr.py`：局部变量名（`store_local` slot 命名）
   - `emitter.py`：模块名（将 Java package 路径转换为 Rust module 路径时）

3. 检查当前 `cargo check` 错误中是否有 E0532（keyword 冲突），优先修复。

**验收**：
- Java 中名为 `type`、`ref`、`match` 的字段/方法/参数正确生成为 `r#type`、`r#ref`、`r#match`
- Java 中名为 `self` 的参数正确生成为 `self_`
- 无新增 E0532 错误

---

### T65 · 并行类解析与代码生成
**状态**：`[ ]`  
**文件**：`codegen/transpile.py`、`codegen/emitter.py`  
**优先级**：P3

**背景**：当前翻译流水线完全串行：javac → 解析 → BFS 发现 → 代码生成。处理 `java.base` 全量 800+ 类时，耗时随类数量线性增长，无法利用多核 CPU。

**实施步骤**：

**阶段一：类解析并行化**
```python
from concurrent.futures import ProcessPoolExecutor

with ProcessPoolExecutor() as executor:
    class_infos = list(executor.map(_parse_class_file, class_paths))
```

注意：`_parse_class_file` 必须是纯函数（不依赖全局可变状态），且返回可 pickle 的对象。

**阶段二：代码生成并行化**
```python
with ThreadPoolExecutor() as executor:
    # 各类之间无数据依赖（除了 registry 只读）
    futures = [executor.submit(_gen_class_rs, ci, registry) for ci in class_infos]
    results = [f.result() for f in futures]
```

注意：`_scan_native_impls` 必须在并行生成前完成，其结果以只读方式传入。

**前置条件**：T63（BFS deque）先完成；全局可变状态（registry、native_impls_map）需在并行前固化为只读。

**预期收益**：800 类生成时间从 ~60s 降至 ~15s（4 核）。

**验收**：
- 并行生成的代码与串行生成结果完全一致（diff 为零）
- 处理 800 类时，墙钟时间降低 50%+

---

### T66 · 泛型类型精确化（LocalVariableTypeTable 驱动）
**状态**：`[ ]`  
**文件**：`codegen/sig_parser.py`、`codegen/stack.py`、`codegen/instr.py`  
**优先级**：P4  
**依赖**：T61（SSA）为完整方案；阶段一可独立实施

**背景**：`sig_parser.py` 将所有带泛型参数的类型简化为 `Object`：

```python
if has_type_args:
    rust_type = 'Object'  # 带泛型参数的类类型暂时简化为 Object
if c == '[':
    return 'Object', next_i  # 数组也简化
```

导致泛型集合操作（`list.get(i)` 应返回具体元素类型）退化为 `Object`，生成大量无必要的 downcast。

**阶段一：LVTT 驱动（独立可实施）**

`classfile.py` 已解析 `LocalVariableTypeTable`，`ParsedMethod.local_types` 中存有各 slot 的完整泛型签名（如 `Ljava/util/List<Ljava/lang/String;>;`）。

修改 `StackSim.store_local`（stack.py）：
```python
def store_local(self, idx, expr, ty):
    if idx in self.method.local_types:
        # 优先使用 LVTT 中的精确泛型类型
        precise_ty = parse_generic_sig(self.method.local_types[idx])
        ty = precise_ty
    ...
```

**阶段二：调用站点泛型替换（依赖 T61 SSA）**

记录每个 SSA 变量的声明类型（含泛型参数），调用泛型方法时用接收者的具体类型参数替换被调方法的 TypeVar（如 `List<String>.get(0)` 返回 `String` 而非 `Object`）。

**验收**：
- 阶段一：`List<String>` 类型的局部变量被正确推断为含泛型，不退化为 `Object`
- 阶段二：`list.get(0)` 返回类型由 `Object` 精化为 `String`（当 LVTT 信息可用时）

---

### T67 · 消除 RawExpr / RawStmt（IR 结构化）
**状态**：`[ ]`  
**文件**：`codegen/rs_ir.py`、`codegen/instr.py`、`codegen/method.py`、`codegen/render.py`  
**优先级**：P5（持续改进，非阻塞）

**背景**：`rs_ir.py` 定义了完整的 IR ADT，但 `instr.py` 和 `method.py` 大量使用 `RawExpr(f"...")` 和 `RawStmt(f"...")` 逃生舱，导致：
- `_analyze_mutation` 无法分析 RawStmt 中的赋值（保守处理，可能漏标 `mut`）
- 无法对表达式做优先级分析、类型推导
- 重构时容易遗漏 raw 字符串中的内容

**目标**：将每类 `RawExpr` 用途替换为对应的结构化 IR 节点。

**替换映射**（`rs_ir.py` 已有或需新增的节点）：

| RawExpr 用途 | 目标 IR 节点 |
|---|---|
| 方法调用 `obj.method(args)` | `MethodCall(recv, name, args)` |
| 静态调用 `Type::method(args)` | `StaticCall(type, name, args)` |
| 数组下标 `arr[i]` | `Index(arr, idx)` |
| 类型转换 `(expr as T)` | `Cast(expr, ty)` |
| 字段访问 `obj.field` | `FieldAccess(obj, field)` |
| 条件表达式 `if c { a } else { b }` | `IfExpr(cond, then, else_)` |

**实施策略**：
1. 每次修改 instr.py 时，优先使用结构化节点而非 `RawExpr`
2. 建立 `RawExpr` 计数追踪（在 `render.py` 中统计），每个版本目标递减
3. 对"真正无法结构化"的情况（如属性宏、unsafe 块），允许保留 `RawExpr` 并加注释说明原因

**验收**：
- `grep -r "RawExpr\|RawStmt" codegen/instr.py | wc -l` 每个 milestone 递减
- 移除某类 RawExpr 后，`_analyze_mutation` 能正确分析该类语句中的赋值
- 无功能回归（cargo check 错误不增加）

---

**阶段十一任务依赖**：

```
T62 (异常表)     ─ 独立 Step 1/2 可立即开始；Step 3/4 依赖 T56（CFG）
T63 (BFS deque) ─ 独立，立即可做，一行改动
T64 (Rust 关键字) ─ 独立，立即可做，可能修复现有 E0532 错误
T65 (并行处理)   ─ 依赖 T63（deque）先做；全局状态固化后再并行
T66 (泛型精确化) ─ 阶段一独立；阶段二依赖 T61（SSA）
T67 (消除 Raw)   ─ 持续改进，与其他任务并行
```

**推荐执行序**：
- 立即（低成本高收益）：T63（一行）、T64（关键字转义，可能有现存 E0532 错误）
- 短期：T62 Step 1/2（异常表解析，P0），T62 Step 3/4 依赖 T56
- 中期：T66 阶段一（LVTT 驱动泛型，独立可做）
- 长期：T65（并行处理，待代码稳定后）、T66 阶段二（依赖 SSA）、T67（持续）

---

## 阶段十二：JNC 计划参考项（Phase 12 — JNC Reference）

> 来源：`/Users/yuwei/dev/workspace/jnc/docs/plans/2026-09-14-codegen-improvement-plan.md`  
> 新增洞察：JVM slot 复用根因分析（R2）、虚方法 BFS 完整性（R3）

---

### T68 · JVM slot 复用与变量作用域追踪（E0425 根因修复）
**状态**：`[ ]`  
**文件**：`codegen/stack.py`、`codegen/method.py`  
**优先级**：P1（当前 310 个 E0425 错误的主要根因）

**背景**：JVM 局部变量槽（slot）允许在不同词法作用域中复用同一编号。例如：

```java
for (int i = 0; i < n; i++) {
    String s = items[i];  // slot 2 = s
}
// 循环结束后 slot 2 被复用
for (int i = 0; i < m; i++) {
    int x = values[i];    // slot 2 = x（与上面的 s 同一编号）
}
```

当前 `StackSim.store_local` 对同一 slot 的第二次赋值生成 `AssignStmt`（`v2 = ...`），但如果该 slot 对应的 `LetStmt`（`let mut v2: String`）已在第一个循环内作用域被声明，第二个循环中的 `v2` 引用就会产生 E0425（找不到变量）或 E0308（类型不匹配）。

**根本原因**：`store_local` 不追踪当前嵌套深度，无法区分"同一作用域内的重新赋值"和"新作用域内对同一 slot 编号的复用"。

**实施步骤**：

**Step 1：追踪变量声明的作用域深度**

在 `StackSim`（stack.py）中增加深度追踪：
```python
self._slot_decl_depth: dict[int, int] = {}   # slot → 声明时的嵌套深度
self._current_depth: int = 0                  # 当前嵌套层级
```

进入 `LoopStmt`/`IfStmt` 体时深度 +1，退出时深度 -1。

**Step 2：slot 复用时生成新 LetStmt**

修改 `store_local`（stack.py）：
```python
def store_local(self, idx, expr, ty):
    decl_depth = self._slot_decl_depth.get(idx)
    if decl_depth is None:
        # 首次声明：生成 LetStmt
        self._slot_decl_depth[idx] = self._current_depth
        self.emit(LetStmt(name=f'v{idx}', ty=ty, value=expr, mutable=True))
    elif decl_depth == self._current_depth:
        # 同作用域：生成 AssignStmt（重新赋值）
        self.emit(AssignStmt(target=Var(f'v{idx}'), value=expr))
    else:
        # 不同作用域：slot 复用，用 Rust shadowing（新 LetStmt）
        self._slot_decl_depth[idx] = self._current_depth
        self.emit(LetStmt(name=f'v{idx}', ty=ty, value=expr, mutable=True))
```

Rust 的变量遮蔽（shadowing）语义天然支持这个模式：同名 `let mut v2` 可以声明多次，各自独立作用域。

**Step 3：method.py 通知深度变化**

在 `method.py` 的 `LoopStmt`/`IfStmt` 生成点，调用 `sim.enter_scope()` / `sim.exit_scope()`：
```python
sim.enter_scope()   # depth += 1
# 生成循环体指令
sim.exit_scope()    # depth -= 1，清理该深度的 _slot_decl_depth 记录
```

**特殊情况**：
- `iinc` 指令（`i++`）直接修改 slot，不经过 `store_local`，需要单独处理（生成 `v2 += 1`，不创建新 LetStmt）
- 参数 slot（slot 0 = this，slot 1-N = 参数）永远不会在同一方法内被"复用"，不需要深度追踪

**验收**：
- 含两个连续 for 循环且各自 slot 复用的 Java 方法生成合法 Rust（无 E0425）
- E0425 错误数从 310 下降（预期降低 50%+）
- 原有循环变量正确性不退化

---

### T69 · 虚方法 BFS 完整性（invokevirtual 展开子类实现）
**状态**：`[ ]`  
**文件**：`codegen/transpile.py`、`codegen/hierarchy.py`（T55 依赖）  
**优先级**：P2  
**依赖**：T55（ClassHierarchy）完成后才能枚举已知实现类

**背景**：当前方法级调用链 BFS 只追踪静态可达路径。对于 `invokevirtual` 指令，字节码记录的是声明类（如 `AbstractList.add`），而运行时实际调用的是子类实现（如 `ArrayList.add`）。若子类实现未被 BFS 发现，翻译后子类方法保持为 `panic!("stub: ...")`，运行时崩溃。

**示例**：
```java
List<String> list = new ArrayList<>();
list.add("hello");  // invokevirtual List.add
                    // 实际调用 ArrayList.add（但 BFS 只发现了 AbstractList.add）
```

**实施步骤**：

**Step 1：在 BFS 处理 invokevirtual 时展开子类**

修改 `transpile.py` 的 `_discover_jdk_classes_method_level`，在入队 `invokevirtual` 目标时：
```python
if instr.opcode == 'invokevirtual':
    # 入队声明类中的方法（现有行为）
    queue.append((decl_class, method_name, descriptor))
    
    # 新增：通过 ClassHierarchy 找到所有已知子类的同名方法，一并入队
    if hierarchy:
        for impl_class in hierarchy.concrete_subclasses(decl_class):
            if hierarchy.overrides(impl_class, method_name, descriptor):
                queue.append((impl_class, method_name, descriptor))
```

**Step 2：invokeinterface 同理**

`invokeinterface` 的情况更典型：接口声明类（如 `Iterable`）本身没有实现，必须展开所有已知实现类（`ArrayList`、`LinkedList` 等）。

**Step 3：BFS 深度限制**

子类展开可能引入大量新类（每个接口有数十个实现类），需要：
- 只展开当前 workspace 中已解析到的类（不无限延伸）
- 保留现有的 `MAX_BFS_DEPTH` 或等效限制

**验收**：
- `ArrayList.add` 在 `list.add("hello")` 调用链上被发现并翻译（不再是 stub）
- BFS 生成的类数量合理增加（不爆炸式增长）
- `cargo check` 中相关 E0599（方法找不到）数量下降

---

**阶段十二任务依赖**：

```
T68 (slot 复用)   ─ 独立，可立即开始；不依赖其他任务
T69 (BFS 虚方法) ─ 依赖 T55（ClassHierarchy），需要 concrete_subclasses() API
```

**推荐执行序**：
- 立即：T68（P1，独立可做，可能修复大量 E0425）
- 中期：T69（P2，等 T55 ClassHierarchy 完成后）

---

## 阶段十三：多文档 Gap 补全（Phase 13 — Cross-Plan Gap Fill）

> 来源：对 ruva 计划（§1.5 / §5.1 / §5.4）+ JNC 计划（I-4）与 T01-T69 全量对照后的剩余 gap  
> T71（注册表键规范化）是隐蔽性最高的架构正确性 bug，T70（布尔压缩）是代码质量，T72（语义桩追踪）是可维护性基础设施

---

### T70 · 布尔值 i32 模式压缩（`if c {1i32} else {0i32}` → bool 表达式）
**状态**：`[ ]`  
**文件**：`codegen/method.py`、`codegen/instr.py`、`codegen/stack.py`  
**优先级**：P2  
**来源**：ruva 计划 §1.5 + JNC 计划 I-4

**背景**：JVM 用 `i32`（0/1）表示 boolean，Java 编译器在需要将 boolean 表达式结果存入变量时，会生成以下字节码模式：

```
; 源码：boolean flag = (a > b);
if_icmple <false_label>   ; a <= b 时跳转
iconst_1                  ; push 1（true）
goto <end_label>
false_label: iconst_0     ; push 0（false）
end_label: istore <slot>
```

当前 java_rta 将这个模式直接翻译为：
```rust
let mut v5: i32 = if v1 > v2 { 1i32 } else { 0i32 };
```

而实际上应该生成：
```rust
let mut v5: bool = v1 > v2;
```

同样的问题出现在 `ifeq`/`ifne` 将 bool 存回 i32，以及方法调用返回 bool 时的 0/1 转换。

**影响**：
- 生成代码充满 `!= 0i32`、`== 0i32` 噪音（每次使用 bool 变量时都要转换）
- 类型推导链中断：变量被推断为 `i32` 而非 `bool`，导致后续比较产生类型不匹配
- 可读性极差，难以人工审查

**实施步骤**：

**Step 1：cfg.py 中扩展现有布尔模式识别**

`cfg.py` 已有三指令布尔模式检测（`bool_cond_map`）。在此基础上，识别四指令模式：
```
if_icmpXX <false>   或  ifXX <false>
iconst_1
goto <end>
false: iconst_0     (或 iconst_0 / goto / iconst_1 的镜像)
end: ...
```

检测逻辑：
```python
def detect_bool_pattern(instrs, idx) -> tuple[str, int] | None:
    """
    返回 (cond_expr_str, consumed_count) 或 None。
    cond_expr_str: 如 "v1 > v2"（从 if_icmpXX 推导）
    consumed_count: 消耗的指令数（3 或 4）
    """
```

**Step 2：StackSim 类型追踪**

识别到布尔模式后，推入 `(BoolExpr, BOOL)` 而非 `(Lit('1i32'), I32)` 或 `(IfExpr(...), I32)`。这样后续 `istore` 生成 `let v5: bool = ...` 而非 `let v5: i32 = ...`。

**Step 3：BOOL 类型在比较中的使用**

当 `ifeq`/`ifne` 的操作数类型已知为 `BOOL` 时，直接使用 `if expr { ... }` 或 `if !expr { ... }`，不再生成 `if expr != 0 { ... }`。

**Step 4：方法返回 bool 的处理**

`jvm_to_rust` 中 `Z` 描述符已映射为 `bool`，确保方法调用结果被压入 `BOOL` 类型而非 `I32`，消除 `result != 0` 转换。

**验收**：
- 含 `boolean flag = (a > b)` 的 Java 方法生成 `let mut flag: bool = a > b;`，无 `i32`/`!= 0` 噪音
- 含 `if (obj.isEmpty())` 的代码生成 `if obj.is_empty()` 而非 `if obj.is_empty() != 0`
- E0308 中的 `expected bool, found i32`（或反向）错误数量减少

---

### T71 · 内部类名注册表键规范化（防止静默查找失败）
**状态**：`[ ]`  
**文件**：`codegen/type_map.py`、`codegen/emitter.py`、`codegen/transpile.py`  
**优先级**：P1（静默失败，极难排查）  
**来源**：ruva 计划 §5.1

**背景**：Java 内部类名有两种合法形式：

| 形式 | 示例 | 使用场景 |
|------|------|---------|
| JVM 二进制格式 | `java/util/AbstractMap$SimpleEntry` | `.class` 文件、字节码注释、BFS 队列 |
| Rust 安全标识符 | `AbstractMap__SimpleEntry` | 生成的 `.rs` 文件、`use` 语句 |

**当前风险**：`$` → `__` 转换（`replace('$', '__')` 或 `replace('$', '_')`）在代码中的多个位置独立发生，时机不一致：

```python
# type_map.py 中某处
cls = cls.replace('$', '__')  # 提前转换

# emitter.py 中另一处  
name = binary.split('/')[-1]  # 未转换，含 $

# registry 查找时
info = registry.get(cls_with_dollar)   # 用 $ 形式查找
info = registry.get(cls_with_underscore)  # 用 __ 形式查找，取决于注册时的形式
```

若注册表以 `$` 格式存储键，而查找时用 `__` 格式（或反之），`registry.get(...)` 返回 `None`，触发 fallback（生成 `Object` 或跳过），产生**静默的语义错误**，完全没有任何警告或异常。

这类 bug 极难排查：编译错误消息只显示最终症状（类型不匹配、方法找不到），根因是两处相隔很远的命名形式不一致。

**实施步骤**：

**Step 1：审查并确立规范**

确定注册表键的权威形式（推荐：JVM 二进制格式，即含 `$`，如 `java/util/AbstractMap$SimpleEntry`）：
- `registry`：始终以 JVM 二进制格式为键
- `$` → `__` 的转换**只在最终生成 Rust 标识符时**进行，其他任何地方不做此转换

**Step 2：定义规范化入口函数**

```python
# type_map.py
def jvm_binary_name(name: str) -> str:
    """确保是 JVM 二进制格式（斜杠分隔，$ 内部类），不做任何 $ 转换。
    这是注册表查找的唯一合法输入形式。"""
    return name.replace('.', '/')  # 只处理点→斜杠，绝不改 $

def rust_ident_name(binary_name: str) -> str:
    """JVM 二进制格式 → Rust 标识符（最后一步转换，只在生成文件时调用）"""
    return binary_name.split('/')[-1].replace('$', '__')
```

**Step 3：扫描代码中所有 `replace('$', ...)` 调用**

```bash
grep -rn "replace('\$'" codegen/
grep -rn 'replace("\$"' codegen/
```

对每处调用，判断：
- 是否在"最终生成标识符"时？→ 保留，改用 `rust_ident_name`
- 是否在"查找/比较"时？→ 删除，使用原始 JVM 格式

**Step 4：注册表查找断言（调试期）**

在开发期加入断言，确保查找键不含 `__`：
```python
def registry_get(registry, key):
    assert '__' not in key, f"registry 查找使用了 Rust 格式键: {key!r}，应使用 JVM 格式"
    return registry.get(key)
```

**验收**：
- `grep -rn "replace('\$'" codegen/` 所有调用点都有注释说明是 JVM 还是 Rust 形式
- 含内部类的 Java 类（如 `Map.Entry`、`Thread.State`）正确翻译，不产生 fallback Object
- 在涉及内部类的场景下，`cargo check` 中无新增 E0433/E0412 错误

---

### T72 · 语义桩显式追踪（未实现指令统一标记与计数）
**状态**：`[ ]`  
**文件**：`codegen/instr.py`、`codegen/emitter.py`、`scripts/main.py`  
**优先级**：P2  
**来源**：ruva 计划 §5.4 + java_rta 改进计划 §5.1

**背景**："语义桩"是指以看似合理但实际语义错误的代码替代未实现功能，而非显式 `todo!()` 或编译错误。java_rta 当前已知的语义桩：

| 指令 | 当前行为 | 正确行为 |
|------|---------|---------|
| `instanceof` | 硬编码返回 `true` | 运行时类型检查（T53） |
| `tableswitch` | 只弹出 key，无 match 分支 | 生成 `match` 语句（T57） |
| `lookupswitch` | 只弹出 key，无 match 分支 | 生成 `match` 语句（T57） |
| `try/catch` | 异常表被 skip(8) 完全丢弃 | 结构化异常处理（T62） |
| `invokedynamic` | 未知处理 | Lambda 翻译（T49） |
| `monitorenter/exit` | 未知处理 | 忽略或映射到 Mutex |

**问题**：
1. **无法统计**：不知道代码库中有多少处语义桩，无法追踪进度
2. **无法检测**：语义桩在运行时静默产生错误行为，没有任何警告
3. **易被遗忘**：硬编码 `true`/`false` 等值看起来"合法"，代码审查很容易漏掉

**实施步骤**：

**Step 1：定义语义桩生成函数**

在 `instr.py` 中，将所有语义桩替换为统一的辅助函数：
```python
def _semantic_stub(sim, opcode: str, description: str, stub_value=None, stub_ty=None):
    """生成语义桩，并向 sim 注册该 stub 信息（用于统计和审计）。"""
    sim.record_stub(opcode, description)  # 记录到 sim 的 stub 列表
    if stub_value is not None:
        sim.push(stub_value, stub_ty)
    else:
        # 生成 todo!() 表达式（在 Rust 中会 panic，不静默）
        sim.push(RawExpr(f'todo!("stub: {opcode} in {{}}", file!())'), RsNamed('!'))
```

**Step 2：StackSim 增加 stub 记录**

```python
class StackSim:
    def __init__(self, ...):
        ...
        self.stubs: list[tuple[str, str]] = []  # [(opcode, description)]
    
    def record_stub(self, opcode, description):
        self.stubs.append((opcode, description))
```

**Step 3：emitter.py 生成桩统计注释**

在每个翻译的方法头部，若 `sim.stubs` 非空，生成注释：
```rust
// SEMANTIC_STUBS: instanceof(line 42), tableswitch(line 67)
pub fn some_method(&self) -> Result<i32> {
    ...
}
```

**Step 4：scripts/main.py 生成汇总报告**

代码生成完成后，汇总所有方法中的 stub 记录：
```python
# 输出 stub 汇总
stub_summary = {}
for cls, method, stubs in all_stubs:
    for opcode, desc in stubs:
        stub_summary[opcode] = stub_summary.get(opcode, 0) + 1

print("=== Semantic Stubs Summary ===")
for opcode, count in sorted(stub_summary.items(), key=lambda x: -x[1]):
    print(f"  {opcode}: {count} occurrences")
print(f"  Total: {sum(stub_summary.values())} stubs")
```

**Step 5：将现有硬编码语义桩迁移到 `_semantic_stub`**

- `instanceof`: `sim.push(Lit('true'), BOOL)` → `_semantic_stub(sim, 'instanceof', 'hardcoded true')`
- `tableswitch`/`lookupswitch`: 只弹栈 → `_semantic_stub(sim, 'tableswitch', 'no match generated')`

**目标**：每个 Release 版本，语义桩总数应单调递减。

**验收**：
- `python3 scripts/main.py` 输出末尾显示 stub 汇总表
- `instanceof`、`tableswitch` 等已知桩出现在汇总中，数量可追踪
- 不存在硬编码 `Lit('true')` / `Lit('false')` 用于表示未实现指令结果的情况

---

**阶段十三任务依赖**：

```
T70 (布尔压缩)     ─ 独立，可立即开始；cfg.py 布尔模式识别基础上扩展
T71 (键规范化)     ─ 独立，可立即开始；优先做 Step 3 扫描（低风险）
T72 (语义桩追踪)   ─ 独立，可立即开始；Step 1-3 不改变生成结果，只增加统计
```

**推荐执行序**：
- 立即（低风险高价值）：T71 Step 3（扫描 replace('$') 调用点，判断正误）、T72 Step 4（main.py 汇总报告）
- 短期：T70（布尔压缩，改善代码质量，减少 E0308）、T71 Step 1-2（规范化函数）
- 中期：T72 Step 5（将现有桩迁移到统一接口）

---

## 阶段十四：跨项目历史教训（来源：JNC/ruva 任务文档）

> 来源：`/Users/yuwei/dev/workspace/jnc/docs/TASKS.md`、`/Users/yuwei/dev/workspace/ruva/docs/task-history.md`、`/Users/yuwei/dev/workspace/ruva/docs/task.md`
>
> 这三份文档是 JNC（Java→Rust transpiler v2）和 ruva（Ruby→Rust transpiler）的任务历史。
> 它们已经历了完整的开发周期，踩过大量坑并修复。
> 以下任务是在 java_rta 中**提前规避**这些已知问题的行动项。

---

### T73 · 整数算术溢出语义修复（`wrapping_add/sub/mul`）

**状态**：`[ ]`  
**文件**：`codegen/instr.py`  
**优先级**：P1（正确性 bug，Java 与 Rust 行为不同，release 编译才会出现差异）  
**来源**：ruva `SEM-1`（task-history.md）、ruva `PH1-11`（task.md）

**背景**：

Java 的整数运算（`int`、`long`）语义是 **wrap-around（模 2^32/2^64）**，溢出时回绕，不会报错。  
Rust 在 **debug 模式**下整数溢出会 **panic**；release 模式下虽然回绕，但行为与 debug 不一致。

当前 java_rta 生成形如：
```rust
let _v3 = _v1 + _v2;
```
这在 debug 模式下运行到溢出场景时会 panic（例如 JDK `String.hashCode()`、`HashMap` 哈希计算等都依赖整数溢出）。

ruva 项目在 `SEM-1` 任务中修复了全部整数运算，将所有 `+/-/*` 替换为 `wrapping_add/sub/mul`，并将这一修复纳入 P1 优先级。这是生成代码语义正确性的根本保证之一。

**实施步骤**：

**Step 1：instr.py 所有整数二元运算改为 wrapping 版本**

定位 `iadd`、`isub`、`imul`、`ladd`、`lsub`、`lmul` 等算术指令的生成代码：

```python
# 修改前
case 'iadd': _gen_binop(sim, '+', I32)
case 'isub': _gen_binop(sim, '-', I32)
case 'imul': _gen_binop(sim, '*', I32)
case 'ladd': _gen_binop(sim, '+', I64)
case 'lsub': _gen_binop(sim, '-', I64)
case 'lmul': _gen_binop(sim, '*', I64)

# 修改后（对应 _gen_wrapping_binop）
case 'iadd': _gen_wrapping_binop(sim, 'wrapping_add', I32)
case 'isub': _gen_wrapping_binop(sim, 'wrapping_sub', I32)
case 'imul': _gen_wrapping_binop(sim, 'wrapping_mul', I32)
case 'ladd': _gen_wrapping_binop(sim, 'wrapping_add', I64)
case 'lsub': _gen_wrapping_binop(sim, 'wrapping_sub', I64)
case 'lmul': _gen_wrapping_binop(sim, 'wrapping_mul', I64)
```

新增辅助函数：
```python
def _gen_wrapping_binop(sim: StackSim, method: str, ty: RsType):
    b, _ = sim.pop()
    a, _ = sim.pop()
    v = sim.fresh()
    sim.emit(RawStmt(f"let {v} = {a}.{method}({b});"))
    sim.push(Var(v), ty)
```

**Step 2：整数除法与取模（保留标准运算符）**

`idiv`、`irem`、`ldiv`、`lrem` 除法在 Java 中会抛 `ArithmeticException`，Rust 会 panic（除零），语义一致，无需 wrapping 版本。但需确保保留现有的除零错误传播。

**Step 3：一元取反不需改动**

`ineg`/`lneg` 生成 `-_v1`，在 Rust 中 `i32::MIN.wrapping_neg() == i32::MIN`，语义与 Java 一致。可选：改为 `wrapping_neg()` 以明确意图。

**Step 4：移位运算屏蔽高位**

Java 的 `ishl`、`ishr`、`iushr`（及 long 版本）对移位量用 `& 0x1f`（`& 0x3f` for long）屏蔽。Rust `<<`/`>>` 在超出位宽时是 undefined behavior（debug 下 panic）。

修改生成代码：
```rust
// Java ishl x, n → Rust
let _v3 = _v1.wrapping_shl((_v2 & 0x1f) as u32);
// Java ishr x, n → Rust (算术右移)
let _v3 = _v1.wrapping_shr((_v2 & 0x1f) as u32);
// Java iushr x, n → Rust (逻辑右移)
let _v3 = ((_v1 as u32).wrapping_shr((_v2 & 0x1f) as u32)) as i32;
```

**验收**：
- 生成的 Rust 代码中 `iadd/isub/imul/ladd/lsub/lmul` 全部对应 `wrapping_*` 方法调用
- `cargo check` 不引入新错误
- `ishl/ishr/iushr` 生成带 `& 0x1f` 屏蔽的 `wrapping_shl/shr`

---

### T74 · `$assertionsDisabled` 合成字段识别

**状态**：`[ ]`  
**文件**：`codegen/instr.py`（`getstatic` 处理）、`codegen/emitter.py`（字段生成）  
**优先级**：P2（正确性 bug，JDK 断言机制产生静默错误行为）  
**来源**：ruva `SEM-4`（task-history.md）

**背景**：

Java 编译器在任何使用 `assert` 关键字的类中自动插入一个合成字段：
```java
static final boolean $assertionsDisabled;
static {
    $assertionsDisabled = !ClassName.class.desiredAssertionStatus();
}
```
该字段用于控制 `assert` 语句是否执行：
```java
if (!$assertionsDisabled) {
    if (!(condition)) throw new AssertionError(...);
}
```

在生成的 Rust 代码中，若 `getstatic $assertionsDisabled` 被翻译为读取一个正常字段，则：
- 字段默认值为 `false`（断言未禁用 → 断言启用）→ 断言检查会执行
- 被断言包裹的代码（如集合边界检查）会触发额外的 `Result::Err` 传播，导致测试失败

ruva 的修复方案是：识别名称为 `$assertionsDisabled` 的 `getstatic` 调用，**直接返回常量 `true`**（即"断言已禁用"，等价于 JVM 默认以 `-da` 运行）。

**实施步骤**：

**Step 1：instr.py 中 getstatic 特殊处理**

```python
def _gen_getstatic(sim, class_name, field_name, descriptor):
    # 合成字段：断言控制标志，始终视为禁用（= true）
    if field_name == '$assertionsDisabled':
        sim.push(Lit('true'), BOOL)
        return
    # ... 正常 getstatic 逻辑
```

**Step 2：emitter.py 跳过合成字段生成**

在生成字段声明时过滤掉该字段，避免生成无意义的 `$assertionsDisabled: bool = false;`：
```python
SYNTHETIC_SKIP_FIELDS = {'$assertionsDisabled'}

for field in cls.fields:
    if field.name in SYNTHETIC_SKIP_FIELDS:
        continue
    # 正常生成字段
```

**验收**：
- `getstatic $assertionsDisabled` 生成 `true`（常量）而非字段访问
- `cargo check` 不引入新错误
- 对含 `assert` 关键字的 JDK 类（如 `ArrayList`）生成代码不再因断言路径产生类型不匹配

---

### T75 · 无 checked exception 方法裁剪返回类型（`T` 而非 `Result<T, E>`）

**状态**：`[ ]`  
**文件**：`codegen/emitter.py`（方法签名生成）、`codegen/method.py`（`?` 运算符使用）  
**优先级**：P3（代码质量优化，减少冗余 Ok/Err 包装，不影响正确性）  
**来源**：ruva task.md `13.4-T3`（无异常 Result 裁剪）

**背景**：

当前 java_rta 将所有生成的 Java 方法签名包装为 `Result<T, JvmError>`，并在方法体内统一使用 `?` 运算符传播错误。  
但 Java 的 checked exception 机制允许区分：
- 有 `throws` 声明的方法 → 可抛 checked exception，翻译为 `Result<T, JvmError>` 合理
- 无 `throws` 声明（且方法体不抛 checked exception）的方法 → 不需要 `Result` 包装

过度包装导致：
1. 调用链末端大量 `.ok()?` / `unwrap_or_default()` 噪音
2. 无法利用 Rust 类型系统精确区分"可能失败"和"不可能失败"的方法

**实施步骤**：

**Step 1：classfile.py 解析 Exceptions attribute**

`ParsedMethod` 已有 `descriptor`，但需要解析 `Code` attribute 中的 `Exceptions` 属性，提取 checked exception 类列表：

```python
@dataclass
class ParsedMethod:
    ...
    checked_exceptions: list[str]  # 来自 Exceptions attribute，空列表表示无 checked exception
```

**Step 2：emitter.py 条件生成返回类型**

```python
def method_return_type(method: ParsedMethod, base_ty: str) -> str:
    if method.checked_exceptions or method.is_virtual or method.calls_throwing_methods:
        return f"Result<{base_ty}, JvmError>"
    else:
        return base_ty
```

**Step 3：method.py 条件生成 `?` 运算符**

只有在方法签名为 `Result<T>` 时，调用内部函数才追加 `?`。若当前方法返回值不是 Result，则被调函数若返回 Result，需 `.unwrap_or_default()` 或重构调用链。

**注意**：此任务属于**优化阶段**，完整实现需确保调用链一致性。建议在 T62（异常表解析）完成后再实施，否则"无 checked exception"的判断可能不准确。

**验收**：
- 无 `throws` 声明且方法体中不直接调用可抛方法的简单方法（如 `getter/setter`）生成为 `fn get_x(&self) -> i32` 而非 `fn get_x(&self) -> Result<i32, JvmError>`
- `cargo check` 不引入新错误
- 调用这类方法的代码不再出现多余的 `?`

---

### 附录：T56/T62 CFG 实现时的已知陷阱

> 来源：ruva `task-history.md`（CFG-F1/F2/F4）、`task.md`（F-RECORD-2、CFG-ARCH-1a）
>
> 以下 bug 是 ruva 在实现 CFG 结构恢复时实际遇到并修复的。  
> java_rta 实现 T56（CFG 支配树）和 T62（异常表建模）时，应**提前规避**这些问题，而不是等到出现再修复。

**CFG-F1：嵌套 try-catch 内层 catch 丢失**

Java 允许 try 嵌套：
```java
try {
    try { ... } catch (IOEx e) { ... }  // 内层
} catch (Ex e) { ... }                  // 外层
```
朴素的 CFG 建模若只看 handler_pc 不区分嵌套层级，会将内层 handler 的 successors 连接到外层 handler，导致内层 catch block 在结构恢复时被"吞掉"。

**防范**：按 `[start_pc, end_pc)` 区间严格区分 handler 的保护范围；同一 handler_pc 可出现多次（对应多个 try 范围），需全部记录。

---

**CFG-F2：loop exit 检测未涵盖 Switch/Jump**

back-edge 检测只考虑 `if*` 系指令作为循环 exit，遗漏了 `tableswitch`/`lookupswitch` 作为循环 exit 条件（Java switch 可出现在循环末尾），以及无条件 `goto` 的 long-range jump。

**防范**：loop exit 检测需对所有跳转指令（`if*`、`switch*`、`goto`）统一处理。

---

**CFG-F4：`try end_pc` 必须加入 leader set**

基本块切割时，leader 识别通常包含：方法入口、跳转目标、跳转后的下一条指令。  
但 `exception_table.end_pc`（受保护区间结束处）并不是跳转目标，朴素实现会遗漏这个 leader，导致基本块边界错误，handler_pc 块的前驱建立出错。

**防范**：将所有 `exception_table` 的 `start_pc`、`end_pc`、`handler_pc` 三个值都加入 leader set。

---

**F-RECORD-2：离散 try 区间共享同一 handler 必须合并**

JVM 允许多个不相邻的 try 区间共享同一个 handler：
```
try_range [10, 20) → handler 50
try_range [40, 60) → handler 50
```
若直接按条目建模，会生成两个独立的 `TryCatchStmt`，但实际上两者应对应同一个逻辑处理块。

**防范**：按 `(handler_pc, catch_type)` 对区间分组，合并共享同一 handler 的所有 try 区间为 `union([10,20), [40,60))`，生成一个 `TryCatchStmt`。

---

**CFG-ARCH-1a：handler block 必须加入 successors 列表**

在 phi 节点插入算法（支配边界计算）中，若 handler_pc 对应的基本块没有被加入 predecessors 处于保护区间内的任意 BB 的 `successors` 列表，则 phi 插入时不会在 handler block 的汇合点插入 phi 节点，导致 handler 块内的变量访问拿到的是未定义值（Rust E0425 或错误初始值）。

**防范**：建 CFG 时，对受保护区间内的每个 BB，除正常的控制流后继外，额外加入 `handler_bb` 到 `successors`。

---

**阶段十四任务依赖**：

```
T73 (wrapping 算术)    ─ 独立，可立即开始；改动集中在 instr.py，低风险
T74 ($assertionsDisabled) ─ 独立，可立即开始；两处改动，零风险
T75 (Result 裁剪)      ─ 依赖 T62 异常表解析完成后再实施
CFG 陷阱清单           ─ 作为 T56/T62 实施时的检查清单使用，不单独执行
```

**推荐执行序**：
- 立即（P1 正确性）：T73（wrapping 算术，避免 debug 模式 panic）
- 短期：T74（`$assertionsDisabled` 一处改动，五分钟可完成）
- 长期（依赖 T62）：T75（Result 裁剪，需异常表解析作为基础）

---

## 阶段十五：架构级修复（消除 61% 错误的根因）

> **背景（2026-09-14）**：当前 2333 个编译错误中，61%（约 1422 个）来自同一根因——
> Java 类继承层次在生成的 Rust 代码中完全缺失。所有基于错误码的补丁式修复（T68/T64/E0277 等）
> 都无法触及这一问题。本阶段的两个任务是从架构层面解决这 1422 个错误的唯一正确路径。

### T78 · 注解完整化：补全 java_class/java_field/java_method 的 modifiers 字段

**状态**：`[ ]`  
**文件**：`codegen/emitter.py`（`_java_class_attr`、`_java_field_attr`、`_java_method_attr`）  
**优先级**：**P0（T76/T55 的前置条件，改动量小，应最先完成）**  
**方案文档**：`docs/plans/2026-09-14-annotation-driven-java-metadata.md`

**背景**：

当前注解缺少 `modifiers` 字段，无法区分 `static`/`final`/`native`/`abstract` 等修饰符。
补全后，注解成为 Java 类文件的完整元数据镜像，后续 proc-macro 和 build.rs 可仅依赖注解工作。

**实施步骤**：

**Step 1：`_java_class_attr` 补充 `modifiers`**

```python
# codegen/emitter.py
_ACC_SYNTHETIC  = 0x1000
_ACC_ANNOTATION = 0x2000

def _class_modifiers_str(flags: int) -> str:
    parts = []
    if flags & _ACC_FINAL:      parts.append('final')
    if flags & _ACC_ABSTRACT:   parts.append('abstract')
    if flags & _ACC_INTERFACE:  parts.append('interface')
    if flags & _ACC_ENUM:       parts.append('enum')
    if flags & _ACC_SYNTHETIC:  parts.append('synthetic')
    if flags & _ACC_ANNOTATION: parts.append('annotation')
    return ' '.join(parts)

# 在 _java_class_attr inner_lines 中追加：
inner_lines.append(f'    modifiers   = "{_class_modifiers_str(ci.access_flags)}",')
```

**Step 2：`_java_field_attr` 补充 `modifiers`**

```python
_ACC_VOLATILE   = 0x0040
_ACC_TRANSIENT  = 0x0080

def _field_modifiers_str(flags: int) -> str:
    parts = []
    if flags & _ACC_STATIC:    parts.append('static')
    if flags & _ACC_FINAL:     parts.append('final')
    if flags & _ACC_VOLATILE:  parts.append('volatile')
    if flags & _ACC_TRANSIENT: parts.append('transient')
    if flags & _ACC_SYNTHETIC: parts.append('synthetic')
    return ' '.join(parts)
```

**Step 3：`_java_method_attr` 补充 `modifiers`**

```python
_ACC_SYNCHRONIZED = 0x0020
_ACC_BRIDGE       = 0x0040
_ACC_VARARGS      = 0x0080

def _method_modifiers_str(flags: int) -> str:
    parts = []
    if flags & _ACC_STATIC:       parts.append('static')
    if flags & _ACC_FINAL:        parts.append('final')
    if flags & _ACC_SYNCHRONIZED: parts.append('synchronized')
    if flags & _ACC_NATIVE:       parts.append('native')
    if flags & _ACC_ABSTRACT:     parts.append('abstract')
    if flags & _ACC_BRIDGE:       parts.append('bridge')
    if flags & _ACC_VARARGS:      parts.append('varargs')
    if flags & _ACC_SYNTHETIC:    parts.append('synthetic')
    return ' '.join(parts)
```

**验收**：
- 重新生成代码后，任意类文件（如 `ArrayList`）的 `java_class` 注解包含 `modifiers` 字段
- 任意静态字段（如 `SIZE`）的 `java_field` 注解包含 `modifiers = "static final"`
- 任意 native 方法的 `java_method` 注解包含 `modifiers = "static native"` 或类似

---

### T76 · 继承基础：`_super` 字段 + proc-macro Deref/From

**状态**：`[ ]`  
**文件**：`codegen/emitter.py`（struct 生成段 + upcast 方法生成）、`java_rta_macros/src/lib.rs`（直接父类 From impl）  
**优先级**：**P0（架构级，高于所有其他任务）**  
**依赖**：T78（注解完整化先完成）  
**方案文档**：`docs/plans/2026-09-14-annotation-driven-java-metadata.md`

**背景**：

> **设计变更（2026-09-14）**：
> - 原方案（字段展平）：Python 把祖先字段复制进子类——有字段副本语义缺陷
> - 中间方案（Deref）：_super + proc-macro 生成 Deref——Rust 反模式，违反 API Guidelines
> - **选定方案**：_super 嵌套 + 显式 upcast 方法（`as_io_exception` / `into_io_exception`）
>   — 字段语义正确，符合 Rust 惯用法，调用侧完全显式

这导致了三类级联错误（共 1422 个，占总错误 61%）：
- **E0609（64 个）**：父类字段在子类上不可见 → `_super` + `Deref` 解决
- **E0308（~1000 个）**：子类不能用于父类类型的位置 → `From` impl 解决
- **E0599（部分）**：父类方法在子类 struct 上不可见 → `Deref` 使父类方法穿透可见

**实施步骤**：

**Step 1：emitter.py 生成 `_super` 字段**

```python
# codegen/emitter.py: _gen_struct_fields()
def _gen_struct_fields(ci: ClassInfo, ...) -> list[str]:
    lines = []
    # 有父类（且不是 Object）时，第一个字段为 _super
    if ci.super_class and ci.super_class != 'java/lang/Object':
        parent_rust = short_cls(ci.super_class)
        # 父类有泛型参数时用占位（暂用无参数形式，后续 T66 精化）
        lines.append(f'    pub _super: {parent_rust},')
    # 自身字段（不变）
    for f in ci.fields:
        lines.append(_gen_field_line(f, ...))
    return lines
```

此步让每个子类 struct 嵌入直接父类，通过 Deref 链自动穿透：

```
fnfe._super             → IOException
fnfe._super._super      → Exception（通过 IOException 的 _super）
fnfe.detailMessage      → 自动 Deref 三层找到 Throwable.detailMessage ✓
```

**Step 2：emitter.py 生成显式 upcast 方法（不使用 Deref）**

> **设计决策（2026-09-14）**：Deref 模拟继承是 Rust 社区明确的反模式
> （违反 Deref 语义预期，深层继承时错误信息难读，与 Rust API Guidelines 冲突）。
> 改用显式 upcast 方法，语义清晰，符合 Rust 惯用法。

```python
# codegen/emitter.py: _gen_upcast_methods()
def _gen_upcast_methods(ci: ClassInfo, registry: dict) -> str:
    if not ci.super_class or ci.super_class == 'java/lang/Object':
        return ''
    lines = [f'impl {short_cls(ci.name)} {{']
    access_path = '_super'
    cur = ci.super_class
    while cur and cur != 'java/lang/Object':
        rust_name = short_cls(cur)
        snake = to_snake(rust_name)
        lines.append(f'    pub fn as_{snake}(&self) -> &{rust_name} {{ &self.{access_path} }}')
        lines.append(f'    pub fn into_{snake}(self) -> {rust_name} {{ self.{access_path} }}')
        parent_ci = registry.get(cur)
        cur = parent_ci.super_class if parent_ci else None
        access_path += '._super'
    lines.append('}')
    return '\n'.join(lines)
```

生成结果：

```rust
impl FileNotFoundException {
    pub fn as_io_exception(&self)  -> &IOException  { &self._super }
    pub fn into_io_exception(self) ->  IOException  { self._super }
    pub fn as_exception(&self)     -> &Exception    { &self._super._super }
    pub fn into_exception(self)    ->  Exception    { self._super._super }
    pub fn as_throwable(&self)     -> &Throwable    { &self._super._super._super }
    pub fn into_throwable(self)    ->  Throwable    { self._super._super._super }
}
```

**Step 3：proc-macro 生成直接父类 From impl + 接口 trait impl**

proc-macro 读 `super_class` 只生成**直接父类** `From`（不跨层，不生成 Deref）：

```rust
impl From<FileNotFoundException> for IOException {
    fn from(v: FileNotFoundException) -> IOException { v._super }
}
```

读 `interfaces` 生成接口 trait 的空 impl：

```rust
impl Serializable for FileNotFoundException {}
```

**Step 4：调用侧插入显式转换（emitter.py / instr.py）**

在类型期望父类、实际为子类的位置，插入显式 upcast：
```rust
// 期望 IOException，实际 FileNotFoundException
let e = fnfe.into_io_exception();
// 或利用 From：
let e: IOException = fnfe.into();
```

**预计收益**：
- E0609（64 个）：完全消除（`_super` 字段提供字段访问路径）
- E0308（~1000 个）：消除直接父子类型不匹配（跨层 From 由 T55 补全）
- 错误总数预计从 2333 降至约 1000-1200

**验收**：
- `fnfe.as_throwable().detailMessage.get()` 编译通过
- `let e: IOException = fnfe.into()` 编译通过
- E0609 错误数为 0
- E0308 中直接父子类型不匹配错误消除

---

### T77 · 类型擦除一致性（方法边界 Object 化）

**状态**：`[ ]`  
**文件**：`codegen/emitter.py`（方法签名生成）、`codegen/instr.py`（调用侧参数传递）  
**优先级**：P1  
**依赖**：T76 完成后实施（T76 消除了继承类型的不匹配，T77 处理剩余的擦除不一致问题）

**背景**：

JVM 字节码层面，所有引用类型在方法边界上都是 `Object`（类型擦除）。生成的 Rust 代码在某些地方用具体类型，在某些地方用 `Object`，导致不匹配：

```rust
// 生成的方法签名（具体类型）：
fn add(&self, e: String) -> Result<bool>

// 调用侧（传 Object）：
self.add(obj)?  // E0308: expected String, found Object
```

而 JVM 真实情况是：`ArrayList.add(Ljava/lang/Object;)Z` 的参数类型在字节码层就是 Object。

**解决方向**：

对于方法参数类型，遵循 JVM 字节码的实际描述符，而非 Java 源码层的泛型参数：
- `ArrayList.add(Ljava/lang/Object;)Z` → 参数类型 `Object`，不是 `E`
- 在调用侧自动插入 `Object::from_any(val)` 将具体类型擦除为 Object
- 在方法体内需要具体类型时，用 `downcast` 恢复

这与当前 `Object::from_any` + downcast 机制的方向一致，但需要系统性地在所有方法边界执行，而非只在特定位置手动处理。

**注意**：此任务与 T76 有交叉，T76 优先。T76 完成后通过编译日志重新评估 E0308 剩余量，
再决定 T77 的实施范围。

**验收**：
- 所有 `ArrayList.add` / `HashMap.put` 等泛型集合方法的参数接收 Object
- E0308 中"expected Object, found ConcreteType"类错误数为 0

---

**阶段十五任务依赖**：

```
T78（注解完整化）
  ├──→ T76（_super 字段 + proc-macro Deref/From）★ 消除 E0609 + 直接 E0308
  │         │
  │         └──→ T77（类型擦除一致性）──→ 消除剩余 E0308
  │
  └──→ T55（build.rs 扫描注解 → 跨层 From impl）★ 消除跨层 E0308
            │
            ├──→ T53（instanceof 语义）
            ├──→ T69（virtual BFS 完整性）
            └──→ T66（泛型推断）
```

**推荐执行序**：
1. **立即**：T78（注解完整化，emitter.py 三处加 modifiers，改动量小，低风险）
2. **T78 完成后**：T76（emitter.py 加 `_super` 字段 + proc-macro 扩展）
3. **T76 完成后**：T55（build.rs 扫描注解，生成跨层 From impl）
4. **T55 完成后**：重新 cargo check，评估剩余 E0308 数量，决定后续顺序
