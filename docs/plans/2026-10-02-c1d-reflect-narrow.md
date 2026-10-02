# C1d 反射按名查找收窄与 Class 值精度（2026-10-02）

接手 `2026-10-01-c1d-closure-bloat.md` §19.4、§19.5 与 native-gaps 移交的 getSuperclass 精度问题。分支 `c1d-pick`（起点 20ef4fdc）。

基线（20ef4fdc，`--stop-after closure`）：DeepCopy 1640 类，fold_props 42，`class_init.unknown = true`（4 个调用点各 772 类）。

## 一、阅读结论

### 1.1 序列化回调的查找点

- `ObjectStreamClass.<init>(Class cl)` 里的 `PrivilegedAction`（JDK 21 为 `ObjectStreamClass$2.run`）
  以 `getPrivateMethod(cl, "writeObject" | "readObject" | "readObjectNoData", argTypes, ret)` 与
  `getInheritableMethod(cl, "writeReplace" | "readResolve", argTypes, ret)` 查回调；两者内部是
  `Class.getDeclaredMethod(name, argTypes)`，接收者与名字都经形参流入。
- `getInheritableMethod` 沿 `defCl = defCl.getSuperclass()` 上溯。`Class.getSuperclass` 是 native、
  无返回模型，结果是 open 的 Class，于是接收者值集恒含 open——上溯得到的超类全部推不出。
- `cl` 的来源：`ObjectOutputStream.writeObject0` 的 `obj.getClass()`（写出侧）、`ObjectStreamClass` 构造器里
  `cl.getSuperclass()` 递归、`ObjectInputStream` 读描述符时 `Class.forName(名字来自流)`（读入侧，open）。

### 1.2 17f7d04b 为什么膨胀

半成品对「全部已实例化的类（含超类链）上声明了该名的方法」点名，与真正到达查找点的类无关，
共 221 个回调入链。新入链的代码里 `System.getSecurityManager()` 不再恒为 null，
`GetPropertyAction.privilegedGetProperties` 的 `doPrivileged` 分支变活，返回值与属性表常量合流后丢标签，
属性折叠全部失效（fold_props 42 → 0）。

## 二、终态方案

1. **getSuperclass 返回值按接收者镜像求超类镜像**（清单 `[facts.reflect] superclass_of_receiver`，
   引擎 `RetModel::Super`、镜像流边带变换 `MirrorOp::Super`）：类镜像 → 直接超类镜像；接口 / 根类为 null；
   数组为根类；所指未知的 Class 仍为所指未知。`getInheritableMethod` 的上溯与 `Enum.getDeclaringClass`
   由此得到确定的值集。
2. **形参透传的名字与接收者镜像相乘**：接收者值集只含真正流到查找点的类镜像（`getClass` / `getSuperclass`
   逐调用点求出），名字取各调用点在该形参上的字符串常量；所指未知的部分仍记反射缺口，不做模糊扩展。

3. **反射调用实参池**（`engine/reflect_call.rs`）：反射成员的形参与接收者不再 open。
   - 量出的根因（x1 实测：只做 1、2 时 StockTrans 1864 类、fold_props 0）：被点名的回调按「形参 open」入链，
     `ArrayList.writeObject` 的 `this` 为 open(ArrayList)，读 `elementData` 得到全部已逃逸列表的元素并集，
     这些值回流到 `writeObject0` 的 `obj.getClass()`，把 132 个类的回调点名进来；其中
     `CopyOnWriteArrayList.readObject → resetLock → Field.set` 使字段句柄写入口可达，挂起的「类推不出的字段枚举」
     生效为全部字段不折叠（`fopen_all`），`System.getSecurityManager` 不再折叠为 null，
     `privilegedGetProperties` 的 `doPrivileged` 分支变活、返回合流丢标签，fold_props 42 → 0。
   - 终态：反射成员只经反射调用入口执行——清单 `method_invokers`（按反射对象调用的 native）与签名多态调用点。
     这些调用点上声明为 Object / Object[] 的实参（签名多态按 Object；数组另取元素）汇入实参池 `Node::RP`，
     按声明类型流向反射成员的形参；不可覆写的实例方法接收者同样取自实参池；可覆写的按池中接收者逐个选中实现
     （容器对象按接收者克隆上下文，与字节码虚调用同口径），池中 open 的部分退回 VM 枢纽。
     记录分量访问器与构造器另有调用面，仍按声明类型 open。

（实施与实测见下节；2026-10-02 C1d-b 停止，后续按「四、交接」拆分接手。）

## 三、实施记录

| 步 | 提交 | 内容 | DeepCopy 类数 | fold_props | 备注 |
|---|---|---|---|---|---|
| A | fdb540d3 | 手写体数组写入判定：只读视图不计，引用元素视图 + 改写元素方法（`set` / `__update` / `with_vec`）或 Object 引用元素存取才计；宏内标识符与签名视图组合 | 未单测 | — | 单元测试 `ref_array_access` 覆盖 14 个形态 |
| B | e4279787 | Unsafe 偏移读写闸门（`hw_mem.rs` `offset_read` / `offset_write`）：按偏移读写限于偏移已可取得的实例字段，未取得的挂起，字段开放 / 枚举挂起时接上 | 未单测 | — | 堵住 memory_read 值泄漏（见 4.2 第 1 条） |

A、B 基于 28090062，均已过 `cargo build` 与生成器全部单元测试；按停止指令未做 e2e / 闭包实测，接手第一步是测 B 的类数与耗时（T0）。

**T5（分支 `c1d-t5`，提交 37a006ef，基于 2c16454b）**：方法句柄解释器的 Unsafe 读写按 DMH 所指字段接入。

- 清单 `vm_intrinsics.toml [facts.handle_interpreters] members`：`MethodHandle.invokeBasic / invokeExact / invoke / linkToStatic / linkToVirtual / linkToSpecial / linkToInterface`
  （手写体经 `method_handle_ext.rs interpret` 执行 LambdaForm）。依据：LambdaForm 里的 Unsafe 引用读写成员只来自
  DirectMethodHandle 字段访问器（`preparedFieldLambdaForm`），基址是对象或静态字段基址类镜像；数组元素句柄走
  `MethodHandleImpl$ArrayAccessor` 字节码 / VarHandle；`jdk.internal.misc.Unsafe` 不导出，用户取不到其方法句柄。
