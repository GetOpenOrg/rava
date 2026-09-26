# FS-C4：ServiceLoader 静态服务目录（方案）

> 过渡态清单 FS-C4：「服务目录恒为空，ServiceLoader 找不到任何 provider」。
> 本文给出最终态设计；当前语料无 ServiceLoader 用例，按用例触达排期。

## 一、JDK 的两条查找路径

`ServiceLoader.load(S.class)` 的迭代器依次走：

1. **模块路径**：`ModuleServicesLookupIterator`。
   - 数据来源是各类加载器的 `ServicesCatalog`，即模块 `provides` 声明。
   - 迭代时调用 `loadProvider(ServiceProvider)`：
     - `Class.forName(module, providerName)` 取得类；
     - 有公开静态 `provider()` 方法则调用它，否则用公开无参构造器实例化。
2. **类路径**：`LazyClassPathLookupIterator`。
   - 由 `ClassLoader.getResources("META-INF/services/" + S)` 逐行读出 provider 类名，再同样实例化。

现状：

- `ServicesCatalog.getServicesCatalogOrNull` 恒返回 null，所以模块路径为空。
- `ClassLoader.getResources` 恒缺席（FS-C2），所以类路径也为空。

## 二、最终态

原生二进制没有运行期类加载。按 GraalVM native-image 的同构做法，服务表**在生成期静态确定**，运行期只查表。

### 生成侧（Python）

1. **收集 provider**：
   - JDK 模块：读 jmods 里 `module-info.class` 的 `Module` 属性 `provides` 表，得到 `(service, [provider…], module)`。
   - 用户输入：读输入 jar / classes 目录的 `META-INF/services/<S>`（按行，`#` 注释），以及用户 `module-info` 的 provides。
2. **RTA 播种**：调用链上出现 `ServiceLoader.load*` 调用点时：
   - 若服务类由 `ldc S.class` 静态可见，就把 S 的全部 provider 登记为已实例化，并把它们的 `provider()` / `<init>()V` 加入 BFS 根；
   - 若服务类不可静态确定，就保守地播种全部已知服务的 provider，并在 `[raw-audit]` 中计数。
3. **发射服务表**（由 build.rs 汇总，与层次表、方法表同一属性协议）：`SERVICE_TABLE: &[(service, &[(provider, module)])]`。

### 运行侧（手写层）

1. `ServicesCatalog.getServicesCatalogOrNull(loader)`：
   - boot / platform / app 加载器返回按服务表构造的目录；
   - `findServices(S)` 返回 `List<ServiceProvider>`（ServiceProvider 是 ServicesCatalog 的内部 record，经 upcalls 声明入闭包）。
2. `loadProvider` 链上的 `Class.forName(Module, String)`、`getMethod("provider")`、`getConstructor()` 走既有反射注册表（N6 的实例化登记保证 provider 的构造器闭包存在）。
3. 类路径分支：`getResources("META-INF/services/…")` 由 FS-C2 的静态资源表承载；在资源表落地前，用户 `META-INF/services` 条目并入模块路径目录。可观察顺序与 JDK 一致：模块 provider 在前，类路径 provider 在后，同一文件内按行序。

## 三、验收

新增 e2e（期望由 JVM 生成）：

- 用户接口加两个实现：`META-INF/services` 声明顺序迭代、`stream().map(Provider::type)`、`findFirst`；
- 带静态 `provider()` 方法的实现；
- 无 provider 的服务返回空迭代；
- `reload()`。

测试资源放在 `tests/e2e/<dir>/META-INF/services/`，由 run_tests 的 javac 类路径与转译输入同时携带。

## 四、依赖与顺序

- FS-C2（资源表）不是前置条件：模块路径目录即可覆盖用户 `META-INF/services`。
- 与 FS-K1..K6（JCA Provider 服务按静态可见入选）同源，落地后 JCA 的算法登记可改走本表。
