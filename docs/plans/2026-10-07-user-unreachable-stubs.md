# 用户类不可达方法发存根（2026-10-07）

分支：`user-unreach-stubs`（基于 rust-closure-analyzer @ b6ed3950）

## 一、目标

用户 crate 与 JDK 侧采用同一个存根判据（CLAUDE.md 原则 §2）：不在档案调用链上的用户方法生成 `__stub("stub: 类名.方法名:描述符")`，不翻译方法体，也不分析方法体内引用的类型。终态要求：**用户类不可达方法的翻译数为 0**。

vtable / trait 槽位所需的签名照常生成，只把方法体换成存根。可达性只以闭包分析器的事实为准，emitter 中不做任何放宽。

## 二、现状根因

闭包分析器对用户域本来就使用同一规则：入口是 main 加种子，反射（reflect.rs 的 expose）、序列化（serial_alloc 与反射名查找到的 hook）、lambda、enum `values` 都已建模。分析器产出的 `EmitInput.visited` 也已包含用户方法。问题出在判定层与 emitter 对用户类做了短路：

| 位置 | 用户特判 |
|---|---|
| `input/src/plan.rs` `Planner::in_chain` | `if self.user { return true }`，用户类恒视为在链上 |
| `input/src/plan.rs` `Planner::type_only` | 类型存根判据排除了用户类 |
| `emit/src/ctx.rs` `in_chain` | 注释「用户类恒全量」，与 plan 一致 |
| `emit/src/class_writer/inherit.rs` | `chain_all` / `iface_body_all`：用户类展开接口 default 时全量翻译 |
| `emit/src/class_writer/super_inherit.rs` | `cc_all`：用户类继承祖先方法体时全量翻译 |
| `emit/src/imports/cross.rs` | `CrossInput.all_in_chain`：跨 crate 声明收集时用户类全量 |
| `emit/src/class_writer/mod.rs` `is_type_only` | `!site.is_user()` |
| `emit/src/sam.rs` prescan | `user \|\| in_chain(..)` |

## 三、改法

1. **统一谓词放到 `EmitInput`**（`input/src/build.rs`）：
   - `in_chain(cls, name, desc)`：`visited` 成员判定，不区分用户与 JDK；
   - `initialized(cls)`：`(cls, "<clinit>", "()V") ∈ visited`；
   - `type_only(ci)`：类不在初始化集合中，且没有任何声明方法在链上。
   `Planner`、`EmitCtx`、`class_writer::is_type_only` 全部委托给这三个谓词，并删除 `user` 字段。
2. **类型存根判据新增「不在初始化集合」条件**：只有默认值静态字段、没有 `<clinit>` 的类（例如 `static int count;` 只经 getstatic/putstatic 访问）此前可能被误判为类型存根，导致字段访问器变成 panic。分析器会把它记为已初始化，因此不得作为类型存根。
3. **删除 emitter 的用户特判**：`chain_all`、`iface_body_all`、`cc_all`、`all_in_chain`、sam prescan 中的 `user ||`。
4. **继承祖先方法体时按精确键判定**（super_inherit.rs `Anc`）：原来的 `in_cc` 拆成两项：
   - `bridge_live = in_chain(ci, 方法)`：子类的桥 / 覆写槽是否可达，不可达时桥占槽并发存根；
   - `body_live = in_chain(sup, 方法)`：祖先方法体是否可达，只有可达才翻译祖先体。
   JDK 祖先的门控仍沿用 `in_chain(ci) || in_chain(sup)`。
5. **接口 default 展开**：`in_cc = in_chain(实现类, m) || in_chain(接口, m)`，与 JDK 侧原有口径一致。没有被选中的 default 在实现类中发存根，接口里只保留声明。

不写类名特判，生成器 crate 中没有新增 JDK 类名字面量。

## 四、影响面

- **用户 crate**：链外方法（包括未被调用的 `<init>`）变为存根，其方法体内引用的类型不再拉入 import，用户生成代码体积下降。
- **JDK 档案侧（有意修正）**：类在初始化集合中、未声明 `<clinit>`、且没有链上方法的 JDK 类，此前按类型存根处理，静态字段生成的是 panic 访问器；现在生成 `pub static`。生成树会因此出现 diff，按预期 diff 只出现在静态字段块。boundary 类的 clinit 键原本就被过滤，不受影响。
- **反射分派表**（phase2/dispatch.rs）对用户类仍全成员出臂，臂指向存根函数，可以编译。如果反射建模漏掉某个方法，运行期会命中精确的 `stub: <类>.<方法>:<描述符>` panic，这是设计上预期的失败方式。
- **import 扫描**（referenced.rs）仍扫描全部方法体，但结果按生成集过滤，属于过近似，无害。

