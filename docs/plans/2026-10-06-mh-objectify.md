# 方法句柄对象化：现状、终态设计与收益上界实测（2026-10-06，分支 `mh-objectify-plan`，基于 181e7f60）

> 本文只有方案和测量，生成器代码一行未改。前情见 `2026-10-02-c1d-reflect-narrow.md` §7.4 / §8.3 / §9.4：4b、b1 两次收窄的收益都是 0，原因归到同一个前置条件，即「方法句柄对象化」。本文回答的问题是：对象化做完以后，闭包到底能收多少。

## 一、结论先行

1. **方法句柄对象化的收益上界是 0 类**。在 HelloWorld、StockTrans、DeepCopy、TestSerialDefaultSuid 四例上，把方法句柄通道的全局实参池 `RP(1)` 整个切掉，类集合逐一不变，方法只少 33 个（DeepCopy 20907 → 20874，−0.16%）。这个实验不健全，比任何健全的对象化方案都更乐观，因此它是对象化收益的严格上界。
2. **对象化 + 4b + b1 合计：−5 至 −6 类、约 −250 方法**。
   - 基线之上合计 −6 类（就是 §8.1 / §9.1 那 6 个 `Itr` / `$1` 类，−0.18%）；
   - 叠在 jar+jca+rb 之上时少 5 类、255 方法（DeepCopy 2814 → 2809）。
3. **§8.2 的根因判断在 DeepCopy 规模上不成立**。§8.2 的取证来自小例。在 DeepCopy 上，切掉 `RP(1)` 之后反射对象池 `RP(0)` 仍有 7479 类、204 个 open（基线 7498 类、218 个 open），基本没变。也就是说，反射对象池的大值集**不是**由方法句柄池灌进来的，它来自实例化集本身经 open 值和字段视图的汇合（来源见 §2.5）。
4. **判断：不值得实施**（详见下条与 §八）。上界低于派发规则的 2% 门槛，前后两次收窄（4b、b1）挂在它后面也一并失去理由。终态设计（§三）写成文档备用，**不排期**：只有当以后的大改造（引导映像求值器、§29 的三项能力）让基线大幅缩小、句柄池在新基线上重新成为主导来源时，才重测（重测方法见 §六）。
5. **最大的收益项是 §29 的三项能力，不是对象化**。jar+jca+rb 三刀合计 DeepCopy 3422 → 2814（−608 类，−17.8%），再叠对象化 + 4b + b1 到 2809。新发现的「格式串按调用点常量求值」在带序列化的全程序上只值 −10 类（3422 → 3412），因为 Formatter 数值 / 日期分支带进来的类多数另有序列化路径可达；但在不走序列化时它是 3144 类里的最大块（§七）。
6. **新目标**（§八）：DeepCopy ≤ 2810 类、StockTrans ≤ 2807、TestSerialDefaultSuid ≤ 2814、HelloWorld 468 不变，即各项能力实测上界之和；取代作废的「DeepCopy ≤1640」。

## 二、现状剖析

### 2.1 两条通道、三个节点

反射调用分两条通道，见 `generator/crates/closure/src/engine/reflect_call.rs`：

- 反射对象通道 `RC_OBJ = 0`；
- 方法句柄通道 `RC_HANDLE = 1`。

每条通道有三个节点：

| 节点 | 标签 | 作用 |
|---|---|---|
| `Node::RP(c)` | `reflect-call 实参池 <通道>` | 全局实参池：调用入口的接收者、实参都汇入这里，按池中接收者派发反射实例成员 |
| `Node::RN(c)` | `reflect-call 实参池（去冗余） <通道>` | 去冗余视图：被池中 open 涵盖的值不逐个列出，成员形参从这里接值 |
| `Node::RA(c)` | `reflect-call 实参数组 <通道>` | 按数组打包传入的实参（`invoke0` 的 `Object[]`），元素并入 `RP(c)` |

