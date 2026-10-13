# 运行时性能深剖（2026-10-11，分支 rt-perf-prof）

> 对象：docs/tasks.md「10-11 晚收拢」第 4 条的三项——`Method.invoke` 微基准、`rd::unbox_*` 的
> `__obj_str` 文本解析、无 GC 第一步后 PartitionInteger 的 stream 管线开销。任务书以「热点定位
> 到行 + 占比数字」为验收；本报告给出同机（kr1）rava 与 JVM 对照、热点函数与行号、占比，
> 以及随附微修复与 unbox 承载方案评估。基线提交 7b0bf81a（batch-1011k + 剖析脚本），
> 修复链 9157a73e → fc68236d → 57eda4ee（P-B 收窄定稿）。作业索引见 §八。

## 〇、交接（6 小时到点，2026-10-12）

- **收尾在途**：rtpp-verify4（57eda4ee 的 7 例回归复跑）在云服务器排队/执行中，结果落
  `cluster_results/job/rtpp-verify4/01_*.log`（grep `\[PASS|\[FAIL`）。预期 7/7：上一档
  fc68236d 已 5/7 通过，两个失败各自的修因都已落地——TestReflectBoxCast 的 Float 行经
  P-B 收窄回文本路径；TestUnboxWrapperFaces 的 expected 已入库（上轮 SKIP 仅因 expected
  缺席）。若仍有失败，按差分定位，回滚面 = 单文件 `runtime/java_runtime/src/reflect_dispatch.rs`
  的 unbox_i32 包装类臂（revert 到 `_ => None` 即回到纯文本路径）。
- **已交付**：基线剖析与三对象结论（§二–§四）、微修复两处（§六，generator / runtime
  子 crate 本机 check 过、发射构建经 rtpp 作业验证）、A/B 与收窄依据（§三.4/§三.5）、
  下一步建议与遗留（§七）。分支已推 origin 与 github（3f969d41）。
- **未做**（超时让位）：43 µs ↔ 4.3 µs 差异的 43f37f63 同机对照；翻译 Float.hashCode
  应答 0 的根因（独立缺陷嫌疑，§七.0）；jackson jc1 的 String 跨 crate 4% 复测（S7 线）。
>
> 注：jackson jc1 分派视图约 19% 已归 S7 线（tasks.md 第 4 条），不在本报告范围。

## 一、方法

- 剖析：`scripts/perf_line_profile.sh`——release 档带符号
  （`CARGO_PROFILE_RELEASE_DEBUG=line-tables-only`、strip 关），每例两次采样：F997 无调用栈
  （自采样，`<case>.self.txt`）+ F300 dwarf 调用链（children 视图，`<case>.children.txt`）。
  `perf report -l` 在该构建形态下未产出 srcline 行（符号级行为主）；「定位到行」以符号 →
  本仓库源码位置的逐一映射给出（运行时侧函数行号取自当期源码，翻译体符号标注其 JDK
  方法语义），LLVM 内联使被调方开销部分落在调用方符号内，聚簇解读时已计及。
- 对照微基准：`scripts/perf_rt_bench.sh`——rava 与参考 JVM（SerialGC）同机同口径：
  1. `BenchReflectArg`：`idInt(int)`（拆箱+装箱）vs `idRef(Integer)`（引用直传）vs
     `idInt2(int,int)`——同一分派协议下差值即 unbox/box 边界开销；
  2. `RldcSegments`：TestReflectLdcHotLoop 三段（ldc / 类字面量 / Method.invoke）逐段墙钟；
  3. `BenchStreamLoop`：`Arrays.stream(combo).map(...).sum()` vs 手写 for 循环（每元素 ns）。
- JVM 计时均为参考 JDK 21（SerialGC），5 次取中位数。

## 二、对象 1：Method.invoke 微基准（TestReflectLdcHotLoop）

### 1. 一次 invoke 的完整开销清单（代码侧盘点，基线 7b0bf81a）

`step.invoke(o, i & 255)` 每次迭代经过（源码位置）：

| 步骤 | 位置 | 开销性质 |
|---|---|---|
| `Integer.valueOf(i & 255)`（现场装箱） | 翻译体 | 128..255 缓存外，每迭代新分配翻译 Integer |
| varargs `Object[]` 装配 | 翻译体 | 1 次数组分配（JVM 同） |
| Method.invoke / 访问器链 | 翻译体（Method.invoke → accessor.invoke → invoke0） | 若干次虚分派 + 访问检查 |
| **`__member_key` 描述符重建** | `method_impl.rs:36-58` | **每次 invoke 重建 (类名, 方法名, 描述符) 三个 String：4 次 Java String→Rust String 转换 + 约 5 次分配**（参数 / 返回类型逐个 format + replace + push） |
| `is_static_descriptor` | `reflect_dispatch.rs:279` | 元数据表二分 + **类方法表内线性 find（逐方法 String 比较）**；native_invoke 与 reflect_invoke 各查一次 |
| 引用实参可赋值检查 | `reflect_dispatch.rs:191-203` → `class_impl.rs:304` | `is_instance_of`（描述符超类型表）失败后 `__name_assignable` **线性扫 class_hierarchy 全表**（数千行；已修，§六） |
| 接收者检查 | `reflect_dispatch.rs:141` | `is_instance_of` + `superclass_chain().any()` 沿父类链走 |
| 分派表查找 | `reflect_dispatch.rs:80-83` | 冻结表二分（82f55952 后无锁无克隆） |
| 分派臂 → 目标 | 用户 crate `__reflect_dispatch` | `try_cast` 视图 + `unbox_i32(实参)` |
| **`unbox_i32` 文本往返** | `reflect_dispatch.rs:303`（基线） | **翻译 Integer.toString 体（构建 Java String）+ UTF-16→UTF-8 转换 + `parse::<i32>()`**（P-B 目标，§三） |
| 返回装箱 | `reflect_dispatch.rs:217-233` | 原生盒 downcast + `Integer.valueOf_i`（缓存外新分配） |
| `take_bad_arg` | `reflect_dispatch.rs:294` | TLS Cell 读写 |

