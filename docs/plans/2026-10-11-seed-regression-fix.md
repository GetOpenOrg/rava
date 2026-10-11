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

### 验收（作业 seedfix-unit / seedfix-accept）

（待作业完成后回填数字：单测 closure_independent_of_hash_seed / closure_independent_of_order；
TestSerialLookupPairing 种子 0/1/2 一致且等于 0b91182b 种子 1；HelloWorld / DeepCopy / CollectorsDemo
三例种子 0/1/2 与基线一致。）

## 六、遗留

- 临时诊断打印（RAVA_DBG_*）以独立提交落在分支历史里，合批前可 revert 或保留
  （环境变量缺省不生效、编译干净）。RAVA_DBG_FIELD 支持逗号多键；RAVA_DBG_RET 打印被调成员
  的汇合格 / per / site 答复来源（纯读；曾有一版在打印里调 eval() 拖慢作业，已去掉）。
- 引擎层面的普遍现象（本修复不展开）：全局字段格 `fvals` 只升不降，某轮「暂时 Unknown」的调用
  答复经 putfield 以 Top 并入后不可撤回，是否到达 Top 依赖分析次序。本次以调用点事实消除
  getSecurityManager 的不确定性；其它无返回事实的取值法若再现同类摆动，按同一口径补清单事实。
