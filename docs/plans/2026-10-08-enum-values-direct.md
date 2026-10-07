# 直连反射调用：枚举常量取得路径不再经 Method.invoke 通用反射扇出

> 分支 `enum-values-direct`（基于 batch-1007 98e733c9，已并入 boot-image-s4 17dd9b7c）。
> 起因：闭包构成报告（`docs/reports/2026-10-07-closure-composition.md`，c3a06331）的反事实切除——
> 枚举反射门切除后 CollectorsDemo 2990 → 528 类，约 2460 类经 `Method.invoke` 入链。

## 一、根因

JDK 25 `Class.getEnumConstantsShared`：

```java
final Method values = getMethod("values");          // 变长形参：iconst_0; anewarray Class
java.security.AccessController.doPrivileged(...);   // 25 已为 values.setAccessible(true)
T[] temporaryConstants = (T[])values.invoke(null);   // 反射调用门
...
catch (InvocationTargetException | NoSuchMethodException | IllegalAccessException | NullPointerException | ClassCastException ex) { return null; }
```

`Class.enumConstantDirectory`（Enum.valueOf / EnumSet / EnumMap / switch on enum 的 `valueOf` 路径）经它取常量。
boot-image-s4 e40e805c 起 `enumConstantDirectory` 回归字节码翻译（删除手写常量目录），`Enum.valueOf` 与 JDK
同样经这一反射调用——同一个门，本方案一并覆盖。

`Method.invoke` 的体：

```java
boolean callerSensitive = isCallerSensitive();     // 解析方法注解（注解解析器 + 反射工厂整链）
Class<?> caller = null;
if (!override || callerSensitive) caller = Reflection.getCallerClass();
if (!override) checkAccess(caller, clazz, Modifier.isStatic(modifiers) ? null : obj.getClass(), modifiers);
MethodAccessor ma = methodAccessor;
if (ma == null) ma = acquireMethodAccessor();      // 访问器工厂
return callerSensitive ? ma.invoke(obj, args, caller) : ma.invoke(obj, args);
```

`isCallerSensitive()` 与两路访问器调用的取舍取决于运行期的反射对象，分析器无从折叠——这些代码在运行期
确实执行，单靠分析精度去不掉。要去掉就得改写调用点：在能证明反射对象只可能是非 CS 静态目标的调用点上，
改调一个语义逐句一致、但不含 CS 判定与访问器工厂的特化入口。

## 二、设计（终态）

### 2.1 清单（事实在 `runtime/java_runtime/vm_intrinsics.toml`，生成器无类名字面量）

```toml
[facts.reflect.direct_invokers."java/lang/reflect/Method.invoke:(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;"]
helper = "java/lang/reflect/Method$Direct.invoke:(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;"
lookups = { "java/lang/Class.getMethod:(...)..." = "public", "java/lang/Class.getDeclaredMethod:(...)..." = "declared" }
```

- 键：反射调用入口（实例方法，接收者 = 反射对象）；
- `helper`：特化入口，静态方法，形参 = 反射对象 + 原入口形参，返回类型同原入口（解析时校验描述符）；
- `lookups`：按名查找入口 → 口径（`public` 沿超类链取公开成员；`declared` 只查声明类）。

解析：`generator/crates/closure/src/manifest/direct.rs`。

### 2.2 特化入口（VM 支持类，字节码生成）

`runtime/java_support/java.base/java/lang/reflect/Method$Direct.java`：

```java
@CallerSensitive
static Object invoke(Method m, Object obj, Object[] args) throws IllegalAccessException, InvocationTargetException {
    if (!m.override) {
        Class<?> caller = Reflection.getCallerClass();
        int modifiers = m.getModifiers();
        m.checkAccess(caller, m.getDeclaringClass(), Modifier.isStatic(modifiers) ? null : obj.getClass(), modifiers);
    }
    return invoke0(m, obj, args);
}
private static native Object invoke0(Method m, Object obj, Object[] args) throws InvocationTargetException;
```

与 `Method.invoke` 在「目标非 CS、访问器为本地访问器（`jdk.reflect.useNativeAccessorOnly`）」时逐句一致：
`isCallerSensitive` 恒假 → 只在未 override 时取调用者；本地访问器的非 CS 调用即 `invoke0`。`@CallerSensitive`
使生成器在改写后的调用点压入调用方所在类（`caller_sensitive_decl`），`getCallerClass` 取到的即原调用者。

