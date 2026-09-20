# e2e 测试缺口分析（基于已有用例内容，2026-09-20）

> 本文档是对 `2026-09-20-e2e-coverage-plan.md` 的**深化版**。前一份只按目录结构找缺口，随后已补 52 个用例（现共 117 个 `.java` + 117 个 `expected`）。本份**读取了已有用例正文**，按「Java 版本特性」和「边界/语义正确性」两个维度，给出下一层该补的测试。
>
> 分析依据：对 `tests/e2e/**/*.java` 做了特征扫描（覆盖/缺失 token 计数）+ 精读 20+ 个代表性用例（switch/pattern-match/record/textblock/lambda/generics/bitwise/double/autoboxing/casting/varargs/static-init/string-ops/stream-collectors/optional 等）。

---

## 0. 当前已覆盖得较好的部分（避免重复造轮子）

| 维度 | 已覆盖要点 |
|------|-----------|
| 控制流 | 短路、三元、带标签 break/continue、do-while、for-each、switch 表达式（arrow + `yield` + `when` 守卫）、switch 穿透/返回、静态初始化顺序/初始化块 |
| OOP | 继承/多态/重载/重写、字段遮蔽、构造链、`bridge` 方法、内部类（匿名/静态嵌套/非静态/局部）、接口（默认/静态/私有方法 + 冲突解决） |
| 泛型 | 基础、有界、泛型方法、通配符、嵌套泛型、多边界组合（`<T extends Number & Comparable<T>>`） |
| 异常 | 基础、多 catch、try-with-resources、try-in-loop、嵌套 try、自定义异常、finally-return |
| 函数式 | lambda（表达式/块体/捕获）、方法引用多形态、`@FunctionalInterface`、Optional 基础（of/empty/map/filter/get/orElse/isPresent）、Stream 中间操作（map/filter/sorted/distinct/limit/skip/anyMatch/allMatch/noneMatch/findFirst/reduce/flatMap）+ 收集器（joining/toList/counting/groupingBy/partitioningBy/summingInt） |
| 现代语法 | `var`、record（基础+进阶，含 compact 构造器）、sealed、文本块、instanceof 模式匹配、switch 模式匹配（含 `when` 守卫）、instanceof 链 |
| 集合 | ArrayList/LinkedList/ArrayDeque/Iterator、Map 家族（HashMap/HashSet/TreeMap/PriorityQueue/LinkedHashMap）、`List.of`、`Collections` 工具、Arrays 工具（copyOf/sort/binarySearch/fill/equals） |
| 字符串 | StringBuilder/Buffer、trim/strip/lower/upper/replace/contains/startsWith/endsWith/indexOf/substring/split/join/repeat/isBlank、正则、format、比较、搜索 |
| 数值 | int/long/double/bitwise、溢出、移位掩码（`&31/&63`）、long/double 双 slot、窄化转换 |
| 枚举 | 基础、方法、进阶 |
| 并发/IO | synchronized 三形态、线程 join、PrintStream 进阶 |

---

## 1. Java 版本特性覆盖矩阵

图例：✅ 已覆盖　🟡 部分覆盖　❌ 完全缺失/几乎没测

### JDK 8（基线，基本齐）
| 特性 | 状态 | 说明 |
|------|------|------|
| lambda / Stream / 默认方法 / 函数式接口 / 方法引用 / 多 catch / try-with-resources | ✅ | 充分 |
| Optional（of/empty/map/filter/get/orElse/isPresent） | ✅ | 基础有 |
| 无符号整数工具（`toUnsignedLong`/`divideUnsigned`/`parseUnsignedInt`/`remainderUnsigned`） | ❌ | 见 §2.7 |
| BigDecimal / BigInteger（任意精度） | ❌ | **大缺口**，见 §2.4 |
| 新日期时间 API（`java.time`） | ❌ | **大缺口**，见 §2.5 |
| Base64 / String.join | 🟡 | join 有，Base64 无 |
| 重复注解 `@Repeatable` / 类型注解 | ❌ | 见 §3.21 |
| CompletableFuture / ForkJoin | ❌ | 见 §3.29/§3.30 |

