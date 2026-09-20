# e2e 测试增量规划：Java 特性覆盖补缺

日期：2026-09-20
依据：仅依据 `tests/e2e/**` 目录结构与文件名（未逐个读文件正文），叠加 `scripts/run_tests.py` 的框架约束与 `runtime/` 已具备的能力面。

---

## 0. 现状盘点

| 项 | 值 |
|---|---|
| 专题目录数 | 31（`01_basics` … `31_callchain`） |
| Java 用例数 | 65 |
| 期望输出数 | 65（`tests/expected/<Class>.txt`，与用例 1:1） |
| 发现方式 | `scripts/run_tests.py` → `E2E.rglob("*.java")` 递归发现 |

现有 31 个专题面：基础语法、OOP、泛型、集合、字符串、异常、lambda、数值、枚举、静态、内部类、高级泛型、高级集合、函数式、switch、现代语法、字符串进阶、数组进阶、抽象、可变参数、转型、自动装箱、算法、Object 方法、多捕获、接口进阶、正则、通配符、嵌套泛型、Stream、调用链。

**结论：语言主干（值/引用类型、类/接口/继承/多态、异常基本形态、lambda/方法引用、泛型基本形态、Stream 三件套、Record/Text Block/Var/Pattern Match）已有覆盖。缺口集中在三类：**

1. **字节码层面的边角语义**（转译器里最容易出错、但又必须正确的跳转/初始化/分派规则）
2. **JDK API 广度中的结构性空白**（Map 家族完全没有专项用例）
3. **runtime 已发力但零用例的区域**（`thread_impl.rs`、`synchronized.rs` 宏、`PrintStream`）

---

## 1. 编写新用例的硬约束（先读，否则白写）

来自 `scripts/run_tests.py` 的实际行为：

1. **类名全局唯一**。`expected/<Class>.txt` 只按类名索引，不带目录名 → 两个不同专题下不能出现同名类，文件名必须等于其中的 public 类名。
2. **单文件自洽**。`javac` 每次只对**单个** `.java` 编译 → 辅助类必须写在同一个文件里（非 public 顶层类、静态嵌套类、内部类均可）。跨文件的包结构在现有框架下不成立。
3. **只比对 stdout**。`expected` 由 `java` 运行捕获 stdout 生成；stderr 和退出码不参与比对。→ **异常必须自己 catch 后 println 类型 + 消息**，不能直接抛出往下冒泡。
4. **输出必须完全确定**。禁用：无种子的 `Random`、`System.currentTimeMillis/nanoTime`、`identityHashCode`、并行流、多线程交错导致的顺序。`HashMap/HashSet` 若要遍历，先排序或先转成 `ArrayList` 再排序打印（避免实现/版本差异）。
5. **打印密度要够**。diff 只能看到行差异，建议每条断言带编号前缀和实测值，方便一眼定位第几条语义错。
6. **新增流程**（注意参数是 `--filter`，没有 `-f` 短选项；多个 filter 之间是 OR 匹配，可一次给多个类名）：
   ```bash
   # 1) 写 tests/e2e/<NN_topic>/<Name>.java
   # 2) 用真实 JVM 生成期望（javac -g → java 捕获 stdout）
   PYTHONPATH= .venv/bin/python scripts/run_tests.py --update-expected --filter <Name>
   # 3) 转译 + cargo run + diff
   PYTHONPATH= .venv/bin/python scripts/run_tests.py --filter <Name>
   ```
   不需要 `-Xlint`，但**禁止用系统 python 3.9**：脚本用了 `str | None` 语法（3.10+）。
7. **先 `--no-run` 探路**：新用例会沿调用链拉 JDK 类加载闭包（TreeMap/PriorityQueue 之类尤其重），建议先加 `--no-run` 确认转译不炸，再跑全链路。
8. **顶层类型名必须全树唯一**（不只是 public 类名）。`--update-expected` 把所有 `.java` 编译进同一个 `tests/classes`，辅助类重名会被 JVM 解析到错误的那个 → `IncompatibleClassChangeError`。本次实际踩到：一个文件的 `interface Box` 与另一个文件的 `record Box` 冲突。写新用例前必须对 `^(class|interface|record|enum)` 做全树查重。
9. **一次只加一个**，加完立刻看 jdk-scan 报告，避免一次性堆几十个失败用例把定位成本放大。

### 语言层注意（JDK 21 下已验证）

