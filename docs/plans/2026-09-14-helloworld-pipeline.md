# HelloWorld 转译流水线全流程

> 以 `tests/e2e/01_basics/HelloWorld.java` 为例，梳理从 Java 源码到可运行 Rust 代码的完整路径。

---

## 顶层流程

```
HelloWorld.java
    │
    ▼ [1/4] javac（codegen/transpile.py: transpile()）
HelloWorld.class
    │
    ▼ [2/4] 解析 .class（codegen/classfile.py: parse_class()）
ClassInfo（字段 + 方法 + 带注释的字节码指令列表）
    │
    ▼ [3/4] 方法级调用链 BFS（codegen/transpile.py: _discover_jdk_classes_method_level()）
jdk_infos: {binary_name → ClassInfo}  ←  ~940 个 JDK 类
visited_methods: {(cls, method, desc)}  ←  ~5855 个可达方法
    │
    ▼ [4/4] 生成 Rust（codegen/emitter.py: write_cargo_project()）
output/
├── java_runtime/   ← 手写，git 管理
├── jdk_classes/    ← 生成，~940 个 JDK 类翻译
└── user/           ← 生成，HelloWorld 翻译
```

---

## 阶段一：javac 编译

```
transpile()
└── subprocess.run(['javac', '-g', '-d', 'classes/', 'HelloWorld.java'])
    ├── 输入：HelloWorld.java
    ├── 输出：tests/e2e/01_basics/classes/HelloWorld.class
    └── -g：保留 LocalVariableTable，解析时可得到参数/变量名
```

---

## 阶段二：.class 文件解析

```
classfile.parse_class('HelloWorld.class')
│
├── _Reader（字节流顺序读取）
│
├── _parse_constant_pool()
│   ├── 读取所有常量池条目（Utf8 / Class / Methodref / Fieldref / ...）
│   └── 返回按索引随机访问的 pool 列表
│
├── 读取类元数据
│   ├── access_flags（ACC_PUBLIC / ACC_INTERFACE / ...）
│   ├── this_class → class_name（如 "HelloWorld"）
│   ├── super_class（如 "java/lang/Object"）
│   └── interfaces 列表
│
├── 解析 fields → [FieldInfo]
│   ├── name / descriptor / is_static / access_flags
│   └── Signature attribute → generic_signature（泛型类型）
│
├── 解析 methods（两步）
│   ├── 第一步：收集原始数据（name / descriptor / Code attribute bytes / ...）
│   └── 第二步：_parse_code_attribute() 解码字节码
│       ├── 读取 LocalVariableTable → local_names（slot → 变量名）
│       ├── 读取 BootstrapMethods attribute（invokedynamic / 字符串拼接模板）
│       └── _decode_bytecode() → [Instr(pc, opcode, operand, comment)]
│           ├── invokevirtual/invokespecial/invokestatic
│           │   comment = "Method java/io/PrintStream.println:(Ljava/lang/String;)V"
│           ├── invokeinterface
│           │   comment = "InterfaceMethod java/util/List.add:(Ljava/lang/Object;)Z"
│           ├── getstatic / putstatic / getfield / putfield
│           │   comment = "Field java/lang/System.out:Ljava/io/PrintStream;"
│           └── new / checkcast / anewarray
│               comment = "java/lang/StringBuilder"
│
└── 返回 ClassInfo
    ├── name = "HelloWorld"
    ├── fields = [FieldInfo(name="message", descriptor="Ljava/lang/String;")]
    └── methods = [ParsedMethod(<init>), ParsedMethod(greet), ParsedMethod(repeat), ParsedMethod(main)]
```

**native/abstract 方法**：没有 Code attribute，`instrs=[]`，`is_native=True`，不解码字节码。

---

## 阶段三：方法级调用链 BFS

### 总体逻辑

```
_discover_jdk_classes_method_level(class_infos)
│
├── 初始化
│   ├── visited_methods: set[(cls, method, desc)]
│   ├── queue: deque[(cls, method, desc)]
│   └── field_discover_classes: set[cls]
│
├── 种子扫描：遍历用户类所有方法的指令
│   └── enqueue_refs(m.instrs) → _collect_method_refs(instrs)
│       ├── "Method ..."  → (cls, meth, desc) 入队
│       ├── "Field ..."   → cls 加入 field_discover_classes（不展开）
│       └── "new/checkcast ..." → (cls, '<init>', '()V') 入队
│
├── BFS 主循环
│   ├── 出队 (cls, meth, desc)
│   ├── JdkResolver.resolve(cls) → 从 JDK .jmod 读取 .class bytes
│   ├── parse_class_bytes(data) → JDK 类的 ClassInfo
│   ├── 把类记入 jdk_infos
│   └── 找到该方法，扫描其指令，继续入队新引用
│
└── BFS 结束后：补充 field-only stub 类
    ├── 遍历 field_discover_classes
    ├── 排除已在 jdk_infos 中的类
    └── 对剩余类：resolve → parse → 加入 jdk_infos（不展开方法）
```

