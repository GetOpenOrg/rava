# JUnit → Rust 依赖包：条件分析与试点方案（P0 落地设计）

> 日期：2026-09-23
> 来源：用户命题「怎么将 JUnit 编译为 Rust 依赖包、需要什么条件」——基于 dep_scan 实测数据（pilot-deps 项目 2026-09-23 透视）、现有管线能力盘点与远期路线图条目的交叉分析。
> 定位：**P0 阶梯的落地设计**——回答「JUnit 成 crate 需要哪六个条件、哪些现在就能开工、哪些挂在既有决策上」，并给出 M1–M5 里程碑与 M1/M2 任务书草稿。事实跟踪仍归 [tasks.md](../tasks.md) 与 [remaining-issues](2026-09-19-remaining-issues.md)；阶段归属见 [long-term-roadmap §四-3](2026-09-23-long-term-roadmap.md)。
> 关联：[rust-generator-rewrite](2026-09-20-rust-generator-rewrite.md)（库模式=其 driver/emit crate 的需求预演）、[codegen-type-convergence](2026-09-21-codegen-type-convergence.md)（反射 L3 合流对象）。

---

## 一、目标定义：「JUnit 编译为 Rust 依赖包」

一个 Cargo **lib crate**（设名 `junit4`）：junit-4.13.2.jar（350 类）+ hamcrest（108 类）+ 其触达的 JDK 闭包的翻译产物，对外暴露与 Java 侧对应的公开 API 面（`org.junit.Assert`、`@Test` 注解、`JUnitCore` 等），供其他翻译产物以依赖消费。两个形态：

- **过渡形态（本方案主攻）**：workspace 内 path 依赖——`junit4` crate 自带闭包，下游测试代码翻译进同一 workspace 消费它。不要求 JDK 闭包全局稳定，只要求闭包指纹确定性（G-4 已交付，重录与增量构建的前提）。
- **终态形态（挂重写 R9）**：`java-runtime-core` crate（JDK 侧闭包版本化）+ 薄库 crate，可 `cargo publish`。semver 顾虑小（消费者也是生成代码）。

## 二、六个前置条件（按现有管线差距排序）

### ① driver 的 jar 输入模式（不存在，需建）

现状入口是「单个 .java → javac → BFS」。BFS 语料机制**架构上已兼容任意 .class 来源**（registry 按类名索引、`parse_class` 吃字节，不区分 jmods 与 jar），入口层需扩展：jar 条目枚举 + 种子集选择（库的公开 API 面 = 全部 public 成员）+ 跨 jar 解析（junit→hamcrest 的 11 个引用类）。jar 遍历模式 dep_scan / scan_jdk_boundary 已趟过。规模估计 100–300 行，无架构风险。

### ② 库模式发射（不存在，需建）

现状 `project_writer` 发射 `user/` crate + 每测试类一个 `[[bin]]`。需新增 lib crate 发射：`crate-type=["lib"]`、`lib.rs` 汇出模块树、Java 可见性映射（public→`pub`、package-private→`pub(crate)` 近似）、公开面清单由 classfile `access_flags` 驱动（零猜测）。**这是重写方案 driver/emit crate 的需求预演——现在做的设计将来全额继承。**

### ③ JDK 闭包稳定化（最大的架构条件）

现状 `java_runtime/` 是每测试 scratch（闭包按入口裁剪）。「依赖包」语义要求其 JDK 侧版本化可复用。过渡形态绕行（见 §一）；终态等重写 R9。

### ④ 反射 L3 + 注解元数据（JUnit 的灵魂；主机器既定队列）

JUnit 4 运行模型即反射：`getDeclaredMethods` 扫 `@Test`、`isAnnotationPresent`/`getAnnotation`、`Method.invoke` 驱动测试。现状：L1/L2 已落地（Method/Field 元数据表转正，`d8f17b8`/`18f1fa2` 系）；缺口 = (a) build.rs 四表扩 **RuntimeVisibleAnnotations** 扫描；(b) `Method.invoke` 的 L3 分派协议（remaining-issues 在列，与 A-4/TypeIR 合流）。**没有 L3，JUnit 只有 Assert 静态方法族可用，Runner 模型跑不起来**（dep_scan 透视 junit 引 `java.lang.reflect` 的直接后果）。

### ⑤ 线程模型的 join(timeout) 等价（可显式绕开）

`FailOnTimeout` 起线程 + `join(millis)`；单线程协作调度器（`8eca47b`）有 start/join 嵌套泵，但带超时 join 的「超时后终止」在协作模型下无真抢占——要么等价近似、要么标边界。**第一里程碑合法绕法：验证集不含 `@Test(timeout=)` 用例**；timeout 族作为线程模型决策（roadmap §四-2）的实测输入单独立项。

### ⑥ 验证 harness（做这件事的目的本身）

JUnit 的 golden 面友好：`JUnitCore.runClasses` 输出确定性格式的结果摘要。闭环 = 同一测试类在 JVM（真 JUnit）与翻译侧（翻译后的 JUnitCore）输出逐字对账。用例来源：先手写 10–20 条覆盖 Assert/hamcrest 断言族；后期可上 junit 自身的 self-test 源码（在源码发行包，不在 jar 内）。

## 三、有利条件（为什么 JUnit 是对的 P0）

