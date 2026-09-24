# TypeIR 剩余位点调研（清单第 5 项：type_surgery 余 10 处）

> 日期：2026-09-24　性质：只读调研（未改代码、未跑 e2e/cargo）
> 基线：`630c6f3`（分支 claude/jolly-dijkstra-diftum，含 `8c8ba84` TypeIR 批次 3 合入）
> 关联：[收敛路线图](../plans/2026-09-21-codegen-type-convergence.md) §二/§三 L1-a、[远期路线图](../plans/2026-09-23-long-term-roadmap.md) §三 S2/S3/S4、[tasks.md](../tasks.md) 第 14/45/47/81/90 行

## 结论摘要

1. `type_surgery_sites=10` 由 `codegen/raw_audit.py:43` 计算：扫描 `codegen/**/*.py` 的全文，匹配两个正则。**其中 1 处是误报**（`raw_audit.py:10` 模块 docstring 里写着 `startswith('JArray<')`），**真实位点 9 处**，分布在 6 个文件：coerce 2、vars 3、hierarchy 1、invoke 1、member_owner 1、render 1。
2. 9 处中有 8 处可以直接用现有 TypeIR API 替换，语义不变。可用的 API 有 `from_rust_type` / `rust_head_name` / `strict_erased_subtype`（批次 3 新增），以及 `stack.erased_class_of` / `is_jvm_array`（批次 2 新增）。`render.py:171` 有两种做法：最小改法可以直接换；要彻底收口，得先把 `CastExpr.target` 从字符串改成类型对象（IR 结构改动，和 M-3 有交集）。**宏侧（java_rta_macros）不需要改任何地方**，9 处只是使用宏生成的 `From<X> for Object` 和 `.into()` 上转链这些已有约定。
3. 建议按三批做：A 批是 Object 边界族 4 处（coerce×2 + vars:50 + hierarchy:175）；B 批是 invoke 残余 2 处（invoke:814 + member_owner:506）；C 批是 G-3 两处（vars:272/273），**在窗口 3 开工前做，或者直接作为窗口 3 的第 0 步**。render:171 单独做成 D 批，排在 CastExpr 升维或 M-3 试点里。四批合计大约 +60/−15 行，外加单测。
4. **计数口径有盲区**：还有约 25 处同类文本解剖没被计进去，包括 `split('<', 1)[0]`、`partition('<')`、`startswith('JArray')`（不带 `<`）、`startswith('Vec<'/'Rc<')`、`find/index('<')`。其中 invoke_sig 3 处、invoke 4 处还在批次 3 已经"收口"的 invoke 域里。9 处清零后计数会是 0（修掉误报后），但 L1 其实还没做完，需要扩大计数口径。
5. 依赖关系：C 批和窗口 3（第 7 项）的 vars.py 写入面重叠，必须先后串行；D 批和 M-3 或 CastExpr 升维交织；和 P-1（第 8 项）只是同文件，没有硬依赖；R0（第 10 项）的门槛里没有 type_surgery=0，但 jvm_type 是 Rust `ty` crate 的规格本体，建议在 R0 前清零（按扩大后的口径）。另外，S2 的退出判据"27→个位数"按真实口径（9）**已经达到**。

---

## 一、计数口径核实

### 1.1 计数代码

`codegen/raw_audit.py:25-28, 43-56`：

```python
_SURGERY_PATTERNS = (
    re.compile(r"""\.split\(['"]<['"]\)\[0\]"""),
    re.compile(r"""startswith\(['"]JArray<"""),
)
def type_surgery_sites() -> int:   # rglob codegen/*.py 全文（含注释 / docstring），逐文件 findall 累加
```

`scripts/main.py:273` 在转译后打印 `[raw-audit] ... type_surgery_sites=N`，`scripts/run_tests.py:555-570` 负责汇总。这个口径是**静态源码卫生**，和具体跑哪个测试无关。