### JdkResolver 工作方式

```
JdkResolver.resolve("java/util/ArrayList")
│
├── 查 _name_to_jmod 缓存（命中则直接读）
│
└── 按优先级扫描 jmod 文件
    ├── java.base.jmod  ← 优先
    ├── java.desktop.jmod
    ├── java.logging.jmod
    └── ...（其他 jmod）
        │
        └── zipfile.read("classes/java/util/ArrayList.class")
            └── 返回 .class bytes
```

### BFS 发现结果（HelloWorld 示例）

```
BFS 扫描结果
├── 调用链 BFS 发现：306 个类
│   └── 从 main() → System.out.println() → PrintStream → ... 沿调用链展开
│
├── Field-only stub：634 个类
│   └── 例：System.out（getstatic），CodingErrorAction.REPLACE 等
│       只生成类型存根，不展开其方法体
│
└── 合计：940 个 JDK 类
```

---

## 阶段四：Rust 代码生成

```
write_cargo_project(out_dir, class_infos, jdk_class_infos, visited_methods)
│
├── 构建 registry = {binary_name → ClassInfo}（用户类 + 所有 JDK 类）
│
├── _scan_native_impls(out_dir)
│   └── 扫描 output/native_impls/ 目录
│       ├── native_impls_map: {(cls, method, desc) → (fn_name, rel_path)}
│       ├── synthetics_map:  {cls → [synthetic_method_info]}
│       └── extra_fields_map: {cls → [(field_name, field_type)]}
│
├── 生成 jdk_classes crate（对每个 JDK 类）
│   └── _gen_class_rs(ci, call_chain=visited_methods)
│       ├── struct 定义（字段用 JField<T> 包装）
│       └── impl 块（对每个方法判断生成策略）
│           │
│           ├── is_native 或 is_abstract
│           │   ├── native_impls_map 中有手写实现
│           │   │   └── → pub fn xxx() { _native::xxx(...) }
│           │   │         mod _native { include!("native_impls/..."); }
│           │   └── 无手写实现
│           │       └── → panic!("native: ClassName.method:descriptor")
│           │
│           ├── (cls, name, desc) NOT IN visited_methods（不在调用链上）
│           │   └── → panic!("stub: ClassName.method:descriptor")
│           │
│           └── (cls, name, desc) IN visited_methods（在调用链上）
│               └── gen_method_body() 翻译字节码
│                   ├── StackSim 模拟操作数栈类型
│                   ├── CFG 分析（find_loops / find_boolean_conditions）
│                   ├── sim_instr() 按指令生成 IR 语句
│                   └── render_stmt() / render_expr() → Rust 字符串
│
└── 生成 user crate（HelloWorld）
    └── hello_world.rs
        ├── struct HelloWorld { message: JField<String> }
        └── impl HelloWorld
            ├── fn new()  → 翻译字节码（在调用链上）
            ├── fn greet() → 翻译字节码（在调用链上）
            ├── fn repeat() → 翻译字节码（在调用链上）
            └── fn main()   → 翻译字节码（程序入口）
```

---

## 输出目录结构

```
output/
├── Cargo.toml                    # workspace（成员：java_runtime, jdk_classes, user）
│
├── java_runtime/                 # 手写，git 管理
│   └── src/
│       ├── error.rs              # JvmError / Result<T>
│       ├── types.rs              # JField<T>
│       └── java/                 # 临时手写：String / ArrayList / System 等
│
├── jdk_classes/                  # 生成，gitignored
│   └── src/
│       └── java/
│           ├── lang/
│           │   ├── string.rs         ← java/lang/String（调用链方法翻译 + 其余 stub）
│           │   ├── string_builder.rs
│           │   └── ...
│           ├── io/
│           │   ├── print_stream.rs
│           │   └── ...
│           └── util/
│               ├── array_list.rs
│               └── ...
│
├── user/                         # 生成，gitignored
│   └── src/
│       ├── main.rs               # bin 入口
│       ├── hello_world.rs        # HelloWorld 翻译
│       └── lib.rs
│
└── native_impls/                 # 手写，git 管理
    └── java/lang/system.rs       # System.arraycopy 等 native 实现
```