### 2. 同机对照与热点占比（kr1，基线 7b0bf81a，作业 rtpp-prof1）

整例墙钟（3 次）：rava 4.25 / 4.31 / 4.31 s，JVM 0.11 / 0.11 / 0.12 s（约 39 倍）。
逐段（RldcSegments 计时副本，n = 1×10⁶，中位）：

| 段 | rava | JVM | 倍数 |
|---|---:|---:|---:|
| ldc（字符串常量驻留 + length） | 31.3 ms（31 ns/迭代） | 7.9 ms | 4.0× |
| 类字面量 / getClass / isInstance | 496 ms（496 ns/迭代） | 3.2 ms | 155× |
| Method.invoke（每次迭代 2 调用） | 3529 ms（**1.76 µs/调用**） | 62.9 ms（31 ns/调用） | 56× |

分诊 C 节所记「约 43 µs/迭代」（perf_hotpath，us1，43f37f63，整例 43.197 s / 1M 迭代；
彼时测试尚用 `hashCode()` 取值绕开反射装箱强转缺陷，6333f82d 已恢复直接强转）与本测
kr1 上 4.25 s 差约 10 倍：分段计时显示三段合计 + 启动即构成整例，kr1 无单段能达到 40 s
量级，疑为彼时提交 lineage（43f37f63 → batch-1011k 之间并入 fix-invoke-box-cast 等）或
机差所致，**未闭合**——如需精确归因，可在 kr1 补一次 43f37f63 对照（遗留，非本步必需）。
以本报告 kr1 同机数字为准：**1.76 µs/调用**。剩余 56 倍的构成（受控对照
BenchReflectArg，ns/调用，中位）：

| 测点 | rava | JVM | 差值含义 |
|---|---:|---:|---|
| idRef(Integer)（引用直传） | 1732 | 20 | 协议机底座：member_key 重建 + 双重 static 判定 + 实参校验 + 上溯分派 |
| idInt(int)（拆箱+装箱） | 1910 | 34 | 边界开销 = idInt−idRef = **178 ns**（valueOf 新分配 + unbox + box_value） |
| idInt2(int,int) | 2465 | 29 | 每加一个 int 实参 +555 ns（第二个 valueOf 新分配 + unbox + 边界） |

自采样前三热点（F997，全二进制约 82% 样本在 invoke 段；行号为本仓库源）：

1. **String 构建 / 转换簇 ≈ 17%**：`String` 的 `Display::fmt` 7.05%（`string_ext`）+
   `String::units` 3.48% + `core::fmt::write` 2.27% + `String__inner` vtable 胶水 2.16% +
   `String.__init_on_arr` 1.23% + `write_str`/`Display` 0.91%——主要来自
   **`__member_key` 每次 invoke 重建 (类名, 方法名, 描述符)**（`method_impl.rs:36-58`，
   经 `__reflect_key`（`:62`）← `invoke0`（`native_accessor_impl.rs:27`）每调用执行：
   3–4 次 Java String→Rust String 转换 + 约 5 次堆分配）与 unbox 文本路径的转换份额。
2. **分配器簇 ≈ 16%**：malloc 6.46% + cfree 3.91% + realloc 1.46% + raw_vec
   reserve/finish_grow 2.95% + format_inner 1.0% + calloc 0.41%（varargs 数组
   `Store::to_vec` 1.52% + `JArray::to_vec` 0.54% 为其需求方之一）。
3. **分派协议簇 ≈ 10%**：`is_static_descriptor` 2.27%（`reflect_dispatch.rs:279`，
   每调用两次：`native_invoke:180` 与 `reflect_invoke:116`）+ `lookup` 1.09% +
   `native_invoke` 1.05% + `reflect_invoke` 0.73% + `descriptor_params` 胶水 0.80% +
   用户 `__reflect_dispatch` 臂 0.71% + `try_cast` 0.52% + `box_value` 0.57% +
   `handle::release_last` 0.91%。

其余可辨识簇：`invoke0` 本体 6.42%（含 member_key 内联）、Integer 装箱簇 ≈ 6%
（`valueOf_i` 1.23 + `getChars` 1.05 + `toString_i` 0.89 + `__init_on_i` 0.84 +
`AbstractStringBuilder` 1.23）、`Method_Direct.invoke` 翻译体 1.34%、
`__caller_sensitive` 压栈 0.68%、`injected_invoker::registered` 0.59%、
`image_rt::image_mirror` 0.68%、`exec_context::current` 0.39%。

**结论**：56 倍中约四分之一在「每次 invoke 重建分派键」（member_key），约六分之一在
其引发的分配，约十分之一在分派协议本身（含两遍 static 判定）；装箱本身（valueOf）约 6%。
类字面量段 155×（496 ns/迭代：两次镜像比较 + isInstance）与 ldc 段 4× 相对次要。

### 3. 分派键免重建（2026-10-12，分支 invoke-key-opt，commit c0713ed8）