## 五、风险点

1. **闭包建模缺口会暴露为运行期 panic**：此前用户类全量翻译，掩盖了分析器在用户域的漏点（反射名查找、序列化 hook、MethodHandle 查找、ServiceLoader 加载的用户实现、注解处理等）。遇到 `stub: <用户类>…` 时，应修闭包侧，不得在 emitter 中放宽。
2. **接口 default 展开的符号键**：`in_chain(实现类, m)` 可能命中调用点符号键，而对应方法体并未被分析器分析过。这是 JDK 侧原有的口径，本次未改动，用户侧现在与之一致。
3. **只有静态字段的类**：如果闭包把这类类定为 L1 opaque 而没有记 clinit 键，字段访问器仍会是存根。新增单测 `UserUnreached$Counter` 覆盖这种情况，需要服务器验证。
4. **用户类 `<init>` 存根**：用户类只经反射 `newInstance` 或反序列化构造时，依赖 reflect / serial 建模把 `<init>` 记入链。
5. **corpus 模式**：compose 取 single 的用户方法，需要确认语料模式下用户方法集与单测模式一致。

## 六、待验证清单（服务器）

### 单测
- `(cd generator && cargo test --release)` 全量，重点：
  - 新增 `driver/tests/build_cli.rs::user_unreachable_methods_stubbed`（fixture `UserUnreached.java`）：
    - `unusedStatic` / `Hello.unusedInstance` / `Hello.quiet` / `Counter.<init>` 为存根；
    - `main`、`lambda$main$0`、`Hello.greet`、`Hello.loud`（经接口 default 展开）照常翻译；
    - 用户文件中不出现 `ArrayDeque`（不可达方法体内的类型不得拉入 import）；
    - `Counter.count` 为 `pub static`，不是存根访问器；
  - 现有 build_cli 各例（RecordSwitch、DefaultSlot、TryFinallyReturn、ConcatOrder、NullView、OpaqueLevels、ProxyIface、JdkDefault*）应不受影响；
  - `no_jdk_literals` / `jdk_literal_lint`。

### e2e 抽查
- 基础：HelloWorld、CollectorsDemo、DeepCopy；
- 反射：`Class.forName` + `getMethod` / `getDeclaredMethods` / `newInstance` 的用户测试，以及 enum `valueOf` / `values`；
- lambda / 方法引用 / 匿名类 / 内部类 / Thread·Runnable；
- 序列化：`readObject` / `writeObject` / `readResolve` / `writeReplace`；
- 接口 default 方法（含多层继承、钻石继承）、record、Comparable + TreeMap / Collections.sort；
- 动态代理（Proxy + 用户接口）、ServiceLoader（如有用户实现）。

### 树对照与计数
- `scripts/compare_trees.sh <base> <new>`：确认 JDK 档案侧的 diff 只出现在静态字段块（存根访问器变为 `pub static`）；
- 统计用户 crate 中不可达方法的翻译数：用户 crate 内所有「非存根方法体」的键均应属于 `visited`，目标值为 0；
- 统计用户 crate 生成行数与编译耗时的下降，作为附带收益记录。

### 失败处置
- 运行期出现 `stub: <用户类>.<方法>:<描述符>` 时，用 `rava closure --why` / `--trace-class` 定位分析器漏点，并修闭包侧。
- 编译错误（例如签名缺失、桥槽缺失）应修 emitter 的签名生成，不得恢复全量翻译。

## 七、进度

- [x] 根因定位与判据统一（plan / ctx / class_writer / inherit / super_inherit / cross / sam）
- [x] 类型存根判据加入初始化条件
- [x] 新增单测 `user_unreachable_methods_stubbed`（只编译、未运行）
- [x] `cargo check --release --tests` 通过（无 warning）
- [ ] 服务器单测全量
- [ ] e2e 抽查与树对照
