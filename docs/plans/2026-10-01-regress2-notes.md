# 分布式全量第二轮基线回归（8 例）定位记录

分支 regress2（基于 rust-closure-analyzer@f685c7b5）。复现与验证均单例运行。

> 状态（2026-10-02）：§1–§9 ✅ 6c7eb831；§10.1a 栈帧来源统一 ✅ 已合入集成分支 be1b97be（frames-unify bf91f075）。
> 遗留（2026-10-09 复核，§10.1b，作业 r2-wait-a79e2b60）：过渡类手写 `<init>` 不成帧 ✅ 已随过渡手写删除消失；
> Object.wait 帧 ⏳ 仍在——根因不是帧登记，而是根类 `wait()` / `wait(J)` / `wait(JI)` 有字节码却整体手写，
> 终态为根类非 native 方法按字节码翻译（用户已定，object-bytecode 分支实施中，见 §10.1b-实施；e2e 复验未完）。

## 1. TestForNameInit —— 已修

- 现象：`Class.forName("[I").getName()` 处 null_recv 违约 panic。
- 根因：闭包分析器按名取类（`engine/class_lookup.rs`）对描述符形式的数组类名直接跳过，候选集为空且非 top，
  调用点结果只剩空集 → 后续 `getName` 被判接收者恒 null（null_recv 折叠）。
- 修法：数组类名的元素类型可解析（单字符基本类型或类路径上的类）时，结果取数组类镜像（同 ldc 数组类常量，
  只 touch 不初始化元素类）；有实例化期望类型时不保留（数组类不可实例化）。
- 验证：单例输出与期望一致；HelloWorld / TestForNameInit 的 classes / methods / instantiated 集合不变。

## 2. TestNestedGeneric —— 已修

- 现象：`Box<Integer>.map(Object::toString)` 内 `new Box<>(f.apply(value))` 生成 `Box::<T>::new(From::from(_t0))`，
  T = Integer 时对 String 做 checkcast → ClassCastException。
- 根因：Rust 生成器擦除方法级类型变量（`map` 的 R → Object），构造实参是顶层 Object；
  `resolve_ctor_turbofish_args` 规则 1 视顶层 Object 为「无约束」，落到规则 2（同顶层类同名形参）取调用方的 T。
  Python 版本保留方法级类型变量（实参类型为 R，规则 1 直接绑定），故基线通过。
- 修法（`instr/src/invoke/bind.rs`）：裸类型变量形参收到顶层 Object 实参且别处未绑定时绑定为 Object
  （存储擦除下实例化只是视图，Object 恒可行；取调用方 T 会引入 Java 不做的检查转换）。
- 验证：TestNestedGeneric 通过；HelloWorld / GenericClassDemo / GenericMethodTest / TestRawTypes / ArrayListTest 单例通过（编译 + 输出）。

## 3. TestClassNaming / TestClassLiteral —— 已修

- 现象：TestClassNaming 编译 E0599（main.rs 类初始化钩子引用不透明类 `__class_init`）；修掉后 / TestClassLiteral
  输出不符：`isAssignableFrom` 恒 false、`getEnclosingClass` / `isMemberClass` 等反射结果缺失。
- 根因：L1 不透明类发射（C3 第 6 项）只发 `#[binary_name]`，丢了类级元数据（all_supertypes / inner_classes /
  enclosing_method / modifiers / super_class / interfaces 等），java_meta 构建脚本建不出层次表与内部类表；
  而类镜像是非 null 的 Class，镜像上的反射查询读的正是这些表。另 `class_init_hooks` 未排除不透明类。
- 修法（emit）：`head::opaque_metadata_lines` 对不透明类照发类级元数据（去掉 has_clinit / 注解原始字节），
  `opaque.rs` 写入 `java_class_opaque!`（宏解析本就忽略未知外层属性）；`entry.rs` 钩子跳过不透明类（用户 / JDK 两侧）。
- 验证：TestClassNaming / TestClassLiteral / HelloWorld 单例与期望一致；`cargo test --release -- --test-threads=1` 通过。
  TestAnnoValues 编译错误随之消失，运行期另有根因（见 4）。

## 4. TestAnnoValues —— 已修