### 1.2 实测（本次用 importlib 单独加载 raw_audit.py 执行，结果 = 10）

| # | 文件:行 | 命中形态 | 性质 |
|---|---|---|---|
| — | `codegen/raw_audit.py:10` | `startswith('JArray<` | **误报**：模块 docstring 自述（`split('<')[0]` 那一处因为没有前导 `.` 没被匹配到） |
| S1 | `codegen/instr/coerce.py:99` | `startswith('JArray<` | 真实位点 |
| S2 | `codegen/instr/coerce.py:104` | `.split('<')[0]` | 真实位点 |
| S3 | `codegen/method/vars.py:50` | `.split('<')[0]` | 真实位点 |
| S4 | `codegen/method/vars.py:272` | `.split('<')[0]` | 真实位点（G-3 `7f94f65` 新增） |
| S5 | `codegen/method/vars.py:273` | `.split('<')[0]` | 真实位点（G-3 `7f94f65` 新增） |
| S6 | `codegen/instr/hierarchy.py:175` | `.split('<')[0]` | 真实位点 |
| S7 | `codegen/instr/invoke.py:814` | `.split('<')[0]` | 真实位点 |
| S8 | `codegen/instr/member_owner.py:506` | `.split('<')[0]` | 真实位点 |
| S9 | `codegen/render.py:171` | `startswith('JArray<` | 真实位点 |

数字演变：69 →（批次 2）27 →（G-3 在 vars.py 新增 2 处）29 →（批次 3，`8c8ba84` 合入说明写的 "29→10"）10。

---

## 二、逐位点清单

说明：「现有 API」一栏只用已经合入的查询入口。`jvm_type.py` 提供 `from_rust_type`、`rust_head_name`、`strict_erased_subtype`、`JvmType.erasure/is_subtype_of`、变体 `ClassRef/Array/Primitive`；`stack.py` 提供 `erased_base`、`erased_class_of`、`is_jvm_array`。

### S1 + S2　`instr/coerce.py:99, 104` — `_coerce_to_object(val_str, ty, registry, class_type_params, clone)`

- **现状手术**：`ty.startswith('JArray<')` 判断是不是数组，是就发射 `Object::from(src)`；`_registry_short_index(registry).get(ty.split('<')[0].strip())` 判断是不是 registry 里的类，是就发射 `Object::from(src)`，否则 `Object::from_any(src)`。
- **消费 / 产出**：消费值的 Rust 类型串 `ty`（全形态，37 个调用点，是 Object 装箱的公共入口）；产出三种装箱发射形态之一。
- **对应 TypeIR 查询**：`t = from_rust_type(ty, registry)` 之后判断：`isinstance(t, Array)` 走数组臂；`isinstance(t, ClassRef) and t.binary in registry` 走类或接口臂（等价于短名索引命中，因为 `from_rust_type` 内部就是经 `_registry_short_index` 把名字反查成 binary）。也可以直接用批次 2 的 `is_jvm_array(ty)` 和 `erased_class_of(ty, registry) is not None`。
- **等价性注意**：`from_rust_type` 会剥掉前导 `&`，旧路径不剥。如果有 `&JArray<..>` 或 `&Foo` 流进来，旧路径走 `from_any`，新路径走 `Object::from`。需要插桩双算来确认调用面上不会出现 `&` 形态。同函数第 88 行的基本类型元组判定、第 95 行的 `class_type_params` 成员判定也是字符串判定（不在计数内），可以顺手改成 `Primitive` 判定。类型变量那一支需要 TypeIR 补一项能力，见 §三 G2。
- **改动量**：约 +8/−4 行。**风险中等**：调用扇出最大（37 处），但决策只有三支。

### S3　`method/vars.py:50` — `_coerce_acmp_operand(expr_str, ty_node, registry, class_type_params)`