### 2.2 方法句柄通道的池怎么形成

- **`rcall_site`**：调用边 `m@off → t`，t 是反射调用入口。
  - t 是签名多态入口（清单 `method_invokers` 里的 `MethodHandle.invokeExact` / `invokeBasic`，以及 `[facts.handle_interpreters]` 的 7 个成员）：接收者（句柄本身）和全部引用实参都并入 `RP(1)`。
  - t 非字节码（手写解释器 `method_handle_ext.rs` 的 `interpret`）：t 的手写值池 `S(t, POOL)` 也并入 `RP(1)`，它包括 LambdaForm 常量、绑定值、字段 / 内存读取。
- **`rcall_bind`**：经方法句柄通道调用的成员 t：
  - 形参（不含接收者）接 `RN(1)`；
  - 返回值 `R(t)` 回流 `RP(1)`，形成环。这是为了建模「具名函数的结果是后续具名函数的实参」。
- **成员怎么进入方法句柄通道**（`reflect_writes.rs` 的 `reflective_writes`）：
  - 清单 `method_lookups` 里返回类型不是反射对象的查找，即 `Lookup.findStatic` / `findVirtual` / `findSpecial`、`resolveOrFail` / `resolveOrNull`、`MemberName.<init>(Class,String,MethodType,B)`（LambdaForm 具名函数）。凡名字是常量，都调用 `reflect_name(cls, name, RC_HANDLE)` 按名点名（不看描述符）；
  - `method_to_handle`（`MemberName.<init>(Method,Z)`，对应 `unreflect*`）把反射对象通道的成员另接到方法句柄通道。来源推不出时，反射对象通道的全部成员整体转入（`rcall_global_m2h`）。DeepCopy 上 `method_to_handle = false`，即没有发生整体转入。

### 2.3 参与的站点与成员（DeepCopy 基线，`summary.rcall` 与 `sigpoly_sites`）

| 项 | 数 |
|---|---|
| 方法句柄通道成员 / 派发目标 | 231 / 369 |
| 方法句柄通道池 | 7584 类 + 218 open |
| 反射对象通道成员 / 派发目标 | 377 / 1799 |
| 反射对象通道池 | 7498 类 + 218 open |
| 反射成员合计 | 606（其中 `java/lang/invoke` 98、`sun/invoke/util` 81） |
| hub_fallbacks（可覆写成员退回 VM 枢纽） | 42 |
| 签名多态调用点 | 253 |

签名多态调用点按来源分布：

| 来源 | 站点数 |
|---|---|
| `BoundMethodHandle$Species_*` 的 `copyWith*` / `make` | 95 |
| VarHandle 访问（ByteArrayAccess 39、CSLM 16、MD5 16、ByteArray 14） | 85 |
| `MethodHandle*FieldAccessorImpl`（反射字段访问器） | 36 |
| `ReflectiveInvoker` / `InjectedInvokerDyn` | 1 / 2 |
| 其余 | 34 |

注入点：§8.2 小例里 `RP(1)` 有 617 个直接前驱。这些前驱的组成，DeepCopy 上的 `@in` 分组见 §2.5。

### 2.4 关键实测：句柄池不决定闭包

| 实验（DeepCopy） | 方法句柄池 | 句柄目标 | 反射对象池 | hub_fallbacks | 实例化 | 类 / 方法 |
|---|---|---|---|---|---|---|
| 基线 | 7584 + 218 open | 369 | 7498 + 218 open | 42 | 2241 | 3422 / 20907 |
| mh（切 `RP(1)`） | 0 | 231 | 7479 + 204 open | 28 | 2240 | 3422 / 20874 |
| mhp（只切 `RN(1)`） | 7574 + 215 open | 369 | 7486 + 204 open | 42 | 2241 | 3422 / 20879 |

