# rava 全方位代码审查报告 · 第二轮（2026-10-11）

> 第一轮（[docs/reports/2026-10-11-code-review.md](2026-10-11-code-review.md)）覆盖工程七域（架构/Rust 质量/并发/性能/安全/Python 工具链/测试工程）。本轮为 12 个专项深度审查：语义正确性三连（转译保真、手写边界、生成产物）、工程外围（宏系统、DX、构建、文档、治理流程）、深度专项（unsafe 逐行、fuzz 规划、跨平台、许可合规）。审查为只读，共 19 个领域子代理、全部发现带 file:line 证据。

## 一、总体结论

**产品的核心承诺经受住了最严格的检验：翻译语义保真度 4.5/5、手写边界 4.5/5、unsafe 逐行审计 141 处仅 1 处确凿违例——第一轮最大的盲区（"翻译出来的东西对不对"）补上了，且结果好于预期。** 真正的红灯在两处：**对外发布链路（许可合规）有两个 P0 级阻断**——仓库零许可声明，且 "JDK 字节码翻译进产物" 的 GPLv2+CE 定性完全未决；**合入治理与第一轮结论需要修正**——机器闸门 merge_daemon 自 2026-10-07 起已停用，现行放行全靠协调者合并叙事，"main 既有" 的实测滞留窗口为 8.5 小时跨 5 次合入。

| # | 领域 | 评分/风险 | 一句话结论 |
|---|---|---|---|
| 1 | JVM 语义保真 | 4.5 / 5 | 最担心的整数溢出/Rust panic 疑点全量排查不成立（全走 wrapping/掩码）；无 P0，8 条 P2/P3 缺口 |
| 2 | 手写边界语义（102 个 _impl.rs） | 4.5 / 5 | 消息级保真罕见认真；唯一行为性 bug 是 System.arraycopy 缺前置范围检查 |
| 3 | 生成产物质量 | 4 / 5 | 可读性有硬审计门；声明层样板仍是编译峰值主体（Digester 4.51GB vs 目标 2GB），未捕获异常无 Java 栈帧 |
| 4 | proc-macro 宏系统 | 4 / 5 | 单一真源与确定性是显式工程；体分类文本启发式存在两份已漂移实现 |
| 5 | 开发者体验 | 3 / 5 | 贡献者体验 4 分、外部 Java 用户 2 分：崩溃无 Java 栈帧、生成代码编译失败是死胡同、无用户文档 |
| 6 | 构建系统 | 4.5 / 5 | 三层自举结构清晰；产物以绝对路径回指 rava 仓库（可移植性无出口） |
| 7 | 文档信息架构 | 3 / 5 | 链接健康率 100%，但 plans/ 109 份无索引无状态制度，4 份状态头失真 |
| 8 | 失败治理流程 | 3 / 5 | 知识库 4.5 分、流程执行 2 分：机器闸门停用、放行靠叙事、值守无告警环 |
| 9 | unsafe 逐行审计 | 141 处全审：✅118 / ⚠️15 / ❌1 | 唯一 ❌：Handoff 无界 Send；另发现直接内存原子/非原子混访（真 UB，P1） |
| 10 | fuzz 就绪度 | 4 / 5 | 代码侧教科书级 fuzz 对象；新发现 2 处递归 panic 面；方案已备好待落地 |
| 11 | 跨平台 | 4 / 5 | 实为 Linux+macOS 双平台设计（macOS ≈90% 就绪）；Windows 编译期明确排除 |
| 12 | 许可合规 | **高（发布场景）** | 零 LICENSE + JDK GPLv2+CE 翻译链路定性未决 = 两个发布阻断；语料溯源未核实 |

第二轮发现总计：**P0 × 4、P1 × 14、P2 × 30+、P3 × 25+**（明细见 §三；与第一轮重复的交叉验证项已标注）。

### 对第一轮的修正与更新