- 现象：`AnnotationFormatError: Invalid default: public abstract TestAnnoValues$Color TestAnnoValues$Rich.color()`。
- 根因：注解种子收集（`closure/src/seeds/annotation.rs`）把用户类排除在枚举 / Class 元素类型之外——沿用「用户类全部入链」
  的旧假设；分析器改为按可达入链后，只在注解属性体里出现的用户枚举 `Color` 停在 L1（不透明，无 `<clinit>`），
  AnnotationParser 取不到枚举常量。另外收集只扫挂载点上的注解，不扫注解类型的 `AnnotationDefault`。
- 修法：枚举 / Class 元素类型不再排除用户类；注解类型方法的 AnnotationDefault 值一并收集（含嵌套注解）。
- 验证：TestAnnoValues 单例通过；HelloWorld / TestAnnoValues 闭包集合只增不减。

## 5. TestPropertiesDefaults / TestSystemPropsSpec —— 已修（拼接 null）

- 现象：`"... absent=" + System.getProperty("no.such.prop")` 处 NPE。
- 根因：分析器把表外键折叠为 null，拼接实参静态类型不再是 String，走引用实参分支，生成 `Object::toString()`；
  f7977d04 起手写 Object.toString 对 null 接收者按 invokevirtual 隐式判空抛 NPE，而拼接语义是 String.valueOf（null → "null"）。
  非 String 静态类型的 null 拼接实参（如 `"x" + (Object) null`）同样受影响。
- 修法（`instr/src/sim/dynamic/concat.rs`）：引用实参按 String.valueOf 语义显式判空，null 给 "null"，否则 toString。
- 验证：两例 `--closure-json` 单例与期望一致（e2e 跑批带 --closure-json）。

## 6. java_meta 系统属性 / 模块服务表依赖可选的 closure.json —— 已修

- N2（b5291a03）起 closure.json 只在 `--closure-json` 时落盘、否则删除；C3 第 3 项（ea82aedd）的 java_meta 构建脚本从
  `closure_input/closure.json` 读系统属性表与模块服务表。缺省 `main.py` 不带该选项 → 表为空：
  `java.vm.specification.version` 等键在运行期为 null（TestSystemPropsSpec 缺省单跑 NPE）。e2e 跑批带 --closure-json，不受影响。
- 终态：两张表进 ClosureFacts（from_closure / from_json 同源），由发射层恒写入 scratch，构建脚本不再读调试产物。

## 7. 拼接 / record toString 引用实参改为调用 String.valueOf(Object)（5 的终态写法）

- 5 的修法在生成代码里内联 `_is_jnull` 判空，可读层不对应 Java 语义。拼接（JLS §5.1.11）与 record toString
  对引用实参的语义即 `String.valueOf(Object)`，终态直接发射对它的调用，方法体由 JDK 字节码翻译。
- 清单：`vm_intrinsics.toml [indy] concat_stringify = "java/lang/String.valueOf:(Ljava/lang/Object;)Ljava/lang/String;"`
  （生成器 / 分析器不写类名）。
- 分析器（`closure/src/engine/lambda.rs` `stringify`）：Concat 调用点的每个引用实参、ObjectMethods toString 的引用分量
  汇合节点，经静态边接入该方法的形参（上下文选择同 invokestatic）；toString 派发由其字节码自然产生。未登记时回落为直接派发。
- 生成器（`instr/src/sim/dynamic/concat.rs`）：引用实参压栈后走 `gen_invokestatic`，生成
  `let _tN: String = String::valueOf_obj(..)?;`；String 静态类型的实参仍按引用直接追加（null 由运行时给 "null"）。
- 验证：TestPropertiesDefaults / TestSystemPropsSpec / HelloWorld / TestRecord 单例与期望一致；四例闭包
  classes / methods / instantiated 集合与改前完全相同（valueOf(Object) 原已在链上）。
- 未动：record hashCode / equals 的引用分量仍内联 `_is_jnull`（语义为 Objects.hashCode / Objects.equals，可同法改写）——已在 §9 处理。

## 8. 6 的实施：两张表随 ClosureFacts 由发射层写入