`invoke0` 是 native（准入类别 ①），共置手写 `runtime/java_runtime/src/java/lang/reflect/method_direct_impl.rs`：
与 `DirectMethodHandleAccessor$NativeAccessor.invoke0` 同体（按 Method 声明键走 L3 反射分派、目标异常包装为
InvocationTargetException）。它不登记 `method_invokers`（不开通用反射实参池通道）；`[facts.array_writes]` 登记为空。

### 2.3 分析器（`generator/crates/closure/src/engine/reflect_direct.rs`）

在字节码调用点（`invoke.rs::invoke` 入口，`prop_key_site` 之后）判定：

1. 指令是 invokevirtual，被调成员是清单所列反射调用入口；
2. 接收者值唯一来自本方法内一个查找调用点（`site_of`），该点事件是清单 `lookups` 所列入口；
3. 查找实参：名字为字符串常量；形参类型数组为 null 或常量长度 0 的数组（`Obj::Len(0)`）；其它形参形状不收；
4. 查找类集：Class 实参 / Class 接收者值集中字节码类镜像所指的类（类字面量直接取）；基本类型类镜像没有方法，
   查找恒抛异常，不产生目标。值集里所指未知的部分（open、非镜像的 Class 对象、隐藏 / 代理类镜像）按**反射缺口**
   口径处理，不使调用点回退：查找点自身经 `method_lookup.rs::recv_mirrors` 记缺口，分析器不做模糊扩展——缺口类的
   成员本就不经该查找点进入调用链，原入口与特化入口在缺口上同样落到调用链外（两者的 native 共用
   `reflect_dispatch::native_invoke`，非 CS 方法的实参检查、空接收者 NPE、装箱逐句一致）；
5. 按口径逐类解析无参方法：`public` 沿超类链取公开者（数组类即根类公开成员；类链任一超接口有同名无参
   实例方法即不满足——它参与 getMethod 的选择）；`declared` 只取声明类。同类多个同名无参（桥接）即不满足；
   解析不到 = 查找抛 NoSuchMethodException，不产生目标；
6. 解析出的目标必须是静态、非 `@CallerSensitive`、返回引用或 void。

满足时：

- 调用点 → 特化入口（实参按位置原样，按 `@CallerSensitive` 声明压栈调用方；结果节点不接特化入口的返回值）；
- 调用点 → 每个目标（无实参；结果 = 目标返回值，流入调用点结果 `S(m, off)`）；目标所属类初始化，
  目标入反射分派面 `reflect_members`（native invoke0 的分派表由此生成）；
- 不接原入口——`Method.invoke` 体内的 CS 判定、注解解析、访问器工厂链不经本调用点入链。

单调性：条件随值集增长可由真变假（查找类集新增的类上解析出非静态 / CS / 基本类型返回的目标等）。一旦不满足，调用点记入
`rdirect_fallback`，此后恒按原入口接边；已接的直连边保留（过近似，安全）。查找调用点所在方法登记为跨偏移
读者（重分析时重跑）。

### 2.4 导出与档案并

- `folds[].direct_calls = [{"pc", "target"}]`（`engine/report.rs::direct_calls`）：同一方法的全部克隆中，
  每个在该点可达的克隆都按直连处理（从未回退）且特化入口相同时才导出；与 null_recv / noreturn / consts 不相交。
  统计：`summary.fold_direct_calls`。
- 档案并（`profile/fold_join.rs`）：`Direct(h) ⊔ Direct(h) = Direct(h)`；`Direct(h) ⊔ NullRecv = Direct(h)`
  （NullRecv 的入口不给原入口入链，并后按原入口调用会撞到闭包外存根；接收者为 null 时特化入口同样抛 NPE）；
  其余组合 = Normal（Normal / NoReturn / Const 的入口照常给原入口入链）；该点在某入口不可达时该入口不参与。

### 2.5 消费（`generator/crates/input`）

- `MethodFold.direct_calls: BTreeMap<u32, MemberRef>`（`facts.rs`：进程内 `from_closure` 与 JSON `parse_fold`）；
- 规范化（`norm.rs`）校验：pc 是活的 invokevirtual、不与其它折叠重叠、特化入口描述符 = `(L原属主;` + 原形参 + 原返回；
- 改写：该指令替换为 `invokestatic 特化入口`（偏移沿用，栈形不变，异常表不动）。发射层只见规范化后的
  静态调用，按普通静态调用翻译（CS 声明照常压入调用方类）。

## 三、语义核对