- **[重要修正] merge queue 并非现行流程**：第一轮测试工程审查基于 merge_queue.py/merge_daemon.py 分析闸门；本轮流程审查发现 daemon 自 2026-10-07 起停用（cluster-testing.md:552），现行是"合批 + 协调者叙事放行"。第一轮"抽查名单提交方自选"的结论仍成立且被强化，但"恢复守护 or 删除死代码"成为新决策点。
- **[数据更新]** known_failures 现 6 条（第一轮 8）、failure_patterns 101 条（第一轮 92，77 fixed/19 pending/5 open）。
- **[交叉印证]** 未捕获异常无 Java 栈帧被产物质量与 DX 两个独立代理同时发现；第一轮并发审查的"监视器侧表"结论被 unsafe 审计确认为"逻辑缺陷非 UB"。

## 二、跨领域主题

**主题 1：语义正确性全面达标，剩余是"见证缺口"而非"实现错误"。** 转译保真无 P0——整数 wraparound/除法/移位/浮点转换/异常表/类初始化全部正确且有 e2e 覆盖；手写边界唯一 P1 是 arraycopy。剩余发现集中在三类：审计埋点缺失（field-npe 零发射点但文档声称已接入）、兜底分支依赖分析器正确性（checkcast 空栈折叠）、无金标见证的边界（string switch 哈希冲突、NaN 位模式）。修法都是低成本补点。

**主题 2：「最后一公里」断裂反复出现。** Java 栈帧机制（pc_map/capture_java_frames/行表）全部建好，但 uncaught 出口没接；merge_daemon 状态机 546 行完备但停用；known_failures 判定器精确但单测无基线。模式是：**能力已建成、闭环没接通**——这是流程/集成债而非技术债，也是回报率最高的修复方向。

**主题 3：对外发布的整条链路从未被审视过。** 许可（零声明 + GPLv2+CE 未决）、可移植性（产物绝对路径回指）、用户文档（零）、安装引导（无 MSRV/无 rustup 指引）——产品定位是"Java 开发者本机得到原生二进制"，但所有质量投入都朝向贡献者侧。

**主题 4：macOS 是被低估的资产。** epoll/kqueue 双后端、ELF/Mach-O 双格式、refjdk 四平台段、历史 mac 修复记录——macOS 就绪度远高于典型 Linux-only 项目，只差三处 /proc 语义降级 + 一次实测验证。

## 三、最高优先级行动清单

### 发布阻断（若计划开源/对外交付，P0）

1. **许可基础设施**：选定并添加 LICENSE/NOTICE；19 个 Cargo.toml + pyproject 补 license 字段。
2. **法务专项：JDK 翻译链路定性**：把事实面（Temurin GPLv2+CE → .class 机器翻译 → 静态编译进用户二进制）与关键问题（Classpath Exception 是否覆盖非 Java 目标语言翻译）提交法律意见；仓库对 junit 库已自我认定"翻译产物属衍生作品"，同一逻辑必须对 JDK 链路给出结论。参考先例：GraalVM native-image 采用 GPLv2+CE + 商业双轨。
3. **语料溯源**：核实 jnc/ruva（含 wesleyegberto 命名空间）上游许可，建 PROVENANCE，确认不了的替换。

### 正确性（P1）

4. **System.arraycopy 前置范围检查**（手写 #1）：负 length 静默成功、越界先部分写入再抛——入口一次校验五参数，保"异常时 dest 零写入"。
5. **直接内存原子/非原子混访封死**（unsafe #1）：`native_memory.rs` 的 read/write/fill/copy（memcpy）与 update（原子 CAS）可对同一地址并发 = Rust 级 UB，普通 Java（VarHandle CAS + Arrays.fill 同缓冲区）可触发；修复模式在 heap 数组侧已有现成实现可平移。
6. **未捕获异常接 Java 栈帧**（产物 #2 = DX #1）：`error.rs:204` 的 report_uncaught 只打消息 + Caused by，而 `capture_java_frames()` 基础设施已全部就位——"无需学 Rust"承诺最便宜的兑现。
7. **「main 既有」机器基线**（治理 #2，强化第一轮）：单测期望失败集入库 + 合批判定改集合差分；实测 order 回归滞留 8.5h/5 次合入，转绿纯靠修复恰好在途。
8. **Handoff 收紧 Send 界**（unsafe #2）+ TLS lint 修复合入（第一轮遗留）+ `start0` 失败回滚（第一轮遗留）。