- `input/src/facts.rs`：`ClosureFacts.system_properties`（`SysPropFacts { values, dynamic }`）与
  `SeedFacts.module_services`（模块 provider 的 (服务, provider)，事实序）；from_closure 读引擎、from_json 读
  closure.json 同名字段，两路同源（driver 冷算时的 `{:#?}` 一致性校验覆盖新字段）。经 `EmitInput` 交给发射层。
- `emit/src/project/entry.rs` `write_closure_tables`：每次构建写 `<scratch>/closure_input/closure_tables.rs`
  （内容相同不重写，保留 mtime，java_meta 不无谓重编）。java_meta 的 `lib.rs` 直接 `include!` 该文件；
  构建脚本的 closure.json 解析（`build_script/closure_tables.rs`）删除。
- 验证：TestSystemPropsSpec / TestPropertiesDefaults / HelloWorld 不带 `--closure-json` 经 `rava build` 单跑与期望一致
  （scratch 中无 closure.json）；`cargo test --release -- --test-threads=1` 通过。

## 9. 去掉「清单未登记回落」；record hashCode / equals 引用分量改调 Objects.hashCode / Objects.equals

- 清单（`vm_intrinsics.toml [indy]`）新增 `component_hash = Objects.hashCode(Object)`、
  `component_equals = Objects.equals(Object,Object)`，与 `concat_stringify` 并列为分量处理入口。
  两侧装载时校验：登记了 concat 引导而缺 `concat_stringify`、或登记了 object_methods 而缺三项之一，直接报错
  （分析器 `closure/src/manifest/indy_helpers.rs`；生成器 `input/src/manifest.rs` `indy_helpers` / `indy_helper(key)`）。
- 分析器（`engine/lambda.rs`）：`stringify` 泛化为 `indy_helper`，Concat 引用实参、ObjectMethods 三个方法的引用分量
  一律经静态边接入入口形参（equals 两个实参取同一分量汇合节点）；原 toString / Object 方法直接派发的回落路径与
  `object_method_desc` 删除。HelloWorld / TestRecord 闭包 classes / methods / instantiated 与改前完全一致。
- 生成器：`concat.rs` 无回落分支，缺项报 `清单缺 concat_stringify`；`object_methods.rs` 引用分量压栈后
  `gen_invokestatic` 调入口（`hash_of` / `eq_of` 拆为基本类型专用的 `prim_hash` / `prim_eq`），equals 每个分量的物化语句
  收进该分量自己的块（`{ 语句; 比较 } && { .. }`），保持短路求值。
- import 扫描（`emit/src/imports/referenced.rs` `indy_helper_refs`）：concat / object_methods 调用点把入口声明类计入引用，
  否则用户类缺 `use Objects`（E0433）。
- `_is_jnull` 生成面复核：record / 拼接路径计数为 0。仍存在且属正当语义的：
  - ifnull / ifnonnull（Java `== null`）：`cfg/src/cond.rs`、`method/src/unify.rs`、`method/src/cond_text.rs`；
  - typeSwitch null 选择子 → -1：`instr/src/sim/dynamic/type_switch.rs`；
  - 反射构造分派按接收者是否为 null 区分 new / `<init>`：`emit/src/phase2/dispatch.rs`。

## 10. 分布式基线外非超时失败 14 例（regress2-b）

复现交由服务器（本机不跑 e2e / 单例）。读码预分类：

### 10.1 PrintDebugStatement / ReflectionGetSource —— FS-E1（栈帧为 Rust 符号），架构项

- 现状：`throwable_impl.rs` `fillInStackTrace` 解析 `std::backtrace` 的 Display，类名 / 方法名取 Rust 符号路径，
  文件名 / 行号为生成的 `.rs` 位置。classfile 未解析 LineNumberTable（只读了 SourceFile）。
- 终态：StackTraceElement 的类 / 方法 / 文件 / 行号与 JVM 一致，Rust 符号 / `.rs` 路径外露 0 处；运行期零额外开销
  （不维护影子栈，只在 fillInStackTrace 时查表）。
- 机制：回溯帧本就带 `at <生成文件>:<行>`（宏对方法体 token 保留原 span），据「生成文件行 → (Java 方法, Java 行)」
  表即可同时恢复方法名与行号，不依赖 Rust 符号。
