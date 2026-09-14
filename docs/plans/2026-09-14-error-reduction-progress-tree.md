# 编译错误降至 0：问题树与推进层次

**日期**：2026-09-14（持续更新）  
**目标**：`cd output && cargo check` 输出 `error[E*]: 0`  
**当前**：6224 errors（2026-09-14）  
**基线**：8251 errors（项目启动）→ 已降 24.6%

---

## 阅读说明

```
✅ 已解决      ⚠️ 进行中      ❌ 尚未解决      🔜 待前置条件满足后执行
```

每个节点格式：
```
[状态] 问题描述（任务编号 · 优先级）
      错误类型：影响的 Rust 错误码
      根因：...
      解决方案：...
```

---

## 问题树

```
目标：cargo check 0 errors
│
├── 第 0 层：基础设施（全部 ✅）
│   ├── ✅ .class 解析器（T01）
│   ├── ✅ Rust IR 数据结构（T02/T03/T04）
│   ├── ✅ StackSim 操作数栈（T04）
│   ├── ✅ 方法级 BFS（T30）
│   ├── ✅ invokevirtual/invokestatic/invokespecial（第 5 层）
│   ├── ✅ native_impls 链接机制（T18）
│   ├── ✅ Boxing/Unboxing（T35/T36）
│   ├── ✅ Ergonomic API（T37/T38/T39/T40）
│   └── ✅ HelloWorld 字节码翻译运行（T13 + progress-tree.md 第 10 层）
│
├── 第 1 层：P0 语义级 bug（不修则生成代码永远错误）
│   │
│   └── ❌ 异常表解析与 try/catch 结构恢复（T62 · P0）
│           错误类型：生成代码语义静默错误，不表现为编译错误
│           根因：classfile.py 中 `r.skip(8)` 完全丢弃 exception table
│                 所有含 try/catch 方法只走正常路径，catch block 被静默删除
│                 JDK 大量方法（IO/反射/集合）依赖 try/catch，影响范围极广
│           解决：Step1 解析四元组 [start,end,handler,type]
│                 Step2 CFG 增加异常边 + handler_bb 标记
│                 Step3 TryCatchStmt IR 节点
│                 Step4 render.py 生成 Result 链式错误处理
│           注意：实现时参照附录"CFG 五大陷阱"提前规避已知 bug
│
├── 第 2 层：P1 高频错误修复（当前 6420 errors 的主要来源）
│   │
│   ├── ❌ JVM slot 复用 → E0425 "cannot find value"（T68 · P1）★ 最高优先级
│   │       错误类型：E0425（310 个，占比最大单一根因）
│   │       根因：JVM 局部变量槽在不同词法作用域可复用同一编号
│   │             循环体内变量退出作用域后，外部同名赋值找不到原声明
│   │       解决：追踪变量声明的嵌套深度；退出作用域后同名赋值提升为新 let mut
│   │             在 _promote_undeclared_assigns 基础上扩展深度追踪
│   │
│   ├── ❌ 内部类名注册表键规范化（T71 · P1）★ 静默失败，极难排查
│   │       错误类型：E0425（方法找不到）/ E0412（类型找不到）+ 静默错误
│   │       根因：JVM 格式 `Outer$Inner` 与 Rust 格式 `Outer__Inner` 不一致
│   │             registry 写入 `Outer__Inner`，查找时用 `Outer$Inner` → 静默 miss
│   │       解决：引入 normalize_class_key() 统一转换；扫描所有 replace('$') 调用点
│   │
│   ├── ❌ Rust 关键字转义完整覆盖（T64 · P1）
│   │       错误类型：E0532 / E0261（参数名、方法名含 Rust 关键字）
│   │       根因：Java 允许 type/ref/match/impl 作标识符，Rust 不接受
│   │             当前 safe_ident 覆盖不完整，遗漏 in/loop/move/unsafe/where 等
│   │       解决：补齐完整 Rust 保留字列表（stable + reserved），统一过 safe_ident
│   │
│   ├── ❌ 整数算术溢出语义修复（T73 · P1）
│   │       错误类型：运行时 panic（debug 模式）而非编译错误
│   │       根因：Java wrap-around 整数语义，Rust debug 模式溢出 panic
│   │             iadd/isub/imul 直接生成 +/-/*，遇溢出场景崩溃
│   │       解决：iadd/isub/imul/ladd/lsub/lmul → wrapping_add/sub/mul
│   │             ishl/ishr/iushr → wrapping_shl/shr + & 0x1f 屏蔽
│   │
│   ├── ❌ 精确 mut 标注（T54 · P1）
│   │       错误类型：E0384（cannot assign twice to immutable variable）
│   │       根因：_analyze_mutation 未递归覆盖 if/else 和 loop 嵌套块
│   │             嵌套块内的赋值未被识别，导致外层 let 未标 mut
│   │       解决：_analyze_mutation 递归进入 IfStmt/LoopStmt 的所有子块
│   │
│   └── ❌ BFS 队列改用 deque（T63 · P1）⚡ 一行改动，立即可做
│           错误类型：无编译错误，但 O(n²) 性能问题
│           根因：transpile.py BFS 用 list.pop(0)，大型类图下 O(n²)
│           解决：from collections import deque，push_back + popleft，均摊 O(1)
│
├── 第 3 层：P2 类型不匹配（E0308 主力修复）
│   │
│   ├── ❌ instanceof 语义修复（T53 · P1→P2）
│   │       错误类型：E0308（bool ← i32）+ 运行时语义错误
│   │       根因：instanceof 硬编码返回 true（语义桩），类型为 i32 不是 bool
│   │       解决：接入 ClassHierarchy（T55）后实现真实 is_instance_of 检查
│   │       依赖：T55 ClassHierarchy 完成后
│   │
│   ├── ❌ ClassHierarchy 类层次图（T55 · P2）← T53/T69 的前置条件
│   │       错误类型：不直接产生编译错误，是 T53/T69 的基础设施
│   │       根因：缺少继承图导致 instanceof/invokevirtual 无法正确解析
│   │       解决：构建 {class → [superclass, interfaces]} 图
│   │             支持 is_subtype(A, B)、all_subclasses(C)、field_lookup(C, f)
│   │
│   ├── ❌ invokevirtual BFS 完整性（T69 · P2）
│   │       错误类型：E0308（实际调用子类方法，Rust 类型不匹配）
│   │       根因：BFS 只展开声明类方法，不展开已知子类实现
│   │             运行时调用子类 override 版本，但 Rust 找不到该实现
│   │       解决：invokevirtual 额外展开 ClassHierarchy.all_subclasses(C) 的实现
│   │       依赖：T55 ClassHierarchy
│   │
│   ├── ❌ 布尔值 i32 模式压缩（T70 · P2）
│   │       错误类型：E0308（bool ← i32 比较结果）
│   │       根因：JVM 用 iconst_0/1 表示 false/true
│   │             if c {1i32} else {0i32} != 0i32 模式生成冗余比较
│   │       解决：识别 4 指令模式（if* / iconst_1 / goto / iconst_0），直接生成 bool
│   │
│   └── ❌ 语义桩显式追踪（T72 · P2）
│           错误类型：无编译错误，防止未实现指令产生静默语义错误
│           根因：instanceof/tableswitch 等用硬编码 Lit('true') 作为桩
│                 产生静默错误，代码通过编译但运行结果不正确
│           解决：引入 _semantic_stub()，通过 sim.record_stub() 汇总统计
│
├── 第 4 层：P3 类型系统 & 代码质量
│   │
│   ├── ❌ 类型系统 RsType 化（T58 · P3）
│   │       错误类型：E0308（类型推断歧义导致 Rust 无法确定类型）
│   │       根因：jvm_to_rust 返回字符串，类型信息丢失，无法做精确类型推断
│   │       解决：引入 RsType 枚举（Primitive/Named/Generic/Array/Fn）
│   │             render_type() 负责字符串化，其他模块操作结构体
│   │       注意：大型重构，需先完成 T67 IR 结构化减少字符串依赖
│   │
│   ├── ❌ switch/tableswitch/lookupswitch（T57 · P3）
│   │       错误类型：E0308 / E0425（switch 块变量丢失）
│   │       根因：tableswitch/lookupswitch 目前生成语义桩（T72 对象）
│   │       解决：生成 match 表达式，处理 default case 和 fallthrough
│   │
│   ├── ❌ 方法级 native 注入（T59 · P3）
│   │       错误类型：减少 panic!("stub:") 命中率，提高运行时覆盖率
│   │       根因：当前 native_impls 粒度是文件级，不能为单个方法注入
│   │       解决：/// @method 标注机制，emitter 按方法名分发到 _native
│   │
│   ├── ❌ 表达式优先级括号优化（T60 · P3）
│   │       错误类型：不产生编译错误，改善可读性
│   │       根因：RawExpr 拼接字符串，无法跟踪运算符优先级，产生多余括号
│   │       解决：在 Rust IR 层记录运算符优先级，渲染时按需插入括号
│   │
│   ├── ❌ $assertionsDisabled 合成字段（T74 · P2）
│   │       错误类型：运行时语义错误（断言路径意外激活）
│   │       根因：Java 编译器自动注入 $assertionsDisabled 字段
│   │             默认值 false 导致断言逻辑激活，产生额外 Result::Err 传播
│   │       解决：getstatic $assertionsDisabled → 直接返回常量 true
│   │
│   └── ❌ 并行类解析与代码生成（T65 · P3）
│           错误类型：不产生编译错误，性能优化
│           根因：879 个 JDK 类串行解析，main.py 耗时随类数线性增长
│           解决：parse-once 模式 + rayon 并行 codegen，减少重复解析
│
├── 第 5 层：长期架构（减少未来新错误的根本性修复）
│   │
│   ├── ❌ if/else 控制流结构恢复（T56 · P2）
│   │       根因：所有控制流平坦化，覆盖率瓶颈的根本原因
│   │       解决：基本块切割 → 支配树 → 结构恢复（while/if-else/match）
│   │       注意：实现时使用 CFG 五大陷阱检查清单（附录）
│   │       依赖：编译错误降至 2000 以下后再实施，避免回归难判断
│   │
│   ├── ❌ 消除 RawExpr/RawStmt（T67 · P5）
│   │       根因：IR 节点层 Raw 逃生舱占比过高，阻碍后置优化
│   │       解决：逐条将 RawExpr/RawStmt 替换为结构化 IR 节点
│   │       依赖：T58 RsType 化完成后进行
│   │
│   ├── ❌ 泛型类型精确化 LocalVariableTypeTable 驱动（T66 · P4）
│   │       根因：集合 get() 返回 Object（类型擦除），Rust 无法推断具体类型
│   │       解决：LocalVariableTypeTable 提供泛型签名，narrow 变量类型
│   │       依赖：T58 RsType 化 + T55 ClassHierarchy
│   │
│   ├── ❌ SSA 构建（T61 · P4）
│   │       根因：缺少 SSA 导致 phi 节点插入不完整，变量版本混乱
│   │       解决：Dominator tree → DF → phi insertion → SSA rename
│   │       依赖：T56 CFG 完成后实施
│   │
│   ├── ❌ 无 checked exception 方法 Result 裁剪（T75 · P3）
│   │       根因：所有方法均包装为 Result<T>，调用链产生多余 ? 噪音
│   │       解决：无 throws 声明的方法返回 T 而非 Result<T, E>
│   │       依赖：T62 异常表解析完成后再实施
│   │
│   └── 🔜 单 crate 架构迁移（T33 · P4）
│           根因：当前 3-crate workspace 有循环依赖风险
│           解决：合并为单 crate（消除 java_runtime / jdk_classes / user 边界）
│           依赖：所有手写 runtime 完成迁移（已满足）+ 编译错误为 0
│
└── 第 6 层：JDK 标准库完整翻译（端到端测试目标）
    │
    ├── ❌ IR 对象化完成（T50）—— T05/T06/T07 最终收尾
    ├── ❌ 端到端测试自动化框架（T51）
    ├── ❌ StringBuilder 字节码翻译（T43）
    ├── ❌ 虚方法派发多态测试（T44）
    ├── ❌ java/lang/String 字节码翻译（T46）
    ├── ❌ java/lang/System + PrintStream 字节码翻译（T47）
    ├── ❌ 类继承与接口实现场景测试（T48）
    └── ❌ Lambda 与匿名类翻译（T49）
```