### 高价值低成本（P2 批量）

- fuzz 四处修复后落地（tableswitch/lookupswitch 分配封顶、注解/描述符递归深度上限——fuzz 代理已给出完整引入方案与目标函数草案）。
- 宏系统：消灭 classify 文本启发式双实现、`java_try!` 标签改确定性编号、`jvm_native` 解析失败报错而非静默。
- 构建产物可移植出口（env 占位符或 `--vendor-runtime`）；rust-version 声明；java_runtime/build.rs 改 if-changed 写。
- 文档：docs/README.md 一页索引 + 四值状态头清偿（4 份失真头部）；README 三处口径修正（Python 版本、用例数、边界描述）。
- DX：QUICKSTART + 排查文档一页；长构建心跳输出；cargo/JDK 缺失行动提示。
- 治理：决定 merge_daemon 生死；pattern 库放行时校验 + pending 自动翻 fixed + fixed 条目不出 hint。
- 跨平台：macOS 实测验证跑 + 三处 /proc 降级（totalMemory 返回 0 是语义错误）。

## 四、各领域详细报告

### 4.1 JVM 语义保真（4.5/5）

**最高疑点排除**：整数 add/sub/mul/neg 全走 `wrapping_*`（`instr/src/sim/arith.rs:70-176`）；idiv/irem 走运行时函数（除零消息 `"/ by zero"` 与 JVM 逐字一致，MIN/-1 用 wrapping_div）；字面量除法白名单仅放行"非零十进制正整数"（数学上排除 Rust debug panic）；移位一律 `&0x1f/&0x3f`；浮点转换依赖 Rust 1.45+ `as` 的饱和语义恰与 JLS §5.1.3 一致；fcmpl/g 的 NaN 分支精确；strictfp 在 JDK17+ 为无操作，忽略即正确。

八类语义对照全部正确且大多有 e2e 覆盖（详见报告语义对照表）：异常表按序匹配/finally 复制、类初始化完整 JVMS §5.5 状态机（含 ConstantValue 例外）、数组协变存储检查、lookupswitch/tableswitch、字符串拼接配方保孤立代理项。

发现（无 P0）：
- **[P2] field-npe 审计 ID 零发射点**——`instr/src/log.rs:20,50` 定义但无任何 `log.audit(Audit::FieldNpe)` 调用；compatibility.md §4 声称"除 stacktrace 外均已接入"不实，`--deny equiv::field-npe` 永不触发。
- **[P2] checkcast/instanceof 空栈兜底**（`sim/control.rs:151,224`）：折叠为 `Lit(false)` 的正确性完全转嫁给闭包分析器的 fold 决策，无运行时兜底。
- **[P2] string switch 哈希冲突分支无 e2e 见证**（全语料无 "Aa"/"BB" 类同哈希标签）。
- **[P2] 类字面量跨线程同一性**：线程内按名缓存（`object.rs:85-93`），跨线程 `X.class == X.class` 为 false——已声明的近似，但 §3 未点明具体偏差形态。
- **[P3] final 方法 vtable 登记规则与对照文档 §6.4 表述不一致**（语义等价但文本漂移）；**[P3] tableswitch 全展开无条数上限**；**[P3] catch 体终点依赖 LVT**（无 `-g` 输入未验证）；**[P3] frem NaN 位模式无对照承诺**。

亮点：22 处 wrapping/掩码发射点全量可审计；常量池 String/StringUtf16 双形态保 UTF-16 码元全链路；异常表规划的"全 return 间隙才合并"保守策略。Top 行动：补 field-npe 发射点；建"整算术必须 wrapping"的机器扫描规则；补三个探针 e2e。

### 4.2 手写边界语义（4.5/5）

