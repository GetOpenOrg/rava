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
| B2 | 栈还原改为按地址查表（不依赖符号名），之后 release 加 `strip = "symbols"` | 打印栈类 e2e 输出不变；`__LINKEDIT` 降到 ≤0.5 MB | ✅ 8f5ad0c5（合并 6f9b189b）：release 验证 b2-8f5ad0c5-rel1 5/5 rc=0（ubuntu），HelloWorld release 7,410,488 B |
| B3 | 体积档位评估：可选 `release-small`（opt "s"，不设 "z"）与缺省 release（opt 3）对照，三测点 + 构建峰值内存；改判规则：s 运行慢 <5% 且二进制小 >15% 则 s 转缺省 | 数据与结论写本文 §五 | ✅ 38c17d97（合并 5bc31469）：按规则维持 opt 3 缺省（LynchBell / Factorion 运行 +13%）；16 GB 机器上 opt 3 构建大闭包用例 OOM，作为缺省档约束交用户决定（§五） |
| — | 每类 vtable 协议方法 | 归 S7-2 / S7-3 | 🔄 S7-2 进行中 |
| — | 闭包多出 161 类（indy 67）、clinit 0.3 MB | 归闭包精度线（engine-order / C1d） | 🔄 |

**B2 实施要点（binsize-b2）**
- 摸底：现状不靠符号名成帧，而靠 DWARF 行号。`vm_stack::capture_java_frames` 用 `Backtrace::force_capture()` 的 Display 文本，取每个（含内联）帧的 `at 文件:行`，再查 B1 行表（Rust 文件 + 行 → 方法、Java 行）；符号名只用来滤掉闭包帧。运行时解析 DWARF：macOS 经符号表的调试映射读 target 下的 `.o`，Linux 读二进制内的 `.debug_*`。所以 strip 后栈全空，二进制离开构建机后行号也会丢。
- 终态做法（对标 Go pclntab / GraalVM CodeInfo）：链接期把 DWARF 一次性预解析成「地址区间 → Java 帧序列」表，嵌进二进制；运行时用 `_Unwind_Backtrace` 取返回地址，按地址查表，不读 DWARF、符号表或 target 下的文件。
  - 链接器包装 `rava-link`：rava compile 以 cargo `--config target.<host>.linker` 指定。先照常链一次（不 strip），取 DWARF 和内联链，套用旧成帧规则：闭包帧不成帧、块外与序言不成帧。对照发射层写出的旁路行表 `closure_input/frame_lines.json`，生成表对象。再带上表对象重链一次，表进 `__DATA,__rava_pcmap`（ELF 为 `rava_pcmap`），校验两次链接的锚点与 `__text` 一致。
  - 二进制内的 `LINE_TABLES`（Rust 行表）删去，帧方法元数据改随地址表发射；`LINE_NUMBERS`（bci ↔ 行）保留。
  - release 加 `strip = "symbols"`。strip 在 rustc 链接之后执行，不影响包装内读 DWARF；Linux 第一次链接时去掉 `--strip-*`。

## 五、B3 体积档位对照（2026-10-05）

**实现（binsize-b3 38c17d97，合并 5bc31469）**
- 发射层根 `Cargo.toml` 增加 `[profile.release-small]`：`inherits = "release"`、`opt-level = "s"`。缺省 release 不变：opt 3 + fat LTO + codegen-units 1 + `strip = "symbols"`。按用户 10-05 定，不设 "z" 档。
- `rava build` / `rava compile` 增加 `--release-small`，与 `--dev-opt` / `--release` 三者互斥；`scripts/run_tests.py --release-small` 产物在 `target/release-small/`。
- 对照脚本 `scripts/opt_profile_bench.py`：每例转译一次；各档在独立 target 目录冷编译（依赖一并重编，构建耗时可比）；交替轮转运行取中位数，输出与 `tests/expected` 逐字节核对。记四项：运行中位数、构建耗时、二进制大小（另记 `.text`），以及构建峰值内存（单个 rustc 进程的最大常驻集）。

**实测一：ubuntu（x86_64，8 核，24 GB），作业 b3-bench2-b064f315**
- 小用例每档运行 11 次，计算用例 5 次。输出全部一致。
- 增减为 s 相对 opt 3。

