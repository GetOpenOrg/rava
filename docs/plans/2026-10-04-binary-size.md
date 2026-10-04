# 生成二进制体积：剖析与终态方案（2026-10-04）

> 2026-10-04 用户把本线交给主会话统一安排推进（另一会话完成剖析后不再实施）。
> 闭包目标「正确且最小」的判据之一是二进制小（CLAUDE.md 第 2 条、`feedback_closure_goal_correct_minimal`）。

## 一、剖析（`python3 scripts/run_tests.py --filter HelloWorld --release`）

剖析时的树早于 main d840bb83（S7-1 已删掉按类的 `is_instance_of` / `__view_into`）。B0 在当前 main 上重测，数字以重测为准。

**B0 基线（main 10795076，`scripts/binsize.sh build/hello_world`）**

测量方法：
- `rava build tests/e2e/01_basics/HelloWorld.java --stop-after emit` 生成 scratch；
- 经 heavy_lock 跑 `rava compile build/hello_world --release --keep-artifacts`（档位同上：opt=3、fat LTO、不 strip）；
- 用 `scripts/binsize.sh <scratch>` 读取段 / 节大小（`size -m`）、元数据源码字节、表行数，以及元数据在二进制里的估算量。B0 的估算口径是「行数 × 结构体大小 + 去重字符串字节」；B1(c) 起改由生成器写出的 `[meta-stats]` 精确计数（估算器已删）。

| 项 | B0 |
|---|---:|
| 二进制文件 | 15,137,696 B |
| `__TEXT` 段 / 其中 `__text` / `__const` | 6,684,672 / 5,606,828 / 958,916 |
| `__DATA_CONST` 段 / 其中 `__const` | 3,391,488 / 3,385,888 |
| `__LINKEDIT` | 5,013,504 |
| `meta_tables.rs` / `line_tables.rs` 源码 | 3,714,667 / 2,451,667 B |
| 源码中 `CLASS_METHODS` / `CLASS_FIELDS` / `LINE_TABLES` / `LINE_NUMBERS` | 3,078,692 / 344,564 / 1,377,545 / 1,073,929 B |
| `CLASS_METHODS` 行数 / 可达方法数 | 9,786 / 1,813 |
| `CLASS_FIELDS` 行数 | 1,986 |
| 行表方法项 / 行（其中 Java 行 0 的行） | 9,782 / 20,695（10,338） |
| `LINE_NUMBERS` 项 / (pc, 行) 对 | 7,161 / 37,932 |
| 四张大表在二进制中的估算量 | **3,506,260 B**（结构体 3,199,532 + 去重字符串 306,728）。与 `__DATA_CONST` 3.39 MB 吻合 |
| 闭包反射数据 | `reflect.fields` 30（JDK 内部按名取字段偏移），members / gaps / field_names / allocations 均为 0 |

B1 验收线：四张大表在二进制中的量 ≤ 350,626 B（B0 的 10%）。

**B1 实测（binsize-meta，HelloWorld release，同一测量方法）**

| 项 | B0 | B1(a) 行表只留翻译方法 | B1(b) 成员表按档案裁剪 | B1(c) 池 + 字节流编码 |
|---|---:|---:|---:|---:|
| 二进制文件 | 15,137,696 B | — | 12,291,456 B | **11,842,272 B** |
| `__DATA_CONST,__const` | 3,385,888 | — | 1,073,288 | **648,744** |
| `__TEXT,__text` | 5,606,828 | — | 5,599,852 | 5,612,980 |
| `meta_tables.rs` / `line_tables.rs` 源码 | 3,714,667 / 2,451,667 | — / 733,149 | 326,123 / 754,798 | 125,888 / 501,999 |
| `CLASS_METHODS` / `CLASS_FIELDS` 行 | 9,786 / 1,986 | 同 B0 | 0 / 192（17 类） | 同 B1(b) |
| 行表方法项 / 行 | 9,782 / 20,695 | 2,265 / 12,622 | 同 B1(a) | 同 B1(a) |
| 元数据在二进制中的量 | 3,506,260（估算） | 2,349,684（估算） | — | **255,720（`[meta-stats]` 精确）** |

B1(c) 的 255,720 B 构成：反射元数据 66,536（池 47,972 + 各表流 18,564）、行表 187,659（池 83,187 + `LINE_TABLES` 71,761 + `LINE_NUMBERS` 32,711）、闭包派生表 952、用户侧 573。达到验收线（≤ 350,626 B，B0 的 7.3%），二进制里 `&str` 胖指针 0 个。

