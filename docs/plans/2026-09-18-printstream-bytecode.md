# PrintStream 全链路字节码翻译

日期：2026-09-18

## 目标（终态）

- 手写泛型 `println_v<T: Display>` 及生成器中所有指向它的路径：数量 = 0。
- `System.out.println/print` 的每个重载（char/int/boolean/long/double/String/Object/char[]）
  均由 `java/io/PrintStream.class` 字节码翻译得到，调用点按描述符经重载改名规则
  （`hierarchy_overloaded_names`）选中 `println_c` / `println_i` / `println_str` / `println_obj` …
- 手写 PrintStream 行为方法：数量 = 0（`print_stream_impl.rs` 已删除）。
- `System.out` 是一个真实的、由翻译后构造器构造出来的
  `PrintStream(BufferedOutputStream(FileOutputStream(FileDescriptor(1)), 128), true, UTF_8)`。

## 调用链与边界

```
PrintStream.println(X)                       字节码翻译
 → String.valueOf(X) / writeln / implWriteln  字节码翻译
 → BufferedWriter.write / newLine / flushBuffer  字节码翻译
 → OutputStreamWriter.write / flushBuffer     字节码翻译
 → sun/nio/cs/StreamEncoder                   内部边界类（手写，按需）
 → OutputStream.write([BII)（虚分派）
   → BufferedOutputStream.write / flush       字节码翻译
   → FileOutputStream.write([BII)             字节码翻译
   → FileOutputStream.writeBytes([BIIZ)       ACC_NATIVE，file_output_stream_impl.rs
```

### 手写清单（全部位于 `runtime/java_runtime/src/`，与生成文件共置）

| 文件 | 类别 | 已实现成员 | 其余 |
|---|---|---|---|
| `java/io/file_output_stream_impl.rs` | native | `writeBytes`（fd 1/2 → stdout/stderr）、`FD_ACCESS` 静态初值 | 其他 fd → `panic!("stub: …")` |
| `java/io/file_descriptor_impl.rs` | native | `getHandle`、`getAppend` | — |
| `java/lang/system_impl.rs` | native / 静态初值 | `out`、`err`、`lineSeparator` 字段初值 | — |
| `java/lang/object_impl.rs` | native | `Object::new()`（`new Object()` 锁对象） | — |
| `java/lang/thread_impl.rs` | native | `currentThread` | — |
| `sun/nio/cs/stream_encoder_impl.rs` | 内部边界 | `forOutputStreamWriter`、`write(I)`、`write([CII)`、`write(String,II)`、`flushBuffer`、`flush` | 生成的 `panic!` 存根 |
| `sun/nio/cs/utf_8_impl.rs` | 内部边界 | `INSTANCE` | 存根 |
| `jdk/internal/misc/vm_impl.rs` | 内部边界 | `isBooted` | 存根 |
| `jdk/internal/misc/blocker_impl.rs` | 内部边界 | `begin`、`end` | 存根 |
| `jdk/internal/misc/internal_lock_impl.rs` | 内部边界 | `newLockOrNull`（返回 null，等价 `-Djdk.io.useMonitors=true`）、`lock`、`unlock` | 存根 |

### 静态初始化替身

下列静态字段当前由 `_impl.rs` 中的 native 初值函数给出；通用 `<clinit>` 机制落地后
改由翻译后的 `<clinit>` / `initPhase1` 赋值，届时删除对应手写函数：

- `System.out`、`System.err`、`System.lineSeparator`
- `FileOutputStream.FD_ACCESS`
- `sun/nio/cs/UTF_8.INSTANCE`

## 机制设计

### 1. native → Java 回调边（upcalls）

手写方法会构造 / 调用翻译出的 Java 成员（`System.out` 构造 PrintStream，
StreamEncoder 调 `OutputStream.write`）。这些边不在任何字节码里，BFS 看不到。

在 `#[jvm_native]` / `#[jvm_boundary]` / `#[jvm_ext]` 上声明：

```rust
#[jvm_boundary(upcalls = "java/io/OutputStream.write:([BII)V")]
pub fn write_i(&self, c: i32) -> Result<()> { … }
```

`codegen/native_upcalls.py` 扫描 overlay 后的 runtime 源码，BFS 触达某成员时把其 upcalls
目标入队（`<init>` 目标同时计入实例化集合）。Python 侧不出现任何 JDK 类名字面量——
类名只存在于手写 Rust 文件的属性里。

### 2. BFS：方法解析 + RTA

`codegen/transpile.py::_discover_jdk_classes_method_level` 的调用边：

1. **方法解析（JVMS §5.4.3.3）**：常量池类未声明 `(name, desc)` 时，沿父类链找最近声明者，
   再找父接口的 default 方法；入队的是真正被执行的方法。
