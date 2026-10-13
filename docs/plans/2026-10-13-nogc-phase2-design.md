# 无 GC 二期设计：所有权推断 Weak、JDK 方法体内区域、引用类语义

> 日期：2026-10-13（设计轮：产出可立项的实施设计，不做大实施）
> 关系：[no-gc-memory-model](2026-10-07-no-gc-memory-model.md) 第一节第 2 条第二项（所有权推断）、第 4 条（引用类语义）、§4.5（区域只插用户方法的开放世界后续）与 §六-5「下一步」第 2–5 项的展开。约束沿用该文档第三节 1–8 条，本文不再复述，只引用编号（如「约束 1」）。
> 范围：tasks.md「剩余工作」第 7 条四个子项。本轮唯一代码改动是判据探针 `tests/perf/NogcWeakCycle.java` 与 `thread_local_lint` 文档头登记（§四），其余全部为设计。

## 〇、总览与依赖关系

三个机制子项共享**一个运行时原语：`__Obj` 的弱引用能力**（弱计数 + upgrade/downgrade）。

```
            ┌────────────────────────┐
            │ P0 弱引用原语（obj_ref） │  ← 本文档 §一.3
            └──────┬─────────┬───────┘
                   │         │
     ┌─────────────▼──┐   ┌──▼──────────────────┐
     │ 一、所有权推断  │   │ 三、引用类语义       │
     │ Weak（回指字段）│   │ （referent 观察弱）  │
     └──────┬─────────┘   └──┬──────────────────┘
            │                │
     ┌──────▼────────────────▼───────┐
     │ 二、JDK 方法体内区域（开放世界）│  ← 依赖一的发射形态与三无关，
     └───────────────────────────────┘    依赖闭包引擎 user_extensible 口径
```

- P0 是一、三的公共前置；一与三可并行（都只依赖 P0）。
- 二与一、三正交，但其首批事实清单里有一类「配合 Weak 线的 JDK 侧事实」（§二.5）。
- 四（thread_local_lint 登记）是顺手小事，本轮已完成（§四）。

建议实施序：**P0 → 三（引用类，语义面窄、验收面清晰）→ 一（弱化推断 v1）→ 二（开放世界区域）**。理由：三先把「观察弱」的运行期路径（登记表、僵尸、摘表纪律）在最受控的场域（Reference 一个字段）跑实，一再把弱引用扩展到推断字段；二独立可随时插队。

---

## 一、所有权推断 Weak：逃逸到旧对象但成环

### 1.1 现状（问题存在方式）

- 对象模型**无弱引用**：`__Obj` 头只有强计数与位布局字（`obj_ref.rs` 文档头明言「无弱引用：对象模型不用 Weak」）。全部引用字段是 `__RefField<T>` 强存储。
- 回收手段只有两个：引用计数（无环时）与区域整组释放（不逃逸且成环时）。
- 「逃逸到旧对象但成环」两头都够不着：对象进入旧对象（putfield 到 O / putstatic）区域即判否（`escape/src/analysis.rs::store`「区域对象存入旧对象」），而环使引用计数失效 → **按第一节第 3 条登记为已知限制，泄漏**。这是有意的设计决定，不是缺陷；本子项的目标是把这个「已知限制」的可解部分解掉。
- **实测证实**（探针 `tests/perf/NogcWeakCycle.java`，作业 nogcd-probe1，数字见 §五）：双向链（`next` 前向强 + `prev` 回指强）经静态锚逃逸到旧对象后摘锚，cyclic 模式 RSS 随迭代线性增长（≈110 B/节点全泄漏，400k×8 达 357 MB），acyclic 对照（`prev` 恒 null）恒定 15.1 MB——问题存在方式与设计假设一致：泄漏归因于回指成环，而非逃逸本身。

### 1.2 证明义务（判定的核心不变量）

弱化一个字段不是「把它变弱」这么简单——**弱化的读语义必须与强字段不可区分**，否则悬空可观察（读出 null）。约束 1「宁可泄漏，不可悬空」的形式化：

> **INV(f)**：任意执行时刻，对每对满足「h 强可达 ∧ h.f 上次写入值为 d」的 (h, d)，d 经**不经过 h.f 的强路径**强可达。

