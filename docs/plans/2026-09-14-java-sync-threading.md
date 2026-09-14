# Java 同步语义 → Rust 实现方案

**日期**：2026-09-14  
**关联任务**：T79、T80、T81  
**状态**：设计阶段

---

## 一、问题背景

Java 有两套同步机制，在字节码层面均需要翻译到 Rust：

### 机制 A：`jdk/internal/misc/InternalLock`（JDK 21 引入）

JDK 21 起，`PrintStream`、`BufferedWriter`、`OutputStreamWriter` 等 I/O 类将原有的
`synchronized(this)` 替换为 `InternalLock` 字段。典型字节码模式：

```java
// PrintStream.writeln(String) 等所有 write/print/println 方法都是此模式
if (lock != null) {
    lock.lock();                   // invokevirtual InternalLock.lock
    try    { implWriteln(s); }
    finally { lock.unlock(); }    // invokevirtual InternalLock.unlock
} else {
    synchronized (this) {         // monitorenter / monitorexit
        implWriteln(s);
    }
}
```

`lock` 是 `PrintStream` 的 `private final jdk/internal/misc/InternalLock` 字段（**不是方法**）。

**影响范围**：HelloWorld 调用链上 PrintStream 所有 29 个 write/print/println 方法均包含此模式。
实现 `InternalLock` 后，这 29 个方法可直接从字节码翻译，**无需手写 native**。

### 机制 B：`monitorenter` / `monitorexit` 字节码

Java `synchronized` 块/方法编译为 `monitorenter`/`monitorexit` 指令对。
每个 Java 对象都有一个隐含的 monitor（可重入互斥锁）。

当前 `instr.py` 未翻译这两条指令（生成 stub），导致 synchronized 块语义丢失。

---

## 二、当前状态（单线程绕过方案）

```rust
// output/native_impls/java/io/print_stream.rs （现状）
// 直接调用 Rust I/O，跳过整个 lock/implWriteln 调用链
pub fn println__str(_this: &PrintStream, x: String) -> Result<()> {
    println!("{}", x);
    Ok(())
}
```

**优点**：HelloWorld 可以跑通。  
**缺点**：
- 多线程时不安全
- 同时有 29 个 PrintStream 方法需要手写（维护成本高）
- `monitorenter`/`monitorexit` 指令完全未处理

---

## 三、目标架构

### 3.1 InternalLock 实现

```rust
// output/native_impls/jdk/internal/misc/internal_lock.rs

use parking_lot::ReentrantMutex;
use std::cell::RefCell;
use std::sync::Arc;

pub struct InternalLock {
    inner: Arc<ReentrantMutex<()>>,
}

// 每线程维护 guard 栈，支持同线程重入、lock/unlock 跨越方法调用
thread_local! {
    static GUARDS: RefCell<Vec<parking_lot::ReentrantMutexGuard<'static, ()>>>
        = RefCell::new(Vec::new());
}

impl InternalLock {
    pub fn new_instance() -> Self {
        Self { inner: Arc::new(ReentrantMutex::new(())) }
    }

    /// jdk/internal/misc/InternalLock.lock:()V
    pub fn lock(&self) {
        // SAFETY: guard 生命周期与 inner Arc 绑定，Arc 保证内存存活
        let guard = unsafe {
            std::mem::transmute::<
                parking_lot::ReentrantMutexGuard<'_, ()>,
                parking_lot::ReentrantMutexGuard<'static, ()>,
            >(self.inner.lock())
        };
        GUARDS.with(|g| g.borrow_mut().push(guard));
    }

    /// jdk/internal/misc/InternalLock.unlock:()V
    pub fn unlock(&self) {
        GUARDS.with(|g| { g.borrow_mut().pop(); });
    }
}
```

**单线程性能**：`parking_lot::ReentrantMutex` 无竞争路径开销 ≈ 2ns，可忽略。  
**多线程安全**：`ReentrantMutex` 是 `Send + Sync`，支持跨线程安全使用。  
**接口稳定**：`lock()`/`unlock()` 接口不变，单→多线程切换只改这一个文件。

### 3.2 monitorenter / monitorexit 翻译