- 裁剪口径（B1(b)）：闭包分析器输出 `reflect.meta_methods` / `reflect.meta_fields`（`generator/crates/closure/src/engine/meta_classes.rs`），档案模式按入口并集。方法表收成员枚举所指类、按名查方法 / 构造器的类、按名取类得到的类、反射成员面的声明类、补种点名类与整类放开类、注解类型、序列化分配目标；字段表收按名查字段的声明类（目标推不出时取声明该名字段的闭包类）、字段枚举与整类放开的类、可序列化字段枚举的类。两者都按超类型闭包。反射缺口不扩大集合。
- 帧方法元数据（修饰符 / static / native / 注解）随行表方法项发射，栈遍历不再读成员表，所以成员表可以 0 行。
- 编码（B1(c)）：分三组（反射元数据 / 闭包派生表 / 行表），每组一个去重池（项 = LEB128 长度 + 内容），每表一条字节流（LEB128，有符号先 zigzag，行表的 Rust 行、pc、行号存增量）。运行时首次查询时解码成原元素类型（`runtime/java_runtime/src/meta_codec.rs`），`meta::*` 查询 API 不变；裁剪与编码都不依赖 LTO。
- 已知偏宽：用了反射枚举的程序里，`enumerated`（开放接收者的枚举类值集）约 2,138 类，`meta_methods` 约占闭包 85%；DeepCopy 的字段枚举作用域推不出，`meta_fields` 取全闭包。口径正确，收窄靠分析精度（枚举接收者的类值集），不是 B1 的范围。

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
| 元数据表（`__DATA_CONST` 中反射表 + 行表） | 反射不可达类 0 行；行表只覆盖翻译方法，存根 0 行；编码为字符串池 + 字节流（LEB128 下标），0 个 `&str` 胖指针 |
| 每类 vtable 协议方法 | 0（随 S7-2 / S7-3） |
| HelloWorld release 二进制（opt=3） | ≤ 3 MB（估算，B0 重测后修订） |
| GraalVM 参照 23 例 release 二进制（opt=3） | 逐例 ≤ 同机 GraalVM native-image 缺省构建大小，即 5.9–33.1 MB（macOS arm64，`docs/reports/2026-10-04-graalvm-baseline.md` §二、§三）；起点：LynchBell 13.3 MB，原生 5.9 MB |
| 行为 | 反射、栈回溯、MH 解析输出不变（服务器抽查） |

## 四、步骤（每步单独提交、单独抽查）

| 步 | 内容 | 验收 | 状态 |
|---|---|---|---|
| B0 | 在当前 main 上重测 HelloWorld release 二进制的段构成（同一口径脚本化：`scripts/binsize.sh`） | 基线数字写回本文 | ✅ §一 B0 基线（binsize-meta） |
| B1 | 元数据按档案、按类裁剪：反射成员表只为反射可达 / MH 可解析的类出行；行表只覆盖翻译方法；编码改为字符串池 + 字节流（LEB128 下标） | 生成器单测通过；反射 / 栈 / MH 类 e2e 抽查通过；HelloWorld 元数据段下降到 B0 的 ≤10% | ✅ 2682139c（抽查 18/18）：HelloWorld 元数据 3,506,260 → 255,720 B（7.3%），二进制 15,137,696 → 11,842,272 B，`__DATA_CONST,__const` 3,385,888 → 648,744 B（§一 B1 实测） |
| B2 | 栈还原改为按地址查表（不依赖符号名），之后 release 加 `strip = "symbols"` | 打印栈类 e2e 输出不变；`__LINKEDIT` 降到 ≤0.5 MB | ⏳ ◀── B1 |
| B3 | 体积档位评估：opt=s / z 在计算密集型 e2e（R1 超时用例等）上的性能对照 | 数据交用户决定是否提供体积档位 | ⏳ ◀── B2 |
| — | 每类 vtable 协议方法 | 归 S7-2 / S7-3 | 🔄 S7-2 进行中 |
| — | 闭包多出 161 类（indy 67）、clinit 0.3 MB | 归闭包精度线（engine-order / C1d） | 🔄 |

**B2 实施要点（binsize-b2）**
- 摸底：现状不靠符号名成帧，而靠 DWARF 行号。`vm_stack::capture_java_frames` 用 `Backtrace::force_capture()` 的 Display 文本，取每个（含内联）帧的 `at 文件:行`，再查 B1 行表（Rust 文件 + 行 → 方法、Java 行）；符号名只用来滤掉闭包帧。运行时解析 DWARF：macOS 经符号表的调试映射读 target 下的 `.o`，Linux 读二进制内的 `.debug_*`。所以 strip 后栈全空，二进制离开构建机后行号也会丢。
- 终态做法（对标 Go pclntab / GraalVM CodeInfo）：链接期把 DWARF 一次性预解析成「地址区间 → Java 帧序列」表，嵌进二进制；运行时用 `_Unwind_Backtrace` 取返回地址，按地址查表，不读 DWARF、符号表或 target 下的文件。
  - 链接器包装 `rava-link`：rava compile 以 cargo `--config target.<host>.linker` 指定。先照常链一次（不 strip），取 DWARF 和内联链，套用旧成帧规则：闭包帧不成帧、块外与序言不成帧。对照发射层写出的旁路行表 `closure_input/frame_lines.json`，生成表对象。再带上表对象重链一次，表进 `__DATA,__rava_pcmap`（ELF 为 `rava_pcmap`），校验两次链接的锚点与 `__text` 一致。
  - 二进制内的 `LINE_TABLES`（Rust 行表）删去，帧方法元数据改随地址表发射；`LINE_NUMBERS`（bci ↔ 行）保留。
  - release 加 `strip = "symbols"`。strip 在 rustc 链接之后执行，不影响包装内读 DWARF；Linux 第一次链接时去掉 `--strip-*`。

**与在途线的关系**
- B1 改动的是表的内容与编码：`generator/crates/emit/src/project/meta_sides.rs`、`line_tables/`。
- M2（按模块切 crate）在改同一目录的 crate 布局；M3 负责元数据表按模块归属。
- B1 不动 crate 布局；M2 合入后同步一次。M3 实施时沿用 B1 的编码。
