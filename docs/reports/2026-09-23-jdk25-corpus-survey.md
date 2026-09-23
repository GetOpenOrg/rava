# JDK25 语料适配——失配点全清单与适配方案（纯调查报告）

> 2026-09-23，纯调查代理交付（主会话入档）。实测环境：本机双 JDK
> （openjdk@21/21.0.11 与 openjdk@25/25.0.4.1），jimage 提取 java.base 后
> javap 逐类对比（提取物在 `/tmp/jdkcmp/{j21,j25}`）。
> 证据等级：【实测 javap】/【文档推证】。

## 一、失配点清单（三族）

### 族 1：jdk/internal/math 手写 overlay 结构失配（17 处错误主因）【实测 javap】

JDK25（实为 JDK 22 引入、JDK 23 深化的 Ryū/Schubfach 重构）核心变化：
**新增抽象基类 `jdk.internal.math.ToDecimal`**，把"实例字段缓冲"模型改为
"byte[]/index 显式传参"模型。

| 类 | JDK21 | JDK25 | 变化 |
|---|---|---|---|
| `ToDecimal` | 不存在 | 新增抽象类 | 字段 `latin1`、`JLA(JavaLangAccess)`；方法 `putChar/putDigit/put8Digits(+Latin1/UTF16 static)/y/removeTrailingZeroes/putSpecial/length/special` |
| `DoubleToDecimal` | `final class`，实例字段 `bytes:byte[]`+`index:int` | `extends ToDecimal`，**删除 bytes/index 字段** | 新增静态实例 `LATIN1`/`UTF16`、`$assertionsDisabled`、实例方法 `putDecimal(byte[],int,double)`、私有 `toDecimalString(double)`；`toDecimal/toChars/toChars1..3/lowDigits/exponent` 全部签名改为 `(byte[], int, ...)` 前缀传参；`appendTo(double,Appendable)` 删除（`y/append/appendDigit/append8Digits/removeTrailingZeroes/charsToString` 上移 ToDecimal 或删除）；`toString(double)`/`split(double,FormattedFPDecimal)` 签名不变 |
| `FloatToDecimal` | 同上（float 域） | 同上（float 域） | 同构变化 |
| `MathUtils` | — | — | **字节码逐指令一致**（javap -c diff=0，1227 条 ldc2_w 计数相同）→ G 表/POW10/常量零改动 |
| `FormattedFPDecimal` | — | — | 字段集不变（f/e/n/digits/exp）；新增 `valueForDoubleToString/getSignificand/getPrecision/getExp` + 私有 `split` → 手写引用面（字段访问器+set）不受影响 |
| `FloatingDecimal` | — | 删 3 常量（BIG_DECIMAL_EXPONENT/INFINITY_LENGTH/NAN_LENGTH） | 手写未引用 → 兼容 |
| `FDBigInteger` | — | `<init>(long,char[],int,int)`→`(long,byte[],int,int)` | 无手写，转译自动适应 |

下游连带【实测 javap】：`AbstractStringBuilder`（实例方法 `ensureCapacityInternal/newCapacity/inflate/putCharsAt/appendChars/...` 全部 static 化+buffer/coder 显式传参）；`Integer`（删 `stringSize/getChars`、`digits` char[]→byte[]、删 DigitTens/DigitOnes）；`Long`（删 `stringSize/getChars/fastUUID`）——转译域正常语料差异，无手写依赖。

调用链事实【实测 javap】：JDK21/25 的 `Double.toString`/`Float.toString` 均仍调 `DoubleToDecimal/FloatToDecimal.toString`（签名不变）；Formatter `%f/%e/%g` 均仍走 `FormattedFPDecimal.valueOf(DIC)`（手写覆盖，split 不触达）；JDK25 的 `ASB.append(double)` 改走 `LATIN1.putDecimal`。

### 族 2：DecimalDigits 迁移重写 + Random/concurrent 邻域

