# runtime / codegen 大文件拆分方案

> 日期：2026-09-20
> 范围：`runtime/` 与 `codegen/` 下 >500 行的手写源码（文档、报告、脚本目录不在范围内）
> 关联计划：[2026-09-18-block-rs-refactor.md](2026-09-18-block-rs-refactor.md)、[2026-09-18-p0-python-refactor.md](2026-09-18-p0-python-refactor.md)（本文是两者的收尾 + 补充，不推翻其设计）

---

## 一、现状盘点

>500 行文件共 15 个。对照既有计划，实际执行状态如下：

| 文件 | 行数 | 状态 |
|------|------|------|
| `runtime/java_rta_macros/src/block/mod.rs` | 2139 | Phase 1 已拆出（parse/util/rewrite/classify/generic_sig/class_init），**Phase 2（gen/）未执行**，`expand_inner` 仍为 ~1600 行单函数 |
| `codegen/type_map.py` | 1563 | 从未拆过，5 类职责混居 |
| `codegen/emitter/class_writer.py` | 1400 | p0 计划**部分执行**（struct_gen/method_gen/field_gen 等已拆出），`import_gen`/`clinit_extract` 未拆，`_gen_class_rs` 仍 ~1300 行含 8 个嵌套函数 |
| `codegen/instr/coerce.py` | 1010 | `_coerce_arg` 已并入（按计划），但混入了 ~800 行与"值强转"无关的层次/成员解析函数 |
| `codegen/method/blocks.py` | 908 | 从未拆过 |
| `codegen/classfile.py` | 866 | — |
| `codegen/transpile.py` | 819 | `_discover_jdk_classes_method_level` 为 ~520 行函数，内含 15 个嵌套闭包 |
| `codegen/instr/invoke_virtual.py` | 713 | p0 计划只做了整体搬移，`_gen_invokevirtual` 仍为 ~580 行单函数 |
| `codegen/instr/invoke.py` | 662 | p0 计划执行完毕，处于拆分后稳态 |
| `codegen/stack.py` | 621 | `_store_local` 单方法 ~293 行 |
| `codegen/method/vars.py` | 620 | `_hoist_if_vars` 单函数 ~277 行 |
| `codegen/emitter/project_writer.py` | 561 | — |
| `codegen/cfg/structure.py` | 538 | 前半是结构树构建，后半 ~270 行是独立的 simplify 化简通道 |
| `codegen/instr/invoke_sig.py` | 503 | p0 计划执行完毕的产物 |
| `runtime/java_rta_macros/src/try_macro.rs` | 502 | — |

`codegen/instr/sim/` 的拆分（sim.py 841 → 2 行 dispatch + 10 个子模块，全部 ≤350 行）已完成且 e2e 稳定，是本方案的可行性先例。

---

## 二、判定标准

**拆**，须同时满足：① 存在 >300 行的单函数，或文件内职责混居（改动一个语义特性要跨多个不相关段落）；② 能找到依赖单向的切分线；③ 拆分后每个文件有独立的、可一句话说清的职责。

**不拆**，满足任一：① 单一职责且贴线（500–670 行）；② 拆分会打散天然对应关系（格式解析器 ↔ 规范章节）；③ 已是前次拆分的稳态产物。

---

## 三、要拆的文件（8 个）

### 3.1 `block/mod.rs`（2139）— P0，执行既有计划

剩余工作 = [2026-09-18-block-rs-refactor.md](2026-09-18-block-rs-refactor.md) 的 Phase 2 + Phase 3，设计（GenContext 字段表、四阶段验收）以该文档为准，此处只补 Phase 1 之后新留在 mod.rs 里的两段：

| 现行段（mod.rs） | 去向 |
|------------------|------|
| L54–262：`expand_non_virtual_fn` + `erase_type`/`erase_signature`/`objectize_args`/`filtered_anc_impl_header` 等擦除辅助 | 新建 `block/erasure.rs`（~200 行） |
| L263–536：`expand_interface`/`expand_interface_impl`/`erased_wrapper_call` | 新建 `block/interface.rs`（~270 行） |
| L537–2139：`expand_inner`（~1600 行） | 按 Phase 2 拆入 `gen/{context, struct_layout, virtual_dispatch, wrapper, type_conversions}.rs` |
| mod.rs 本体 | 只留 `expand()` + `expand_inner` 调用序列，≤100 行 |

### 3.2 `codegen/type_map.py`（1563）— P1，按层次四分

