# R-2「Deref<Target=Parent> 替换 From 继承链」调研报告

> 2026-09-24 · 只读调研（未改代码、未跑 cargo/e2e）· 基线 HEAD `1075f11`（浅克隆，历史仅 135 提交）

## 结论摘要

1. **R-2 条目已过期，按原文不可做、也不该做**：`class_writer.py` 里已没有 `From<Child> for Parent` 链（T55 早已删除，只剩注释）；`__into_super` 在代码、宏、生成物里**都不存在**（0 处）。现在的上转全部来自宏 `type_conversions.rs` §10 生成的 `From<Self> for Ancestor`（vtable trait upcasting）。
2. **在现有对象模型下，Deref 在技术上走不通**：wrapper 是 `{vtable: Rc<dyn C__VTable>, any, _jvm_null}`，子类里并没有存着一个父类 wrapper，`deref(&self) -> &Parent` 拿不到可借出的值。而且现有发射点全是**按值**上转（实参、返回值、字段、数组、局部槽位），deref coercion 只作用于 `&T`；方法解析这一层已经由 wrapper 上的继承转发方法覆盖。
3. 「25 处」是 2026-09-21 收敛路线图对 **codegen 源码**的 grep 口径（`Into::`/`__into_super`），其中绝大多数是 Object 装箱（R-1 域）和接口载体（A-4 域），不是类继承上转。本次复核：真正属于「类祖先上转」的 Python 发射点共 **13 处（8 个文件）**，写法有 **3 种文本形态**。
4. 建议：**关闭 R-2**，改立一个真正的小件 **R-2′「上转发射统一 + 过期文档/注释清理」**：约 40–80 行 Python，加上 6 份文档修订。可选的第二步是把上转做成 IR 节点 `UpcastExpr`，并入 TypeIR 余项 / IR 结构化。两步都不需要改宏。
5. 按原定义 R-2 不是小件（要改宏的对象模型，禁改域，价值为负）。按 R-2′ 的定义是小件，但它与在途的 typeir-b3（invoke_sig 域）撞文件，应排在它之后。

---

## 1. 发射点全表与「25 处」口径

### 1.1 `__into_super` 与 `From<Child> for Parent` 的现状

| 项 | 现状 | 证据 |
|---|---|---|
| `__into_super()` 方法 | **不存在**：宏不生成，Python 不发射，`build/jdk21/*` 生成物 0 处 | `grep -rn into_super runtime/java_rta_macros/src` 为空；生成物 grep 为空 |
| `__super()` 方法 | **不存在**：`_super` 嵌套已被「平铺字段」取代（`struct_layout.rs:24`「继承字段（平铺，不再有 _super）」） | `codegen/instr/hierarchy.py:234` `_super_prefix_to_expr` 在 `invoke.py:33` 被 import，但**没有调用点，是死代码** |
| `class_writer.py` 的 From 链（T55） | 已删除，只剩注释 `class_writer.py:1287-1293`，而且注释内容写错了（说「由 `__into_super()` 链替代」「宏生成 Deref」，两者都不成立） | — |
| **实际的 From 链** | **宏** `runtime/java_rta_macros/src/block/gen/type_conversions.rs:111-172`（§10）：对 `all_superclasses` 里的每个祖先生成 `impl From<Self> for Ancestor`（非泛型祖先直接生成；泛型祖先走 A-1 γ' 形态，对祖先的任意实参都成立），实现为 `Ancestor::__from_parts(child.vtable as Rc<dyn Ancestor__VTable>, child.any, child._jvm_null)` | 禁改域 |
| 宏内部的其他消费方 | `wrapper.rs:163-200`：`__view_as` / `__view_into` 的祖先臂复用 `<Anc as From<Self>>::from`（catch 按运行时类还原、`Object::downcast::<T>` 都要用） | 禁改域 |
| 已有的 Deref | 只有接口载体一处：`interface.rs:199` `Deref<Target = Object>`（A-4 载体） | 与类继承无关 |
| `_into_super_chain` | `hierarchy.py:248-252` 是 1 行存根，**固定返回 `'.into()'`**，参数全部没用上 | — |

### 1.2 Python 侧「类祖先上转」发射点（本次计数：13 处）

口径：只数 codegen 中**把子类值转成类祖先（非接口、非 Object）类型**的表达式发射位点，按源码位点计，不按生成物计。

