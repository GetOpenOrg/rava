# java_rta 产品定位：Java 语言的新编译后端

> 创建日期：2026-09-18  
> 关联文档：[`java-rust-translation-reference.md`](java-rust-translation-reference.md)（转译对照规则）、[`2026-09-17-java-rust-type-1to1.md`](2026-09-17-java-rust-type-1to1.md)（类型 1:1 任务）

---

## 一句话定位

> **让 Java 开发者写 Java 代码，自动获得 Rust 级别的性能和内存安全。**

这不是"Java 转 Rust 迁移工具"，而是 **Java 语言的一个新编译后端**：

```
Java 源代码
    ↓
[java_rta 编译器]
    ↓
可读的 Rust 中间层（1:1 对应，宏封装复杂逻辑）
    ↓
原生二进制（无 JVM）
```

Rust 在这里扮演的角色类似于 LLVM IR——既是可读的中间表示，也是最终编译目标。

---

## 核心体验：开发者永远只写 Java

```java
// 开发者继续这样写，完全不变
public class OrderService {
    private final OrderRepository repo;

    public Order createOrder(UserId userId, List<Item> items) {
        var total = items.stream()
            .mapToDouble(Item::getPrice)
            .sum();
        return repo.save(new Order(userId, items, total));
    }
}
```

构建流程自动生成这个，**开发者能读懂，但不需要自己写**：

```rust
// 自动生成的 Rust（与 Java 1:1 对应）
java_class! {
    pub struct OrderService {
        repo: OrderRepository,
    }

    impl OrderService {
        pub fn createOrder(&self, userId: UserId, items: Array<Item>) -> Result<Order> {
            let total = items.stream()
                .mapToDouble(|i| i.getPrice())?
                .sum()?;
            self.repo.save(Order::new(userId, items, total)?)?
        }
    }
}
```

背后的所有权、生命周期、GC 替代、trait dispatch——**全部由宏和封装层处理，开发者不感知**。

---

## 技术核心：1:1 可读中间层

这是与所有现有方案的本质差异。生成的 Rust 是**可读的、可审查的**，不是黑盒。

| Java 写法 | 生成的 Rust（可读层） | 隐藏在哪里 |
|---------|-------------------|-----------|
| `Animal animal = new Dog()` | `let animal: Animal = Dog::new()` | vtable trait 在宏展开里 |
| `animal.speak()` | `animal.speak()` | 动态分发在 `Animal__VTable` 里 |
| `arr[i] = v` | `arr[i] = v` | `borrow_mut()` 在 `Array<T>` 里 |
| `(Dog) obj` | `obj.into()` | `downcast` 在 `From<Object>` 里 |
| `catch (Exception e)` | `Err(e) if e.is_instance_of(...)` | 错误类型在 `JvmError` 里 |

Java 开发者看到的是业务逻辑；Rust 工具链看到的是完整的内存安全 IR。  
详细规则见 [`java-rust-translation-reference.md`](java-rust-translation-reference.md)。

---

## 核心技术能力

- **全自动转译**：Java 字节码 → 可读 Rust，无需人工干预
- **1:1 语义等价**：所有边界条件行为一致，包括整数溢出、异常语义、null 处理
- **零开发者感知**：所有权、生命周期、borrow check 全部由 `java_class!` 宏和 codegen 封装
- **可读中间层**：生成代码可审查、可调试，Java 开发者能直接对应原始逻辑
- **全依赖支持**：所有 Java 依赖包经由相同管线转译，生态完整继承
- **持续转译**：开发者永远只写 Java，转译是构建流程的一部分（类比：`javac` 换成 `java_rta`）
- **原生二进制**：编译产物无需 JVM，单一可执行文件

---

## 竞争格局：没有人同时做到这些

| 方案 | 继续写 Java | 全依赖支持 | 生成可读代码 | 语义完全等价 | 原生二进制 |
|------|:---------:|:--------:|:---------:|:---------:|:--------:|
| **java_rta** | ✅ | ✅（目标） | ✅ | ✅ | ✅ |
| GraalVM Native Image | ✅ | ❌（反射限制） | ❌（黑盒） | ⚠️（框架兼容差） | ✅ |
| Quarkus / Micronaut | ❌（需改代码） | ❌（框架绑定） | ❌ | ⚠️ | ✅（依赖 GraalVM） |
| 直接重写为 Rust | ❌（重写） | ❌（重写） | N/A | ❌（人工误差） | ✅ |
| 继续用 JVM | ✅ | ✅ | N/A | ✅ | ❌ |

