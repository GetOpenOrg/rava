# ruva 兼容模型参考（评估结论与远期素材）

> 日期：2026-09-21
> 来源：外部文档《ruva 转译等价层级模型》v1.1（`ruva-compat-model_1.md`）
> 定位：**参考资料，非实施计划**。记录对该模型的评估结论（已采纳 / 已验证 / 不采纳）与远期可用的设计方案。等价等级标注已吸收进 `2026-09-19-remaining-issues.md`。

---

## 一、评估结论总表

| 模型内容 | 结论 | 去向 |
|---|---|---|
| 六级等价分类 + 传递性规则 | **已采纳** | remaining-issues「等价等级标注」章节；166 测试归类框架 |
| 反射族静态注册表路线（`inventory` / 层次表） | **已采纳为终态方向** | `Class::for_class` + `CLASS_HIERARCHY`（build.rs）已验证；`Method.invoke`/`Field.get` 按需立项 |
| `Rc::ptr_eq` 引用相等、wrapping 算术、`Result` 异常、try-with-resources RAII、`Condvar` wait/notify | **已验证殊途同归** | 本项目已有等价或更强实现（异常对象为真实 Throwable，强于模型的 `JException` 结构体） |
| 单继承组合 + `Deref` | **不采纳** | 本项目 vtable 双指针 + `MethodKind::Inherited` 转发，多态保真度更高 |
| `String` = `Vec<u16>` | **不采纳** | 本项目 String 走字节码翻译，保留 compact strings（Latin1/UTF16） |
| `null` = `Option<T>` | **不采纳** | 与 Object 模型冲突；null 缺口按 S-2/S-3 单独修 |
| 「内部 API 不支持」三层边界 | **不采纳** | 哲学相反：本项目翻译 JDK 自身字节码，`jdk/internal/`/`sun/` 走边界类按需手写（K-3） |
| `synchronized` = `Mutex<T>` | **不采纳（本项目更保真）** | `InternalLock` 用 `parking_lot::ReentrantMutex`——Java monitor 本来就可重入 |

---

## 二、中期参考：告警规范与 `--deny` 机制

> 对应本项目 `[cfg-audit]` / `[bfs-audit]` / `[readability-audit]` 审计行的演化方向（V-3 的终态形态）。

模型的设计要点：

1. **近似等价 / 条件等价不允许静默编译通过**——强制输出结构化告警：

   ```
   [warning] NearApprox: <特性名称> at <源文件位置>
     Behavior: <偏差描述>
     Safe when: <安全使用条件>
     Risk: <可能引发的问题>
     See: <文档锚点>
   ```

2. **废弃 API 告警为独立正交类别**（与近似等价告警不合并，同一特性可同时触发两类）。

3. **`--deny` 分级升级**：默认告警放行编译；`--deny near-approx` 全局升级为错误；`--deny near-approx::cycle-ref` 细粒度升级（对齐 Rust lint 的 deny 模型）。

对本项目的落地形态（待 166 基线后立项）：近似/条件等价语义缺口命中时（codegen 可检测的：如 `monitorenter` 生成、identity hash 路径、null 数组比较），在转译输出中记录结构化告警行，`run_tests.py` 汇总为 `[equiv-audit]`；红线跑批默认放行，`--deny equiv` 时升级。

---

## 三、等价比例估算（java.base 的预期基调）

```
规格等价   ~60%   基础运算、并发原语、RAII
行为等价   ~25%   异常、对象模型、数组
语义等价   ~10%   反射、字符串、继承
近似等价    ~4%   循环引用、finalize、动态代理
不可等价    ~1%   自定义 ClassLoader、Agent
```

对 166 测试归类的含义：约 95% 的 java.base 用例应达到语义等价以上——失败测试中预期只有少数落在已知近似等价边界（见 remaining-issues 边界表），其余均为可修的真缺陷。此比例是外部模型的估算，仅作预期基调，以本项目实测为准。

---

## 四、远期参考：框架适配层（v2+，java.base 之后）

> 核心原则：**将 Java 在运行时做的事，转移到 Rust 编译期完成**（编译期发现 → 编译期生成 → 编译期注入；工具：过程宏 + 代码生成 + `inventory` 注册表）。

| Java 动态机制 | Rust 静态替代 | 复杂度 | 等价级别 |
|---|---|---|---|
| `@ComponentScan`（运行时 classpath 扫描） | 每个组件类生成 `inventory::submit!` 注册，容器启动遍历注册表 | 低 | 语义等价 |
| `@Transactional` / AOP（运行时代理） | 过程宏编译期识别注解，直接生成包装函数（事务 begin/commit/rollback 内联） | 中 | 语义等价 |
| 通用 AOP 切面 | 过程宏 + 静态切点解析（切点表达式必须编译期可解析） | 高 | 语义等价 |
| Hibernate 懒加载代理（运行时子类化） | 字段改 `OnceCell<T>` + 生成 `get_or_try_init` 懒加载 getter | 中 | 语义等价 |
| 动态字节码增强（dirty 标记注入） | 过程宏为 `@Column` 字段生成 setter + `DirtyTracker` 注入 | 中 | 语义等价 |
| 运行时注解元数据读取 | 编译期元数据结构体 + 注册表 | 低 | 语义等价 |

**硬边界（扩展层也不可支持）**：AOP 切点目标运行时动态变化；运行时加载编译期完全未知的类（外部 jar / 网络字节码）；OSGi 热插拔；Java Agent 插桩。

**语义类型判定规则**（比"哪个框架"更本质的边界）：
- ✅ 公开 API + 静态语义 → 无论 java.base 还是第三方，均在核心层支持范围
- ❌ 公开 API + 动态语义（`Class.forName(变量)`、`Proxy.newProxyInstance`、classpath 扫描）→ 扩展层，需框架适配
- ❌ 内部 API（`sun.*` 直接调用）→ 本项目例外：翻译 JDK 自身时内部类在范围内；**用户代码**直接调内部 API 才是边界

---

## 五、转译决策树（对新特性落地时的判断顺序）

```
1. Rust 原语满足 Java 规格全部约束？          → 规格等价，直接映射
2. 运行时库可构造一致的外部可观测行为？        → 行为等价，调用运行时库
3. 输入输出/异常一致，内部机制允许不同？       → 语义等价
4. 等价性取决于运行时条件（对象图/环境）？     → 条件等价，分档 + 结构化告警
5. 偏差可明确列举且仅限特定场景？             → 近似等价，强制告警
6. 以上皆否                                  → 不可等价，拒绝编译并报错
```
