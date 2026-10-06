# C4 全量未登记失败分诊（c4-misc）

> 2026-10-07，分支 `c4-misc`（自集成分支 `rust-closure-analyzer` 3f59a8e9 拉出），worktree `java_rta_c4misc`。
> 范围：C4 全量中不在 Python 基线、合法 Java 的四例：TestSqlDateTime、TestStringGetCharsLegacy、
> TestFileCanonicalPaths、PartitionInteger。日志：`server_maintenance/rava/test_results/error_logs/<Test>_*_jdk21.log`。

## 一、结论总表

| 用例 | 失败 | 根因层 | 处理 | 提交 |
|---|---|---|---|---|
| TestSqlDateTime | `java_sql` E0034 `multiple __as_Date found` | 宏（`rava_macros_core` vtable trait 缺省方法） | 已修 | 005f98e7 |
| TestFileCanonicalPaths | 运行期 `IOException: No such file or directory (os error 2)` | 手写 native 语义偏差（`UnixFileSystem.canonicalize0`） | 已修 | 4308ba57 |
| TestStringGetCharsLegacy | 运行期 `native: BootLoader.getSystemPackageLocation` | VM 模型：命名模块 / boot layer 未物化 | 登记 known_failures（与 TestProtectionDomainFaces 同根） | 本文档同提交 |
| PartitionInteger | 运行超时 900 s | 运行性能（R1）+ 引用环泄漏（无环回收） | 非死循环；内存模型取舍待决（§五） | — |

## 二、TestSqlDateTime：祖先与本类简单名相同时钩子调用歧义

- 现象：`java.sql.Date extends java.util.Date`，两者的 vtable trait 都声明 wrapper 重建钩子 `__as_Date`
  （钩子按类定义名生成，`context.rs` `as_self_hook`）。`java.sql.Date__VTable: java_util_Date__VTable`，
  本类缺省方法体里 `self.__as_Date()` 以方法调用语法解析时，本 trait 与 supertrait 各有一个同名候选 → E0034。
- gen-lc-naming（1b3c26d1）只处理类模块名与类型名分离、接口跨层重载命名，未覆盖本问题。
- 修复：`trait_decl.rs` 两处（有体非 Safe 方法、手写体方法）改为本类 trait 限定调用
  `Date__VTable::__as_Date(self)`；其余钩子调用点（`erasure.rs`、`phase2/inherited.rs`、`__dyn_<X>` 视图）早已限定。
- 验证：`rava_macros_core` 单测 16/16；e2e 抽查见 §六。

## 三、TestFileCanonicalPaths：canonicalize0 要求路径存在

- 现象：`new File(root, "sub/../x.txt").getCanonicalPath()`，`sub` 不存在。手写 `canonicalize0` 直接
  `std::fs::canonicalize`（realpath），ENOENT 即抛 IOException。
- JDK 语义（`canonicalize_md.c` `JDK_Canonicalize`）：整条 realpath 失败时从末尾逐段去名，ENOENT / ENOTDIR /
  EACCES 继续、其他错误失败；取能 realpath 的最长前缀接回未解析尾部，再 `collapse` 语法消解 `.` / `..`；
  一段都解析不了返回原路径（同样 collapse）。路径不必存在。失败消息按
  `JNU_ThrowIOExceptionWithLastError(env, "Bad pathname")`：errno 文案优先。
- 修复：`unix_file_system_impl.rs` 逐句移植 `JDK_Canonicalize` / `collapse` / `splitNames`（含 PATH_MAX 检查），
  消息取 `net_posix::strerror`。本机以独立 rustc 小程序对照 JDK 21：`<tmp>/sub/../x.txt`、`/nonexist/a/../b`、
  `/a/..`、`/tmp/./q/./../r`、`<文件>/y` 五例输出与 `File.getCanonicalPath` 逐字相同。

## 四、TestStringGetCharsLegacy：与 TestProtectionDomainFaces 同根，登记已知