- INV 成立 ⇒ 弱化后 `h.f` 的读值与强字段恒同（upgrade 永不失败），行为零差异。
- INV 破坏 ⇒ d 提前死，`h.f` 读出 null——悬空，禁止。
- 等价读法：**f 不是任何目标的唯一持有者**（约束 1 的原文即此）。INV 是「不是唯一持有者」的时序强化（不是某一刻不唯一，而是 h 持有期间一直不唯一）。

INV 是程序不变量，只能整程序证明（写点建立覆盖、摘点保持覆盖、无开放写入）。这是设计难点所在，路线按证明方式分。

### 1.3 运行时原语 P0：弱引用的两条路线（含弃选）

所有弱化消费方（一、三）都要求 `__Obj` 支持：弱句柄持有值地址不增强计数；`upgrade() = 原子 CAS + 失败分支`（约束 2 的性能口径）。

**路线 R1（推荐）：内联弱计数，头 16 → 32 字节。**

```rust
pub(crate) struct Header {
    strong: AtomicUsize,   // 同今
    word: AtomicUsize,     // 位布局同今（第三节第 8 条），第 1 位启用为 WEAK_DEAD（见下）
    weak: AtomicUsize,     // 新增：弱句柄计数（含僵尸期）
}
```

- 头对齐 16 不变，`size_of::<Header>()` 24 → 32：**每对象 +16 B**。
- `upgrade`：`strong` 先读，`0` → None；否则 CAS(n, n+1)，失败重读（同 `Arc::upgrade`）。
- `downgrade`：`weak.fetch_add(1)`。
- 强释放归零（`drop_slow`）：摘侧表照旧 → 若 `weak > 0`：`drop_in_place` 值、**不归还分配**（僵尸：strong=0、值已析构、分配保活）；弱释放 `weak` 归零且 strong=0 → 归还分配。
- **区域整组释放互动**（`obj_ref.rs::group::free`）：组内对象可能被**组外弱句柄**指着（弱引用不构成强逃逸，`store` 判否不拦它）：
  - 步骤 ① 对每个对象置 `word` 的 `WEAK_DEAD` 位（第 1 位，现保留位）；`upgrade` 见该位即 None——挡住整组释放中途的 CAS 复活（BIG 哨兵下 strong ≠ 0，裸 CAS 会成功并拿到析构中的值）。
  - 步骤 ② 逐个析构、debug 断言「计数恰为 BIG+1」**不变**（组外弱句柄不增强计数）。
  - 步骤 ③ `weak == 0` 照旧归还；`weak > 0` 转僵尸（strong 置 0、跳过归还，弱释放收尾）。
- 映像对象快路径：弱句柄 upgrade 前判 `in_image` → 恒成功、不增减 weak（永生对象无僵尸期）。
- `MAX_REFCOUNT`、`IMMORTAL`、计数漂移结论全部不变；`HEAP_DEFINED_FLAGS` 增 `WEAK_DEAD`（第三节第 8 条的常量组照常集中定义）。

**路线 R2（备选）：零布局，身份侧表弱单元。** 第三张 `IdentityTable`（identity → `{weak, dead}`）；upgrade = 分片锁 + 存活判定；强归零若表有项则不归还。代价：upgrade 是哈希 + 互斥，**违反约束 2 的「原子 CAS + 失败分支」口径**（需先修订该约束），且每次读弱字段都过全局表——高争用。仅当 +16 B/对象在内存敏感语料（已知 14G 槽 OOM 族、DeepCopy 11.9G 上限例）上被证不可接受时启用。

**路线 R3（弃选）：弱计数嵌 strong 高位（48..62 位，14 位）。** 零布局且 upgrade 仍 CAS，但 14 位上限会被大 `WeakReference[]` 溢出（子项三的合法构造），且 inc/dec 最热路径引入掩码位操作——与「位布局集中定义」的纪律相悖，弃。

| | R1 内联弱计数 | R2 侧表 | R3 高位嵌位 |
|---|---|---|---|
| 每对象空间 | +16 B | 0 | 0 |
| upgrade 成本 | CAS（合约束 2） | 哈希+分片锁 | CAS |
| 弱计数上限 | isize 级 | 无 | 16383（可溢出） |
| 热路径改动 | 无（inc/dec 不动） | 无 | inc/dec 加掩码 |
| 实现面 | obj_ref.rs 集中 | 新侧表+摘表纪律 | 全计数路径 |