### JDK 9
| 特性 | 状态 | 说明 |
|------|------|------|
| 接口私有方法 | ✅ | TestInterfacePrivate |
| `List/Set/Map.of` 工厂 | ✅ | TestListOf + 33_maps |
| Optional 增强（`ifPresentOrElse`/`stream`/`or`） | ❌ | 见 §3.14 |
| `Stream.takeWhile/dropWhile/ofNullable/iterate(seed,hasNext,next)` | ❌ | 见 §3.17 |
| `Collection` 工厂不可变（写入抛 `UnsupportedOperationException`） | ❌ | 见 §3.15 |
| 钻石操作符用于匿名类 | ❌ | 边界，低优先 |
| `VarHandle` | ❌ | 见 §3.30 |
| 模块系统 `module-info` | ❌ | **转译器范围外，跳过** |
| `Flow`（响应式流） | ❌ | 低优先 |

### JDK 10
| 特性 | 状态 | 说明 |
|------|------|------|
| `var` 局部变量 | ✅ | TestVar / TestVarContext |
| `List.copyOf/Set.copyOf/Map.copyOf` / `Optional.orElseThrow()` 无参 | ❌ | 见 §3.15 |

### JDK 11（LTS）
| 特性 | 状态 | 说明 |
|------|------|------|
| `HttpClient` | ❌ | 网络 IO，转译器大概率不支持，**跳过** |
| String：`stripLeading/stripTrailing/lines/transform/indent`（`trimIndent`/`stripIndent` 属 JDK 15） | 🟡 | 仅 `strip/isBlank/repeat` 有，见 §3.11 |
| `Files.readString/writeString`、`Path.of` | ❌ | 见 §3.26（注意确定性） |
| `Optional.isEmpty()` | 🟡 | 间接有 |
| `Collection.toArray(IntFunction)` | ❌ | 见 §3.15 |
| lambda 参数用 `var` | ❌ | 见 §3.27 |
| 嵌套类私有访问（nest-based access） | 🟡 | 内部类用例间接覆盖，未专项 |

### JDK 12
| 特性 | 状态 | 说明 |
|------|------|------|
| switch 表达式（预览，已 std in 14） | ✅ | 已覆盖 |
| `String.indent` | ❌ | 见 §3.11 |
| `Collectors.teeing` | ❌ | 见 §3.16 |
| `Files.mismatch` / `CompactNumberFormat` | ❌ | 低优先 |

### JDK 14
| 特性 | 状态 | 说明 |
|------|------|------|
| record（预览，已 std） | ✅ | TestRecord / TestRecordAdvanced |
| instanceof 模式匹配（预览） | ✅ | TestInstanceOfChain |
| 友好 NPE（消息含字段名） | ❌ | 边界，低优先 |

### JDK 15
| 特性 | 状态 | 说明 |
|------|------|------|
| 文本块（std） | ✅ | TestTextBlock |
| sealed（预览） | ✅ | TestSealed |
| `trimIndent/stripIndent` | ❌ | 见 §3.11 |

### JDK 16
| 特性 | 状态 | 说明 |
|------|------|------|
| record / instanceof 模式（std） | ✅ | |
| `Vector API` | ❌ | 转译器范围外，跳过 |
| `Stream.toList()` | ❌ | 见 §3.17 |

### JDK 17（LTS）
| 特性 | 状态 | 说明 |
|------|------|------|
| sealed（std）/ instanceof 模式（std） | ✅ | |
| `HexFormat` | ❌ | 见 §3.32 |
| `RandomGenerator` 体系 | ❌ | 见 §3.33 |
| 强封装 / 新渲染 | ❌ | 运行时不相关，跳过 |

