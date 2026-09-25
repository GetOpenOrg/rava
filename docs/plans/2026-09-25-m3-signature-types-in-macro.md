# M-3：方法签名类型决策进宏——试点方案

> 2026-09-25 · 关联：tasks.md #9（M-3，已授权改 `runtime/java_rta_macros`）、
> `docs/plans/2026-09-21-codegen-type-convergence.md` L1-b

## 一、现状（调研结论，路径相对仓库根）

- **Python 单一来源**：`emitted_method_sig_types`（`codegen/sig_types.py:374`，K-6）
  → `method_sig_types`（覆盖方法取最远祖先签名 + 实参替换）/ `constructor_sig_types`
  → `parse_method_param_types`（`sig_parse.py:292`）→ 逐位过滤后回落 `jvm_to_rust`（`type_map.py:60`）。
  消费方：`gen_method_body`（定义侧）、class_writer / inherited_gen / vtable_util（槽位与桥）、
  invoke_sig / invoke_virtual（调用侧）。**第二来源**：native 存根 `_gen_native_stub`
  （`emitter/method_gen.py`）自带一份过滤逻辑。
- **宏侧**：`block/generic_sig.rs` 有泛型签名解析，只服务 vtable 擦除（`virtual_dispatch.rs:95`）；
  descriptor 仅用于 stub panic 消息。现有映射与 Python 口径**不一致**（数组 `Vec<T>` vs `JArray<T>`、
  `C`→`char` vs `u16`、装箱类→基本类型 vs S-3.1 引用形态）。

## 二、决策输入分类——M-3 的上限

| 输入 | 例 | 宏可自行得到 |
|---|---|---|
| 基本类型 / 基本数组 | `I`→`i32`、`[J`→`JArray<i64>` | ✅ |
| String / Object 及其一维数组 | `JArray<String>` | ✅ |
| 类形参 / 方法级形参（含遮蔽规则） | `T`、`E` | ✅（类形参宏已有；遮蔽规则需移植） |
| 类是否在生成闭包（否则落 Object） | — | ❌ registry |
| 接口擦除 / 载体 `I<Object,..>` | carrier_type_positions.txt | ❌ registry |
| 被引用类的形参个数（补 `Name<Object,..>`） | effective_class_type_params | ❌ registry |
| prelude / 跨包同名消歧 | `_QUALIFIED_SHORT_NAMES` | ❌ 全局计算 |
| 覆盖方法取祖先签名 | method_sig_types | ❌ 继承链 |
| 构造器隐式形参（外部实例 / 匿名类转发） | — | ❌ |
| 手写 `_impl.rs` 的 vtable 签名 | _scan_vtable_impl_sigs | ❌ 跨文件 |

**结论**：不引入 registry 侧通道时，M-3 的上限是「纯位 + 形参 T」。引用类位置的决策要么留在
Python，要么由生成器产出 registry 摘要文件供宏读取（成本显著，另行评估）。

## 三、试点步骤

| 步 | 内容 | 验收 |
|---|---|---|
| M3-a | 宏新增 `block/desc_types.rs`：descriptor → Rust 类型（纯位：基本 / 基本数组 / String / Object 及一维数组），口径逐项对齐 `type_map.py:26-42`；顺带修正 `generic_sig.rs` 的 char / Vec / 装箱口径分叉 | 宏单测：映射表逐项对拍 Python `jvm_to_rust` |
| M3-b | **差分审计模式**（env 开关，默认关）：宏对每个 `#[java_method]` / native 存根，把纯位上的已发射类型 token 与 descriptor 推导比对（`same_type_tokens` 口径），计数 + 首例明细输出到 build 日志；零产物变化 | 验收集 27 例 + 语料 163 例审计零分歧；生成树逐字节一致 |
| M3-c | 定义侧 + 调用侧**同源**切换：static 且非覆盖、全部位置为纯位的方法，Python 只发属性与 `_` 占位，宏补全类型；调用侧仍按 Python 类型（纯位下两侧必然相同，M3-b 已证） | 生成树仅签名行变化；e2e 零回归；Python 侧该类方法不再调用 jvm_to_rust |
| M3-d | 类形参 T 位 + 方法级遮蔽规则移入宏 | 同上 |

## 四、风险

1. **K-6 token 全等契约**：vtable 擦除名单、调用侧、桥都依赖同一签名字符串；只能放纯位，且定义侧 /
   调用侧同源——M3-c 仅对非覆盖、非接口实现的方法生效。
2. 覆盖方法的签名来自祖先（描述符相同但祖先泛型签名更具体）→ 试点排除。
3. native 存根是第二来源，审计必须覆盖。
4. 超出纯位后需要 registry 侧通道，属于 Rust 重写（R0）的接口设计问题，本试点不做。
