# 注解 / 泛型签名解析链出闭包（U13 后续，2026-10-08，分支 `annot-sig`）

起点 a88d7075（batch-1011）。前序量化见 `2026-10-05-boot-image-evaluator.md` §5.6.10（当时结论「不实施」，其中「open(Comparable) 回退在开放世界下不可删」一条本节实测推翻）。

## 一、目标与口径

- 链：`sun/reflect/generics/**`（签名解析、树、反射类型对象）与 `sun/reflect/annotation/**`（`AnnotationParser` 等）。
- 目标：HelloWorld / CollectorsDemo / DeepCopy 中链类数为 0；真正运行期反射读注解的程序（TestAnnoReflect 等）仍正确。
- 计数口径：`rava closure` 导出的 classes / methods（`closure_composition_job.sh`），产物在 `cluster_results/job/<tag>/01/build/ccomp/`。

## 二、基线（a88d7075，作业 `asig-diag-a88d7075` / `asig-base-a88d7075`）

| 例 | 类 / 方法 | generics 类 / 方法 | annotation 类 / 方法 |
|---|---:|---:|---:|
| HelloWorld | 3304 / 18999 | 47 / 211 | 10 / 96 |
| DeepCopy | 3553 / 21573 | 47 / 211 | 11 / 99 |

## 三、可达路径与终态处理

### P2：泛型签名树（`sun/reflect/generics` 47 类）

`--why` 首达链：`HashMap` / `ConcurrentHashMap` 树化桶的键比较类查询（`comparableClassFor@21`）→ `Class.getGenericInterfaces`（`[concrete] entries` 具体求值入口）→ `Class.getGenericInfo` → `ClassRepository.make` / `getSuperInterfaces` → `SignatureParser` 与整棵签名树。

§5.6.10 把它记为 open(Comparable) 回退，实测不是开放世界：接收者全是**已知镜像**（全部可比较键类型），实测三层原因叠加：

| 层 | 现象（hello，`--flows @concrete`） | 终态处理 | 提交 |
|---|---|---|---|
| ① 接收者上限 | 两个调用点各报「回退：接收者超过 64 个」（实际 200+ 个镜像），整点接抽象调用边 | 只按接收者枚举（实例入口、无其他形参）时上限 `RECV_LIMIT = 4096`：各接收者独立求值、按 (入口, 镜像) 记忆，代价线性；带形参的笛卡尔积仍 64 | 0864913d |
| ② 非映像缓存写入 | 回退解除后 109 个接收者「并：写镜像 X 的非映像缓存字段 Class.reflectionData」（无泛型签名的类走 `getInterfaces(true)` 写 reflectionData），按冷 / 热之并入闭包，冷路径带签名解析 | **温求值**：冷求值兼写映像缓存（`image_memo_fields`）与非映像缓存时，只撤销后者再求值一次；映像缓存照常物化，按温 / 热之并入闭包（运行期首次执行即温路径：映像缓存已在、非映像缓存为空） | 2b85e1e3 |
| ③ 驻留串引用 | 温求值后剩 2 个接收者（`ZonedDateTime` / `LocalDateTime`）「引用映像对象 String（非不变静态字段所持）」 | 片段值新增 `FVal::Str`：映像中的驻留字符串按内容引用，由构建期求值器驻留并追加（驻留串身份即内容） | 51c0751d |

另：诊断改为未物化组合全部列出（原只列前 64 组，②的原因被截断不可见，6c1f518f）。

①②单独落地时闭包类数不变（3304 / 18999），因为③与 P1 仍各自把签名树拉进来；三层一起才断开 P2（实测见第四节）。

### P1：注解解析（`sun/reflect/annotation`）

唯一来路：`logRuntimeExit` → `LoggerFinderLoader.service` → `ServiceLoader$ProviderImpl.invokeFactoryMethod@20` → `Method.invoke` → `isCallerSensitive` → `Reflection.isCallerSensitive` → `isAnnotationPresent(CallerSensitive)` → `AnnotationParser.parseAnnotations`。`Method` 由 `findStaticProviderMethod` 经 `JavaLangAccess.getDeclaredPublicMethods(clazz, "provider")` 取得、存入 `ProviderImpl.factoryMethod` 字段，不满足 reflect-direct「同方法内常量名查找」条件，回退原入口。

