# Python 生成器 → Rust 重写方案

> 日期：2026-09-20
> 状态：方案定稿，**未开工**（前置条件未满足）
> 前提决策（已拍板）：重写在以下条件全部满足后进行——
> ① **生成器语义发现收敛**（**2026-09-21 用户拍板修订**，原「65/65 全绿」基数过时——测试基数已扩至 166，且与活跃队列的 downcast 链/A-5/A-6/线程层等多周级任务冲突，按旧口径重写将永远排队）。新口径三条可观测信号：(a) 红线集全绿；(b) 166 全量失败全部归类为 runtime/边界/线程类——无「生成器形态」新条目持续产出；(c) 新错误族发现频率降到月级（2026-09 当期为周级）。含义：remaining-issues 的 A 类条目全部闭环、或显式降级为「Rust 时代条目」（重写时随 crate 定型一并解决，如 TypeIR 完全体）；
> ② Python 拆分方案 [2026-09-20-source-file-split-plan.md](2026-09-20-source-file-split-plan.md) **窗口 0–3 全部完成**（§五 终态指标达成；2026-09-21 时点 0/1/2 已完成并指纹验证，仅窗口 3 挂 G-1/G-2 开工时）；
> ③ 批次 5 收尾项中 **P-1（JDK 字面量清零）与 G-4（生成确定性）先行完成**——前者决定哪些表"不迁移"，后者是 golden diff 可比性的前提。**G-4 已随阶段 A 仪表提级（2026-09-21，收敛路线图 §六处方 1）**。
>
> 决策含义：**拆分不只是代码卫生，它定义了移植的模块边界**——拆分后的每个 Python 模块 = 一个移植单元，本方案 §3.3 的移植表与其 §3.2–3.8 一一对应。

---

## 一、目标与终态（量化）

| 指标 | 现状 | 终态 |
|---|---|---|
| 生成器实现语言 | Python ~24.3k 行（codegen/ + scripts/，其中 trace 脚本 ~5k 不移植） | Rust；**Python 归零**（`codegen/`、`scripts/`、`pyproject.toml`、`uv.lock`、`.python-version` 全部删除） |
| 分发形态 | `python3` + `.venv` 环境依赖 | **单二进制**（`java-rta`，子命令覆盖生成/运行/测试编排） |
| 宏展开实现 | `java_rta_macros` proc-macro 内 ~3.3k 行，不可单测 | `java_rta_gen` 库 crate 承载全部 expand 逻辑；宏壳 ≤100 行；**生成器与手写层共用同一 expand** |
| IR | rs_ir 已强类型，但 coerce/vars/blocks 边界存在字符串往返 | **IR 是唯一货币**：模块边界 API 零 String 表达式往返（类型系统强制） |
| regex 依赖 | 111 处使用点 | 热路径改真 parser；其余经 `regex` crate 原样移植并由 golden 覆盖 |
| 隐式默认语义 | 61 处 `setdefault`/`defaultdict` | 逐点显式化为结构体字段/带默认构造 |
| 可读层禁令 | §16 禁止清单 + CLAUDE.md 规则 4 靠人工审查 | 生成器内建 `lint` 子命令 + CI 单测，全量禁止项 0 命中 |
| e2e | 65/65 | 全程 65/65 不退步；移植期 golden diff **零语义差异** |
| 生成耗时 | 未测量 | R0 先 profile 存日志，以数据定目标（不凭感觉承诺） |

---

## 二、架构设计

### 2.1 crate 布局

