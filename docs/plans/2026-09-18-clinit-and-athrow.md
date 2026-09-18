# `<clinit>` 类初始化语义与 athrow 异常对象（终态方案）

> 日期：2026-09-18
> 范围：`codegen/`（异常表解析、try 区域规划、BFS）、`runtime/java_rta_macros`（`java_class!` 类初始化状态机、`java_try!`）、`runtime/java_runtime`（`JvmError`、VM 抛出的异常）
> 相关：`java-rust-translation-reference.md` §8 / §14 / §16；`2026-09-18-cfg-structuring-rewrite.md`（控制流结构化，另一任务）

## 1 目标

| 项 | 终态 |
|----|------|
| `<clinit>` | 整体从字节码翻译；首次主动使用时恰好执行一次；父类先行；同线程递归立即返回；用户类与 JDK 类同一机制 |
| 静态字段 | 宏生成的 thread_local 存储；可读层只有 `X::f()?` / `X::set_f(v)?` |
| `athrow` | `JvmError` 只携带真实 Throwable 对象；`Custom("athrow")` 等字符串化异常数量 = 0 |
| try/catch | 由异常表驱动；按运行时类（含子类）匹配；可读层为 `java_try!` |
| 未捕获异常 | `Exception in thread "main" <类全名>: <消息>`，退出码 1 |
| 常量提取 / 模式识别式的 `<clinit>` 处理 | 0 处（`clinit_extract.py` 已删除） |

## 2 类初始化（JVMS §5.5）

### 2.1 生成形态

- codegen 把 `<clinit>` 作为普通方法翻译为 `fn __clinit() -> Result<()>`，放在 `java_class!` 块内。
- `static` 字段只声明（名字 + 类型），不带初值表达式；`ConstantValue` 属性的赋值同样由宏在初始化时完成。
- 宏（`block/class_init.rs`）为每个类生成：
  - thread_local 存储单元 + `f()` / `set_f()` 访问器（返回 `Result`）；
  - thread_local 状态单元（未初始化 / 初始化中 / 已初始化 / 出错）；
  - `__class_init()`：状态机入口。

### 2.2 状态机

```
未初始化 --首次主动使用--> 初始化中 --父类 __class_init()--> 执行 __clinit
                                   ├─ Ok  → 已初始化
                                   └─ Err → 出错（首次抛 ExceptionInInitializerError，
                                                   之后每次主动使用抛 NoClassDefFoundError）
初始化中 --同线程再次进入--> 立即返回（JVMS §5.5 步骤 3）
```

接口初始化不触发父接口初始化。

### 2.3 触发点

| 字节码 | 注入位置 |
|--------|---------|
| `getstatic` / `putstatic` | 静态访问器入口 |
| `invokestatic` | 无接收者且返回 `Result` 的函数入口 |
| `new` | 构造器（同样是无接收者函数）入口 |

注入由宏完成，可读层不出现 `__class_init`。

### 2.4 BFS

- 任一类进入调用链时，其 `<clinit>` 及父类链上各类的 `<clinit>` 一并入队。
- 边界截断：`jdk/internal/`、`sun/` 以及 `runtime/java_runtime/vm_boundary.txt` 登记的类
  （由 VM 自举、不可能由字节码自洽初始化的公开包类）视为手写边界类，按需实现。
- `runtime/java_runtime/vm_roots.txt`：手写 VM 层直接调用的已翻译方法（VM 抛出的异常的构造器、
  `Throwable.getMessage`），作为 BFS 种子，相当于 HotSpot 的 well-known classes。

## 3 异常对象模型

### 3.1 `JvmError`

```rust
pub struct JvmError { thrown: Object }          // 被抛出的 Throwable 对象，唯一字段
impl<T: Into<Object>> From<T> for JvmError      // athrow：return Err(JvmError::from(x))
```

| API | 用途 |
|-----|------|
| `is_instance_of(binary_name)` | 异常表匹配：运行时类或其任一祖先 |
| `catch_as::<T>(binary_name)` | 把异常对象按运行时类还原为 catch 声明类型的引用视图（`ObjectVTable::__view_as`） |
| `catch_any()` | catch-any 表项的绑定（类型 `Throwable`） |
| `report_uncaught()` | main 出口的未捕获异常报告 |
| `null_pointer()` / `arithmetic(..)` / `array_index_out_of_bounds(..)` / `clone_not_supported(..)` … | VM 抛出的异常：直接构造翻译出的异常类实例 |

`ObjectVTable` 上新增的三个钩子都是实现细节（`__` 前缀，宏为每个类 override），不是新的 trait：

| 钩子 | 语义 |
|------|------|
| `__class_name()` | 运行时类的 binary name |
| `__view_as(any, type_id)` | 以本类或任一祖先类的类型重建引用视图 |
| `__shallow_copy()` | `Object.clone()` 的逐字段浅拷贝 |

跨 crate 的祖先视图 / 向上转换通过 `#[doc(hidden)] X::__from_parts(..)` 构造（字段保持私有）。

### 3.2 VM 抛出的异常

