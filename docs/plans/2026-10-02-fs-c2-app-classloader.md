# FS-C2：应用类加载器非空与断言状态按加载器求值

> 状态（2026-10-02）：🔄 实施中，分支 fs-c2（基于集成分支 229a253b），由原 native-gaps 子代理负责；完成后接着做 boot layer（另需 C1d-a a2）。
> 关联：[`2026-09-29-rust-closure-analyzer.md`](2026-09-29-rust-closure-analyzer.md) §六 C1d 行、
> [`docs/reference/handwritten-boundary.md`](../reference/handwritten-boundary.md)（VM 注入状态属准入第 ③ 类）。

## 一、现状（2026-10-02 按代码核实）

### 1.1 TestClassNestNatives 第 63 行 NPE 的来源

- `ClassLoader.getSystemClassLoader()` 本身非空：手写 `class_loader_impl.rs::getSystemClassLoader` 返回进程单例
  `build_system_class_loader()`（name "app"，parent = 手写 `ClassLoaders::platformClassLoader()` 的具名空对象）。
- NPE 来自**类镜像不带定义加载器**：类镜像由手写 `Class::for_class(binary_name)` 建立，只填 `name`（数组再填
  `componentType`），从不写 `Class.classLoader`。`getClassLoader()` / `getClassLoader0()` 是字节码翻译，直接读该字段，
  所以任何类（用户类、嵌套类、数组类、JDK 类）都返回 null。第 60 行 `app` 因此为 null，第 63 行
  `app.setPackageAssertionStatus(...)` 抛 NPE。
- 即使镜像带上加载器，手写的 app 加载器也跑不通字节码：`build_system_class_loader` 不经构造器组装，
  `assertionLock` 为 null。`desiredAssertionStatus` / `setXxxAssertionStatus` 的字节码要 `synchronized (assertionLock)`，
  会再次 NPE。可见 app / platform / boot 三个加载器都必须由 JDK 构造器建出，即 `ClassLoaders.<clinit>` 的字节码。

### 1.2 断言状态是常量特判

- `vm_intrinsics.toml [facts.returns]` 中有两条：
  - `Class.desiredAssertionStatus0:(Ljava/lang/Class;)Z = false`：VM 初值，无 `-ea`；
  - `Class.desiredAssertionStatus:()Z = false`。
- 后者让分析器与生成器把所有 `X.class.desiredAssertionStatus()` 调用点折叠为 false。测试第 62/64/66/68/70 行
  需要按加载器的断言表求值（JDK 21 实测依次为 false / false / true / true / false）；折叠为常量后，这几行的输出
  与加载器状态无关，结果不对。

### 1.3 直接删掉常量特判的代价（实测 `rava closure`）

| 用例 | 类（前 → 后） | 方法（前 → 后） |
|---|---|---|
| HelloWorld | 268 → 269 | 723 → 737 |
| TestClassNestNatives | 366 → 366 | 1122 → 1130 |
| TestStackWalkerFrames | 422 → 1113 | 1468 → 6921 |

- 原因：javac 为含 `assert` 的类生成 `$assertionsDisabled = !X.class.desiredAssertionStatus()`。特判一删，
  JDK 类的 assert 体（及其消息拼接、异常构造）全部变为可达。
- JDK 21 上，引导加载器定义的类 `classLoader == null`，`desiredAssertionStatus()` 走 `desiredAssertionStatus0`。
  无 `-ea` 时结果恒 false，assert 体本就不可达。
- 所以精度缺口在分析器：它不认识「类字面量的镜像字段 `classLoader` 由 VM 写入、引导类为 null」这一 VM 状态。
  不是特判本身必要。

### 1.4 加载器层级改走字节码的代价（实测）

- 实验做法：
  - `ClassLoaders` 进 `[vm_boundary]`，按方法划分；
  - 放行 `BuiltinClassLoader` / `URLClassPath` / `ArchivedClassLoaders`；
  - 删掉 `class_loaders_impl.rs` 的三个访问点；
  - `for_class` 建 app 类镜像时**立即**取 `ClassLoaders.appClassLoader()`。
- 结果：HelloWorld 268 → 320 类（723 → 1015 方法），TestStackWalkerFrames 422 → 483 类。
- 增量来自 `ClassLoaders.<clinit>` 的整条构造链：
  - `URLClassPath` 的 `toFileURL`：`File` / `UnixFileSystem` / `ParseUtil`；
  - `ClassLoader$ParallelLoaders`：`WeakHashMap`；
  - `ReentrantLock` / AQS 等。
- 这些类只在程序读取某个镜像的定义加载器时才有用。立即填充会让每个用例（几乎都有 `getClass()`）都带上它们，
  所以填充改为**首次读取时**进行（见 §2.1）。

### 1.5 与 C1d-a（1e623cec / c1d-p0）的重叠

