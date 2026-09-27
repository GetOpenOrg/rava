# N11：BoundMethodHandle 动态物种（运行期类生成的原生承载）

> 状态：方案（2026-09-27）。属 FS-H0「运行时类生成」组与 N11 MH-native。
> 验收：RecordsSerializationTest（record `(int, int)` 反序列化）、TestMethodHandleCombinators。

## 一、问题

`ClassSpecializer$Factory.loadSpecies(speciesData)`：

1. `BootLoader.loadClassOrNull("java.lang.invoke.BoundMethodHandle$Species_" + key)` 取预生成物种；
2. 取不到 → `generateConcreteSpeciesCode(className, speciesData)`：ASM 生成字节码 + `Lookup.defineClass`。

jlink 预生成的物种只有 15 个（`L`…`L×9`、`LJ`、`LLJ`、`LLLJ`、`I`、`IL`、`D`、`DL`），已由 `8514fed` 纳入类宇宙。
其余 key（如 record `(int,int)` 反序列化需要的 `LII`、组合子绑定出的 `LI` / `LJL` 等）在真实 JVM 上同样由 ASM 现场生成。
原生二进制不能在运行期定义类，且 key 由运行期参数形状决定，静态不可判定。
静态穷举也不可行：长度 ≤4、字母表 {L,I,J,F,D} 已有 780 个类。

## 二、物种类契约（由预生成类反汇编确认，全部是 key 的纯函数）

以 key = `T0 T1 … Tn-1` 为例（`Ti ∈ {L,I,J,F,D}`，L 的字段类型为 Object）：

| 成员 | 形态 | 消费方 |
|---|---|---|
| `static SpeciesData BMH_SPECIES` | 由 `linkCodeToSpeciesData` 经 `MethodHandleNatives.staticFieldOffset` + `Unsafe.putReference` 写入 | `speciesData()` |
| `final <Ti> arg<Ti><i>` | 构造时写入 | `findGetter(speciesCode, "arg"+Ti+i, type)` → LambdaForm 读绑定参数 |
| `private <init>(MethodType, LambdaForm, T0 … Tn-1)` | `super(mt, lf)` + 逐字段赋值 | `make` |
| `static BMH make(MethodType, LambdaForm, T0 … Tn-1)` | `new Species(...)` | `findStatic(speciesCode, "make", …)` → `SpeciesData.factory()` |
| `final SpeciesData speciesData()` | 返回 `BMH_SPECIES` | BMH 虚方法 |
| `final BMH copyWith(MethodType, LambdaForm)` | `BMH_SPECIES.factory().invokeBasic(mt, lf, arg…)` | 虚方法 |
| `final BMH copyWithExtend<X>(mt, lf, X narg)` | `BMH_SPECIES.extendWith(X).factory().invokeBasic(mt, lf, arg…, narg)` | 虚方法 |

## 三、方案：通用物种载体 + 合成物种类（HotSpot 隐藏类运行时支持的原生对应）

### 3.1 对象模型

新增 **VM 支持类** `java/lang/invoke/BoundMethodHandle$Species_Dyn`（Java 源码，`javac --patch-module java.base` 编进 java.lang.invoke 包，经常规字节码翻译入闭包，与镜像独有类同一入闭包规则：父类 BoundMethodHandle 在闭包内）：

```java
final class BoundMethodHandle$Species_Dyn extends BoundMethodHandle {
    final SpeciesData sd;      // 实例级物种（替代每类一个的静态 BMH_SPECIES）
    final Object[] args;       // 按序存放绑定参数；I/J/F/D 以包装对象存放
    SpeciesData speciesData() { return sd; }
    BoundMethodHandle copyWith(MethodType mt, LambdaForm lf) { … sd 同、args 同 … }
    BoundMethodHandle copyWithExtendL/I/J/F/D(…) { … sd.extendWith(X) + args 追加 … }
}
```

- 全部方法由字节码翻译，无手写越界覆盖（FS-H0 口径）。
- 预生成物种不受影响：15 个预生成 key 仍用翻译出的专用类（BootLoader 命中）。

### 3.2 合成物种类（Class 对象）

`generateConcreteSpeciesCode(className, speciesData)` 是 ClassSpecializer 中唯一不可翻译的点（ASM + defineClass），改由 VM 内建承载（intrinsics.txt 准入，理由：原生二进制无运行期类定义）：

- 返回名为 `BoundMethodHandle$Species_<key>` 的**合成 Class**：运行时类表登记为「Species_Dyn 的 key 视图」。`forName0` / `isInstance` / `asSubclass(BoundMethodHandle)` 均按 Species_Dyn 判定；
- 合成类的**成员面**由 VM 按 key 应答（二节契约）：
  - `MethodHandleNatives.resolve`：`make` / `<init>` / `arg<Ti><i>` / `BMH_SPECIES` 按 key 合成 MemberName（修饰符、类型由 key 推导）；
  - `reflect_invoke(合成类, "make", desc, args)` → 构造 `Species_Dyn(mt, lf, sd = 合成类登记的 SpeciesData, args)`；
  - `reflect_field(合成类, "arg<Ti><i>", recv)` → `recv.args[i]`（基本类型按 `Ti` 拆箱，经 Wrapper 语义）；
  - `BMH_SPECIES` 静态写：登记「合成类 → SpeciesData」（`linkCodeToSpeciesData` 的 Unsafe 写路径，`static_field_id` 表命中合成类名即写入该登记）。

### 3.3 与现有管线的接点

| 管线 | 接点 | 改动 |
|---|---|---|
| ClassSpecializer（翻译） | `generateConcreteSpeciesCode` | 入内建清单，手写 VM 承载（唯一一处） |
| `BootLoader.loadClassOrNull` / `forName0` | 合成类已登记 → 返回 | 运行时类表增加合成类登记表 |
| `MethodHandleNatives.resolve` | 合成类成员 | 按 key 合成元数据 |
| `reflect_invoke` / `reflect_field` | 合成类名 | 路由到 Species_Dyn 的通用实现 |
| Unsafe 静态臂 | `BMH_SPECIES` | 合成类的静态字段写登记 |

## 四、实施步骤

1. **VM 支持类的构建通道**：`runtime/java_support/java.base/java/lang/invoke/BoundMethodHandle$Species_Dyn.java`，转译前按当前 JDK 编译（`--patch-module java.base=…`），以「补充 JDK 类」身份并入解析器（与运行时镜像回落同一优先级，jmod 未命中时查询）。
2. **callchain**：补充类按镜像独有类同一规则入闭包（父类在闭包内 → 实例化 + 全部方法 + 全量反射面）。
3. **runtime**：合成类登记表（名字 → key、SpeciesData）；`generateConcreteSpeciesCode` 内建；resolve / reflect_invoke / reflect_field / Unsafe 静态臂接入。
4. **验收**：新增 e2e `TestBmhDynamicSpecies`（绑定 int / long / double / 混合形状，覆盖非预生成 key 的 bindTo / insertArguments 链、copyWithExtend 链），期望由 JVM 生成；RecordsSerializationTest、TestMethodHandleCombinators。

## 五、不做什么

- 不静态穷举物种（闭包爆炸）。
- 不改 ClassSpecializer 的翻译体（只承载其唯一的类定义点）。
- 不把预生成物种改走通用载体（保留专用字段布局，与 JDK 行为一致）。