R1 的空间代价判据（实施 P0 时必测）：PartitionInteger 峰值 RSS 57 MB 基线复测（对象计数不随本改动变，预期 +16 B × 存活对象 ≈ 个位数 MB）；DeepCopy / junit OOM 族抽查不恶化。

### 1.4 推断的路线对比（生成器侧）

**路线 A（终态）：整程序不变量证明。** 以闭包引擎的按对象字段事实（`ovals`/`owild`，`engine/obj_fields.rs`）与流图为底座，对候选字段做全局不动点验证 INV：写点建立覆盖（同窗配对）、摘点保持覆盖（摘除覆盖的写点同窗重写 h.f 或 h 同时死亡）、无开放写入。精度最高、工程最大；INV 的静态反例路径与约束 6 的 `cycle_finder` 动态诊断互为印证。**不作为 v1**。

**路线 B（推荐 v1）：认证惯用法（链接窗配对）+ 封闭写集。** 只对满足以下全部条件的字段弱化（规则 W1，先一条）：

- **W1-a（ctor 回指形态）**：闭包内 `C.f` 的全部 `putfield` 都在 `C.<init>` 内、写入值恰为某形参 `p_k`；`f` 非 volatile、非 final（final 字段语义上持有所有权，不弱化）。
- **W1-b（链接窗配对）**：每个（抽象）`new C` 站点的链接窗（站点所在直线段 + `<init>` 摘要）内，`p_k` 的实参 d 满足「h 被写入 d 的某个非弱字段」（`d.g = h`：前驱的 `next`、父的 `children`、被观察者的监听器表）——即 **d 强持有 h 与 h 弱回指 d 在同一窗内成对建立**。
- **W1-c（覆盖保持，封闭写集切片）**：d 上被 W1-b 写入的覆盖字段 g 的全部写点满足其一：① 同窗重写 h.f；② 写点处 h 的全部强引用同时摘除（流图上 h 不可达，链整体死亡）；③ 写点维持覆盖（写入的新值仍是 d 的强持有者的持有边——链表 `unlink` 移动 `last` 锚属此类）。`f` 与 `g` 都无开放写入（反射 / Unsafe / VarHandle 按偏移可达即全部作废——现有 `fopen`/`hw_offset` 机制直接给出判定）。
- W1 的三个命中面（验证语料）：用户双向链表 `prev`（§五探针）、孩子 → 父回指（`children` 列表 + `parent` 字段）、观察者回指（源对象的监听器表 + 监听器的源字段）。

**路线 C（JDK 侧）：清单声明。** JDK 类字段（`LinkedList$Node.prev`、`AbstractMap` 视图回指等）不在用户封闭世界内（其全部写点在档案共享体里，用户程序的字节码分析看不见档案外的调用面）——经 `vm_intrinsics.toml` 新节 `[weak_fields]` 声明，语义由 JDK 侧担保（JDK 自身代码维护 INV；用户对 JDK 字段的反射写在闭包里可见，命中即整类不弱化——发射层按「该字段开放 ⇒ 保持强」守门）。与 `[escape]` 同一机制哲学：清单即边界，非类名特判进运行时。

**推荐组合**：C + B(W1) 先行（各自可独立验收），A 立项为终态路线（`cycle_finder` 落地后用动态诊断喂静态规则）。证明不了的字段保持强引用并泄漏（约束 1），由 `cycle_finder`（约束 6，独立子项）报告。

### 1.5 发射形态

弱化是**存储与访问器体的变化，签名与 vtable 面零变化**（与 volatile 分流同一先例，`rava_macros_core` 按字段属性分流）：

```rust
// 用户类 Node 的 prev（推断弱化，属性 java_field(weak = true) 由生成器写出）：
// 存储：__RefField<Option<__WeakOf<Node>>>（弱句柄包装：Clone=downgrade、Default=null、PartialEq=身份）
// 访问器：fn __get_prev(&self) -> Node { self.prev.get_or_default() /* 内部 upgrade，失败=null 句柄 */ }
//         fn __set_prev(&self, v: Node) { self.prev.set(Some(__weak(v))) }
```

- 擦除字段（`Object` 存储）弱化同形态（弱句柄以 Object 面进出）。
- `Rc` 环断开后链式释放可能超深——已有释放队列（`handle.rs::__release` 计深 + 待释放队列）天然覆盖，不新增机制。
- 弱读的 `upgrade` 计数进性能预算（约束 2）：推断结果附带读次数（`--escape-trace` 同款诊断出口），热点字段弱化在对标三测点过目。