1. **零 jdk-internal**（dep_scan 实证）——无需任何伴生/手写排期；
2. 核心 `Assert` 族 = 静态方法 + 装箱/字符串格式化——S-19/S-6/intern 驻留表（`755ad7e` 系）直接复用；
3. hamcrest `Matcher<T>` 泛型接口族 = A-4/T-2 载体系统现成压力面；
4. 输出确定性 → golden 可比；
5. 闭包规模可控：junit+hamcrest+JDK 闭包预计与 TestOptionalChain（56+1321 类、bin 192MB）同量级，构建分钟级，不越资源纪律。

## 四、里程碑（每步独立止损）

| 里程碑 | 内容 | 退出判据 | 主要前置 |
|---|---|---|---|
| M1 | **hamcrest 先行成 crate**（108 类纯函数式） | 手写断言用例 main 消费该 crate，golden 逐字一致 | ①②（jar 输入 + lib 发射）|
| M2 | junit 的 **Assert 子集**进 crate（Runner 不翻） | 断言类测试 golden 一致 | 无新增（M1 延续）|
| M3 | **注解元数据 + 反射 L3** → Runner 路径 | `JUnitCore.runClasses` 跑通 `@Test` 发现与调用 | ④（主机器域，需协调）|
| M4 ✅ | timeout/线程边界 | `join(ms)` 等价或 compatibility.md 显式标注 | ⑤（随线程模型决策）——**2026-09-25 GOLDEN OK (m4)**：JunitTimeoutMain（限时内通过/失败/sleep/混合）；真实超时不可达已入 compatibility.md |
| M5 ◐ | **workspace 打包终态**：`java-runtime-core`+`hamcrest`+`junit4`+用户测试 path 依赖全链 | 一个 workspace 一键构建运行 | ③终态（可等重写 R9）——**2026-09-25 过渡形态 GOLDEN OK (m5)**：跨 crate 分派链（user 类实现 lib 类型并被回调，三 crate 往返）逐字一致；修 protected→pub 可见性。终态（java-runtime-core 版本化）仍挂 R9 |

**结论**：①②⑥ 是本机/新代理可立即开工的纯增量工程；④ 是硬前置但在主机器既定队列；③终态与⑤挂在既有决策上且都有过渡形态绕行。**M1+M2 现在就能排任务书，不阻塞任何在途工作。**

> **M1+M2 已落地（2026-09-23，`5f00970`）**。双机撞车遗留的对照实验结论（另一台机器的
> WIP `5b11391` 已弃）：jar→crate 的关键分叉点在「lib 类走哪条通道」——整类生成+用户
> 通道语义（不参与虚分派传播）会让覆盖方法落 panic 存根（StringDescription.append 实证）；
> 可达性通道 + class_cache 落置是已验证的正确路线。M3+ 的实施以此为准。

## 五、licensing 一句话

JUnit 4 = EPL 1.0、hamcrest = BSD-3：翻译产物属衍生作品，内部 pilot 无碍，**publish 前过一次法律口径**。

---

## 附录：M1/M2 任务书草稿（供派发，两件合一单）

> 域 = `scripts/main.py` 入口层 + `codegen/callchain.py` 种子/BFS 侧 + `codegen/emitter/project_writer.py` lib 发射。与主机器在途（G-3 收尾 / VarHandle 引用族 / VirtualThread 调查）零重叠；**不动** build.rs 元数据表（反射 L3 主机器域）、`runtime/java_rta_macros`、`var_handle*`、`jdk/internal/reflect`。

**第 0 步**：worktree `git worktree add /tmp/wt-libpilot -b feat/jar-input-lib-emit`（基于最新 main）。

**件 1（M1）hamcrest crate**：driver 增加 `--jar <path>` 输入模式（枚举条目→parse_class 进 registry，种子=全 public 类；跨 jar 解析预留多 `--jar` 重复项）；project_writer 增加 `--lib` 发射（lib crate + access_flags 驱动 pub 面 + 模块树）；以 hamcrest-3.0.jar（`pilot-deps/target/pilot-libs/`）产出 `hamcrest` crate；手写 10+ 断言用例的 main（Assert 风格 + assertThat/Matcher 链）作 bin 消费它，`--jdk 21 --clean` 与 JVM 侧 golden 逐字对账；回归：TestOptionalChain 等既有 6 例零回归（driver 入口改动面）。
**件 2（M2）junit Assert 子集**：第二 `--jar` 喂 junit-4.13.2.jar，种子收敛到 `org.junit.Assert`（+CompareAssertion/AssertionError 邻域，Runner/annotation 族显式不入种子）；产出 `junit4` crate（依赖 hamcrest crate）；用例 main 走 `Assert.*` 全族，golden 对账；BFS 闭包计数与 bin 体积入档（对标 TestOptionalChain 量级）。

**验收**：两件 golden 全绿或如实归类下一层；`python3 -m compileall -q codegen scripts` 过；双种子生成树 diff 归零；闭包指纹/审计线变化逐项解释。
**资源纪律**：禁全量；定向 + 回归 ≤6 例；顺序无 -j；大闭包一次一个。
**提交**：每件独立提交，中文不加尾注；有 forgejo remote 则 push 分支报主机器。

## 六、记录维护

- 里程碑推进时更新 §四 状态列；M1/M2 派发后任务书定稿以 tasks.md 为准；
- 与 [long-term-roadmap §四-3](2026-09-23-long-term-roadmap.md) 的阶梯表互为引用（本文 = 其 P0 的展开）；
- 反射 L3 / 线程模型条件的状态变化回落 remaining-issues 对应条目。
