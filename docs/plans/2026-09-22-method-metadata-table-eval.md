# MemberName / 方法元数据表评估结论

> 2026-09-22 · fix/reflection-metadata 收尾时点 · 前任（WIP 177bc84）对方法侧无任何进展，
> 本文为评估结论，不改 codegen（A-4 第四任并行改造 codegen+宏中，禁改域）。

## 1. 结论（一句话）

方法元数据表的**数据源已经存在**（codegen 已无条件发射 `java_method` 属性，比
`java_field` 更丰富），build.rs 侧造表（`method_table.rs`）与查询层
（`Class.getDeclaredMethod`）完全可复制本批 field_table 的模式，属本域可做；
但核心消费 `Method.invoke` 需要「按名/描述符分派方法」的运行时协议，该协议只能
由宏/codegen 发射（A-4 域）——**建议分层推进：L1/L2 待 A-4 合入后立即做，
L3 与 A-4 的 vtable 改造合流设计，避免两套按名协议**。

## 2. 现状盘点

### 2.1 数据源：java_method 属性已就位（无需改 codegen）

`_java_method_attr`（codegen/emitter/attrs.py:372）对每个进入闭包的方法无条件发射
`name` / `descriptor` 身份键，另有 `access` / `modifiers` / `is_static` /
`is_native` / `is_abstract` / `is_synthetic` / `exceptions` / `generic_signature` /
`is_deprecated` / `virtual_in` / `vtable_name` / `vtable_erasure` / `body` /
`method_parameters`。字段侧本批消费的元数据子集（名/描述符/修饰位/static/
常量）方法侧全部有对应物（方法无 ConstantValue 对应物，不需要）。

形态注意：方法属性行在 `java_class! { impl ... }` 块内是**单段路径**
`#[java_method(...)]` / `#[java_native(...)]`（非 cfg_attr 包裹）；scratch 实测
只存在这一种形态。

### 2.2 消费方：三层全缺

| 层 | 现状 |
|---|---|
| 查询层 `Class.getDeclaredMethod/getMethod` | runtime 无任何实现（class_impl.rs 无此方法，`grep getDeclaredMethod` 零命中） |
| 表示层 `Method/Constructor` | 生成文件全 stub（`Method.<init>`、`copy`、`getRoot`… 均 `panic!("stub: ...")`） |
| 分派层 `Method.invoke` / `MethodHandles` / MemberName | `jdk/internal/reflect/` 只有 reflection_impl.rs 等四个文件，MemberName 类未物化 |

（对照：字段侧接手时查询层已有「查询即构造」雏形，本批补表+读写；方法侧是零起点。）

### 2.3 规模（TestReflectProbe scratch，1345 类实测）

- `java_field` 行：6,414 → field_table.rs 885KB，编译无压力（已验证）
- `java_method/java_native` 行：26,891（≈4.2 倍）→ 估 method_table.rs ~3.5MB
- 结论：单文件静态表仍可行（build.rs 造表无大小障碍），如编译时间受影响再考虑
  按 binary name 首段分片（后话，不过早设计）

## 3. 已识别的设计坑（本评估的核心产出）

### 3.1 对齐空格坑在方法侧是真身

`_java_field_attr` 发射 `is_static = true`（单空格），而 `_java_method_attr` 发射
`is_static    = true`（**4 空格对齐**，attrs.py:399-405）。本批 build.rs 的
`extract_attr`（严格 `key = "` 单空格模式）若被直接复用到方法侧的布尔标志，
会静默漏配——**scan_class_methods 的全部键提取必须用 padded 容忍形态**
（本批 `extract_flag` 已是 trimmed+strip('=') 容忍式，可直接推广；身份键
`name = "` / `descriptor = "` 两发射器都是首二段单空格，严格式亦安全）。

### 3.2 方法身份键是 (name, descriptor) 二元组

字段按名唯一；方法重载使 name 不唯一，命中判定必须 (name, parameter-types)
配对——JDK `getDeclaredMethod(name, Class<?>... parameterTypes)` 把参数类型还原
成描述符后比对。描述符还原函数与 `class_for_descriptor`（class_impl.rs）互为
逆操作，放同一处。

### 3.3 两套属性行前缀都要扫

`java_method(` 与 `java_native(`（native 方法的元数据也是方法表行，JDK 反射
对 native 方法一视同仁）。

### 3.4 not-found 语义同构可做

`NoSuchMethodException` upcall 种子 + 真实异常对象、消息=方法名——与本批
NoSuchFieldException 完全同构（error.rs vm-upcalls 或 jvm_boundary upcalls 二选一，
本批实际两处都写了，集合语义幂等）。

## 4. 分层推进建议（终态架构）

- **L1 造表（本域：build.rs）**：`scan_class_methods` → OUT_DIR/method_table.rs
  （binary name → 方法序列：name/descriptor/modifiers/is_static/is_native/
  is_abstract/exceptions，声明序 = Method 声明序语义）。坑 3.1/3.3 的提取器
  直接复用本批代码。半天量级。
- **L2 查询层（本域：class_impl 伴生）**：`getDeclaredMethod/getDeclaredMethods`
  命中 → 查询即构造 Method（clazz/name/描述符还原的参数表/修饰位/slot），
  not-found → NoSuchMethodException。对象身份按 (声明类, name, descriptor)
  ——与 Field 的不透明身份协议（unsafe__impl 消费 (声明类, 字段名)）同族扩展。
- **L3 分派层（A-4 域，禁改）**：`Method.invoke` 的按名分派。两个候选：
  - (a) ObjectVTable 追加 `__method_invoke(name, desc, args) -> Object` 臂，
    宏为平铺方法生成按名分派——与 `__unsafe_int/long_cell` 按名协议**同一形态**，
    vtable 臂数受调用链闭包约束（推荐）；
  - (b) codegen 发射 per-class 描述符→fn 指针闭包表——静态直连但引第二套
    按名机制，与 (a) 二选一，不可并存。
  - 推荐 (a)，且必须与 A-4 的 vtable/宏改造合流设计（同一套按名协议承载
    Field 引用读写扩展与方法分派），避免两套协议。
- **MemberName 定位**：`jdk/internal/reflect` 边界类（规则 3b），是 Method 与
  分派协议之间的内部适配层，HotSpot 中 Method.invoke → MemberName.invoke 的
  对应物在本架构里是 Method → `__method_invoke` vtable 臂。L3 落地前**不物化
  MemberName 文件**；落地时也仅作内部实现细节，不进公开命名空间。

## 5. 与本批（field_table）的衔接

field_table.rs 已绿（TestReflectProbe/FieldDemo 转正、TestCollectionFactory
FAIL→PASS）。method_table 完成后，`Class.getDeclaredFields/getDeclaredConstructors`
（本批未做，getDeclaredField 的复数形态）可与 L1/L2 同批补齐——同一张表，
`+` 循环输出即可。

## 6. 明确不做清单

- 不改 `_java_method_attr` 的发射格式（含其 4 空格对齐）——数据已够用，
  格式是 A-4 域的活物；
- 不在 A-4 合入前动 `runtime/java_rta_macros`（vtable 臂生成）；
- 不提前物化 MemberName（无消费者的手写文件违反规则 3b 的按需节奏）。
