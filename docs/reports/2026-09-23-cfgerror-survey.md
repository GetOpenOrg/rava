# 两固定 CfgError stub 方法的形态定性报告（纯调查，零修改）

> 2026-09-23，纯调查代理交付（主会话入档）。证据链三方闭合：javap 字节码 ×
> 代码逻辑推演 × 既有仪表日志（fallback-audit 的 /tmp/java_rta_audit 抽样产物）。

## 一、两方法的异常表形态（JDK21 javap 实测）

### `java/util/AbstractMap.equals(Object)Z`

```
Exception table:  from   to  target  type
                    39   115    140   ClassCastException
                    39   115    140   NullPointerException
                   116   133    140   ClassCastException      ← 同 handler 双类型 ×3 区间
                   116   133    140   NullPointerException
                   134   137    140   ClassCastException
                   134   137    140   NullPointerException
```

源码形状：`try { for (Entry e : entrySet()) { ...两个 return false 断开循环体... } } catch (CCE|NPE) { return false; }`。javac 的受护区间精确化把**不抛异常的指令排除出区间**：两个 `return false`（114-115、132-133 的 ireturn）与 continue goto（134: `goto 49` 回循环头）被切出，一个源码 try 拆成 3 个不相连区间。handler 140 = `astore; iconst_0; ireturn`（返回 false，athrow 式终结，不汇回正常流）。

### `java/io/ObjectInputStream$BlockDataInputStream.readBlockHeader(Z)I`

```
12-35, 36-83, 84-107, 108-115, 116-174, 175-234, 235-238  → 全部指向 handler 238, 仅 EOFException
```

同型放大版：源码是 `do { ...switch 重试循环... } while` 包在 try 里，6 个 `ireturn`（35/83/107/115/174/234）+ 尾部 retry 回边（235: `goto 12`）把区间切成 7 段。注意末区间 `[235,238)` 的 end_pc == handler_pc == 238（TryCatchPlan 只在 `end_pc > handler_pc` 时截断，该区间原样保留），区间内**只有一条 goto**。handler 238 = `astore; new StreamCorruptedException; athrow`（纯 throw）。

## 二、卡点精确定位（与既有仪表日志互证）

审计日志直接给出炸点：**equals 卡 `pc=134`、readBlockHeader 卡 `pc=235`**——恰好都是两个方法的**最后一个区间的首 pc，且都是单条 goto 回边**（134: goto 49 = for 的 continue；235: goto 12 = do-while 的 retry）。

根因链（五步）：

1. javac 区间精确化 → 一个 Java try 拆成多个不相连区间共享 handler（`codegen/method/try_catch.py:66-83` 按 handler_pc 聚合，`_merge_ranges` 只合并数值重叠区间 `s<=e`，134>133 / 235>234 均不合并 → TryGroup 携带 3/7 区间）。
2. `blocks.py:474-476` 只为**组首区间**（`g.start_idx`）装 try 节点，`try_entries` 保护集也只收首区间体入口块。
3. **致命一步**：`blocks.py:536-544 _thread_jumps` 把「单条 goto 的块」当 trampoline 消除——块 134/235 恰好是单 goto 块、不在 `try_entries`/`handler_bind` → 所有指向它们的边（equals: 111/129 的 `ifne 134`；readBlockHeader: 190 的 `goto 235`）被改指 goto 终点（49/12），块死亡、不入活图。
4. `codegen.py:141-144 _split_disjoint_try_ranges` 按序处理 `ranges[1:]`：equals 的 116、readBlockHeader 的 36/84/108/116/175 都是实代码块（aload 起头）补装成功，轮到最后区间时 `by_start_pc.get(134/235)` → None → `CfgError("try 区间 pc=N 缺少可作体入口的块")`。
5. `class_writer.py:427` 吞异常 → `panic!("stub: ...")`（产物实证：abstract_map.rs:373-374，且 hash_map.rs:1307-1308 的 `AbstractMap__equals_base` 委托意味着 HashMap 实例调 equals 即 dispatch 到此 panic）。

「缺体入口块」不变量的确切含义：**docstring 的前提「build_blocks 以区间两端为 leader，该块恒存在」在 trampoline 场景不成立**——区间首块是单 goto 时，模拟层的跳转线程化先把它从图里删了。

## 三、与 record 卫兵形态的异同

相同点：多区间共享同一 handler 的 TryGroup、handler 均为 athrow 式私有子图——**数据结构层同型**，这是它们都进 `_split_disjoint_try_ranges` 的原因。

本质差异（325d7da 未覆盖的原因）：

| 维度 | record 卫兵 | equals / readBlockHeader |
|---|---|---|
| 区间断点成因 | JEP 440/441 逐访问器卫兵（每访问器独立 3 指令区间） | javac 把不抛异常指令（xreturn/goto）剔除出区间 |
| 区间首块内容 | 恒为 `aload+invoke+istore` 实代码（13/22） | **末区间首是单条 goto 回边**（134/235），其余区间首是实代码 |
| trampoline 可消除性 | 不可（多指令实块） | **可**（单 goto 块恰好满足 `blocks.py:541-543` 全部条件） |