```
generator/                    # 新 workspace 成员：Rust 生成器（终态单二进制 java-rta）
├── crates/
│   ├── classfile/           # .class 解析（← classfile.py：_Reader/pool/decode/attributes）
│   ├── jvm_sig/             # 泛型签名递归下降（← sig_parse.py）
│   ├── ty/                  # 类型映射 + 层次实参 + 签名类型（← type_map/type_args/sig_types）
│   ├── resolve/             # 层次查询 + 成员归属 + 命名（← hierarchy/member_owner/member_naming + invoke_sig）
│   ├── ir/                  # RsIR 节点 + render 单出口（← rs_ir.py，A-3 缺口在此直接定型）
│   ├── sim/                 # 栈模拟 + 变量物化 + 块模拟（← stack/vars/blocks/unify/fusion）
│   ├── cfg/                 # 结构化 + simplify（← structure/simplify）
│   ├── instr/               # 指令翻译（← sim/ 十个子模块 + invoke 家族 + coerce 值强转）
│   ├── emit/                # 产物发射（← class_writer 收尾形态 + attrs/vtable_util/struct_gen/…/project_writer）
│   └── driver/              # CallChainDiscovery BFS + 编排 + cargo 调用 + lint（← transpile/callchain + scripts/main.py）
│       └── src/bin/         # java-rta：generate / run / test / lint 子命令
runtime/java_rta_gen/         # 从 java_rta_macros 拆出：expand 全逻辑 + GenContext（库）
runtime/java_rta_macros/      # 薄 proc-macro 壳：parse → java_rta_gen::expand（≤100 行）
```

- 拆库的硬约束：**proc-macro crate 技术上只能导出过程宏**，要让 expand 可被生成器调用、可被单测，必须拆出 `java_rta_gen` 库——这不是偏好，是语言限制。
- scratch 工作区的 path 依赖由一个变两个（`java_rta_gen` + `java_rta_macros`），绝对 path + 共享 `CARGO_TARGET_DIR` 缓存语义不变。

### 2.2 展开路径决策：保持宏编译期展开（单一展开路径）

**决策**：生成器继续产出 `java_class!` 块，展开仍发生在 cargo 编译期（薄宏壳 → `java_rta_gen`）；库化只为可测性与复用，不改为生成期预展开。

- 理由 ①：可读层是产品目标（CLAUDE.md：vtable dispatch、`Rc<dyn Trait>`、borrow 不出现在生成文件里）。预展开会把 vtable 机械写进 .rs，违反定位。
- 理由 ②：手写 `runtime/` 与生成代码走**同一条**展开路径，零分叉、零一致性维护。
- 理由 ③：宏壳薄 + 库全量单测后，"宏黑盒难调试"的问题由测试侧解决，而不需要牺牲可读层。
- **否决的替代**：生成器预展开。可读层倒退，且手写/生成两条路径需持续保证一致。

### 2.3 IR 与类型系统（重写的第一原则）

- `ir` crate 直接定型 A-3 缺口（`CastExpr`/`InstanceOfExpr` 节点），不经过 Python 中转。
- **强制手段**：`sim`/`instr`/`resolve` 的公开 API 签名不接受、不返回 `String` 形式的表达式/类型；`_str_to_rs_type` 这类"字符串解析回 IR"的函数**禁止存在**——用 API 设计杜绝，不靠纪律。
- `registry: dict[str, ClassInfo]` 定型为 `Registry` 结构体（内部 `BTreeMap`，迭代顺序确定性）；61 处隐式默认逐点显式化。
- 111 处 regex 逐一盘点三分类：① 已有等价 parser（签名解析）直接对接；② 热路径改写 parser；③ 低频的经 `regex` crate 原样移植，golden diff 覆盖。

### 2.4 禁令自动化（重写带来的新能力）

`driver` 新增 `lint` 子命令 + CI 单测，对生成产物扫描，要求 0 命中：

- §16 可读层禁止清单：`borrow()`、`downcast::<T>()`、`Rc::new`、`Object::from_any` 等；
- CLAUDE.md 规则 4：生成器源码中不得出现 JDK 类名字面量（P-1 三张表 `_CLS_ABBREV`/`_CLASSNAME_MAP`/`_INTERFACE_IMPLS` **不迁移**）；
- 命名红线：无 `jvm_` 前缀、无 Java 命名空间外的自造 trait。