- 引擎 `hw_mem.rs`：手写调用点按调用方取读写口径 `Gate`（`Offset` / `Handle`）。`Handle` 口径下
  ① `hw_site_arrays` 的写入目标臂对 `fields` 写入成员不接数组元素（取代 `C1DR_NOMHARR`）；② `memory_read` 不读数组元素；
  ③ 实例字段按 `offset_exposed(fi, Handle)` = `field_open_under(fopen_all, deser = false)`：只经反序列化放开的字段不算
  （反序列化经 FieldReflector 自身的 Unsafe 调用点读写，不构造字段方法句柄）；挂起的偏移读写（`offset_waits` / `offset_read_waits`）
  带口径，字段按口径开放时才接上。静态字段 / 类镜像基址与普通口径相同（按名打开的静态字段）。
- `manifest.rs`：`is_handle_interpreter`（按成员引用比对，不格式化）与解析单测；生成器单元测试全过。
- 本机 emit 实测（`rava build --stop-after emit --closure-json --clean`，基线 2c16454b → T5）：

  | 用例 | 类数 | code 级 | 方法数 | fold_props | 耗时 |
  |---|---|---|---|---|---|
  | HelloWorld | 266 → 266 | 144 → 144 | 723 → 723 | 0 → 0 | 2 s → 4 s |
  | TestMethodHandleDirect | 1546 → 1546 | 1141 → 1137 | 9114 → 9098 | 53 → 53 | 15 s → 15 s |
  | TestReflectFieldMethod | 1559 → 1559 | 1155 → 1151 | 9257 → 9241 | 53 → 53 | 15 s → 13 s |
  | DeepCopy | 1823 → 1823 | 1403 → 1403 | 11102 → 11102 | 53 → 53 | 112 s → 104 s |
  | StockTrans | 1821 → 1821 | 1401 → 1401 | 11097 → 11097 | 53 → 53 | 116 s → 104 s |

  MHD / RFM 少掉的 16 个方法全是原生流 `Node$OfInt / OfLong / OfDouble / OfPrimitive` 与 `Nodes$EmptyNode*` 的
  `copyInto` / `getChild` / `asPrimitiveArray`（数组写入臂把解释器值池泛写进 `Node[]` 等数组所致；两用例不用流）。
  DC / ST 集合不变，与 4.3「只关 NOMHARR 不止血」一致——两者的膨胀在 T4 / b1 / T3 / T2。耗时差在同机噪声范围内。

### 3.1 T6 getSuperclass 返回模型（分支 `c1d-t6`，基于 2c16454b）

提交 d540606c：清单 `[facts.reflect] superclass_of_receiver = ["java/lang/Class.getSuperclass:()Ljava/lang/Class;"]`；
`defs.rs` 新增 `MirrorOp { Of, Super }` 与 `RetModel::Super`（`RetModel::mirror_op` 统一类镜像 / 超类镜像两种按接收者变换的返回）；
`mflows` 边带变换 `(dst, MirrorOp)`，`flow.rs` 推送与 `mflow` 按变换求值；`reflect.rs` 新增 `mirror_op` / `super_set`
（类镜像 → 直接超类镜像；接口 / 根类为 null 不入结果；数组 → 根类；非镜像的 Class、类文件缺失 → 所指未知的 Class；open 仍 open）；
`invoke.rs` `edge_ret` 按 `mirror_op` 走镜像流边；`methods.rs` 识别清单；`hw.rs` 超类镜像返回已建模、不经 open 返回值交出。
没有类名特判；`getSuperclass` 的手写体仍是运行期实现，只是闭包里返回值按调用点求。

实测（本机 `scripts/diag/c1d_measure.sh … emit`，基线 2c16454b → T6；耗时含机器负载波动）：

| 用例 | 类数 | 方法数 | fold_props | 耗时 s | 闭包差异 |
|---|---|---|---|---|---|
| HelloWorld | 266 → 266 | 723 → 723 | 0 → 0 | 4 → 1 | 无 |
| DeepCopy | 1823 → 1823 | 11102 → 11102 | 53 → 53 | 111 → 121 | `reflect.fields` +24（超类的 `serialVersionUID`） |
| TestSerialDefaultSuid | 1828 → 1828 | 11100 → 11100 | 53 → 53 | 119 → 121 | 同上；`reflect.gaps` 13 → 11 |
| TestSerialProxyForm | 1825 → 1825 | 11099 → 11100 | 53 → 53 | 113 → 120 | `TestSerialProxyForm$Base.readResolve` 与 `Base.<init>` 入反射面（getInheritableMethod 上溯得到）；`reflect.gaps` 11 → 9 |
| TestReflectProbe | 1553 → 1553 | 9139 → 9139 | 53 → 53 | 15 → 15 | 无 |

- 消失的两个缺口：`ObjectStreamClass.getInheritableMethod` / `getPrivateMethod` 的 `recv(open(java/lang/Class))`——
  接收者值集不再因 `getSuperclass` 掺入 open(Class)。
- `serialVersionUID` 字段 +24：`ObjectStreamClass` 沿超类链递归建描述符，超类镜像现为确定值，按名字段查找点到
  `Number` / `Enum` / 各异常基类等超类的同名字段（JVM 语义如此），类数不变。
- `class_init`：`unknown` 仍为 true（b3 处理）；`DirectMethodHandle.checkInitialized` 等 3 个调用点的已知目标 +141，
  全部是闭包内已有类的超类（超类镜像进入 Class 值池），类数不变。
- DeepCopy 基线已是 1823 类 / fold 53（非 §一 的 1640 / 42），属 T0 范畴，T6 不改变它。

### 3.2 T4 未知 Class 字段枚举收窄（分支 `c1d-t4`，基于 e6f01e2f）

提交 cf873117、93555b44：

- **getClass(open T) → 有界镜像集**（`reflect.rs` `mirror_set` / `mirror_into` / `mirror_reopen`）：open(T) 按 G 中 T 的
  已实例化子类型（数组分配点须已逃逸，同虚调用接收者的 open 展开）逐个取类镜像；`getClass` 镜像流边作用于 open(T) 时
  登记结果节点（`mirror_open`），T 的子类型此后进入 G / 数组逃逸时由 `reopen` 补入镜像——值集单调，不动点时与最终 G 一致。
  `flow.rs` 的两处镜像流推送改走 `mirror_into`（b1 文件，两行），`invoke.rs` `edge_ret` 同。
