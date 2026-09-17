# 任务管理（当前活跃）

> 创建：2026-09-16（取代已归档的 `docs/tasks-history.md`）  
> 定位：**只保留开放的问题与任务**。完成即关闭删除，不累积历史。  
> 历史任务（T01-T81 全记录）见 `docs/tasks-history.md`。

**关联追踪文档**（细粒度证据与状态）：
- `docs/plans/2026-09-15-e2e-unresolved-issues.md` — e2e 失败普查 + 问题条目（A~N 系列），**当前主线**
- `docs/plans/2026-09-15-cfg-ifelse-unresolved.md` — CFG 翻译质量
- `docs/jvm-attributes-checklist.md` — JVM 属性解析覆盖

**基线（2026-09-17 全量实测，04ed5a0）**：60 e2e，8 通过 / 52 失败（44 编译 + 8 运行阶段）。
**对照基线（c259180，同一套 CSR 前的 anchor）**：12 通过 / 48 失败（38 编译 + 10 运行阶段）。

---

## P0 · 错误家族攻坚（性价比最高）

> 已清偿：E0433 `crate::error`（闭包分支路径改裸 `Result`，invoke.py/sim.py 三处）✅ 2026-09-16  
> 已清偿：E0599 `__get_value` on 基本类型（getfield 接收者为基本类型时恒等返回，sim.py）✅ 2026-09-16  
> 附带修复：`println(D/F)` 浮点参数经 `java_fmt_*` 格式化（Java 语义 `3.0`）✅ 2026-09-16

**🔴 回归待修（最高优先级）**：**E0782 接口返回类型泄漏 ×8** — `method_gen.py` 新增「返回类型优先 generic_signature」
时未套用 Arch-1 的接口→Object 降级，生成 `Result<Iterator<E>>`（`Iterator` 是 Object 别名且未导入，
被解析成 `std::iter::Iterator` trait）。影响：HelloWorld / TestGenerics / TestCollections / TestComparator /
TestLinkedList / TestMethodRef / TestArrayDeque / TestTryResources。**对照 c259180 全部 PASS 或可编译**，属本次自伤。

| 任务 | 影响 | 说明 |
|------|------|------|
| E0782 接口返回类型泄漏 | ×8 测试 | 见上方回归条目，修法：返回值复用参数的接口降级规则 |
| E0308 类型不匹配 | ×14 测试 | 见 e2e 文档普查表；含 arrays.rs 栈下溢占位（i32 vs `Vec<i8>`）阻塞用例 |
| E0592 duplicate 定义 | ×9 测试 | `__get_this_0` 等访问器重复生成（较 doc 的 ×2 已扩散） |
| E0599 方法/关联函数缺失 | ×7 测试 | compareTo / evaluate / set_xxx stub 等；见 e2e 文档 |
| E0425 cannot find type `init` | ×3 测试 | 构造器方法引用（Arch-3 已知债务），集中于 streams |
| E0521 borrowed data escapes | ×2 测试 | TestEqualsHashCode / TestSwitchString |
| E0615 Record 属性 | ×1 测试 | 直接关联 TestRecord |
| 运行阶段失败 | ×8 测试 | 编译已过但跑挂/输出不符，见下方清单 |

**运行阶段失败清单（编译通过 ×16 中的 8 个）**：
- 存根 panic `java/io/Writer.write` — TestSorting、TestRecursion（`println` 全链路未实现）
- `user/src/` panic — TestExceptions、TestSwitchExpression、TestAbstractClass
- 栈溢出 — TestStaticNested
- 输出 diff — TestInheritance、TestCasting（多态/接口分派语义差异）

## P1 · 功能缺口

| 任务 | 来源 | 说明 |
|------|------|------|
| method reference | Arch-3 / T49 | `System.out::println` 等 + `ArrayList::<init>()` 构造器引用（E0425 ×2）；主 lambda 已通 |
| 异常处理 try/catch/finally | T45/T62 / cfg-ifelse #6 | codegen 零实现，异常表未解析 |
| `Record` 属性解析 | checklist 唯一待办 | 直接关联 TestRecord E0615 |

## P2 · 翻译质量

| 任务 | 来源 | 说明 |
|------|------|------|
| 短路逻辑 `\|\|`/`&&` | cfg-ifelse #7 | 未做 |
| ternary 方法调用物化 | cfg-ifelse #2/#4 | 分支含语句时退化为顺序代码 |
| H-1 primitive 数组 autobox | e2e H-1 | `__get_value` 已修，现阻塞于 enum compareTo（Thread_State）|
| H-2 TestRecord 输出格式 | e2e H-2 | 被 E0615 阻塞，随 Record 解锁 |
| E-1 `_impl.rs` 方法级去重 | e2e E-1 | 生成前扫描手写 companion 跳过重名方法（架构防御） |
| D-2 `areturn` Default 兜底 | e2e D-2 | 等 Arch-3 完成后删除 |
| **T-3 String 走生成** | 2026-09-17 类型 1:1 方案 | `java/lang/String.class` 字节码生成替换 `JVM_RUST` 硬编码；最高语义价值；启动条件：BFS 完整收录 String 依赖后启动；详见 `2026-09-17-java-rust-type-1to1.md` |

## P3 · 长期重构（不阻塞主线）

### 类型 1:1 对应系列（详见 `2026-09-17-java-rust-type-1to1.md`）

| 任务 | 启动条件 | 说明 |
|------|---------|------|
| **T-1 bounds 进宏** | 无依赖，可立即启动 | `impl ArrayList<E>` 不写 Rust bounds，由 `java_class!` 宏展开时注入。codegen 只写裸参数名，宏从 `generic_signature` 补全 `Clone + Default + 'static` |
| **T-2 接口泛型透明** | T-1 完成后 | `java_class!` 输入中接口类型可携带泛型参数（`List<E>`、`Iterator<E>`），宏识别接口类型后在展开时擦除为 `Object`（Arch-1）；需宏新增 `#[interfaces(...)]` 属性支持 |
| **T-4 包装类走生成** | T-3 完成后 | `Integer`/`Long`/`Boolean` 等从字节码生成，移除 `JVM_RUST` 透明映射；autoboxing 指令序列 codegen 特判 |

### 其他长期重构

| 任务 | 来源 | 说明 |
|------|------|------|
| IR 结构化（消 RawExpr/RawStmt） | T05/T06/T07/T50/T58/T61/T67 | 架构级，收敛口径见历史文档 T67 |
| slot 复用与作用域追踪 | T68 | E0425 根因修复 |
| 泛型精确化 | T66 | LocalVariableTypeTable 驱动 |
| mut 标注递归覆盖 | T54 | 嵌套块 |
| 括号优化 | T60 | 表达式优先级 |
| 布尔压缩 | T70 | `if c {1i32} else {0i32}` → bool |
| 语义桩计数 | T72 | 指令级统一标记未做 |
| 并行类解析 | T65 | 测试级并行已实现（run_tests.py），类级未做 |
| Arch-6 `_rust_type_to_binary` | e2e Arch-6 | Python 侧字符串逆查，低风险 |
| A-2 间接子类传参 | e2e A-2 | 潜在未触发，等触发再修 |

---

## 维护规则

1. 完成一项 → 从本表删除，证据（commit / 测试结果）记入对应追踪文档
2. 新发现问题 → 入表并标注来源（e2e 普查 / 代码审查 / 实验发现）
3. 每 e2e 全量运行后刷新 P0 家族数字
4. 本文档不记历史——需要查完成记录去历史文档或 git log