§七.1 所记「最大单项」已实施，形态比当时设想的两条路线（身份侧表 / 发射层预计算）更直接：
**表构造的成员对象 slot 即声明类方法元数据表的行号**（`Class.__method_from_meta` /
`__table_method` / `__table_declared_ctors` / Proxy$Dyn 接口方法直构一律如此写入；字节码侧
唯一构造点 `Method.copy()` / `Constructor.copy()` 原样转传 slot）。据此 `__member_key`
（method_impl.rs / constructor_impl.rs）按下式免重建：

- **首用解析**：(clazz 身份, slot) → 类方法表行二分（同 `meta::row` 口径）→ `rows[slot]`
  的静态 `name` / `descriptor`——与发射层分派闭包键、`is_static_descriptor` 同一真源
  （parameterTypes / returnType 本就从该描述符还原，直取即恒等往返），并与 Method 的
  名字段核对（防伪 slot；不符判非表形态）。`__reflect_key` 返回三元组改为全 `&'static str`。
- **进程级缓存**：判定结果按 (clazz 身份, slot) 记入 `__process_static!` + RwLock 表；
  此后每次 invoke 只做一次哈希取静态引用——**零 Java String→Rust 转换、零分配**。
  回收论证：类镜像是进程级规范对象（CLASSES / PRIMITIVES 驻留表或引导映像 `&'static`
  永久持有），身份无回收复用；条目数 ≤ 被 invoke 的不同声明数（编译期有界）。
- **非表形态**（核对不符 / 表外类 / slot 越界）恒走原重建路径（逐字保留），结果驻留
  `&'static`（同一键一份；现实语料全部构造点为表构造，池为空）。键只依赖声明三元组
  （final 字段承载），与 setAccessible 等访问标志无涉。

同机 A/B（sg1，作业 ikopt-base（batch-1012 = 99de3268）→ ikopt-after（c0713ed8）顺序执行，
各 5 次取中位；JVM 参照同机同刻）：

| 测点 | 基线 batch-1012 | invoke-key-opt | 变化 |
|---|---:|---:|---:|
| RldcSegments invoke 段（1M 迭代 ×2 调用） | 3030.3 ms（**1.515 µs/调用**） | 1292.5 ms（**0.646 µs/调用**） | **−57.4%** |
| BenchReflectArg idRef（协议底座） | 1636 ns | 587 ns | −64.1% |
| BenchReflectArg idInt / idInt2 | 1568 / 1907 ns | 663 / 829 ns | −57.7% / −56.5% |
| class 段（未瞄准） | 553.9 ms | 531.0 ms | −4.1% |
| ldc 段 | 32.8 ms | 33.8 ms | 持平 |
| TestReflectLdcHotLoop 整例（**debug 档**，perf_hotpath 口径） | 42.92 s | 25.18 s | −41.3% |
| JVM invoke 参照 | 35.5 ns/调用 | 34.4 ns/调用 | rava/JVM 由 43× 降至 19× |

剖析簇前后（F997 自采样，% 全样本，同机同构建口径）：

| 簇 | 基线 | 优化后 |
|---|---:|---:|
| member_key String 构建/转换簇（Display::fmt + units + fmt::write + write_str + format_inner + `__type_desc`） | ≈ 15.0% | ≈ 8.2%（`__type_desc` 1.20%→出 top45；units 2.50→0.95；write_str 0.80→0.33） |
| 分配器簇（malloc + cfree + realloc + raw_vec） | ≈ 15.3%（malloc 6.55 / cfree 3.77 / realloc 1.43 / raw_vec 2.27） | ≈ 7.3%（malloc 4.00 / cfree 1.62 / realloc 0.76 / raw_vec 0.91） |
| varargs `Store::to_vec` | 1.23% | 0.62% |
| `Method_Direct::invoke0` self（含 member_key 内联） | 6.40% | 4.86% |
| `is_static_descriptor` | 2.88% | 4.72%（占比升系总分母缩小，绝对量近似持平） |
| **新增：键缓存哈希**（`RandomState::hash_one` + Sip13 `Hasher::write`） | — | ≈ 2.4% |

要点：

- **member_key 簇退出剖析 top 的 invoke 部分**：剩余 Display::fmt 5.38% 的主要需求方是
  `Class.isPrimitive` 的类名 `format!`（children 视图 12.59% 在 fmt 下——class 段的
  `isInstance` → `isAssignableFrom` → isPrimitive 链），属类字面量段（155× 那一项），
  非 invoke 段；`__type_desc` 归零、member_key 的转换/分配份额随之消失。
- **分配器簇（第二热点）占比减半**（~15.3% → ~7.3%，少了每次调用约 5 次分配），只写
  数字；剩余需求方为 varargs 数组、返回装箱（valueOf 缓存外）与缓存哈希表节点。
- **键缓存的自身代价** ≈ 2.4%（usize 键 SipHash + 读锁）；如需再压，可用乘法散列的
  轻哈希器替换 RandomState（未做）。
  → **10-13 ikopt2 已实施**（f2232b29，§二.4）。
- **口径更正（§二.2 遗留的 10 倍差）**：`perf_hotpath.sh` 不带 `--release`（debug 档），
  `perf_rt_bench.sh` 带 `--release`——43f37f63（us1）的 43 µs/迭代与本报告 kr1 的
  4.3 µs/迭代是 **debug vs release 口径差**（sg1 上 debug 整例 42.9 s / release 分段
  合计 3.6 s，同构 10 倍），非 lineage 或机差，§七.0 该遗留闭合。