- 切掉 `RP(1)` 后，句柄目标从 369 降到 231：只剩成员本体，不再按池中接收者派发覆写实现。但这 138 个派发目标所在的类，都已经因为别的路径在闭包里。少掉的 33 个方法是：
  - `Number` 各子类的 `byteValue` / `shortValue` / `floatValue` / `intValue` 及 `Integer.signum`、`Math.scalb`（19 个，`sun/invoke/util/ValueConversions` 拆箱具名函数按池中接收者派发出来的）；
  - `DirectMethodHandle$*.checkReceiver` / `checkCast` 4 个、`MethodHandle.updateForm` 1 个；
  - `BigDecimal` / `BigInteger` / `Atomic*` / `LongAdder` 的同类拆箱方法 9 个。
- 实例化集只少 1 个类（2241 → 2240），闭包类集合不变。**方法句柄池的值本身就是已经实例化的类**：它是结果，不是原因。

### 2.5 反射对象池的真实来源（`@in` 取证，作业 mhc-07d2f691）

`rava closure --flows '@in'`（DeepCopy，切 `RP(1)` 与基线各一次）：

- **基线 `RP(1)`** 有 748 个源分组，大头是 `BoundMethodHandle$Species_*` 的 `copyWith*`（@10）、`MethodHandleImpl.makeCollector`、`Class.cast` 的返回：都是句柄组合子自身把已有句柄再放回池，值集合就是已实例化的 Species / DMH 类。
- **切 `RP(1)` 后 `RP(0)` 的直接前驱**只剩两类：
  1. `NativeAccessor.invoke` 两个重载的 P1（接收者）；
  2. 11 个 `[Ljava/lang/Object;` 数组分配点的奇 / 偶元素（键值交错数组，如 `@11021:46`、`@14930:17`）。
  基线下的前驱另多一个 `ConcurrentHashMap.put` 的 P1，差别仅此。
- **`RA(0)` 的前驱**：`NativeAccessor.invoke(…,Class)@35` 与 `NativeAccessor.invoke(O,O[])` 的 P2。
- **`@openstat`**：open(Object) 出现在 64547 个节点，引入点 25767 个，最大来源是 `Reference.get`（`WeakHashMap$Entry` 等弱引用表）。

结论：反射对象池的大值集来自 `Object[]` 容器元素与 `Reference.get` 引入的 open(Object)，与句柄池无关。要收窄它，需要的是容器元素按分配点区分（`Object[]` 元素敏感）与弱引用表的键值类型建模，不是对象化。

## 三、终态设计（备用，不排期）

本节按任务书给出的方向写成终态设计。§一已经说明它在现基线上收益为 0，所以只作为备用方案，等触发条件出现时（§六）再取用。

### 3.1 句柄伪值

- 做法与类镜像伪 id（`reflect.rs` 的 `mirror()`：`java/lang/Class#<cls>`，记在 `self.mirrors`）一致：每个**句柄产出点**分配一个伪类型 `MethodHandle#<点序号>`，记在 `self.handles: HashMap<u32, HandleVal>`。
- `HandleVal` 包含：
  - `shape`：`Direct(member)` / `Bound { target, bound }` / `Invoker { exact }` / `Field(field, kind)` / `Const` / 其他形状（见 3.3）；
  - 槽节点：句柄实参 `Node::HV(h, i)`、句柄返回 `Node::HO(h)`。现有 `HP` / `HR` 已被开放接收者枢纽占用，所以另起名字。
- 抽象粒度按**分配点**，不按成员：同一成员经两个查找点得到的是两个伪值，并且共用成员形参。这和镜像「按类」不同，原因是 `bindTo` 的绑定值要按分配点区分。
- 伪值像普通引用值一样经字段、数组、返回值流动。字段折叠、`checkcast` 照常处理：伪类型的父类型是 `MethodHandle`；`DirectMethodHandle` 等具体子类由形状决定，`instanceof` 判定按形状对应的类求值。

