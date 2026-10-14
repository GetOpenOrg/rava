# 种子无关性回归修复（fix-seed-reg，2026-10-11/12）

任务：修复 `closure_independent_of_hash_seed` 在 batch-1011i（0b91182b）上的失败——
`tests/e2e/35_io/TestSerialLookupPairing.java` 种子 0 闭包比种子 1/2 多
`java/security/ProtectionDomain$JavaSecurityAccessImpl` 与 `jdk/internal/access/JavaSecurityAccess`
两类（本分支实测类数 2711 vs 2709；任务书引历史口径 2317 vs 2316）。
分支自 1011i 切出（1011j/1011k 上被掩盖，不作为开发基线）。

## 一、诊断链（六轮服务器插桩作业 seedreg-diag1..8，日志 cluster_results/job/seedreg-diagN/）

每轮一个假设、一个环境变量门控打印（`RAVA_DBG_CTOR`/`RAVA_DBG_INIT`/`RAVA_DBG_SITE`/
`RAVA_DBG_FIELD`/`RAVA_DBG_EXT`/`RAVA_DBG_METH`，见 git 历史上的临时诊断提交）：

1. **不是构造器请求面**（diag2）：c1d-a54-reflect 的 `ctor_reqs`（77974a5e/4e7f24f5）在两种子下
   收敛一致——`ReflectionFactory.newConstructorForSerialization(Class)@46` 与
   `ObjectStreamClass.getExternalizableConstructor@5` 对 ProtectionDomain 都请求 `"()"`，
   `expose` 均未选中（user=false、picked 空）。任务书的首要嫌疑排除。
2. **不是类初始化触发**（diag3）：`init(java/security/ProtectionDomain)` 的触发站点集合
   （SecureClassLoader$1.apply@12 的 new、ForkJoinPool 工厂、InnocuousThread.<clinit> 等）
   两种子完全一致。
3. **分派接收者来自引导映像字段**（diag4，用户线索确认）：种子 0 在
   `SharedSecrets.getJavaSecurityAccess@0` 读点把映像常量
   `Const(Ref{ JavaSecurityAccessImpl, obj: Image(227, []) })` 并入字段常量格
   （`field_put`），`ObjectStreamClass.newInstance@65` 的分派答案于是含
   `JavaSecurityAccessImpl.doIntersectionPrivilege`；种子 1 两者皆无。
4. **映像状态两子相同**（diag6）：`[img-install]` 证实装载后
   `SharedSecrets ∈ build_time` 且 `statics(javaSecurityAccess)=R(227)` 在种子 0/1 都成立
   （引导映像 digest 亦同，`214de741…`）。差异在于种子 1 的 `javaSecurityAccess` 字段节点
   从未创建（`image_field` 未打印）——即 `getJavaSecurityAccess` 的字节码事件从未被应用。
5. **差异在 ObjectStreamClass.newInstance 的重分析时机**（diag7）：种子 0 里该方法节点被
   处理 3 次（aseq 1→3，两轮失效重分析），种子 1 只处理 1 次；只有重分析后的那次应用了
   `@38 invokestatic getJavaSecurityAccess`。
6. **可达性由 `domains` 字段的 getfield 折叠决定**（diag8）：`newInstance()` 的
   `@15 ifnull domains`——`domains` 只被全 JDK 内无人调用的私有方法
   `getProtectionDomains` 写入，真程序恒 null，`@38/@65` 不可达。oracle 对
   `ObjectStreamClass.domains` 的答复在两种子已观测部分全部为 Null（通配/按对象），
   尚未观测到使 @38 存活的非空/未知答复（diag8 的 uniq -c 被 head -60 截断，
   diag9 补全中；`ObjAns::Never`（⊥）分支当前打印未覆盖，是剩余盲点）。

根因概括：**种子改变的内部表迭代序 → OSC.newInstance 的失效/重分析轮数不同 →
某轮分析中 `domains` 读不被折为 null → `@38 getJavaSecurityAccess` 进入事件集 →
字段节点首次出现、引导映像值并入 → @65 分派牵出 impl + 接口**。种子 1（2709，
不含两类）是语义正确侧；种子 0 是过近似（真程序 domains 恒 null，@65 不可达）。

## 二、1011i↔1011j 二分（作业 seedreg-bis-*，三种子 TestSerialLookupPairing 类集合对照）

| 提交 | 位置 | counts（类） | 一致 | impl 在种子 0/1/2 |
|---|---|---|---|---|
| 0b91182b | batch-1011i 顶 | [2711, 2709, 2709] | 否 | [是, 否, 否] |
| 9e253ee0 | 1011j 线，s0-g1-share 合并后 | [2711, 2709, 2709] | 否 | [是, 否, 否] |
| fe923a2f | 1011j 线，fix-jdk25 合并后 | [2711, 2709, 2709] | 否 | [是, 否, 否] |
| d279b821 | fix-jdk25 分支自身（扫描器 cfg 提交） | [3082, 3082, 3082] | 是 | [否, 否, 否] |
| 74919e58 | batch-1011j 顶（含 nogc-escape 合并 5df7e9ba） | [2709, 2709, 2709] | 是 | [否, 否, 否] |