| 用例 | 运行中位数 opt 3 → s | 构建耗时 opt 3 → s | 构建峰值内存 opt 3 → s | 二进制 opt 3 → s |
|---|---|---|---|---|
| HelloWorld | 0.003 → 0.003 s（0%） | 139.7 → 104.9 s（−24.9%） | 1,835 → 1,368 MB | 7,619,968 → 6,152,768 B（−19.3%） |
| ExceptionPropagation | 0.042 → 0.048 s（+14%，毫秒级，单次 0.028–0.067 s 噪声大） | 1,201.2 → 826.7 s（−31.2%） | **14,480** → 9,251 MB | 79,878,320 → 62,487,672 B（−21.8%） |
| TestStackTraceOps | 0.009 → 0.009 s（0%） | 142.3 → 107.3 s（−24.6%） | 1,855 → 1,380 MB | 7,693,192 → 6,217,168 B（−19.2%） |
| LynchBell | 120.90 → 136.95 s（**+13.3%**） | 141.4 → 106.2 s（−24.9%） | 1,832 → 1,369 MB | 7,618,296 → 6,160,664 B（−19.1%） |
| Factorion | 34.78 → 39.37 s（**+13.2%**） | 140.2 → 105.3 s（−24.9%） | 1,827 → 1,369 MB | 7,629,168 → 6,163,424 B（−19.2%） |
| DeepCopy | 0.048 → 0.052 s（+8%，毫秒级噪声） | 1,412.1 → 966.3 s（−31.6%） | **15,723** → 10,352 MB | 92,485,544 → 70,773,704 B（−23.5%） |
| PrimorialNumbers / SelfNumbers | 同一作业 job 07 / 08 在途（ubuntu 排队），结果取回到 `server_maintenance/rava/test_results/job/b3-bench2-b064f315/0{7,8}/b3_*.json` | | | |

**实测二：16 GB 机器（ARM，8 核，16 GB；作业 cgroup 上限 11,891 MB），作业 b3-mem16-b064f315**
- 每档运行 1 次，只看构建可行性与峰值内存。
- 后四例 opt 3 都在 fat LTO 阶段被 OOM 杀；s 档全部构建成功、输出一致。

| 用例 | opt 3 | s |
|---|---|---|
| HelloWorld | ✅ 138.2 s，峰值 1,843 MB，7,619,944 B，运行 0.003 s | ✅ 105.5 s，峰值 1,372 MB，6,152,736 B，运行 0.003 s |
| TestStackTraceOps | ✅ 143.5 s，峰值 1,860 MB，7,693,208 B | ✅ 110.1 s，峰值 1,386 MB，6,217,184 B |
| LynchBell | ✅ 143.2 s，峰值 1,840 MB，运行 63.6 s | ✅ 109.8 s，峰值 1,373 MB，运行 89.3 s（**+40.4%**） |
| Factorion | ✅ 142.8 s，峰值 1,840 MB，运行 25.4 s | ✅ 109.0 s，峰值 1,372 MB，运行 31.2 s（**+22.5%**） |
| ExceptionPropagation | 💥 OOM（1,118 s 时达上限） | ✅ 837 s，峰值 9,260 MB，62,487,720 B，运行 0.039 s |
| DeepCopy | 💥 OOM（1,307 s） | ✅ 983 s，峰值 10,358 MB，70,773,960 B，运行 0.059 s |
| PrimorialNumbers | 💥 OOM（1,127 s） | ✅ 831 s，峰值 9,244 MB，62,250,568 B，运行 42.3 s |
| SelfNumbers | 💥 OOM（1,141 s） | ✅ 865 s，峰值 9,241 MB，62,243,440 B，运行 19.6 s |

前一轮作业 b3-bench-38c17d97 在 kr2 上已出现同样现象：ExceptionPropagation opt 3 构建被 OOM 杀，s 档构建成功。

**判读**
- 体积：s 档二进制比 opt 3 小 19.1–23.5%，`.text` 小 33–35%，超过 15% 门槛。
- 运行：计算密集用例 s 档变慢 13%（x86_64），ARM 上变慢 22–40%，远超 5% 门槛。启动级用例无差别。
- 按 5%/15% 规则：**不改判**，缺省 release 维持 opt 3，`release-small` 作为可选体积档保留。
- 构建：s 档构建耗时少 25–32%，构建峰值内存少 25–36%。
- 16 GB 约束：大闭包用例（二进制 60–80 MB 级，ExceptionPropagation 等 4 例）在 opt 3 + fat LTO 下，单个 rustc 进程峰值 14.5–15.7 GB（x86_64 实测），16 GB 机器构建不出来。s 档峰值 9.2–10.4 GB，16 GB 机器可构建。
- 待用户决定：上述 16 GB 约束是否改变缺省档位。规则本身判 opt 3。可选做法有三种：
  - 维持 opt 3，构建机要求 ≥24 GB；
  - 缺省改 s，opt 3 作为可选性能档；
  - 按闭包规模分档。