- **现状手术**：先 `render_type(ty_node)`，再对渲染结果 `split('<')[0]`，然后查短名索引；命中就发射 `Object::from(Clone::clone(&x))`（保持对象身份），否则落到 `_coerce_to_object`。
- **消费 / 产出**：消费 `if_acmp` 两个操作数的 RsType 节点（调用方是 `method/blocks.py:136-137`）；产出引用相等比较的上转发射。
- **对应 TypeIR 查询**：`erased_class_of(ty, registry) is not None`，或 `isinstance(from_rust_type(ty, registry), ClassRef)` 加上 in-registry 判定。更好的做法是直接从 `ty_node` 构造（绕开"渲染成字符串再解析回来"这一圈），这需要补 G1。
- **改动量**：约 +3/−2 行。**风险低**。和 S1/S2 的 in-registry 判定是同一口径，应该一起改。

### S4 + S5　`method/vars.py:272-273` — `_forms_alignable(later_ty_s, hoisted_ty_s, registry)`

- **现状手术**：两个类型串分别取头部，任何一边在 `PRIMITIVE_RUST_TYPES` 里就返回 False；否则返回 `hierarchy._is_subtype(later_base, hoisted_base, registry)`。
- **消费 / 产出**：消费同一槽位两次 store 的渲染类型串（G-3 三重复用槽的分型判定；调用点在 `vars.py:698/727/810`）；产出"合并进提升绑定（加 `.into()`）"还是"拆成独立绑定"的决策。
- **对应 TypeIR 查询**：`a, h = from_rust_type(later, reg), from_rust_type(hoisted, reg)`；`isinstance(a|h, Primitive)` 时返回 False；否则 `strict_erased_subtype(a, h, reg)`。批次 3 的 S5 已经证明 `strict_erased_subtype` 和 `_is_subtype` 逐点一致（不自反、排除 Object、actual 必须在 registry 域内）。
- **等价性注意**：`u8/u32/u64/usize` 在 `PRIMITIVE_RUST_TYPES` 里，但不是 JVM 基本类型，`from_rust_type` 会把它们解析成 `ClassRef('usize')` 这样的占位。不过占位不在 registry 里，`strict_erased_subtype` 也会返回 False，所以**最终结果一样**。如果想让语义写得明确，可以补 G3。
- **改动量**：约 +5/−4 行。**风险中等**：代码逻辑简单，但它是 G-3 刚合入的承重判定（CHM transfer 吞节点、ZonedDateTime/FilesApi E0308 的根治都依赖它）。

### S6　`instr/hierarchy.py:175` — `_is_interface(rust_short, registry)`

- **现状手术**：`split('<')[0]` 后查短名索引，返回 `ci.is_interface`。
- **消费 / 产出**：唯一调用方是 `_common_ref_type_widening`（hierarchy.py:161），传进来的是 `_common_ref_type` 返回的裸短名；产出"公共祖先是接口就放弃 widening"的判定。
- **对应 TypeIR 查询**：`t = from_rust_type(rust_short, registry)`，判断 `isinstance(t, ClassRef) and t.is_interface`（`from_rust_type` 已经从 ci 补全了 is_interface；占位恒为 False，和旧行为一致）。
- **改动量**：约 +3/−3 行。**风险最低**。同函数第 147 行的 `_split` 用了 `t.partition('<')`（不在计数内），它要把实参串重新拼回去，属于 G4 的能力缺口，本批不动。

### S7　`instr/invoke.py:814` — `_gen_invokestatic`（祖先精化后的擦除返回同步）

- **现状手术**：`rust_ret.split('<')[0].strip() == short_cls(_cp_cls_bin)`，判断擦除返回类型是不是被调类自身（`Enum.valueOf` 返回 `Enum<Object>`），是就把 `_sig_ret_s` 同步成精化后的实例化。
- **消费 / 产出**：消费 `jvm_to_rust(ret)` 的擦除返回串；产出调用点的返回类型标注。
- **对应 TypeIR 查询**：沿用批次 3 V4 的样板，`rust_head_name(from_rust_type(rust_ret, registry).erasure()) == short_cls(_cp_cls_bin)`。**不要**改成 `ClassRef.binary == _cp_cls_bin`：如果被调类不在 registry 域内，`from_rust_type` 给出的是短名占位，和 binary 比较会出现分歧。
- **改动量**：约 +3/−1 行。**风险低**：这是批次 3 的漏网位点，形态和 V4 完全一样。

