# 分布式全量第二轮基线回归（8 例）定位记录

分支 regress2（基于 rust-closure-analyzer@f685c7b5）。复现与验证均单例运行。

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