- 回归：rtpp-verify4 七例清单 + TestWidenFacesInvoke / TestWrapperValueFaces 共 9 例
  PASS（作业 ikopt-verify，kr2；PartitionInteger run 50.18 s 与基线 48–50 s 持平）；
  java_runtime 的编译验证由 9 例端到端发射构建承载（本机裸检出不可 cargo check，
  E0583 既有形态），另以 batch-1012 系 spot 检出的发射 crate 替换两文件后整 workspace
  cargo check 通过。作业索引：ikopt-base / ikopt-after（sg1，计时与剖析）、
  ikopt-verify（kr2，回归），结果在 `cluster_results/job/ikopt-*/`。

### 4. 遗留两小项实施（2026-10-13，分支 ikopt2，f2232b29）

§七.1 所记两个后续小项已实施（单 commit，`runtime/java_runtime` 两文件，发射侧零改动）：

1. **键缓存轻量哈希器**（`method_impl.rs`）：`METHOD_KEYS`（(类镜像身份, slot) → 分派键
   的进程级缓存）由缺省 `RandomState`+Sip13 换 **FxHash 风格乘法散列**（`FxKeyHasher`：
   每分量 rotate_left(5) ^ w 后乘固定奇常数 0x517cc1b727220a95，一次查键 = 两次乘法 +
   异或，无 SipHash 多轮压缩、无 RandomState 每实例密钥初始化）。**碰撞安全性**：键为
   (指针身份, slot) 内部值、非攻击者可控；HashMap 命中哈希后必做键相等核对，碰撞代价
   只是一次 12 字节核对、绝无错答；条目数 ≤ 被 invoke 的不同声明数（编译期有界，量级
   数百），64 位乘法散列下碰撞概率 n²/2⁶⁵ 可忽略——指针低位对齐零熵由 slot 分量的旋转
   异或补足（独立验证：2000 个 16 对齐指针形键零错答、零碰撞）。冷路径 `REBUILT_KEYS`
   （驻留池，现实为空）与 `CTOR_KEYS`（newInstance 路径，非热）维持 RandomState。
2. **`is_static_descriptor` 行内查找二分化**（`reflect_dispatch.rs`）：原 = 类行二分
   （`meta::row`）+ 类方法表行**线性 find**（逐方法两个 String 比较）。**行序不可重排**
   ——行序 = slot（Method / Constructor 对象构造点写入的 slot 即行号，`__slot_row` 同一
   真源），构建侧排序会改写全部 slot 语义，故**不动表、不动发射**：首次判定时从冻结表
   派生进程级有序索引（`STATIC_INDEX`，OnceLock），每类一组按 (name, descriptor) 有序的
   紧凑三元组 (name, descriptor, is_static)——分量是表内 `&'static` 引用 + bool 位，
   不复制字符串，一次构建常驻（O(总方法数 × log)，约 24 B/方法）。热路径 = 类行二分 +
   类内 (name, descriptor) 二分；**≤ 8 行的小类仍顺序扫**（个位数元素二分不敌顺序扫，
   且 24 B 三元组比约 130 B 的整行 `MethodMeta`——跨缓存行——便宜）。同答性经独立
   逻辑验证（命中 / 未命中 / 表外类 / 边界类名 / 重载面，小类线性与 40 行大类二分两路，
   新旧全同答）。

**验证与数字**：java_runtime 编译验证 = 作业 ik2-compile（jp1，`rava compile`
TestReflectLdcHotLoop 端到端，`build_status.ok=True`、1558 个 .rs，409 s）PASS；本机
generator `cargo check --release --tests` 通过（裸检出 java_runtime 不可 check 为既有
形态，§八）。同机 A/B（作业 ik2-base（基线 31e4762d）/ ik2-after（f2232b29），
RldcSegments + BenchReflectArg 口径各 5 次取中位）与回归（ik2-verify：rtpp-verify4
七例 + TestWidenFacesInvoke / TestWrapperValueFaces 共 9 例）见 §八作业索引。
量级预期（ikopt-after 剖析口径，sg1）：键缓存哈希簇 ≈ 2.4–2.8%（`hash_one` 1.29+0.33 +
`Sip13::write` 1.14）为可期主项；`is_static_descriptor` self 4.72% 中行内 find 对基准
类（2–5 方法行）只占少数——大头是类级 `meta::row` 对全表数千类名的字符串二分（每
invoke 两次），行内二分的受益面主要是方法数十行以上的声明类（经反射调用的 JDK 类），
对基准口径贡献有限。


## 三、对象 2：rd::unbox_* 解析 __obj_str 文本

### 1. 开销形态（基线）

翻译包装类实例（`Integer.valueOf` 产出的翻译 Integer，非原生值盒 `Rc<i32>`）在 `unbox_i32`
等入口的前四个 downcast 全部落空，落入 `v.0.__obj_str().parse()`：

1. `__obj_str` 桥接（`rava_macros_core/src/block/gen/struct_layout.rs:166`）调**翻译体
   `Integer.toString`**——按值逐位构建 Java String（UTF-16 缓冲分配）；
2. `ToString::to_string(&s)`——Java String → Rust String 的 **UTF-16→UTF-8 转换 + 分配**；
3. `str::parse::<i32>()` 文本解析。

即一次拆箱 = 一次完整的「值→文本→值」往返（≥3 次分配 + 数位格式化 + 解析）。

### 2. 方案评估

