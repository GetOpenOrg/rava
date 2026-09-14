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