- **非字节码类镜像**（`synthetic_mirror`）：lambda 合成类与手写实现对象的 `getClass` 给共用的 `Class#<synthetic>`——
  一个 Class 类型的抽象对象（Class 实例字段按对象建模），不登记为镜像（类初始化、成员查找 / 枚举照所指未知），
  字段查找与字段枚举视为所指已知、无 Java 字段，`getSuperclass` 给根类镜像。此前它们给裸 Class，裸 Class 经
  `getSuperclass` 又注入 open(Class)。
- **字段枚举按接收者值集逐类放开**（`invoke.rs` 枚举分支）：`class_values` 取接收者 Class 值集里的镜像逐类
  `enumerate_fields(Some(c))`，值集增长时站点重跑；只有含所指未知的 Class（open(Class)、非镜像 Class）才
  `enumerate_fields(None)`，并登记 `reflect.field_enum_gaps`（summary 计数，另出 `field_writer_live` / `fields_open_all`）。
- 新边界用例 `35_io/TestSerialGetClassFields`（接口类型值的 getClass 序列化 / 字段枚举、超类字段、lambda 与数组的类镜像；
  期望为 JDK 21 输出）。

实测（本机 `rava closure`，同机先后跑，基线 e6f01e2f → T4；耗时含其它代理负载）：

| 用例 | 类数 | 方法数 | fold_props | reflect.gaps | field_enum_gaps | fopen_all | 耗时 s |
|---|---|---|---|---|---|---|---|
| HelloWorld | 266 → 266 | 723 → 723 | 0 → 0 | 0 → 0 | 0 | false | — |
| StockTrans | 1802 → 1803 | 10911 → 10924 | 53 → 53 | 13 → 14 | 2 | false | 102 → 127 |
| DeepCopy | 1804 → 1804 | 10916 → 10916 | 53 → 53 | 13 → 14 | 2 | false | 105 → 126 |
| TestReflectProbe | 1536 → 1536 | 8949 → 8949 | 53 → 53 | 2 → 1 | 0 | false | 12 → 14 |
| TestReflectFieldNames | 1536 → 1536 | 8954 → 8954 | 53 → 53 | 2 → 1 | 0 | false | 12 → 14 |
| TestSerialGetClassFields | 1807 → 1807 | 10918 → 10918 | 53 → 53 | 13 → 14 | 2 | false | 102 → 124 |

- ST +1 类 +13 方法：`CollSer`、`ImmutableCollections$ListN / List12.writeReplace`、`ClassNotFoundException.writeObject`
  与 `PutFieldImpl` 一组、`CopyOnWriteArrayList.writeObject`。原因是基线里 `getInheritableMethod` / `getPrivateMethod`
  的接收者是 `getClass` 给出的裸 Class，记缺口后**不解析**（漏掉回调，不安全）；T4 后接收者是确定镜像集，按 writeObject0
  实参值集（b1 的大汇点）里真实实例化的类解析出回调。耗时增长随这组新方法与镜像对象（contexts 58553 → 61879）而来。
- reflect.gaps：基线两条 `recv(java/lang/Class)` 消失；新增三条 `getDeclaredConstructors0 / getDeclaredMethods0 /
  getRecordComponents0 <- Class#<synthetic>`（lambda 类成员枚举，所指非字节码类，照旧记缺口不解析）。
- **未达标项：ST 仍有 2 个字段枚举缺口**（`ObjectStreamClass.getDefaultSerialFields@1`、`computeDefaultSUID@174`），
  因而字段写入者活时（Field.set*）仍会 `fopen_all`，≤ 1700 类 / < 120 s 未达到。缺口接收者里的 open(Class) 来源
  （`@openorig` / `@path`）：① `writeObject0` P1 的万能值集（7418 值 + 数百 open，经 SCC 代表
  `U ClassLoader.assertionLock` 合并）含 open(Class)，`writeClass` → `lookup(cl)` 把它带进描述符构造——b1 范畴；
  ② 原生返回 Class 的手写体（`forName0`、`getCallerClass`、`defineClass1/2`、`findLoadedClass0` 等，读侧
  `resolveClass` → `forName`）给 open(Class)，这是真正所指未知；③ `ObjectStreamClass.<init>@69` / `getClassDataLayout0`
  的 `getSuperclass` 对 open 输入给 open 输出，属①②的传递。T4 本身已不再产生 open(Class) / 裸 Class。
- COWAL 的实例化是合法的（`ServicesCatalog.addProviders@18`，经 `AccessibleObject.<clinit>` 的引导代码）；
  它进入 `writeObject0` 是经①的大汇点，不是经 open 值面实例化——由 b1 收窄。

### 3.3 b3 `class_init.unknown` 归零（分支 `c1d-b3`，基于 e6f01e2f）

**结论：b3 在 `class_init.rs` 内无法可靠归零。** 每个未知调用点的成因都在上游值集——Class 实参里掺了 open(Class)
或非镜像 Class，`class_init.rs` 只是如实读出。把这些值当作「不初始化」即不可靠（运行期该调用点可能初始化闭包内任一
带 `<clinit>` 的类），所以兜底（生成器对闭包内全部 `<clinit>` 登记钩子）在上游修好之前必须保留。本步只交付诊断，
归零依赖下表各上游修复。

提交 c718837c：`ClassInitFacts.unknown_sites`（调用点 → 成因：`open(类型)` / 非镜像值名 / `non-reference`），
closure.json `class_init.unknown_sites` 输出；成因用 `rava closure --flows '@openorig:java/lang/Class|<节点>'` /
`'@path:<节点>|open:java/lang/Class'` 追到注入点。无行为变化（单元测试全过；`rava closure` 集合与 e6f01e2f 相同）。

实测（`rava closure`，c718837c）：

| 用例 | 类数 | unknown | 已知目标 | 未知调用点（成因） |
|---|---|---|---|---|
| HelloWorld | 266 | false | 0 | — |
| TestEnumSetMap | 286 | false | 2 | — |
| TestByteArrayViewVarHandle | 1444 | true | 53 | EnumSet.getUniverse@4（非镜像 Class） |
| TestModuleLayerDefine | 1787 | true | 68 | VarHandles.makeFieldHandle@442（open）；EnumSet@4 |
| TestMethodHandleDirect | 1529 | true | 394 | DMH.checkInitialized@9、shouldBeInitialized@104（非镜像 + open）；EnumSet@4 |
| TestReflectFieldMethod | 1542 | true | 395 | 同上 + MethodHandleAccessorFactory.ensureClassInitialized@14（非镜像 + open） |
| DeepCopy | 1804 | true | 899 | 以上 5 个全有 |