---

## 错误类型分析（2026-09-14 快照）

| 错误码 | 描述 | 估计占比 | 主要修复任务 |
|--------|------|---------|-------------|
| E0308 | 类型不匹配（mismatched types）| ~45% | T53/T55/T58/T70 |
| E0425 | 找不到值（cannot find value）| ~25% | T68/T71/T64 |
| E0382 | 使用已移动值（use of moved value）| ~10% | T54 |
| E0412 | 找不到类型（cannot find type）| ~8% | T71/T64 |
| E0061 | 参数数量错误 | ~5% | T55/T69 |
| 其他   | E0277/E0599/E0502 等 | ~7% | 各层修复后自然消除 |

---

## 推进历史

| 阶段 | 时间 | 内容 | 错误数 | 提交 |
|------|------|------|--------|------|
| 基线 | 2026-09 前 | HelloWorld 运行 + JDK 翻译开启 | 8251 | — |
| 接口强制转换 | 2026-09 | invokevirtual null coerce | 7134 | — |
| ldc/Long/aastore | 2026-09 | ldc class + interface + 长整型算术 | 6928 | — |
| aastore/areturn | 2026-09 | Object 强制转换 | 6445 | — |
| icmp 操作数 | 2026-09-14 | _coerce_icmp_operand（u16/i8/i16 → i32）| 6420 | `method.py` |
| Object 返回类型 | 2026-09-14 | Object 接收方调用返回具体类型时 Default::default() 占位 | 6224 | `instr.py` |