102 个 `*_impl.rs`（~10.7K 行）深读 14 个单元。消息级保真罕见：`"Index {} out of bounds for length {}"`、ClassFormatError 整句、ProcessBuilder 尾空格都逐字复刻 HotSpot；JLS §5.1.2 拓宽矩阵完整；zip 族与 Inflater.c 逐位一致；canonicalize0 是 JDK_Canonicalize 逐句移植。

发现：
- **[P1] System.arraycopy 缺前置范围检查**（`system_impl.rs:16-75`）：`length<0` 静默返回 Ok；越界时 dest 先被部分改写才抛 AIOOBE。JDK javadoc/HotSpot 均为前置检查 + "arraycopy: length is negative" 文案 + 异常时零写入。
- **[P2]** `FileDescriptor.sync0` 异常降型 IOException 代替 SyncFailedException（窄 catch 不再命中）；readBytes 越界异常两文件形态互相矛盾（消息参数 off vs len 疑用错）；`mapLibraryName` 缺分隔符 IAE；`getNestHost0` 用 `$` 启发式而 NestHost 元数据已在库中（顶层类名合法含 `$` 时误判）；`Class.forName0(null)` 得 CNFE("null") 而非 NPE；`TimeZone.getSystemGMTOffsetID` 恒 "GMT"。
- **[P3]** getRawTypeAnnotations 恒 null、NPE 扩展消息 ""/null 差异、FOS 以 O_RDWR 打开、canonicalize0 消息缺前缀、multiNewArray 维度消息。

未深读残余风险：method_handle_natives.resolve 的 kind 矩阵、unix_native_dispatcher 字段映射、network_interface/process_impl 管道关闭序。Top 行动：arraycopy 前置检查；IO 族异常形态统一对一次 io_util.c；nest 元数据替换启发式。

### 4.3 生成产物质量（4/5）

（证据限制：build/ 不在工作区，基于发射模板 + 宏展开代码 + 测试断言 + 性能文档实测数据。）

量化画像（文档实测）：宏展开 5.2×（HelloWorld 4.7MB→24.6MB）；IR 归类中 `__` 样板 39.8% + gil 泛型 4.5% ≈ **44% 是 runtime/宏样板**；Digester 声明层构成：`__jb_*` 外壳 16.4%（3 万 fn）、wrapper 固有 impl 35%（7.5 万 fn）、vtable impl 11.7%；拆层后实现 crate 全部达标（≤1.26GB），**但声明层 Digester 4.51GB / DeepCopy 5.44GB vs 目标 2GB 未达**，驱动量是 fn 条目数（斜率 0.071GB/MB）。用户逻辑占比 <0.1%（HelloWorld 33 行 Java vs 5.7 万行生成）。增量：改用户源时重编 ≈2.0s、JDK 侧 0 重编。

发现：
- **[P1] 声明层样板仍是编译峰值主体**——文档已排期未落地的三件套（继承字段访问器按需生成 −1.35 万 fn、接口载体回退收敛、wrapper 字段协议查询合一）。
- **[P1] 未捕获异常默认无 Java 栈帧**（`error.rs:204-229`，注释自认）——基础设施全在，只差出口接线。
- **[P2]** 拼接/装箱求值序噪声（`String::of("")` 空串头、三重包装、合成 temp 名）；宏展开剩余未瘦身项挂起 S7；构造器存根孪生双写（Digester 665 对）；拆层后类块属性头双份落盘；`From<X> for 每个祖先` 全链展开；大用户项目无增量路径。
- **[P3]** JDK 体内合成局部变量名；pc_map 缺表 panic（文案清晰）；body crate 装箱对档案变化敏感；元数据双路径消费无双写（正面确认）。

亮点：可读性五禁形态硬审计门 + 测试断言；行级 Java↔Rust 调试映射成体系（`// line N` → frame_lines.json → rava-link DWARF 预解析 → printStackTrace 还原 Java 帧）；生成确定性工程化（27 例逐字节对照、BTreeMap 修 HashMap 迭代序、热写零重写保 mtime）；stub 策略语义安全（漏边 panic 而非静默错执行）。