- HelloWorld 体积目标（≤3 MB）：opt 3 为 7.62 MB，s 档为 6.15 MB，都未达标。剩余差距在闭包规模与 vtable 协议方法（§四 末两行），不在优化档位。

**与在途线的关系**
- B1 改动的是表的内容与编码：`generator/crates/emit/src/project/meta_sides.rs`、`line_tables/`。
- M2（按模块切 crate）在改同一目录的 crate 布局；M3 负责元数据表按模块归属。
- B1 不动 crate 布局；M2 合入后同步一次。M3 实施时沿用 B1 的编码。

## 六、B4 内存安全的缺省构建档（2026-10-06，暂停记录 + 恢复入口）

**用户决定（10-06）**：「运行效率和二进制大小的问题先放一边，要确保在我现有资源条件下能够编译通过，特别是在内存不足的时候。后面我会使用性能更好的机器来进行打包编译，但是开发的时候要资源友好一些。」

**终态目标**
- 缺省 release 以内存为硬约束：B3 八例在 16 GB 机器（作业 cgroup 上限 11,891 MB）上 0 OOM，构建进程树峰值 ≤ 上限 85%（10,107 MB）；满足者中取运行最快，体积作次序。
- opt 3 + fat LTO + cgu 1 移入 `--release-max`（大机器打包档）。
- 内存感知作业数：rava 构建时读可用内存，据此定 cargo 并行作业数，作业叠加不得 OOM（终态规则，非失败重试）。

**状态：暂停（10-06 用户定 C4 全量正确性优先，非正确性线让出服务器）。** 分支 build-memsafe，head 见提交记录。

### 6.1 已实现（build-memsafe，未合入）

- be06b815：`scripts/opt_profile_bench.py` 候选档（以 `CARGO_PROFILE_RELEASE_*` 环境覆写 release）：`o3-fat`、`o3-thin`、`o3-thin16`、`o3-local16`（`lto = false` 即 crate 内 thin-local）、`o2-thin`、`s-fat`，及 `release` / `release-small` / `release-max` / `dev`。增记构建进程树常驻集峰值（0.5 s 采样）、cgroup `memory.current` 峰值、逐 crate rustc 峰值（`RUSTC_WRAPPER=scripts/crate_mem_profile.py`，`b4_<例>_<档>_crates.jsonl`）与各 crate 源码量（`b4_<例>_src.json`）。
- 16d5ef4a / ec937f6a：
  - 发射层根 `Cargo.toml`：`[profile.release]` 暂定 opt 3 + **thin LTO** + cgu 1（待 6.3 定档）；`[profile.release-max]` = `inherits release` + `lto = true` + cgu 1；`release-small` 改继承 `release-max`（opt "s" + fat，与 B3 实测一致）。
  - `rava build` / `rava compile` 增 `--release-max`（四档互斥），`scripts/run_tests.py --release-max`（产物 `target/release-max/`）。
  - 内存感知作业数（`generator/crates/driver/src/mem_probe.rs`、`mem_budget.rs`）：可用内存 A = `--build-mem-mb` 或 min(MemAvailable, 各级 cgroup v2/v1 余量（不活跃文件页不计占用）, macOS vm_stat)；预算 B = 85% A；逐 crate 估计 E = 基数 + 斜率 × 源码 MB，LTO 档位下 user crate 另按全工作区源码估全程序链接；J = 「最大的 J 个 E 之和 ≤ B」的最大 J，上限为调用方 `CARGO_BUILD_JOBS` 否则核数；单进程估计超 B 时 J = 1 并告警；探测不到时 J = 上限。决定写入 `build_status.json` 的 `mem` 字段并打印 `[cargo-env]`。旧「重型闭包 ≥1700 类 → jobs 1」规则删除，`Heavy` 只余超时判定。
  - 系数目前为**暂定值**（`Model::of`），待 6.3 校准。

### 6.2 候选实测（作业 b4-mem16-be06b815，16 GB ARM，cargo jobs 4，每档运行 1 次）

结果目录 `server_maintenance/rava/test_results/job/b4-mem16-be06b815/0N/b4_*`。小闭包四例（总源码约 11.3 MB）全部构建成功、输出一致：