- **DecimalDigits**【实测 javap】：JDK21 `java.util.DecimalDigits`（实现 `java.util.Digits`，`digits(long,byte[],int,MethodHandle)`）→ JDK25 **`jdk.internal.util.DecimalDigits`（全新静态工具类）**：`stringSize(I/J)`、`uncheckedGetCharsLatin1/UTF16(I/J, int, byte[])`、`getChars`、`putPair` 族、`UNSAFE` 字段。消费面【实测 javap -c】：`Integer.toString`、`Long.toString`、`StringConcatHelper`（字符串拼接 fast path）、`ASB`。**这就是 1f3f3ed 提交实测"运行卡 DecimalDigits.stringSize stub"的根因**。
- **java.util.Random**【实测 javap -c】：JDK21/25 **字节码零差异**（含方法体）。docs 中"E0432 = JDK25 的 Random 内部结构变化"判定**与实测不符**，需重定位（见 §五）。
- **RandomSupport**【实测 javap】：JDK25 新增 `computeNextExponentialSoftCapped(RandomGenerator,double)` + `MAX_EXPONENTIAL` 常量；`RandomSupport$RandomGeneratorProperties` 内部类移至 `java.util.random`（`RandomGeneratorFactory$RandomGeneratorProperties`）。手写 `random_support_impl.rs` 提供的 mixMurmur64/mixStafford13 等方法在两版均在 → 兼容。
- **concurrent**【实测 javap】：`AtomicLong` 删除 native `VMSupportsCS8()` + `VM_SUPPORTS_LONG_CAS` 字段（手写 `atomic_long_impl.rs` 变冗余但**无害**——固有 fn 在两版都编译通过，JDK25 下仅 dead_code）；`ThreadLocalRandom` 删 BAD_* 3 常量/`setInheritedAccessControlContext`，增 `JLA`；`RandomGenerator` 增 default `equiDoubles` + lambda 编号偏移。
- jdk.internal.misc 杂项【实测】：`InnocuousThread$1-4`/`InternalLock`/`MainMethodFinder`(→`MethodFinder`)/`VirtualThreads` 删，`CDS$ProcessLauncher` 增等（转译域）。

### 族 3：LVT 间隙声明反向装箱（1 处）【文档推证】

出处 `docs/tasks.md`。邻域已定位：`codegen/stack.py` `_store_local`——形参槽 `decl=None` 强制（:387）、null/Object 形参分支（:622/:630）不覆盖"具体引用类型形参+异型引用值"（需 `From<Object>`/`_box_object` 重建），:677 槽位复用判定。**同域先例已修**：TestBigInteger 形参重绑定（10-15 行）。JDK25 暴露的是**间隙（非形参槽）**的一处新形态，具体位点需 JDK25 scratch 实测。

## 二、手写 overlay 失配明细

| 文件（runtime/java_runtime/src/jdk/internal/math/） | 行数 | JDK25 兼容性 | 失配点 |
|---|---|---|---|
| `double_to_decimal_impl.rs` | 1049（G 表 618） | **失配** | `__get_index/__set_index` 9 处 + `__get_bytes/__set_bytes` 6 处（`_append`/`_append_digit`/`_remove_trailing_zeroes`/`_append8_digits`/`_exponent`/`_to_chars1..3`/`_low_digits`/`_to_chars`/`_to_decimal*`/`_chars_to_string`/`new`）→ JDK25 生成结构无字段即无访问器 → E0599；`pub fn appendTo` 对应方法 JDK25 已删（无害 dead_code）；G 表/常量/MathUtils 内联零改动 |
| `float_to_decimal_impl.rs` | 472 | **失配** | 同构（9+6 处访问器） |
| `formatted_fp_decimal_impl.rs` | ~200 | **兼容** | 字段集不变；引用面仅字段访问器+set |
| `floating_decimal_impl.rs` | 143 | **兼容** | 引用 `DoubleToDecimal::toString`/`FloatToDecimal::toString` 静态入口（两版签名同）；FloatingDecimal 删的常量未被引用 |

生成侧实证【实测 scratch】：`build/test_string_search/.../double_to_decimal.rs` 的 `java_class!` 块含 `pub bytes: JArray<i8>`/`pub index: i32` 字段与 `impl_methods = "appendTo;new;toString"`——JDK25 语料生成时字段消失、super_class 变 `jdk/internal/math/ToDecimal`、新增 LATIN1/UTF16/$assertionsDisabled。

## 三、适配方案分层