### 4.4 proc-macro 宏系统（4/5）

core/shell 分层名副其实（shell 346 行）；`plan` feature 只 gate 导出不 fork 逻辑；剥体判定单一真源有 decl 常量/body 重算的 `const assert!` 双保险；确定性显式设计（BTreeMap、sort+dedup、HashMap 只点查）；展开路径零 panic/unwrap。

发现（无 P0/P1）：
- **[P2] 体分类文本启发式两份已漂移实现**——`classify.rs:20-46` vs `base_fns.rs:37-53`（后者 `chain().skip(1)` 语义不同，当前侥幸无害）：plan 判定与实际展开分叉的唯一现实通道。
- **[P2] `java_try!` 标签取自进程级 AtomicUsize 计数器**（`try_macro.rs:55`）——同输入两次展开非字节稳定，破坏增量缓存命中；团队专门做首现序归一等于承认。
- **[P2] 静默降级三处**：`#[jvm_native]` 解析失败原样返回（丢类初始化触发点）、`attr_str` 吞错、未知属性一律忽略。
- **[P2] 单测错位**：19 个测试全在小模块，erasure(534 行)/rewrite(521)/inner_impls(477)/interface(442)/parse(374) 零直接测试。
- **[P2] shell 里 281 行真实展开逻辑**（opaque/upcast/native_attr）违背自身分层目的。
- **[P3]** `__` 前缀卫生无防线（Java 字段 `__r` 可撞宏生成名）；属性派生 Ident 丢 span；`java_field` 属性文本匹配双轨制；audit 双跑；`VTableBodyKind::Skip` 死分支；递归无深度防护。

亮点：M3-c 差分审计（宏持纯位映射逐方法核对生成器类型，失配 compile_error）；链接符号带签名指纹（FNV-1a，签名不一致链接失败而非静默错配）。Top 行动：消灭双实现、补确定性回归测试、消除静默降级。

### 4.5 开发者体验（3/5）

贡献者体验 4 分（build_status.json 教科书级、取包命令可复制、precheck 编译前报缺口、参数校验带迁移指引、重型闭包隐形保护）；**外部 Java 用户体验 2 分**。

发现：
- **[P1] 崩溃无 Java 栈帧**（同产物 #2）；**[P1] 长构建零进度**（分钟级静默，观感等于挂起，`cargo.rs:237-264`）；**[P1] 无用户向导/排查文档**（docs 全部面向贡献者）。
- **[P2]** cargo/rustc 缺失裸报 os error 2、无 MSRV；文档残留已删除的 `--lib`（三处）；生成代码编译错误对 Java 用户死胡同；JDK 缺失无下一步；stub 命中后无引导链；plans 109 份无索引。
- **[P3]** diag/ 25 脚本不可发现；README 数字过期（Python 3.11+ vs 实际 >=3.12、"61 类 1066 例" vs 实际 73/1110）；CLI 参考藏在 environment-variables.md。

Top 行动：接通崩溃→Java 源最后一公里；一页 QUICKSTART + 三个失败场景排查；构建心跳 + 环境缺失行动提示。

### 4.6 构建系统与工具链（4.5/5）

三层自举结构（generator workspace → runtime 真源 → 生成 scratch），关键决策有文档论证；fetch_reference_jdk.sh 生产级（sha256 先验 + 版本交叉验证 + 原子落位 + 目录锁）；档案复用键 128 位摘要三元组；实测确认 panic=abort 不影响 cargo test。

