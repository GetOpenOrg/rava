# instr golden 差异登记（初稿）

## 生成与运行

```bash
# 转储（在一次正常转译中截获 Python `sim_instr`；输出 build/golden/instr/<Test>.jsonl）
python3 scripts/golden/dump_instr.py tests/e2e/33_maps/TestHashMapOps.java --jdk 21 --clean
# 回放比对（目录缺省 <repo>/build/golden/instr；文件缺失的用例跳过并打印生成命令）
cd generator && INSTR_GOLDEN_DIR=<dir> cargo test -p instr --test golden -- --nocapture --test-threads=1
```

| 环境变量 | 含义 |
|---|---|
| `INSTR_GOLDEN_DIR` | golden 目录（缺省 `<repo>/build/golden/instr`） |
| `INSTR_GOLDEN_SAMPLE=N` | 每 N 条记录回放 1 条（全部用例）；缺省 TestCompletableFuture 取 10，其余 1 |
| `INSTR_GOLDEN_SHOW=N` | 已移植指令打印前 N 条不一致（缺省 20），其余指令前 N/2 条 |
| `INSTR_GOLDEN_TRACE=<op>` | 打印该指令前 3 条记录的两侧全部比对行（核对口径用） |

文件按行流式读取（TestCompletableFuture 约 1.5GB），逐指令分类表、未移植分支计数、版式规则命中
输出到 stderr。

## 比对口径

每条记录按 meta 行重建 Python 转译时的真实环境（类路径：用户类 → JDK jmods → 镜像 / VM 支持类；
按 Python registry 插入序装入注册表；`ty::Manifest` / `RuntimeManifest` / `InstrFacts` 取自同一
runtime 目录），以 pre 状态重建 `StackSim::from_state`，经**真实** `InstrEnv`（层次 / 接口 / 装箱 /
菱形求解都走 Rust 实现，不回放 Python 钩子）执行 `sim_instr`，两侧生成同形的逐行快照：

1. 新增语句：渲染文本 | slot | bind_off | value_ty；
2. 后状态：栈（表达式文本 : 类型文本 + 非平凡条目的 dup 身份分组）、局部变量（名 / 类型 / is_new）、
   计数器、合成槽类别、深度 / 声明深度 / 绑定点、下溢标志（与 sim golden 同一快照函数）；
3. `let` 回写：调用前被栈上变量引用的 `let`（`lets`）在调用后同一下标的语句；
4. 副作用：`audit <id>` / `instanceof_fold` / `inherited` / `lambda_ref` / `sam_site`，按发生顺序；
5. 错误：Python `err`（repr）与 Rust 非 Unported 错误各记一行。

### 分类