成因（每条都追到了注入点）：

| # | 成因 | 涉及调用点 | 修复位置（属主） | 修法 |
|---|---|---|---|---|
| S1 | `Enum.getDeclaringClass` 对 `this` 取 `getClass()`，`this` 含 open(Enum)；`mirror_set(open)` 给非镜像 Class | EnumSet.getUniverse@4 | `reflect.rs` `mirror_set`（T4） | getClass(open T) 取 T 的已实例化子类镜像（有界），不给非镜像 Class |
| S2 | 手写体写入的字段（`hw_syntax.rs` 记 `hw_written` 并 `open_field`）进了 `fopen`，`offset_exposed` 因此把它们当作「偏移可得」；方法句柄解释器（Handle 口径）的 `Unsafe.putReference` 把解释器值池（含 open(Class)）写进 `field? MemberName.clazz` / `Field.clazz` / `Method.clazz` / `Constructor.clazz` | DMH 两点、MHAF@14 的 open 与大部分已知目标 | `hw_mem.rs` `offset_exposed`（b1）+ `facts.rs` | 「不折叠」与「偏移可得」分开：手写写入只记「不折叠」（facts.rs 另设 `fhw`，`field_open` 计入），`offset_exposed` 改用不含 `fhw` 的口径。实验（Handle 口径排除 `hw_written`）：TestMethodHandleDirect 目标 388 → 79、TestReflectFieldMethod 342 → 81，类数不变 |
| S3 | `Reflection.getCallerClass` 的返回是 open(Class)（单一注入点）→ `MethodHandles.lookup@0` → `Lookup.lookupClass` → `getFieldVarHandleCommon@237` / `getDirectMethodCommon@127` | VarHandles.makeFieldHandle@442 及 DMH 两点的 open 一部分 | `invoke.rs` `edge_ret` / 调用边登记（T4）+ `defs.rs` / `methods.rs` | 清单 `[caller_sensitive]` 驱动的返回模型：`@CallerSensitive` 方法 M 内的 `getCallerClass` 结果 = M 各调用方所在类的镜像（`callers[M]` 增长时补入）；反射调用 M 时取 `Method.invoke` 调用方 |
| S4 | 手写 `Class.for_class`（产出含 `getPrimitiveClass` 的 open(Class) 与非镜像 Class）经 `InvokerBytecodeGenerator` 值池进 `MemberName.<init>` P1 | DMH 两点的非镜像 Class | 手写模型层（`hw.rs` / `hw_syntax.rs`） | 按字面类名的 `for_class` 给镜像；原始类型镜像建模（原始类无初始化，class_init 像数组一样过滤） |
| S5 | 手写 `getDeclaredFields0` 的值池把声明类与字段类型混在一起（`getPrimitiveClass` / 数组默认值），流进 `field? Field.clazz` | MHAF@14 | 手写模型层 + S2 | 声明类取接收者镜像，字段类型不进 `clazz` |
| S6 | DeepCopy 的已知目标约 900：`MemberName.getDeclaringClass` / `Field.getDeclaringClass` 各含约 1000 类，来自序列化大值池 | 已知目标（不是 unknown） | T4 / b1（§4.2 第 2–4 条） | — |

顺序：S2、S3 修完后 DMH 两点、MHAF、VarHandles 只剩 S4 / S5 的非镜像 Class；S1 由 T4 的 `mirror_set` 收窄带走。
以上全部合入后再在 `class_init.rs` 验 `unknown = false`，并补边界 e2e（EnumSet.noneOf / VarHandle 静态字段 /
`MethodHandles.lookup().findStaticGetter` 触发初始化，期望输出取 JDK 21）。本步没有改变行为，所以没有新增 e2e。

**S5 / S4 实施（协调分工：S2 归 b1，S1 / S3 归 T4，b3 只做 S4 / S5，不改 hw_mem.rs / facts.rs / invoke.rs / reflect.rs）**

- S5 写入值是接收者：`FieldAccess.value_self`——手写体写访问器的实参是 `self`，并经保持身份的转换
  （`Clone::clone(self)`、`.clone()`、`Object::from`、引用、`?`）写入（`handwritten/syntax.rs` `is_self_value`）。
  同文件 fn 传递时（`scan.rs` `close_transitive`），只在全程以 `self` 为接收者（`self.f(…)`）的调用链上保留。
  以非 `self` 接收者调用或引用的名字记入 `BodyScan.nonself`，经它到达的 fn 里该标记取消。`hw_syntax.rs`
  `hw_value` 对 `value_self` 写入取接收者形参 `P(m,0)`，不取值池。`getDeclaredFields0` / `getDeclaredConstructors0` /
  `getRecordComponents0` 的 `__set_clazz(Clone::clone(self))` 因此只得到接收者值集，字段类型（`class_for_descriptor`）
  不再混进 `clazz`。单元测试 `scan::field_value_self`。
- S4 基本类型类：清单 `[facts.reflect] primitive_class`（`Class.getPrimitiveClass`）。返回值是一个 Class 类型的抽象对象
  `Class#<primitive>`（九个基本类型类合一，`hw.rs` `primitive_mirror`），不再是 open(Class)。它不登记为类镜像
  （所指不是字节码类：成员查找与枚举、超类、引用比较照所指未知处理）；`class_init.rs` 遇到它跳过（基本类型类
  没有初始化，JVMS §5.5）。
- S4 `for_class`（本步不做，理由如下）：runtime 里约 25 处调用，名字来自调用点各自的事实。
  - 调用者栈帧（`reflection_impl` / `security_manager_impl`）属 S3。
  - `getClass` / `getSuperclass` 的手写体已由 `mirror_of_receiver` / `superclass_of_receiver` 返回模型绕开，体内的
    `for_class` 只进值池。
  - 元数据表里的字段、参数、返回类型（`class_impl.rs` 的 `class_for_descriptor`）要按接收者镜像查表给值，属成员面
    （T4 的 reflect.rs / field_lookup）。
  - `forName` 与 species。
  - 没有一条统一的「按字面类名给镜像」规则可用。实测 `for_class` 的非镜像 Class 只经 S2 的 `field?` 视图
    （`assertionLock` 汇点）到达 class_init 调用点，S2 合入后再按剩余路径逐点建模。