| 情形 | 原 `Method.invoke` | 改写后 |
|---|---|---|
| 反射对象为 null | `NullPointerException`（invokevirtual） | `m.override` 读字段抛 NPE，同被 catch |
| 未 override | `checkAccess(caller, …)` | 同调用、同调用者 |
| 目标非 CS | `ma.invoke(obj, args)` → 本地访问器 `invoke0` | 直接 `invoke0`，同分派、同异常包装 |
| 目标抛异常 | `InvocationTargetException` | 同 |
| 实参个数不符 | `IllegalArgumentException`（访问器） | `native_invoke` 同口径（`take_bad_arg`） |

不满足条件的调用点（目标可能 CS、反射对象来源不明）一律不改写，语义不变。

## 四、单元测试

- `manifest/direct.rs`：清单解析、特化入口描述符与口径校验、缺节为空；
- `engine/reflect_direct.rs`：空数组判定；
- `profile/fold_join.rs` `lattice` + `profile/tests.rs::fold_join_direct_calls`：直连格与档案并；
- `input/src/unit_tests.rs`：`direct_call_becomes_invokestatic`（改写与校验）、`parse_fold_reads_direct_calls`。

## 五、验收

| 用例 | 判据 |
|---|---|
| CollectorsDemo | 本方案：getEnumConstantsShared 导出直连、类数不增；终态（§八 其余入口全部不经 `Method.invoke` 体）降到切除实验的 ~528（另加特化入口与 `checkAccess` 链） |
| HelloWorld / DeepCopy / TestJcaSasl | 闭包类数不增 |
| TestEnumBasic / TestEnumAdvanced / SwitchExpressions | 闭包不增；e2e 输出不变（Enum.valueOf 经 enumConstantDirectory 走本路径） |
| 全部 e2e | 输出不变 |

实测见 §六（作业 enumval-622cfbaf：开 / 关清单节对照；enumvaldiag2-30c63bc0：回退诊断）。

## 六、实测

### 6.1 首轮（作业 enumval-622cfbaf，kr2，ref 622cfbaf；同一二进制，清单开 / 关 `direct_invokers` 节）

| 用例 | 类（开 / 关） | 方法（开 / 关） |
|---|---|---|
| CollectorsDemo | 3011 / 3010 | 17893 / 17891 |
| HelloWorld | 3011 / 3010 | 17861 / 17859 |
| DeepCopy | 3214 / 3213 | 20165 / 20163 |
| TestJcaSasl | 3062 / 3061 | — |
| TestEnumBasic | 3012 / 3011 | — |
| TestEnumAdvanced | 3019 / 3018 | — |
| SwitchExpressions | 3014 / 3013 | — |

开时特化入口（`Method$Direct.invoke` / `invoke0`）入闭包，但 `fold_direct_calls = 0`、`Method.invoke` 仍在。合入
boot-image-s4 后 HelloWorld 也带着同一 ~3000 类团（`Enum.valueOf` 经 `enumConstantDirectory` 入链）。

### 6.2 回退诊断（作业 enumvaldiag2-30c63bc0，kr2，临时分支 enum-values-direct-diag 30c63bc0 的 stderr 打点）

HelloWorld 与 CollectorsDemo 结果相同，`Method.invoke` 的调用点共 5 个回退：

| 调用点 | 回退原因 |
|---|---|
| `Class.getEnumConstantsShared` @49 | 查找类集含所指未知值：接收者值集除 17 个类镜像（各枚举类与 Boolean / Integer 等，isEnum 判定不收窄）外，另有一个**非镜像的 Class 对象**（Class 类型的抽象对象，不指向字节码类）。**假设成立**（原假设为 open，实为非镜像 Class 对象，同属所指未知） |
| `BasicImageReader$2.run` @37 | 查找 `FileChannelImpl.getMethod("setUninterruptible")` 解析出**实例方法**，不可直连 |
| `ServiceLoader$ProviderImpl.invokeFactoryMethod` @20 | 反射对象取自字段（`factoryMethod`），不来自本方法查找点 |
| `AnnotationInvocationHandler.equalsImpl` @121 | 反射对象取自成员方法数组，不来自本方法查找点 |
| `HostLocaleProviderAdapter.findInstalledProvider` @39 | 查找名字不是常量 |

`Method.invoke` 的入链 via 是 `BasicImageReader$2.run`（`Random.<clinit>` → `getDeclaredField` →
`getReflectionFactory` → `doPrivileged` 按 PrivilegedAction 全部实现派发）；`Executable.declaredAnnotations`（注解解析团的入口）
经 `Method.invoke` → `isCallerSensitive` 入链。

### 6.3 修正（90b430b1）

查找类集改按反射缺口口径（§2.3 第 4 条）：所指未知的值不再使调用点回退。getEnumConstantsShared @49 由此直连
（17 个类镜像上解析：枚举类得 `values()`，其余无公开无参 `values` 抛 NoSuchMethodException 不产生目标）。

