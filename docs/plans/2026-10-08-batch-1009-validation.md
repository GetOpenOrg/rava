# batch-1009 合批验证记录（2026-10-08）

batch-1009 = batch-1008（f19e46e0，含 HelloWorld 闭包 OOM 修复与 docs-align）+ closure-gates（c6a19c4c）+ boot-image-s5（57a9cec2），合批提交 d5a2cb5d。
本批修复提交在 batch-1009 上：

| 提交 | 内容 | 归属 |
|---|---|---|
| df1eb230 | jdk_literal_lint 豁免外置测试模块 `<x>_tests.rs`（s5 的 `project/jimage_tests.rs` 含 JDK 类名字面量） | s5 |
| 6db384e8 | 映像容器对象的字段值记入按对象值表（见 §一） | batch-1008（c1d P1 × 引导映像） |

## 一、ModuleDescriptor.provides 存根（已修，6db384e8）

现象：d5a2cb5d 抽查 HelloWorld / FieldDemo / CollectorsDemo / ReflectionAPI / ReflectionBasic 运行期 panic 于存根
`java/lang/module/ModuleDescriptor.provides:()Ljava/util/Set;`。运行路径：`System.newPrintStream` → `Charset.forName` →
`ServiceLoader` → `ModuleLayer.getServicesCatalog`（boot layer 的 servicesCatalog 运行期为 null）→ `ServicesCatalog.register` → `provides`。

根因：c1d 按对象实例字段值（P1，`obj_fields.rs`）只记字节码 `putfield`；映像中的容器形态对象（`HashMap@image647` 等，
`image_obj_site`）不经字节码写入，按对象读只得初值——`size` 为 0、`table` 为 null，`HashMap$HashIterator` 的迭代被折成不可达，
`getServicesCatalog@53` 的 `next()` 无值，`register` 的形参为空。98e733c9（b1007）上 @53 有值、`provides` 可达；
f19e46e0 与 d5a2cb5d 均不可达，故归 batch-1008（c1d P1 与引导映像的交互），与 gates / s5 无关。

修法：`image_drain` 对有抽象对象的映像实例，逐字段把映像值（`image_pv`）经 `obj_field_put` 记入该对象的按对象值表。
验证：kr2 上 HelloWorld 闭包 `--why provides` 可达（`register@6` 为 `provides()` 的 37 个类）；抽查中上述 5 例在 6db384e8 上全部通过。

## 二、b1009-spot-d5a2cb5d 抽查（ref 6db384e8，同 tag 续跑）

名单：/tmp/b1008-tests.txt 去掉 TestJcaSasl，加上 s5 各项，共 47 例。上述 5 例的 d5a2cb5d 旧失败已从 state 中删除，并在 6db384e8 上重跑。

- 截至交接：通过 22 例，失败 1 例（TestFieldReflectAll），us1 正在跑 TestJndiNoProvider，约 24 例待跑（sg1 / us1，日志 /tmp/b1009-spot4.log，结果在 cluster_results/spot/b1009-spot-d5a2cb5d/）。
- 通过：CollectorsDemo、DeepCopy、FieldDemo、HelloWorld、ReflectionAPI、ReflectionBasic、SwitchExpressions、TestAesGcmRound、TestAppClassLoader、
  TestBootLayer、TestCipherDesModes、TestClassForNameInit、TestClassModuleFace、TestClassResourceStream、TestDirectBuffer、TestEmbeddedUrlRebuild、
  TestEnumAdvanced、TestEnumBasic、TestFieldHandleOffsets、TestForNameComputedName、TestForNameInit、TestGetFields。
  其中 DeepCopy 与前 14 例中的部分结果来自 d5a2cb5d（续跑时沿用已通过的结果）。
- 引导审计（d5a2cb5d，jp1）：initPhase1/2/3 完成，未登记失败为 0，引导期没有读 `/java-runtime` 或 `lib/modules`。

## 三、TestFieldReflectAll：null_recv 违约（未修，归 batch-1008 c1d-url-b2）

现象：`FieldAccessorImpl.throwFinalFieldIllegalAccessException(Object)` 中，`o.getClass().getName()` 的接收者被判定恒为 null，
运行期 panic `null_recv 违约：java/lang/Class.getName`。触发点是用户第 54 行 `CONST.set(null, 1)`（static final 写，期望 IllegalAccessException）。

证据（kr2，作业 b1009-attr3-6db384e8）：6db384e8 与 d5a2cb5d 上，`MethodHandleIntegerFieldAccessorImpl.set` 的 P2（写入值）
与 `throwFinalFieldIllegalAccessException(Object)` 的 P1 都没有流，只有 P0（接收者）。所以这不是 6db384e8 引入的。

