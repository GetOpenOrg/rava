# 生成二进制体积：剖析与终态方案（2026-10-04）

> 2026-10-04 用户把本线交给主会话统一安排推进（另一会话完成剖析后不再实施）。
> 闭包目标「正确且最小」的判据之一是二进制小（CLAUDE.md 第 2 条、`feedback_closure_goal_correct_minimal`）。

## 一、剖析（`python3 scripts/run_tests.py --filter HelloWorld --release`）

剖析时的树早于 main d840bb83（S7-1 已删掉按类的 `is_instance_of` / `__view_into`）。B0 在当前 main 上重测，数字以重测为准。

**流程与耗时**
- 转译 1.19 s：闭包 469 类（翻译 458、boundary 9），可达方法 1813，实例化 225。
- 发射 5 个 crate（java_runtime / java_meta / java_body_1 / java_body_2 / user），1087 个文件，约 20.9 万行。
- cargo build 1 min 15 s。release 档位（`generator/crates/emit/src/project/entry.rs:342`）：opt-level=3、fat LTO、codegen-units=1、panic=abort、debug="line-tables-only"。
- 运行 0.59 s。dyn-compare 漏覆盖 0；静态多出 161 类，其中 indy 模型 67 类。

**二进制 14.4–15.1 MB 的构成**

| 段 | 大小 | 内容 |
|---|---:|---|
| `__TEXT,__text` | 5.6 MB | java_runtime 2.1M；java_body_* 1.5M；每类 ObjectVTable 协议方法 0.76M；clinit 0.3M；std / core 约 0.35M |
| `__DATA_CONST` | 3.4 MB | 反射元数据表与栈帧行表（生成源码 `meta_tables.rs` 3.7 MB、`line_tables.rs` 2.4 MB） |
| `__LINKEDIT` | 5.0 MB | 符号表等 |

**三处浪费**
1. **元数据表未按闭包裁剪，LTO 也删不掉。**
   - `CLASS_METHODS` 有 9786 行，可达方法只有 1813，本例 `reflect_members=0`。
   - 每行 `MethodMeta` 是 8 个 `&str` / 切片，约 136 B，另加字符串本身。
   - `LINE_TABLES` / `LINE_NUMBERS` 覆盖全部方法。
   - 这些表是 `#[export_name]` static，任何 opt-level 下都保留，是最大的固定开销。
2. **每类 vtable 协议方法无条件生成（270 类，每类约 2.6 KB）。**
   - `__shallow_copy` 318K、`__unsafe_*` 234K、`__erased_vtable` 115K。
   - `is_instance_of` 110K、`__view_into` 55K 已在 S7-1 删除。
3. **构建档位只优化速度。**

**档位实测（同一份生成代码）**

| 配置 | 二进制 | `__text` | 运行耗时 |
|---|---:|---:|---:|
| opt=3，不 strip（现状） | 15.1 MB | 5.6 MB | 0.57 s |
| opt=3 + strip | 10.5 MB | 5.6 MB | — |
| opt=s + strip | 8.9 MB | 4.1 MB | 0.70 s |
| opt=z + strip | 6.9 MB | 2.1 MB | 0.48 s |

HelloWorld 不是计算密集型，运行耗时差异属噪声。

## 二、约束（主会话审阅结论）

1. **元数据按档案裁剪，不按单个程序裁剪。**
   - 语料模式下，档案只生成一次，与具体程序无关（CLAUDE.md 第 2 条）。
   - 生产模式下，档案就是用户项目的调用链并集。
2. **裁剪单位是类。** `Class.getDeclaredMethods()` / `getDeclaredFields()` 返回全部声明成员。
   - 反射可达的类保留完整成员表。
   - 反射不可达、MH 也不可解析的类整表不出行。
   - 依据取自闭包的 reflect / rcall 数据，生成器不写类名特判。
3. **vtable 协议方法不做「按需生成」。** 这是过渡方案。终态是 S7-3：浅拷贝、反射字段、Unsafe 槽位走字段描述，按类的 `__shallow_copy` / `__reflect_field` / `__unsafe_*` 整体删除（`2026-10-04-s7-object-handle-descriptor.md` §七）。
4. **`strip = "symbols"` 现阶段不可用。**
   - `runtime/java_runtime/src/vm_stack.rs:171` 用 `std::backtrace::Backtrace::force_capture()` 取 Rust 帧符号名，再映射回 Java 帧。
   - 去掉符号会破坏 `Throwable.getStackTrace`。
   - 要 strip，先把栈还原改为按地址查表（B2）。
5. **缺省 release 保持 opt=3 + fat LTO（D2）。**
   - R1 有 12 个运行超时用例，opt=z 不作缺省。
   - 体积档位单列，由用户决定是否提供。

## 三、终态目标（量化）

| 项 | 目标 |
|---|---:|
| 元数据表（`__DATA_CONST` 中反射表 + 行表） | 反射不可达类 0 行；行表只覆盖翻译方法，存根 0 行；编码为字符串池 + u32 索引，0 个 `&str` 胖指针 |
| 每类 vtable 协议方法 | 0（随 S7-2 / S7-3） |
| HelloWorld release 二进制（opt=3） | ≤ 3 MB（估算，B0 重测后修订） |
| GraalVM 参照 23 例 release 二进制（opt=3） | 逐例 ≤ 同机 GraalVM native-image 缺省构建大小，即 5.9–33.1 MB（macOS arm64，`docs/reports/2026-10-04-graalvm-baseline.md` §二、§三）；起点：LynchBell 13.3 MB，原生 5.9 MB |
| 行为 | 反射、栈回溯、MH 解析输出不变（服务器抽查） |

## 四、步骤（每步单独提交、单独抽查）

| 步 | 内容 | 验收 | 状态 |
|---|---|---|---|
| B0 | 在当前 main 上重测 HelloWorld / DeepCopy release 二进制的段构成（同一口径脚本化） | 基线数字写回本文 | 🔄 随 B1 |
| B1 | 元数据按档案、按类裁剪：反射成员表只为反射可达 / MH 可解析的类出行；行表只覆盖翻译方法；编码改为字符串池 + u32 索引 | 生成器单测通过；反射 / 栈 / MH 类 e2e 抽查通过；HelloWorld 元数据段下降到 B0 的 ≤10% | 🔄 binsize-meta 已派 |
| B2 | 栈还原改为按地址查表（不依赖符号名），之后 release 加 `strip = "symbols"` | 打印栈类 e2e 输出不变；`__LINKEDIT` 降到 ≤0.5 MB | ⏳ ◀── B1 |
| B3 | 体积档位评估：opt=s / z 在计算密集型 e2e（R1 超时用例等）上的性能对照 | 数据交用户决定是否提供体积档位 | ⏳ ◀── B2 |
| — | 每类 vtable 协议方法 | 归 S7-2 / S7-3 | 🔄 S7-2 进行中 |
| — | 闭包多出 161 类（indy 67）、clinit 0.3 MB | 归闭包精度线（engine-order / C1d） | 🔄 |

**与在途线的关系**
- B1 改动的是表的内容与编码：`generator/crates/emit/src/project/meta_sides.rs`、`line_tables/`。
- M2（按模块切 crate）在改同一目录的 crate 布局；M3 负责元数据表按模块归属。
- B1 不动 crate 布局；M2 合入后同步一次。M3 实施时沿用 B1 的编码。
