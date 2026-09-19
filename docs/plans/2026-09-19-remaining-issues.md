# 遗留问题总表（R5 轮集成之后）

> 创建日期：2026-09-19
> 基线：分支 `integrate-tsb` @ `8910fa5`（CFG 结构化重写 + 擦除运行时身份阶段 1 + `<clinit>`/异常对象 + PrintStream 字节码翻译 + R5-A/B/C/D 四个错误族清零）
> 关联计划：[`2026-09-18-erased-runtime-identity.md`](2026-09-18-erased-runtime-identity.md)、[`2026-09-18-cfg-structuring-rewrite.md`](2026-09-18-cfg-structuring-rewrite.md)、[`2026-09-18-clinit-and-athrow.md`](2026-09-18-clinit-and-athrow.md)、[`2026-09-18-printstream-bytecode.md`](2026-09-18-printstream-bytecode.md)、[`2026-09-15-e2e-unresolved-issues.md`](2026-09-15-e2e-unresolved-issues.md)、[`java-rust-translation-reference.md`](java-rust-translation-reference.md)

本文记录当前已知的**全部**遗留问题：架构缺口、JVM 语义缺口、生成器内部质量、项目原则违规、验证覆盖缺口、仓库事务。每一条给出现状、根因、终态目标（量化）。所有条目的解法只允许落在生成器（`codegen/`）、宏（`runtime/java_rta_macros/`）、手写层（`runtime/java_runtime/` 的 native `*_impl.rs` 与内部边界类）；禁止修改 `build/` 下的生成文件。

---

## 0 当前已验证状态

`--clean` 重新生成 + `cargo build --release` + `timeout 30` 运行，于 `8910fa5` 实测：

| 测试 | 编译错误 | `stub_fallback` | `unconsumed` | `handler_methods` | 输出 vs `tests/expected` |
|------|---------|-----------------|--------------|-------------------|--------------------------|
| TestStringBuilder | 0 | 0 | 0 | 0 | 一致（22 行） |
| TestArrayList | 0 | 0 | 0 | 0 | 一致 |
| TestCollections | 0 | 0 | 0 | 0 | 一致 |
| TestExceptions | 0 | 0 | 0 | 0 | 一致 |
| TestStaticInit | 0 | 0 | 0 | 0 | 一致 |
| TestTryShape | 0 | 0 | 0 | 0 | 无期望文件（见 V-2） |

TestStringBuilder 闭包规模：1287 个生成文件、7378 个方法、19598 个跳转、486 个 try 区域。

---

## 问题索引