### 3.2 产出点（全部由清单列出，生成器不写类名）

| 产出点 | 伪值形状 | 清单 |
|---|---|---|
| `Lookup.findStatic` / `findVirtual` / `findSpecial`，常量名、所指类已知 | `Direct(按名 + 描述符解析出的成员)`。描述符取 `MethodType` 实参的常量求值，求不出时按名取全部重载 | 现有 `method_lookups` |
| `findConstructor` | `Direct(<init>)`，派发走 `<alloc>` / `<init>` 臂 | 现有 `constructor_lookups` |
| `findGetter` / `findSetter` / `findStaticGetter` / `findStaticSetter` / `findVarHandle` / `unreflectGetter` / `unreflectSetter` | `Field`。沿用 `field_handles.rs` 的标记对象，到写入口时放开 | 现有字段句柄清单 |
| `unreflect` / `unreflectSpecial` / `unreflectConstructor` | 接 §3.6 的 Method 伪 id：已知成员给 `Direct`，否则给 open | 现有 `method_to_handle` |
| ldc `CONSTANT_MethodHandle` | `Direct`。`lambda.rs` 的 `invoke_mh` 目前按声明类型 open 处理，改为产出伪值 | 字节码常量，无需清单 |
| LambdaForm 具名函数（`MemberName.<init>(Class,String,MethodType,B)`） | 不产出句柄伪值，见 3.5 | 现有 `method_lookups` |

以下情况产出 **open 句柄**，保持健全：

- 名字不是常量、所指类未知；
- 手写体返回的句柄（如 `MethodHandleImpl$BindCaller` 的手写 `bindCaller`）；
- 由 `resolvedHandle`、`MethodHandleNatives.linkMethodHandleConstant` 等 VM 回调给出的句柄；
- 清单没有列出形状的组合子返回值。

### 3.3 组合子形状清单 `[facts.handle_shapes]`（`vm_intrinsics.toml`）

- 词汇表是封闭的：键为组合子方法（`类.方法:描述符`），值为形状和各实参的角色序号。生成器只认形状名，不认类名。

| 形状 | 语义（句柄伪值 h = 组合子返回值） | 典型组合子 |
|---|---|---|
| `bind { target, value, pos }` | 绑定值接 `HV(target, pos)`，`HV(h, i)` 接 `HV(target, i 移位)` | `bindTo`、`insertArguments`（位置为常量时精确，否则按成员不分位置接） |
| `drop { target, pos }` / `permute { target, order }` | 实参按常量位置重排后接目标 | `dropArguments`、`permuteArguments` |
| `filter_args { target, filters, pos }` / `filter_ret { target, filter }` | `HO(filter) → HV(target, pos)`；`HO(target) → HV(filter, 0)` | `filterArguments`、`filterReturnValue` |
| `fold { target, combiner }` / `collect` / `spread` | 合并器结果插入目标实参；数组打包 / 展开经 `E(分配点)` | `foldArguments`、`collectArguments`、`asCollector`、`asSpreader` |
| `guard { test, target, fallback }` / `catch { target, handler }` | 实参同时接三者，结果为两个分支结果的并集 | `guardWithTest`、`catchException` |
| `invoker { exact }` | `HV(h, 0)` 的值集里每个句柄伪值 t：`HV(h, i + 1) → HV(t, i)`，`HO(t) → HO(h)` | `invoker`、`exactInvoker`、`JLIA.reflectiveInvoker` |
| `same { target }` | 透传（类型转换不改值集） | `asType`、`asVarargsCollector`、`withVarargs`、`asFixedArity` |
| `constant { value }` / `identity` | `HO(h)` 取常量值 / `HV(h, 0)` | `constant`、`identity`、`zero` |