**目前没有任何方案同时满足：继续写 Java + 全依赖支持 + 生成可读代码 + 语义等价 + 原生二进制。**

---

## 历史类比

### 最接近：TypeScript 对 JavaScript

| | TypeScript | java_rta |
|--|-----------|---------|
| 存量生态 | JavaScript 代码库极大 | Java 代码库极大 |
| 开发者迁移成本 | 超集，完全兼容，逐步迁移 | 零改动，零学习成本 |
| 编译目标 | JavaScript（运行时不变） | 原生二进制（运行时替换） |
| 核心收益 | 类型安全 | **性能 + 成本**（更容易量化，对企业决策者更直接） |
| 结果 | 前端事实标准 | — |

**关键区别**：TypeScript 的收益（类型安全）是工程收益；java_rta 的收益（内存、成本、启动时间）是**可量化的财务收益**，对企业决策者的说服力更强。

### 编译器类比：Emscripten + LLVM 的逻辑

```
各语言 → LLVM IR → 原生二进制      （LLVM 的做法）
C/C++ → WebAssembly → 浏览器运行   （Emscripten 的做法）
Java  → Rust IR → 原生二进制       （java_rta 的做法）
```

Rust 既是可读的中间表示，也是最终编译目标。与 Emscripten 最接近，但目标市场（企业 Java 生态）比 WebAssembly 大一个数量级。

---

## 目标市场优先级

### 第一优先：Serverless / FaaS（吸引力最强）
- **冷启动**是 Lambda/Cloud Run 的核心痛点，毫秒级启动直接解决
- 按调用计费，性能提升直接等于成本下降，ROI 可以精确计算
- AWS Lambda、Google Cloud Run、Azure Functions

### 第二优先：微服务容器化（吸引力强）
- 内存减少 70-80%，同等 K8s 资源可运行更多实例
- 启动速度提升，弹性伸缩响应更快
- 云计算账单直接降低

### 第三优先：金融/交易系统低延迟（吸引力强，销售周期长）
- GC 停顿是金融系统长期痛点，无 GC 从根本上消除
- 愿意为性能付高价

### 第四优先：边缘计算/嵌入式（新增市场）
- JVM 完全不适用的场景，原生二进制直接打开大门

### 第五优先：存量系统现代化（吸引力中，决策周期极长）
- 大量金融、电信、政府 Java 系统有现代化需求

---

## 量化价值

| 指标 | 典型改善 |
|------|--------|
| 冷启动时间 | 3秒 → 20毫秒 |
| 内存占用 | 减少 70-80% |
| 部署镜像体积 | 数百 MB → 数十 MB（无 JRE） |
| GC 停顿 | 消除 |
| P99 延迟 | 显著改善 |

---

## 进入市场路径

1. **做无可辩驳的公开 Demo**
   - 选知名 Spring Boot 开源项目完整转译
   - 发布完整性能对比（启动、内存、测试通过率）
   - 发布到 Hacker News / Reddit r/rust / r/java

2. **用 Demo 获得种子用户**
   - 云成本压力大的初创公司
   - 已经被 GraalVM 坑过的团队

3. **第一个真实生产案例的数据作为销售工具**

4. **根据市场反应决定：融资建公司 vs 接受收购**

---

## 商业模式

| 模式 | 描述 | 参考对标 |
|------|------|---------|
| **核心开源 + 企业订阅** | 编译器开源，企业级支持/私有包转译/SLA 收费 | RedHat / Databricks |
| 托管编译 SaaS | 按构建次数或节省成本收费 | 对齐客户利益 |
| 卖给大公司 | Oracle / Amazon / Google / Microsoft | 一次性退出 |

**推荐策略：核心开源，引爆社区，再决定卖还是建公司。**

---

## 核心结论

> 产品的本质不是"转译工具"，而是 **Java 语言的新编译后端**，对标的不是某个工具的市场，而是 JVM 本身作为运行时基础设施的市场。
>
> 对开发者：零学习成本、零改动、零工作流变化。  
> 对企业：可量化的 ROI（内存、成本、启动时间）。  
>
> 如果技术能力全部成立，这是过去二十年 Java 生态最重要的基础设施创新。  
> 唯一需要做的事：**让世界看到它，并且信。**