- 边界用例 `TestReflectStaticFieldInit`（期望输出为 JDK 21 实测），覆盖：
  - `Field.getInt` / `Field.get` 静态字段：只初始化声明类，不初始化字段类型；
  - `unreflectGetter` / `findStaticGetter` 延迟到调用时初始化；
  - `findStaticVarHandle` 创建时初始化；
  - 基本类型类：`int.class == Integer.TYPE`、`long.class == Field.getType()`、`void.class`、`double.class.getSuperclass() == null`。

实测（`rava closure`，本步）：

| 用例 | 类数 | 目标数 | 与上一步比 |
|---|---|---|---|
| HelloWorld | 266 | 0 | 不变 |
| TestMethodHandleDirect | 1529 | 394 | 不变 |
| TestReflectFieldMethod | 1542 | 395 | 不变 |
| TestModuleLayerDefine | 1787 | 68 | 不变 |
| DeepCopy | 1804 | 899 | 不变 |
| TestReflectStaticFieldInit | 1579 | 395 | 新用例 |

未知调用点集合不变。`@path` 复查 MHAF@14 的 open(Class)：原链 `getPrimitiveClass → getDeclaredFields0 值池 →
field? Field.clazz` 已断开。剩余两条链都在 S2 / S3 之后：
- open(Class) 由调用方（S3 / T4）进入 `getDeclaredConstructors0` 的接收者 `P0`，经 `field? Constructor.clazz`
  到 `assertionLock` 汇点；
- 非镜像 Class 从 `__class_for_descriptor` 进 `assertionLock`。

两条都汇到 S2 的 `field?` 视图。DMH 两点仍是 S3（`getDirectMethodCommon@127 → MemberName.<init>`）加 S2。

**抽查 c1db3-61f33314 揭出的语义缺口：`findStaticVarHandle` 创建时不初始化声明类**（`TestReflectStaticFieldInit`
输出 `varhandle ready` 早于 `ByVarHandle init`）。

- 字节码：`VarHandles.makeFieldHandle` 的静态字段分支 `@428–442` 是
  `if (UNSAFE.shouldBeInitialized(refc)) UNSAFE.ensureClassInitialized(refc)`。
- 闭包与发射都在：调用点 @442 在 class_init 事实里，生成代码也保留了这个分支，钩子也已登记。
- 缺口在手写层：`Unsafe`（边界类）的 `shouldBeInitialized` 恒答 `false`（注释的理由是「推迟到首次使用等价」），
  分支从不进入。这个等价不成立——`<clinit>` 的副作用顺序可观察。
- 修法：运行时记录已成功完成初始化的类（`gil::clinit_exit` 登记、`clinit_done` 查询）。
  `class_needs_initialization` 对登记了初始化钩子且未完成初始化（含初始化中、曾失败）的类答 `true`，
  与 `ensureClassInitialized` 同一张钩子表。`shouldBeInitialized` 按它作答，null 抛 NPE。
- `DirectMethodHandle.shouldBeInitialized` 也因此能选带初始化屏障的形态，与 JVM 一致。

**抽查 c1db3-8f9f2da9 揭出的闭包缺口：截断体的同类被调方成为存根**（`TestMethodHandleDirect` / `TestReflectStaticFieldInit`
运行期命中 `stub: jdk/internal/misc/Unsafe.bool2byte:(Z)B`）。

- 链条：`MethodHandle.updateForm` → `Unsafe.compareAndSetBoolean`（内部边界类、无手写承载 = 截断体，发射层翻译字节码）
  → `bool2byte` / `compareAndSetByte`。截断体的被调方分析器不展开，只在另有路径时入闭包，否则发射层给存根；
  上面的 `shouldBeInitialized` 修好后 `DirectMethodHandle` 走带初始化屏障的形态，运行期才执行到这一体。
- 修法（`classes.rs` `touch_truncated_body`）：截断体内对**同一类**上同属截断的方法的调用（INVOKESTATIC / SPECIAL /
  VIRTUAL，被调方 `boundary_cut`）登记为方法，via `truncated-call`；被调方照截断语义只触及引用类、不展开，
  并递归覆盖它自己的同类截断被调方。手写承载的被调方不登记——实现恒在手写层，不会成为存根。
- 跨类的截断被调方不登记：截断按类进行（调用链进入 `[boundary]` 类即停止展开），放开跨类会沿 java/security、JCA、
  `sun/reflect/generics` 链展开（实测 TestReflectStaticFieldInit +159 类、+632 方法，HelloWorld +13 类）。这一残余随
  `[boundary]` 过渡类归零而消失，不另建模。
- 实测（`rava closure`，同类口径；对照为合入 T4 前的 61f33314）：HelloWorld 266 → 278 类（`ClassRepository.make` /
  `ClassScope.make` 等截断体的同类构造器，其体引用的 `sun/reflect/generics/tree` 类型进 Layout）；
  TestReflectStaticFieldInit 1579 → 1610、TestMethodHandleDirect 1529 → 1560（`Unsafe.bool2byte` /
  `compareAndSetByte` / `compareAndExchangeByte`、`Policy.loadPolicyProvider`、`KeyStore.<init>` 等 49 个同类截断方法）。
  新增类都是已在闭包里的截断体运行期会执行到的同类方法所引用的类型。

**抽查 c1db3-1314a487 揭出的运行时缺口：子字 CAS 经 int 字别名**（`TestMethodHandleDirect` / `TestReflectStaticFieldInit`
运行期命中 `stub: jdk/internal/misc/Unsafe.getint:(Ljava/lang/Object;J) (offset=8 无字段闭包)`）。

- 链条：`compareAndSetBoolean` → `compareAndSetByte` → `compareAndExchangeByte`（字节码）按 `offset & ~3` 取所在 int 字，
  `getIntVolatile` + `weakCompareAndSetInt` 读-比-写。旧偏移模型的实例字段 id 从 1 起逐个加 1，int 访问器只认 int 字段
  （`__unsafe_int_cell`），子字字段既无 int 视图，`offset & ~3` 也会落到相邻字段的 id 上。