### S8　`instr/member_owner.py:506` — `_resolve_virtual_sig_params(sim, cls, mname, params, ret, ...)`

- **现状手术**：`_recv_base_v = _recv_ty.split('<')[0].strip()`，后面拿它做 `!= cls` 比较、经 `_rust_type_to_binary` 查 binary（2 次）、作为参数传给 `_lookup_method_sig_params`。
- **消费 / 产出**：消费接收者渲染类型（已经过类型变量上界视图、菱形闭合处理）；产出"接口方法经具体类接收者调用"和"桥接描述符"两级策略里的接收者身份。
- **对应 TypeIR 查询**：`_recv_t = from_rust_type(_recv_ty, registry)`；`_recv_base_v = rust_head_name(_recv_t.erasure())`，这是逐输入一致的最小替换。可以再深一步，用 `_recv_t.binary` 替代下游两次 `_rust_type_to_binary(_recv_base_v)` 字符串反查，属于可选改动，会多一轮等价核验。
- **等价性注意**：`RsNamed.path` 目前在全库没有任何调用点设置（已 grep 确认），所以 `render_type` 不会产出 `a::b::C` 这种带路径的形态；`from_rust_type` 碰到 `::` 只取第一段这一分歧点目前不会触发。
- **改动量**：约 +3/−1 行（最小）或 +8/−6 行（深一步）。**风险中等**：这是虚调用签名解析的主干。同文件第 451/461 行 `_close_open_type_args` 里的 `split('<', 1)` 不在计数内，需要拆实参并改写，属于 G4。

### S9　`render.py:171` — `render_cast(c: CastExpr)`（checked 数组目标臂）

- **现状手术**：`c.target.startswith('JArray<') and c.target.endswith('>')` 之后，用切片 `c.target[len('JArray<'):-1]` 取元素类型串，发射 `try_cast_array::<elem>("binary")`。
- **消费 / 产出**：消费 `CastExpr.target: str`（rs_ir.py:216）。收敛路线图 §六评审采纳 4 已经点名这是"IR 里藏字符串"；产出 checkcast 的数组元素 turbofish（运行时约定在 `runtime/java_runtime/src/java/lang/object_ext.rs` 的 `try_cast_array`，属于手写层，不是宏）。
- **对应 TypeIR 查询**：
  - 最小改法：`isinstance(from_rust_type(c.target), Array)` 判定形态（render 层没有 registry，但判断 Array 不需要 registry），元素串用 `type_args.split_rust_type_args(c.target)[0]` 取（那是指定的语法拆分器，属于解析层）。
  - 彻底改法：checked 的 CastExpr 全部在 `instr/sim/control.py:79-115` 构造，那里的 `binary_name`（也就是 `comment`，数组形如 `[L...;`）可以直接用 `from_descriptor` 得到 `Array`。由构造方把类型对象和元素 Rust 串写进节点，render 只按节点分派。这就是 "CastExpr.target 族升维"，需要改 rs_ir 结构。
- **改动量**：最小 +4/−3 行；彻底改法 +25/−10 行（涉及 rs_ir、control 4 个构造点、render）。**风险**：最小改法低；彻底改法中等（control.py:63 对 `cast_rust` 有 `replace` 后处理，元素串必须在后处理之后再取）。

---

## 三、分类