### 6.4 修正后核对（作业 enumvalfix-c223f31b，kr2，ref c223f31b）

| 用例 | 类 | 方法 | `fold_direct_calls` | `Method.invoke` 在闭包 |
|---|---|---|---|---|
| HelloWorld | 3011 | 17861 | 1（getEnumConstantsShared pc 49 → `Method$Direct.invoke`） | 是 |
| CollectorsDemo | 3011 | 17893 | 1（同上） | 是 |

**结论**：本修正只消掉枚举取常量这一个入口；`Method.invoke` 仍经其余 4 个调用点入链，闭包规模不会单因本修正
降到 ~528。降到切除实验的规模需要这 4 个入口同样不经 `Method.invoke` 体，见 §八。

## 七、待验证清单

1. **合批 e2e**：CollectorsDemo、HelloWorld、DeepCopy、TestJcaSasl、TestEnumBasic、TestEnumAdvanced、
   SwitchExpressions 输出与 JDK 一致；含 EnumSet / EnumMap / `Enum.valueOf` / switch on enum 的用例抽查。
2. **VM 支持类编译**：`Method$Direct.java` 经 `image.rs` 的 `javac --patch-module java.base` 编入（本机已用 javac 21 /
   graalvm 25 手动编过；访问 `AccessibleObject.override` 与包私有 `checkAccess`，同包合法）。
3. **手写 invoke0 编译**：`method_direct_impl.rs` 与生成的 `Method_Direct` 声明层对齐（本机不能单独 check java_runtime）。
4. **反射分派表**：直连目标经 `reflect_members` 入 L3 分派表，`native_invoke` 能按声明键找到 `values()`。
5. **getEnumConstantsShared 直连生效**：修正后闭包导出的 folds 中 `Class.getEnumConstantsShared` 带 `direct_calls`
   （pc 49 → `Method$Direct.invoke`），`fold_direct_calls ≥ 1`；`Method.invoke` 仍在闭包（由 §6.2 其余 4 个调用点
   入链），类数与 §6.1「开」持平或略降。
8. **缺口口径的运行期等价**：getEnumConstantsShared 的非镜像 Class 接收者在运行期若真到达（来源待查，见 §八），
   特化入口经 `native_invoke` 按声明键分派；目标不在分派表时与原入口同样落到调用链外，核对 e2e 无新增存根命中。
6. **档案并 Direct ⊔ Normal**：同一方法在某入口直连、另一入口回退时并后按原入口调用；原入口由回退入口入链，成立。
   需在语料档案构建上核对无存根命中。
7. **克隆混合**：某克隆为具体执行（concrete）而未经 `invoke` 处理时不导出直连（按原入口调用），需确认此时原入口在闭包内。

## 八、遗留

`Method.invoke` 体（CS 判定 → 注解解析、访问器工厂）不再入链的终态，要求它的全部调用点都不经原入口。§6.2 的
其余 4 个入口分别需要：

- **实例目标**（`BasicImageReader$2.run`、`AnnotationInvocationHandler.equalsImpl`）：直连目标放宽到非 CS 实例方法，
  调用点按接收者实参（`invoke` 的 obj）值集对解析出的方法做虚派发接边；特化入口已按 `native_invoke` 处理实例方法与
  空接收者 NPE，无需改动。
- **反射对象跨方法流动**（`ServiceLoader$ProviderImpl.invokeFactoryMethod` 取字段、`equalsImpl` 取数组元素）：查找结果
  不再只认本方法内的查找点，改为给反射对象建「解析出的成员键」抽象值（查找点产出、经字段 / 数组流动），调用点按
  反射对象值集取目标；值集含未解析的反射对象才回退。
- **非常量名字**（`HostLocaleProviderAdapter.findInstalledProvider`）：名字按 `method_lookup.rs` 的拆段口径求候选集，
  逐候选解析。
- **`doPrivileged` 派发精度**：`BasicImageReader$2.run` 经 `Random.<clinit>` 的 `doPrivileged` 按全部 PrivilegedAction
  实现派发入链，属派发精度问题，与本方案正交。
- getEnumConstantsShared 接收者里的非镜像 Class 对象来源未查（疑为手写返回的 Class），查清后可在来源处建镜像。

- `Constructor.newInstance` 同型门（反射构造）可按同一清单形状扩展（`direct_invokers` 键为构造入口），未实施。
- 有实参的静态目标（形参类型数组非空）未纳入：需按形参类型精确解析重载与实参拆箱，另行设计。