- 修法（取「每字段独占 4 字节槽」，对实例 / 静态、任意对象形状都成立，相邻字段不共字）：
  - 偏移 id 恒为 4 的倍数（`reflect_dispatch::FIELD_SLOT`）：实例字段从 4 起步进 4，静态字段 `STATIC_FIELD_ID_BASE + 4·n`。
    小端下 `offset & ~3` 就是字段自身偏移、`shift = 0`，字的低位即字段值，其余位是恒为 0 的填充；
  - 新 ObjectVTable 协议 `__unsafe_word`（java_class! 宏为平铺非擦除的 int / boolean / byte / short / char 字段生成臂，
    wrapper 先问静态类 inner、再委托 vtable）：在字段共享单元上原子地做字视图读-改-写（`__PrimCell::__word_update`，
    字段值零扩展为字，写回取低位截断，boolean 取低字节非 0）；
  - Unsafe 的 int 访问器全族（get / put / CAS / compareAndExchange / getAndSet / getAndBitwise* / getAndAdd、
    acquire / opaque 变体）与子字 get / put 统一走字视图；静态字段经字段闭包按装箱值读写（只承载读与无条件写）；
  - VarHandle 手写伴生的 CAS 族（`_field_exchange`）把 boolean / byte / short / char 与 int 一同走字视图（原先这四族直接存根）。
- 不用手写 `compareAndSetBoolean` / `Byte` 绕开：JDK 子字 CAS 的字节码原样翻译执行，语义由偏移与字段模型承担。
- 闭包侧无须改动：槽独占使 `offset & ~3` 读写的仍是偏移已暴露的那个字段，偏移暴露 / 字段开放（`field_open`）按字段计，
  与按 int 读取还是按子字读取无关；基本类型读写不产生值流。
- 残余（与 int 同一现状，不属子字）：静态字段 CAS 与基本类型数组元素的 Unsafe 访问仍无共享原子单元，命中报存根。
- 边界用例 `tests/e2e/48_refs/TestSubwordFieldCas.java`（期望为 JDK 21 实测）：VarHandle 对相邻 boolean / byte /
  short / char 字段的 CAS、compareAndExchange、getAndSet、getAndAdd、weakCompareAndSet 循环，相邻字段互不影响，
  普通写入与 CAS 同一存储；未初始化类的静态方法句柄首次调用（`DirectMethodHandle.ensureInitialized` →
  `MethodHandle.updateForm` → `Unsafe.compareAndSetBoolean` 字节码路径）与同一句柄反复调用。

## 四、交接（2026-10-02，C1d-b 停止）

### 4.1 分支与提交

| 引用 | 哈希 | 内容 |
|---|---|---|
| `c1d-pick` | 见本节所在提交 | 28090062 + A（fdb540d3）+ B（e4279787）+ 本文档与 `scripts/diag/` |
| `c1d-pick-wip` | 63d90e86 | 半成品快照，**基于 e90a592d**，不合入；含名字×镜像交叉、反射调用池、getSuperclass 变换、按值集字段枚举、临时诊断探针、新用例 TestSerialCollectionFields（附录 B） |

### 4.2 根因：`ArrayList.writeObject` 反射臂缺失的完整链条

**缺失本身**（e90a592d）：`ObjectStreamClass$2.run` 以 `getPrivateMethod(cl, "writeObject", …)` 查回调，名字经形参
`name` 透传，接收者 `cl` 经形参透传。e90a592d 的 `method_lookups` 只对**调用点上的字面量**与接收者镜像相乘，
形参透传的名字不乘（为防 17f7d04b 的 221 回调膨胀而一刀切），于是 `ArrayList.writeObject` / `readObject`
等 JDK 集合回调全部不入链——StockTrans / TestSerialDefaultSuid / TestSerialProxyForm 运行时找不到分派臂。

**恢复交叉之后的连锁膨胀**（c1d-pick-wip 上逐步量出）：

1. **Class[] 污染 → 形参签名失效。** `getPrivateMethod` 的 `argTypes`（`Class[]`）本应只含类字面量镜像
   （`ObjectOutputStream.class` 等），用它约束按名查找（`ParamSig`）。实测其元素混入 open(Class)，
   `ParamSig` 推为 None（不约束），名字对全部同名方法点名。来源链：
   `ArraySpliterator.array`（任意 Object[]）→ `Unsafe.getReference`（memory_read 读**任意对象的全部引用字段**）→
   `ObjectStreamClass.FieldReflector.getObjFieldValues` → `writeObject0` → `setObjFieldValues` 的 putReference →
   方法句柄数组写入臂 → 各 `Class[]` 元素。**已由 B（偏移闸门）修复**：w16 中 `getPrivateMethod` 的 Class[] 不再 SIGNONE，
   剩余 SIGNONE 只在 `createFunction` / `findCSMethodAdapter`（真正开放的查找）。
2. **`writeObject0(obj)` 的 obj 来源**（决定 `obj.getClass()` 进而决定 `cl` 镜像集）：
   - `writeObject` 形参 P1：用户 `ArrayList@110:0`；`load()` 里 `readObject` 结果强转 `List` 得 open(List)；
     open(ClassNotFoundException)；
   - `writeFatalException`：open(IOException)；
   - `writeArray@509`：数组元素 open(Object)；
   - `defaultWriteFields@232`：`getObjFieldValues` 读字段值 → 再 `writeObject0`，构成 obj → 字段 → obj 的环；
   - `PutFieldImpl.writeFields@137`。
3. **memory_read 作用于 open 对象**：读 open(T) 的字段时值集补 open(Object)，经第 2 条的环回到 obj。
4. **getClass(open T) 给出裸 Class**（`mirror_set` 对 open 值只能给所指未知的 Class），`writeClass` 得 open(Class) →
   `ObjectStreamClass.lookup(未知)` → `getDeclaredFields` → `enumerate_fields(None)`（ENUM-ALL），挂起在 `fenum_pending`。
   首次 ENUM-ALL 出现在 `getDefaultSerialFields` / `computeDefaultSUID`。
5. **写入口变活 → fopen_all。** `CopyOnWriteArrayList.readObject → resetLock → Field.set`（WRITER-LIVE）使字段句柄写入口可达，
   挂起的 `enumerate_fields(None)` 生效为全部字段不折叠，随后 `System.getSecurityManager` 不再折叠为 null、
   `privilegedGetProperties` 的 `doPrivileged` 分支变活、fold_props 42 → 0，类数 2000+，StockTrans 超时。
6. **COWAL 镜像从哪里进入 `getPrivateMethod` 接收者：未查完。** w21 显示它最早出现在
   `ClassSpecializer.findSpecies` 与 `CopyOnWriteArrayList.addAll` 的 `getClass` 调用点——即 COWAL 实例本身由
   open 值面（不是用户代码）实例化，再经第 2 条的环流到 `writeObject0`。这是 T4 的入口问题。

### 4.3 已排除的假设