- 步骤（每步单独提交；S2 起以「剥去行标记后生成树逐字节不变」为验收，交服务器跑 gen_trees / compare_trees）：
  - S1 classfile：Code 属性解析 `LineNumberTable`（pc → 行），NormCode 透传。生成物不变。
  - S2 sim / method：语句的来源偏移随语句流动，不进 IR 语句树（避免各遍历对标记做透明处理）：
    `SimState.stmt_pcs` 与 `stmts` 等长，`push_stmt` 是唯一追加入口；`Node.stmt_pcs`（合成语句 None）经
    `push_stmt` / `append_stmts` / `clear_stmts` 维护；`Entry.pc` 由树发射标注（语句 = 来源指令，if / while /
    match 头 = 节点终结跳转指令）。渲染时（`method/src/lines.rs`）行变化处先出独立标记行，行级后处理
    （尾 return 删除、补 Ok、构造器收尾、数组折叠）对其透明，方法体收尾并入其后第一条内容行行尾 `// line N`
    （不增行）。验收：剥去 ` // line N` 后生成树与改前逐字节一致。
  - S3 emit（`emit/src/project/line_tables.rs`）：拆层后、落盘的每个生成文件（声明层 / 实现层 / 用户类）扫描最终文本：
    `java_class!` 块内 `#[java_method(name)]` 行起一个方法区段（`__init_on` 等紧随的辅助体归同一方法），行尾标记
    给 (Rust 行, 方法下标, Java 行)，块结束行记「块外」。汇总写 `<scratch>/closure_input/line_tables.rs`
    （同 closure_tables，内容不变不重写），java_meta 以 `__java_meta_LINE_TABLES` 导出，`meta::line_tables()` 读取。
  - S4 运行时（throwable_impl.rs，VM 驱动类 ③）：帧按 `at` 的路径（各 `/` 边界后缀查表）+ 行取「Rust 行不大于该行」
    的末项得 (类, 方法, SourceFile, Java 行)；查不到 / 块外 / 方法首个标记之前（宏生成的分派包装、`new` 序言）的帧剔除；
    方法体内 Rust 闭包帧与紧随的同一 Java 方法外层帧合一；按 HotSpot 规则跳过 fillInStackTrace 帧与本异常类及其
    超类（`meta::class_hierarchy`）的 `<init>` 帧；上限 1024 帧。
  - S5 scratch profile：release 由 `strip = "symbols"` 改为 `debug = "line-tables-only"`（与 dev 同；剥离会连同行信息
    一起去掉，栈帧无从恢复）。代价是 release 二进制带行表段，属 JVM 语义所需。
  - 已知缺口：继承展开到子类块内的方法体按所在块的类报告（JVM 报声明类），待确认生成器是否有此形态后补。
- 验收用例：PrintDebugStatement、ReflectionGetSource、TestCustomException（printStackTrace 形态）。
- 量级：classfile / ir+sim / emit / 运行时 / profile 五步，main 已同意先行实施。

- S2 生成树对照（s2-a6a7c26e，基线 59da6137）：剥去 `// line N` 后方法体文本逐字节不变；7 例的差异是个别文件换了
  body crate（`layers.rs` 按文本字节数装箱，标记使文本变长，边界附近的文件可能换箱）。行表按落盘路径扫描，不受影响。

### 10.1a 栈帧来源统一（frames-unify，实施）

- 终态落地：`vm_stack::capture_java_frames` 是全部栈帧消费方（fillInStackTrace / getCallerClass / getClassContext /
  StackWalker）的唯一来源，帧由行表给出；Rust 符号解析（parse_symbol / java_method_of / executes_body / 派发入口规则 /
  capture_frame_classes 等）整段删除，`MethodMeta.dispatched`（只服务于符号规则）随之删除。
- 行表（`emit/src/project/line_tables/`）：方法项为 (帧归属类, 方法名, 描述符, 源文件, 宿主类)，运行时按
  (类, 名, 描述符) 取归属类自身声明的 MethodMeta，归属类无表项时取宿主类的行。手写方法的 `// [meta]` 行结束上一区间。