判读：
- **s0-g1-share（G1 枢纽共享 / 按效果键去重）不是修复点**（9e253ee0 仍失败）。
- **fix-jdk25 合入 1011j 线也不是修复点**（fe923a2f 仍失败）；任务书原先的「被 fix-jdk25 掩盖」
  需修正为：d279b821 所在的 fix-jdk25 分支基线本就不含触发回归的 batch-1011i 改动
  （其闭包 3082 类、无 c1d-a54-reflect 收窄），通过≠修复。
- seedreg-bis-jtip 已确认 74919e58 通过（2709×3，种子 0 掉到正确侧），掩盖落在
  fe923a2f..74919e58，即 **nogc-escape 合并（5df7e9ba）**——该区间闭包 crate 仅 manifest
  新增 [escape] 节（inert），但 runtime/java_runtime 手写源码有实质改动
  （class_impl.rs `Class::for_class` 加 `region::__Suspend`、string_ext.rs `From<&str>` 同、
  obj_ref.rs / region.rs 等），手写层扫描随之变化。这是**掩盖而非根治**：根因（§一）仍在，
  换任何手写层扰动都可能重新翻出。

## 三、修复策略（交接：未在本轮落地，硬时限到点）

二分已证明 1011j 线上没有任何一次「根治」：通过侧（74919e58）靠 nogc 手写层扰动把种子 0
压到正确侧，属掩盖。根治须让 `ObjectStreamClass.newInstance` 的 `@15 ifnull domains`
折叠在两种子下同轮收敛，候选修法（按证据强度排序）：

1. **补齐 `ObjAns::Never`（⊥）路径的失效-重算闭环**：diag7 显示种子 0 里该方法被重分析 3 次、
   种子 1 只 1 次；oracle 已观测答复全部为 Null/未知（diag9），剩余盲点是
   `obj_field_answer` 走 `bottom_answer()`（`Ret::Never`，当前诊断打印未覆盖）的轮次。
   先给 `Some(ObjAns::Never)` 分支加打印复跑 diag9 形态作业，确认 Never → Value(Null) 的
   重算在种子 1 缺席的原因（读者登记 / `obj_queries_bind` / `obj_readers_recheck`）。
2. 若确认是「未登记读者」：修登记点，使按对象读 ⊥ 的方法在对象集 / 对象值增长时必失效重分析
   （次序无关的不动点口径，与 `ctor_init.rs` 模块注释的既有口径一致）。
3. 验收口径：`closure_independent_of_hash_seed` 在 fix-seed-reg 基线上通过，且四例
   （TestSerialLookupPairing / HelloWorld / DeepCopy / CollectorsDemo）种子 0/1/2 一致、
   修复后各种子集合与 0b91182b 种子 1（正确侧，2709 形态）相同。

## 四、验收与二分结论（本轮）

- 二分结论见 §二（1011i→9e253ee0→fe923a2f 全失败；74919e58 经 nogc 合并掩盖通过；
  d279b821 的通过是分支基线不含触发改动，非修复）。
- 代码修复未落地（6 小时硬上限），分支以可编译 + 完整诊断链 + 二分数据交接。
- 本地 cargo check（`--release --tests`，heavy_lock）通过；诊断打印均环境变量门控、
  缺省零生效。

## 五、修复（2026-10-12，seedfix-diag1 插桩观测后落地）

### 根因观测（作业 seedfix-diag1，commit f70bb38c，种子 0/1 对照）

补齐 RAVA_DBG_FIELD 的通配未知原因打印（fvals / fopen / fopen_names / fopen_all / deser / hw）、
field_put 值打印、fopen / 全局开关翻转打印与 [proc] 节点号后，两子对照（cluster_results/job/seedfix-diag1/）：

- `ObjAns::Never`（⊥）分支两种子都**从未触发**（按对象答复 0 次——ObjectStreamClass 非容器形态类，
  `obj_field` 直接 None）；交接文档 §三.1 的「按对象读 ⊥ 读者登记」方向排除。
- 唯一的 `[open-all]` 是 `deserializer` 的 deser 翻转（两种子相同）；`domains` 因 ObjectStreamClass
  声明空的 `serialPersistentFields`（<clinit> 无字符串常量 → serial_declared = ∅）**不受 deser 放开**。
