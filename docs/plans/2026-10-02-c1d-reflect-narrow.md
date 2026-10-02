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
  T7 把 GROW / TRACECLS 做成正式 `--flows` 查询后删除这些探针。

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