### 1.6 判据

1. **探针转绿**：`tests/perf/NogcWeakCycle.java` cyclic 模式 RSS 平稳（现状基线：400k×8 节点 357 MB、≈110 B/节点线性增长，acyclic 15.1 MB 恒定，§五；目标：cyclic 400k 落到 ≈15 MB 同档），checksum 逐字不变；acyclic 模式不回归。
2. **无悬空**：弱化字段读值与 JDK 逐字一致——探针扩展「保持锚存活时遍历 prev 链」的读路径（INV 保证 upgrade 恒成功、读值非 null 且与 JDK 同）；W1-c 的每个豁免面配反例测试（反射写覆盖字段 → 该类不弱化，保持强，泄漏但正确）。
3. **回归面**：PartitionInteger（区域路径不受影响：弱化字段写不构成强逃逸，判定语义不变）、DeepCopy / TestSerial 族（无弱化命中时生成物逐字节不变——弱化零命中程序的成本只有 R1 的 +16 B/对象）。
4. **诊断**：`cycle_finder`（若同批实施）在探针上报告环上类与字段；`--escape-trace` 输出弱化决策与读次数。

### 1.7 实施分解（供派实施代理）

| 步 | 内容 | 主要文件 | 规模 |
|---|---|---|---|
| P0-a | Header 增 weak 计数、upgrade/downgrade、僵尸路径、单测（含并发 upgrade 竞态、区域互动的 Rust 级测例） | `runtime/java_runtime/src/obj_ref.rs` | 中 |
| P0-b | `group::free` 的 WEAK_DEAD 位与僵尸转交；debug 断言口径核 | 同上 `mod group` | 小 |
| P0-c | 性能/空间基线复测（PartitionInteger RSS、二进制大小）与 R1/R2 决策点 | 作业 | 小 |
| W-a | `[weak_fields]` 清单节（解析 + 档案侧 JDK 字段弱化发射） | `closure/src/manifest/sections.rs`、`rava_macros_core/block/…` | 中 |
| W-b | 用户类推断 v1（W1-a/b/c 规则 + 链接窗分析，复用 escape 引擎的窗分析骨架） | 新 `generator/crates/escape/src/weak.rs`（或独立 crate） | 大 |
| W-c | 弱读计数诊断出口 + 判据测试族（探针扩展 + 反例） | `tests/perf/NogcWeakCycle.java`、e2e 新例 | 小 |

---

## 二、JDK 方法体内区域：开放世界事实的建模口径

### 2.1 现状

- 区域只插用户方法：`emit/src/method_bodies.rs:188` 的门 `ctx.is_user(declaring_class)`；§4.5 设计明言「JDK 方法体是档案共享的，要用开放世界，留作后续」。
- 分析的五处封闭世界依赖（`escape/src/world.rs` + `analysis.rs`）：
  1. `World::cha`：对旧接收者按 `input.instantiated`（**本程序**实例化集）做 CHA；
  2. `World::lambdas`：扫描 `input.visited`（**本程序**调用链）的 lambda 站点；
  3. `callee` 的 `in_chain` 判定（**本程序**是否解析；否则 `Stub`=无效果）；
  4. `handwritten` / `intrinsic_members`（全局清单，程序无关，已开放世界安全）；
  5. `[escape]` 效果清单（全局，程序无关，同上）。
- 档案共享机制：档案 crate 包版本 = 源码树摘要，同内容共享编译产物（`emit/src/ctx.rs:35`）。**判定若依赖程序事实，体文本随程序变，共享即失效**——这是必须开放世界的根因，也是本子项的第一约束。

### 2.2 建模口径：开放世界 = 五个查询的保守重解读 + 清单事实

**定义（开放判定）**：一个档案体的区域判定是**开放有效**的，当且仅当它对「以该体可达的任何合法消费程序」（任意实例化集、任意用户子类、任意用户 lambda）都成立。实现为分析新模式 `Analysis::open`：