- 手写方法成帧（`line_tables/handwritten.rs`）：生成文件以 `// [meta]` 注释对、`#[java_native]` 声明、
  `body = "handwritten"` 声明登记手写方法，伴生 `<stem>_impl.rs` 用 syn（span-locations）解析 `impl` 块，各登记 fn 的区间
  （含属性行，`#[jvm_native]` 插入的类初始化 span 在属性行）写行表项，Java 行取哨兵：native → -2（Native Method），
  其余 → -1。getCallerClass / getClassContext / callStackWalk 自身的 native 帧由此出现，跳帧规则改为 HotSpot 原样：
  getCallerClass 第 0 帧本方法、第 1 帧 CS 方法、其后首个不被安全栈遍历忽略的帧。
- native-gaps 的语义经行表自然保留：帧归 `declared_by`（行表本就读该属性）；继承转发外壳、派发入口、vtable impl、
  `X__m_base` 包装以调用点 span 落在块外或方法序言，不成帧，方法体 token 保留原 span 在实际执行帧上成帧。
- 闭包帧一律不成帧（符号含 `{closure`）：原位闭包单行，外层帧同一行；延迟 lambda 代理闭包的位置是创建点，
  旧 Throwable 规则会在创建方法上多出一帧，现消除。
- 手写根类成帧：根类无生成文件，登记源为根类自身字节码（`project::root_line_registration` →
  `handwritten::root_methods`）：每个方法对应 `invokespecial` 落点 `<根>__<fn>_base`（根类实现本体，object.rs），
  不可覆盖方法（final / private / static）另对应 `impl <根>` 固有 fn（object_impl.rs；可覆盖方法的固有 fn 是静态类型
  为根类的虚调用入口，转 vtable 到覆盖体，与派发入口同样不登记）；fn 名按调用侧根类重载命名规则（与方法体生成器同源）。
  伴生 fn 首条语句若为接收者 null 检查 `if <recv>.is_jvm_null() {..}`，该区间记 Java 行 0（序言，不成帧）：
  invokevirtual 的隐式 null 检查在调用点抛 NPE，被调方法不入栈。clone / notify / notifyAll / getClass / wait 由此成帧。
- StackWalker 行号与 Throwable 同源：行表另附各方法的 LineNumberTable（`__java_meta_LINE_NUMBERS`），帧的 bci 取
  行表 Java 行的首个 start_pc（手写无行 → -1），`StackStreamFactory` 填 `StackFrameInfo.bci`，
  `initStackTraceElement` 按 HotSpot `Method::line_number_from_bci` 由 bci 定行（native → -2）。
- 过渡类手写 `<init>`（手写构造工厂不对应 Java `<init>` 帧）不成帧，只记录：随 C1d-a 删除过渡类手写后自然消失。
  根类 `wait(J)` / `wait(JI)` 等有字节码的非 native 方法当前为手写体（帧行 -1，JDK 为 `wait0` native 帧 + `wait` 行号），
  归手写边界收窄（字节码翻译后自然对齐），不属帧来源问题。
- 边界用例：06_exceptions/TestNativeFrameTrace（sleep0 native 帧、延迟 lambda、catch 区段）、
  62_reflection/TestStackWalkerLines（直接 / 递归 / lambda / default / 继承 / 构造器 / clinit 帧的 StackWalker 行号）、
  06_exceptions/TestObjectNativeFrames（Object.clone / notify / notifyAll native 帧），expected 均为 JDK 21 实测。

### 10.1b regress2 遗留复核（2026-10-09，regress2-rest）

作业 r2-wait-a79e2b60（us1，ref a79e2b60，参考 JDK jdk-21.0.11+10）：TestNativeFrameTrace / TestObjectNativeFrames /
TestStackWalkerLines 通过；新增边界用例 06_exceptions/TestObjectWaitFrames（expected 为参考 JDK 实测）失败。

- **② 过渡类手写 `<init>` 不成帧 —— ✅ 已消失**。证据：`closure.toml [vm_boundary]` 只剩 `java/lang/Class`
  （其 `<init>` 只由 VM 调用，Java 代码不可达，不出现在任何栈上）；`runtime/` 下 `#[jvm_boundary]` 0 处；
  `non_native_overrides` 0（09-28 起）；作业中 4 个 scratch 内 `// [meta] #[java_method(name = "<init>"` 均为 0 处
  （手写 `<init>` 的唯一登记形态，`class_writer/methods.rs` 对 `<init>` 不出签名注释即不成帧的分支已无输入）。