当前 5 类职责自上而下天然分层，切分线即依赖方向（**单向：type_map → sig_parse → type_args → sig_types，禁止回环**）：

| 新文件 | 内容 | 行数 |
|--------|------|------|
| `type_map.py`（保留） | JVM→Rust 基础映射（`jvm_to_rust`/`sig_type`/`rust_default`/`is_jdk`/短名配置）+ descriptor 解析（`parse_descriptor_*`/`descriptor_to_suffix`/`mangle_name`） | ~300 |
| `sig_parse.py`（新） | 泛型签名递归下降解析：`_parse_type_list`/`_parse_type_args`/`_parse_one_type`/`_extract_method_tparam_bounds`/`parse_field_type`/`parse_class_type_params`/`parse_method_param_types`/`substitute_signature_type_vars` | ~500 |
| `type_args.py`（新） | 层次类型实参解析：`superclass_type_args`/`superinterface_type_args`/`ancestor_type_args`/`ancestor_vtable_args_by_short`/`effective_class_type_params`/`enclosing_*`/`outer_instance_*`/`implemented_interface_views`/`interface_signature_views` | ~450 |
| `sig_types.py`（新） | 方法/构造器签名类型 + RsType 构造：`constructor_sig_types`/`method_sig_types`/`hierarchy_overloaded_names`/`method_name_is_mangled`/`_rust_str_to_rs_type`/`jvm_to_rs_type`/`infer_type_args_from_declared` | ~300 |

调用方 import 路径机械替换到归属模块，不做 re-export 兼容层。

### 3.3 `codegen/emitter/class_writer.py`（1400）— P0，完成 p0 计划剩余步骤

= [2026-09-18-p0-python-refactor.md](2026-09-18-p0-python-refactor.md) §2 中未执行的第 4–6 步：

1. 新建 `import_gen.py`（~450）：提升嵌套函数 `_strip_generic`(L93)/`_add_desc_refs`(L141)/`_add_precise_import`(L299) 为模块级，连带引用收集 + cross_imports 生成段
2. 新建 `clinit_extract.py`（~130）：`<clinit>` 常量/数组提取
3. 嵌套的字段解析器 `_extract_type_names`/`_validate_field_type`/`_resolve_field_rust`/`_resolve_anc_field_rust`/`_outer_ref_field_rust` 并入既有 `field_gen.py`
4. `_gen_class_rs` 精简为调用序列（~250 行）

### 3.4 `codegen/instr/coerce.py`（1010）— P1，按"强转 / 层次查询 / 成员归属 / 命名"四分

现在 2/3 内容与"值强转"无关，是给 invoke/invoke_virtual/invoke_sig 共用的解析基础设施，属于 codegen 的共享解析层（与 `invoke_sig.py` 同级）：

| 新文件 | 内容 | 行数 |
|--------|------|------|
| `coerce.py`（保留） | 值强转：`_coerce_value`/`_coerce_to_object`/`_coerce_from_null`/`_to_i32`/`_float_lit`/`_escape_str` | ~250 |
| `hierarchy.py`（新） | 层次查询：`_is_subtype`/`_is_direct_subtype`/`_get_all_subtypes_ordered`/`_common_ref_type(+widening)`/`_super_path_to_class`/`_find_super_chain_to_class`/`_super_prefix_to_expr`/`_into_super_chain`/`_is_interface` | ~350 |
| `member_owner.py`（新） | 成员归属解析：`_resolve_method_owner`/`_resolve_special_method_owner`/`_resolve_static_method_owner`/`_resolve_static_field_owner`/`_find_method_super_prefix(+_for_type)`/`class_inherits_default_method`/`_resolve_interface_special_target`/`interface_special_member_name`；并吸收 §3.7 从 invoke_virtual 抽出的接收者类型实参解析段 | ~400 |
| `member_naming.py`（新） | 命名与 ref 解析：`_mangle_if_overloaded`/lambda 命名 + `_LambdaNameLedger`/`_bridge_call_target`/`_resolve_bridge_target`/`parse_method_ref`/`_parse_field_ref`/`_method_ref_*` | ~250 |

### 3.5 `codegen/transpile.py`（819）— P1，BFS 引擎独立

`_discover_jdk_classes_method_level`（L292–818）的 15 个嵌套闭包收拢为显式状态类：

