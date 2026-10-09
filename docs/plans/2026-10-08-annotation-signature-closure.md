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

### 并入集成分支后（2026-10-09，合并 b4395473 = 本分支 + ce554b84）

计数口径同上（`closure_composition_job.sh`，`--flows @concrete`）；注意此口径下集成分支 HelloWorld 为 3456 类，与 tasks.md 的 3233（另一口径）不可直接比。

| 头 | 作业 | hello | collectors | deepcopy | hello gen / ann | deepcopy gen / ann |
|---|---|---|---|---|---|---|
| ce554b84（集成分支，基线） | as-base3-ce554b84 | 3456 / 18678 | — | 3757 / 21717 | 47 (210) / 0 | 47 (210) / 0 |
| b4395473（合并） | as-merge-b4395473 | 3433 / 18542 | 3433 / 18549 | 3735 / 21592 | 28 (92) / 0 | 29 (102) / 0 |
| 9a535840（合并 + 两处定论修复） | as-settle-9a535840 | **3407 / 18453** | 3407 / 18460 | **3727 / 21500** | **4 (8)** / 0 | 22 (13) / 0 |

净收益（对集成分支）：hello −49 类 / −225 方法，deepcopy −30 类 / −217 方法。`SignatureParser` 三例均「不在闭包内」，`comparableClassFor@21` 两点全部组合「⇒映像」、无回退。

- ann = 0：P1 已由集成分支的 reflect-marker（查找结果标记化）断开，与第四节 csanno 上界一致。
- hello 剩余 gen 4 类 / 8 方法：`ClassRepository.getSuperInterfaces` 与 `ParameterizedTypeImpl` 的访问器 / `equals` / `hashCode` / `toString`——热路径读映像中的 genericInfo 后实际执行的方法，是运行期真实残余。
- deepcopy 剩余 gen 22 类只有 13 个方法：多出的 18 类是映像中 genericInfo 对象图的类型（签名树节点、`ClassScope`、`CoreReflectionFactory`，level 为 layout / alloc），属映像数据，不带代码。

#### 种子确定性（续作 0）

并入集成分支后 `reflect_new_array_element_precision` 不再失败（作业 as-seed-b4395473：TestModuleLayerDefine 种子 0 / 1 / 2 的类 / 方法 / 反射成员集合逐项相同，3523 / 19174 / 645；该单测与 `profile_union_key_and_coverage` 均通过）。原失败由集成分支 fix-1011 / batch-1013 的顺序根因修复消解（同属 §5.8.6「看到未定论状态须答 ⊥」一类）。

排查中在本分支代码上另找到两处同类顺序依赖，按终态修掉（不靠排序）：

1. **类初始化登记取决于共享 VM 的求值历史**（cbd5f9d7）：具体求值所有组合共用一个 VM，类初始化状态跨组合保留，结果只登记「本次求值触发的初始化」（inited）。先求值的组合若按热路径入闭包，其冷路径完成的初始化不登记；之后按冷 / 热之并入闭包的组合再请求该类时它已完成初始化、也不登记——该类 `<clinit>` 是否入闭包随求值次序变化。改为一律按「请求初始化」（touched）登记：请求集合只取决于本组实参的执行路径。
2. **片段可物化判定读未定论的构建期初始化结局**（9a535840）：`image_memo_prepare` 对片段所引静态字段（`FVal::Static`）只查当时的 `build_time` 集合；声明类尚未尝试构建期初始化时按「运行期初始化」答复，该组实参按冷 / 热之并入闭包，之后该类经别的路径构建期初始化也撤不回（`applied` 已登记）。改为先对这些声明类调 `image_ext` 定论（同 `image_settle_reads`）。

#### P2 第四层：三个待查假设的结论

并入后 csanno 已无必要（P1 由 reflect-marker 断开），直接在合并头上查 `--why SignatureParser`：首达链为 `ConcurrentHashMap.comparableClassFor@21`（concrete）→ `SignatureParser.<init>`，诊断里两个组合未物化：`ZonedDateTime` / `LocalDateTime`「（并：引用运行期初始化类的静态字段 sun/reflect/generics/tree/BottomSignature.singleton）」——其 genericInfo 含通配符类型实参 `Comparable<ChronoLocalDateTime<?>>`，`Wildcard` 的下界是 `BottomSignature.make()` 的单例。

| 假设 | 结论 |
|---|---|
| ① 温轨迹重解析（惰性字段被温求值撤销） | 否：定论修复后全部组合「⇒映像」，签名解析不再出现在任何入闭包轨迹里 |
| ② 初始化轨迹（`ClassRepository.<clinit>` / 映像类型要求运行期初始化） | 否（`ClassRepository` 已由集成分支 U13 列入 `[concrete.boot]` 构建期初始化）；同形问题出在 `BottomSignature`：片段引其静态单例，类的构建期初始化结局未定 |
| ③ 闭包单调（早期轮次失败后撤不回） | 是，但不在回退：两点无任何回退记录；撤不回的是「按冷 / 热之并」的 `applied` 登记，触发者是②中未定论的构建期初始化结局（修复 2） |

asig-cs（并入前、csanno 下）诊断全标「⇒映像」却仍有签名解析，推测同属修复 1 / 2 的次序依赖（入闭包的判定早于类状态定论）；并入前的头未复测，不作定论。

## 五、失败 / 否定路线

- §5.6.10 的「annsig（只折叠 `parseSig`）」：健全收益 0 类，不实施（结论不变）。
- 以手写取代签名解析：性能替换，不满足手写准入。

- 切 `invokeFactoryMethod@20`（`factinv`）/ 全部已知 invoke 点（`minvoke`）：闭包不变，见 P1 归因。P1 不能靠逐个 invoke 点处理，必须标记化 invoke（reflect-marker）。

## 六、续作入口

1. ~~合 reflect-marker 后联测~~：已量（as-settle-9a535840），ann = 0；gen 余 hello 4 类 / 8 方法（热路径真实残余）、deepcopy 22 类 / 13 方法（映像数据类型）。gen 再降须让 `comparableClassFor` 的结果本身构建期折叠（另立项，非本分支）。
0. ~~种子确定性回归~~：已消解（见「并入集成分支后」）。
1'. ~~P2 第四层~~：已完成，`SignatureParser` 出闭包（见「并入集成分支后」）。
2. **csanno 上界**：已量（第四节），hello −59 类，deepcopy −54 类；合 reflect-marker 后的实测应逼近该上界，若差距大则剩余 invoke 点未被标记覆盖。
3. **终态备选（P1）**：若标记化覆盖不全，`Method` 的 CS 判定结果在构建期按方法注解求值并写入映像（与 `Class.genericInfo` 同走 `image_memo_fields` 清单，类名只在 TOML），运行期 `isCallerSensitive` 只读缓存位，不再到达 `AnnotationParser`。
4. **运行期反射读注解的正确性**：链出闭包只针对不读注解的程序；`TestAnnoReflect` / `TestAnnoDeepAccess` 等真读注解的程序链仍可达（入口是用户代码的 `getAnnotation`），合入前抽查。
5. **与其他分支的重叠**：
   - reflect-marker：P1 归它；本分支不碰 `method_marks.rs`。
   - log-chain2：路径 A 去掉 hello 的 `LoggerFinderLoader.service()`，会同时去掉 `invokeFactoryMethod@20` 这一 P1 入口；不改 concrete.rs，无代码冲突。
   - fix-1010 / charset-ext：不碰 `engine/concrete*` 与 `image_memo.rs`，预期无冲突（扩展字符集类进闭包会增加 `comparableClassFor` 接收者数，在 `RECV_LIMIT` 之内）。
