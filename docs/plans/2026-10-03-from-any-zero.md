# 生成代码 `Object::from_any` 归零（2026-10-03，调查稿，待 c1d-lambda-class 合入后实施）

> 状态：①–④ 已实施（① ec714d98 合入；② b820c8aa 合入；③ 44b3a3b3；④ 见第六节）。终态：可读性审计 `from_any = 0`，覆盖全部生成 crate（`java_runtime`、`java_body_*`、lib crate、`user`），并在全量语料上成立。
> 手写层（`*_impl.rs` / `object_ext.rs` 等无生成标记的文件）的 `Object::from_any` 审计不统计；④ 已逐处清点归属，见第六节。

## 一、现状实测（TestLambdaHiddenClass，c1d-lambda-class 6baa49ad）

- `[readability-audit] from_any=9` 是**低估值**。`build_cmd.rs` 传给 `readability_counts` 的 crate 列表只有 `java_runtime` + lib + `user`，没有实现层拆分 crate `java_body_1..5`。
- 按生成标记逐文件实数，结果如下：

| crate | from_any 次数 |
|-------|---------------|
| java_runtime | 9 |
| java_body_1 | 2 |
| java_body_2 | 32 |
| java_body_3 | 8 |
| java_body_4 | 4 |
| java_body_5 | 10 |
| user | 0 |
| 合计 | 65 |

- 全部 65 处都是同一形态：`let _tN = Object::from_any(<调用>?);`。被调方法在泛型签名里返回**裸类型变量**（`V` / `E` / `R` / `T`），描述符擦除为 Object，而生成器在调用点没能把这个类型变量实例化出来。
- 新增的 lambda 隐藏类 / sam_objects 路径不发射 `from_any`。这 65 处与本分支无关，是存量问题。

## 二、发射点与规则

| # | 发射点 | 触发条件 | 本例命中 |
|---|--------|----------|----------|
| E1 | `instr/src/invoke/virtual_/direct.rs`，`resolve_direct` 尾部的 `None if is_object(rust_ret) && erased_tv()` 臂 | `lookup_method_sig_ret` 返回 None，且被调方法签名的返回类型是裸类型变量 | 绝大多数 |
| E2 | `instr/src/invoke/special.rs`，`emit_call_result` | invokespecial 返回裸类型变量。这里**完全不做实例化**，Rust 返回类型只要是 Object 就套 `from_any` | `SoftReference.get → Reference__get_base::<T>`、`LocalDate.query → ChronoLocalDate_super_query` |
| E3 | `instr/src/coerce.rs` `to_object` 的 `ObjectKind::Opaque` 臂，及其文本版 `method/src/coerce_text.rs` | 值的 Rust 类型不是基本类型 / 作用域类型形参 / 数组 / 注册表内类（`()` 也落到这里） | 本例 0 |
| E4 | `instr/src/invoke/virtual_/args.rs:111` | 基本类型接收者调用根类声明的方法，生成 `Object::from_any(prim).m()` | 本例 0 |
| E5 | `instr/src/sim/dynamic/lambda_body.rs:176`、`instr/src/sim/dynamic/boxing.rs:38` | 方法引用 / lambda 体结果装箱（②实施时补查到，例：DateTimeFormatterBuilder 中 `Map::get` 方法引用） | 验收集 1 处（第④步一并处理） |
| — | `instr/src/invoke/special/ctor.rs:185`、`method/src/postprocess.rs:53` | 不发射 `from_any`，只是把它当作已知前缀做识别 / 改写 | 发射点清零后同步删去 |

E1 / E2 背后有四个类型层缺口，按本例的落点归类：

- **A. 签名声明者只沿超类链找，不进超接口。**
  - 涉及 `instr/src/owner.rs::resolve_method_owner`（只走 super_class）和 `sig/ret.rs::lookup`（只查 `call.owner` 自己声明的方法）。
  - 后果：方法继承自超接口时，查签名直接失败。
  - 本例：`NavigableMap.put/remove`（声明在 Map，TreeSet ×2）、`ConcurrentMap.get/put`（Map，LocaleObjectCache ×2，ZoneOffset / ZoneRulesProvider 等）、`AbstractImmutableList.get`（List ×2）、`AccumulatingSink.get` / `TerminalSink.get`（Supplier，ReduceOp / FindOp）。