---

## 三、移植策略

### 3.1 真相源与行为冻结

- 移植全程 **Python 版是唯一真相源**：发现 bug → 修 Python → 重录 golden → Rust 跟进。Rust 侧任何行为差异必须在 golden 报告中显式豁免并登记差异清单；**清单归零是 Python 退役的条件**。
- 移植期冻结生成器语义：不加特性、不改输出语义。e2e 新发现的 bug 一律按原流程在 Python 修。

### 3.2 golden 基建（R0 先行，先于任何移植代码）

- **corpus**：65 个 e2e 测试的全量生成物快照（Python 版产出）。
- **diff 工具**：token 级归一化对比——空白/格式容差，语义零容差。
- 每个移植单元的验收 = 该模块在 corpus 上替换后 diff 归零 + 相关 e2e 子集通过。

### 3.3 移植顺序（leaf → root，依赖单向，单元 = 拆分后的模块）

| 阶段 | Python 源（拆分后） | Rust 目标 | 验收 |
|---|---|---|---|
| R0 | — | golden corpus + diff 工具 + Python 管线 profile | corpus 固化；耗时分布存日志 |
| R1 | `java_rta_macros` | `java_rta_gen` 拆库 + 薄宏壳 | 宏展开快照 golden；`cargo check`；行为零变化 |
| R2 | `classfile.py` | `classfile` | corpus diff 归零（纯解析层） |
| R3 | `sig_parse` / `type_map` / `type_args` / `sig_types` | `jvm_sig` + `ty` | diff 归零 |
| R4 | `hierarchy` / `member_owner` / `member_naming` / `invoke_sig` | `resolve` | diff 归零 |
| R5 | `rs_ir` / `stack` / `vars` / `blocks` / `unify` / `fusion` | `ir` + `sim` | diff 归零 |
| R6 | `structure` / `simplify` | `cfg` | diff 归零 |
| R7 | `sim/`×10 / `invoke` / `invoke_virtual` / `coerce` | `instr` | diff 归零 + 全量 e2e |
| R8 | `attrs` / `vtable_util` / `struct_gen` / `field_gen` / `interface_gen` / `inherited_gen` / `method_gen` / `import_gen` / `clinit_extract` / `class_writer` / `project_writer` | `emit` | diff 归零 + 全量 e2e |
| R9 | `callchain` / `transpile` / `jdk_resolver` / `inherited_calls` / `native_upcalls` + `scripts/main.py` / `run_tests.py` | `driver` | 全量 e2e + 单二进制跑通 65 测试 |
| R10 | — | 双版本并行一个全量周期 → 切默认 → **删除 Python** | 65/65；差异清单归零；`grep -r python` 工作流无残留 |

- 并行编排：worktree + subagent 按 crate 边界分派，每阶段独立提交、独立 golden 验收（沿用既有流程）。
- 阶段间无行为交叉：每个阶段结束时仓库处于"双实现并存、golden 全绿"状态，任何时候可暂停。

### 3.4 切换与退役

R10 双跑期间默认仍是 Python；切换后观察一个全量周期（含至少一次 scratch 冷启动）再删 Python。删除是终态动作，不留兼容层。

---

## 四、注意事项（教训 → 硬性规则）