---

## 任务依赖图

```
T62（异常表）─────────────────────────────→ T75（Result裁剪）
     │
     ↓（CFG 基础）
T56（if/else CFG）──→ T61（SSA）
     │
     ↓
T55（ClassHierarchy）──→ T53（instanceof 语义）
                    └──→ T69（virtual BFS 完整性）
                    └──→ T66（泛型推断）

T68（slot 复用）──→ 消除 E0425（独立，立即可做）
T71（键规范化）──→ 消除静默失败（独立，立即可做）
T63（deque）     ──→ 性能（独立，一行改动）
T64（关键字）    ──→ 消除 E0532（独立，立即可做）
T73（wrapping）  ──→ 运行时正确性（独立，立即可做）
T74（$assertions）──→ 运行时正确性（独立，5分钟可做）

T58（RsType化）──→ T67（消除 Raw）──→ T60（括号优化）
```

---

## 立即可做（无依赖，可并行）

| 任务 | 估计工作量 | 预期效果 |
|------|-----------|---------|
| T63 · BFS deque | **1 行**改动 | 性能 O(n²)→O(n) |
| T73 · wrapping 算术 | ~30 行 | 运行时溢出 panic 消除 |
| T74 · $assertionsDisabled | **2 处**改动 | 断言路径静默错误消除 |
| T64 · 关键字转义 | ~50 行 | E0532/E0261 减少 |
| T71 · 键规范化 Step3 | 扫描 + 0 行改动 | 评估静默失败范围 |

---

## 附录：T56/T62 CFG 实现时五大已知陷阱

> 来源：ruva 项目 CFG-F1/F2/F4、F-RECORD-2、CFG-ARCH-1a（已修复案例）

| 编号 | 陷阱 | 防范方式 |
|------|------|---------|
| CFG-F1 | 嵌套 try-catch 内层 catch 丢失 | 按 [start,end) 严格区分保护范围，同一 handler_pc 可有多条记录 |
| CFG-F2 | loop exit 未覆盖 switch/goto | 所有跳转指令（if*/switch*/goto）统一处理 loop exit |
| CFG-F4 | try end_pc 未加入 leader set | exception_table 的 start/end/handler 三个 pc 均加入 leader set |
| F-RECORD-2 | 离散 try 区间共享 handler 未合并 | 按 (handler_pc, catch_type) 分组合并区间 |
| CFG-ARCH-1a | handler block 未加入 successors | 受保护区间内每个 BB 的 successors 额外加 handler_bb |