- **① Object.wait 帧 —— 仍在，且不是「行号哨兵」问题**。JDK 21 的 `wait()` / `wait(J)` / `wait(JI)` **不是 native**：
  `wait()` → `wait(0L)`（Object.java:339）；`wait(J)` 经 `Blocker.begin()` 调 **native `wait0(J)`**（:366）、
  捕获 InterruptedException 时对虚拟线程清中断、finally `Blocker.end`；`wait(JI)` 校验纳秒（:476 / :480 抛 IAE）后调
  `wait(J)`（:488）。JDK 帧形态为 `wait0 (Native Method)` + 各 `wait` 重载的字节码行号帧（见 expected）。
  现状：根类手写体把三个重载各写成一个 fn（`object.rs` 的 ObjectVTable 默认方法与 `object_impl.rs` 的固有方法），
  按非 native 记 -1，产出单帧 `Object.wait Object.java:-1`，少 `wait0` 帧与重载间的嵌套帧。
  改哨兵为 -2 也不对：JDK 的 native 帧是 `wait0`，`wait` 帧有真实行号，只能从 `wait` 自己的字节码得到。
  另外手写体跳过了 `Blocker.begin/end`（虚拟线程在 `wait` 期间钉住载体时对 ForkJoinPool 的补偿），
  与 VirtualThread 字节码翻译终态（2026-10-03 定）语义不一致——帧只是同一缺口的可观测面。
  handwritten-boundary.md 二节「`Object.wait` / `notify` 是 native」的表述不准，已更正。
- **终态（待用户定）**：根类按手写边界规则逐方法划分——`ObjectVTable` 对象模型仍是运行时基础设施，但根类**有字节码的
  非 native 方法**（`wait()` / `wait(J)` / `wait(JI)`、`equals`、`toString`、`finalize`）按字节码翻译，手写只剩 ACC_NATIVE
  （`getClass` / `hashCode` / `clone` / `notify` / `notifyAll` / `wait0`）。帧随之自然对齐（翻译体带 `// line N`，
  `wait0` 按 native 登记为 -2），不需要任何帧登记特判。需要的通用机制：
  1. 生成器为根类发射翻译体：根类无 `java_class!` 生成文件，翻译体落为 `invokespecial` 落点同形的自由函数
     `Object__<m>_base<T: ObjectVTable + ?Sized>(this: &T, ..)`（与现有 `Object__toString_base` 等同一命名），
     放根类目录下的生成文件；`impl Object` 固有方法与 ObjectVTable 默认方法只做转发（不成帧，同派发外壳）；
  2. 行表：根类翻译体按生成文件的 `// line N` 标记成表，`root_line_registration` 只登记 native；
  3. 闭包：根类非 native 方法改按字节码建模（当前按手写体扫描），`Blocker` / `CarrierThread` 分支的折叠依赖
     「非导出包类不可由用户扩展」，非虚拟线程程序的闭包应不增长（验收：HelloWorld / DeepCopy 类集不变大）；
  4. 根类非 native 手写方法计入 raw-audit（现不计数，是隐形手写），终态 0。
  量级：closure / instr / emit / runtime 四处，非小修；涉及根类模型，按授权范围先报用户再实施。

#### 10.1b-实施（object-bytecode，2026-10-09，用户已定终态）

与上面草案的差异：翻译体不做成泛型 `_base<T>`，而是**以根类句柄为接收者的自由函数**
`Object__<fn>_body(this: &Object, ..)`——根类字节码里的 `this` 是 Object，方法体生成器对根类接收者走 bare-Object
固有方法路径（`this.getClass()?` / `this.hashCode()?` / `this.wait0(..)?`），与生成类的 `let this = self;` 同型。
各对象类型需要能给出「自身的 Object 句柄」，为此 `ObjectVTable` 加 `__object()`（缺省 None）。

- **闭包**（`closure/src/engine/facts/kinds.rs`）：根类方法按自身字节码分类——native → 手写，无码 → 抽象，其余 → 字节码。
- **输入 / 上下文**：`EmitInput.root` 携带根类 `ClassInfo`（根类仍不入注册表）；`EmitCtx::code_class` 在注册表未命中时回落根类，
  方法体生成器（`method_bodies.rs`）与行表的行号查询都用它。