- 新建 `codegen/callchain.py`（~520）：`CallChainDiscovery` 类，字段即闭包共享的 worklist/loaded/unresolved/static_fields/vm_roots，方法 = `_enqueue_*`/`_load_class`/`_process`/`_propagate_virtual_targets` 等
- `transpile.py` 保留（~300）：管线编排 `transpile()` + 引用收集 + 扫描报告

这是 CLAUDE.md 规则 2（调用链分析）的核心引擎，闭包簇是当前唯一无独立测试入口的段落。

### 3.6 `codegen/method/blocks.py`（908）— P2，抽纯函数，BlockSimulator 保留整体

类本身是内聚状态机不拆，只抽两簇自由函数：

- 新建 `method/unify.py`（~240）：L112–348 的 `jump_condition`/`unify_pair`/`unify_values`/`inline_temps`/`_same_entry`/`_same_stack`/`_parse_temp_let` 等
- 新建 `method/fusion.py`（~130）：窥孔融合三件套 `_try_fuse`/`_try_short_circuit`/`_try_ternary`/`_ternary_value` 改为接收 `sim: BlockSimulator` 的模块级函数
- `blocks.py` 保留 BlockSimulator 图构建/合并/派发（~540）

### 3.7 `codegen/instr/invoke_virtual.py`（713）— P1，函数分解为主

`_gen_invokevirtual`（L88–667，~580 行）按注释分段落分解为 5 个阶段函数（接收者解析与泛型实参映射 → 接口经具体类接收者 → bridge 识别 → 分派决策 → `_build_call` 组装），单函数 ≤150 行；其中接收者/类型实参解析段（~150 行）上移到 `member_owner.py`（§3.4）。文件本体 ~500 行。

### 3.8 `codegen/cfg/structure.py`（538）— P2，零风险二分

切分线是现成的：L272 起的 `_significant`/`completes_normally`/`_count_breaks`/`_retarget_breaks`/`_pass_tail`/`_pass_if`/`_pass_while`/`simplify`/`walk` 是结构树构建之后的独立化简通道，全部纯函数，整体平移到新文件 `codegen/cfg/simplify.py`（~270），`structure.py` 剩 ~270。

---

## 四、不拆的文件（7 个）

| 文件 | 行数 | 理由 |
|------|------|------|
| `runtime/java_rta_macros/src/try_macro.rs` | 502 | 单一职责（try/catch/loop-exit 宏展开），结构清晰（parse → rewriter → expander），贴线即止 |
| `codegen/instr/invoke_sig.py` | 503 | p0 拆分的稳态产物，纯函数签名查询层 |
| `codegen/instr/invoke.py` | 662 | p0 拆分后的 dispatch 层，最大函数 `_gen_invokespecial` 267 行；观察项：若再增长，按 ctor/super 两路分解 |
| `codegen/classfile.py` | 866 | **唯一行数豁免**。JVM 二进制格式解析器，与 JVMS 规范章节一一对应是它的内聚性所在；低改动率，按格式段拆会打散规范对应关系 |
| `codegen/stack.py` | 621 | 文件不拆（StackSim 是核心内聚状态）；但 `_store_local`（~293 行）分解为类型收敛/重声明判定/borrow 物化等 ≤150 行的阶段方法 |
| `codegen/method/vars.py` | 620 | 文件不拆（变量物化单一领域）；但 `_hoist_if_vars`（~277 行）分解为分支扫描/类型合并/提升发射三个 ≤150 行子步骤 |
| `codegen/emitter/project_writer.py` | 561 | 单一职责（Cargo workspace 发射），贴线保留；观察项 |

---

## 五、终态指标

| 指标 | 当前 | 终态 |
|------|------|------|
| runtime/ + codegen/ 最大文件 | 2139 | ≤ 600（唯一豁免 `classfile.py` 866） |
| >600 行文件数 | 8 | 0 |
| >300 行单函数 | 4 个（expand_inner ~1600、_gen_class_rs ~1300、_gen_invokevirtual ~580、_discover ~520） | 0 |
| 新拆函数行数上限 | — | ≤ 150 |
| 嵌套闭包簇 | 15 个（transpile） | 0 |
| `expand_inner` | ~1600 | ≤ 100（对齐 block-rs-refactor 指标） |
| string-based token match / dead code | 见 block-rs-refactor §八 | 0（随 Phase 2 一并达成） |

---

## 六、执行时序（与任务路线图对齐）