| 来源 | 实现 |
|------|------|
| 数组越界 | `JArray::get / set` 返回 `Result`；生成代码为 `arr.get(i)?` |
| 整数除零 | `idiv / irem / ldiv / lrem(a, b)?`（同时实现 `MIN / -1` 回绕）；除数为非零字面量时保留运算符 |
| null 接收者 | 宏在返回 `Result` 的实例方法入口注入 `_jvm_null` 检查 |
| `Object.clone()` | 手写 `Object__clone_base`（`java/lang/object.rs`）：Cloneable 检查 + `__shallow_copy` |

## 4 try/catch

### 4.1 输入

`codegen/classfile.py` 解析 Code 属性的 `exception_table`（`(start_pc, end_pc, handler_pc, catch_type|None)`），
存入 `ParsedMethod.exception_table`。catch 类型进入 BFS 的字段/存根类集合与 `use` 导入集合。

### 4.2 区域规划（`codegen/method/try_catch.py`，与结构化器解耦）

1. 表项按 `handler_pc` 分组：同一处理器的多条表项（multi-catch、被内联 finally 切开的区间）合并；
   `start_pc >= handler_pc` 的自保护表项丢弃。
2. 覆盖区间集合相同的处理器组成一个 `TryGroup`（同一 try 的多个 catch）；同起点的多个组，处理器越靠后越外层。
3. try 体文本范围 = `[start_idx, 首个处理器)`；其中不被某个活动组覆盖的指令用 `java_unguarded!` 包裹，
   层数 = 不覆盖它的活动组数。
4. 处理器体终点：非末位 → 下一个处理器；末位 → `goto END` 目标、处理器变量的 LVT 作用域终点、当前块终点三者最近者。
5. 处理器首条 `astore` 合并进 catch 头；catch-any 的绑定类型沿 `vm_roots.txt` 中任一异常类的父类链取到根下一层
   （Python 中不出现 JDK 类名字面量）。
6. 循环与 try 同起点时：处理器全部落在循环内 → try 属于循环体，先开循环。

### 4.3 `java_try!`（`runtime/java_rta_macros/src/try_macro.rs`）

```rust
{
    let __java_try_r: Result<u8, JvmError> = 'java_try_N: { BODY'; Ok(0) };
    match __java_try_r {
        Ok(1) => break, Ok(2) => continue, Ok(_) => {}     // try 体从不正常完成时为 unreachable!()
        Err(__thrown) => { if <匹配> { let e: T = ...; HANDLER } else { return Err(__thrown); } }
    }
}
```

try 体改写（`syn::visit_mut`）：`expr?` 与 `return Err(x)` → `break 'java_try_N Err(..)`；循环深度 0 的
`break` / `continue` → 流程码转发；闭包与嵌套 item 不改写；嵌套 `java_try!` 先展开再由外层改写其残余出口；
`java_unguarded!` 剥一层且内容不访问。

### 4.4 与控制流结构化重写的衔接

- `try_catch.py` 只依赖三个回调（处理一段指令、新建子模拟器、冲刷语句），不依赖 `process_block` 的内部结构。
- `codegen/method/codegen.py` 中的接入点只有两处：构造 `TryCatchPlan`、块循环顶部的钩子（先 `emit_unguarded_run`，再 `group_starting_at` → `emit_try_group`）。
  新结构化器在"线性发射一段基本块序列"的位置调用同一组接口即可。
- `ParsedMethod.exception_table` 字段名与该计划一致，可直接共用。

## 5 顺带修复的生成器缺陷（均为根因修复）

| 缺陷 | 修复 |
|------|------|
| `vars.py` 的块嵌套计数把字符串字面量里的 `{` `}` 计入 | `_brace_delta` 先剔除字符串/字符字面量 |
| `_hoist_if_vars` 按名字把全方法同名声明一并提升，互不相干作用域里不同类型的同名槽（switch 各 case 的 `int i` / `long i`）类型冲突 | 以触发提升的那次声明为准，只改写提升点所辖语句内的声明 |
| `super.clone()`（根类 native）无落点 | 手写 `Object__clone_base` + 根类 `__base` 函数导入 |

## 6 遗留项

| 项 | 说明 |
|----|------|
| `checkcast` 失败 | 现为 `downcast` 内 `expect`；需改为抛 `ClassCastException`。downcast / 泛型擦除链路属另一任务，待其合入后接入 `JvmError::class_cast` |
| `newarray` 负长度 | `JvmError::negative_array_size` 已就绪，数组创建尚未接入 |
| 字段访问的 null 接收者 | `getfield` / `putfield` 访问器不返回 `Result`，null 时不抛 NPE |
| 手写 static native | 不经宏注入，不触发类初始化 |
| 含 default 方法的接口初始化 | JVMS §5.5 要求类初始化时初始化声明了非抽象非静态方法的父接口，尚未实现 |
| class 字面量 / `desiredAssertionStatus` | `Class` 为 VM 边界类，`$assertionsDisabled` 恒为按边界实现返回值 |
| `monitorenter` 区域 | synchronized 的 catch-any 处理器已结构化，监视器本身为单线程空实现 |
| `HashMap.putVal → treeifyBin → resize` 死循环 | 控制流结构化任务范围；修复后调用链将继续触达 `StaticProperty` / `LocaleUtils` 等边界类，按需实现 |