- `record` **不能 `extends` 类**（隐含继承 `Record`），只能 `implements` 接口；要做 sealed 层次就用接口承载。
- 接口的 `X.super.m()`：要求 `X` 是**直接**父接口，且 `m` 没有被另一个父接口覆盖过；多接口默认方法冲突时，**每一个**冲突方法都要显式重写，不能只解决其中一个。
- 多线程用例：不同路径必须用**同一把监视器**才能得到确定的计数。分别用 `static synchronized` 方法与 `synchronized(其他对象)` 会真的产生数据竞争，输出不可比对。
- 输出确定性建议**连跑 3 次 diff** 再落 expected，尤其是涉及线程的用例。

---

## 2. P0 — 第一批：字节码语义高危区（16 个）

这批的共同点：**它们在源码层看着平平无奇，但字节码/控制流重构（CFG）层极易做错**，而且一旦做错，高层 JDK API 测试全都会被污染。建议优先做掉。

| # | 目标目录 | 新文件 | 要补的语义点 |
|---|---|---|---|
| 1 | `01_basics` | `TestShortCircuit.java` | `&&` `\|\|` 短路求值；右侧有副作用（调带 println 的方法）时是否被跳过；`&` `\|` 非短路对比 |
| 2 | `01_basics` | `TestTernary.java` | 三元嵌套；三元分支类型不一致时的提升/装箱；三元结果直接参与算术 |
| 3 | `01_basics` | `TestLabeledBreak.java` | 标签 `break/continue` 跳出多层嵌套循环；标签 + `switch` 混用；跳转落地位置 |
| 4 | `08_numbers` | `TestOverflow.java` | int/long 溢出环绕、`-Integer.MIN_VALUE`、`Long` 乘除、整数除零 `ArithmeticException`（自 catch 打印） |
| 5 | `08_numbers` | `TestShiftOps.java` | **移位量掩码语义**：int `<<`/`>>`/`>>>` 仅取低 5 位、long 取低 6 位；负位移数；负数 `>>>` |
| 6 | `08_numbers` | `TestWideSlots.java` | long/double 占两个局部变量槽：形参混排、slot 复用、`long` 参数后接 `int` 参数的调用约定（CFG slot 模拟的经典坑） |
| 7 | `15_switch` | `TestSwitchFallthrough.java` | int 经典 switch 贯穿（无 break）、`default` 置于中间/缺失、相邻 case 共享分支体、tableswitch/lookupswitch 边界值 |
| 8 | `06_exceptions` | `TestFinallyReturn.java` | try/catch/finally 中 `return` 的优先级：finally 修改寄存器中的返回值无效 vs. finally 自己 `return` 覆盖；finally `return` 吞掉异常 |
| 9 | `10_static` | `TestInitOrder.java` | 静态字段 → 静态块 → 实例字段 → 实例块 → 构造器，跨继承链的完整顺序；子类初始化触发父类 `<clinit>` |
| 10 | `02_oop` | `TestConstructorChain.java` | `this(...)` / `super(...)` 调用链；构造器内 super 构造回调虚方法的效果；字段初值与构造器赋值的先后 |
| 11 | `02_oop` | `TestOverload.java` | **静态重载分派优先级**：精确匹配 > 提升 > 装箱/拆箱 > 可变参数 > 父类参数；最具体方法选择（多个候选的二义性靠编写方式规避） |
| 12 | `02_oop` | `TestBridgeMethod.java` | 桥接方法：泛型父类/接口的方法被具体类型重写、**协变返回类型**，两者都会生成合成 bridge 方法 |
| 13 | `11_inner_classes` | `TestInnerClass.java` | 非静态内部类（现有只有 anonymous 与 static nested，缺这一类）：`Outer.this`、访问外部私有成员、持有外部引用的构造过程 |
| 14 | `07_lambdas` | `TestLambdaCapture.java` | 捕获 `this`、捕获实例字段、捕获 effectively final 的局部变量、捕获后作为返回值跨方法传递 |
| 15 | `18_arrays_advanced` | `TestArrayCovariance.java` | `Object[] r = new String[]{}` 的数组协变 + `ArrayStoreException`（runtime 已有 `fix/s4-array-covariance` 分支，正好需要一个回归用例锁住） |
| 16 | `34_concurrency`（新） | `TestSynchronized.java` | `synchronized` 块/方法 + 静态同步；**单线程即可验证语义**（计数器一致性），输出确定，不需要多线程 |

> 第 16 个支撑依据：`runtime/java_runtime/src/java/lang/thread_impl.rs` 与 `runtime/java_rta_macros/src/synchronized.rs` 已实现，但 `tests/e2e` 下**没有并发专题**，属于「runtime 已发力、测试零覆盖」。

---

## 3. P1 — 第二批：API 广度与结构性空白（约 18 个）