根因：485d30b6（c1d-url-b2，成因 4）对 `[facts.field_writes] handle_setters / handle_getters` 的调用点建模时，
对象实参与写入值**不流入被调方形参**（`engine/field_access.rs` 模块注释），存取效果由调用点直接给出。
但被调方字节码在运行期照常执行，而且会使用这些形参：最终字段写入的异常路径调 `value.getClass().getName()`，
各访问器还按 `value instanceof Xxx` 分支。形参值集为空，依赖它的事实（null 接收者折叠、instanceof 折叠、派发）就不健全。

终态修法方向（由新代理实施）：被切断的形参不能当作空集。可选做法：
- 给被调方形参一个不逃逸的概括值（值的类型身份，不含抽象对象本身），让 getClass / instanceof 有值可依；
- 或者把这类形参标为未建模，让依赖它的折叠（null_recv / instanceof / 去虚化）退回保守结果。

两种做法都要保持成因 4 的收益：写入值不经 `FieldAccessor` 派发汇合点与 `invokeExact` 手写值池整组逃逸。

## 四、单测失败判定（b1009-ut-d5a2cb5d：4 例失败）

| 用例 | 判定 | 证据 |
|---|---|---|
| jdk_literal_lint `no_jdk_class_literals_outside_lang` | s5 引入，已修（df1eb230） | `project/jimage_tests.rs` 第 78–91 行 |
| closure_cli `param_string_constants_fold_switch` | 已知失败，b1007 起即失败 | 引导映像（L）日志链膨胀，`sun/net/www/protocol/jrt/Handler` 入链，HelloWorld 超过 1000 类；U12 处理中 |
| closure_cli `container_elements_per_object` | batch-1008 内已存在（c1d-url-b2 × 引导映像），非 gates / s5 | 作业 b1009-attr2-6db384e8（kr2）：c1d-url-b2 分支尖 3391eb2d 通过（3.6s）；合入 batch-1008 的 04600e21 与 b558e0c2 闭包 OOM（ulimit 13G）；s5 尖 57a9cec2（基于 b558e0c2）同样 OOM；f19e46e0、d5a2cb5d、6db384e8 断言失败（Square.name 入链，m1/m2 元素汇合）。待查：引导映像下 ConcurrentHashMap 元素按对象分开为何失效 |
| gates_cli `known_gate_ranks_first` | d5a2cb5d 合批后即失败（gates × batch-1008 基线交互），根因未定 | 作业 b1009-attr3 / attr4（kr2）：6db384e8 上排第一的门是 `AccessController.executePrivileged:(PrivilegedAction;…)`（body，single.model 284 / real 536），其后是 executePrivileged(PrivilegedExceptionAction) 53、Charset.lookup 5；`GateFixture.main@198` 根本不在门候选中。`--why D0.<init>` 显示唯一路径为 main 根 → GateFixture.main@198 → Impl0.run@4 → D0.<init>，按理该点单切增量约 1600，却未入候选。夹具闭包 4438 类、196s、2.5GB。gates 分支尖 c6a19c4c 基于 b558e0c2，本测试 closure_cli.rs:55 失败 / 闭包 OOM（RSS 超过 12G，已手动终止），无法在其自身基线上复核。待查：`engine/gates.rs` 约 181–206 行候选资格（`eligible` / `cands` 过滤）在 batch-1008 基线下为何排除该点 |

另：s6 代理在 b1008-ut（6934dc93）上看到的 16 例 build_cli 失败，在 b1009-ut-d5a2cb5d 上不存在（该测试二进制 17 例全过）。
协调者已定位到 reflect-direct（fb9fde6d），与 batch-1009 无关。

## 五、在跑 / 已完成作业

- 抽查 b1009-spot-d5a2cb5d：ref 6db384e8，sg1 + us1，本机分发器日志 /tmp/b1009-spot4.log；跑完后看 state_jdk21.json 与 error_logs/。
- 归因作业（均已完成）：b1009-attr-6db384e8、b1009-attr2-6db384e8、b1009-attr3-6db384e8、b1009-attr4-6db384e8，结果在 cluster_results/job/<tag>/01_kr2.log。

## 六、下一步

1. 等 b1009-spot 跑完，汇总通过 / 失败；新失败按 d5a2cb5d 与 6db384e8 对照归因。
2. 按 §三修 TestFieldReflectAll（field_access 被切断形参的健全性）。
3. 查 container_elements_per_object（引导映像下 CHM 元素汇合）与 known_gate_ranks_first（接口调用点边形态）。
4. 以上修好后，在新 head 上重跑全量单测（`--job-timeout 14400`）。

## 七、batch-1010 回归修复（分支 fix-1010）

三项回归均源自 batch-1008（c1d-url-b2 × 引导映像），在 batch-1010 基线 c18e8fc4 上修复（5f3a844c 合入该基线）。

### 1. TestFieldReflectAll null_recv（87c24dea）