发现：
- **[P1] 产物 workspace 绝对路径依赖 rava 仓库内 runtime crate**（`emit/src/ctx.rs:132`、`overlay.rs:141`）——用户拿到 scratch 必须在同路径持有 rava checkout，可移植性无出口（无 env 插值/vendor 模式逃生门）。
- **[P2]** java_runtime/build.rs 构建期改写仓库内已提交文件（native_status.toml）且无 if-changed 保护；fetch_pilot_deps.sh 的 sha256 是事后记录（无先验基线比对，首取被投毒的 jar 以"合法哈希"入语料）；全仓无 rust-version 但 build.rs 已用 cargo ≥1.77 语法；syn 双版本（2.0.119 + 3.0.4 via serde_derive）；method 直声明 regex 破坏 workspace 继承。
- **[P3]** `.cargo/config.toml` 依赖 cwd 发现；overlay 对 Cargo.toml 字符串改写无失配断言；lint 口径四处重复（12/12/15/1 条漂移）；CLAUDE.md 引用仓库外 heavy_lock.py；pyproject description 占位符；rava_coro 独立 profile 无效。

### 4.7 文档信息架构（3/5）

链接健康率 100%（160+ 链接零死链）；reference/ 4 份全部新鲜（10-03~10-10 更新）；TOML 头注释是"文档即契约"范例；存在可复制的标注范例（三份文档有正确的取代/历史标注）。

发现：
- **[P1] plans/ 无索引**——109 份靠文件名日期区分；tasks.md 用缩写引用（"c1d 计划 §31"×18 次）零全路径；被引用最多的 5003 行活文档从任何入口无一次全路径链接。
- **[P1] 时效标注制度性缺失**——仅 16/108 有状态头且 12 种表述无词表；Python 生成器 10-01 删除后无一份 Python 时代文档被回溯标注。
- **[P1] 口径漂移比已知更宽**——用例数四版本并存（README 1066 / compatibility 166 / tasks 1109 / 实际 1110）；README 边界描述与 closure.toml 直接矛盾（第一轮已发现，本轮确认）。
- **[P2]** 状态头失真 4 份（"架构定稿"实已死、"待实现"早已实现）；jvm-attributes-checklist.md 整份过期（引用已删的 classfile.py）；java-rust-translation-reference.md（1019 行活跃权威）放错 plans/；无归档机制。
- **[P3]** reports/ 32 份无索引；tasks.md 定位漂移（50+ 行已完成叙事）；取代关系单向指针；plans 混入 .txt 数据文件。

### 4.8 失败治理与合入流程（3/5）

知识沉淀 4.5 分（diagnose 字段精确到文件/命令可照做；known_failures 判定器口径正确；doc-only 闸门有机器防呆）；流程执行 2 分。

**流程现状（修正第一轮）**：设计态 merge_daemon（enqueue→spot→机器判定→闸门→回滚→finish）2026-10-07 起停用；现行合批流程 = 子代理自选验证清单 → 协调者逐分支 merge → 一次全量单测 + 一次抽查 → 协调者叙事判定放行。最近 12 次 batch 合并中 8 次（67%）带叙事性例外。

发现：
- **[P0] 机器闸门停用成死代码**——546 行状态机的防呆（闸门失败自动 reset、tag 复用拦截）在现流程不存在；queue/日志均在 gitignore 本机目录，放行过程不可审计。需决策：恢复 or 删除。
- **[P0]「main 既有」无机器基线（实测窗口）**——order 回归 10-10 13:12 引入 → 跨 5 次合入 → 22:40 因修复在途偶然转绿，滞留 8.5h；若误判且无修复在途可无限期滞留。最近 12 次合并 4 次含"main 既有"、4 次含"先合入后验证放行"。
- **[P1] 抽查三因素叠加**——自选名单（0.9%–3.2% 抽样）+ spot 基线滞后（1010q 的 merge-base 早于含修复的 1010o）+ 删已知条目（删除时无机器复验），实证两个危险组合。
- **[P1] 值守无消费者闭环**——patrol 只打印不告警、无心跳文件、协调者单点（合入停摆无检测）；配置/结果全在本机（bus factor 集中一台 Mac）。
- **[P1] pattern 库噪声化**——宽匹配（单个 "NPE"/"timeout"/"error[E0283]" 子串）命中已修复条目输出误导 hint；只增不删 + 19 条 pending 靠记忆收尾；已有两次人工事故（重复 id 让整库 hint 静默失效）。
- **[P2]** gate 自报无 diff 推导；A/B 单测命令在协调者 /tmp；死代码与现行双远端策略不一致；**[P3]** deferred 无复查钩子（确认第一轮）。