- **P-A「ObjectVTable 原生值视图臂」（结构化承载，终态方案）**：`ObjectVTable` 增
  `__prim(&self) -> Option<PrimVal>`（默认 None），rava_macros 对包装类桥接到其 `intValue()`
  等翻译体，`rd::unbox_*` 先问视图。优点：直取值、无契约耦合、覆盖 Long/Double；代价：
  vtable 增一臂（全体类付槽位税）、宏与包装类桥接属协议层改动、需与 S7 K 系（方法 trait /
  vtable 形态演进）协调。**建议随 S7 线排期，不单独做。**
- **P-B「hashCode 结构化取值」（本步落地的可回退原型，`reflect_dispatch.rs:298` 起）**：
  包装类均 final 且 hashCode 契约与值一一对应（Integer/Byte/Short 恒等、Character=码元、
  Boolean 1231/1237、Float=floatToIntBits），经 hashCode 虚分派一次取回值位；Long/Double
  折叠到 int 不可逆，仍走文本。改动集中在 unbox 六函数内，接收类名集合与原路径一致
  （不新增可拆箱类型），单 commit 可整体回退。
- **缓存方案（否决）**：对 `(对象身份 → 已解析值)` 做侧表缓存——侧表键为地址，地址复用
  需 pin 对象防假命中（泄漏随 distinct 包装实例无界增长）；包装类不可变但实例海量
  （循环装箱即海量），缓存收益不如 P-B 且引入生命周期风险，不取。

### 3. 量化（kr1，基线 7b0bf81a，作业 rtpp-prof1）

- **边界开销**：`idInt − idRef = 178 ns/调用`（含 `Integer.valueOf` 缓存外新分配与
  `box_value`，unbox 文本往返为其中可去除的主要部分）。
- **self 归属**：unbox 文本路径的可辨识片段 = `Integer.toString_i` 0.89% +
  `Integer.getChars` 1.05% + `AbstractStringBuilder` 1.23% + `String.__init_on_arr`
  1.23% + String `Display::fmt`/`units` 中 `__obj_str` 的 to_string 份额，
  合计约 4–5% 全样本（invoke 段占 82% ⇒ 折合每次调用约 80–100 ns）。

### 4. A/B 结果与 P-B 收窄（作业 rtpp-dbg2，57eda4ee 前一档 4b4ce416）

A/B 两档跨机（基线 kr1 / 修复档云池另一台，同规格 15G；RldcSegments ldc 段两机 31.3 vs
31.5 ms 基本一致、class 段 +9% 差异，解读时按 ~10% 机差折算）：

| 测点 | 基线（kr1） | 修复档 | 变化 |
|---|---:|---:|---|
| RldcSegments invoke 段（1M 迭代 ×2 调用） | 3529 ms | 3120 ms | **−11.6%**（扣 ~10% 机差 ≈ −20%） |
| BenchReflectArg idInt | 1910 ns | 1615 ns | −15% |
| BenchReflectArg idInt2 | 2465 ns | 1944 ns | −21% |
| TestReflectLdcHotLoop 整例墙钟 | 4.25–4.31 s | 3.66 s（rtpp-verify3 实跑） | −14% |

**P-B 收窄（实测否决与见证不足的部分）**：rtpp-verify3 回归 7 例中 TestReflectBoxCast
`narrow` 行 `1.5 → 0.0`——`half(3.0f)` 的 Float 实参经 hashCode 位形取值得到 0.0，即翻译
Float 的 hashCode 路径（→ `floatToIntBits` 链）对 3.0f 应答 0（根因未查，独立缺陷嫌疑，
已记 §七遗留）；Character / Boolean 的实参级路径无既有测例见证。最终形态（57eda4ee）：
**Integer / Short / Byte（恒等 hashCode，回归通过）走结构化取值；Character / Boolean /
Float / Long / Double 维持文本路径**。收窄后 A/B 的 int 路数字仍然成立（回归与 A/B 用例
只经 int 实参）。

**【10-12 fix-widen 更正与终局】**：上述 §三.4 的两个判断都需要更正——

1. **「Integer / Short / Byte 回归通过」是见证不足**：当时的回归与 A/B 用例对 Integer 走的是
   invoke 站点装箱的**原生盒**（`downcast_ref::<i32>` 先命中，未到 hashCode 臂），Short /
   Byte 的翻译包装实例则根本没有测例触达。batch-1012 新测例 TestWidenFacesInvoke
   （fdddd6b5，作业 b1012-widen5）实测：翻译 Short 经 hashCode 取值应答 **0**
   （`widen-local-short 1`，应 -299）、翻译 Character 经 toString 文本路径 IAE
   （`widen-char IAE`）——P-B 的 hashCode 臂与文本路径**双双落在存根上**。
2. **Float.hashCode 对 3.0f 应答 0 与本簇同根因，且不是 `floatToIntBits` 链缺陷**：根因是
   **运行时经 ObjectVTable 桥的 hashCode / toString 调用对闭包分析器不可见**——程序自身
   不触达这些槽时（`in_chain` = false，`input/src/build.rs::in_chain`），方法体按
   `stub.rs::stub_body` 发安全存根：hashCode → `Ok(0)`、toString → `Ok(二进制名文本)`。
   Float 的 0、Short 的 0、Character 的多字符文本（解析失败 → IAE）全是这两行存根的应答。
   生成代码铁证（作业 wfx-gen3，TestWidenFacesInvoke scratch 的
   `java_base_body_1/src/body/java/lang/`）：short.rs L48-50 的桥属性
   `#[hash_code_vtable = "Short"]` 俱在而 L191-193 槽体是 `Ok(0)` 存根，character.rs /
   float.rs 同形；integer.rs 为真体（该程序里 Integer.hashCode 在链上）。
   TestReflectBoxCast 当时 Float 行能走通，是 `+ fl` 字符串拼接让 Float.toString 有了翻译
   调用点（入链 → 真体）；直呼站点入链后翻译体本身正确（TestWrapperValueFaces 分层见证：
   直呼 / 擦除 Object 桥分派 / 反射拆箱三层一致）。