### JDK 21（LTS，当前默认）
| 特性 | 状态 | 说明 |
|------|------|------|
| **Record Patterns（解构模式，JEP 440）** | 🟡 | 仅简单 `case Circle c`，缺 `case Point(int x, int y)` 解构 + 嵌套解构 |
| **Pattern Matching for switch（std）** | ✅ | TestPatternMatch 含 `when` 守卫 |
| **Sequenced Collections（JEP 431）** | ❌ | `SequencedSet/SequencedMap`、`reversed()`、`getFirst/Last` 见 §3.28 |
| **Virtual Threads（JEP 444）** | ❌ | 运行时不支持则会暴露缺口，见 §3.29 |
| Unnamed Classes & Instance Main（预览） | ❌ | 预览，低优先 |
| Scoped Values（预览） | ❌ | 见 §3.29 |

### JDK 22 / 23（需更高 JDK，你有 25/26 可测）
| 特性 | 状态 | 说明 |
|------|------|------|
| **Unnamed variables & patterns（`_`）** | ❌ | 高价值，见 §2.3 |
| Primitive types in patterns / instanceof（预览） | ❌ | 见 §3.27 |
| Implicitly Declared Classes（预览） | ❌ | 预览，跳过 |
| Module Import Declarations（预览） | ❌ | 跳过 |
| Statement before `super()`（预览） | ❌ | 见 §3.21 |

---

## 2. 边界 / 语义正确性缺口（按类别，附建议测试名）

### 2.1 Record 解构模式（JDK 21，🟡→建议补到 ✅）
- `TestRecordPattern.java`：在 `instanceof` 与 `switch` 中用 `case Point(int x, int y)` 解构；嵌套 record 解构 `case Tree(Leaf(int v))`；与 `when` 守卫组合。

### 2.2 switch 模式匹配边界（JDK 21）
- `TestSwitchNull.java`：JDK 21 允许 `case null ->` 与 `case String s ->` 混用；验证 null 落入 `case null` 而非抛 NPE。
- `TestPatternSwitchEdge.java`：`switch` 对 `null` 引用变量（旧规则仍抛 NPE，除非显式 `case null`）；守卫中 `when` 取反分支；类型模式 + 解构混合。

### 2.3 Unnamed 变量/模式（JDK 22，❌）
- `TestUnnamedVar.java`：`var _ = expr;`、增强 for 中 `for (var _ : coll)`、`case Foo _`、`try (_ r = ...)`、`(_ , b) = pair()`（若有分解）。需 JDK 22+。

### 2.4 任意精度数值（❌，高价值）
- `TestBigDecimal.java`：加减乘除、`setScale`/`ROUND_HALF_UP`、compareTo、精度、字符串构造、`BigDecimal.ZERO/ONE/TEN`、`stripTrailingZeros`、`toPlainString`。
- `TestBigInteger.java`：加减乘除、mod、pow、gcd、isProbablePrime、`shiftLeft/Right`、`and/or/xor/not`、符号、compareTo。

### 2.5 日期时间 API（❌，高价值，JDK 8）
- `TestLocalDate.java`：of/now（now 确定性需固定 → 改用 of 与 `plusDays/minusMonths`/`with`/isLeapYear/`getDayOfWeek`/`lengthOfMonth`/比较。
- `TestDurationPeriod.java`：Duration.between/ofHours/ofMinutes、plus/minus、toHours/toMinutes；Period.of/withYears/between/getYears。
- `TestDateTimeFormat.java`：DateTimeFormatter 预定义 + 自定义 pattern、`LocalDateTime.parse/format`、格式化确定性输出。
- `TestZonedDateTime.java`（P2）：ZonedDateTime/OffsetDateTime、`withZoneSameInstant`、ChronoUnit.between、时区换算。（需注意：时区数据一致即可确定性）