| # | 文件:行 | 函数 / 场景 | 发射形态 |
|---|---|---|---|
| 1 | `codegen/instr/invoke_sig.py:676-687` | `_coerce_arg`：子类实参 → 父类形参（TypeIR S5 已接 `strict_erased_subtype`） | `Clone::clone(&e).into()`（经 `_into_super_chain`） |
| 2 | `codegen/instr/invoke_sig.py:597` | `_upcast_to_ancestor_instantiation`：目标是祖先的另一种实例化（raw / 通配符） | `<Exp as From<Object>>::from(Object::from(Into::<ExactAnc>::into(src)))` |
| 3 | `codegen/instr/sim/returns.py:113-116` | areturn：返回子类型值 | `expr.into()` 或 #2 |
| 4 | `codegen/instr/sim/fields.py:234-235` | putfield：子类值 → 祖先类型字段 | `Clone::clone(&v).into()` |
| 5 | `codegen/instr/sim/arrays.py:174-175` | aastore：子类值 → 祖先元素类型 | `Clone::clone(&v).into()` |
| 6 | `codegen/stack.py:462-468` | 局部槽位：描述符声明为父类、存入子类值 | `Clone::clone(&x).into()` / `(e).into()` |
| 7 | `codegen/stack.py:552-565` | 局部槽位：LVTT 声明为父类（`Node e = putTreeVal(..)`） | 同上 |
| 8 | `codegen/method/vars.py:171` | `_widen_into_merged`：汇合槽取公共类祖先 | `src.into()` / `(src).into()` |
| 9 | `codegen/method/vars.py:288` | 降级声明值侧对齐到提升后的声明类型 | 同上 |
| 10 | `codegen/method/blocks.py:222-227` | 三目合并点，非泛型公共祖先 | `Common::from(tv)` |
| 11 | `codegen/method/blocks.py:229-246` | 三目合并点，泛型父子臂 | `<Common as ::std::convert::From<_>>::from(tv)` |
| 12 | `codegen/instr/invoke.py:594-610` | `super(...)` 构造链，K-5：this 以父类视图传入父类 `__init_on` | `<Sup as ::std::convert::From<Self>>::from(::std::clone::Clone::clone(&this))` |
| 13 | `codegen/emitter/inherited_gen.py:560` | 桥方法返回值（协变返回，部分是上转） | `::std::convert::Into::<Ret>::into(call?)` |

这 13 处共用**3 种文本形态**：后缀 `.into()`、UFCS `<T as From<_>>::from(..)` / `T::from(..)`、turbofish `Into::<T>::into(..)`。决策逻辑分两套：#1 走 TypeIR 的 `strict_erased_subtype`，#3–#9 仍用字符串适配层 `_is_subtype` + `erased_base`。

### 1.3 「25 处」指什么

- 出处：`docs/plans/2026-09-21-codegen-type-convergence.md:16` 与 `:66`（L2-c），原文是「`Into::`/`__into_super` 链 25 处」。这是 **codegen 源码的 grep 计数**（非注释发射行），不是生成物处数。
- 今日同口径复核：`grep -rnE "Into::|into_super" codegen --include=*.py` 共 43 行。去掉注释、docstring、import、def 后，**发射行约 24 行**，与 25 基本吻合（两天内有少量增删）。
- 这 24 行的归属：
  - `Into::<Object>`（Object 装箱，R-1 域）约 9 行：`class_writer` 1012/1055/1056、`sam_objects` 415、`stack` 483、`dynamic` 30、`control` 131、`coerce` 98、`invoke_virtual` 669 等；
  - `Into::<Iface>`（接口载体，A-4 域）约 5 行：`invoke_virtual` 186/696/698/700、`dynamic` 421；
  - 类祖先上转只有 3–4 行：`invoke_sig` 597/679、`inherited_gen` 560，以及 `_into_super_chain` 的 3 个 sim 调用点。
- 结论：「25 处」**把 R-1、A-4、R-2 三个域的发射混在了一起**。R-2 自身真正的面是 §1.2 的 13 处，其中大部分用的是 `.into()`/`From::from` 形态，根本不含 `Into::` 字样，所以原 grep 口径既漏掉了真实的上转点，又混进了不相干的发射。

### 1.4 生成物形态（`build/jdk21/` 现存 9 个测试工作区，宏展开前的源码）