> 路线图见 [2026-09-19-remaining-issues.md](2026-09-19-remaining-issues.md)，# 编号沿用其任务序号。
> 原则：**拆分跟着"谁接下来要写这个文件"走——写到哪、拆到哪**；在途分支在改的文件绝不拆，没人碰的冷文件才允许提前拆。每个窗口都卡在"基线全绿 + 该文件最大修改者尚未开工"的节点。
> 不采用"全部任务做完再拆"：`block/mod.rs` 是 #3/#5/#7/#8/#10 五批任务的写入目标，每合入一批，最终拆分更大更险；也不采用"现在全拆"：A-1 在途 + type_map/coerce/emitter 是后续任务热区，提前拆完到开工时 diff 上下文已变，合入冲突照旧。

### 路线图任务 → 文件写入面（决定窗口位置）

| 路线图任务 | 主要写入文件 |
|---|---|
| #1/#2 A-1 vtable 去形参化（在途，`/tmp/wt-a1b`） | `block/mod.rs` |
| #3 G-11 `__new_with_super` | `block/mod.rs`（拆分后即 `gen/struct_layout.rs`） |
| #5 S-3.1 装箱真实对象化 | `type_map.py`、`coerce.py` 值强转半区、gen/ 派发 |
| #6 S-4 数组协变 | gen/、invoke 链 |
| #7 A-3 完整 IR 化 | `coerce.py`（吸收 `_reinstantiate_generic` 4 处发射点）、rs_ir |
| #8 A-7 / #10 A-5/A-6/A-4 | gen/、`invoke.py`/`invoke_virtual.py`、class_writer（lambda 类发射） |
| #11 G-1/G-2 变量提升迁 rs_ir | `stack.py`、`method/vars.py`、`method/blocks.py` |

### 窗口表（每步一个独立提交）

| 窗口 | 时点 | 拆什么 | 为什么是这里 |
|---|---|---|---|
| 0 | **现在**（#1 A-1 代理运行期间） | ① `transpile.py` → `callchain.py`（§3.5）② `cfg/structure.py` → `simplify.py`（§3.8） | 冷文件，路线图全程无人碰，零冲突，用掉等待时间。A-1 在途期间**不碰** block/mod.rs、type_map、coerce、emitter |
| 1 | #2 A-1 合入并过红线回归后，#3 G-11 与 #5 S-3.1 之前 | ③ `block/mod.rs`（§3.1：erasure/interface 抽出 → gen/ 五模块 + GenContext）④ `class_writer.py`（§3.3：clinit_extract → import_gen → 字段解析器并入 field_gen → 主函数调用序列化）⑤ `type_map.py` 四分（§3.2） | 刚合入 + 刚全绿 = 最理想基线；#3/#5/#8 的写入面全部落在拆分后的小模块上。#3 G-11 在 ③ 之后做，改 `gen/struct_layout.rs` |
| 2 | #7 A-3 开工前 | ⑥ `coerce.py` 拆分（§3.4：hierarchy / member_owner / member_naming）⑦ `invoke_virtual.py` 函数分解（§3.7，接收者解析并入 member_owner） | A-3 要吸收 coerce 的 `_reinstantiate_generic`，先拆再吸收免二次搬移；#5 S-3.1 对 coerce 的改动集中在值强转半区，该半区拆分后原地不动，不受影响 |
| 3 | #11 G-1/G-2 开工时（作为其第一步） | ⑧ `method/unify.py` + `method/fusion.py`（§3.6）⑨ `stack._store_local` 分解（§四）⑩ `vars._hoist_if_vars` 分解（§四） | G-2 本来就要迁变量提升逻辑，这三个文件的拆分就是它的第一步，不单独立项 |

净成本 ≈ 3 个天级窗口 + 窗口 0 顺手项，换取 #3/#5/#7/#8/#10/#11 全部在 ≤600 行、单函数 ≤150 行的模块上开发，且拆分成本不再随任务增长。

---

## 七、验收

- 每步：`cargo check`（Rust）/ `python3 -m py_compile codegen/**/*.py runtime`（Python）通过
- 基线：改造前 `python3 scripts/run_tests.py --clean` 全量跑一遍存日志；每步后 `--filter` 定向重跑受影响测试；每个窗口完成后全量对比基线，通过率不得退步
- 纯搬移步骤（窗口 0、1、2 全部，及窗口 3 的 ⑧）零逻辑改动，diff 只允许 import 行与函数签名；⑨⑩ 为方法内部分解，diff 允许阶段函数抽取
- 终态：`wc -l` 全目录核对 §五 指标表