即：record 形态下补装前提恒成立；这两方法的形态差在「区间收尾于控制流回边」——javac 对循环的区间拆分必然以 continue/retry goto 开新段，该段首块注定被线程化消除。

## 四、修法设计（不动码→主会话实施）

**主修（方案 C）：TryCatchPlan 源头合并「裸 return 间隙」**。两方法的区间间隙经 javap 逐 pc 核验**全部恰好为单条 xreturn**（equals: 115、133；readBlockHeader: 35/83/107/115/174/234 全 ireturn）。在 `try_catch.py` 聚合后追加合并步：同 handler 的排序相邻区间 (s1,e1),(s2,e2)，若 `[e1,s2)` 内全部指令均为 xreturn，合并为 (s1,e2)。JVM 语义安全：xreturn 不抛异常、终结控制流，并入受护区间不改捕获行为；与 `_adopt_bare_returns`（blocks.py:520，把单前驱裸 return 块归组）方向完全一致。合并后两方法均退化为**单区间 try**：equals → `[39,137)` = 「try 包 for 循环」；readBlockHeader → `[12,238)` = 「try 包 do-while retry」——与 Java 源码形状同构，走既有全绿路径，134/235 的 goto 块在区间内照常被 trampoline（continue 在 try 内，异常语义正确）。record 形态零影响（其间隙是 iload/istore/分支，非 xreturn，不满足合并条件，继续走 325d7da 分裂路径，`multi` 判定自然分流）。量化目标：`_split_disjoint_try_ranges` 的 CfgError 触发数 → 0，stub_fallback（大闭包）2 → 0。

**兜底（方案 B，分级实施）**：若未来出现「间隙含非 return 不抛指令」的多区间混合形态，分裂框架仍会在 goto 首块上炸——可在 `_install_try_nodes` 把多区间组非首区间的首块 id 并入 trampoline 排除集（享 `try_entries` 同等待遇），保留块供补装。不推荐首选：需结构化器消化「T_j 体 = 单 continue goto」新形态（continue 穿透 java_try! 宏、Break 标签跨 try 层——`structure.py:178` 的 parent_try 修复正是为同组多区间标签问题而设，此路径需另行验证）。

## 五、回归风险评级：低（两方法均是）

- **stub 语义**：`panic!("stub: ...")`，触达即 panic（非 Err 传播）。此前「测试绿」= 运行时未触达，不是语义近似。
- **静态可达但动态未触达**（三方证据）：① equals 在调用链上（否则走 class_writer.py:412 裁剪 stub、不进 stub_fallback 计数）；② 三个大闭包语料源码**零 `.equals(` 调用**；③ 全部 e2e 语料无「对 Map 实例整体调 equals」断言（grep 命中均为 String/Integer/enum/record/Optional/自定义类）。readBlockHeader 仅 ObjectInputStream 反序列化 block-data 路径触达，语料无 Java 序列化 round-trip。
- **修复后行为变化面**：HashMap/TreeMap/LinkedHashMap/WeakHashMap 均不覆写 equals（javap 实测 0 覆写；EnumMap 覆写），运行时对这四类 Map 的 equals 调用会经 `AbstractMap__equals_base` 委托走到新真体。绿测试的调用图与执行路径不变 → 不存在「由绿变红」的机制性风险；唯一残余风险 = 真体自身转译正确性（equals 含迭代+双分支+CCE/NPE 双 catch；readBlockHeader 含 tableswitch+异常构造，均为既有支持形态），由既有 cfg-audit（try_regions 应 +2）/equiv 审计线 + 全套 e2e（重点 33_maps、24_object_methods）+ 大闭包三件套比对覆盖。
- **受益**：消除 7680 语料级闭包中每闭包固定 2 个语义缺口；未来任何 `map1.equals(map2)` 断言的语料从必 panic 变可用。

## 关键文件

- `codegen/method/codegen.py:42-170`（`_split_disjoint_try_ranges`，L141-144 即炸点）
- `codegen/method/try_catch.py:62-140`（聚合与 `_merge_ranges`，方案 C 落点）
- `codegen/method/blocks.py:445-516`（try 节点安装，仅首区间）、`:520-534`（`_adopt_bare_returns`）、`:536-564`（`_thread_jumps` trampoline，根因第 3 步）
- `codegen/cfg/structure.py:167-200`（parent_try 支配链最深规则，B 方案关联）
- 证据：`/tmp/java_rta_probe_{abstractmap,ois,recpat}.txt`、`/tmp/java_rta_audit/logs/*_instr.log`、`/tmp/java_rta_audit/i_teststreamadvanced/.../abstract_map.rs:373`