| 工作区 | `Clone::clone(..).into()` | `<X as ::std::convert::From<Self>>::from` |
|---|---|---|
| test_record / test_annotations / test_enum_set_map 等小测试 | 23 | 76–86 |
| test_array_list | 34 | 86 |
| test_atomics / test_bridge_method / test_pattern_match（含 j.u.c 大链） | 277 | 663–670 |

注：`.into()` 的计数里也含 Object 装箱，不能全部算作上转；`From<Self>` 主要来自 #12 构造链。用户类示例在 `test_bridge_method/user/src/test_bridge_method.rs:116-138`，例如 `let mut b: CovBase = (CovDerived::new()?).into();`，这已经是「可读层」形态，没有 `Rc::new`/`downcast`。

---

## 2. R-1 落地内容与 R-2 的差距；Deref 语义评估

### 2.1 历史

- `docs/plans/2026-09-17-java-rust-type-1to1.md:655-656` 记录 R-1+R-3 在 `770fec3`、R-2 在 `b40b8c3`（2026-09-17）完成。**这两个提交在当前浅克隆里都不存在**（`git show` 报 unknown revision），无法核对 diff。
- 那一版 R-2 的前提是「inner struct 嵌套 `_super: Parent__inner`，`Deref` 返回 `&self.0.borrow()._super`」（type-1to1 §R-2「目标」一节）。之后的 K-5 单一对象模型、平铺字段、A-1 擦除 vtable、wrapper 化（`vtable + any` 两个指针）**整体推翻了这个前提**：`_super` 字段、`__super()`、`__into_super()`、Deref 全部消失，上转改由宏 §10 的 `From<Self> for Ancestor` 承担。`docs/tasks-history.md:2651` 也写明「被块级宏方案取代」。
- R-1（blanket `From<T> for Object`）现状：保留，没有空 binary_name 的类由宏 `type_conversions.rs:100-109` 补 `Into<Object>`。**R-1 是 R-2 的前置条件这一说法已无意义**：两者作用于不同的目标类型，不存在冲突。
- 所以 tasks.md:145 的描述「删除 class_writer.py 的 `From<Child> for Parent` 链生成」**指向一个已经不存在的东西**。

### 2.2 Deref 方案在当前模型下能覆盖什么

| 情形 | Deref 能否覆盖 | 说明 |
|---|---|---|
| 返回 `&Parent`（前提） | **不能** | 子类 wrapper 里没有父类 wrapper 值。`Rc<dyn Child__VTable>` 和 `Rc<dyn Parent__VTable>` 的胖指针元数据不同，不能 transmute。唯一出路是在 wrapper 里按层缓存祖先视图（`OnceCell<Parent>` × 继承深度），要改宏的 struct 布局、Clone/Default/PartialEq，每个对象的内存按层数膨胀 |
| 按值上转（实参 / 返回 / 字段 / 数组 / 局部） | **不能** | deref coercion 只做 `&Child → &Parent`；`let a: Animal = dog` 需要的是一个拥有所有权的 `Animal`。§1.2 的 13 处**全部是按值**，所以即使 Deref 存在，也一处都替换不掉 |
| 方法解析（`dog.getName()`，定义在 Animal） | 已有更好的方案 | 宏的 `inherited` 转发（`context.rs:200-206`）加 `inherited_gen.py` 已经在 wrapper 上生成转发方法，并且走 vtable 虚分派。Deref 自动解引用得到的是**静态绑定**到父类视图的调用，只能靠 vtable 补救，不带来任何收益 |
| 多层继承 | 只有链式 Deref 可以 | 现有 `From<Self> for Ancestor` 本来就是「每个祖先一跳」，多层也没有额外发射 |
| 接口载体 | 不能 | Rust 一个类型只能有一个 `Deref`，接口多继承表达不了；载体已经占用了 `Deref<Target=Object>` |
| 泛型父类（`Sorter<T> → CountedCompleter<Object>`） | 不能 | `Deref::Target` 是唯一的关联类型，只能对应一种实例化；宏 γ' 的 `From` 对任意祖先实参都成立（跨实例化上转），Deref 无法表达 |
| 方法名歧义 | 会变差 | 自动解引用在子类同名固有方法（重载改名、`__get_x` 访问器）与父类之间静默选中第一个，Java 重载 / 遮蔽语义更难保证 |
| CLAUDE.md 可读层禁令（`Rc::new`/`downcast`/`borrow`） | 现状已合规 | 可读层里只有 `.into()` / `From::from`，`Rc` 转换封装在宏的 `__from_parts` 中。Deref 方案对可读层没有改善；若按原设计 `&self.0.borrow()._super`，反而会把 `borrow` 带回宏内 |

