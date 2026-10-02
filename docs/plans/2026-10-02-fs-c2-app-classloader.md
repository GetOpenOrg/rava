# FS-C2：应用类加载器非空与断言状态按加载器求值

> 状态（2026-10-02）：🔄 实施中，分支 fs-c2（基于集成分支 229a253b），由原 native-gaps 子代理负责；完成后接着做 boot layer（另需 C1d-a a2）。
> 关联：[`2026-09-29-rust-closure-analyzer.md`](2026-09-29-rust-closure-analyzer.md) §六 C1d 行、
> [`docs/reference/handwritten-boundary.md`](../reference/handwritten-boundary.md)（VM 注入状态属准入第 ③ 类）。

## 一、现象

- 用户类的 `Class.getClassLoader()` 返回 null（等同引导加载器）。JDK 21 下应用类由 `ClassLoaders$AppClassLoader` 加载，返回值非空。
- `TestClassNestNatives`（e2e，native-gaps 分支新增）对用户类调用 `getClassLoader()` 时 NPE，作为 FS-C2 已知失败保留。
  该测试写法合法、JDK 21 可运行，**不改测试、不改 expected**（2026-10-02 用户决定：子代理曾改用自定义加载器绕开，已回退）。
- `Class.desiredAssertionStatus` 在 `vm_intrinsics.toml` 中硬编码为 false，而不是按 `ClassLoader` 的断言表求值。

## 二、终态

1. 类镜像携带定义加载器（VM 注入的隐藏字段，由清单声明、生成器追加）：
   - JDK 类为 null（引导）或平台加载器，按模块归属分配，与 JDK 21 一致；
   - 用户类与 lib crate 类为系统 / 应用加载器 `ClassLoaders.appClassLoader()`。
2. `ClassLoader.getSystemClassLoader()`、`getParent()` 链（app → platform → null）按 JDK 字节码初始化，不手写近似对象。
3. `desiredAssertionStatus` 走 `Class` / `ClassLoader` 的字节码路径：读加载器的 `classAssertionStatus` / `packageAssertionStatus` / `defaultAssertionStatus`。
   VM 初值（`-ea` 未启用 → 全 false）作为 VM 注入状态登记在清单中，`vm_intrinsics.toml` 中的常量特判删除。
4. 依赖加载器身份的行为（`getResource*`、`Class.forName(name, init, loader)`、`ServiceLoader` 的加载器参数、`Module.getClassLoader`）一致。

## 三、验收

- `TestClassNestNatives` 原样通过。
- 新增 e2e 用例覆盖以下边界，expected 取自 JDK 21 实测输出：
  - 用户类、嵌套类、数组类（取元素类的加载器）、基本类型与 `void.class`（null）、JDK 引导类（`String`）、平台类（如 `java.sql` 未入闭包时以 `java.net.http` 等替代）的 `getClassLoader()`；
  - `getSystemClassLoader()` 与用户类加载器同一性、`getParent()` 链长度与末端 null；
  - `desiredAssertionStatus()` 对用户类 / JDK 类的取值，`setClassAssertionStatus` / `setPackageAssertionStatus` / `setDefaultAssertionStatus` 之后新加载类的取值；
  - `Class.forName(name, false, loader)` 按应用加载器查找用户类。
- `vm_intrinsics.toml` 中 `desiredAssertionStatus` 常量条目：0。
- raw-audit `non_native_overrides` = 0 保持。