### 4.9 unsafe 逐行 soundness 审计

**覆盖 141 处全部逐条判定：✅ ≈118 / ⚠️ 15 / ❌ 1。** 范围：java_runtime 核心 13 文件 86 处 + rava_coro 全部 41 处 + extern "C" 块/回调/zlib 绑定逐字段比对 + set_var 全仓扫描（runtime 0 处）。

- **[P1] 直接内存原子/非原子混访**（`native_memory.rs:81-120` vs `127-181`）：update 走 SeqCst CAS，read/write/fill/copy 走 memcpy——同地址并发即 Rust 级 UB（heap 数组侧反而全程原子，模式可平移）。
- **[P2] `Handoff<T>` 公开无界 Send**（`gil.rs:158`）：`unsafe impl<T> Send` 应为 `T: Send`——下游把手写 native 里的 Rc 跨线程移动即 UB，一行修复。
- **[P2] 对象头前置契约无 release 防线**（from_value/from_storage/__mark_monitor，当前调用点全部合规）；**[P2] zlib 负长度回绕**（`input_len as u32` 巨量越界读，一行 clamp）。
- **[P3]×7**：guard 安装窗口 static mut 竞争（先建快照再注册可消）；from_static_const 零检查契约；meta_codec from_utf8_unchecked 把映像受信升级为 UB 面；锁内释放纪律仅 debug 断言；prim_at 无界生命周期；`__ImageArr` Sync 外部纪律；pc_map 节内容无校验。

亮点：`__Obj` 三指针形态用严格出处手法（标记位只经 map_addr、标记指针从不解引用）；协程切换与两架构 ABI 逐槽核对一致（含 MXCSR/x87 控制字）；信号三层 async-signal-safe（const TLS + write + abort，自管道移出 Java 投递）；zlib 绑定与 zlib.h 逐字段一致；蹦床 panic 隔离完整。

### 4.10 模糊测试就绪度与引入规划（4/5）

classfile 是教科书级 fuzz 对象：纯函数入口、零全局状态、非测试代码零 unwrap/panic（rg 全量核实）；panic 面收敛为"撒谎长度预分配"与"无界递归"两类。

**新发现（已知 3 处之外）**：
- **注解互递归第 4 处**：`extras.rs:146-194` anno↔ev 无深度上限（与已知 class.rs:427-446 同构，修复须同步）。
- **描述符递归三连**：`descriptor.rs:56` 的 `[` 无界自递归，65535 个 `[` 的描述符可从 .class 送达，且 **closure/absint/fixpoint.rs:51 直接以不可信串调用**（下游触发）；`FieldType::descriptor()` 与 Drop glue 递归加重栈深。
- `archive.rs:217` zip 声明大小预分配（恶意 jar 场景）；Debug 递归帧比解析帧肥（应纳入 oracle）；排除性结论：全部 u16 计数驱动的 with_capacity 天然封顶安全。

**引入方案（已备好）**：generator/fuzz/（空 [workspace] 脱离根 workspace 防 fat-LTO 拖慢迭代）；双目标（target_classfile_parse 含 4 个 oracle + target_descriptor_mutf8 含往返不变量）；closure 级 fuzz 明确搁置（秒级冷算使覆盖率引导失效）；语料 = e2e 1110 现编译 + jmods 抽样 + 字典（魔数/常量池 tag/属性名全集）；`-malloc_limit=512M` 必开；分诊接 failure_patterns 同构字段。阻碍：4 处 DoS 必须先修（否则淹没首轮信号）、nightly 工具链缺失（10 分钟）。成本：骨架半天 + ASan 首轮清扫 1 小时 CPU。

### 4.11 跨平台可移植性（4/5）

