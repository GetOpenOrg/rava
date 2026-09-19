# trace_callchain.py 完整版实现(2026-09-19)

## 背景

对 `docs/trace_callchain_analysis_principle.md` 的逻辑评审发现:文档描述的"五条引用发现通道"
是基线版与 0919 版的**并集**,没有任何一个现存脚本版本同时满足;且代码里存在四处影响正确性的缺口。
本计划将 `scripts/trace_callchain.py` 重写为文档描述的完整版本(历史版本 0917/0919 保留作对照,不再演进)。

## 评审发现的缺口(→ 实现内容)

| # | 缺口 | 实现 |
|---|------|------|
| 1 | 字段指令的字段描述符类型通道在 0917 被移除(`_fref_type_desc` 仅存在于基线版) | 恢复:字段指令同时提取属主类与描述符类型 |
| 2 | `multianewarray`(0xC5)只跳长度不提取元素类型 | `_elem_cls`:剥 `[` 前缀提取元素类名 |
| 3 | 出队只在**本类**方法表匹配,继承方法/default 方法断链且静默丢弃 | `find_declaring`(JVMS §5.4.3.3/§5.4.3.4 近似):父类链 → 接口闭包 default → 同名 native 兜底(签名多态);`<init>/<clinit>` 不参与 |
| 4 | `get()` 缓存命中提前返回,`hier_super/hier_ifaces` 不随 `shared_cache` 重建 → two-pass 第二程 `is_subtype` 退化为 `sub==sup`,接口分派整体丢失 | 缓存命中即回填层次索引;`is_subtype` 惰性解析中间类 |
| 5 | `<clinit>` 只由 new 近似触发,且"出队时顺带检查"有时序缺口(类的方法全部出队后才被实例化 → clinit 永不展开) | 三类触发(new / invokestatic / getstatic·putstatic 属主)+ 父类链递归(`_init_class`)+ 触发即入队;`initialized` 与 RTA 的 `instantiated` 分离 |
| 6 | 方法表只提取 ACC_NATIVE | 增加 ACC_ABSTRACT / ACC_STATIC(7 元组),报告统计 |
| 7 | VTA 仅覆盖 invokevirtual | 接收者追踪覆盖 invokeinterface |
| 8 | 解析失败静默 | `unresolved` 集合随报告输出;签名多态方法归入 native 边界而非 unresolved |

结构变化:`_scan` 返回 `ScanResult`(NamedTuple,vcalls/dcalls/scalls/new_cls/sfield_cls/other_refs);
`bfs()` 返回 `BfsResult`(visited/all_classes/native_stubs/instantiated/cache/initialized/unresolved)。

## 验证(全部通过,16 项)

- `py_compile`;定向探针(编译到临时目录,jmods 解析):
  - A 组(扫描级):字段声明类型 / catch 类型 / ldc Class / multianewarray 元素类型 /
    字段指令描述符类型(System.out→PrintStream)/ 标志元组位
  - B 组(BFS 级,two-pass):RTA 接口分派命中 ArrayList.add / invokestatic·getstatic·new 三类
    `<clinit>` 触发 / 继承方法 toString 方法体展开(StringBuilder 仅可来自该路径)/
    热缓存 hier 重建回归 / 签名多态归 native 边界
  - C 组(受限域判别):白名单 resolver 把闭包缩到 ArrayList/List 等,`List.add`→`ArrayList.add`
    的 RTA 分派成为该键唯一来源——新版命中,0919 冷/热均未命中(证实其层次索引退化)
- HelloWorld 全量(6 模式 + 报告),与 0919 对比,日志:
  `/tmp/trace-complete-helloworld.log`、`/tmp/trace-0919-helloworld.log`

## 关键行为变化(HelloWorld, Two-pass RTA 基准)

- two-pass 由"退化偏小"变为**超过单程**(hier 修复后第二程找回接口分派目标):
  单程 2456 类/23331 方法 → two-pass 2550 类/26315 方法(0919 的 two-pass 小于其单程);
  详见两份日志的模式对比表。

## 文档同步

`docs/trace_callchain_analysis_principle.md` 同步修订:§0 版本说明与引用口径、2.3 标志提取、
2.4 multianewarray 行、3.4 层次索引生命周期、3.5 VTA 接口覆盖、**新增 3.6 方法解析**、
第五节 `<clinit>` 完整触发语义、第七节模块清单(+MethodResolver,10 项)、第八节局限表重排
(2/3/6 已修复)、9.1 对照表修正(模块清单 9/10,消除与 3.5 行的矛盾)、9.2 移出 JVMS 解析条目。

## 后续(不在本次范围)

- invokedynamic/BootstrapMethods(主流程已实现,脚本侧见局限表第 1 条)
- 反射调用、注解处理器(局限表第 4/5 条,两侧均未处理)