3. **终态修法 = P-C 承载字段直读**（fix-widen）：`unbox_*` 对翻译包装类改经实例字段协议
   （`__instance_field`，与 Field.get / Unsafe 按名单元同一数据面）直读本类 final 基本字段
   `value`——不依赖任何方法体是否在闭包链上，Long / Double 也按位精确（NaN / -0.0 位形
   保留），P-B 的无分配收益保留且扩展到全部八包装类；种类不符回落文本路径。§三.4 的
   P-B 收窄口径由 P-C 取代（`reflect_dispatch.rs` 文件头注记有完整演进记录）。
   验收（fix-widen，作业 wfx-acc1，748e0987）：TestWidenFacesInvoke / TestUnboxWrapperFaces /
   TestWrapperValueFaces 3/3 PASS，rtpp-verify4 七例清单（TestReflectLdcHotLoop /
   TestReflectBoxCast / TestReflectFieldMethod / TestInvokeNullArgs / TestIfaceAssignWalk /
   PartitionInteger / TestUnboxWrapperFaces）全 PASS——PartitionInteger run 48.19 s 与基线
   48–50 s 持平，TestReflectBoxCast 期望文件无需修正（narrow 行 1.5 原本正确）。

回归（rtpp-verify3，fc68236d 档，7 例）：PartitionInteger PASS（run 49.4 s，与基线
48–50 s 持平——微修复不伤 stream 路径）、TestIfaceAssignWalk（新）PASS、
TestInvokeNullArgs / TestReflectFieldMethod / TestReflectLdcHotLoop PASS、
TestReflectBoxCast FAIL（即上述 Float 行，已修）、TestUnboxWrapperFaces 因 expected
当时未入库被 SKIP（expected 已随 57eda4ee 入库，rtpp-verify4 复跑）。

### 5. 修复后剖析（rtpp-prof2，kr1，fc68236d 档，F997 自采样 top）

unbox 文本簇整体退出 top15（`Integer.getChars` / `toString_i` / `AbstractStringBuilder` /
`String.__init_on_arr` 由 0.84–1.23% 各项归零）；新的 top = `invoke0`（含 member_key
内联）7.67%、String `Display::fmt` 7.10%、malloc 6.12% + cfree 4.51%、`String::units`
3.83%、`is_static_descriptor` 2.61%、`native_invoke` 1.50%、varargs `Store::to_vec`
1.71%——**member_key 分派键重建成为明确的下一目标**（与 §七.1 一致）。

## 四、对象 3：PartitionInteger release 48 s（JVM 0.65–0.70 s）

### 1. 同机对照（kr1，基线 7b0bf81a，作业 rtpp-prof1）

- 整例墙钟（3 次）：rava 48.34 / 48.50 / 50.08 s，JVM 0.65 / 0.68 / 0.70 s
  （**约 72 倍**；无 GC 文档 §六在 812b763f 记 46.2–47.0 s，batch-1011k 增量 +2~3 s
  属 review-soundness 等并入后的机况差异，结构未变）。
- 受控对照（BenchStreamLoop，24 元素 `Arrays.stream(combo).map(i -> primes[i]).sum()`，
  每次调用 ns，中位）：rava **stream 6435（268 ns/元素）vs 手写循环 193（8 ns/元素）**；
  JVM stream 39（约 1 ns/元素）vs 循环 8。即 stream 管线在 rava 侧每元素附加
  ~260 ns、是同二进制手写循环的 33 倍、是 JVM stream 的 268 倍——慢不在数组遍历本身，
  在管线协议。

### 2. children 分解（F300 dwarf，占比有重叠，看结构）

| 聚簇 | children 占比 | 构成（符号 → 语义） |
|---|---|---|
| 终端求值 `IntStream.sum` | 71.7% | `copyInto` 33.8 + `Spliterator.forEachRemaining` 22.4（接口入口；`IntConsumer.accept` 17.6）+ `Sink.accept_i` 9.25 + `ReducingSink.accept_i` 7.42 + `IntBinaryOperator.applyAsInt` 2.33 |
| 管线构建（每叶子一次） | ~25–30% | `Head` 构造 10.2 + `StatelessOp` 构造 8.28 + `AbstractPipeline` 构造 7.91+2.42 + spliterator 构造 2.97+3.52 + `makeInt` 3.43 + `ReduceOps_6::new` 2.60 + `StreamSupport.intStream` 3.40 |
| 接口分派基础设施 | ~8% | `__IfaceRef::enter` 3.22（`handle.rs:187`，含每次接口调用的 `__stack_check`，self 1.10）+ `Handle::view` 2.18 + `try_cast::<Sink>` 2.60 |
| 区域整组释放 | ~6% | `___Region::exit` 4.65（`region.rs`，日志 Vec 追加 + 撤销）+ `on_alloc` 胶水 1.12 |
| 装箱 | <3% | `Integer.valueOf_i` 2.19 children / `__init_on_i` 0.47 self（仅静态初始化与个别角落） |
| 分配 / 释放 | ~3% | malloc 2.24 self + cfree 0.50 |