| 类别 | 判定 |
|---|---|
| exact | 两侧逐行全等 |
| layout | 行数相同，逐行不等处经 [版式规则](#版式规则) 全部规范化后相等（按规则计命中数） |
| unported | Rust 返回 `InstrError::Unported(分支)`（按分支名计数，不比对输出） |
| mismatch | 其余（含回放构造失败：指令无法定位 / 合成、类型文本无法解析等） |

**断言**：`tests/support/tally.rs::PORTED_OPS`（consts / locals / stack / arith 全部操作码与折叠点
`fold_const` / `fold`）mismatch 必须为 0；其余指令只报告。统计同时给出记录数（去重后）与调用数
（记录 × count）两套口径。

### 回放规范化规则

| # | 规则 | 理由 |
|---|---|---|
| N1 | meta 的 `user_dir` 不存在时，把 `build/<Test>/closure_input` 的 `<Test>` 段换成 `to_snake(<Test>)` | dump_instr.py 按类名记录目录，scratch 实际按 snake_case 命名（转储脚本的记录偏差） |
| N2 | 指令取自类文件：沿记录类 → 超类 / 接口广度优先，找同名同描述符、该偏移操作码与视图一致的方法体 | 继承展开会把祖先 / 接口 default 方法体（含 `super.m()` 展开）发射进子类，记录的 (cls, m, d) 是发射所在方法而非字节码出处 |
| N3 | 找不到一致指令的位置按 Python 视图合成：`ldc*` 解析 javap 注释（`int` / `long` / `float` / `double` / `String` / `class`），`bipush` / `sipush` / `xload` / `xstore` / `iinc` / `goto` 解析操作数，其余无操作数指令直接合成 | Python 规范化改写（getstatic 常量替换为装载指令、合成 pop / goto）只存在于视图 |
| N4 | `fold_const`（operand = 弹出数，comment = `装载操作码\t操作数\t注释`）→ `NInsn::FoldField`；带 `fold` 的调用 → `NInsn::FoldCall { call: 类文件指令, load: 视图合成 }`；分类键记作 `fold` | 与 `input::norm` 的折叠点形态对应 |
| N5 | 语句表补到 `pre.n_stmts`：`lets` 按原下标放回，其余位置为 `Stmt::Raw` 占位 | 转储只含被栈引用的 `let`（指令翻译只回看 / 改写这些语句） |
| N6 | dup 身份按首次出现序编号；平凡条目（Python 按新对象重建）不参与分组 | 与 sim golden 同口径（`ValueId` vs 对象 `is`） |
| N7 | Python 类型文本经注册表短名表解析为 `RsType`（表外无实参头名 → 类型形参），解析后须渲染回原文本 | 与 sim golden 同口径 |
| N8 | 副作用：`sam_ctor` 不参与比对（它是查询，由回放钩子按日志回答）；`audit` 的 `count` 参数展开为 count 条 | Rust 以 `InstrHooks::sam_ctor_path` 查询、`InstrLog` 每次计数一条 |

### 版式规则

规则集中在 `tests/support/tally.rs::LAYOUT_RULES`，每次运行逐条打印命中数。

| 规则 | 两侧形态 | 说明 |
|---|---|---|
| `float-int-literal` | Python `0f64` / `2f32`；Rust `0.0f64` / `2.0f32` | fconst / dconst：Python 以 `Lit(f"{n}f64")` 拼文本，Rust 以 `FloatLit` 渲染；同值同类型的 Rust 字面量 |

## 现状（2026-09-30，src = 0f52b6ea）

记录数（去重后；本批三例每条记录 count 均为 1，调用数与记录数相同）。TestCompletableFuture 为全量
（`INSTR_GOLDEN_SAMPLE=1`，225203 条，debug 约 70s）；缺省采样 1/10 时为 22521 条，同样 0 mismatch。

| 组 | TestHashMapOps exact / layout / unported / mismatch | TestStreamBasic | TestCompletableFuture |
|---|---|---|---|
| consts | 5268 / 0 / 0 / 0 | 6259 / 2 / 0 / 0 | 45696 / 57 / 0 / 0 |
| locals | 7522 / 0 / 0 / 0 | 11051 / 0 / 0 / 0 | 70526 / 0 / 0 / 0 |
| stack | 2352 / 0 / 0 / 0 | 2759 / 0 / 0 / 0 | 20653 / 0 / 0 / 0 |
| arith | 662 / 0 / 0 / 0 | 1075 / 0 / 0 / 0 | 8207 / 0 / 0 / 0 |
| fold（`fold_const` + `fold`） | 9 / 0 / 33 / 0 | 28 / 0 / 47 / 0 | 18 / 0 / 180 / 0 |
| 其余（未移植组） | 0 / 0 / 6948 / 0 | 0 / 0 / 10606 / 0 | 0 / 0 / 79866 / 0 |
| 合计 | 15813 / 0 / 6981 / 0（22794） | 21172 / 2 / 10653 / 0（31827） | 145100 / 57 / 80046 / 0（225203） |

`fold` 的 unported 来自被折叠调用本身（invoke 组未移植）；`fold_const` 全等。

## 已知差异（不影响 golden 比对）

1. **`Const::String` 孤立代理项丢失**：`classfile::reader::decode_mutf8` 解码到 Rust `String`，孤立
   代理项替换为 U+FFFD；Python 字符串可以保留孤立代理并原样进入 `ldc` 字面量。含孤立代理的字符串
   常量两侧发射不同（本批三例未出现，ldc 全等）。
2. **`hierarchy._has_subtypes` / `_is_direct_subtype` 未移植**：Python 侧两函数无调用点（死代码），
   Rust `hierarchy` 不提供对应 API。

## 未移植分支（按 `InstrError::Unported` 分支名，占位）

各组移植完成后在此登记剩余分支与理由；当前（src = 0f52b6ea）fields / arrays / methods / returns /
control / dynamic 六组整组返回 `"<op> 指令"`。

| 分支 | 所在组 | TestHashMapOps | TestStreamBasic | TestCompletableFuture | 理由 / 计划 |
|---|---|---|---|---|---|
| _（待各组移植后填写）_ | | | | | |