| 用例 | 档位 | 构建 s | 单 rustc 峰值 MB | 进程树峰值 MB | cgroup 峰值 MB | 二进制 B | 运行 s |
|---|---|---|---|---|---|---|---|
| HelloWorld | o3-thin | 109.9 | 1,595 | 2,870 | 3,260 | 8,550,424 | 0.003 |
| | o3-thin16 | 85.8 | 1,380 | 2,117 | 2,393 | 8,793,104 | 0.004 |
| | o3-local16 | 78.9 | 1,402 | 2,727 | 2,986 | 8,876,984 | 0.004 |
| | s-fat | 113.2 | 1,376 | 2,602 | 3,013 | 6,301,712 | 0.003 |
| TestStackTraceOps | o3-thin | 110.2 | 1,554 | 2,930 | 3,322 | 8,627,680 | 0.006 |
| | o3-thin16 | 89.3 | 1,387 | 2,136 | 2,412 | 8,874,816 | 0.006 |
| | o3-local16 | 80.5 | 1,420 | 2,739 | 2,993 | 8,976,144 | 0.006 |
| | s-fat | 113.9 | 1,386 | 2,644 | 3,056 | 6,366,096 | 0.006 |
| LynchBell | o3-thin | 106.9 | 1,588 | 2,915 | 3,290 | 8,543,880 | 70.7 |
| | o3-thin16 | 86.4 | 1,368 | 2,031 | 2,342 | 8,815,624 | 65.1 |
| | o3-local16 | 78.1 | 1,397 | 2,698 | 2,947 | 8,910,272 | 88.1 |
| | s-fat | 111.5 | 1,375 | 2,613 | 3,010 | 6,309,696 | 88.1 |
| Factorion | o3-thin | 110.5 | 1,574 | 2,805 | 2,651 | 8,559,176 | 26.1 |
| | o3-thin16 | 87.4 | 1,380 | 2,139 | 1,863 | 8,808,312 | 27.7 |
| | o3-local16 | 80.1 | 1,421 | 2,700 | 2,418 | 8,890,288 | 33.4 |
| | s-fat | 112.5 | 1,374 | 2,591 | 2,454 | 6,313,024 | 29.1 |

逐 crate 峰值（HelloWorld）：java_base_decl 5.4 MB 源码 → 1.38–1.54 GB；body crate 约 3 MB → 0.85–1.1 GB；user crate（全程序链接）o3-thin 1,595 / thin16 505 / local16 266 / s-fat 1,341 MB。

**初步判读（单次运行，需 ubuntu 多次中位数确认）**：小闭包下四档构建内存都远低于上限；运行上 o3-thin / o3-thin16 明显快于 o3-local16 与 s-fat（LynchBell 65–71 s 对 88 s，Factorion 26–28 s 对 29–33 s）。

**在途 / 未完成**（暂停时）：
- 大闭包四例（ExceptionPropagation job 01 jp1、DeepCopy 02 jp2、PrimorialNumbers 03 kr1、SelfNumbers 04 kr2）仍在跑，按协调要求跑完不再续发；结果落同一目录 `01`–`04`。这四例才是定档关键（B3 fat LTO 单 rustc 14.5–15.7 GB）。
- ubuntu 多次中位数作业 b4-bench-be06b815（o3-thin / o3-thin16 / o3-local16）一直排队未执行，已按 PID 停止。

### 6.3 恢复入口

1. 读取 b4-mem16-be06b815 的 `01`–`04`：`b4_*.json`（进程树 / cgroup 峰值、运行、体积）与 `*_crates.jsonl`。若某档 OOM 或进程树峰值 > 10,107 MB 则淘汰。
2. 在 ubuntu 重发多次中位数作业（同 b4-bench 命令，候选为 6.2 及大例存活者），记运行 / 构建耗时 / 构建峰值 / 体积四项。
3. 定缺省 `[profile.release]`（发射层 `generator/crates/emit/src/project/entry.rs`），以逐 crate 数据校准 `mem_budget.rs` 的 `Model::of` 系数（含 dev 档；dev 需补测以覆盖旧重型规则）。
4. 16 GB 机器验收作业：`--profiles release`（内存感知作业数）8/8 构建成功且输出一致；服务器跑全量单测 rc=0（含 JDK 字面量 lint）。
5. 更新 `docs/environment-variables.md`（`--release-max`、`--build-mem-mb`、内存感知作业数一节替换「重型闭包的自动处理」、`CARGO_BUILD_JOBS` 改为上限、`build_status.json` 的 `mem`）、CLAUDE.md 选项列表、`docs/tasks.md` BS-B4 行，并补本节定档结论。