| 假设 | 结论 | 依据 |
|---|---|---|
| `Node::Array`（未知数组写入）把值灌进 Class[] | 排除 | StockTrans 中 `@array` 为空 |
| 只关方法句柄 → putReference 的数组写入臂（`C1DR_NOMHARR`）即可止血 | 排除 | 单独打开前后均 1831 类 / fold 53 |
| Class[] 里的 open(Class) 是固有的（反射查找天生不精确） | 排除 | 是 memory_read 泄漏，B 堵住后消失 |
| 膨胀来自名字×镜像交叉本身 | 排除 | 交叉只在接收者镜像集被污染（第 2、4 条）时膨胀：B 之后 NOENUM 下 ST 为 1821 类、fold 53、39 s，膨胀全部来自字段枚举放开（w17） |

### 4.4 StockTrans 实测（`rava closure`，c1d-pick-wip 各阶段）

| 运行 | 改动 | 类数 | fold_props | 耗时 | 说明 |
|---|---|---|---|---|---|
| e90a592d 前后各版 | — | — | — | 3.6–3.9 min（build --stop-after emit） | 反射臂缺失，运行失败 |
| w8 | A + NOMHARR，NOENUM | 1831 | 53 | 271 s | |
| w16 | A + B（读闸门），NOENUM | 1821 | 53 | 39 s | Class[] 不再 SIGNONE |
| w17 | A + B，字段枚举放开 | 2040（超时时） | — | > 900 s | `fopen_all = true`；ENUM-ALL 首现 `getDefaultSerialFields` / `computeDefaultSUID`；WRITER-LIVE 来自 COWAL.resetLock |

NOENUM = `C1DR_NOENUM`，把 `enumerate_fields(None)` 整个跳过（不健全，仅用于隔离第 4、5 条）。HEAD（28090062 + A + B，不含交叉）未测。

### 4.5 诊断工具

- `scripts/diag/c1d_measure.sh <tag> <closure|emit> <Test...>`：构建 rava 后逐例 `--stop-after` 实测类数 / 方法数 / fold_props / 耗时。
- `scripts/diag/c1d_closure_probe.sh <out> <Test> <秒> [--flows ...]`：不构建，直接 `rava closure`，配合 `--flows` 查询：
  `'@path:<节点>|<类>'`（值从哪条路径流入节点）、`'@openorig:<类型>|<节点>'`、`'@openinj:<类型>'`、`'@array'`、`'elem:<数组>'`。
- 两者都是重命令，须经 `heavy_lock.py`。分配序号（`alloc id`，如 `ArrayList@110:0` 之外的数字 id）随代码改动漂移，跨版本对照要按类名 / 调用点重查。
- WIP 分支专有探针（`flow.rs` / `worklist.rs` / `invoke.rs` / `method_lookup.rs`，环境变量开关）：
  - `C1DR_PROG=1`：每 50 万批打印 classes / methods / fopen / fopen_all / 挂起数 / 各工作队列长度；
  - `C1DR_GROW=<节点子串>`：打印匹配节点每次增长及其源节点；
  - `C1DR_TRACECLS=<类或分配名>`：打印每个获得该值的节点及源节点（最快定位「值从哪进来」）；
  - `C1DR_WATCH`：打印指定边的建立；`SIGNONE`：ParamSig 推为 None 的查找点；`ENUM-ALL` / `WRITER-LIVE` / `LOOKUP`：字段枚举与写入口；
  - `C1DR_NOENUM` / `C1DR_NOMHARR`：隔离开关（不健全）。
  T7 已转正（c1d-t7）：GROW → `--flows '@grow:<节点>'`、TRACECLS → `'@trace:<类或分配名>'`（另有 `'@trace:open:<类型>'`）、
  WATCH → `'@edge:<节点>'`，记录型查询分析前登记、传播中记录并实时写 stderr（语法见 `docs/environment-variables.md`）；
  `C1DR_*` 环境变量探针不移植，集成路径 0 处。PROG（进度）不属类型流查询、未移植；NOENUM / NOMHARR 为不健全隔离开关，
  由 T4 / T5 取代；SIGNONE / ENUM-ALL / WRITER-LIVE / LOOKUP 随 T2 / T4 的实现另定。

### 4.6 拆分（可并行、独立验收）

验收用例记号：ST = StockTrans，SDS = TestSerialDefaultSuid，SPF = TestSerialProxyForm，SCF = TestSerialCollectionFields（WIP 新增），
DC = DeepCopy（≤ 1640 类、fold_props ≥ 42），RP = TestReflectProbe，RFN = TestReflectFieldNames，HW = HelloWorld，m3 = lib pilot m3 golden。