- 种子 0 独有：`[field-put] domains <- Top` 三个写点——`ObjectStreamClass$2.run@336`（首个）、
  `initProxy@147`、`initNonProxy@548`；此后 fvals[domains] = Top，newInstance 等读点全部转通配未知。
  种子 1 全程只有 `Const(Null)` 写入。**交接文档「domains 只被无人调用的私有方法写入」有误**：
  `getProtectionDomains` 的调用方在内部类 `ObjectStreamClass$2.run@333`（javap 证实）。

### 真正的根因链（修订）

`$2.run@336` 的 `putfield domains` 写入 `@333 invokevirtual getProtectionDomains(cons, cl)` 的返回值。
`getProtectionDomains` 的字节码（JDK 21）在 `@13 invokestatic System.getSecurityManager` / `@16 ifnull 148`
处本应折「恒 null → 直接 return null」，但 `System.getSecurityManager` 是普通字节码方法（读 static
`System.security`），**无任何返回事实**——它是否折 null 取决于当时 `System.security` / `cons` 等全局格子的
中间态，于是 `rvals[getProtectionDomains]` 在两种子下于 Const(Null) 与 Top 之间分叉；种子 0 某轮 Unknown
答复经 `write_pv → PV::Top` 并入全局 fvals[domains]（格只升不降、不可撤回），`newInstance@15 ifnull` 随之
展开，@38/@65 牵出两类。种子 1 的 Const(Null) 是「运气好」的一侧：同样依赖处理次序，不是稳定不动点。

### 修法（commit 64ddf8d0）

清单 `[facts.returns]` 新增 `"java/lang/System.getSecurityManager:()Ljava/lang/SecurityManager;" = null`：
原生二进制没有启动器、无人调 `setSecurityManager`（HotSpot 由启动器按 -D 装配，JDK 21 缺省不装），
`System.security` 恒 null。调用点事实在 `invoke_result` 里先于汇合格 / 派发求值返回（`call_info.fact`），
两种子、任何轮次都得到同一个 Null——`@16 ifnull` 恒跳、数组构造分支静态不可达、`rvals` 恒 Const(Null)、
`$2.run@336` 恒写 Null，`domains` 的折叠不再依赖任何格子的中间态。这是文档既定的建模口径
（docs/plans/2026-09-29-rust-closure-analyzer.md §3.2「清单即边界」；vm_intrinsics.toml 175-176 行注释
本就写明「getSecurityManager 恒 null」而未落地条目），不是类名特判、不依赖固定种子、不依赖 nogc 掩盖。

### 验收（作业 seedfix-unit4 / seedfix-accept4，commit b898b399）

- **集合口径（seedfix-accept4，jp2，3155s，ACCEPTANCE PASS）**：TestSerialLookupPairing 种子
  0/1/2 一致（2709/2709/2709）且三集合（类/方法/反射成员）逐项等于 0b91182b 基线种子 1（2709）；
  HelloWorld 641×3、CollectorsDemo 662×3、DeepCopy 2707×3，三例各种子集合与 0b91182b 基线
  同种子完全一致（修复未扰动这三例）。
- **单测口径（seedfix-unit4，jp1）**：cargo test --release -q -p driver --test closure_cli -- --exact
  closure_independent_of_hash_seed closure_independent_of_order——作业超出本轮时限仍在跑，结果落
  cluster_results/job/seedfix-unit4/（该测例的种子一致性性质已由 seedfix-accept4 在四例集合口径上
  验证，含回归用例 TestSerialLookupPairing 本身）。
- generator 本机 cargo check（--release --tests，heavy_lock）通过。

## 六、遗留

- 临时诊断打印（RAVA_DBG_*）以独立提交落在分支历史里，合批前可 revert 或保留
  （环境变量缺省不生效、编译干净）。RAVA_DBG_FIELD 支持逗号多键；RAVA_DBG_RET 打印被调成员
  的汇合格 / per / site 答复来源（纯读；曾有一版在打印里调 eval() 拖慢作业，已去掉）。
- 引擎层面的普遍现象（本修复不展开）：全局字段格 `fvals` 只升不降，某轮「暂时 Unknown」的调用
  答复经 putfield 以 Top 并入后不可撤回，是否到达 Top 依赖分析次序。本次以调用点事实消除
  getSecurityManager 的不确定性；其它无返回事实的取值法若再现同类摆动，按同一口径补清单事实。

## 七、第二例（s7-k4 链，2026-10-14 闭环）：写站点先于返回常量建立的次序窗口

K4-2a 交接的确定性遗留（S7 §9.9）：`closure_independent_of_hash_seed` 在 s7-k4 链失败，
二分定归属阶段 1/2（s7k4d-detmid（042465b7）与 s7k4d-det4 逐字同，基线 a07fbb6f 过）。
作业 s7k4s-why1（e9597fef，`--why` + RAVA_DBG_FIELD/RET，种子 0/2 = 2709/2711）取证：