### 2.6 Math 精确与 IEEE 边界（❌）
- `TestMathExact.java`：`Math.addExact/multiplyExact/subtractExact/negateExact/toIntExact`（溢出抛 `ArithmeticException`）；`floorDiv/floorMod`；`copySign/nextAfter/nextUp/nextDown`；`hypot/atan2/expm1/log1p/ulp`。
- `TestMathRound.java`：`Math.round/floor/ceil/rint/signum/IEEEremainder/abs/max/min`；与 `(int)` 截断的区别；`Math.min`/`max` 的 NaN 行为。

### 2.7 整数位运算与无符号（❌）
- `TestIntegerBits.java`：`rotateLeft/rotateRight`、`bitCount`、`numberOfLeadingZeros/TrailingZeros`、`highestOneBit/lowestOneBit`、`reverse/reverseBytes`、`signum`、`toBinaryString/toHexString`。
- `TestUnsignedInt.java`：`toUnsignedLong`、`divideUnsigned/remainderUnsigned`、`parseUnsignedInt/formatUnsignedInt`、`compareUnsigned`、`Integer` 无符号右移边界。

### 2.8 浮点 NaN / 位级（❌）
- `TestFloatBits.java`：`Double.longBitsToDouble`/`doubleToLongBits`、`Float.intBitsToFloat/doubleToFloatBits`、`isNaN/isInfinite/compare`；`Double.compare` 的 NaN 排序；`Float.MIN_NORMAL/MAX_VALUE`。
- `TestNaN.java`：`NaN != NaN`、`0.0/0.0 = NaN`、`NaN + 1 = NaN`、`-0.0 == 0.0` 但 `1/-0.0 = -Inf`、`Double.MIN_NORMAL`、无穷大算术（`Inf + 1 = Inf`、`Inf - Inf = NaN`）。

### 2.9 assert / 字段修饰符（❌，语言特性）
- `TestAssert.java`：`assert cond : msg;` 两条分支。**注意**：需 JVM 开 `-ea`；若 e2e 运行不用 `-ea`，assert 恒通过——要在文档里标注「需 `-ea` 或改用显式 if+throw 验证行为」。若转译器运行环境不传 `-ea`，此用例价值有限，可降级为 P2。
- `TestFieldModifiers.java`：`volatile`/`transient` 字段声明（转译器至少要能**解析接受**；transient 在运行时不参与序列化可忽略语义，但关键字必须不报错）。

### 2.10 字符串新方法（🟡）
- `TestStringNewMethods.java`：`stripLeading/stripTrailing`、`lines()`（按行拆分并验证行数/内容）、`transform(Function)`、`indent(int)`、`trimIndent()/stripIndent()`（文本块缩进规整）、`chars()/codePoints()` 计数。

### 2.11 字符串码点与代理对（❌）
- `TestStringCodePoints.java`：`codePointAt/codePointCount/offsetByCodePoints`；emoji/代理对下 `length()`（UTF-16 单位）vs `codePointCount`（实际字符数）的差异；`Character.isHighSurrogate/isLowSurrogate`；`charAt` 切到代理对半截的边界。

### 2.12 字符串边界（❌）
- `TestStringEdge.java`：`null + "x"` → `"nullx"`（与 `"x" + null`）；`intern()`；`regionMatches/regionMatches(ignoreCase)`；`contentEquals`；`getBytes(StandardCharsets.UTF_8)` 确定性字节；`substring(begin,end)` 双参数边界；`String.valueOf` 各类型。

### 2.13 Optional 增强（❌，JDK 9/10）
- `TestOptionalFull.java`：`ifPresentOrElse`、`stream()`（扁平化）、`or(Supplier)`、`orElseThrow()` 无参、`flatMap`、`filter` 全 false、`empty().stream()` 为空、链式 `map.orElse`。