- 根因：c1d-url-b2 的 field_access 按调用点建模 Field.get/set 时，对象实参与写入值不流入被调方形参；被调方字节码在运行期照常使用这些形参（`throwFinalFieldIllegalAccessException` 里的 `value.getClass().getName()`），空值集被判为 null_recv 并折叠成 panic。
- 修复：被切断的被调方形参按未建模来源处理，由 unmodeled 以它们为根沿流边标记。只影响折叠导出，闭包类集不变。
- 验证：
  - 抽查 fix1010-fra-87c24dea 6/6 通过（含 TestFieldReflectAll）。
  - SyspropsLambdaLeak 3499 类，与基线一致（作业 fix1010-sp）。
  - TestSetAccessibleBoundary 与 TestFieldReflectAll 同点 panic（field_accessor_impl.rs:234）：见 §七.4。

### 2. container_elements_per_object / known_gate_ranks_first（a102b325、6db7cbd9）

- 根因（二者同源）：
  - f19e46e0 修 parseSig 维数爆炸 OOM（04600e21 的超时）时，把反射数组分配（`Array.newInstance`）的结果概括为 open(Object)。涉及两种情况：元素镜像已达 2 维，以及元素 Class 推不出。
  - `BytecodeDescriptor.parseSig@125` 的 `Array.newInstance(t,0).getClass()` 作用在 open(Object) 上，展开为全部已实例化类的镜像，用户类也在其中。
  - 这些镜像经 AnnotationType 与 getDeclaredMethods 枚举进入反射暴露，使用户类的全部方法入链。表现为 ElemTrack 的 `Square.name` 入链，以及 GateFixture 出现无条件的 `C:Impl0 → M:Impl0.run` 边（门候选由此消失）。
- 二分旁证：
  - c1d-url-b2 尖 3391eb2d 通过，466 类。
  - 合入后 04600e21 / b558e0c2 闭包 OOM。
  - f19e46e0 之后断言失败。
- 修复：
  - a102b325：限维结果改为 open(`[[Object`)。3 维及以上的结果恒为 Object[][] 的子类型，这样概括仍可靠，不动点有限。
  - 6db7cbd9：元素未知的结果改为 open(`[Object`) ∪ open(`[Z`…`[J`)（`reflect.rs::any_array`）。在数组协变下恰好涵盖全部数组，getClass 只展开为数组镜像。
  - 只有 a102b325 不够，两例仍失败，因为剩余泄漏来自元素未知的那一支。
- 验证（作业 fix1010-v-6db7cbd9，kr1）：
  - container_elements_per_object 通过（101s）。
  - known_gate_ranks_first 通过（228s）。
  - HelloWorld 闭包 3350 类，RSS 2.54 GB，99.8s。
  - ElemTrack 3353 类，Square.name 不入链。
- 与 batch-1011（a88d7075）的关系：
  - 在 a88d7075 上仍失败（diag5：Square.name 仍入链）。
  - locale-build 修复 2（零长映像数组不收集全程序 ArrayList.add）与修复 3 不涉及这条反射数组路径，不能消解本问题。本修复与它们正交。

### 3. 撤回的尝试

24e6a882（长度 0 的映像数组按空数组暂存）实测无效，且与 locale-build 修复 2 同点重复，已由 084cd438 撤回。终态以 a88d7075 的实现为准。

### 4. 验证汇总

- 全量单测，fix1010-ut3-6db7cbd9（kr1，6068s，batch 标准命令 `--no-fail-fast`，跳过种子 / 顺序两例）：
  - 只剩已知失败 `param_string_constants_fold_switch`（U12 日志链）。
  - container_elements_per_object、known_gate_ranks_first、reflect_new_array_element_precision 均通过。
  - 其余 37 个测试二进制全过。
- 早先一次 fix1010-ut-6db7cbd9 没有加 `--no-fail-fast`，在 closure_cli 处停止。它跑了被跳过的两例：
  - `closure_independent_of_hash_seed` 失败（closure_cli.rs:150）。
  - `closure_independent_of_order` 失败（closure_cli.rs:237）。
  - 二者属上游已知不稳定，batch 单测一律跳过。本修复未单独复核。
- reflect_new_array_element_precision（b1011-ut-a88d7075 新增失败，seeds_agree 断言）：在 6db7cbd9（batch-1010 基线 + 本修复）上通过，来源在 log-chain / locale-build，与 batch-1010 基线无关。
- TestSetAccessibleBoundary：
  - 抽查 fix1010-sab-87c24dea（kr2）1/1 通过，确认第 1 项修复已覆盖该 panic。
  - 在 084cd438 上两次复跑（fix1010-sab、fix1010-sab2）都在 java_base 编译阶段 OOM（11.9G）。属 c18e8fc4 引入的已知 OOM 阻塞，charset-ext 代理在二分。
- 反射数组 e2e 抽查（fix1010-arr-6db7cbd9）：TestArrayComponentType 同样在编译阶段 OOM，已中止。等 OOM 修复后复跑 TestArrayComponentType、TestReflectArrayDeep、TestVmPlatformNatives。

完工 head：fix-1010 的 6db7cbd9（加本记录提交），按计划合入 batch-1010。