- **不是 §五修法的旁路**：`getSecurityManager = null` 事实两种子均生效（`getProtectionDomains`
  的 `@16 ifnull` 恒跳），§五结论仍然成立。
- **同一两类、新一层机理**：种子 2 里 `ObjectStreamClass$2.run@336 putfield domains`
  首轮写 **Top**、其后（失效重写）才写 Const(Null)；种子 0 全程 Const(Null)。链：
  `@333 invokevirtual getProtectionDomains` 的返回常量格 `rvals` 在种子 2 的首轮
  **缺席**（被调方该轮分析无返回点）→ oracle 答 Unknown → 写入以 Top 并入全局 `fvals`
  不可撤回（格只升不降）→ `newInstance@15 ifnull domains` 不折 → `@38
  SharedSecrets.getJavaSecurityAccess` → 映像常量分派牵出 impl + 接口两类。
  rvals 随后出现时经 `Dep::Ret` 失效重写已晚——**写入是否污染依赖「调用方写站点」与
  「被调返回常量建立」的先后**，正是 §六末段预言的普遍现象的字段值集面。
- **closure crate 在 a07fbb6f..e9597fef 间零改动**：阶段 1/2（e6edfa24/a867b085）的
  手写层挪移（obj_ref/sync_model 出 runtime 树、object.rs `__Obj` impl 移出等）只是
  扰动内部表迭代序，把潜伏的次序敏感翻出面——与 §二 nogc「掩盖」同一性质：基线过
  ≠ 无缺陷，只是两侧恰好同面。s7k4d-seed2（19802647）的「四例全同」同属侥幸
  （三种子恰好都落在含两类的一面，集合相等但非正确侧）。

### 修法（cc2cfc3e）：pending_write 暂缓并入 + 失效重算强制重跑

- `bytecode.rs::pending_write`：PUTFIELD/PUTSTATIC 的写入值恰是**无标签、单来源
  （产生它的调用点）**的引用，调用点有唯一字节码目标（`Ctx::call_info`）、目标已建
  节点（`noreturn.created`——标记建模旁路调用边的站点无节点、返回常量不会出现，
  不属暂态）、且 `rvals` 缺席时，本次 Top **暂不并入**（`fvals`/`ovals`/`owild`），
  偏移登记进 `pending_skips`。与 `noreturn.rs`「尚无返回」的乐观-重算同一原理
  （次序无关不动点口径）：缺席是暂态，不是 ⊤。
- 方法失效重算时（`process_bytecode` 的 changed 偏移计算）`pending_skips` 一并并入：
  事件值不变也重跑——被调返回常量升到 Top 时按新格补并入（防漏），出现常量时写入
  常量（防污染）。真不返回的被调方使写入不可达，不并入同样正确。
- 验收（全绿）：**s7k4s-det1**（jp2，rc=0）`closure_independent_of_hash_seed` +
  `closure_independent_of_order` **2 过 0 失**（6612s，较修复前 5058s 慢约三成——
  暂缓写入多出的 Dep::Ret 失效重算轮，可接受）；**s7k4s-tslp1 / s7k4s-seed1**
  （us1/kr1，rc=0 ×2，`seed-sets: ALL-IDENTICAL`）四例种子 0/1/2 集合对 e9597fef
  逐项全同（TSLP 2662×3、HelloWorld 641×3、CollectorsDemo 662×3、DeepCopy 2660×3
  ——修复只影响确定性不影响集合）；**s7k4s-link1**（kr1，rc=0）HelloWorld +
  TestStackWalkerFrames 链接级逐字对 JVM PASS；generator `cargo check --release
  --tests` 本机过、closure --lib 252 过。
- 附带澄清（交接疑点的 disposal）：「两工具口径差异」非闭包缓存指纹——指纹本就计入
  hash seed（cache.rs `f.field("hash_seed", …)`），且两口径都不开缓存；真因是
  `--jdk 21`（cargo 测试与 --why 作业，JDK 发现）与 `--java-home`（s7k6_seed_sets.sh
  固定参考 21.0.11）解析到不同 JDK，闭包宇宙差 47 类（TSLP 2709/2711 vs 2662），
  回归只在发现侧宇宙复现、在参考侧宇宙两种子恰同面（集合相等但非语义正确侧）。

### 遗留（本次不展开）

- 摘要共享（`share.rs`）的陈旧窗口：共享摘要携带的旧 Unknown 事件值在 rvals 已建立后
  落地仍会写 Top（与修复前同行为，不劣化）；如再现按「pending 入 Analysis 工件」口径根治。
- 分支直接依赖 pending 调用值的暂态两侧展开（类集合只增）未在本例出现，机制上与
  `noreturn.rs` 头部注释的收尾口径同源，再现时按同一口径处理。