- **B. 接口视图没有代入接收者的实际类型实参。**
  - `sig/ret.rs::resolve_type_vars` 的 K-6b 分支调用 `implemented_interface_views(recv_ci)`。它内部用 `ancestor_type_args(recv, None)`，得到的是接收者类**自己的形参名**（例如 HashMap 视图下 Map 的 `[K, V]`），没有再按接收者的 Rust 实参（`HashMap<Object, Object>`）替换。
  - 后果：结果里的 `K` / `V` 不在调用者作用域内，`all_known` 判为假，返回 None。
  - 本例：`HashMap<Object,Object>` 局部变量上调 `Map.put`（DateTimeFormatter 19 处、CalendarSystem、DateFormatSymbols…）、`ArrayList<Object>.get/remove`（经 List，ProtectionDomain ×4）、`ArrayDeque<Object>.pop`（经 Deque，Configuration）。
  - 修了 A 之后，A 类调用点同样要经过 B 这一步（例如从 NavigableMap 视图到 Map），所以 A、B 必须一起修。
- **C. 方法级类型变量。** 例如 `<R> R query(TemporalQuery<R>)`。
  - 生成的 Rust 方法签名把方法级类型变量擦除为 Object（`fn ChronoLocalDate_super_query(..) -> Result<Object>`），所以调用结果本来就是 Object，`from_any` 是纯冗余。
  - 生成器目前不区分方法级和类级类型变量，统一按「无法实例化」处理。
- **D. invokespecial 不做实例化**（即 E2）。
  - `super.m()` 的接收者是 `this`。被调类的实参完全可以由当前类的超类型视图给出，例如 `SoftReference<T> → Reference<T>`，得到 `T`，而 T 在作用域内。

## 三、终态方案

**原则：** 调用结果的值类型等于「被调方法生成的 Rust 返回类型，在接收者 Rust 类型下的实例化」。这个类型完全由生成器自己产出的签名和接收者类型决定，因此**不存在**「无法实例化」的情形，`from_any` 臂整体删除，不留兜底。

1. **声明者解析（修 A）**
   - `instr/src/owner.rs` 新增按 JVMS §5.4.3.3 / §5.4.3.4 的方法解析：先走超类链，再在超接口中找最具体的声明（maximally-specific）。
   - `resolve_method_owner` 的现有调用方语义不变，签名查找改用新解析。
   - `direct.rs::resolve_direct_call_sig` 在 `owner_bin` 为 None 且命中接口声明时，把 `sig_owner` 设为接口声明者，`sig_recv_ty` 设为该接口在接收者下的视图（交给步骤 2）。
   - `sig/ret.rs::lookup` 不再只认 `call.owner` 自己声明的方法：在 `call.owner` 未声明时走同一解析。
2. **视图实参代入（修 B）**
   - `ty/src/type_args/views.rs::implemented_interface_views` 增加 `self_args: Option<&[RsType]>` 参数，透传给 `ancestor_type_args(recv, self_args)`，并在接口 BFS 的每一层把映射建立在实际实参上。
   - `sig/ret.rs::resolve_type_vars` 的 K-6b 分支传入 `recv.type_args()`。
   - `direct.rs::owner_view`（`ancestor_vtable_args_by_short`）核对是否已有同一缺口，有则同修。
   - 原始类型（raw）接收者（实参为空）：类型变量按其上界擦除，与描述符一致，结果为 Object 或上界类。
3. **方法级类型变量（修 C）**
   - `sig::erased_ret_is_type_var` 改为返回类型变量的作用域：类级 / 方法级。
   - 方法级时，Rust 返回类型即签名擦除后的类型（上界，通常是 Object），按描述符类型直接落值，不装箱。
4. **invokespecial 实例化（修 D）**
   - `special.rs::emit_call_result` 改为走与 direct 相同的 `lookup_method_sig_ret`，接收者类型取 `this` 的 Rust 类型（当前类加上其形参）。
   - 与 direct 共用同一组 match 臂：签名类型与描述符不同时按签名类型入栈。
   - 「签名类型与描述符不同就按签名入栈」这组 match 臂抽成 `sig::bind_call_result` 一个函数，direct / special / bare 三处共用，避免规则再分叉。
