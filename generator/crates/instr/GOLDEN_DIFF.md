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
| N11 | invokedynamic 的常量池下标取自 classfile 指令操作数 `Operand::InvokeDynamic.index`（与记录 operand 一致，无需回放钩子） | 与 Python 指令 operand 同源 |
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

1. **`Const::String` 孤立代理项（已消解，2026-10-01）**：`classfile::reader::decode_mutf8` 在含孤立代理项时
   另携无损 UTF-16 码元，常量池字符串常量产出 `Const::StringUtf16`，ldc 发射 `Lit::JStringUtf16`
   （`String::from_utf16_lit`），与 Python 侧保值一致；拼接配方（format! 构造 Rust 文本）仍按替换字符承载。
2. **`hierarchy._has_subtypes` / `_is_direct_subtype` 未移植**：Python 侧两函数无调用点（死代码），
   Rust `hierarchy` 不提供对应 API。
3. **invokedynamic 常量池下标（已消解，P5b）**：`classfile::Operand::InvokeDynamic` 直接携带常量池下标
   `index`（与 Python 指令 operand 同源，lambda 站点变量名 `__lam_{idx}` / `__lam_cap{idx}_{i}` 由此得出），
   `InstrHooks::indy_cp_index` 钩子与对应未移植分支已删除。
4. **@CallerSensitive 包装的例外判定**：Python 以「声明类名以 `/Reflection` 结尾」跳过 `getCallerClass`
   自身的包装；Rust 以「方法名 `getCallerClass` 且 native」判定（生成器不写 JDK 类名），闭包内唯一满足者即
   `jdk/internal/reflect/Reflection.getCallerClass`，两侧行为一致。
5. **getstatic `$assertionsDisabled` 常量分支不移植**：Python 该分支比较的是已 `safe_ident` 的字段名
   （`_assertionsDisabled`），永不命中（死分支）；两侧都走普通 static 字段读取。
6. **`_coerce_stored_value` 的 `Vec<` → `JArray<Object>` 改写**：已移植但不可达（渲染后的数组类型不含
   `Vec<`，Python 注释亦言明不触发）。

## 显式错误分支（原 `InstrError::Unported`，2026-10-01 清零）

`InstrError::Unported` 已删除。原未移植分支按性质改为三类显式错误，均不发射占位文本：

| 分支 | 位置 | 现错误 | 理由 |
|---|---|---|---|
| `ldc 常量形态 …`（MethodType / MethodHandle / 动态常量） | `sim/consts.rs` | `OutOfScope` | 合法但 javac 不产出的字节码；Python 落入 `{operand}i32` 兜底（错值），不复制 |
| `switch 标签形态 …` | `sim/dynamic/type_switch.rs` | `OutOfScope` | `c`/`s`/`i`/EnumDesc 之外的标签，javac 不产出 |
| `接口实现方法 … 无接收者实参` | `sim/dynamic/lambda_body.rs` | `BadInsn` | JVM 链接期即抛 LambdaConversionException；Python 对空实参表取下标（IndexError） |
| `invokestatic 目标短名含包路径` | `invoke/static_call.rs` | 删除 | 短名表对注册表类恒不含 `/`，表外类经 `unresolved_stub` 先行落存根，分支不可达 |
| `IrError::BadFloat` | `sim/consts.rs`（经 `From<IrError>`） | `SimError::Ir` | 生成器内部不变量（浮点记号由 classfile 数值格式化产出） |

## 静默占位（S1–S6，2026-10-01 清零）

原复制 Python 占位文本或发射错值的六处全部消除：

| # | 位置 | 原发射 | 现处理 |
|---|---|---|---|
| S1 | `invoke/special/ctor.rs`（`new`） | `/* {raw_cls}::new() */` | `new` 类名为空 → `BadInsn`（常量池 Class 项不可为空）；表外类走精确存根 |
| S2 | `invoke/special/ctor.rs`（`init_on`） | `/* invokespecial Method … */` | 接收者非未初始化对象 → `BadInsn`（JVMS §4.10.1.9 校验器拒绝） |
| S3 | `sim/dynamic/lambda.rs` | `/* TODO: invokedynamic */` + `Object::default()` | 引导实参畸形 → `BadInsn`；清单外引导方法 → `panic!("stub: 引导类.方法:描述符")`；ObjectMethods / enumSwitch 引导点真实翻译 |
| S4 | `sim/dynamic/lambda.rs` | `(impl parse failed)` + `Object::default()` | 实现句柄直接取 MemberRef，解析失败分支不可达，已删 |
| S5 | `sim/dynamic/type_switch.rs` | `/* TODO S-17 */` + `Object::default()` | EnumDesc 动态常量标签结构化解码，按身份比较枚举常量 |
| S6 | `invoke/virtual_/bare.rs` | `/* A-5 … */` / `Default::default()` | 接收者无可分派实现 → `panic!("stub: 类.方法:描述符")` |

### 与 Python 的有意偏离

1. **record ObjectMethods**：见 `emit/GOLDEN_DIFF.md` 二·9。`equals` 为单个 bool 表达式
   （`o.is_instance_of(cls) && { let that = …; (分量比较 && …) }`），分量比较整体加括号，避免语句位置的
   `{ .. } && ..` 被解析为块语句。
2. **字符串拼接实参的 toString 物化次序**：Python `concat_from_stack` 逐个出栈并即时物化 `toString()`
   临时量，临时量按**从右到左**求值（与 Java 从左到右的求值序相反，副作用可见时语义错误）；Rust 先整体出栈
   再按实参顺序物化，`_t0` 对应最左实参。
3. **类 vtable 视图分派落空**（`invoke/virtual_/vtable.rs`）：Python 在 `__virtual_view` 返回 None 时
   发射 `else { Default::default() }`，静默给默认值。Rust 按 invokevirtual 语义处理：
   - 接收者先判空（`__virtual_view(obj.__nn()?)`），null 抛 NullPointerException，与 getfield / putfield 判空同一路径；
   - 非 null 而视图落空，只可能是生成器缺陷（接收者静态类型已由 javac 校验）。以
     `panic!("vtable-view-miss: 类.方法:描述符")` 精确报出，不再给默认值。