1. **旧接收者虚分派**（替换查询 1）：调用属主**用户可扩展** → 拒绝区域（用户子类可在某程序里传入旧接收者）。可扩展口径**移植闭包引擎 `engine/open_world.rs::user_extensible`**（接口；非 final 且有 public/protected 构造器；sealed 任一许可可扩展；lib 同名包；找不到类文件保守可扩展）——同一项目内两处口径必须同源（建议抽成 `input` crate 公共函数，闭包引擎与 escape 都调它）。属主封闭（final / 包私有 JDK 类）→ 对**档案注册表全部类**做 CHA（不再用 per-program instantiated）。
2. **lambda 站点**（替换查询 2）：只取**体自身**的 invokedynamic 站点（档案体固定）；程序侧 lambda 只能经旧值流入 → SAM 属主用户可扩展即拒绝（同 1 的口径）；封闭接口可对档案内实现类解析。
3. **存根语义**（替换查询 3）：`in_chain` 为假的档案方法在某个消费程序里可能在链上 → `Stub` 按 `Code` 分析（保守、更贵）；预算控制沿用 `STEP_BUDGET` / `MAX_PASSES`。
4. 手写 / 内建 / 清单效果（查询 4、5）：不变——`[escape]` 事实本身程序无关，开放模式直接复用；这是「事实取自 JDK 侧」的载体。
5. **发射门**：`plan_regions` 的调用点扩为「用户方法（现行封闭模式）或档案 JDK 方法（`Analysis::open`）」；`World` 增 `open: bool` 透传。

**何时入区域 / 何时保守逃逸（口径总纲）**：档案体内分配的对象入区域的条件与 §4.5 完全相同（不进旧对象、不被不透明代码触及、成环），只是「旧对象」的效果面按 1–3 的保守重解读放大；任何放大到「用户可扩展」的接触点一律整区域判否（保守逃逸）。**宁可少插，不可错插**——与约束 1 同一纪律。

### 2.3 与 vm_intrinsics.toml 清单机制的关系

- `[escape]` 四效果（pure / returns_receiver / fresh / array_copy）已是**程序无关事实**，开放模式原样消费；新增效果种类（如 `copy_of` = fresh + array_copy 复合、`returns_arg_k`）按需加，语义文档随清单维护。
- 新节 `[escape.open_world]`（或复用 `[escape]` 加键）：登记「开放模式下已知无效果/已知逃逸」的 JDK 方法级事实，替代部分保守拒绝；每条事实**必须配反例测试**（见 2.5）。
- 清单事实与 JDK 版本绑定（清单本身按参考 JDK 分发，已有机制），档案判定随 JDK 版本重建。

### 2.4 首批事实清单草案（survey 后增删，先立骨架）

第一批以「让候选 JDK 体的判定能跑起来」为目标，三类：

1. **纯查询 native / 手写内建**（消除高频不透明点；survey 先核承载形态）：
   - `java/lang/String.length/isEmpty/charAt`（手写承载 `string_impl.rs`，现为 Opaque；长度 / 字符读取不存实参 → pure）
   - `java/lang/StrictMath.*` 基本类型纯函数（手写承载 `strict_math_impl.rs` → pure）
   - `java/lang/Math.*`（字节码委托 StrictMath：委托体可分析，通常无需登记——survey 确认）
2. **新建类**（fresh 族扩展）：`java/util/Arrays.copyOf*`（建议新效果 `copy_of`：目标新建、元素自源复制，现四效果表达不了「fresh + 内容自实参」的复合）、`java/lang/String.valueOf` 基本类型重载（fresh String；手写承载面 survey 后定）。
3. **配合 Weak 线的 JDK 侧结构事实**（§一.4 路线 C 的先导调查）：`java/util/LinkedList$Node.prev/next`、`java/util/stream/AbstractPipeline.sourceStage/previousStage.nextStage` 的全部写点是否封闭在档案内（决定 `[weak_fields]` 可声明面）。

### 2.5 验证口径（判据）

1. **判定开放有效（确定性）**：同一 JDK 方法在 ≥3 个不同程序下发射的体文本逐字节一致（含 `java_scope!` 与否）；`--escape-trace` 的判定报告随程序不变。
2. **保守性反例**：构造「把候选体对象存入静态/旧容器」的负向程序（应判否），确认无区域插入；用户子类化封闭属主外的类传入旧接收者（应判否）。
3. **内存见证**：一个热点完全在 JDK 体内循环的测点（候选：`String.repeat` / `Arrays.sort` 临时图 / `String.format` 内部）RSS 平稳。
4. **断言零违例**：§4.5 的 debug 档区域逃逸断言在抽查集上保持零 panic（开放判定错插会立刻被抓——断言即守门员）。
5. **共享不失效**：档案 crate 摘要在无 JDK 变更时跨程序稳定（编译缓存命中率不降——`build_status.json` 对比）。