5. **Opaque / 基本类型接收者（E3 / E4）**
   - `coerce::object_kind` 的 Opaque 臂改为 `Into::<Object>::into(..)`。理由：所有引用形态的 Rust 类型都有 `From<T> for Object`，见 `java-rust-translation-reference.md` §16 中 `from_any → .into()` 一行。
   - `()` / void 进入 `to_object` 属于生成器内部错误，改为返回 `InstrError` 断言，并在 fallback-audit 中计数，而不是静默装箱。
   - `args.rs:111` 改用 `coerce::to_object`（Prim 臂 `.into()`）。
6. **清理**
   - 删去 `ctor.rs:185` 与 `postprocess.rs:53` 正则中的 `from_any` 识别项。
   - `java-rust-translation-reference.md` §16 中 `from_any` 一行标记为完成。
7. **审计口径（第一步先做）**
   - `driver/src/build_cmd.rs` 把 `java_body_*` 实现层 crate 纳入 `readability_counts`。基线因此从 9 变为真实值（本例 65），后续以真实值验收。
   - `rava_moved` 声明与实现层的同一方法只计一次：声明层只有签名，没有方法体，天然不重复。
8. **守护**
   - 生成器单元测试新增四例：A（继承自超接口的 `Map.get`）、B（`HashMap<Object,Object>` 经 Map 视图）、C（`<R> R query`）、D（`super.get()` 返回 T），断言调用结果的文本与入栈类型。
   - 再加一个生成树断言：全部生成 crate 中 `from_any` 计数为 0（`audit.rs` 测试形态）。

## 四、涉及文件

| 文件 | 改动 |
|------|------|
| `generator/crates/driver/src/build_cmd.rs` | 审计 crate 列表纳入 `java_body_*` |
| `generator/crates/instr/src/owner.rs` | 方法解析纳入超接口（maximally-specific） |
| `generator/crates/instr/src/invoke/sig/ret.rs` | `lookup` 解析继承声明；`resolve_type_vars` 视图代入实参，raw 接收者擦除到上界 |
| `generator/crates/instr/src/invoke/sig.rs` | `erased_ret_is_type_var` 区分类级 / 方法级；新增 `bind_call_result` |
| `generator/crates/ty/src/type_args/views.rs`（必要时含 `type_args/mod.rs`） | `implemented_interface_views` 支持实际实参 |
| `generator/crates/instr/src/invoke/virtual_/direct.rs` | 接口声明者的 `sig_owner` / `sig_recv_ty`；删 `from_any` 臂，改用 `bind_call_result` |
| `generator/crates/instr/src/invoke/special.rs` | `emit_call_result` 改为实例化 + `bind_call_result` |
| `generator/crates/instr/src/invoke/virtual_/bare.rs` | 改用 `bind_call_result`（规则统一） |
| `generator/crates/instr/src/invoke/virtual_/args.rs` | 基本类型接收者经 `to_object` |
| `generator/crates/instr/src/coerce.rs`、`generator/crates/method/src/coerce_text.rs` | Opaque → `Into::<Object>::into`；void 断言 |
| `generator/crates/instr/src/invoke/special/ctor.rs`、`generator/crates/method/src/postprocess.rs` | 删 `from_any` 识别项 |
| `docs/plans/java-rust-translation-reference.md` | §16 状态更新 |

`Object::from_any` 作为手写层 API 保留，生成代码不再引用它；`runtime/` 只清手写层中属于 Java 语义的一处（见第六节）。

## 五、影响面与验收

- **影响面：** 不只是这 65 处。修 A / B / D 之后，凡是「返回类型变量、继承自超接口或经接口视图」的调用点，入栈类型都会从 Object 变成精确类型（例如 `E`、`T`、具体类）。
  - 下游的实参强转、`checkcast` 折叠、局部变量槽位类型、块合并（`unify`）都会随之变化。
  - 预计 JDK 集合 / 流 / time / 安全包的生成树会大面积变化，用户类变化很小。
  - 规则仍是 `Into::<Object>::into` / 按签名入栈这两条既有路径，没有新的 Rust 形态，所以编译风险集中在「精确类型流入以前只见过 Object 的槽位」。这条路径由 `coerce_arg` / `unify` 已有的 TypeVar 处理覆盖，需要用全量编译确认。
- **与在途分支的关系：** 只动生成器的 instr / ty / driver 层，不碰闭包分析器，不涉及 b1 / b3 文件，与 C1d 各分支无文件冲突。
- **验收：**
  1. `scripts/gen_trees.sh` + `scripts/compare_trees.sh` 对 27 例验收集出对照，差异只限于本计划的调用点形态。
  2. 全部生成 crate 的 `from_any` = 0，验收集与全量语料都要满足。
  3. 验收集全部本机 `rava compile` 通过。
  4. 分布式全量 e2e 与基线对照无新增失败。
  5. `cargo test --release`（含 jdk_literal_lint）通过。