| 类别 | 位点 | 说明 |
|---|---|---|
| **可以直接替换**（现有 API 足够，语义不变） | S1 S2 S3 S6 S7 S8 S4 S5 | 8 处。S4/S5 虽然可以直接替换，但它们在窗口 3 的写入面里，排期要协调（见 §五） |
| **最小改法可直接替换，彻底收口需要先升维 IR 或 TypeIR** | S9 | 需要 CastExpr 的类型载荷升维；和 M-3 有交集（JvmType→Rust 串的渲染权归谁） |
| **牵涉宏侧（禁改域）** | 无 | 9 处都只是**使用**宏已有的约定：`From<Wrapper> for Object`（`java_rta_macros/src/block/interface.rs:179`、`gen/type_conversions.rs`）和 `.into()` 上转链。语义不变的前提下宏侧零改动。只要 S1/S2/S3 的装箱分支真的发生变化（例如 `&` 形态从 `from_any` 翻到 `Object::from`），就会触碰宏的 From 约定；这属于**必须由双算拦截的分歧**，不在本工作的合法范围内 |

**TypeIR 能力缺口（都不阻塞 9 处的最小替换，是"完全体"要补的）**：

- **G1 RsType→JvmType 桥**：新增 `from_rs_type(node, registry)`。S3、S8 以及大量 `render_type(node)` 后再解析的位点，现在都是"节点→文本→对象"绕一圈。
- **G2 作用域内类型变量**：`from_rust_type` 不认识类型形参，会把 `T` 当成 `ClassRef('T')` 占位。S1 第 95 行的 `ty in class_type_params`、`invoke_sig.py:369` 等位点都要自己判断。建议加参数 `tparams=`，让它产出 `TypeVar`。
- **G3 Rust 宿主基本类型**：`u8/u32/u64/usize` 没有对应的变体，现在是占位 ClassRef。建议在 `from_rust_type` 里单独标注（比如加一个 `HostPrim` 变体或谓词），这样 S4/S5 这类判定就不用依赖"占位不在 registry 内"这个巧合。
- **G4 带实参的 JvmType→Rust 串回渲染**：`hierarchy.py:147`、`member_owner.py:451/461`、`invoke.py:150-152` 都需要"拆开实参→改写→重新拼回"。这正是 `type_map.jvm_to_rust` 的职责，而 M-3 要把这部分决策移进宏。**在 M-3 方向定下来之前不建议在 Python 侧新建渲染器**，G4 类位点可以留到 M-3 试点时一起处理。
- **G5 双查询面合一**：批次 2 的 `stack.erased_base/erased_class_of/is_jvm_array`（用正则取首个标识符）和批次 3 的 `jvm_type.from_rust_type/rust_head_name` 并存，两者对 `&X`、`()`、`a::b` 的处理细节不一样。完全体应该收成以 jvm_type 为唯一实现，stack 那一侧改成薄转发。

---

## 四、迁移顺序建议

| 批 | 位点 | 依赖 | 风险 | 预估 | 计数变化 |
|---|---|---|---|---|---|
| **A：Object 边界族** | S1 S2 S3 S6 | 无（API 已有）；建议同批加 in-registry 谓词 `class_ref_in_registry(t, reg)` 或者复用 `erased_class_of`，四处共用 | 中（S1/S2 扇出 37 个调用点） | +20/−13，外加约 6 个单测 | 10→6 |
| **B：invoke 残余** | S7 S8 | 无；沿用批次 3 V4 样板 | 中低 | +6/−2（最小） | 6→4 |
| **C：G-3 分型判定** | S4 S5 | **和窗口 3 串行**（vars.py 写入面，拆分方案 §3.6/§四 ⑩ `_hoist_if_vars` 分解会搬动这段代码） | 中（G-3 承重） | +5/−4 | 4→2 |
| **D：CastExpr 升维** | S9 | 最小改法可以随 A 批一起做；彻底改法排在 M-3 试点或 L3 转换族 IR 化 | 低 / 中 | +4/−3 或 +25/−10 | 2→1 |
| **E：口径修正** | raw_audit 误报 + 扩大口径 | 必须在 A 批之前或同时落地，否则最后留下的那个"1"永远清不掉 | 低 | +10/−3 | 1→0，然后按新口径重新定基线（约 25） |

