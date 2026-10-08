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
| 51c0751d ①②③ | asig-str-51c0751d | 见下 | | | |

`comparableClassFor@21` 的未物化接收者（hello）：①后 109（全部「写非映像缓存 reflectionData」）→ ②后 2（驻留串引用）→ ③后见下。回退（接收者超限）①后 0。

类数在①②后不变的原因：签名树另经 P1 的 `AnnotationParser.parseSig` 可达（`--why` 首达链换成注解链），所以 P2 单独断开不减类，**gen = 0 须 P1 同时断开**。

asig-str 结果：（作业进行中，回报时补）

## 五、失败 / 否定路线

- §5.6.10 的「annsig（只折叠 `parseSig`）」：健全收益 0 类，不实施（结论不变）。
- 以手写取代签名解析：性能替换，不满足手写准入。

- 切 `invokeFactoryMethod@20`（`factinv`）/ 全部已知 invoke 点（`minvoke`）：闭包不变，见 P1 归因。P1 不能靠逐个 invoke 点处理，必须标记化 invoke（reflect-marker）。

## 六、续作入口

1. **合 reflect-marker 后联测**：本分支 ①②③（P2）+ reflect-marker（P1 invoke 直连）合并后，量 hello / collectors / deepcopy 的 gen / ann；目标 0 / 0。合并后若 ann 仍非 0，用 `--why sun/reflect/annotation/AnnotationParser` 查剩余 invoke 点（`NTLMAuthenticationProxy` 等 `<clinit>` 期反射调用是否被标记覆盖）。
2. **csanno 上界**：切除集 `scripts/closure_composition_cuts/csanno.txt`（切 `Reflection.isCallerSensitive@18`）给出 P1 全断的上界；与第 1 项实测对比判断剩余缺口属 P1 还是别处。
3. **终态备选（P1）**：若标记化覆盖不全，`Method` 的 CS 判定结果在构建期按方法注解求值并写入映像（与 `Class.genericInfo` 同走 `image_memo_fields` 清单，类名只在 TOML），运行期 `isCallerSensitive` 只读缓存位，不再到达 `AnnotationParser`。
4. **运行期反射读注解的正确性**：链出闭包只针对不读注解的程序；`TestAnnoReflect` / `TestAnnoDeepAccess` 等真读注解的程序链仍可达（入口是用户代码的 `getAnnotation`），合入前抽查。
5. **与其他分支的重叠**：
   - reflect-marker：P1 归它；本分支不碰 `method_marks.rs`。
   - log-chain2：路径 A 去掉 hello 的 `LoggerFinderLoader.service()`，会同时去掉 `invokeFactoryMethod@20` 这一 P1 入口；不改 concrete.rs，无代码冲突。
   - fix-1010 / charset-ext：不碰 `engine/concrete*` 与 `image_memo.rs`，预期无冲突（扩展字符集类进闭包会增加 `comparableClassFor` 接收者数，在 `RECV_LIMIT` 之内）。