- 组合子内部字节码（`BoundMethodHandle$Species_*` 的 `copyWith*`、`LambdaFormEditor`）照常翻译、照常分析。形状只用来决定**伪值之间的连边**，不替代字节码。
- 新组合子没有列入清单时，其结果是 open 句柄（健全）。审计在 `rava audit` 里新增一项「未列形状的组合子调用点」计数。

### 3.4 签名多态调用点按句柄分池

调用点 `invokeExact` / `invoke` / `invokeBasic`（以及 `linkTo*` 的末参 MemberName）对接收者值集逐个处理：

- 句柄伪值 h：实参接 `HV(h, i)`，调用点结果取 `HO(h)`。`Direct(m)` 的 `HV(h, i) → P(m, i)`、`R(m) → HO(h)`；实例成员按 `HV(h, 0)` 的值集派发，同字节码调用。
- open 句柄或非伪值的 `MethodHandle`：沿用现状，接全局池 `RP(1)`。这样健全性由「伪值之外一律走旧路径」保证。
- `invoke`（非 exact）调用 `asType`，按 `same` 形状透传。

`ReflectiveInvoker` 链路在终态下的走法：

1. `ReflectiveInvoker.<clinit>` 的 `findVirtual(NativeAccessor.class, "invoke", …)` 产出 `Direct(NativeAccessor.invoke(O,O[]))`，存入静态字段。
2. `invoke` 内 `JLIA.reflectiveInvoker(caller)` 产出 `invoker` 形状。
3. `invokeExact(mh, obj, args)` 按 `invoker` 形状把 `obj` / `args` 接到 `NativeAccessor.invoke(O,O[])` 的 P1 / P2。
4. 结果：P1 / P2 只得到 `NativeAccessor.invoke(O,O[],Class)` 的 P1 / P2，不再接整池。

### 3.5 LambdaForm 解释器的内部池

- `interpret`（手写，运行模型替换）解释执行 `names[]`。具名函数（`DirectMethodHandle.checkReceiver`、`ValueConversions.unbox*`、`BoundMethodHandle.arg*` 等）由 `MemberName.<init>` 按名点名。它们的实参来自同一个 LambdaForm 的前序 name，或者来自句柄的实参 / 绑定值。
- 终态把具名函数接到**解释器池** `RP(LF)`，与目标成员隔离：
  - 解释器池收集全部句柄实参与绑定值，健全；
  - 解释器池**不**再接目标成员的形参，目标成员只接 `HV`；
  - 具名函数的返回只回流解释器池，不回流 `HO`。
- 解释器手写体本身不改。改的是分析器对 `S(t, POOL)` 的接法：它连到 `RP(LF)`，不再连 `RP(1)`；`RP(1)` 只剩 open 句柄的调用点。

### 3.6 §7.4 Method 伪 id 按成员分池（对象化之上）

- `getMethod` / `getDeclaredMethod` 是常量名且所指类已知时，产出 `Method#<成员>` 伪值。伪值经字段（`ObjectStreamClass.writeObjectMethod` 等）照常流动。
- `invoke0` 调用点按 Method 实参值集里的伪值，把接收者 / 实参接到该成员自己的池 `RP(member)`；open Method 仍接 `RP(0)`。
- 访问器包装（`Method.methodAccessor` → `DirectMethodHandleAccessor.target`）在清单里声明绑定点：访问器构造时把 Method 伪值的成员绑定给访问器对象。清单没声明的访问器路径退回 `RP(0)`。

### 3.7 合规

- **手写边界**：解释器 `interpret`、`bindCaller` 属于运行模型替换（类 ②），已经登记；本方案不新增手写方法，只改分析器对已有手写体值池的接法。
- **无 JDK 字面量**：产出点、形状、访问器绑定点全部写在 `runtime/java_runtime/vm_intrinsics.toml` 的 `[facts.reflect]` / `[facts.handle_shapes]` 里，生成器 crate 只认形状名与清单键；`no_jdk_literals` 测试守护。
- **只建模用户无法扩展的类型**：`MethodHandle` 的构造器是包私有的，具体子类都在 `java.lang.invoke` 内，用户不能扩展；句柄形状是 JDK 侧事实。用户代码里的句柄值，同样只在产出点是清单列出的 JDK 方法时才是伪值。
- **档案**：伪值是档案分析内部的抽象，档案键只依赖 JDK 侧事实，和程序无关。

