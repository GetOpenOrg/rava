# input golden 对照差异

采集：`python3 scripts/golden/dump_input.py <Test.java>` → `build/golden/input/<Test>.jsonl`；
对照：`cargo test -p input --test golden`（在 `generator/` 下，`--target-dir ../build/gen-target`）。

| 用例 | 注册表类数 | 记录数 | 细项数 | 失配 |
|---|---|---|---|---|
| TestHashMapOps | 264 | 515（+1 手写类集合） | 16296 | 0 |
| TestStreamBasic | 399 | 688（+1 手写类集合） | 21330 | 0 |

细项的计法：
- 集合 / 列表记录：每个元素计一项。
- `plan` 记录：每个方法判定计一项，另加类级一项。
- `code` 记录：每条指令计一项。
- 字典记录：值的元素数之和。

## 采集口径

两侧输入相同：
- 同一类路径：用户类目录 → JDK jmods → 镜像类目录；
- 同一 `closure.json`；
- 同一 `runtime/java_runtime` 清单。

折叠与 VM 常量剪枝按真实流程执行。`code` 只采集被折叠或剪枝改动过的方法体，`normalized_keys` 记录这些方法的键集。

采集的记录种类：
- 注册表插入序、用户 / lib / JDK 类划分；
- 调用链 visited、字段存根；
- 反射面三项；
- 资源束 / 注解枚举 / JCA / 模块资源补种；
- 预检 visited；
- 清单投影；
- 手写扫描（方法集、核心签名、接口 vtable 签名）；
- 规范化方法体；
- 逐方法发射判定。

## 设计性偏差（golden 输入面不触发，登记备查）

1. **lib 类参与折叠**。Rust 对注册表里的全部类施加折叠与剪枝；Python 不折叠 lib crate 的类。本用例集没有 lib crate，所以不触发。
2. **资源束判定的类路径不同**。纯数据资源束判定（`Boundary::is_data_bundle`）在原始类路径上装载类，排除用户档案；Python 经 registry / JDK 装载器取类。两侧的判据结构相同（复用闭包的 `Carriers`）。
3. **手写扫描的数据源不同**。Rust 直接扫描 `runtime/java_runtime/src` 真源；Python 扫描 overlay 后的 scratch。采集脚本用软链让 Python 也扫描真源，所以两侧口径一致。
4. **`str.isidentifier` 只做近似**。反射字段名判定时，Rust 按 ASCII 字母、数字、下划线和 ≥0x80 字节判定；非 ASCII 标识符的边界可能与 Python 不同。
5. **不建模翻译失败回退**。Python 在发射时如果单个方法翻译失败，会回退为存根；这是 P4/P5 的职责，Rust 的判定层只给出 `Verdict::Bytecode`。
6. **root 方法不经 prefer_major 选类**。root 方法直接从类路径取，不走 Python 的 `prefer_major` 多版本选择。本用例集中没有多版本类。
7. **接口 lambda 实现的 Rust 名**。Rust 直接取 `safe_ident(name)`；Python 经 `lambda_impl_rust_name`。本用例集的判定全等。
8. **接口 default 方法有无方法体**。Rust 按规范化后的代码判定；Python 按折叠后的 `m.instrs` 判定，两者同源。
9. **F / D 常量折叠未移植**。闭包分析器目前不产出 float / double 折叠常量。根因是 `FoldValue`（`facts.rs`）没有
   Float / Double 变体，`parse_fold_value` 对 JSON 数字一律按 `as_i64` 解码：
   - F / D 的非整数值（如 `1.5`）在 `facts.rs` 报 `Format("整数常量越界")`，到不了 `norm.rs` 的「未移植」分支；
   - F / D 的整数值被解码为 `Int`，J / F / D 的 JSON 字符串（如 `"NaN"`）或布尔值才落入 `norm.rs` 的
     `未移植：{ty} 常量编码`；其中 J 类型的布尔 / 非数字字符串实为格式错误，同样被报成「未移植」。
   - 终态：`FoldValue` 增加 Float / Double 变体、`parse_fold_value` 按 `ty` 解码、`norm.rs` 发射 `ldc` / `ldc2_w`，
     J 的布尔 / 字符串值归格式错误。
