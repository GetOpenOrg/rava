# ty golden 对照差异

采集：`python3 scripts/golden/dump_ty.py <Test.java>` → `build/golden/ty/<Test>.jsonl`；
对照：`cargo test -p ty --test golden`（在 `generator/` 下，`--target-dir ../build/gen-target`）。

| 用例 | 注册表类数 | 记录数 | 失配 |
|---|---|---|---|
| TestHashMapOps | 264 | 63801 | 0 |
| TestStreamBasic | 399 | 79175 | 0 |

记录覆盖 36 个函数入口：类级、方法级、字段级的类型层查询，以及 JvmType 的构造、子类型、擦除、显示和 RsType 桥。

## 采集口径（两侧同一输入）

- 折叠（`closure_folds`）与 VM 常量剪枝在采集脚本中置为恒等。两侧比较的是同一份原始字节码；接入折叠后需补对照。
- Python 的 `registry=None` 模式不移植，不采集。

## 设计性偏差（golden 输入面不触发，登记备查）

1. `substitute_type_params` 在 Python 中用正则做文本替换，Rust 用 `RsType::substitute`，只按 `Param` 节点结构替换。与类短名同名的形参不会被误替换。
2. `_closure_hit`（按短名命中占位目标）不移植。子类型判定以 binary 为唯一身份。
3. `from_rust_type`（文本解析）改为 `from_rs_type`（结构输入）。
   - 畸形文本降级路径不再存在。
   - `?` 实参在 RsType 中没有对应节点。
4. 短名反查（Python 的 `_registry_short_index`）保留插入序、同名后者胜出的语义，但只用于 `Param` 或 `Bare` 这类文本头名。`Class` 节点直接携带 binary，不经反查。因此不会复现 Python 的同短名误解析（例如用户类 `Node` 与 `java/util/stream/Node`）。
5. 标识符字符判定按字节处理，≥0x80 的字节视为字母。Python 的 `str.isalpha` 与 `\w` 按 Unicode 判定，非 ASCII 标识符的边界可能不同。
6. `class_type_param_bounds` 与 `ancestor_vtable_args_by_short` 返回 BTreeMap，按键字典序，而 Python 的 dict 是形参声明序或插入序。对照按映射比较；消费方若依赖声明序，需自行按 `effective_class_type_params` 排序。