2. **RTA**：实例化集合 = 调用链上 `new` 指令 / 构造器引用 / upcalls `<init>` 出现的具体类
   （`checkcast` / `instanceof` / `anewarray` 只引入类型存根，不算实例化）。
   对每个已触达的虚方法 `(S, m, d)`，实例化集合中 S 的每个子类型 X 的 `(X, m, d)` 入队。
3. **根类虚方法**：以 `java/lang/Object` 为常量池类的 `toString/hashCode/equals` × 实例化集合。
4. **边界类型上的虚调用**：常量池类是内部边界类（不翻译）时，其虚调用目标 × 实例化子类型。
5. 不动点：排空队列 → 传播 → 有新增则继续。

### 3. 宏：带方法体的 VirtualDefine 必须经 vtable 分派

原先 NeedsWrapper 方法体直接内联在 wrapper 上，`OutputStream.write([BII)` 永远执行基类
实现，到不了 `BufferedOutputStream` / `FileOutputStream` 的覆盖。终态：

- 每个 `X__VTable` 增加隐藏钩子 `__as_X(&self) -> X`（由 inner 重建 wrapper；inner 字段是
  `Rc<Cell/RefCell>`，克隆共享状态）。
- 非 Safe 方法体发射为 wrapper 上的 `__impl_m`；trait 默认实现与 `X__m_base` 经
  `__as_X()` 调 `__impl_m`；公开的 `m` 一律走 vtable。
- `super.m()` 对非 Safe 定义体同样可用（原为 `panic!("stub: super …")`）。

### 4. `toString` 到 `ObjectVTable` 的桥

`#[to_string_vtable = "<Owner>"]`（取代 `has_to_string_method`）：生成器沿父类链找最近的
字节码 `toString` 声明者，宏在 `impl ObjectVTable for inner` 里把 `__obj_str` 转发到
`Owner__VTable::toString`。`String.valueOf(Object)` → `obj.toString()` 因而到达翻译后的实现。

### 5. 已知具体类进入 `Object`

实参期望 `Object`、实际类型是 registry 中的具体类时，生成
`Object::from(Clone::clone(&x))`（`From<T: ObjectVTable>`，保留自身 vtable），
不再经 `JvmRef` 装箱（后者的 `toString` 只会给出 Rust 类型名）。

### 6. 顺带修正的生成器缺陷（新触达方法暴露）

- downcast 分派链跳过抽象类（抽象类不可能是运行期类型，且可能未声明接口方法）。
- 字段声明类型恢复使用「有效类型参数」（内部类继承外部类的 `K, V`）。
- 同一顶层类之下的兄弟内部类构造共享外部类类型变量（`HashMap_EntryIterator::<K, V>::new`）。
- 三元表达式两臂无公共父类型时统一装箱为 `Object`。
- `putstatic` setter 与同名方法冲突时加 `_field` 后缀。
- `super.m()` 调用点按方法解析找到真实声明者。

## 验证状态

| 测试 | 基线类数 | 现类数 | 编译错误 | 运行 |
|---|---|---|---|---|
| HelloWorld | 57 (22+35) | 74 (15+59) | 0 | 输出正确 |
| TestArrayList | 80 (27+53) | 110 (20+90) | 0 | 输出正确 |
| TestCollections | 76 (25+51) | 110 (20+90) | 0 | 第 3 行起挂起（基线同样挂起，HashMap.put 控制流） |
| TestStringBuilder | 456 (110+346) | 603 (55+548) | 288 | 未能运行 |

TestStringBuilder 的 288 个错误全部位于新触达的方法（`java/time`、`java/util/stream`、
`java/util/regex`、`java/nio`、`jdk/internal/foreign` 存根），成因分属：

- 泛型擦除（`&Object` ↔ `&E`、`PipelineHelper<…>` 实例化不一致、`refersTo(T)`）
- 控制流结构化（对象位置出现 `i32`、`panic!("stack underflow")`）
- 返回 `this` / 子类值向祖先类型的上转（`Ok(Clone::clone(this))` 期望 `Writer`）
- 重载改名碰撞（`ValueLayout$OfByte` 与 `$OfBoolean` 同缩写为 `valuel`）
- 接口 default 方法的 `__base` 导入（`TemporalAccessor__range_base`）
- record 风格 `toString` 快捷路径在 `_base` 自由函数中使用 `self`
- 祖先字段访问器未进入 `__BT` 约束（`__get_countIndex`）
- `java/lang/Void` 类型缺失

这些归零后 TestStringBuilder 即达到验收标准；不得以收窄闭包（放弃 RTA / 方法解析）的方式规避。