### 2.14 集合工厂不可变 + copyOf（❌，JDK 9/10）
- `TestCollectionFactory.java`：`List.of/Set.of/Map.of/ofEntries` 构造；对其 `add/put/set` 抛 `UnsupportedOperationException`（用 try/catch 打印异常类型）；`List.copyOf/Set.copyOf/Map.copyOf` 返回不可变；`toArray(IntFunction)`；`Collection.toUnmodifiable`。

### 2.15 Stream 进阶收集器（🟡）
- `TestCollectorsMore.java`：`toMap`（含 merge 函数）、`toSet`、`groupingBy + mapping/reducing`、`partitioningBy + collectingAndThen`、`averagingInt`、`summarizingInt`（getAverage/getSum/getMax）、`Collectors.teeing`（JDK 12）、`toCollection(TreeSet::new)`。

### 2.16 Stream 进阶中间操作（🟡）
- `TestStreamMore.java`：`takeWhile/dropWhile`（JDK 9）、`Stream.ofNullable`（JDK 9）、`iterate(seed, hasNext, next)`（JDK 9）、`Stream.toList()`（JDK 16）、`mapMulti`（JDK 21）、**可变规约** `collect(Supplier, Accumulator, Combiner)`（三参，验证并发 combiner 路径）、`flatMap` 扁平集合。

### 2.17 枚举深度（🟡）
- `TestEnumConstantBody.java`：常量专属方法体（`enum Op { ADD { int f(int a,int b){return a+b;} } ... }`）、抽象方法分发。
- `TestEnumSetMap.java`：`EnumSet.allOf/of/range/noneOf`、`EnumMap` 读写、`values()/valueOf()` 非法名抛 `IllegalArgumentException`。

### 2.18 泛型深度（🟡→❌）
- `TestRawTypes.java`：原始类型使用、未检查警告路径（转译器需正确处理擦除后的 checkcast）、`List` raw 调 `get` 返回 Object。
- `TestPecs.java`：PECS 完整（`<T extends Comparable<? super T>>`、`? super`、`? extends` 在参数与返回位置的分别）、`Collections.max/min/copy` 签名。
- `TestGenericThrow.java`：泛型 throws（`<T extends Exception> void f() throws T`）、泛型可变参数 + `@SafeVarargs`、自限定递归边界在**类声明**级 `<T extends Comparable<T>>`。

### 2.19 注解（❌）
- `TestAnnotations.java`：自定义 `@interface`（含默认值元素）、标记注解、带值的注解、@Repeatable（同处重复）、@Retention/@Target、注解可被读取（注意：`java/lang/reflect/*` 在 callchain cutoff 列表，反射读注解会断——**改用编译期存在性 + 注解值参与计算输出**来验证，不靠反射）。

### 2.20 方法/构造器引用全集（🟡）
- `TestRefKindsFull.java`：`String::new`（构造器引用）、`int[]::new`/`String[]::new`（数组构造引用）、`ArrayList<String>::new`、`super::method`、`this::method`、静态方法引用、实例方法引用（任意接收者 `String::length`）。验证每种生成正确的调用。

### 2.21 异常深度（🟡）
- `TestSuppressed.java`：try-with-resources 中 close 抛异常且主体也抛 → `getSuppressed()` 含被抑制异常（用 catch 后打印 `getSuppressed().length` 与类型）。
- `TestMultiCatchOrder.java`：多 catch 中更具体异常须在前（`IOException` 早于 `Exception`），验证分派到正确分支。
- `TestExceptionChain.java`：`initCause`/`getCause` 链式、`fillInStackTrace` 返回、自定义 `Throwable` 子类。

### 2.22 数组边界（🟡）
- `TestArrayBounds.java`：负索引抛 `ArrayIndexOutOfBoundsException`、负长度 `new int[-1]` 抛 `NegativeArraySizeException`、`Arrays.copyOfRange` 越界处理、`clone()` 多维数组浅拷贝、`System.arraycopy` 重叠区间（前向/后向覆盖）、`Arrays.asList` 返回的列表 `set` 影响原数组但 `add` 抛异常。