### 2.6 实施分解

| 步 | 内容 | 主要文件 | 规模 |
|---|---|---|---|
| O-a | `user_extensible` 口径抽公共（闭包引擎改调公共函数，行为零变化） | `closure/src/engine/open_world.rs` → `input` crate | 小 |
| O-b | `Analysis::open` 模式（五查询保守重解读）+ `plan_regions` 发射门扩展 | `escape/src/{world,analysis,plan}.rs`、`emit/src/method_bodies.rs` | 中大 |
| O-c | survey 作业：候选 JDK 体 `--escape-trace` 判定报告 → 首批清单草案定稿 | 作业 + `vm_intrinsics.toml` | 中 |
| O-d | 判据测试族（确定性 / 负向 / 内存见证 / 断言守门） | e2e 新例 | 中 |

---

## 三、引用类语义：随 Rc 释放触发

### 3.1 现状

- `Reference.referent` 是**强字段**：从不被清除、从不入队（`reference_impl.rs` 文档头）。`clear0` 是唯一置空途径；`hasReferencePendingList` 恒 false；`waitForReferencePendingList` 永久 park（Reference Handler 线程常驻等待，不阻止退出）。
- `ReferenceQueue.poll` 的「折叠」不是显式开关：闭包求值里 `ReferenceQueue.head` 恒 null（无入队路径）→ `poll0` 的 `head` 常量传播折出 null（JNDI 报告 §三.4 的中间态语义）。入队路径一旦真实，这个折叠就是错的，须撤销（§3.5）。
- 软引用已有**构建期**清除语义（`soft_references` + `null_queues`：导出映像时不可达即清除），与运行期无关，本子项不动它。
- Finalizer 恒禁用（`isFinalizationEnabled=false`），不在本子项（第一节第 4 条只覆盖 Reference 族）。

### 3.2 方案：观察弱 + 登记侧表 + pending 链 + handler 线程（HotSpot 同构）

**语义目标**（约束 3 与第一节第 4 条）：`WeakReference` / `PhantomReference` 的 referent 清空与 `ReferenceQueue` 入队由**对象释放（`Rc` drop）触发**；`SoftReference` 保持「与强引用等价」（约束 3 现状即终态，**不改**）。

**(1) referent 弱化（观察弱字段）。** `Reference.referent` 改弱句柄存储（§一.5 同形态），但与推断弱字段不同类：**观察弱**在 referent 死亡时要有通知动作。经清单声明（`vm_intrinsics.toml` 新节 `[weak_fields]`，键带种类）：

```toml
[weak_fields]
observed = ["java/lang/ref/Reference.referent"]   # 观察弱：死亡触发清空+入队
# inferred = [...]                                  # 推断弱（§一）：无通知，纯弱句柄
```

**(2) SoftReference 的强等价不动 referent。** referent 弱化会波及子类（Java 字段继承，SoftReference 共用 `Reference.referent`）。两案：
- **案 α（推荐）**：发射层对 `SoftReference` 增**合成强字段**（清单驱动：`[ref_semantics] soft_strong = "java/lang/ref/SoftReference"`），构造器写 referent 时同步写强字段——软引用语义 = 强引用，不变量局部、无字段分裂机制；
- 案 β：referent 按「实例化语义子类」分族存储（强/弱两套访问器）——通用但动字段继承机制，留作 α 不行时的备选。