**可以合并的批次**：A+B 可以合成一个 PR（都是 instr 层的查询替换，同一套验证流程，合计约 +26/−15）；D 的最小改法可以一起放进去。C 必须单独做，并和窗口 3 协调。E 的口径修正建议作为 A 批的第 0 个提交，这样"逐步下降"的曲线从一开始就是真实数字。

**扩大口径后的遗留清单**（下一轮 L1 余量，约 25 处，按文件）：invoke 6（150/151/152/236/458/814 中除 S7 以外的）、invoke_sig 3（266/292/368）、member_owner 2（451/461）、coerce 1（141 `_is_cross_instantiation`）、invoke_virtual 1（359）、hierarchy 1（147）、sim/arrays 3（43/146/252 `Vec<`）、sim/fields 1（237 `Rc<`）、stack 1（109 `Rc<`）、emitter/dispatch_gen 2（158/178 `startswith('JArray')`）、emitter/interface_gen 1（107）、emitter/import_gen 1（37）。另外 `sig_types.py:414/465` 和 `type_args.py:237` 是解析器本体，属于合法的文本解析层，建议扩大口径时用白名单排除。

---

## 五、验证方案

### 5.1 通用验收口径（每批都要满足，沿用批次 2/3 的四重验证）

1. **单测**：扩充 `tests/unit/test_jvm_type.py` 和 `tests/unit/test_erased_queries.py`，每个替换谓词都做一个"旧文本解剖 vs 新类型查询"的逐形对拍表，至少覆盖这些形态：裸名、`X<A, B<C>>`、`JArray<E>`、`JArray<JArray<E>>`、`Object`、基本类型、`usize`、`()`、类型变量 `T`、registry 域外短名、空串、`&X`。
2. **插桩双算**：批次 3 的做法，在转译期同时计算新旧谓词，一旦分歧就记录下来；跑定向测试集，要求**分歧数 = 0**。重点要拦住的是 S1/S2/S3 的 `&` 形态分歧和 S8 的 `::` 形态分歧。
3. **生成树逐字节一致**：改动前后用同一个种子生成 `build/jdk21/<test>/` 生成树，`diff -r` 结果必须为 0。
4. **双种子 diff 归零**：`PYTHONHASHSEED` 取两个不同值各跑一次，生成树的 diff 必须为 0（G-4 确定性）。
5. **仪表**：`[raw-audit] type_surgery_sites` 按 §四 表中的数字下降；`raw_expr/raw_stmt` 和 `[readability-audit]`、`[equiv-audit]` 在同一测试上逐位不变。

### 5.2 各批定向回归集（本地纪律：每批 ≤10 例）

| 批 | 测试（tests/e2e 下） | 选取理由 |
|---|---|---|
| A | TestCasting、TestArrayCovariance、TestMultiRefArray、TestEqualsHashCode、TestTernary、TestHashMapOps、TestInterfaces、TestAutoboxing、TestGenerics、TestStreamCollectors | 数组和类的装箱、`if_acmp` 引用相等、三目运算公共祖先 widening、接口载体装箱、自动装箱、集合装箱的热路径 |
| B | TestEnumBasic（`valueOf` 正是 S7 注释里的样本）、TestEnumSetMap、TestHashMapOps（`Map m = new HashMap`）、TestPriorityQueue、TestArrayList、TestCollectionsUtil、TestBridgeMethod、TestGenericMethod、TestBoundedGenerics、TestSequencedCollections | 静态祖先精化、接口方法经具体类接收者调用、桥接描述符解析、类型变量接收者上界视图 |
| C | TestHoistShadow、TestChmTransfer、TestZonedDateTime、TestFilesApi、TestDateTimeFormat、TestNestedTry、TestOptionalFull、TestSuppressed、TestSynchronized、TestArrayList | G-3 和 vars-hoist-v3 的回归集原样沿用（同槽多形态、监视对象槽、catch 形参槽、提升遮蔽） |
| D | TestArrayCovariance、TestMultiArray、TestArraysUtil、TestArrayCopy、TestCasting、TestMultiRefArray、TestCollectionFactory、TestStreamAdvanced | 数组 checkcast（`(E[]) Arrays.copyOf`、`toArray`、多维数组） |