## 四、分步与验收（若重启）

| 步 | 内容 | 验收判据 |
|---|---|---|
| 1 | 句柄伪值与 `Direct` 产出（查找点 + ldc），签名多态站点按伪值分池，open 回退 `RP(1)` | 4 例闭包集合 ⊆ 基线；`rcall.channels[方法句柄].pool` 下降；新增小例（仓库尚无，需随第 1 步补）：`NativeAccessor.invoke` P1 / P2 只含其调用者实参；9 例抽查 e2e 全过 |
| 2 | 形状清单 `bind` / `invoker` / `same` / `constant`，解释器池隔离 | `RP(1)` 只剩 open 句柄站点的输入；`[facts.handle_shapes]` 未列组合子审计计数有报告 |
| 3 | 其余形状（filter / fold / collect / spread / guard / catch） | 未列形状组合子在 4 例上为 0 |
| 4 | §3.6 Method 伪 id 按成员分池 + 访问器绑定点 | `RP(0)` 的 open 数下降；hub_fallbacks 下降 |

## 五、集合正确性守护

- **只许减少**：每步都在服务器上对 4 例 + 验收集 27 例出 `rava closure` 集合，与基线逐一对比，**新增类 / 方法数必须为 0**（`scripts/compare_trees.sh` + `closure.json` 集合差）。
- **动态对照**：`dyn_compare` 用参考 JDK 跑类加载轨迹，闭包必须覆盖每个实际加载的类，覆盖率不得下降。
- **单测**：`generator/crates/closure` 新增小例单测，覆盖 `bindTo` / `insertArguments` / `invoker` / `asType` 四种形状下句柄实参到成员形参的精确连边，以及「未列形状 → open → 全局池」的回退路径。
- **种子序**：`closure_independent_of_hash_seed` 必须照常通过，伪值序号按产出点稳定排序分配。

## 六、实测上界

（数字见下表；作业 mhb-236ef8dc、mhc-07d2f691、mhd-29caebe3，脚本 `scripts/diag/mh_bound_job.sh`，参考 JDK 21，`rava closure` 逐例单跑。`--cut` 不健全，只量上界。基线和 nos 沿用 §8.1 / §9.1：f3f1a90f 到 236ef8dc 之间 generator / runtime 没有改动。）

### 6.1 上界表（类 / 方法）

| 实验 | 含义 | HelloWorld | StockTrans | DeepCopy | TestSerialDefaultSuid |
|---|---|---|---|---|---|
| base | 基线（§8.1） | 468 / 1806 | 3420 / 20888 | 3422 / 20907 | 3427 / 20901 |
| mh | 切 `RP(1)`：对象化上界 | 468 / 1806 | 3420 / 20855 | 3422 / 20874 | 3427 / 20868 |
| mhp | 只切 `RN(1)` | — | 3420 / 20860 | 3422 / 20879 | 3427 / 20873 |
| fmtc | Formatter 只剩 `%s` / `%n`：格式串常量求值上界 | 468 / 1806 | — | 3412 / 20738 | 3417 / 20732 |
| jar+jca+rb | §29 三项能力 | 468 / 1806 | 2812 / 18085 | 2814 / 18097 | 2819 / 18091 |
| mh+4b+b1+jar+jca+rb | 对象化 + 两次收窄 + §29 | 468 | 2807 / 17830 | 2809 / 17842 | 2814 / 17836 |
| fmtc+jar+jca+rb(+mh+4b+b1) | 再叠格式串求值 | — | — | 见 6.3 | 见 6.3 |
| nos | DeepCopy 不走序列化（§9.1） | — | — | 3144 / 17990 | — |
| nos+fmt | 且不进 Formatter | — | — | 465 / 1725 | — |
| nos+fmt+jar+jca+rb | | — | — | 465 / 1725 | — |