- 调用链：`String.class.getPackage()` → `BootLoader.definePackage` → `getDefinedPackage` →
  native `getSystemPackageLocation`（ACC_NATIVE，未手写）。
- 只补 native 不够：HotSpot 对命名模块的包返回 `jrt:/<模块名>`，随后 `PackageHelper.findModule` 经
  `Modules.findLoadedModule(mn)` 在 boot layer 中找模块，找不到即 `InternalError(mn + " not loaded")`。现模型下
  `ModuleLayer.boot()` 是空层、全部类在无名模块（`module_layer_impl.rs` / `module_impl.rs`），补 native 只会把
  panic 换成 InternalError。终态随引导映像第 4 步（命名 java.base、非空 boot layer）与第 5 步（BootLoader 2）
  一并落地：native 按包所属引导模块返回 `jrt:/<模块>`（包到模块的映射来自同一份模块图事实），
  与 c1d-url-b2 B5 的「非 null 结果以 `jrt:/` 开头」清单事实一致。
- 登记：`docs/known_failures.toml` TestStringGetCharsLegacy，签名同 TestProtectionDomainFaces。

## 五、PartitionInteger：非死循环；运行性能 + 引用环泄漏

- 负载（JDK 21 实测插桩）：`findCombo` 叶子 12,347,833 个，每个叶子建一条 `Arrays.stream(combo).map(..).sum()`
  流水线；JVM 0.88 s。主要量在 `partition(40355, 3)`（首素数 2 的全部二元组合约 900 万）与 `partition(22699, 3)`。
- 不是死循环：循环全部有界；R1 起点记录里 release 档「运行 OOM-kill，1m34s 峰值 19.8 GB」，即持续前进并持续分配。
- 内存：每条流水线是引用环——`AbstractPipeline` 头结点 `sourceStage = this`（自环），各阶段
  `previousStage` / `nextStage` 双向链接。对象模型为纯引用计数（`Runtime.gc()` 注释「引用计数即时回收」），
  无环回收，每条流水线（连同 spliterator、sink 链）永不释放；19.8 GB / 1235 万 ≈ 1.6 KB/条，量级吻合。
- 时间：debug 档 900 s / 1235 万 ≈ 73 µs/条为上限，每条流水线十余个对象、每字段一次 Arc 分配，属 R1 运行性能线
  （`docs/plans/2026-09-30-optimization-directions.md` §4 附加验收用例已含本例）。
- 待决（产品 / 架构取舍，停在此项）：Java 程序普遍依赖 GC 回收环（流水线、双向链表、父子指针、监听器），
  纯引用计数下这类程序内存无界增长。选项：
  1. **同步环回收（试删除，Bacon–Rajan）**：在引用计数之上加候选根缓冲，减计数未归零的对象入缓冲，
     定期（按缓冲量 / 分配量）做标记-扫描试删除。保留即时回收与确定的析构时机，改动集中在运行时对象句柄
     （扫描需要逐对象枚举引用字段，可与 S7「统一句柄 + 每类静态描述符」的字段布局表共用）。
  2. **追踪式 GC**（标记-清扫 / 分代）：语义最接近 JVM，但需要精确根（栈上引用枚举），与现有 `Arc` 句柄、
     手写层的直接持有冲突，改造面覆盖生成器与全部手写层。
  3. 维持纯引用计数，把「环不回收」写入兼容性说明：PartitionInteger 一类用例在长运行下不可达终态。
  - 建议：选项 1，作为运行时对象模型项立项；验收为 PartitionInteger release 档峰值内存 ≤ 200 MB、
    输出与 JVM 一致，debug 运行段随 R1 达到 ≤ 30 s。
- 已知失败口径：超时不可登记（`known_failures.toml` 文件头），本例不登记。

## 六、验证与抽查

- 本机：`rava_macros_core` 单测（`cargo test --release`，独立 target）通过。
- 抽查：见下（推送后补记）。