**判定**：Deref 替代 From 是一个在旧对象模型下成立、在现模型下既不可行也不划算的方案。R-2 应**标记为「被 K-5/A-1 取代，关闭」**。

### 2.3 真正还剩的差距（可做的部分）

1. **过期注释 / 死代码**：`class_writer.py:1287-1293`、`invoke_sig.py:678`（注释说「R-2：显式 `__into_super()` 链」）、`jvm_type.py:425`（「__into_super 链」）、`hierarchy.py:248-252` 的 docstring；死函数 `hierarchy._super_prefix_to_expr` 及其在 `invoke.py:33` 的 import。
2. **形态不统一**：同一语义（子类值 → 类祖先）在 13 处有 3 种写法，判定也分两套（TypeIR 与字符串适配层）。
3. **过期文档**：`java-rust-translation-reference.md` §5.1–5.3（`_super` 嵌套、Deref、`__into_super`）、`:346`、`:918`；`2026-09-16-java-class-macro-unified.md:683`（「三个合法入口 `__super()/__into_super()/__new_with_super()`」）；`2026-09-18-macro-family-design.md:24,48`；`2026-09-17-java-rust-type-1to1.md` 的 R-2 状态和 M-1「已跳过」；`2026-09-21-codegen-type-convergence.md` L2-c；`tasks.md:145`；`2026-09-15-e2e-unresolved-issues.md:54`。
4. 命名冲突提示：`2026-09-19-remaining-issues.md:513` 里另有一个「R-2 待清理的 worktree」，与本 R-2 无关，建议改名以免混淆。

---

## 3. 修复方案

### 方案 A（推荐，R-2′）：关闭 R-2，改做上转发射统一 + 清理，不改宏

| 步 | 改动点 | 量 |
|---|---|---|
| A1 | `hierarchy.py`：`_into_super_chain` 改名为 `upcast_expr(src, actual_t, expected_t, registry, *, clone)`，统一输出一种形态。建议 `<Exp as ::std::convert::From<_>>::from(src)` 或保留 `.into()`；为了逐字节零 diff，第一步保留 `.into()`。内部统一经 `strict_erased_subtype` 判定，吸收 `_upcast_to_ancestor_instantiation` 的重新实例化分支 | +30 / −10 |
| A2 | #1、#3–#9 调用点改调 `upcast_expr`，其中 returns/fields/arrays/stack/vars 的 `_is_subtype(erased_base..)` 判定换成 TypeIR（相当于 TypeIR 批次的延续） | 每处 ±3–6 行，约 40 行 |
| A3 | 删死代码 `_super_prefix_to_expr` 与 import，修正 4 处过期注释 | −15 |
| A4 | 文档：reference §5 按「平铺字段 + 宏 `From<Self> for Ancestor` + wrapper 继承转发」重写；tasks.md 的 R-2 行改为「已被取代」加 R-2′；其余 5 份文档加过期标注 | 文档约 80 行 |

- **预估 Python 净改动 40–80 行**，不碰 `runtime/java_rta_macros/`，不碰手写运行时。
- **生成代码形态变化**：第一步严格保持形态（`.into()` 不变），硬约束是**生成树逐字节零 diff**，所以 163 语料的扰动面为 0。若第二步把形态统一成 UFCS，受影响的是 #6–#9 的 `.into()` 行（每个 j.u.c 大工作区约 277 处）。这些是纯表达式替换，类型推断反而更明确（`From<_>` 的目标由左值给定），风险低，但会改变全部 TestAtomics 级别的生成树，需要全量对账一次。
- **风险**：
  - ① #12（super 构造视图）与 #10/#11（合并点）已经用 UFCS，#2 用 turbofish，统一时注意 `<T as From<_>>` 在 `T` 带 `_` 实参时的推断差异，泛型父子臂需要保留 `_`；
  - ② `_is_subtype` 换 `strict_erased_subtype` 必须沿用批次 2/S5 的「204 万对对拍」做法，保证逐点一致；
  - ③ 与在途 **typeir-b3**（`invoke_sig.py` ×5 + `invoke_virtual.py` ×4）撞文件，A1/A2 必须排在它合入之后。