（StockTrans 用 `%d` / `%f` / `%b`，fmtc 对它不成立，不测。所有切除实验相对基线均无新增类，HelloWorld 在每项实验下都不变。）

### 6.2 各项的差集

- **mh（−0 类 / −33 方法）**：19 个 Number 系拆箱方法（含 `Integer.signum`、`Math.scalb`）、4 个 DMH `checkReceiver` / `checkCast`、`MethodHandle.updateForm`，另 9 个 BigDecimal / BigInteger / Atomic* / LongAdder 同类方法。rcall 统计：句柄池 0，句柄目标 369 → 231；反射对象池 7498 + 218 open → 7479 + 204 open；hub_fallbacks 42 → 28；实例化 2241 → 2240。
- **mh+4b+b1 在 jar+jca+rb 之上（−5 类 / −255 方法）**：`java/time/{MonthDay,OffsetDateTime,Year,YearMonth}$1`、`LinkedBlockingQueue$Itr`。
- **fmtc（−10 类 / −136 方法，DeepCopy 与 TSDS 相同）**：`java/time/{MonthDay,OffsetDateTime,Year,YearMonth,ZonedDateTime}$1`、`ChronoZonedDateTime$1`、`Formatter$BigDecimalLayoutForm`、`Formatter$FormatSpecifier$BigDecimalLayout`、`IllegalFormatCodePointException`、`IllegalFormatConversionException`。其中 4 个 `java/time/*$1` 与 mh+4b+b1 那组重合。
- **jar+jca+rb（−608 类）**：明细见 c1d 计划 §29。

### 6.3 待补

作业 mhd-29caebe3 第 2、3 项（`fmtc+jar+jca+rb`、`nos+fmtc`、`nos+fmtc+jar+jca+rb`、`fmtc+jar+jca+rb+mh+4b+b1`）出结果后补进 6.1。它们回答两个问题：格式串求值在 §29 三项之后还剩多少；不走序列化时它能把 3144 收到多少。

### 6.4 重测方法

`scripts/diag/mh_bound_job.sh <组件+组件> [Test...]` 在服务器上逐例跑 `rava closure --cut … --flows '@rcall'`，产物在 `build/cj/<实验>/`。将来基线大幅缩小后，先重跑 `mh` 一项：只有 mh 相对新基线少 ≥2% 类时，才重启 §四。


## 七、不走序列化时（3144 类）的构成

DeepCopy 切掉 `DeepCopy.deepCopy`（nos）后是 3144 / 17990；再切 `PrintStream.implFormat`（nos+fmt）是 465 / 1725，与 HelloWorld（468）基本相同。所以：

**3144 = 465 核心 + 2679 由 `printf` → `Formatter` 带入**。

DeepCopy 的格式串只有 `%s` 和 `%n`。按 javap，`printString` 只走 Formattable / `toString` / `print(String)`，大写标志时再走 `toUpperCaseWithLocale`。这 2679 类全是静态上**不可执行**的转换分支带进来的。按首达链离开 Formatter 的出口帧归类：