| 项 | 目标（终态量化） | 改动范围 | 依赖 | 验收 | 规模 |
|---|---|---|---|---|---|
| **T0** 基线 | 测 HEAD（A+B）的 DC / ST / HW 类数、fold_props、耗时，作为各项对照 | 无代码 | — | DC ≤ 1640、fold ≥ 42；HW 不增 | 0.5 h |
| **T4** 未知 Class 字段枚举收窄 | `enumerate_fields(None)` 在序列化路径出现 0 次：getClass(open T) 给出「T 的已实例化子类镜像集」（有界镜像）而非裸 Class；字段枚举按调用点接收者 Class 值集逐类放开（WIP 已有按值集枚举）；COWAL 不再经 open 值面实例化（4.2 第 6 条） | `engine/reflect.rs`（`mirror_set`）、`engine/field_lookup.rs`（`class_values`）、`engine/invoke.rs`（`enumerate_fields`）、`engine/worklist.rs` | T0 | ST 字段枚举放开时 fopen_all = false、≤ 1700 类、< 120 s；DC；RFN；RP；HW | 2–3 d |
| **T3** 反射回调按接收者克隆上下文 | 反射成员形参 / 接收者不再 open：`method_invokers` 与签名多态调用点的实参汇入实参池 `RP`，按池中接收者逐个选中实现、容器对象按接收者克隆上下文；`ArrayList.writeObject` 的 `this` 只含真正被序列化的列表 | 新文件 `engine/reflect_call.rs`（WIP 有 221 行草稿）、`engine.rs` / `engine/new.rs`（`rcall_*` 字段）、`engine/worklist.rs`（`rcall_pending`）、`engine/invoke.rs`（`rcall_site`） | T0 | SDS、SPF、SCF、DC、m3 | 3–4 d |
| **T2** 名字×镜像交叉 + ParamSig | 形参透传的名字与接收者镜像相乘，按查找点 `Class[]` 形参签名约束；ST 中 `ArrayList.writeObject` 入链，SIGNONE 只剩真正开放的查找点 | `engine/reflect.rs`（`ParamSig`）、`engine/method_lookup.rs`（`lookup_param_sig`）、`engine/invoke.rs`（lookup 分支）、`engine.rs`（`reflect_names` 三层表）、`vm_intrinsics.toml` 注释 | T4、T3 合入后才能达标（单做会 fopen_all） | ST、SDS、SPF、SCF、DC、RP、RFN、m3、HW | 1–2 d |
| **T5** MH → putReference 精确化 | 删除 NOMHARR 式按类名开关的需要：方法句柄解释器（`DirectMethodHandle` / LambdaForm）写字段的调用点由清单 `[facts.handle_interpreters]` 声明，按 DMH 所指字段精确接入，不经数组写入臂泛写 | `engine/hw_mem.rs`、`manifest.rs`、`vm_intrinsics.toml` | T0 | ST、DC、TestMethodHandleDirect、TestReflectFieldMethod、HW | 1–2 d |
| **T6** getSuperclass 返回模型 | `Class.getSuperclass` 按接收者镜像求超类镜像（`RetModel::Super` / `MirrorOp::Super`，清单 `superclass_of_receiver`）；`getInheritableMethod` 上溯得到确定值集 | `engine/defs.rs`、`engine/reflect.rs`（`super_set`、`mflows` 带变换）、`engine/invoke.rs`、`manifest.rs`、`vm_intrinsics.toml` | — | SDS、SPF、DC、RP、HW | 1 d |
| **T7** 探针转正 | GROW / TRACECLS 做成 `--flows '@grow:<节点>'` / `'@trace:<类>'` 正式查询，WIP 的 `C1DR_*` 环境变量探针 0 处残留 | `engine/flow.rs`、`engine/diag.rs`、`engine/report.rs` | — | 单元测试；HW 闭包不变 | 0.5–1 d |
| **b1** 序列化收窄 | 未知接收者的字段视图（`U(f)`）不吸收 open 对象全部字段；`Unsafe` 读与 `setObjFieldValues` 的 obj → 字段 → obj 环只沿已知类字段闭合（4.2 第 2、3 条） | `engine/hw_mem.rs`、`engine/flow.rs` | B（已合）；与 T4 协同验证 | ST、SCF、DC、HW | 2 d |
| **b2** linkToNative / 健全 provider | 等 why2-93e0f28e 结论后定；按原计划 | 待定 | why2-93e0f28e | 待定 | — |
| **b3** `class_init.unknown` 归零 | 4 个调用点各 772 类的未知初始化 → `class_init.unknown = false` | `engine/class_init.rs`（WIP 有 15 行草稿） | T6（超类镜像） | DC、HW、TestBootLayer | 1–2 d |

**同文件冲突**（须排先后）：

- `engine/invoke.rs`：T2、T3、T4、T6；
- `engine/reflect.rs`：T2、T4、T6；
- `engine.rs` / `engine/new.rs`：T2、T3（字段声明，冲突小）；
- `engine/hw_mem.rs`：T5、b1；
- `engine/worklist.rs`：T3、T4；
- `manifest.rs` / `vm_intrinsics.toml`：T2、T5、T6（不同段落，冲突小）；
- `engine/flow.rs`：T7、b1。

建议顺序：T0 → 并行 {T6、T5、T7} → T4 → T3 → T2（T2 是 ArrayList.writeObject 臂恢复的收尾，必须最后合）；b1 与 T4 同期、b3 在 T6 后。

### 附录 B：c1d-pick-wip（63d90e86）中未提交的半成品

基于 e90a592d，与 A、B 的差异需在接手时按项挑取，不整体合并。

| 文件 | 意图 | 当前问题 |
|---|---|---|
| `engine/reflect.rs` | `ParamSig`（按 `Class[]` 元素镜像约束查找形参签名）；名字×镜像交叉；`super_set`（getSuperclass） | 交叉在镜像集被污染时膨胀（T4 前不可合） |
| `engine/method_lookup.rs` | `lookup_param_sig`：查找点 `Class[]` 实参 → 签名；`SIGNONE` 诊断 | 诊断需删 |
| `engine/invoke.rs` | lookup 分支接 ParamSig；`rcall_site`；按 `class_values` 枚举字段；`getSuperclass` 返回模型；`ENUM-ALL` / `LOOKUP` / `WRITER-LIVE` / `NOENUM` 诊断 | 四项混在一起，按 T2 / T3 / T4 / T6 拆 |
| `engine/reflect_call.rs`（新） | 反射调用实参池 RP、按接收者派发（`RcallMember`） | 克隆上下文未做完；ST 下未验证 |
| `engine.rs` / `engine/new.rs` | `rcall_*`、`offset_*`（已入 B）、`reflect_names` 三层表、`mflows` 带 `MirrorOp` | — |
| `engine/defs.rs` | `RetModel::Super`、`MirrorOp` | — |
| `engine/field_lookup.rs` | `class_values` 改为 pub(super) 供枚举用 | — |
| `engine/class_init.rs` | b3 草稿：超类镜像已知时不记未知初始化 | 依赖 T6 |
| `engine/flow.rs` | `C1DR_WATCH` / `C1DR_GROW` / `C1DR_TRACECLS` 探针（thread_local `C1DR_SRC`） | T7 转正后删 |
| `engine/hw_mem.rs` | 偏移闸门（已入 B）；`C1DR_NOMHARR`；`is_poly` 改 pub(super) | NOMHARR 由 T5 取代 |
| `engine/worklist.rs` | `offset_fields_opened`（已入 B）；`rcall_pending`；`C1DR_PROG` | PROG 由 T7 处理 |
| `engine/hw.rs` / `methods.rs` / `report.rs` / `stats.rs` / `lib.rs` | 反射调用池统计、报告字段 | 随 T3 |
| `manifest.rs` / `vm_intrinsics.toml` | `superclass_of_receiver`；enumerators 注释改为按值集放开；method_lookups 注释（形参透传名字相乘） | 随 T6 / T4 / T2 |
| `handwritten/{scan,syntax}.rs` | 数组写入判定 | 已入 A（A 另修宏内 `set` 的判定） |
| `tests/e2e/35_io/TestSerialCollectionFields.java` + expected | JDK 集合作为可序列化字段的回调覆盖（ArrayList / HashMap / LinkedList、空 / 嵌套 / null / 共享引用） | expected 合入前须用 JDK 21 实跑复核；随 T2 合入 |