### 方案 B（不推荐）：真正做 Deref

在宏 wrapper 里为每层祖先加 `OnceCell<Anc>` 缓存并实现 `Deref<Target=DirectParent>`。需要改禁改域 `wrapper.rs` / `struct_layout.rs` / `type_conversions.rs`，也改不掉任何一个按值发射点（§2.2）。**收益约为 0，风险高**：对象体积膨胀、Clone 语义、`PartialEq` 身份判等都要重审。不建议。

### 方案 C（中期，随 IR 结构化）：`UpcastExpr` IR 节点

仿照 A-3 的 `CastExpr`，新增 `UpcastExpr(expr, target_t)`，由 `render_expr` 唯一决定形态，13 处全部 push 节点而不是 RawExpr 字符串。这属于 IR 结构化（T05/T06…）或 L3「转换的表示」，应在 A 完成后顺手进行。

---

## 4. 验证方案

**定向回归集**（本地要求 ≤10 例，分两批跑）：

- 第一批（继承 / 上转主干）：`02_oop/TestInheritance`、`02_oop/TestBridgeMethod`、`02_oop/TestConstructorChain`（#12 super 视图）、`19_abstract/TestAbstractClass`、`31_callchain/TestInheritedMethod`、`18_arrays_advanced/TestArrayCovariance`（#5 aastore）、`46_generics_deep/TestRawTypes`（#2 重新实例化）、`46_generics_deep/TestPecs`、`12_generics_advanced/TestBoundedGenerics`、`06_exceptions/TestCustomException`（异常层次上转与 catch 视图）
- 第二批（合并点 / 泛型父类 / 大 JDK 链）：`01_basics/TestHoistShadow`（#8/#9 槽位 widening）、`02_oop/TestFieldShadow`（#4）、`16_modern/TestSealed`、`16_modern/TestPatternMatch`、`29_nested_generic/TestNestedGeneric`、`03_generics/TestGenerics`、`49_exceptions_deep/TestExceptionChain`、TestAtomics（j.u.c 链上 #10/#11 与 CHM 三目合并的热区）、`04_collections/TestArrayList`

**验收口径**：

1. 方案 A 第一步：上述 19 例**生成树与基线逐字节一致**（`diff -r build/<test>/{user,java_runtime}/src`），运行结果不变；再加全量对账（由用户服务器执行），163 语料通过数不下降。
2. 同时 grep：`codegen` 内 `into_super`/`__super(` 为 0，可读层的 `Rc::new|downcast::<|borrow()` 计数不增加。
3. 若做第二步统一 UFCS：上述 19 例 e2e 输出一致，全量通过数不下降；接受生成树 diff，但 diff 只能出现在「`.into()` → `<T as From<_>>::from`」这种一一对应的替换上（用脚本校验 diff 行都属于这种模式）。

---

## 5. 是不是「小件」，以及依赖

- **按 tasks.md 原定义（删 class_writer From 链、改用 Deref）**：不是小件，而且**不成立**。被删的对象已不存在；Deref 需要改宏的对象模型（禁改域），也不能替换任何按值上转。建议直接关闭。
- **按 R-2′（统一 + 清理）**：确实是小件。Python 约 40–80 行，文档约 80 行，第一步生成物零 diff，半天工作量。
- **依赖 / 交叠**：
  - **TypeIR 余项**：A2 中 returns/fields/arrays/stack/vars 的判定迁移属于 TypeIR 批次的自然延续。`invoke_sig.py` 当前被 typeir-b3 在途占用，**R-2′ 必须排在 typeir-b3 合入之后**，或者只先做不碰 invoke_sig 的 A3/A4。
  - **窗口 3（G-3 三重槽）**：没有依赖；共同碰到的只有 `stack.py` 的槽位逻辑，A2 改 stack.py 前应确认窗口 3 不在途。
  - **M-3 宏拆库 / 签名决策进宏**：R-2′ 不碰宏，与 M-3 正交。只有方案 B（Deref）才需要改宏，届时会受 M-3 拆库节奏约束，这是方案 B 不可取的又一个理由。宏 §10 的 `From<Self> for Ancestor` 在 M-3 中应原样保留，它是上转的唯一运行时真源。
  - **IR 结构化**：方案 C（`UpcastExpr`）依附于它，放在 R-2′ 之后。