- **提交顺序：**
  1. 审计口径（独立小步，先合入，基线变为真实值）；
  2. A + B（类型层与签名解析，单独合入）；
  3. C + D + `bind_call_result`；
  4. E3 / E4 + 清理。

  每步本机编译验收集，推两远端后抽查。

## 六、实施记录（④）与手写层归属

### 6.1 ④ 生成器改动

- `instr/src/coerce.rs`：`ObjectKind` 收为 `Prim` / `Ref` / `Into` / `Void` 四类。原 `TypeVar` 与 `Opaque` 合并为 `Into`，一律发射 `Into::<Object>::into(..)`（`From<T> for Object` 覆盖 ObjectVTable 实现者，自反 `From<Object>` 覆盖 Object 自身）。新增 `to_object_text` 作为文本入口的唯一实现，`method/src/coerce_text.rs::to_object` 与 `sim/dynamic/boxing.rs::obj_text` 改为委托（原先三份平行实现）。
- `Void`：void 值流入 Object 槽位返回 `InstrError::BadInsn`（方法体生成 Fatal），不再静默装箱。文本入口随之改为返回 `Result`（`unify_pair` / `acmp_operand` / `widen_into_merged` / `hoist_loop_vars` 透传）。
- `invoke/sig.rs`：`CallBind::Opaque` 更名 `CallBind::TypeVar`（类级裸类型变量、签名无法实例化），direct / invokespecial 两处发射 `Into::<Object>::into(..)`。
- `invoke/virtual_/args.rs`：基本类型接收者调用根类方法改为 `Object::from(prim).m()`（基本类型实现 ObjectVTable）。
- `sim/dynamic/lambda_body.rs`：lambda / 方法引用一律经 samtype 的 SAM 合成对象（隐藏类身份）。samtype 不可合成（不在注册表 / 非函数式接口 / 预扫描漏登）改为生成器内部错误，删去「无类身份的裸闭包装箱」退化臂。JVM 对每个 lambda 调用点都定义实现 samtype 的隐藏类，该退化臂没有对应的 Java 语义；验收集 27 例命中 0 处。
- 识别项：`special/ctor.rs::strip_boxing` 与 `method/src/postprocess.rs::ERASED_NEW_RE` 删去 `Object::from_any` 前缀。
- 守护：`driver/tests/build_cli.rs::batch_trace_and_debug` 断言 `[readability-audit] from_any=0`（RecordSwitch 全闭包：用户类 + JDK 生成类 + `java_body_*`）；`record_and_switch_bootstraps_translate` 原有的用户类文本断言保留。

### 6.2 手写层清点（`runtime/`，无生成标记的文件）

| 位置 | 形态 | 归属 / 处理 |
|------|------|-------------|
| `sun/util/locale/provider/calendar_data_utility_impl.rs` `_names_map` | `let _ = Object::from_any(map.put(..)?)` | 属于 Java 语义层（丢弃 `Map.put` 返回值），且 `from_any` 本身多余。**已清零**，改为 `map.put(..)?;` |
| `java/lang/throwable_impl.rs` `fillInStackTrace_i` | `Object::from_any(frames)` | **可读层外的 VM 实现细节**。`backtrace` 是 VM 注入的隐藏字段（handwritten-boundary 准入 ③：栈遍历），负载 `StackTraceFrames` 是没有 Java 类的 Rust 结构，只能以不透明载体存入 Object 槽位。`from_any` 正是这一承载原语，保留 |
| `java/lang/object_ext.rs` `Object::from_any` | 定义本身 | 对象模型 API（不透明 Rust 负载 → Object，`JvmRef` 承载）。供上一行这类 VM 负载使用，生成代码不引用。保留 |
| `rava_macros_core/src/block/gen/type_conversions.rs` | 宏展开 `impl Into<Object> for X { Object::from_any(self) }`（`binary_name` 为空的类） | 宏展开内部，不出现在方法体文本中，属于对象模型实现细节。保留 |

结论：可读层（生成方法体 + 手写层中的 Java 语义代码）`from_any` = 0；剩余三处都是对象模型 / VM 负载的承载实现，归属如上。

