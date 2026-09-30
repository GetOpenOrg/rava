# instr golden 差异登记

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

**断言**：`tests/support/tally.rs::PORTED_OPS`（`sim_instr` 分派的全部操作码——consts / locals /
stack / arith / arrays / returns / control / dynamic / invoke / fields——与折叠点 `fold_const` /
`fold`）mismatch 必须为 0；表外指令（分支 / switch 等由 cfg 层处理，不经 `sim_instr`）只报告。统计同时给出记录数（去重后）与调用数
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
| N9 | N2 的匹配另要求成员引用指令（字段 / 方法）与视图注释的 `owner.name:desc`（本类成员省略 owner）一致 | 子类自身方法与接口 default 体同名同描述符、同偏移同操作码时（`LocalDate.query` vs `ChronoLocalDate.query`），只凭成员引用区分出处 |
| N10 | N2 找到的方法体所属类经 `InstrCtx::with_code_owner` 传入（出处类） | invokedynamic 的 bootstrap 表 / 常量池下标属于字节码出处类；生产侧由 P4c 按同一口径传入 |
| N11 | invokedynamic 的常量池下标由回放钩子 `InstrHooks::indy_cp_index` 按记录 operand 回答 | classfile 指令操作数不携带常量池下标（见[已知差异](#已知差异不影响-golden-比对) 3） |
| N12 | `InstrFacts` 的根类方法集取自类路径上的 JDK `java/lang/Object` 类文件（根类由 runtime 手写、不在注册表） | 与 `member_owner._root_virtual_methods` 同源 |

### 版式规则

规则集中在 `tests/support/tally.rs::LAYOUT_RULES`，每次运行逐条打印命中数。**当前为空**：
移植过程中出现过的唯一版式差异（fconst / dconst 的 `0f64` vs `0.0f64`）已在实现侧消除——
`sim::consts` 以整数记号构造 `FloatLit`，与 Python `Lit(f"{n}f64")` 逐字节一致。

## 现状（2026-09-30，全部指令组移植完成）

三例均为**全量**回放（TestCompletableFuture 以 `INSTR_GOLDEN_SAMPLE=1` 跑全量 225203 条，release 约 11s；
缺省采样 1/10 时 22521 条，同样全等）。记录数去重后计；本批每条记录 count 均为 1，调用数与记录数相同。
**全部记录 exact，layout / unported / mismatch 均为 0。**

| 组 | TestHashMapOps | TestStreamBasic | TestCompletableFuture |
|---|---|---|---|
| consts | 5268 | 6261 | 45753 |
| locals | 7522 | 11051 | 70526 |
| stack | 2352 | 2759 | 20653 |
| arith | 662 | 1075 | 8207 |
| arrays | 2397 | 2722 | 20006 |
| returns | 822 | 1579 | 9305 |
| control（checkcast / instanceof） | 112 | 204 | 1401 |
| dynamic（invokedynamic / monitor / athrow / nop / wide） | 176 | 281 | 1478 |
| invoke（new / invoke{special,static,virtual,interface}） | 1842 | 3221 | 27411 |
| fields（get/put field/static） | 1599 | 2599 | 20265 |
| fold（`fold_const` + `fold`） | 42 | 75 | 198 |
| **合计（exact / 总数）** | **22794 / 22794** | **31827 / 31827** | **225203 / 225203** |

## 已知差异（不影响 golden 比对）

1. **`Const::String` 孤立代理项丢失**：`classfile::reader::decode_mutf8` 解码到 Rust `String`，孤立
   代理项替换为 U+FFFD；Python 字符串可以保留孤立代理并原样进入 `ldc` 字面量。含孤立代理的字符串
   常量两侧发射不同（本批三例未出现，ldc 全等）。
2. **`hierarchy._has_subtypes` / `_is_direct_subtype` 未移植**：Python 侧两函数无调用点（死代码），
   Rust `hierarchy` 不提供对应 API。
3. **invokedynamic 常量池下标经钩子取得**：Python 指令的 operand 是 invokedynamic 常量池下标（lambda 站点
   变量名 `__lam_{idx}` / `__lam_cap{idx}_{i}` 与 TODO 占位文本的来源）；`classfile::Operand::InvokeDynamic`
   只携带 (bsm, name, desc)，`ClassFile` 不保留常量池。instr 经 `InstrHooks::indy_cp_index(code_owner, bsm,
   name, desc)` 查询（javac 对同一 (bsm, NameAndType) 只生成一个常量池项，三元组在类内唯一），缺省 None →
   lambda 路径按未移植报告。P4c 驱动须实现该钩子；终态是 classfile 指令操作数直接携带下标（改动 classfile
   与闭包分析的模式匹配，不在本批范围）。
4. **@CallerSensitive 包装的例外判定**：Python 以「声明类名以 `/Reflection` 结尾」跳过 `getCallerClass`
   自身的包装；Rust 以「方法名 `getCallerClass` 且 native」判定（生成器不写 JDK 类名），闭包内唯一满足者即
   `jdk/internal/reflect/Reflection.getCallerClass`，两侧行为一致。
5. **getstatic `$assertionsDisabled` 常量分支不移植**：Python 该分支比较的是已 `safe_ident` 的字段名
   （`_assertionsDisabled`），永不命中（死分支）；两侧都走普通 static 字段读取。
6. **`_coerce_stored_value` 的 `Vec<` → `JArray<Object>` 改写**：已移植但不可达（渲染后的数组类型不含
   `Vec<`，Python 注释亦言明不触发）。

## 未移植分支（`InstrError::Unported`）

全部为 Python 侧的**异常路径**或**生产调用方须提供的输入缺失**，三例命中数均为 0。

| 分支 | 位置 | 理由 |
|---|---|---|
| `ldc 常量形态 …` | `sim/consts.rs` | MethodType / MethodHandle / Dynamic 常量：Python 落入 `{operand}i32` 兜底（语义错误的文本），不复制 |
| `invokedynamic … 的常量池下标不可得` | `sim/dynamic.rs` | `InstrHooks::indy_cp_index` 返回 None（见已知差异 3） |
| `lambda 实现类 … 不在注册表且 SAM 实参 … 需类型变量判定` | `sim/dynamic/lambda_args.rs` | Python 对 None 取 `is_interface`（AttributeError） |
| `接口实现方法 … 无接收者实参` | `sim/dynamic/lambda_body.rs` | Python 对空实参表取下标（IndexError） |
| `invokestatic 目标短名含包路径` | `invoke/static_call.rs` | Python 此处发射注释占位 `/* cls.m(args) */` 作为调用值（不可编译的占位，不复制）；短名表对注册表类恒不含 `/`，只在表外类出现 |
