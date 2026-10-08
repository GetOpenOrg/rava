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