self 视图呈弥散形态：最大单点 `ReducingSink.accept_i` 3.08%（内层真实工作），其余
0.4–3% 碎片分布在数十个翻译体方法与 vtable 胶水（`IntPipeline`/`AbstractPipeline` 各
构造、`estimateSize` 1.42、`__IfaceRef::enter` 三形态 1.10+0.80+0.71 …）——协议税
无单点可修。

### 3. 结论（接口分派 / 装箱 / 迭代器协议之辨）

- **大头是迭代器协议 + 接口分派**：每个叶子整条管线重建（~10 个堆对象与 vtable 构造，
  占 25–30%），每元素走 `forEachRemaining → IntConsumer.accept →（map 的 ChainedInt
  accept）→ ReducingSink.accept → IntBinaryOperator.applyAsInt` 约 5 重间接分派，
  其中 2 重经 `__IfaceRef::enter`（视图查询 + 栈界检查，`handle.rs:187-204`）。
- **装箱不是大头**（<3%）：IntStream 全程基本类型，`Integer.valueOf` 只在静态初始化
  的流构造等角落出现。
- JVM 0.65 s 的构成是 JIT 的标量替换与内联把管线构建与逐元素分派全部消解
  （BenchStreamLoop 里 JVM stream ≈ 1 ns/元素、与手写循环 0.33 ns/元素同量级）；
  rava 无 JIT，等价收益要靠生成器侧（按调用点单态化 / 内联）或 PGO，非运行时单点可修。

## 五、已有符号级基线（nogc-perf3，kr1，812b763f）

`perf record -F 300 --call-graph dwarf`，children 视图（占比有重叠，仅作结构参考）：

| 聚簇 | 占比 | 构成 |
|---|---|---|
| `IntStream.sum` 终端求值 | 68.8% | copyInto 27.7% + forEachRemaining 17.8% + accept 链（IntConsumer.accept 13.6% / Sink.accept_i 6.8% / ReducingSink.accept_i 6.1% 自耗） |
| 管线构建（每叶子） | ~25–30% | AbstractPipeline 构造 8.4% + wrapSink 8.4% + spliterator 构造 ~6% + makeInt 4.3% + source 2.2% |
| 对象释放 | ~5% | Object drop_glue 4.4%（自耗第一）+ malloc 1.5% |
| 装箱 | <3% | Integer::from 0.76% 等（IntStream 全程基本类型，无装箱热点） |

## 六、微修复（9157a73e → fc68236d → 57eda4ee）

1. `Class.__name_assignable`（`runtime/java_runtime/src/java/lang/class_impl.rs:324`）：
   层次表与直接超接口表由线性 `iter().find` 改 `meta::row` 二分（表按名有序，同取首个同名行，
   与 meta::row 既有口径一致）。调用面：反射实参校验（`reflect_dispatch.rs:199`）、数组
   checkcast 视图（`array.rs:366`）、`isAssignableFrom`、代理与 Array.set 分量检查。
   （fc68236d 修解构：`meta::row` 返回 `Option<&V>` 仅值，rtpp-verify2 曾因此发射构建
   E0308 两处——java_runtime 裸检出不可 cargo check，此类错误只有发射构建能拦截。）
2. `rd::unbox_i32`（`reflect_dispatch.rs:298` 起）：翻译 Integer / Short / Byte 实例取值从
   `__obj_str().parse()` 文本往返改为 hashCode 结构化取值（P-B 原型收窄后的最终形态，
   见 §三.4）；Character / Boolean / Float / Long / Double 维持文本路径。

回归测例（expected 由服务器 JVM 生成并入库）：`tests/e2e/62_reflection/
TestIfaceAssignWalk.java`（两条查表路径 + 反射实参校验 + 拒收面）、`TestUnboxWrapperFaces.java`
（全值域 + 拓宽 + Float 位形 + 拒收面）。既有测例覆盖：TestReflectBoxCast / TestInvokeNullArgs /
TestReflectFieldMethod / TestReflectLdcHotLoop / PartitionInteger。

## 七、下一步建议（含本步遗留）

0. **遗留（本步发现，未收口）**：翻译 `Float.hashCode` 链（疑 `floatToIntBits`）对 3.0f
   应答 0——P-B 的 Float 位形取值因此被否决；这本身是一个可疑的翻译面缺陷（或 hashCode
   桥接缺口），值得单独立测例核查（`Float.valueOf(3.0f).hashCode()` 期望 1077936128）。
   另外 43f37f63（us1）43 µs/迭代与本测 4.3 µs/迭代差约 10 倍未闭合（§二）。
   → **10-12 已闭合**：debug（perf_hotpath 口径）vs release（perf_rt_bench 口径）之差，
   非 lineage / 机差，见 §二.3 口径更正块。
   → **10-12 fix-widen 已收口**：同簇根因 = 闭包外根虚方法槽为存根（hashCode → Ok(0)、
   toString → 二进制名），非 floatToIntBits 缺陷；`Float.valueOf(3.0f).hashCode()` 直呼
   （入链）= 1077936128 正确，见 TestWrapperValueFaces 与 §三.4 更正块。unbox 取值改
   P-C 承载字段直读后不再依赖 hashCode / toString 槽。**残余暴露面**（本步未修，单列）：
   翻译类（不限包装类）的根虚方法槽在「类在闭包、方法不在链」时一律为存根——经擦除
   Object 接收者的虚分派（`invoke/virtual_/bare.rs` 根 vtable 单次直调路径）落到它上面：
   hashCode 恒 0（Map 桶退化）、equals 存根 panic（stub_call）、toString 恒二进制名；
   根治面 = 闭包种子（seeds.toml 增「值视图」族：包装类成员触达时补种根虚方法）或
   P-A 协议臂，随反射建模 / S7 K 系排期。