- **L1 兼容改写（推荐终态，适用 math 双件）**：把两个 `_impl.rs` 的私有算法改为 **JDK25 形态**（byte[]/index 显式传参——恰是无实例字段形态，天然双版本兼容）：入口 `toString` 自包含分配缓冲；`appendTo` 保留（JDK21 语料消费，JDK25 下 dead）；删 `new` 覆盖（`<init>` 存根两版均不被触达：JDK21 链 toString/appendTo/FormattedFPDecimal.valueOf 已全覆盖，JDK25 链走手写 toString）。G 表 618+154 行零改动。**不做**"按字段存在性分派"（Rust 无 per-字段条件编译）。
- **L2 版本分治副本（不推荐）**：overlay 按 JDK 版本目录分派需改 `project_writer._is_handwritten`/`method_gen._scan_impl_files`/`callchain`/`native_upcalls` 四处扫描（~40-60 行机制），且 G 表双份拷贝，违背"runtime/ 单一真源"终态。
- **L3 删手写落回转译（S-19 先例）**：对 DoubleToDecimal 不推荐作首选——JDK25 转译链 `ToDecimal.put8DigitsUTF16` 触达 **`JLA.uncheckedPutCharUTF16`/`SharedSecrets.getJavaLangAccess` 新边界依赖**（JDK21 链无 JLA），会拖 JavaLangAccess 大接口进闭包；且手写存在闭包盲区先例（TestPrintStreamApi `stub: FloatToDecimal.toString`，P-3 遗留——类本体不进闭包时手写不编译）。**DecimalDigits 可考虑 L3**（纯算术+UNSAFE 可用 JArray::set 等价），但作为 JDK25 新增类最直接是**新增手写 `jdk/internal/util/decimal_digits_impl.rs`**。
- **AtomicLong.VMSupportsCS8**：保留现状即双兼容（JDK25 dead_code），终态可删（14 行文件）。

## 四、语料切换机制现状

已有【实测代码】：`scripts/jdk_select.py`（`--jdk N`→JAVA_HOME，brew Cellar+java_home 兜底+release 校验）；`codegen/jdk_resolver.py` find_java_home（JAVA_HOME 优先→`prefer_major`：callchain.py:249 从用户 .class major-44 自动推导→最小上界）；`transpile.py` javac 与 jmods 同源；`run_tests.py --jdk`。**双 JDK 交替安全**：`project_writer._write_jdk_mod_tree` 清除"带生成标记但本轮未写"的陈旧 .rs，scratch 自愈。**缺口**：无语料版本 stamp（build/<test>/ 内不可辨 JDK 版本）；手写 overlay 无版本维度（本次失配本质——单一真源服务双版本语料）。JDK25 轮最小机制增量：`--jdk 25` 跑批时在 scratch 落 `.jdk-version` 标记（~10 行，防混用脏树），**不需要** overlay 版本目录分治（L1 已消除需求）。

## 五、工作量估算与待实测清单

| 项 | 量级 |
|---|---|
| double_to_decimal_impl.rs L1 改写 | ~15 函数参数化+3 入口，净改 180-250 行（G 表零动） |
| float_to_decimal_impl.rs L1 改写 | ~120-180 行 |
| decimal_digits_impl.rs 新增（stringSize×2+uncheckedGetChars×4+DIGITS 表构造，UNSAFE→JArray 等价） | ~150-250 行新文件 |
| LVT 间隙反向装箱 1 处（stack.py _store_local 邻域，TestBigInteger 先例） | 10-20 行 |
| atomic_long_impl.rs 处置 | 删 1 件（14 行）或保留 |
| E0432 `Random__nextInt_i_base` | 调查 0.5-1 天+修复未知（判定存疑，见下） |
| scratch jdk-version stamp（可选） | ~10 行 |

**需 JDK25 实测**（原始 17/23 错误日志未落盘——baseline-classification §七明记"补跑务必 tee 落盘"）：

1. `python scripts/run_tests.py --jdk 25 |& tee /tmp/jdk25-full.log`（166 全量，错误按五分类落盘后逐条对号本清单）
2. E0432 重定位：任一触发用例生成 scratch 后 `grep -rn "Random__nextInt_i_base" build/<test>/`（定义侧 vs 引用侧缺失点；java.util.Random 本体 0 diff，疑 ThreadLocalRandom/RandomGenerator 邻域或 javac25 产物形态）
3. LVT 间隙位点：JDK25 scratch 生成 Rust 中 grep 反向装箱报错用例的 `let` 声明区间
4. concurrent 成员解析的其余错误实例（除 AtomicLong 外的 E0432/E0599 样本）

**结论**：17 处错误中可预判构成 = math 双件访问器缺失（每用例 8 类 E0599 × 触达点位）+ DecimalDigits 存根（运行期）+ concurrent/LVT 散点；精确对账必须待 JDK25 落盘日志。math 双件是唯一"手写源必须动"的硬失配，其余均为转译域或新增手写件。