终态处理：查找结果标记化（反射对象携带所指方法，经字段 / 列表 / 复制保持），invoke 点接收者全是非 CS 标记时直连。**该机制正由 `reflect-marker` 分支实现**（3cb63099 / 7c8128b0 / 75bead2f，查找入口含 `getDeclaredPublicMethods`，其实测 hello 3324→3285），本分支不重复实现。

归因：切 `invokeFactoryMethod@20`（切除集 `factinv`）或全部已知 invoke 点（`minvoke`）闭包都不变——`Method.invoke` 还经 `NTLMAuthenticationProxy.<clinit>` → `supportsTransparentAuth@8` 等点到达 CS 判定。全部 invoke 入口经直连 / 标记后的上界用切除集 `csanno`（切 `Reflection.isCallerSensitive@18` 的 `isAnnotationPresent` 派发）量化。

## 四、实测

类 / 方法数取 `--stop-after closure` 的 flows 产物；gen = `sun/reflect/generics/*` 类数，ann = `sun/reflect/annotation/*` 类数（括号内为方法数）。

| 头 | 作业 | hello | collectors | deepcopy | hello gen / ann |
|---|---|---|---|---|---|
| a88d7075（基线） | — | 3304 / 18999 | 3304 / 18995 | 3553 / 21573 | 47 (211) / 10 (96) |
| 0864913d ① | asig-p2-3d92fa35 | 3304 / 18999 | 3304 / 18995 | 3553 / 21573 | 47 / 10 |
| 2b85e1e3 ①② | asig-warm-2b85e1e3 | 3304 / 18999 | 3304 / 18995 | 3553 / 21573 | 47 / 10 |
| 51c0751d ①②③ | asig-str-51c0751d | 3304 / 18997 | 3304 / 18993 | 3553 / 21571 | 47 (209) / 10 (96) |

`comparableClassFor@21` 的未物化接收者（hello）：①后 109（全部「写非映像缓存 reflectionData」）→ ②后 2（驻留串引用）→ ③后 **0**（HashMap / ConcurrentHashMap 两点全部组合「⇒映像」，约 200 个温求值）。回退（接收者超限）①后 0。

类数在①②后不变的原因：签名树另经 P1 的 `AnnotationParser.parseSig` 可达（`--why` 首达链换成注解链），所以 P2 单独断开不减类，**gen = 0 须 P1 同时断开**。

③后 **P2 已完全断开**：`--why sun/reflect/generics/parser/SignatureParser` 的首达链只剩 P1（`ServiceLoader$ProviderImpl.invokeFactoryMethod@20` → `Method.invoke` → `isCallerSensitive` → `isAnnotationPresent` → `AnnotationParser.parseAnnotation2` → `parseSig`）。`ClassRepository` 仍由 `Class.getGenericInfo` 的返回类型签名带入（热路径读映像缓存，类型存在、方法体不可达），属签名边，不拉签名树。方法数 −2（签名解析出 P2 路径的冷部分）。

首达链上，47 个 generics 类都经 P1 到达。

**P1 上界（切除集 `csanno`，作业 `asig-cs-0bac6226`，头 0bac6226）**：

| 程序 | 类 / 方法 | gen | ann | jla |
|---|---|---|---|---|
| hello | 3245 / 18647（−59 / −350） | 28 (92) | 0 | 1 |
| deepcopy | 3499 / 21246（−54 / −325） | 29 (102) | 0 | 1 |

P1 全断后 ann 归零，gen 剩 28 / 29 类。剩余首达链是 `ConcurrentHashMap.comparableClassFor@21`（concrete）→ `SignatureParser.<init>`，方法含 `parseClassSig` / `parseSuperInterfaces` / `ParameterizedTypeImpl.make` 等，即**确有某组合的入闭包轨迹含真正的类签名解析**。但 asig-str 的诊断里两点组合全部标「⇒映像」，说明这是被 P1 遮住的第四层原因，P2 并没有完全断开。待查方向（见续作 1'）：
- ⇒映像组合的温轨迹：温求值前只撤销非映像缓存写入，`ClassRepository` 的惰性字段（`superInterfaces`）若在撤销集合里，温轨迹会重解析。
- 首个组合冷求值中的类初始化轨迹：例如 `ClassRepository.<clinit>` 的 `NONE`（`make("Ljava/lang/Object;", null)` 会走 `parseClassSig` / `parseSuperInterfaces`），它是否作为初始化轨迹并入。另一种可能：映像里有 `ClassRepository` / `ParameterizedTypeImpl` 实例（物化的 genericInfo），类型进 `image_types` 后要求运行期初始化该类。这种情况的终态是映像预初始化这些类的静态状态（`NONE` 入映像），不走运行期 `<clinit>`。
- 闭包单调：不动点早期轮次接收者尚未齐全时，若某组合失败触发了整点回退（抽象调用边），后续轮次即使全部物化也撤不回。要核对 `concrete_fallback` 是否在任一轮被调用过；诊断只打最后一轮的结果。