- **发射**（新模块 `emit/src/project/root_bodies.rs`）：根类全部非 native 有码实例方法（构造器 / 类初始化除外）各生成一个
  `Object__<fn>_body`；在档案调用链上的按字节码翻译，链外的生成同签名 `panic!("stub: ..")` 存根（运行时契约总要链接到
  这组符号）。命名与调用侧同源（`root_bodies::rust_name`：根类重载取描述符后缀名，前提是该名在手写 API 名面）。
  落盘与生成类拆层同构：方法体放首个实现 crate 的 `body/java/lang/object_body.rs`（`#[export_name]`），声明层同路径文件
  放外部声明块（`#[link_name]`，符号由 `rava_macros_core::plan::free_fn_link` 求出，与宏拆层同一规则）——方法体引用的类
  （`StringBuilder` 等）可能落在声明层上层段，不能直接放声明层底段。无实现 crate 时整体落声明层。生成期事实在第二阶段前并入。
- **行表**：方法体文件头 `// [root_bodies] <类> <源文件>` 让 `line_tables::scan` 进入方法区模式（方法属性为注释形态
  `// #[java_method(..)]`，第 0 列 `}` 结束方法区间）；`root_line_registration` 只登记根类 native 方法（`wait0` 记 -2）。
- **运行时**：`ObjectVTable` 的 `equals` / `__to_string` / `wait` 族缺省体与 `Object__{finalize,equals,toString}_base`
  转交翻译体（`__object()` 为 None 的非 Java 类载体——基本类型盒、`JvmRef`、lambda 载体、null 哨兵——保留载体语义）；
  `java_class!` 宏的存储 impl 与数组对象应答 `__object()`（`Object::__from_storage` 引用计数加一取回同一对象）；
  `object_impl.rs` 去掉 equals（含 String 内容比较捷径）/ toString / wait 族的手写近似，固有方法只做 null 检查与转交，
  新增 native `wait0`。
- **审计**：根类非 native 方法若未翻译（构造器体不是单条 return 时）计入 `non_native_overrides`；按构造当前为 0。
- **验证记录**（JDK 21 参考构建 jdk-21.0.11+10）：
  - ob-b-9873a830（jp2）：单测 `cargo test -p emit -p closure -p input` 全过；e2e 全挂 `E0432 unresolved import Blocker`——
    声明层底段看不见上层段类，373a077b 改为声明层文件只用预导入、方法体导入只进实现层。
  - ob-c-db41935c（jp2，合入 main c249cdec 后）：闭包 HelloWorld classes 1870（嵌套口径）/ 3442、translate_code_classes 3022；
    DeepCopy 2109 / 3730、3269（b74d2e7e 基线 3059 / 3304，均降）；14 例 e2e 同一编译错：翻译后 `equals` 体
    `this == obj` 为 `&Object == Object` 无实现。d716bdc3 在 Object 基础设施（`object_ext.rs`）补引用形态同一性比较。
    作业 6600s 超时，c249cdec 基线闭包未测出。
  - ob-d-d716bdc3（us1）：c249cdec 基线闭包对比 + 6 例 e2e 复验——结果见下条（恢复入口：本作业结果目录
    `cluster_results/job/ob-d-d716bdc3/`）。

### 10.2 UTF8EncodeDecode —— 模块资源改由调用链字节码推导

- 现象（C6 抽查，ubuntu）：运行期 `InternalError`，`Caused by: NullPointerException`，dyn miss 0，未命中存根。
- 根因（读码）：`Character.getName` → `CharacterName` 构造经 `getClass().getResourceAsStream("uniName.dat")` 读名称表；
  模块资源只嵌入 seeds.toml `[module_resources]` 手登记的路径（仅 currency.data），`uniName.dat` 缺席 → 资源流为 null →
  `InflaterInputStream(null)` NPE → 构造器包成 InternalError。
- 终态：资源名本就是读取方法体里的 ldc 常量，由字节码推导，不再手登记（`[module_resources]` 删除）：
  调用链上方法体的路径形 ldc 字符串按 `Class.resolveName` 规则（`/` 开头为绝对名，否则相对所在类的包；另按原样试
  `ClassLoader.getResource` 形态）解析，类路径上存在的非类文件即嵌入（`input/src/resources.rs`）。资源随读取代码进出
  闭包：不调 `Character.getName` 的程序不再嵌入它，currency.data 同理只在 Currency 数据读取在链上时嵌入。