**(3) 登记侧表（与 MONITORS/PARKERS 同纪律）。** 第三张 `IdentityTable<Vec<(弱 Reference 句柄)>>`：identity(referent) → 指着它的全部 Reference。
- 登记：观察弱字段的**写路径**（构造器 putfield referent）置 `word` 第 2 位 `REF_MARK`（`__mark_reference`，同 `__mark_monitor` 形态：映像对象 / null 哨兵跳过、`fetch_or`）并把弱句柄入表。写路径内部次序 **①downgrade（weak+1）②置位 ③入表**：Java 语义下构造期实参由调用方帧强持有，referent 不可能在构造中死亡，次序是防御性纪律（先弱化再标记，任何交错最坏漏一次入队、永不悬空）。
- 摘表：`drop_slow` 见 `REF_MARK` → `reference_registry::forget(identity)`——**先删条目、再析构/归还**（第三节第 5 条同款纪律）。僵尸互动：referent 死亡时必有 ≥1 弱句柄（Reference 的 referent 字段），分配保活到 handler 清字段，时序自洽。
- 表项持**弱** Reference 句柄（Reference 自身死亡即摘，不保活——避免「登记表泄漏 Reference」）；摘除在 forget 扫描时顺手清理。

**(4) pending 链与 Reference Handler（不在 drop 链跑 Java）。** `forget` 对每条表项做两件事，全部无 Java 代码：
- 置 `r.referent = null`：弱句柄交换（其 Drop 只减 weak 计数，无析构重入）；
- 把 r 的强句柄 append 到全局 pending 链（`Mutex<Vec<Reference 强句柄>>` + Condvar）并 notify。

Reference Handler 线程（`Reference.<clinit>` 已启动的现有线程）的 JDK 体照常轮询：`waitForReferencePendingList`（native）从「永久 park」改为 **condvar 等待**；醒后 `getAndClearReferencePendingList`（native：swap 链表）取走，JDK 体执行 `r.clear()` + `r.enqueue()`——**入队是 handler 线程上的普通 Java 代码**，与 HotSpot 架构同构，drop 上下文零 Java 执行（呼应 `handle.rs` 的释放纪律）。

**(5) 闭包口径：撤销 poll 折叠。** 运行期 enqueue 使 `ReferenceQueue.head` / `Reference.next` 变为运行期可写字段 → 闭包分析须把它们按**开放字段**处理（现有 open-field 机制登记，如 `[concrete]` 侧的按名放开转正为常开），`poll0`/`remove` 不再折 null；`FinalReference` instanceof 分支随之入闭包（JNDI 报告记录的类数摆动面转正）。种子无关性单测必须复跑（闭包集合会变）。

**(6) 已知语义偏差登记**（第一节第 4 条允许的口径内）：
- 「GC 未运行的窗口」差异：`new WeakReference<>(new Object()).get()` 在 JDK 通常非 null（尚无 GC），本模型构造完临时对象即死 → get() 为 null。典型正确用法（先持局部/字段再建引用）不受影响；登记进已知偏差文档。
- 入队时机 = 最后一个强引用释放的线程内（同步置 referent=null）+ handler 异步入队，早于 JDK 的「某次 GC 后」——语义上只少不晚，`ReferenceQueue.remove()` 的等待语义完好。

### 3.3 与现有侧表 / PARKERS 回收的一致性

- 三张身份侧表同构：`MONITORS` / `PARKERS` / `REFERENCE_REGISTRY` 同一 `IdentityTable` 容器、同一「置位 → `drop_slow` 先删后释放」纪律、同一分片策略（64 片，锁外释放值）。
- `monitor::forget` 扩展为按位分发（`MONITOR_MARK` → 摘监视器+parker；`REF_MARK` → 摘引用表）；位定义进第三节第 8 条的常量组。
- PARKERS 在线程退出的回收（`forget_parker`）不变；Reference Handler 线程的 parker 由 condvar 等待替代后不再进 PARKERS（它是全局等待点，不按线程身份）。

### 3.4 判据

1. **TestReferenceTypes 族**（现有 e2e，`refersTo0` 消费方）不回归：未清空前 `refersTo` 判定照常。
2. 新测例（e2e）：`TestWeakRefQueue`——强引用置 null 后 `get()` 返回 null、`poll()` 取到引用、`remove(timeout)` 超时与命中两径、`PhantomReference` 入队、`SoftReference` 不清空（强等价）。
3. 并发测例：多线程竞争 drop 同一 referent（登记表原子性）、handler 线程与主线程的可见性（`get()==null` 先于 `poll()!=null` 可观察——JDK 规范的 happens-before）。
4. 闭包面：`poll0` 不再折 null 的程序（JNDI 线已有的 `FinalReference` 摆动面）类集合稳定、种子无关性单测过。

### 3.5 实施分解