```
字节码                       翻译为 Rust
────────────────────────────────────────────────────────────
monitorenter (obj_ref)  →   let _mon_N = obj_ref.monitor().lock();
monitorexit  (obj_ref)  →   drop(_mon_N);
                             // 注：Rust 作用域 drop 自动处理 finally 语义
```

**对象 monitor 设计**（两种方案）：

**方案 A：嵌入式 monitor（推荐）**

```rust
// java_runtime/src/types.rs
pub struct JavaObject<T> {
    pub data: T,
    pub monitor: parking_lot::Mutex<()>,   // 每对象一个锁
}
```
- 优点：访问 O(1)，无额外分配
- 缺点：每个对象多 8 字节（Mutex 空结构）

**方案 B：惰性全局 monitor 表**

```rust
// java_runtime/src/monitor.rs
static MONITORS: OnceLock<DashMap<usize, Arc<parking_lot::Mutex<()>>>> = OnceLock::new();

pub fn get_monitor(ptr: usize) -> Arc<parking_lot::Mutex<()>> {
    MONITORS.get_or_init(DashMap::new)
           .entry(ptr).or_insert_with(|| Arc::new(parking_lot::Mutex::new(()))).clone()
}
```
- 优点：未被 synchronized 的对象零开销
- 缺点：全局哈希表查找，多线程竞争时有锁

> **决策**：当前阶段用方案 B（惰性表），避免修改所有已生成的对象结构。
> 多线程压力下评估后再决定是否迁移到方案 A。

### 3.3 PrintStream 边界方法的后续处理

`InternalLock` 实现后，PrintStream 的 29 个 write/print/println 方法体字节码已完整可翻译：

```
bytecode of writeln(String):
  getfield  lock           → 获取 InternalLock 实例（字段访问，可翻译）
  invokevirtual lock()     → InternalLock.lock()   ← T79 提供
  invokevirtual implWriteln() → 纯字节码逻辑        ← 已可翻译
  invokevirtual unlock()   → InternalLock.unlock() ← T79 提供
```

T79 完成后，可删除 `native_impls/java/io/print_stream.rs` 中的手写实现，
改为从字节码自动翻译。这是 **29 个边界方法的系统性消除**。

---

## 四、依赖关系

```
T79（InternalLock 实现）
  └──→ T81（PrintStream 等 I/O 边界方法从字节码翻译，删除手写）

T80（monitorenter/monitorexit 翻译）
  └── 独立，不依赖 T79
      但与 T79 配合后能覆盖 PrintStream 的完整 lock 路径
```

---

## 五、迁移路径（三阶段）

| 阶段 | 做法 | 目标 | 风险 |
|------|------|------|------|
| **阶段 0（当前）** | PrintStream 全手写，绕过 InternalLock | HelloWorld 跑通 | 无线程安全，29 方法手动维护 |
| **阶段 1（T79）** | 实现 InternalLock，PrintStream 改字节码翻译 | 消除 29 个手写方法，正确锁语义 | 需要测试 ReentrantMutex 的 transmute 安全性 |
| **阶段 2（T80）** | monitorenter/monitorexit 翻译 | synchronized 块正确运行 | 需要对象 monitor 设计决策 |
| **阶段 3（多线程）** | 增加线程池/Executor 翻译，压测锁竞争 | 多线程程序正确运行 | 依赖阶段 1+2 全部完成 |

---

## 六、Cargo 依赖

```toml
# output/Cargo.toml 或 java_runtime/Cargo.toml
[dependencies]
parking_lot = { version = "0.12", features = ["arc_lock"] }
# 方案 B 需要：
dashmap = "6"
```

---

## 七、验收标准

- **T79**：HelloWorld 在 `InternalLock` 路径（lock != null 分支）运行正确；
  `native_impls/java/io/print_stream.rs` 中手写的 `writeln`/`newLine`/`write` 方法可删除
- **T80**：含 `synchronized` 关键字的 Java 类翻译后 cargo check 通过；
  多线程 smoke test（2 线程并发调用 System.out.println）无数据竞争（miri 通过）
- **T81**：PrintStream 所有 print/println/write 方法改为字节码翻译，无手写版本，
  HelloWorld 输出结果不变