**修正预期：这是按 Linux+macOS 双平台设计的项目。** epoll/kqueue 双后端、Linux/BSD 双文件系统 dispatcher、rava-link ELF+Mach-O 双格式（含 `__DATA,__rava_pcmap` + LC_BUILD_VERSION）、DWARF Mach-O 调试映射、refjdk.toml 四平台段、macOS 历史修复记录与引导映像报告齐备。Windows 被 `compile_error!` 明确排除（Win64 ABI 切换汇编需 ≥2 周重写，属架构性不支持）。

矩阵：generator 构建 mac ✅；rava build mac ⚠️（链路就绪、端到端待实测）；runtime 语义 mac ⚠️（**totalMemory() 恒 0 是语义错误而非缺失**，ProcessHandle 信息空）；集群远端 mac ❌（cgroup v2 + systemd-run 强制校验拒跑——设计选择，macOS 仅可作控制端）；诊断工具 mac ❌（/proc 直读）。

发现：**[P2]** `runtime_impl.rs:44-47` /proc 直读无 mac 分支（stats.rs:226 已有 proc_pid_rusage 模板可复用）；process_handle 两文件同型；集群平台矩阵未写入文档；两个作业脚本用 GNU timeout/`time -v`（closure_bench.sh/binsize.sh 已有 BSD 分支先例）。**[P3]** 链接器硬编码 cc 不尊重 CC；栈界 TLS 快路径仅 Linux（mac 走慢路径，性能差待实测）；epoll/kqueue 行为对齐只靠 golden、交叉审查缺位；golden 在 mac 的通过率待实测。Top 行动：先实测再改造（一天变确定项）；补三处 /proc 降级；挂 macOS 编译门禁防 cfg 漂移。

### 4.12 许可与合规（风险等级：高——限对外发布场景；纯内部使用为低）

**发布阻断（P0 × 2）**：
1. **零许可基础设施**——全仓库无 LICENSE/NOTICE（find 验证），19 个 Cargo.toml 与 pyproject 零 license 字段，源码无版权头。
2. **JDK 翻译链路定性未决**——参考 JDK 是 Temurin（GPLv2+CE）；产品机制把 JDK .class 机器翻译为 Rust 静态编译进用户二进制；docs 全文对此链路零讨论（仅有的许可讨论针对 junit/EPL，且已自我认定"翻译产物属衍生作品"——该先例认知加重了对 JDK 链路给出结论的必要性）。决定性问题：Classpath Exception（为 Java classpath 链接场景设计）是否覆盖"翻译为其他语言再静态编译"；GraalVM native-image 的 GPLv2+CE+商业双轨是业界先例。结论直接决定 rava 可否走闭源/商业产物模式。

**P1 × 3**：导入语料第三方来源未核实（jnc/ruva，含 `com.github.wesleyegberto.*` 包名提示第三方 GitHub 来源，528+ 例在库）；手写 runtime 与 OpenJDK 的洁净室关系无记录（是否参考过 OpenJDK 源码影响衍生定性）；产物分发的第三方声明机制缺失（MIT/Apache 义务 + mimalloc 微软版权声明）。

依赖面干净：Rust 侧全部 MIT/Apache/BSD 宽松无传染源；Python paramiko LGPL 2.1 仅限不入产物的集群工具链；CLDR/货币数据经 JDK 分发物取得（并入 #2 一并裁定）；Temurin 商标事实性使用合规。Top 行动：法务专项；补齐许可层 + cargo-deny；语料溯源清洗（PROVENANCE + 不可确认者替换）。

## 五、附录

- 两轮合计：19 个领域子代理、~120 条带证据发现（P0 × 5、P1 × 27、P2/P3 其余）。
- 第二轮全部为静态审查；"实测"数字引自仓库内文档（emitter-performance、rustc-memory-and-crate-split 等）。
- 生成产物质量审查受 build/ 不在工作区限制，方法体形态由测试断言反推（断言即生成文本逐字匹配，置信度高）；建议后续在生成一次真实 scratch 后复核量化画像。
- 许可审查基于常见许可证知识 + 仓库证据，未联网；所有"需法律意见"项已把事实面整理成可直接提交的问题清单。