| 编号 | 类别 | 条目数 | 最高优先级 |
|------|------|--------|-----------|
| [A](#a-架构缺口最高优先级) | 架构缺口 | 7 | P0 |
| [S](#s-jvm-语义缺口) | JVM 语义缺口 | 14 | P1 |
| [G](#g-生成器与宏的内部质量) | 生成器与宏内部质量 | 8 | P1 |
| [P](#p-项目原则违规) | 项目原则违规 | 3 | P1 |
| [V](#v-验证覆盖缺口) | 验证覆盖缺口 | 4 | P1 |
| [R](#r-仓库事务) | 仓库事务 | 3 | P2 |

优先级遵循项目原则：**架构问题优先于 Bug 修复**。A 类先于 S 类，S 类中多数条目在 A-1/A-2 落地后自然消解或变得容易。

---

## A. 架构缺口（最高优先级）

### A-1 存储层擦除未落地：可变泛型类无法跨实例化互转 【P0】

- **现状**：`X__inner<T>`、`X__VTable<T>` 仍带类型形参，Rust 单态化使 `X<Object>` 与 `X<T>` 是两个不相关的类型。R5-B 为「字段全 final 的泛型类」加了 `#[immutable_state]`（`From<Object>` 从擦除字段值重建实例并共享 `__identity`），只覆盖不可变类（如 `(Optional<T>) EMPTY`）。可变泛型类的不同实例化之间不能互转，子类对象不能在另一实例化下重建视图。
- **根因**：Java 泛型是擦除的（运行时只有一个 `X`），Rust 生成代码却按类型实参分裂了运行时身份。
- **终态**（即 `2026-09-18-erased-runtime-identity.md` §6 步骤 1，方案 a）：`X__inner`、`X__VTable` 为非泛型；类型变量字段以 `Object` 存储；wrapper `X<A>` 仅以 `PhantomData<A>` 携带类型实参，是同一 `Rc<X__inner>` 上的类型化视图；`From<Object> for X<A>` 对任意 `A` 成立。
- **验收指标**：`_reinstantiate_generic` 调用点 = 0；`#[immutable_state]` 重建路径 = 0（被通用视图机制取代）；依赖「带类型实参 TypeId」的运行时判定 = 0。
- **规模**：多日级重构，涉及宏 `block/mod.rs` 的 struct/vtable 展开、`coerce.py`、`fields.py`、`invoke*.py`。

### A-2 可读层禁用调用未清零 【P0】

- **现状**（TestStringBuilder 生成代码实测，不含 `*_impl.rs`）：

  | 形态 | 出现次数 | 终态 |
  |------|---------|------|
  | `Object::from_any` | 932 | 0 |
  | `.downcast::<T>()` | 839 | 0 |
  | `downcast_ref` | 283 | 0 |
  | `Rc::new` | 214 | 0 |
  | `.borrow()` | 3 | 0 |

  以上均在 [`java-rust-translation-reference.md §16`](java-rust-translation-reference.md#16-禁止出现在可读层的调用列表) 禁止列表内。`from_any` 的发射点：`_coerce_to_object` 对接口类型的分支、`invoke.py`、`invoke_virtual.py`、`sim/dynamic.py`、`blocks.py:193`。
- **根因与依赖**：依赖 A-1（类 vtable 查询取代 downcast 链）、A-3（checkcast IR 化）、A-4（接口 carrier 进类型位置）、A-5（lambda 对象化）。
- **终态**：上表全部为 0；checkcast 的可读形态为 `<T>::from(obj)` / `Into::<T>::into(obj)`，接口调用为 `it.hasNext()`。

### A-3 checkcast / instanceof 仍是字符串形态 【P0】

- **现状**：R5-B 已把「合法的子类向下转换」改为 `<T>::from(Object)`（`ObjectVTable::__view_into`），R5-C 引入 `Object::checkcast`（复用 `__view_as`）。但 `.downcast::<T>()` 字符串形态仍被 `stack.py`（hint 重定向）、`fields.py`、`invoke_sig.py`、`codegen.py` 等 15+ 处模式匹配依赖；`downcast(&self)` 与 `into(self)` 所有权语义不同。instanceof 在「静态类型是目标祖先」时仍走静态判定而非运行时判定。
- **终态**：rs_ir 增加 `CastExpr` / `InstanceOfExpr` 节点，所有消费方按节点而非字符串匹配；instanceof 全部按擦除类做运行时判定；失败的 checkcast 抛 `ClassCastException`（见 S-1）。
- **验收指标**：codegen 中对 `downcast` 字符串的模式匹配 = 0。

### A-4 接口 carrier 未进入类型位置（T-2） 【P1】

- **现状**：接口类型的参数/返回值/局部变量/字段多数仍生成 `Object`，调用点经擦除载体转换。`Constable.describeConstable` 这类「擦除接口签名 vs 类的具体泛型返回」靠 R5-B/C 的局部转换过编译。
- **终态**：接口类型位置一律为 `I<E>` carrier；调用点为 `it.hasNext()` / `it.next()`；协变返回由 vtable 槽位的擦除签名 + carrier 的类型化视图统一处理。

### A-5 lambda 不是对象 【P1】

- **现状**：lambda 以闭包装箱（`Object::from_any` + carrier 中的 `downcast_ref` 回落）；不实现 `I__VTable`，因此 default 方法不能在 lambda 上调用，`__interface` 查询对 lambda 无效。
- **终态**：每个 `invokedynamic` 站点生成实现 `I__VTable` 的合成类（与 JVM 的 LambdaMetafactory 产物同构），捕获变量为字段；闭包 `downcast_ref` 回落 = 0。

### A-6 抽象类、枚举、手写类缺少 `__interface` 【P1】

- **现状**：抽象类跳过接口实现（由具体子类承担）；枚举与手写类（含内部边界类）没有 `__interface`。对这些类的对象做接口查询会得到 `AbstractMethodError`（此前 `FileDescriptor_1` 已踩过一次）。
- **终态**：所有声明了接口的类（抽象、枚举、手写）都经宏获得 `__interface`；手写类通过 `java_class!` 的属性声明接口集合，不手写 vtable 粘合代码。

### A-7 协变返回覆盖未建模为祖先槽位的 override 【P1】

- **现状**：`position(I)ByteBuffer` 这类协变返回覆盖在子类另立同名槽位，与祖先槽位并存。R5-A 用「转发成员按描述符完全限定分派」消除了 E0034，但经祖先类型调用时分派到的仍是祖先槽位的实现，多态语义不完整。桥接方法目前由 R5-C 从桥字节码读取真实目标来解析。
- **终态**：协变覆盖 = 祖先槽位的 override（返回值上转为祖先槽位的擦除返回类型）+ 子类侧的类型化访问器；javac 桥接方法不生成独立槽位。并存槽位数 = 0。

---

## S. JVM 语义缺口

### S-1 失败的 checkcast 不抛 `ClassCastException` 【P1】
`downcast` 失败仍是 `expect("ClassCastException")`（进程 panic，不可被 Java `catch` 捕获）。`JvmError::class_cast` 已就绪。终态：全部失败路径返回 `Err(JvmError::class_cast(..))`，`expect("ClassCastException")` = 0。依赖 A-3。

### S-2 数组无法表示 `null` 【P1】
`JArray::default()` 是空数组，`null` 数组与空数组不可区分；对 null 数组的访问不抛 NPE。终态：`JArray` 具备 null 状态，`arraylength`/`xaload`/`xastore` 在 null 上抛 `NullPointerException`。

### S-3 装箱类型无法表示 `null` 【P1】
`Integer`/`Long` 等被建模为原生值，`Integer x = null`、`Map.get` 未命中返回 null 后拆箱抛 NPE 等语义缺失。终态：装箱类型是真实对象（来自字节码翻译的 `java/lang/Integer`），自动装拆箱即字节码里的 `valueOf`/`intValue` 调用，不做特殊建模。

### S-4 数组协变不完整 【P1】
`JArray` 的 `Covariant` 视图只支持上转为 `Object[]`；转为祖先类数组（`Integer[]` → `Number[]`）失败；存入错误元素类型抛 `ClassCastException` 而非 `ArrayStoreException`。终态：任意祖先元素类型的协变视图 + `ArrayStoreException`。依赖 A-1 的类型化视图机制。

### S-5 `getClass()` / 类字面量不可用 【P1】
`getClass()` 返回空 `Class`；基本类型 `Class` 只带名字；类字面量（`Foo.class`）、`desiredAssertionStatus` 缺失；`getClass() == PrintStream.class` 实际是 `null == null`。终态：每个类有唯一 `Class` 对象（按擦除类），`__class_name` 与之打通，类字面量与 `getClass()` 返回同一对象。

### S-6 identity hash 缺失 【P1】
未声明 `hashCode` 的类返回 0（全部哈希冲突，功能正确但退化为链表）；`Object.equals` 里保留了 String 内容比较的捷径；字符串字面量 intern 的同一性未建模。R5-B 已加入各视图共享的 `__identity`。终态：`Object.hashCode` 的 native 实现基于 `__identity` 生成 identity hash；`Object.equals` 为纯引用比较；`ldc` 字符串字面量经 intern 表返回同一对象；捷径 = 0。

### S-7 record 的 `hashCode` 恒为 0 【P1】
record 的 `toString`/`equals` 已由生成器按组件生成，`hashCode` 体仍是 `Ok(0)`。终态：按 `ObjectMethods` 引导方法语义组合各组件的 hash（31 多项式）。

### S-8 `NegativeArraySizeException` 缺失 【P2】
`newarray`/`anewarray`/`multianewarray` 对负长度未抛异常。终态：VM 抛出异常对象，与数组 `get/set` 的 `Result` 机制一致。

### S-9 null 字段访问不抛 NPE 【P2】
null 接收者的方法调用已抛 NPE，`getfield`/`putfield` 尚未。终态：两者一致。

### S-10 类初始化触发点不完整 【P2】
JVMS §5.5 的触发点里，手写 static native 的调用、带 default 方法的接口的初始化尚未触发 `__class_init()`。终态：§5.5 列出的触发点全覆盖。

### S-11 `InternalLock.newLockOrNull` 恒返回 null 【P2】
当前走 synchronized 回落路径，单线程下无差异。终态：随线程/同步模型（`2026-09-14-java-sync-threading.md`）一并实现。

### S-12 非嵌套 try 区域的布局偏差 【P3】
无调试信息（无 LocalVariableTable）时，catch 之后的代码仍留在 catch 体内（语义等价，形状与源码不同）；`try { a(); } catch (E e) { throw ..; } return;`（void）会把 `return` 放进 try 体。终态：不依赖调试信息，由异常表 + 支配关系推断 catch 体终点。

### S-13 不可归约控制流兜底未经实战 【P3】
`cfg/dispatch.py` 的 `loop { match __pc }` 兜底触发 0 次，与 `java_try!` 的组合没有测试覆盖。终态：补单元用例（人工构造不可归约 CFG + 异常表）。

### S-14 跨包同简单名类 【已处理，待观察】
R5-B 让 `short_cls` 对冲突类生成带包限定的 Rust 类型名（`Era` 冲突）。需确认：重载后缀（`mangle_name`）里两个同简单名类是否仍可能撞名（R5-A 提出的理论风险）。终态：后缀冲突时同样采用包限定，冲突数 = 0。

---

## G. 生成器与宏的内部质量

### G-1 变量提升是文本级的 【P1】
`codegen/method/vars.py` 基于文本做 `let` 提升。R5-A 把 `_mergedN`/lambda 绑定改为 `LetStmt`，R5-C 加了 `LetStmt.value_ty`，但提升主体仍是文本匹配；曾产生「遮蔽已初始化变量」的语义错误（`concurrent_hash_map_tree_bin.rs`）。终态：提升完全在带类型的 rs_ir 语句上进行，文本匹配 = 0。

### G-2 `Default::default()` 占位 【P1】
生成代码中 `Default::default()` 1509 处（TestStringBuilder），主要是提升后的前置声明初值与 `Self::default()` 构造。前置声明的占位会掩盖「未初始化即使用」，也要求所有类型实现 `Default`。终态：前置声明为无初值的 `let x: T;`（由 Rust 的确定赋值分析验证，与 JVM 校验器同语义），占位初值 = 0；构造路径另计。

### G-3 局部变量槽位分型 【P1】
同一 slot 先后存放不同类型的值时（`readObject0`、`LambdaFormEditor.putInCache`、javac 合成变量），R5-B/C 用「合成变量命名 `local_N`」「未命名槽位合并时类型统一」处理。终态：按活跃区间（def-use 链）拆分变量，每个区间独立命名与分型；与 LVT 的对应只用于取名。

### G-4 生成输出不确定 【P2】
提升出的 `let mut x = ...;` 行顺序在两次运行间会变化（遍历 set/dict 的顺序）。只影响 diff，但妨碍回归比对。终态：同输入两次生成的输出逐字节一致。

### G-5 宏里的遗留代码 【P3】
`block/mod.rs` 有遗留占位 `let _ = (...)`，以及 VirtualDefine 的 base 函数段里一个不可达的存根分支。终态：删除。

### G-6 未使用的导入 【P3】
`invoke_sig.py`、`fields.py` 可能残留未使用的 `_class_type_param_bounds` 导入。终态：删除。

### G-7 手写虚方法体机制需写入参考文档 【P2】
R5-B 新增：`_impl.rs` 定义 `__impl_<m>` 时，codegen 在宏块里生成 `body = "handwritten"` 的无体声明，使其进入 vtable；`native_upcalls.py` 识别 `__impl_` 函数上的 upcall 声明；`#[hash_code_vtable]` / `#[equals_vtable]` 取代 `has_hash_code_method`；`#[immutable_state]`；`#[superclass_reference_fields]`。终态：以上属性全部记入 `java-rust-translation-reference.md` 与 `2026-09-18-macro-family-design.md`。

### G-8 类型变量→上界转换经 `Object` 中转 【P2】
R5-C 为绕开「vtable override 上不允许附加 `where TV: Into<Bound>`」而让类型变量到上界的转换经 `Object` + `checkcast`。A-1 落地后类型变量字段本身就是 `Object` 存储，此转换退化为一次视图构造。终态：随 A-1 收敛，不单独处理。

---

## P. 项目原则违规

### P-1 Python 中的 JDK 类名字面量 【P1】
违反「Python 代码中不得出现任何 JDK 类名常量」。已知位置：
- `codegen/type_map.py:170` `_CLS_ABBREV`（重载后缀缩写表，含小写 JDK 类名）
- `codegen/type_map.py:427` `_CLASSNAME_MAP`
- `_INTERFACE_IMPLS`
- `codegen/instr/coerce.py` 中本轮之前就存在的类名字面量

终态：上述表全部删除，信息改从 `.class`（描述符、`generic_signature`、继承链、注解属性）动态解析；`grep` JDK 类名字面量命中 = 0。`_CLS_ABBREV` 在 R5-A 取消后缀截断后已基本无存在必要。

### P-2 `runtime.py` / `java_runtime` 中的手写临时实现 【P1】
核心原则 1 要求 `String`/`ArrayList`/`System` 等来自字节码翻译。`string_ext.rs` 等手写扩展、`Object.equals` 的 String 捷径（S-6）仍在。终态：手写代码仅剩 native `*_impl.rs` 与内部边界类两类。

### P-3 内部边界类按需实现的现状清单 【跟踪项】
本轮新增/修改的边界与 native 实现：`unsafe__impl.rs`（`getUnsafe`、`allocateUninitializedArray`）、`arrays_support_impl.rs`（`vectorizedHashCode`、byte `mismatch`）、`StaticProperty.USER_*`、`locale_utils_impl.rs`、`BaseLocale.hashCode/equals`、`InternalLock.newLockOr`、`Class.getPrimitiveClass`、`shared_secrets_impl.rs`、`class_impl.rs`、`throwable_impl.rs`、`stream_encoder_impl.rs`、`utf_8_impl.rs`、`vm_impl.rs`、`blocker_impl.rs`、`file_descriptor_impl.rs`、`file_output_stream_impl.rs`、`thread_impl.rs`。其余方法保持 `panic!("stub: ...")`，随测试覆盖扩大按需补全（符合原则 3b，非缺陷）。

---

## V. 验证覆盖缺口

### V-1 全量 e2e 未运行 【P1】
本轮只验证了 6 个测试。`tests/e2e/` 其余测试在 CFG 重写、擦除阶段 1、`<clinit>`/异常、命名方案变更之后的状态未知。按项目原则，全量运行放在 A 类架构改造之后；但在合入 `main` 后应至少跑一次 `python3 scripts/run_tests.py -j 4` 建立新基线并归档到 `docs/reports/`。

### V-2 TestTryShape 缺期望输出 【P2】
`tests/expected/TestTryShape.txt` 不存在，R5-B/D 用临时文件比对。终态：用 `java` 实跑生成并提交。

### V-3 可读性无自动化检验 【P2】
A-2 的禁用调用计数目前靠手工 `grep`。终态：`scripts/main.py` 在 `[cfg-audit]` 旁输出 `[readability-audit]`（各禁用形态计数），`run_tests.py` 汇总；目标值全 0。

### V-4 单元测试运行器 【P3】
环境未装 pytest，`tests/unit/test_cfg_structuring.py`（17 个用例）用 `python3 -m unittest` 运行。终态：在 CLAUDE.md「常用命令」中写明单元测试命令。

---

## R. 仓库事务

### R-1 `main` 尚未更新 【P1】
`main` 在 `59c3355`，可快进到 `integrate-tsb` @ `8910fa5`。被主工作区未提交的 `docs/reports/jdk-scan-HelloWorld.md` 阻塞（该文件两边都有修改，可能属于另一个会话）。处理方式：在主工作区提交或还原该文件后执行 `git merge --ff-only integrate-tsb`。

### R-2 待清理的 worktree 与分支 【P2，需用户确认】
- 已合入、可删：R5 四个代理（`agent-a7c86c6c…`、`a91c2200…`、`ada7cb15…`、`a8a7063b…`）与上一轮四个代理（`agent-a404cfb…`、`a3e0755…`、`afae540…`、`a78105d…`）的 worktree 和分支。
- 来源不明、未合入：`worktree-agent-a4015d0b860f76189`（`481b73e`「refactor: 常量统一管理…」）。
- 备份：`backup/main-wip-2026-09-18`、`backup/main-wip-2026-09-18-b`、`java_rta_test`。
- `integrate-tsb` worktree：`main` 更新后可删。
- 临时目录：`/tmp/cfg_base_src/`、`/tmp/era_base`、`/tmp/era_*`、`/tmp/r5*`–`/tmp/r8*` 日志。

### R-3 生成的扫描报告被提交 【P3】
`docs/reports/jdk-scan-*.md`、`docs/plans/jdk-scan-*.md` 每次转译都会改写，造成 R-1 这类无意义冲突。终态：扫描报告输出到 `build/<test>/`（gitignore），仓库内只保留人工归档的版本。

---

## 推荐执行顺序

1. **R-1**（解除 `main` 阻塞）→ **V-1**（全量基线，只记录不修）。
2. **A-1 存储层擦除** → **A-3 Cast/InstanceOf IR 化** → **A-6 `__interface` 全覆盖** → **A-5 lambda 对象化** → **A-4 carrier 进类型位置** → **A-7 协变覆盖**；以 **A-2** 的计数表（全 0）和 **V-3** 的审计行作为统一验收。
3. **G-1/G-2/G-3**（提升与槽位分型在 rs_ir 上重做）与第 2 步并行，二者文件重叠小。
4. **S 类**：S-1、S-4、S-5、S-6 随 A-1/A-3 落地；S-2、S-3、S-7–S-10 独立推进。
5. **P-1/P-2** 清零；**G-4–G-7**、**V-2/V-4**、**R-2/R-3** 收尾。