### 3.1 Map 家族（结构性空白 → 新目录 `33_maps`）

现有 `04_collections`（ArrayList/Collections）、`13_collections_advanced`（ArrayDeque/Iterator/LinkedList）、`30_streams` 都有覆盖，**唯独 Map/Set 没有任何专项用例**，而 `codegen/instr/coerce.py` 已注明 HashMap/HashSet 走 jdk_classes 翻译（即能力面存在，只是没人验）。

| 文件 | 覆盖点 |
|---|---|
| `33_maps/TestHashMapOps.java` | `put/get/containsKey/remove/replace/putIfAbsent/computeIfAbsent/merge/getOrDefault`；null key、null value |
| `33_maps/TestHashSetOps.java` | `add/contains/remove`、重复元素去重、依赖 `equals/hashCode` 的行为 |
| `33_maps/TestMapIteration.java` | `entrySet/keySet/values`、`Map.Entry`、`forEach(lambda)`、`removeIf` 迭代删除 |
| `33_maps/TestTreeMapSet.java` | 排序 Map/Set：`firstKey/lastKey/headMap/tailMap/subMap`、`ceiling/floor/higher/lower`、自定义 Comparator |
| `33_maps/TestPriorityQueue.java` | 堆 `offer/peek/poll` 的出队顺序、Comparator 构造、`remove(Object)` |
| `33_maps/TestLinkedHash.java` | `LinkedHashMap` 插入顺序保证、`LinkedHashSet`（对比 `HashMap` 无序） |

⚠️ TreeMap/PriorityQueue 会拉较大的 JDK 闭包，**务必先 `--no-run` 看转译是否成功、耗时是否可接受**。

### 3.2 其余 P1

| 目录 | 文件 | 覆盖点 |
|---|---|---|
| `28_generics_wildcards` | `TestWildcardCapture.java` | `<? super T>` 写入、`helper capture` 泛型方法、PECS 双向推的改写 |
| `07_lambdas` | `TestMethodRefKinds.java` | 方法引用的四种形态补齐：静态、绑定实例、未绑定实例、构造器引用（`X::new`）、数组构造器引用（`String[]::new`） |
| `26_interface_advanced` | `TestInterfaceConflict.java` | 多接口默认方法冲突 + `X.super.foo()` 显式选择 |
| `09_enum` | `TestEnumAdvanced.java` | 枚举实现接口、逐常量（per-constant）抽象方法、`values()/valueOf`、内含字段与构造器 |
| `16_modern` | `TestRecordAdvanced.java` | record 紧凑构造器/重载构造、static 成员、实现接口、泛化 record |
| `16_modern` | `TestSealed.java` | `sealed interface + permits`、record/类作为许可实现（JDK 17+；若 javac 版本不支持则降级） |
| `05_strings` | `TestStringBuilderOps.java` | `insert/delete/deleteCharAt/reverse/setLength/indexOf/capacity`、链式调用（现有只有基础 `append/toString`） |
| `17_string_advanced` | `TestStringCompare.java` | `compareTo/compareToIgnoreCase/equalsIgnoreCase/regionMatches/startsWith/endsWith/contains/isEmpty/isBlank` |
| `17_string_advanced` | `TestStringSearch.java` | `indexOf/lastIndexOf` 各重载、`charAt`、`split` 带 limit、`replace/replaceAll/replaceFirst`、`trim/strip/stripLeading` |
| `04_collections` | `TestCollectionsUtil.java` | `Collections.sort/max/min/frequency/binarySearch/reverse/rotate/unmodifiableList/emptyList/singletonList/nCopies`（补齐现有 `TestCollections` 未覆盖的部分） |
| `13_collections_advanced` | `TestListOf.java` | `List.of/Map.of/Set.of` 不可变工厂、`UnsupportedOperationException` 自 catch 打印 |
| `18_arrays_advanced` | `TestArrayCopy.java` | `System.arraycopy`、`Arrays.copyOf/copyOfRange`、自复制区间重叠 |
| `35_io`（新） | `TestPrintStreamApi.java` | `System.out.print/println/printf/format` 对 boolean/char/int/double/Object 各重载的格式化结果（现有 test 大量 println，但 print 格式化面未专项覆盖） |
| `34_concurrency` | `TestThreadJoin.java` | `Thread` + `start/join`：join 之后打印，输出确定（不做无 join 的并发竞争） |
| `08_numbers` | `TestNumericCast.java` | 基本类型宽化/窄化完整矩阵：`byte/short/char/int/long/float/double` 互转与精度截断（与 `21_casting` 的引用转型分工） |
| `01_basics` | `TestIncDec.java` | `++/--` 前后缀、`i = i++` 陷阱、复合赋值隐式窄化（`byte b += 1` 合法而 `b = b+1` 不合法的语义差异） |