| # | 教训来源 | 硬性规则 |
|---|---|---|
| 1 | `expand_inner` ~1600 行、`_gen_class_rs` ~1300 行，11 个生成阶段共享局部变量 | 生成阶段 = `&GenContext` 纯函数序列；GenContext 字段表复用 [2026-09-18-block-rs-refactor.md](2026-09-18-block-rs-refactor.md) §四已审计的设计；**单函数 ≤150 行是入库红线** |
| 2 | `classify_vtable_body` 对 `quote!().to_string()` 做空格敏感的 contains 匹配，出过 E0308 | 禁止对生成物做文本匹配判定；一切结构化判定走 IR/AST |
| 3 | rs_ir 强类型但 coerce/vars/blocks 边界字符串往返 | §2.3 API 级禁令：边界签名不出现 String 表达式，"字符串解析回 IR"的函数不得存在 |
| 4 | P-1 三张 JDK 字面量表在 Python 里清不干净 | 不迁移 + §2.4 lint 自动化：规则从"纪律"变"测试" |
| 5 | 泛型该读 `generic_signature` 却读 descriptor 的区分散落在补丁里 | 类型决策集中到 `ty` crate 一处；descriptor 只用于 mangle/重载判定 |
| 6 | per-test scratch + overlay + 共享 target 缓存是编译时间的生命线 | driver 原样保留：手写文件靠"无宏标记"识别、scratch 覆盖规则、绝对 path 依赖、`CARGO_TARGET_DIR` 共享 |
| 7 | 陈旧 scratch 造成"之前通过现在失败"的误诊 | golden 录制与 e2e 一律 `--clean`；R0 的 profile 存日志（log-first） |
| 8 | Python 与 Rust 的 `f64` 格式化、map 迭代顺序天然不同 | R0 建立差异预研清单：Rust 侧统一 `BTreeMap`/稳定排序；浮点格式化规则移植对齐——golden diff 负责暴露全部此类差异 |
| 9 | 重写动机若含"Python 慢"，未测量 | R0 先 profile：解析/BFS/emit 占比；cargo 编译大概率是端到端大头。重写的理由是**单二进制分发 + 与宏/运行时同语言**，性能目标以 R0 数据为准 |
| 10 | Python 动态性的隐式行为（61 处 setdefault/defaultdict、None 传播、异常控制流如 `CfgAuditError`） | 逐点显式化：默认值进结构体、错误用 `Result` 显式传播；禁止用 `unwrap`/静默默认值掩盖未移植的语义 |

---

## 五、风险登记

| 风险 | 缓解 |
|---|---|
| R1 拆库期间宏行为回退（宏是全项目编译的地基） | 拆库本身做宏展开快照 golden；拆库 PR 独立、先于任何移植 |
| 移植把 Python bug 一并搬过去 | §3.1：bug 先修 Python、重录 golden；差异清单制度 |
| 双实现并存期的维护混乱 | 行为冻结（§3.1）+ 阶段化暂停点（§3.3）：任何时刻仓库处于全绿状态 |
| Rust 版初期迭代变慢 | 前置条件已把语义不确定性（65/65）消化完毕，重写期只剩机械翻译 + golden 验收 |
| corpus 覆盖不到的生成路径 | corpus = 65 个 e2e 的全量生成物，与 e2e 同源；新增测试先录 golden 再移植 |

---

## 六、与既有文档的关系

- [2026-09-20-source-file-split-plan.md](2026-09-20-source-file-split-plan.md)：其窗口 0–3 完成是本方案前置条件；其模块边界 = 本方案移植单元；本方案落地后该文档的产物随 Python 一起退役（拆分成果以"语义边界确认"的形式沉淀进 Rust crate 边界）。
- [2026-09-18-block-rs-refactor.md](2026-09-18-block-rs-refactor.md)：Phase 2/3 的 GenContext + gen/ 设计直接成为 `java_rta_gen` 的 API 与本方案 R1 的实现蓝图。
- [2026-09-18-p0-python-refactor.md](2026-09-18-p0-python-refactor.md)：其收尾（窗口 1 ④）完成即使命达；不再有后续 Python 投资。
- [2026-09-13-target-architecture.md](2026-09-13-target-architecture.md)：调用链 BFS、存根化、native 共置三原则在 `driver`/`emit` 中原样继承；其中 `native_impls/` 布局已被 CLAUDE.md 规则 3 的共置方案取代，以 CLAUDE.md 为准。