### 2.23 整数边界与复合赋值（❌）
- `TestIntOverflow.java`：`Integer.MAX_VALUE + 1 == Integer.MIN_VALUE`（回绕）、`Long.MIN_VALUE / -1 == Long.MIN_VALUE`（不抛异常）、`Math.abs(MIN_VALUE) == MIN_VALUE`、`-Integer.MIN_VALUE` 回绕、移位 `1 << 31` 掩码后结果、无符号右移负数。
- `TestCompoundAssign.java`：`byte b = 0; b += 200;` 隐式窄化回绕（结果 != 200）、`short += short`、`char += char`、复合赋值中 `b = (byte)(b + 200)` 与 `+=` 等价性、复合赋值里的类型提升陷阱。

### 2.24 除零/取模边界（❌）
- `TestDivMod.java`：`5 / 0` 与 `5 % 0` 抛 `ArithmeticException`（整数）、`5.0 / 0.0 = Infinity`、`5.0 % 0.0 = NaN`、`-5 % 3` vs `5 % -3` 符号、对 `-1` 取模、`Long`/负零特例。

### 2.25 自动装箱边界（🟡）
- `TestAutoboxEdge.java`：缓存边界 `Integer 127 == 127` 为 true 而 `128 == 128` 为 false（同一方法内两次 `valueOf(128)`）、拆箱 `null` 抛 `NullPointerException`、装箱参与 varargs 时的数组化、基本类型数组与包装数组在重载解析中的抉择。

---

## 3. P2 / 版本特定 / 低优先（列出即可，按需补）

- **3.26** `TestFilesApi.java`：`Files.readString/writeString/readAllLines`、`Path.of`、`Path.normalize()`、`Files.mismatch`。⚠️ 确定性：避免打印临时文件路径，只比对文件**内容**；写临时文件用固定名 + `@After` 清理（e2e 仅比对 stdout，文件副作用不在比对范围，仍需清理）。
- **3.27** `TestLambdaVar.java`：JDK 11 lambda 参数 `var x -> ...`、`(@Nonnull var x)`；JDK 23 基本类型模式 `case int i`（预览）。
- **3.28** `TestSequencedCollections.java`：JDK 21 `SequencedSet/SequencedMap`、`reversed()`、`getFirst/getLast`、`addFirst/addLast`。
- **3.29** `TestVirtualThread.java`：`Thread.startVirtualThread(...)` / `Executors.newVirtualThreadPerTaskExecutor()`；若运行时不支持则**故意暴露缺口**（与 `TestSynchronized` 同需确定性约束）。`ScopedValue` 可同文件或单列。
- **3.30** `TestWaitNotify.java` + `TestAtomics.java`：经典 `wait/notify/notifyAll`、显式 `Lock/Condition`、`AtomicInteger/AtomicReference/AtomicLong`、`CountDownLatch`/`CyclicBarrier`。⚠️ 必须保证输出确定性（统一锁、固定线程数、禁止依赖调度顺序）。
- **3.31** `TestCompletableFuture.java`：`supplyAsync/thenApply/thenCombine/thenAccept`；需 `join()` 等待 + 固定线程池（`ForkJoinPool.commonPool()` 可能不确定 → 用 `new ForkJoinPool(2)`）。
- **3.32** `TestHexFormat.java`（JDK 17）+ `TestFormatLocale.java`：`HexFormat`、定区 `NumberFormat/DecimalFormat`、`String.format` 带 `Locale`、`UUID.randomUUID`（⚠️ 不确定，跳过或用固定值）。
- **3.33** `TestRandomSeed.java`：固定种子 `new Random(42)` 确定性、可变 `SplittableRandom`、`ThreadLocalRandom`（⚠️ 不确定，用固定种子）。
- **3.34** 跳过项：`module-info`（转译器范围）、`HttpClient`/网络、`native` 方法（无法转译）、`Vector API`、`String Templates`（已撤回）、`StrictMath`/模块导入声明（预览）、`JFR`/`VarHandle` 高层用法（除非 runtime 支持）。