---

## 4. P2 — 第三批：纵深与组合场景（约 12 个）

价值偏「覆盖率/回归锁定」，建议在前两批稳定后补。

| 目录 | 文件 | 覆盖点 |
|---|---|---|
| `01_basics` | `TestDoWhile.java` | do-while 至少执行一次；`continue` 在 do-while 中的跳转目标是条件而非循环头 |
| `01_basics` | `TestForEach.java` | 增强 for 遍历数组/Iterable；遍历中对集合结构性修改的 `ConcurrentModificationException`（自 catch） |
| `06_exceptions` | `TestNestedTry.java` | 多层 try-catch-finally 嵌套、内层抛外层接、异常表 range 重叠覆盖 |
| `06_exceptions` | `TestCustomException.java` | 自定义 checked/unchecked 异常、带 `cause` 的构造器、`getMessage/getCause`、异常链传递 |
| `06_exceptions` | `TestTryInLoop.java` | try 内含 `break/continue/return` 时 finally 的执行时机 |
| `15_switch` | `TestSwitchReturn.java` | 各 case 中 `return` 提前退出、switch 表达式中的 `yield` |
| `16_modern` | `TestInstanceOfChain.java` | 连续 `instanceof` + 转型链、`null instanceof`、针对接口的 `instanceof` |
| `16_modern` | `TestVarContext.java` | `var` 用于增强 for 元素、try-with-resources 资源、lambda 参数 |
| `02_oop` | `TestFieldShadow.java` | 字段隐藏：子类同名遮蔽父类字段、`this.x` vs `super.x`、局部变量遮蔽字段 |
| `11_inner_classes` | `TestLocalClass.java` | 方法内局部类、捕获 effectively final 局部变量 |
| `12_generics_advanced` | `TestGenericBoundsCombo.java` | 多边界 `T extends A & B`、递归边界 `<T extends Comparable<T>>` |
| `26_interface_advanced` | `TestInterfacePrivate.java` | 接口私有/私有静态方法（Java 9+）、被默认方法复用 |
| `14_functional` | `TestOptionalChain.java` | `Optional.map/flatMap/filter/orElseThrow/ifPresentOrElse` 链式组合 |
| `30_streams` | `TestStreamNumeric.java` | `IntStream/LongStream` 原生流、`summaryStatistics`、`boxed()`、`flatMap` |

---

## 5. 明确不建议现在碰的（投入产出比差）

这些要么**框架层面不支持**，要么**输出不可确定**，写了也进不了 diff 比对：

- **跨文件 / package 结构**：`javac` 单文件编译，超出one-file 边界；如确需，得先改 `run_tests.py` 的编译与 `--update-expected` 逻辑，属另一件事。
- **并行流 / 无 join 的多线程 / 非同步共享变量**：输出不确定，diff 会假失败。
- **`assert` 关键字**：JVM 默认 `-ea` 关闭，expected 与 Rust 侧都不触发，测了等于没测。
- **依赖对象地址的断言**（`identityHashCode`、默认 `toString()` 里的 `@1b6d3586`）：不可比对。
- **文件 IO / 日期时间 / 本地化**：`FileOutputStream/Locale/DecimalFormat` 在 runtime 有桩，但输出依赖环境，比对意义有限。

---

## 6. 执行节奏建议

```
第 1 轮  P0 前 8 个（控制流 + 数值 + switch + finally）
        → 每个都 mvp：写完 --no-run → 全跑 → 看 diff → 逐个修
        → 目标：把这批的 Semantics 单点锁死，再谈 API 广度
第 2 轮  P0 后 8 个（初始化顺序 / 构造器链 / 重载 / bridge / 内部类 / lambda 捕获 / 数组协变 / synchronized）
        → 这批大概率会暴露 codegen 的结构性 bug，留足分析时间
第 3 轮  P1 的 Map 家族（6 个）
        → cap 个 infra 验证：33_maps 一次只加一个，观察闭包膨胀与编译耗时
第 4 轮  P1 其余 + P2
```

预期用例规模：65 → 约 110。

---

## 7. 备注 / 免责

本规划基于**文件名与目录结构**推断覆盖范围，未读 `.java` 正文，因此可能存在「某个特性其实已在某个 Test 内部被顺带覆盖」的情况。实施时先看一眼目标文件是否已有该用法，避免重复；若发现规划里某项已被覆盖，直接在本文勾选剔除即可。