- C1d-a 把 `jdk/internal/loader/ClassLoaders` 列入 `[vm_boundary]`（按方法划分），并删除 `[release]` 段。
  本任务同样把 `ClassLoaders` 列入 `[vm_boundary]`；为 `BuiltinClassLoader` / `URLClassPath` /
  `ArchivedClassLoaders` 加的放行条目在 C1d-a 取消 `jdk/` 前缀截断后成为冗余，合并时随 `[release]` 一起删即可。
- C1d-a 没有改 `class_loader_impl.rs` 与 `class_loaders_impl.rs`。本任务对这两个文件的改动见 §四。

## 二、终态

1. **定义加载器是 VM 注入状态，由清单声明、生成器追加。**
   - 镜像字段 `Class.classLoader` 在 HotSpot 中由 `java_lang_Class::create_mirror` 写入。原生镜像改为首次读取时按
     定义加载器表填充：
     - `vm_intrinsics.toml [vm_state]` 声明该字段的读取钩子；
     - 生成器把对它的 `getfield` 发射为共置手写的读取钩子；
     - 闭包分析器把读取点连到钩子的手写体（`ClassLoaders.appClassLoader()` / `platformClassLoader()`）。
       程序从不读取定义加载器时，加载器层级不进闭包。
   - 定义加载器表由生成器给出：每个类块带 `defining_loader` 属性，java_meta 汇总成表。
     - 用户类与 lib crate 类为 app；
     - JDK 类按所在 jmod 的模块名查 JDK 自己的模块—加载器映射。映射取自 `ModuleLoaderMap$Modules.<clinit>` 的
       `bootModules` / `platformModules` 常量集合，类名与字段名写在清单里，生成器不写 JDK 类名。在 boot 集合
       的为 boot（null，不带属性），在 platform 集合的为 platform，其余 JDK 模块为 app。
   - 数组类取元素类的加载器；基本类型与 `void` 为 null。
2. **加载器层级 app → platform → null 按 JDK 字节码初始化。**
   - `ClassLoaders.<clinit>` 经 `BootClassLoader` / `PlatformClassLoader` / `AppClassLoader` 构造器建出三个加载器，
     `getParent()`、`ClassLoader.getClassLoader(Class)` 走字节码。
   - `ClassLoader.getSystemClassLoader()` 保留为 VM 状态承载（准入 ③）。`scl` 在 HotSpot 中由 initPhase3 的
     `initSystemClassLoader` 写入；原生二进制没有 `java.system.class.loader` 启动选项，`scl` 即内建 app 加载器，
     所以承载体返回 `ClassLoaders.appClassLoader()`，与用户类的定义加载器是同一对象。
3. **`desiredAssertionStatus` 走 `Class` / `ClassLoader` 的字节码路径。**
   - 删除 `Class.desiredAssertionStatus:()Z` 常量特判。
   - VM 初值保留为 `desiredAssertionStatus0 = false`（`-ea` 未启用）与 `retrieveDirectives` 的空指令表（native）。
   - 分析器的镜像精度：
     - 类字面量 `V::Class` 可作为常量实参求值的输入；
     - 清单声明的镜像加载器字段在类字面量接收者上，对 boot 定义的类读出 null，其余不折叠。
     - 于是 JDK 引导类的 `$assertionsDisabled` 经字节码求值仍折叠为常量，assert 体不入闭包；
       用户类、平台类的 assert 按运行期断言表执行。
4. 依赖加载器身份的行为（`getResource*`、`Class.forName(name, init, loader)`、`ServiceLoader` 的加载器参数、
   `Module.getClassLoader`）与 JDK 一致。boot layer（`Module` / `ModuleLayer` 实体）另立任务，依赖 C1d-a a2，
   不在本任务内。

## 三、验收

- `TestClassNestNatives` 原样通过。
- 新增 e2e 用例覆盖以下边界，expected 取自 JDK 21 实测输出：
  - 用户类、嵌套类、数组类（取元素类的加载器）、基本类型与 `void.class`（null）、JDK 引导类（`String`）、平台类（如 `java.sql` 未入闭包时以 `java.net.http` 等替代）的 `getClassLoader()`；
  - `getSystemClassLoader()` 与用户类加载器同一性、`getParent()` 链长度与末端 null；
  - `desiredAssertionStatus()` 对用户类 / JDK 类的取值，`setClassAssertionStatus` / `setPackageAssertionStatus` / `setDefaultAssertionStatus` 之后新加载类的取值；
  - `Class.forName(name, false, loader)` 按应用加载器查找用户类。
- `vm_intrinsics.toml` 中 `desiredAssertionStatus` 常量条目：0。
- raw-audit `non_native_overrides` = 0 保持。
- 闭包规模：HelloWorld、TestStackWalkerFrames 不因本任务增长（不读定义加载器的用例不带入加载器层级）。

## 四、实施记录与 C1d-a 合并取舍

（随实施逐步补充：改动的手写文件 / 函数、提交哈希。）