A+B 合批时取两个集合的并集，去重后是 17 例，超出本地 10 例的上限。可以分两轮跑，也可以把逐字节 diff 交给服务器全量轮（169 语料的生成树 diff=0 是批次 3 用过的最强口径）。

---

## 六、与第 7、8、10 项的依赖关系

| 项 | 关系 | 结论 |
|---|---|---|
| **第 7 项 窗口 3**（G-1/G-2 迁 rs_ir；也是 R0 门槛②的剩余部分） | S3/S4/S5 位于 `method/vars.py`，正在窗口 3 的写入面里（拆分方案 §3.6 的 ⑧ `method/unify.py`+`fusion.py`、⑩ `_hoist_if_vars` 分解）。窗口 3 要把变量提升迁到 rs_ir，迁完以后 `_forms_alignable` 的输入就是节点（RsType），不再是字符串 | **C 批要么在窗口 3 之前以最小替换落地，要么作为窗口 3 的第 0 步**，不能两边同时改。推荐先做：这样窗口 3 迁移时拿到的是类型化的谓词 `forms_alignable(JvmType, JvmType)`，直接用 G1（`from_rs_type`）接上节点即可。S3 同理，可以随 A 批提前做完，窗口 3 只负责搬移 |
| **第 8 项 P-1**（Python 里的 JDK 类名字面量清零） | 9 处位点都没有引入 JDK 类名字面量（`Object` 走 `constants.OBJECT_CLASS`，`JArray` 是运行时名）。只是 coerce.py 同时也是 P-1 已知位点（remaining-issues §P-1 第 4 条），存在同文件写冲突 | **没有硬依赖**，协调好写入域即可。有一点协同：P-1 的"改成从 .class 动态解析"需要类型身份查询，TypeIR 的 `ClassRef.binary` 身份正好可以替代字面量表的一部分用途 |
| **第 10 项 R0**（Rust 重写启动） | R0 门槛写的是"窗口 0–3 + P-1 + 门槛①"，**type_surgery=0 不是硬门槛**。但收敛路线图 §四明确说 L1 是在给重写"写规格"，jvm_type.py 就是 `ty` crate 的规格本体。如果不清零，将来移植 coerce、vars、invoke 模块时就得原样搬运这些字符串手术，或者当场重新设计 | 建议把"type_surgery（扩大后的口径）=0 且 G5 双查询面合一"列为 S4 重写预备期的软门槛。G2/G3 是 `ty::Ty` 变体设计的直接输入，应该在 R0 之前定型。G4 交给 M-3 决定，不在 Python 侧投资 |
| 附：**M-3 试点** | S9 的彻底改法和 G4 都会碰到"JvmType→Rust 类型串谁来渲染"的问题 | D 批的彻底改法和 G4 类位点放进 M-3 试点一起设计；A/B/C 批和 M-3 互不影响，可以先做 |

---

## 附：关键路径索引

- 计数：`codegen/raw_audit.py:25-56`、`scripts/main.py:273`、`scripts/run_tests.py:555-570`
- TypeIR API：`codegen/jvm_type.py:261`（from_rust_type）、`:393`（rust_head_name）、`:415`（strict_erased_subtype）、`codegen/stack.py:71/80/93`（erased_base / erased_class_of / is_jvm_array）
- 批次 3 样板：`6ca2594`（V4 rust_head_name）、`5d797ef`（S5 strict_erased_subtype）、`a87f66c`（V1 Array 判定）
- 单测：`tests/unit/test_jvm_type.py`、`tests/unit/test_erased_queries.py`
