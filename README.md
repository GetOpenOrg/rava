# java-rta

Java 字节码 → Rust 源码转译器。

通过 RTA（Rapid Type Analysis）可达性分析，从 Java `.class` 文件精确提取最小可达方法集合，将其转译为符合 Java 命名空间同构的 Rust 代码，并生成可直接编译运行的 Cargo 项目。

---

## 核心特性

- **基于字节码**：直接解析 `.class` 二进制，无需 JDK 工具链介入转译环节
- **RTA 裁剪**：只转译可达方法，避免将 5000+ 类的 JDK 全部拉入
- **命名空间同构**：Java 包路径与 Rust 模块路径一一对应（`com.example.Foo` → `src/com/example/foo.rs`）
- **Java 语义保留**：生成代码使用 Java 风格 API（`String`、`ArrayList<T>`、`System::out().println()`），不直接暴露 Rust 标准库
- **可读可改**：生成代码与手工翻译代码完全等价，开发者可直接阅读和修改

## 转译流水线

```
.java 源文件
    │  javac -g
    ▼
.class 字节码
    │  classfile.py（二进制解析 + BootstrapMethods + LocalVariableTable）
    ▼
ParsedClass / ParsedMethod（指令序列 + 常量池 + 变量名）
    │  RTA 可达性分析
    ▼
可达方法集合
    │  cfg.py（循环检测）+ stack.py（栈模拟）+ instr.py（指令翻译）
    ▼
Rust IR（具名变量 + 结构化控制流）
    │  method.py（后处理：别名合并、mut 裁剪、格式简化）
    ▼
.rs 源文件  +  java_runtime/（运行时类型层）
    │  cargo build
    ▼
native binary
```

## 快速开始

**环境要求**：Python 3.12+、JDK 17+、Rust 工具链（stable）

```bash
# 转译单个文件（scratch 自动建在 build/<测试名>/，含手写代码 overlay）
python3 scripts/main.py tests/e2e/01_basics/HelloWorld.java

# 只生成不运行
python3 scripts/main.py tests/e2e/01_basics/HelloWorld.java --no-run

# 清空 scratch 重建
python3 scripts/main.py tests/e2e/01_basics/HelloWorld.java --clean

# 运行全量 e2e 测试
python3 scripts/run_tests.py
```

**示例**：`tests/HelloWorld.java` 经转译后生成：

```rust
// build/hello_world/user/src/hello_world.rs
use crate::java_runtime::prelude::*;

pub struct HelloWorld { ... }

impl HelloWorld {
    pub fn new(message: String) -> Result<Self> { ... }

    pub fn greet(&self) -> Result<()> {
        System::out().println(format!("Hello, {}", self.message.get()))?;
        Ok(())
    }

    pub fn repeat(s: String, times: i32) -> Result<String> {
        let mut sb = StringBuilder::new()?;
        let mut i = 0i32;
        loop {
            if i >= times { break; }
            sb.append(&s)?;
            i += 1;
        }
        Ok(sb.to_string()?)
    }

    pub fn main() -> Result<()> {
        let hw = HelloWorld::new(String::from("World"))?;
        hw.greet()?;
        let r = Self::repeat(String::from("ha"), 3)?;
        System::out().println(r)?;
        let items = ArrayList::<String>::new()?;
        items.add(String::from("foo"))?;
        items.add(String::from("bar"))?;
        System::out().println(items.size())?;
        Ok(())
    }
}
```

## 项目结构

```
java-rta/
├── scripts/
│   ├── main.py                  # CLI 入口
│   ├── java_rta.py              # RTA 可达性分析
│   └── codegen/
│       ├── classfile.py         # .class 二进制解析
│       ├── cfg.py               # 控制流图 + 循环检测
│       ├── stack.py             # JVM 操作数栈模拟
│       ├── instr.py             # 字节码指令翻译
│       ├── method.py            # 方法体生成 + 后处理
│       ├── emitter.py           # Cargo 项目生成
│       ├── type_map.py          # Java → Rust 类型映射
│       └── runtime.py           # 运行时类型定义（java_runtime/）
├── tests/                       # 测试用 Java 源文件
├── runtime/                     # 手写代码唯一真源（java_runtime + java_rta_macros，提交 git）
├── docs/
│   ├── tasks.md                 # 任务管理（当前活跃，只含开放项）
│   ├── tasks-history.md         # 任务历史文档（T01-T81，已归档）
│   └── plans/                   # 设计文档
│       ├── 2026-09-12-java-to-rust-transpiler.md
│       └── 2026-09-12-codegen-java-api-rules.md
└── build/                       # 每测试一次性 scratch（.gitignore，生成代码不提交）
```

## 代码生成规范

生成代码遵循以下原则（详见 [`docs/plans/2026-09-12-codegen-java-api-rules.md`](docs/plans/2026-09-12-codegen-java-api-rules.md)）：

- 命名空间与 Java 完全同构，包路径对应 Rust 模块路径
- 类型名称与 Java 一致（`String`、`ArrayList<T>`），不使用 Rust 标准库类型
- 所有方法返回 `Result<T>`，调用处加 `?`
- 实例字段通过 `Field<T>` 封装，外部通过 `.get()` / `.set()` 访问
- 输出语句使用 `System::out().println()`
- 运行时基础设施（`Rc<RefCell<>>`、`Vec` 等）封装在 `java_runtime/` 内，业务代码不可见