### 单测（作业 `asig-ut-68cb2dfc`，代码同头 a57d4801）

已知失败 3 项（container_elements_per_object / known_gate_ranks_first / param_string_constants_fold_switch），**新增失败 1 项：`reflect_new_array_element_precision`**（closure_cli.rs:150，`seeds_agree("62_reflection/TestModuleLayerDefine.java")`，种子 0 / 1 / 2 的闭包集合不同）。`-q` 下断言消息没有留存。疑似来自本分支：RECV_LIMIT 让更多组合进入具体求值，温求值和驻留串查表也可能引入顺序依赖。**合入前必须修**（续作 0）。

## 五、失败 / 否定路线

- §5.6.10 的「annsig（只折叠 `parseSig`）」：健全收益 0 类，不实施（结论不变）。
- 以手写取代签名解析：性能替换，不满足手写准入。

- 切 `invokeFactoryMethod@20`（`factinv`）/ 全部已知 invoke 点（`minvoke`）：闭包不变，见 P1 归因。P1 不能靠逐个 invoke 点处理，必须标记化 invoke（reflect-marker）。

## 六、续作入口

1. **合 reflect-marker 后联测**：本分支 ①②③（P2）+ reflect-marker（P1 invoke 直连）合并后，量 hello / collectors / deepcopy 的 gen / ann；目标 0 / 0。合并后若 ann 仍非 0，用 `--why sun/reflect/annotation/AnnotationParser` 查剩余 invoke 点（`NTLMAuthenticationProxy` 等 `<clinit>` 期反射调用是否被标记覆盖）。
0. **种子确定性回归（阻塞合入）**：本地不能跑，在服务器上跑 `cargo test --release -p driver --test closure_cli reflect_new_array_element_precision -- --nocapture` 取差异集合。重点查三处：`persist::collect` / `rollback_non_image` 的撤销分区顺序；`Ex.interned` 是 HashMap，构造驻留串表时的遍历顺序；`image_memo::prepare` 追加字符串的顺序。按片段内容排序或改用 BTreeMap。
1'. **P2 第四层（优先）**：用 `--cut-file csanno.txt --flows @concrete` 跑 hello，对 `comparableClassFor@21` 的每个组合输出冷 / 温 / 热轨迹里是否含 `SignatureParser.parseClassSig`（在 concrete.rs 诊断里加一列「轨迹含签名解析的组合」，按方法 id 比对，不写类名字面量，由 `--why` 目标给出），定位后修温轨迹或初始化轨迹的归属。目标是 csanno 下 gen = 0。
2. **csanno 上界**：已量（第四节），hello −59 类，deepcopy −54 类；合 reflect-marker 后的实测应逼近该上界，若差距大则剩余 invoke 点未被标记覆盖。
3. **终态备选（P1）**：若标记化覆盖不全，`Method` 的 CS 判定结果在构建期按方法注解求值并写入映像（与 `Class.genericInfo` 同走 `image_memo_fields` 清单，类名只在 TOML），运行期 `isCallerSensitive` 只读缓存位，不再到达 `AnnotationParser`。
4. **运行期反射读注解的正确性**：链出闭包只针对不读注解的程序；`TestAnnoReflect` / `TestAnnoDeepAccess` 等真读注解的程序链仍可达（入口是用户代码的 `getAnnotation`），合入前抽查。
5. **与其他分支的重叠**：
   - reflect-marker：P1 归它；本分支不碰 `method_marks.rs`。
   - log-chain2：路径 A 去掉 hello 的 `LoggerFinderLoader.service()`，会同时去掉 `invokeFactoryMethod@20` 这一 P1 入口；不改 concrete.rs，无代码冲突。
   - fix-1010 / charset-ext：不碰 `engine/concrete*` 与 `image_memo.rs`，预期无冲突（扩展字符集类进闭包会增加 `comparableClassFor` 接收者数，在 `RECV_LIMIT` 之内）。