---

## 4. 优先级汇总（建议补充的测试文件清单）

### P0（转译器语义正确性高危，建议优先）
1. `TestRecordPattern.java` — record 解构模式 + 嵌套（§2.1）
2. `TestSwitchNull.java` — `case null` + null 引用（§2.2）
3. `TestUnnamedVar.java` — 无名变量/模式（§2.3，需 JDK 22+）
4. `TestBigDecimal.java` / `TestBigInteger.java` — 任意精度（§2.4）
5. `TestLocalDate.java` / `TestDurationPeriod.java` / `TestDateTimeFormat.java` — 日期时间（§2.5）
6. `TestMathExact.java` / `TestMathRound.java` — Math 精确与 IEEE（§2.6）
7. `TestIntegerBits.java` / `TestUnsignedInt.java` — 位运算与无符号（§2.7）
8. `TestFloatBits.java` / `TestNaN.java` — 浮点位级与 NaN（§2.8）
9. `TestAssert.java`（标注需 `-ea`）/ `TestFieldModifiers.java` — 语言特性（§2.9）
10. `TestIntOverflow.java` / `TestCompoundAssign.java` / `TestDivMod.java` — 数值边界（§2.23/§2.24）

### P1（API 广度与深度边界）
11. `TestStringNewMethods.java`（§2.10）12. `TestStringCodePoints.java`（§2.11）13. `TestStringEdge.java`（§2.12）
14. `TestOptionalFull.java`（§2.13）15. `TestCollectionFactory.java`（§2.14）16. `TestCollectorsMore.java`（§2.15）
17. `TestStreamMore.java`（§2.16）18. `TestEnumConstantBody.java` / `TestEnumSetMap.java`（§2.17）
19. `TestRawTypes.java` / `TestPecs.java` / `TestGenericThrow.java`（§2.18）20. `TestAnnotations.java`（§2.19）
21. `TestRefKindsFull.java`（§2.20）22. `TestSuppressed.java` / `TestMultiCatchOrder.java` / `TestExceptionChain.java`（§2.21）
23. `TestArrayBounds.java`（§2.22）24. `TestAutoboxEdge.java`（§2.25）

### P2（版本特定 / 低优先，见 §3）
25–34：`TestFilesApi`、`TestLambdaVar`、`TestSequencedCollections`、`TestVirtualThread`、`TestWaitNotify`/`TestAtomics`、`TestCompletableFuture`、`TestHexFormat`/`TestFormatLocale`、`TestRandomSeed`，以及标注的跳过项。

---

## 5. 写新用例的硬约束（来自 `scripts/run_tests.py`，违反则白写）
- expected 只按**类名**索引 → 类名全局唯一，文件名 = public 类名。
- javac **单文件**编译 → 用例自洽，辅助类写同文件；不可跨包。
- 只比对 **stdout** → 异常须自 catch 后 `println` 异常类型/消息，不可直接冒泡；不可用 `-ea` 之外的 JVM 开关影响结果（assert 用例需显式标注）。
- 输出须**确定** → 禁用无种子 `Random`、系统时间、`identityHashCode`、并行流、线程调度顺序、临时文件路径、UUID.randomUUID。
- 反射读注解/字段被 callchain cutoff 截断（`java/lang/reflect/*`）→ 验证注解请走「编译期存在 + 值参与计算输出」，不要靠运行时反射。

> 说明：本批建议中有若干用例（虚拟线程、Vector、模块、HttpClient、部分 runtime 并发原语）转译器当前可能**尚不支持**，写入后会失败——这正能暴露缺口、驱动 codegen/runtime 修复，符合 e2e 测试「锁住正确行为」的目标。建议先跑 P0 一批，再用 `--no-run` 探路 P2 的高版本特性。