1. **Method.invoke 分派键重建（最大单项，String 簇 ~17% + 其分配 ~16% 的一部分）**：
   → **10-12 已实施**（分支 invoke-key-opt，c0713ed8）：表构造成员对象的 slot 即方法表
   行号，(clazz 身份, slot) 首用经表行直取 `&'static` 键 + 进程级缓存，非表形态回落
   原重建（驻留）。同机 A/B：invoke 段 1.515 → 0.646 µs/调用（−57.4%）、idRef
   1636 → 587 ns、分配器簇占比 ~15.3% → ~7.3%；键缓存自身哈希代价 ≈ 2.4%。详见
   §二.3。当时的两条候选路线（身份侧表 / 发射层预计算）均不再需要。
   - 后续可再压的小项（未做）：缓存哈希器换轻量乘法散列；`is_static_descriptor` 的
     类方法表内线性 find（描述符配对）换 (name, descriptor) 二分索引。
     → **10-13 已实施**（ikopt2，f2232b29，§二.4）：键缓存换 FxHash 风格乘法散列 +
     行内查找改进程级有序索引（行序 = slot 冻结不可重排，索引自冻结表派生、不动发射）。
2. **unbox 结构化承载终态（P-A）**：随 S7 K 系 vtable / 方法 trait 形态演进一并设计
   （`ObjectVTable` 原生值视图臂），Long/Double 也能直取值；P-B 已收割无协议改动的部分。
3. **PartitionInteger / stream 管线**：72 倍的主干是「每叶子管线构建（25–30%）+ 每元素
   约 5 重接口分派（含 2 次 `__IfaceRef::enter` 的视图查询 + 栈界检查，`handle.rs:187`）」，
   运行时侧无单点可修（self 弥散在数十个翻译体方法）。可行路线：
   - S7 线分派视图约 19%（jackson jc1 已归 S7）同源；
   - 生成器侧按调用点对 stream 管线单态化 / 内联（等价 JVM JIT 的标量替换，BenchStreamLoop
     实测 JVM 把管线消解到 1 ns/元素）；
   - 小项：`__IfaceRef::enter` 的 `__stack_check` 与所在方法体的入口检查合并（发射层）；
     `___Region::exit` 4.65% 里日志 Vec 追加的可合并性（区域线）。
4. **类字面量段 155×**（496 ns/迭代：镜像比较 + isInstance + 周期性 getName）单列观察，
   未深剖；量级小于 invoke 段。
5. String 跨 crate 不内联（jackson jc1 约 4%）：属 S7 线口径，本步未复测。

## 八、验证与检查状态

- 本机 cargo check：generator `--release --tests` 通过；runtime 子 crate
  （rava_macros / rava_macros_core / rava_coro）`--release --tests` 通过。
  `java_runtime` 单独不可 cargo check（`src/java` 等目录的 mod.rs 与伴生类模块由发射层
  在 scratch 内生成，裸检出 E0583 为既有形态，pristine 5328072a 同败）——其编译验证经
  rtpp 作业的端到端构建承载（rtpp-dbg1 曾借此把 E0308 定位到行）。
- 作业索引：rtpp-prof1（基线剖析 + 对照基准 + 整例墙钟，kr1，7b0bf81a）、rtpp-verify2
  （首验，E0308 暴露，sg2）、rtpp-dbg1（E0308 定位，sg2）、rtpp-verify3（7 例回归，
  fc68236d）、rtpp-dbg2（失败例差分 + A/B 基准，4b4ce416）、rtpp-verify4（收窄后 7 例
  复跑，57eda4ee）、rtpp-prof2（修复后剖析，视排队情况）。结果均在
  `cluster_results/job/rtpp-*/`。
- **ikopt2 线（§二.4）**：ik2-compile（f2232b29 编译验证，jp1，PASS 409 s）、
  ik2-base（A/B 基线 31e4762d）/ ik2-after（f2232b29）（RldcSegments +
  BenchReflectArg ×5 + perf_line_profile）、ik2-verify（9 例回归：rtpp-verify4 七例 +
  TestWidenFacesInvoke / TestWrapperValueFaces）。10-13 派发时段七台云服务器全被并行
  分发器占满（dev 批闸门 + JDK25 扫尾 + 多子代理），作业按 30 s 退让循环轮候、租约多次
  被打断重启（远端脱钩 + journal 断点续跑不丢已完成段）；结果落
  `cluster_results/job/ik2-*/`，同 tag 原命令重跑即续接。

### §二.4 后记（2026-10-13，回退）

ik2 本地同机 A/B（TestReflectLdcHotLoop 整例 release）：base 1.07 s vs after 1.06 s——合计收益 <1%（判据 ~3%），按任务约束回退 f2232b29 两项改动。归因：键缓存哈希簇 ≈2.4% 为 invoke 段口径，折算整例 ~1.4% 落入噪声；is_static_descriptor 的 self 4.72% 大头是类级 meta::row 对数千类名的字符串二分（每 invoke 两次），行内线性 find 二分化未触及该大头。若再攻此面，方向应是 meta::row 类名查找的结构化（如类名→行号侧索引），非行内扫描。九例回归两态均全过（回退无风险）。
