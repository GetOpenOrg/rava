# 任务管理（当前活跃）

> 创建：2026-09-16（取代已归档的 `docs/tasks-history.md`）  
> 定位：**只保留开放的问题与任务**。完成即关闭删除，不累积历史。  
> 历史任务（T01-T81 全记录）见 `docs/tasks-history.md`。

**关联追踪文档**（细粒度证据与状态）：
- `docs/plans/2026-09-15-e2e-unresolved-issues.md` — e2e 失败普查 + 问题条目（A~N 系列），**当前主线**
- `docs/plans/2026-09-15-cfg-ifelse-unresolved.md` — CFG 翻译质量
- `docs/jvm-attributes-checklist.md` — JVM 属性解析覆盖

**基线（2026-09-16 全量实测）**：60 e2e，10 通过 / 50 失败（47 编译 + 3 diff）。

---

## P0 · 错误家族攻坚（性价比最高）

| 任务 | 影响 | 说明 |
|------|------|------|
| E0433 `crate::error` 缺失 | ×5 测试 | 疑似单点修复，先攻 |
| E0599 `__get_value` on i32 | ×6 测试 | 拆箱（unboxing）翻译缺口，含 H-1 |
| E0308 类型不匹配 | ×10 测试 | 见 e2e 文档普查表 |
| E0424 非关联项 | ×8 测试 | 见 e2e 文档普查表 |

## P1 · 功能缺口

| 任务 | 来源 | 说明 |
|------|------|------|
| method reference | Arch-3 / T49 | `System.out::println` 等 + `ArrayList::<init>()` 构造器引用（E0425 ×2）；主 lambda 已通 |
| 异常处理 try/catch/finally | T45/T62 / cfg-ifelse #6 | codegen 零实现，异常表未解析 |
| `Record` 属性解析 | checklist 唯一待办 | 直接关联 TestRecord E0615 |

## P2 · 翻译质量

| 任务 | 来源 | 说明 |
|------|------|------|
| 短路逻辑 `\|\|`/`&&` | cfg-ifelse #7 | 未做 |
| ternary 方法调用物化 | cfg-ifelse #2/#4 | 分支含语句时退化为顺序代码 |
| H-1 primitive 数组 autobox | e2e H-1 | 被 `__get_value` 编译错误阻塞，随 P0 解锁 |
| H-2 TestRecord 输出格式 | e2e H-2 | 被 E0615 阻塞，随 Record 解锁 |
| E-1 `_impl.rs` 方法级去重 | e2e E-1 | 生成前扫描手写 companion 跳过重名方法（架构防御） |
| D-2 `areturn` Default 兜底 | e2e D-2 | 等 Arch-3 完成后删除 |

## P3 · 长期重构（不阻塞主线）

| 任务 | 来源 | 说明 |
|------|------|------|
| IR 结构化（消 RawExpr/RawStmt） | T05/T06/T07/T50/T58/T61/T67 | 架构级，收敛口径见历史文档 T67 |
| slot 复用与作用域追踪 | T68 | E0425 根因修复 |
| 泛型精确化 | T66 | LocalVariableTypeTable 驱动 |
| mut 标注递归覆盖 | T54 | 嵌套块 |
| 括号优化 | T60 | 表达式优先级 |
| 布尔压缩 | T70 | `if c {1i32} else {0i32}` → bool |
| 语义桩计数 | T72 | 指令级统一标记未做 |
| 并行类解析 | T65 | 测试级并行已实现（run_tests.py），类级未做 |
| Arch-6 `_rust_type_to_binary` | e2e Arch-6 | Python 侧字符串逆查，低风险 |
| A-2 间接子类传参 | e2e A-2 | 潜在未触发，等触发再修 |

---

## 维护规则

1. 完成一项 → 从本表删除，证据（commit / 测试结果）记入对应追踪文档
2. 新发现问题 → 入表并标注来源（e2e 普查 / 代码审查 / 实验发现）
3. 每 e2e 全量运行后刷新 P0 家族数字
4. 本文档不记历史——需要查完成记录去历史文档或 git log