| 步 | 内容 | 主要文件 | 规模 |
|---|---|---|---|
| R-a | P0 原语（§一.3，公共前置） | `obj_ref.rs` | 中 |
| R-b | `[weak_fields]` / `[ref_semantics]` 清单节 + referent 观察弱发射 + SoftReference 合成强字段 | `closure/manifest/sections.rs`、`rava_macros_core`、发射层 | 中 |
| R-c | 登记侧表 + `REF_MARK` + forget 分发 | `monitor.rs` 或新 `reference_registry.rs`、`obj_ref.rs` | 中 |
| R-d | pending 链 + 三个 native 换实现（wait/getAndClear/hasPending）+ handler 唤醒 | `reference_impl.rs` | 小 |
| R-e | 闭包开放字段登记 + 种子无关性复跑 + 测例族 | `vm_intrinsics.toml`、e2e | 中 |

---

## 四、thread_local_lint 文档头登记 region_slot（本轮完成）

`generator/crates/closure/src/handwritten/thread_local_lint.rs` 文档头列载体槽清单（对象释放槽、持锁登记槽），§六-5 第 5 条指出缺 `region_slot`。已补记：区域槽（region.rs）——区域内无让出点、一个区域总在同一载体上开始与结束，与既有两类同判据；`ALLOWED` 不变（region 的 `thread_local!` 本就在 `exec_context.rs`，lint 只差文档头登记）。纯文档注释改动，lint 行为零变化。

## 五、原型实测（作业 nogcd-probe1）

探针 `tests/perf/NogcWeakCycle.java`（判据文件，本轮入库）：`next` 前向强 + `prev` 回指强、经静态锚 `head/tail` 逃逸到旧对象后摘锚；cyclic/acyclic 双模式 A/B。

- **方法**：`rava build --release` 后 `/usr/bin/time -f maxrss_kb` 测峰值 RSS，cyclic 100k/400k 两档（增长斜率归因）+ acyclic 400k 对照 + JVM SerialGC 参照；checksum 确定性核对。
- **结果**：作业 nogcd-probe1（jp2，分支 nogc-design @ ec096cd8，rc=0，7:40）：

| 测点 | rava maxrss | rava wall | JVM（SerialGC）maxrss |
|---|---|---|---|
| cyclic 100k×8 节点 | 100.4 MB（102784 kB） | 0.10 s | — |
| cyclic 400k×8 节点 | **357.0 MB（365568 kB）** | 0.39 s | 88.4 MB |
| acyclic 100k×8 节点 | 15.1 MB（15488 kB） | 0.05 s | — |
| acyclic 400k×8 节点 | **15.1 MB（15488 kB，恒定）** | 0.20 s | 88.5 MB |
| checksum（两模式、两档位） | 与 JDK 逐字一致 | — | — |

- **结论**：cyclic 模式 RSS 随迭代**线性**增长（两档位差 262,784 kB / 2.4M 节点 ≈ **110 B/节点**，高于对象理论 48–64 B 的部分为分配器块粒度开销），acyclic 对照**恒定 15.1 MB** 且两档位逐字节相同——同为逃逸到旧对象、唯一差异是 `prev` 回指，泄漏**精确归因于回指成环**而非逃逸本身；运行时开销可忽略（<0.4 s），JDK 参照两模式皆平稳（GC 回收）由 GC 承担。§一.1 的问题陈述与 §一.6 判据 1 的「转绿」基线由此确立：弱化 `prev` 后 cyclic 400k 应落到 acyclic 同档（≈15 MB）。

## 六、交接（下轮派实施代理的入口）

1. **实施序与并行**：P0（一.7 表）→ 三（3.5 表）与一（1.7 表）并行 → 二（2.6 表）；四已完。
2. **决策点**：R1/R2 头布局取舍在 P0-c 基线复测后定（判据：PartitionInteger RSS 增量与 OOM 族不恶化）；案 α/β 在 R-b 首次发射时定。
3. **纪律**：三批均为行为性改动，独立分支、独立提交、每批配判据测试；本设计轮分支 `nogc-design` 只含探针与文档，可直接作实施分支基。
4. **风险登记**：W1-c 的覆盖保持切片是精度瓶颈（首版可再收窄到「覆盖字段只写 null 时 h 同窗死亡」的保守子集）；开放世界判定与闭包引擎 `user_extensible` 必须同源（O-a），两处漂移是隐性 soundness 洞。