| 出口 | 类数 | 说明 | 需要的能力 |
|---|---|---|---|
| `Formatter.getDecimalFormatSymbols` | 1210 | 只有数值转换（`getZero` / `localizedMagnitude`）调用；带进 DecimalFormatSymbols → LocaleServiceProviderPool → 区域提供者、ResourceBundle、CLDR | 格式串常量求值 |
| `FormatSpecifier.printDateTime` | 494 | `%t` 日期时间转换；带进 java.time / 时区规则 | 格式串常量求值 |
| 无 Formatter 帧（经 `setJavaLangAccess` 等首达） | 311 | 首达链上没有 Formatter 帧，但只在 Formatter 分支存在时可达（nos+fmt 下消失）；多为 Formatter 分支打开的共享引导路径 | 格式串常量求值（间接） |
| `DeepCopy.main` 直接 | 208 | 同上，首达经用户 main，仍依赖 Formatter 打开的类 | 同上 |
| `System.out` | 62 | 同上 | 同上 |
| `Proxy$Dyn` | 57 | 同上 | 同上 |
| `toUpperCaseWithLocale` | 76 | 大写标志（`%S`） | 格式串常量求值 |
| `Formatter.<clinit>` | 45 | Formatter 自身静态初始化 | 真实可达（只要用 printf 就要） |
| 其余零散 | 216 | | 多数同上 |

（首达链是近似归类：一个类只记它的第一条到达路径；表中 1210 + 494 + … 之和即 2679。）

这 2679 类里有 **597 类与 §29 三条路线的切除重合**：即使格式串求值做不到，§29 的三项能力也能单独去掉这一部分。

**关键反差**：在带序列化的全程序上，fmtc 只去掉 10 类（§6.2）。原因是 Formatter 数值 / 日期分支带进的区域提供者、ResourceBundle、java.time 等类，另有经序列化（`ObjectStreamClass` 的 SUID 计算、`ObjectInputFilter$Config` → `System.getLogger` → `SimpleConsoleLogger.format` → `ZonedDateTime.now` 等）与 §29 三条路线的独立到达路径。TSDS 不调用 printf，也经 `Preconditions.outOfBoundsMessage` → `String.format`（格式串全是 `%s`）进入 Formatter。

**按收益排序的能力**：

1. **§29 三项能力**（应用类路径 jar 分支折叠、JCA 提供者逐个装载的建模、ResourceBundle 未知 Class 服务查找的建模）：全程序 −608 类（−17.8%），对所有带序列化 / 反射的例都成立。
2. **格式串按调用点常量求值**（具体求值器 + 调用点上下文：`String.format` / `printf` / `Formatter.format` 的格式串为常量时只开放对应转换分支）：不走序列化时最多 −2679 类中的非 `<clinit>` 部分；带序列化时 −10 类。收益依赖程序形状。
3. **对象化 + 4b + b1**：−5 至 −6 类。
4. **序列化本体**：约 278 类，基本是真实可达，不作为收窄目标。


## 八、推荐的新闭包目标

原目标「DeepCopy ≤1640」已作废（c1d 计划 §九：不走序列化仍有 3144）。新目标按已测能力的上界之和定，只算有实测依据的部分：

| 例 | 基线 | 新目标（类） | 依据 |
|---|---|---|---|
| DeepCopy | 3422 | **≤ 2810** | jar+jca+rb+mh+4b+b1 = 2809 |
| StockTrans | 3420 | **≤ 2807** | 同上 = 2807 |
| TestSerialDefaultSuid | 3427 | **≤ 2814** | 同上 = 2814 |
| HelloWorld | 468 | **468（不变）** | 各项均不影响 |

- 主力是 §29 三项能力（−608），应优先排期；对象化 + 4b + b1 只贡献最后 5 类，排在最后，或者不做。
- 格式串常量求值另立一项：对用 `%s` 格式化、但不走序列化的程序，目标是使其闭包等于不进 Formatter 时的规模加 `Formatter.<clinit>`（DeepCopy 去序列化情形：3144 → ≤ 510，即 465 + Formatter 自身；以 nos+fmtc 实测为准）。它对上表四例的全程序目标只贡献约 10 类，§6.3 的组合结果出来后修订。
- 达到 2810 后，剩余规模以序列化本体（约 278 类）与 `Reference.get` / `Object[]` 容器元素引起的反射对象池大值集（§2.5）为主，届时以新基线重测 mh 一项，再决定是否重启对象化。