---

## 方法处理决策树（单个方法）

```
对 JDK 类中的每个方法 m：
│
├── m.is_native || m.is_abstract ?
│   ├── YES → native_impls/ 中有手写实现？
│   │   ├── YES → 生成代理桩：pub fn foo() { _native::foo(...) }
│   │   └── NO  → 生成 panic 桩：panic!("native: Cls.foo:desc")
│   │
│   └── NO（普通方法，有字节码）
│       │
│       └── (cls, "foo", desc) ∈ visited_methods？
│           ├── NO（不在调用链上）
│           │   └── 生成 panic 桩：panic!("stub: Cls.foo:desc")
│           │
│           └── YES（在调用链上）
│               └── gen_method_body() 翻译字节码 → 真实 Rust 实现
```

---

## 核心原理一：字节码 → Rust 翻译机制

### 基于栈的 → 基于变量的转换

JVM 是**操作数栈**模型，Rust 是**变量**模型。翻译时用 `StackSim` 模拟 JVM 栈：

```
JVM 字节码（操作数栈）          生成的 Rust（变量）
─────────────────────────       ─────────────────────────
iload_1                         // 把 slot-1 推入虚拟栈
iload_2                         // 把 slot-2 推入虚拟栈
iadd                            let t0 = a + b;
istore_3                        let c = t0;
```

`StackSim` 内部：
- `stack: [(RsExpr, RsType)]` — 每个栈槽存 IR 表达式节点 + 类型节点
- `locals: {slot → (name, RsType)}` — 局部变量槽映射
- `stmts: [RsStmt]` — 已生成的 IR 语句列表

每条指令经 `sim_instr()` 处理，操作栈和语句列表，不直接输出字符串。

### IR 节点体系（`codegen/rs_ir.py`）

不直接拼字符串，先构建类型安全的 IR 节点，再由 `render.py` 统一渲染：

```
类型节点（RsType）                 表达式节点（RsExpr）
├── RsPrimitive  i32/bool/()      ├── Lit        字面量
├── RsNamed      ClassName        ├── Var        变量引用
├── RsGeneric    Vec<T>/Rc<T>     ├── BinOp      a + b
├── RsRef        &T / &mut T      ├── Call       函数调用
└── RsSlice      &[T]             ├── MethodCall 方法调用
                                  ├── FieldAccess 字段访问
语句节点（RsStmt）                 └── Cast       类型转换
├── LetStmt      let x = ...;
├── AssignStmt   x = ...;
├── IfStmt       if/else 树
├── LoopStmt     loop { ... }
└── ReturnStmt   return Ok(e);
```

### 控制流识别（`codegen/cfg.py`）

JVM 只有 `goto` 和条件跳转，没有 `while`/`for` 语法。`find_loops()` 通过**回边检测**识别循环：

```
找到 goto 指令 且 目标地址 < 当前地址  →  back-edge
    ↓
在 [target, goto] 区间内找条件分支（ifXX）且跳出区间
    ↓
识别为 while 循环：
  loop {
      if !cond { break; }
      // 循环体
  }
```

---

## 核心原理二：类型系统映射

### JVM 描述符 → Rust 类型（`codegen/type_map.py`）

```
JVM descriptor    Rust 类型
─────────────     ──────────────────────────────
I                 i32
J                 i64
Z                 bool
C                 u16  (Java char 是无符号 16 位)
Ljava/lang/String;  String  (java 命名空间的 String，非 std::string::String)
Ljava/lang/Object;  Object
Ljava/util/ArrayList;  ArrayList<Object>  (泛型参数擦除为 Object)
[I                Rc<RefCell<Vec<i32>>>  (数组 = 堆分配共享可变容器)
[Ljava/lang/Object;  Rc<RefCell<Vec<Object>>>
```

**命名空间同构原则**：JDK 类的 Rust 结构体名与 Java 短类名一致，`String`、`ArrayList` 等直接可用，不加任何前缀。

### 字段包装：`JField<T>`

Java 字段允许通过 `this.field` 读写，即使持有 `&self` 也可变。Rust 不允许 `&self` 修改字段。解决方案：

```rust
// 生成的 Rust 结构体
pub struct HelloWorld {
    pub message: JField<String>,  // JField<T> = Rc<RefCell<T>>
}

// 字段读取（getfield）
let v = self.message.get();   // → RefCell::borrow().clone()

// 字段写入（putfield）
self.message.set(v);          // → RefCell::borrow_mut() = v
```

### 装箱/拆箱透明化