### 10.3 SequenceGenerator —— 再赋值的接口声明局部取接口载体

- 现象（r2 抽查）：运行期 `ClassCastException: java/util/ArrayList$SubList cannot be cast to java/util/ArrayList`。
- 根因：`List<Integer> s = new ArrayList<>(); … s = s.subList(..)`。存储管线把接口声明的局部收窄为首值具体类
  `ArrayList<Object>`；后续存入 subList 结果命中同变量漂移，经 Object 边界按 `ArrayList` 重建 → 运行期 CCE。
- 终态：方法体在 LVT 声明区间内另有引用存储（`SlotDecl.reassigned`，由字节码 astore 扫描得出）且声明类型为接口时，
  首值经 `From` 上转到接口擦除载体（`List<Object>`），声明不收窄；只赋值一次的局部维持具体类收窄（可读性不变）。
- 验收：SequenceGenerator；广谱回归建议 gen_trees / compare_trees（声明类型形态会变）。
- 生成树对照（trees-b8f5ba28，验收集 27 例，raw-audit 不变）：差异仅一种形态——已声明为接口载体的局部被再赋新建
  具体实例时，由 `<I<Object> as From<Object>>::from(Object::from(new C))` 变为 `<I<Object> as From<_>>::from(new C)`
  （HashMap / TreeMap / EnumMap / AbstractMap 的 keySet / values 懒建、Pattern.groups、InternalLocaleBuilder 等），
  经 iface_upcasts 直接上转，少一次 Object 往返，运行期视图相同。

### 10.4 RecordPatternTest —— 同一擦除类的 instanceof 被静态折叠为 false

- 现象：`genericInferenceTest` 的嵌套记录模式整段消失，少输出一行与一个空行。
- 根因：静态 instanceof 判定只认「类型文本相等」或「严格子类型」；`Decorator<Decorator<ColoredPoint>> instanceof Decorator`
  两者擦除基相同但文本不同（目标为 `Decorator<Object>`），落入「互不为子类型」分支被折叠为 false，整个 if 被删。
- 修复：同一擦除类（非数组）与同型同判。
- 验收：RecordPatternTest。

### 10.5 r2-a77f6dd2 其余失败归类

- PrintDebugStatement / ReflectionGetSource：同 10.1（FS-E1，7fdabf63），a77f6dd2 早于该修复。与 native-gaps 07918da9 的
  `vm_stack.rs` 是同一语义的两个帧源：后者按 Rust 符号名解析帧（声明类 + 方法表），行号恒 -1；FS-E1 按 (文件, 行) 查行表，
  给出类 / 方法 / SourceFile / Java 行号，且不受内联与符号形态影响。终态只留一个帧源：`vm_stack::capture_java_frames`
  改走行表（行表条目补方法描述符以对上 MethodMeta），`fillInStackTrace`、StackWalker、getCallerClass、
  getClassContext 共用；符号解析整段删除。两线合入集成分支后由 regress2-b 实施。
- TestRandomAccessFile（`JavaLangAccess.registerShutdownHook` 存根）、TestCharsetNamedStreams（StreamDecoder UTF-16 存根）：
  均为过渡手写类（`java_lang_access_impl.rs` / `stream_decoder_impl.rs`）未实现的方法；c1d-prec 的 1e623cec 已删除二者、
  改按字节码翻译 → 归 C1d，并入后在 c1d-prec 上复测。
- StockTrans / TestSerialDefaultSuid（L3 反射分派缺 `ArrayList.writeObject`）：`ObjectStreamClass.getPrivateMethod(cl, "writeObject", ..)`
  的 `cl` 来自 `obj.getClass()`，不是类常量，按名方法查找的事实只认常量所指类 → 分派闭包缺席。终态：类未知、名与形参
  已知的查找 → 已实例化且声明该名 / 形参方法的类全部计入（序列化钩子 writeObject / readObject / readObjectNoData /
  writeReplace / readResolve 由此进入）。闭包分析器 → 归 C1d。
- UTF8EncodeDecode：同 10.2（404f56b7），待 fse1-404f56b7 结果。