Java 的 `Integer.valueOf(x)` / `intValue()` 是编译器自动插入的装箱拆箱指令。翻译时直接忽略：

```
invokestatic Integer.valueOf:(I)Ljava/lang/Integer;   →  (跳过，值保持 i32)
invokevirtual Integer.intValue:()I                    →  (跳过，值保持 i32)
```

定义在 `BOXING_SKIP_STATIC` 和 `UNBOX_VIRTUAL` 集合中。

---

## 核心原理三：异常 → Result 传播

Java 方法可抛出受检/非受检异常，Rust 用 `Result<T>` 模拟：

```
所有翻译后的方法签名：
  pub fn foo(&self, ...) -> Result<T>

方法调用处自动加 ?：
  Java: int r = obj.foo();
  Rust: let r = obj.foo()?;

void 方法末尾补 Ok(())：
  Java: void bar() { ... }
  Rust: pub fn bar(&self) -> Result<()> { ...; Ok(()) }

构造器返回 Self：
  Java: HelloWorld(String msg) { ... }
  Rust: pub fn new(msg: String) -> Result<Self> { ...; Ok(this) }
```

---

## 核心原理四：native 方法手写桥接

`native_impls/` 目录结构镜像 JDK 包路径，文件内用特定注释标记接口：

```
native_impls/
└── java/lang/system.rs   ←  手写 System.arraycopy 等

// 文件内格式（供 emitter 扫描）：
// @java_class: java/lang/System
// @java_method: arraycopy:(Ljava/lang/Object;ILjava/lang/Object;II)V
pub fn arraycopy(src: Object, ...) -> Result<()> {
    // 手写 Rust 实现
}
```

`emitter._scan_native_impls()` 扫描目录，建立映射：
`(cls, method, descriptor) → (rust_fn_name, relative_path)`

生成代码时：
```rust
// 生成：代理桩
pub fn arraycopy(...) -> Result<()> {
    _native::arraycopy(...)  // 调用手写实现
}

// 生成：native_impls include
mod _native {
    include!("../../native_impls/java/lang/system.rs");
}
```

---

## 核心原理五：泛型类型擦除的处理

JVM 在运行时对泛型做类型擦除（如 `ArrayList<String>` → `ArrayList`，方法实际操作 `Object`）。翻译时：

```
策略：
1. 类级泛型参数从 Signature attribute 解析：
   ArrayList<E>  →  struct ArrayList<E: Clone + 'static>

2. 方法参数/返回值：
   - 有泛型签名  →  用类型变量（如 E）
   - 无泛型签名  →  fallback 到 Object

3. 泛型方法调用返回 Object 时，若上下文期望具体类型：
   let x: ArrayList<String> = obj.get_items()?;
   →  let x = obj.get_items()?.downcast::<ArrayList<String>>();

4. registry 中已翻译类使用具体名称，未翻译类 fallback 到 Object
```

---

## 核心原理六：Cargo workspace 模块树生成

JDK 类的 binary name（`java/util/ArrayList`）直接映射到 Rust 模块树：

```
java/util/ArrayList
    ↓ 目录结构
jdk_classes/src/java/util/array_list.rs   ← to_snake("ArrayList") = "array_list"

    ↓ mod.rs 自动生成
jdk_classes/src/java/mod.rs:
    pub mod util;
    pub use util::*;

jdk_classes/src/java/util/mod.rs:
    pub mod array_list;
    pub use array_list::*;
```

冲突处理：若类名 `foo` 与子包目录 `foo/` 同名（E0761），文件名改为 `foo_t.rs`。

Rust 关键字包名（如 `ref`、`type`）用 `r#` 转义：`pub mod r#ref;`

---

## 关键数据结构

| 结构 | 定义位置 | 含义 |
|------|---------|------|
| `ClassInfo` | `codegen/types.py` | 一个 Java 类的完整元数据 |
| `ParsedMethod` | `codegen/types.py` | 一个方法：名称 + 描述符 + 指令列表 |
| `Instr` | `codegen/types.py` | 单条字节码：pc / opcode / operand / comment |
| `FieldInfo` | `codegen/types.py` | 字段：名称 / 描述符 / is_static |
| `visited_methods` | `transpile.py` BFS | 调用链可达方法集合 `{(cls, method, desc)}` |
| `jdk_infos` | `transpile.py` BFS | BFS 发现的 JDK 类 `{binary_name → ClassInfo}` |
| `native_impls_map` | `emitter.py` | 手写 native 实现索引 `{(cls,m,d) → (fn, path)}` |
| `registry` | `emitter.py` | 全类注册表，用于类型解析和交叉引用 |
