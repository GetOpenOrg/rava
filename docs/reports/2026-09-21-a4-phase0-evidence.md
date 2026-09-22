# A-4 阶段 0 证据：from_any 残余形态分段 + 接口类型位置擦除点清单

> 日期：2026-09-21（分支 `fix/a4-carrier-type-positions`，基于 main @ 74f70dd）
> 方法：44 个测试 build（任务验收集 25 例 + 补充重载例 19 例）`--no-run --clean` 转译后，
> 对带 `java_rta_macros` 标记的生成文件做 occurrence 级分类（脚本 `/tmp/a4/classify.py`，
> 按发射点代码指纹：`_mergedN =` → blocks.py:208；`let _tN =` → invoke*.py 返回对齐；其余 →
> 实参/回退位），JDK 21。

## 1. from_any 分段（总数 2538，44 build）

| 段 | 计数 | 占比 | 发射点 | 典型形态 | 归属 |
|---|---|---|---|---|---|
| **let-align/val-call** | **1580** | 62.2% | `invoke.py:309/336`、`invoke_virtual.py:622`（`_erased_ret_is_type_var`） | `let _t2 = Object::from_any(this.next()?)`、`Object::from_any(map.put(..)?)` | **A-1 取值端（勿误伤）**——类型变量返回的幂等装箱，与接口无关 |
| **merge-box/val-clone** | **419** | 16.5% | `method/blocks.py:208` | `_merged1 = Object::from_any(Clone::clone(&super__m_base::<T,_>(..)?))` | **A-4 域（合并槽目标位）**——合并槽 Java 静态类型为接口（trySplit→Spliterator 等协变三元）或 Object |
| **merge-box/val-ctor** | **242** | 9.5% | `method/blocks.py:208` | `_merged1 = Object::from_any(Clone::clone(&X::new(..)?))` | **A-4 域**——构造结果装箱入 Object 合并槽（from_any 装成 JvmRef，丢 vtable） |
| let-align/val-other | 189 | 7.4% | invoke*.py | super/桥接调用返回对齐、clinit 静态装箱（date_time_formatter `dow.put` 族） | 混合（多为 A-1 取值端上下文） |
| inline-arg（val-clone/call） | 108 | 4.3% | coerce 回退 / 根方法 on 基本类型接收者 | `Object::from_any(this.get(i)?)` 作实参 | A-1 取值端为主 |

**结论**：
1. from_any 残余的主体（62%）是**泛型擦除取值域**（类型变量返回对齐），归 A-1 取值端，A-4 不得触碰；
2. **A-4 可消灭段 = merge-box（661，26%）**——blocks.py:208 的合并槽装箱，其 from_any 甚至对具体类值也丢 vtable（JvmRef 包装，`is_instance_of`/`__class_name` 失真），既是可读性问题也是语义损伤；
3. 143-族聚簇（11 个测试同为 143±7）：残余几乎全部住在**共享 JDK 闭包文件**（date_time_formatter 19、text_provider 14、chrono 族 13、abstract_map 4、proxy 3、各 iterator/spliterator 1），不在用户代码——A-4 的收益将体现在闭包文件层。

## 2. A-4 的主度量其实不是 from_any：接口载体调用点转换

`Into::<I<..>>::into(..).m()`（接口载体调用点手工转换，任务目标的 `it.hasNext()` 直调对立面）：

| 度量 | 44 build 合计 | 说明 |
|---|---|---|
| **`Into::<接口载体>::into(`** | **32931** | A-4 调用点直调的消灭对象（TestStreamBasic 单测 1879） |
| `Into::<Object>::into(` | 19529 | A-1 类型变量边界（不在 A-4 域） |

接口分布（TestStreamBasic）：List 198、Iterator 189、Consumer 149、Spliterator 106、Map 87、Map_Entry 86、TemporalField 79、Set 76、TemporalAccessor 73、Collection 62、CharSequence 61、Node 49、Stream 47、Sink 40、……

## 3. 接口类型位置擦除点清单（五个位置 → 七个擦除点）

| # | 位置 | 擦除点 | 文件 |
|---|---|---|---|
| 1 | 形参/返回（发射签名单一来源，gen_method_body + vtable 擦除名单共用） | `sig_type_string_is_iface(sp)` → 回退 `jvm_to_rust(描述符)` = Object | `codegen/sig_types.py` `emitted_method_sig_types` |
| 2 | 形参/返回（stub 声明的平行实现） | `_is_iface_type(sp)` → 同上 | `codegen/emitter/method_gen.py:199-215` |
| 3 | 调用点实参类型解析 | 接口短名 → None → 描述符 Object | `codegen/instr/invoke_sig.py:119-131` |
| 4 | 描述符基线（LVT 局部变量 fallback、字段 fallback、checkcast 目标） | `jvm_to_rust` interface 分支 → `_iface_full_path` → 'Object' | `codegen/type_map.py:68-70,137-141` |
| 5 | 泛型签名解析（字段位/LVTT/嵌套实参位） | `_parse_one_type` L 分支 interface → 'Object'（接口作为泛型实参时同擦） | `codegen/sig_parse.py` |
| 6 | checkcast 目标合法性 | `_downcast_target_valid` 拒绝接口名 → **checkcast 到接口被静默丢弃**（跨接口强转 CCE 空档，A-5 遗留已登记） | `codegen/instr/invoke_sig.py:204-216`、`sim/control.py:31` |
| 7 | 接口载体调用点 | `Into::<I<Object>>::into(obj).m()` 每调用一包 | `codegen/instr/invoke_virtual.py:197-232` |

## 4. 批次计划（按以上证据排定）

- **批次 1（宏，纯增量）**：`java_class!` 对每个 `implements`（`impl Iface<Args> for Class<Args>` 条目）生成 `impl<..> From<Class<Args>> for Iface<Args>`（载体包装：`Iface::from(Object::from(v))`，保持对象身份）；checkcast 到接口经既有 `try_cast` 的 `is_instance_of` 路径放行（CCE 语义补全）。
- **批次 2（合并槽值侧）**：blocks.py:208 的 from_any → `_coerce_to_object`（具体类 `Object::from` 保 vtable、类型变量 `Into::<Object>`、其余 from_any）——merge-box 661 段清零且语义变好。
- **批次 3+（类型位置载体化）**：emitted_method_sig_types / method_gen / invoke_sig / jvm_to_rust 五点联动，接口名在类型位置发射 `I<Object>` 载体形态；形参位 → 返回位 → 局部/字段位分批，每批后跑量化与回归。

## 5. 基线仪表（44 build，供前后对照）

- `downcast_ref` = 0、`downcast` = 0（全部 build）——保持项。
- `type_surgery_sites` = 62（静态口径）。
- from_any / Into 载体计数：见 §1/§2。

---

## 6. 落地结果（2026-09-22，批次 1'+2 已提交 `2e9e328`）

### 批次与改动

- **批次 2（§1 merge-box 段清零）**：`method/blocks.py` `unify_pair` 的「具体类型臂
  并入擦除 Object 合并槽」分支改经 `_coerce_to_object`（与既有无公共父类分支同一
  约定，clone=False）——具体类 `Object::from` 保 vtable / 类型变量 `Into::<Object>` /
  未知形态 from_any 回落。阶段 0 发现该分支的 from_any 对具体类值也丢 vtable（JvmRef
  包装），既是可读性问题也是语义损伤，本批一并修复。
- **批次 1'（checkcast 到接口真实化）**：`sim/control.py` 对接口目标发射
  `interface_target` 形态 CastExpr（此前 cast_rust=='Object' 静默丢弃——跨接口强转 CCE
  空档）；`render_cast` 分派到 `Object::try_cast_iface`（object_ext.rs 新增：null 通过
  + `is_instance_of` 按运行时类静态超类型名单（含接口闭包）判定 + 幂等，失败
  Err(class_cast) 可捕获，S-1）；配套 `stack.py` 类型变量 hint 取回分支接受 CastExpr
  （`<D extends Iface> D v = (D) t` 形态：javac 按 D 的擦上界发接口 checkcast）。
- **宏侧核查**：协变 upcast `From<ClassName> for Iface` 已在 `interface_gen.py`
  （UPCASTS_SLOT，擦除实例化形态 `From<C<P>> for I<Object..>`）存在，批次 1 无需
  宏改动；`TryFrom<Object> for Iface` 与载体既有 `From<Object>` 触发 std blanket
  TryFrom 冲突（E0119），接口目标 CCE 语义由 `try_cast_iface` 承担（与 A-5 合成对象
  的 TryFrom 模式语义同构）。

### 量化（25 测验收集，前后对照）

| 指标 | 前 | 后 | 说明 |
|---|---|---|---|
| from_any 合计 | 879 | **617** | −262 = merge-box 段全灭；残余 100% 为 A-1 取值段（阶段 0 §1 let-align 族） |
| merge-box 段 | 262 | **0** | 分类器复核归零 |
| `Into::<Object>::into`（A-1 边界） | 1045/TSB | 1045/TSB | 不变（勿误伤域确认） |
| `downcast_ref` / `downcast` / `rc_new` | 0/0/1357 | 0/0/1357 | 保持 |
| `type_surgery_sites` | 62 | 62 | 持平（只降不增口径内，未新增手术位点） |
| `try_cast_iface` | 0 | 267/TSB | 批次 1' 新增（接口 checkcast 真实化） |
| `Into::<接口载体>::into` | 1879/TSB | 1879/TSB | **不变——批次 3+（类型位置载体化）的目标形态** |

### 回归与验证

- 25 测验收集 27/27 in-scope PASS（3 个子串带入失败 TestCollectionsUtil /
  TestOptional / TestOptionalChain 经 74f70dd 干净树对照确认**同型同点**：
  Unsafe.objectFieldOffset stub / Integer.valueOf stub / NPE 族——基线既有）。
- TestStringBuilder 金丝雀 PASS；streams 三测 PASS；lambda 族 + 接口重用例 PASS。
- 双种子生成树 diff 归零（TestStreamBasic）；bfs-audit 三计数不变（22/2734/0）。
- golden diff 全量归类两预期形态：`from_any→Object::from` 42 处（merge-box 段）+
  `try_cast_iface` 新增 267 处（无第三形态）。
- `python3 -m compileall -q codegen scripts` 通过；java_runtime 改动经全部 --clean
  构建验证（宏 crate 本批零改动）。

### 批次 3+（类型位置载体化）遗留计划——接管代理续做

目标：接口名在**类型位置**（形参/返回/局部/字段）发射 `I<Object..>` 载体形态，
调用点 `it.hasNext()` 直调替代 `Into::<I<Object>>::into(..)` 链（TSB 单测 1879 处，
44 build 口径 32931 处——A-4 的主杠杆，本会话未动）。按阶段 0 §3 清单的改造点：

1. **形参位**：`sig_types.emitted_method_sig_types` 的 `sig_type_string_is_iface` 回退
   与 `method_gen.py _is_iface_type` 平行实现合一，接口位发射 `I<Object..>`；
   `invoke_sig.py:119-131` 调用点实参解析同步（None 降级改载体形态）；
   `_coerce_arg` 对 `expected=载体` 的各分支（Fix-18 checkcast 目标合法性放行
   `_downcast_target_valid`、实参装箱 `Into::<I<Object>>`）。
2. **返回位**：同 `emitted_method_sig_types` 返回侧 + `_emit_call_result` /
   `_dispatch_bare_object` 的返回对齐；`areturn` 的 `From::from` 取回目标翻转为载体。
3. **局部/字段位**：`type_map.jvm_to_rust` 接口分支与 `sig_parse._parse_one_type`
   接口分支（当前回退 Object）→ 载体形态（影响 LVT、字段 fallback、嵌套实参位）；
   checkcast 到接口的栈类型记录翻转为载体（批次 1' 已预留——CastExpr 目标形态翻转
   即可，`try_cast::<I<Object>>` 走 is_instance_of 路径已验证可行）。
4. 每批后跑 25 测量集 + 27 例回归矩阵 + 双种子（`/tmp/a4/measure.sh`、
   `/tmp/a4/classify.py`、`/tmp/a4/seedcheck.sh` 可复用，随本报告提交于 /tmp/a4/
   的脚本需随批次重建）。
5. 风险登记：载体进签名后与 K-6 vtable 擦除名单、interface_gen `erased_declaration`
   的 token 全等匹配、sam_objects 闭包签名、继承成员擦除名单五处强耦合
   （emitted_method_sig_types 是它们的单一来源，翻转时五处同变）；建议先在
   TestIterator（闭包最小、Iterator 密度最高）上单测穿透再放量。

## 7. 前后测量明细（25 测验收集，baseline @74f70dd → final @批次 1+2）

```
test                                   from_any A1valcall mrgCtor mrgClone  inl  oth  Into<I>
test_array_list                               1         0       0        1    0    0       63
test_arrays_util                            144        89      14       24    6   11     1823
test_autobox_edge                             5         1       1        2    1    0       62
test_autoboxing                               0         0       0        0    0    0       14
test_bounded_generics                         0         0       0        0    0    0       15
test_casting                                  0         0       0        0    0    0       14
test_collection_factory                     143        89      14       23    6   11     1869
test_collections                              1         0       0        1    0    0       61
test_comparable                               0         0       0        0    0    0       31
test_comparator                               1         0       0        1    0    0       69
test_default_methods                          0         0       0        0    0    0       14
test_enum_methods                           143        89      14       23    6   11     1823
test_for_each                                 2         1       0        1    0    0       26
test_functional_interface                   143        89      14       23    6   11     1865
test_generic_method                           0         0       0        0    0    0       15
test_hash_map_ops                            41        40       0        1    0    0      133
test_inherited_method                         1         0       0        1    0    0       47
test_interface_static                       143        89      14       23    6   11     1823
test_interfaces                               0         0       0        0    0    0       15
test_iterator                                 1         0       0        1    0    0       18
test_lambda                                   1         0       0        1    0    0       62
test_lambda_capture                         143        89      14       23    6   11     1837
test_lambda_var                             143        89      14       23    6   11     1839
test_linked_hash                              7         1       1        4    1    0       88
test_linked_list                              4         2       0        2    0    0       14
test_list_of                                  5         1       1        2    1    0      155
test_map_iteration                            6         5       0        1    0    0      152
test_method_ref                               1         0       0        1    0    0       26
test_multi_catch                              0         0       0        0    0    0       14
test_optional                                 0         0       0        0    0    0       15
test_optional_full                          143        89      14       23    6   11     1839
test_pattern_match                          143        89      14       23    6   11     1823
test_priority_queue                           7         3       1        2    1    0      116
test_sorting                                  0         0       0        0    0    0       14
test_stream_advanced                        145        89      14       23    7   12     1916
test_stream_basic                           143        89      14       23    6   11     1879
test_stream_collectors                      150        95      14       24    6   11     1949
test_stream_more                            145        89      14       23    7   12     1909
test_string_builder                         143        89      14       23    6   11     1831
test_string_builder_ops                       0         0       0        0    0    0        0
test_string_edge                              6         4       0        2    0    0       68
test_string_regex                           143        89      14       23    6   11     1831
test_string_search                          143        89      14       23    6   11     1831
test_tree_map_set                           148        92      14       25    6   11     1909
test_varargs                                  0         0       0        0    0    0       14
TOTAL                                      2538      1580     242      419  108  189    32931

test                                base final
TestIterator                           1     0
TestLinkedList                         4     2
TestDefaultMethods                     0     0
TestInterfaces                         0     0
TestLambda                             1     0
TestLambdaCapture                    143   101
TestComparator                         1     0
TestFunctionalInterface              143   101
TestStreamBasic                      143   101
TestStreamAdvanced                   145   103
TestStreamCollectors                 150   107
TestStringBuilder                    143   101
TestArrayList                          1     0
TestCollections                        1     0
TestCasting                            0     0
TestVarargs                            0     0
TestAutoboxing                         0     0
TestBoundedGenerics                    0     0
TestGenericMethod                      0     0
TestInheritedMethod                    1     0
TestMultiCatch                         0     0
TestForEach                            2     1
TestComparable                         0     0
TestOptional                           0     0
TOTAL                                879   617
```

## 8. 批次 3–5 落地（类型位置载体化，接管代理续做）

四任接力（批次 3/4 = 第三任，批次 5 = 第四任），全部经 `jvm_type.CARRIER_TYPE_POSITIONS`
单一决策点宽化，五擦除点（emitted_method_sig_types / method_gen 存根 / invoke_sig 调用点 /
type_map.jvm_to_rust / sig_parse）自动同变：

| 批次 | 提交 | 铺设接口 | TSB 记分牌 Into::<I> |
|------|------|----------|----------------------|
| 基线 | 74f70dd | — | 1879 |
| 3 | 627f6dc | Iterator | 1744（TestIterator 18→8，Into::<Iterator> 清零） |
| 4 | 9d61405 | List/Collection/Set/Map/Map$Entry/Queue/Deque/ListIterator | 1211 |
| 5 | 29893be | 39 function 接口 + Comparator/Comparable/Collector | **1010** |

TSB 记分牌累计 **1879 → 1010（−46.3%）**。

### 批次 5 要点（第四任）

- **子接口载体 → 父接口载体 upcast**（interface_gen 新发射环）：`impl From<BinaryOperator<Object>>
  for BiFunction<Object, Object, Object>` 等——抽象类 / 接口接收者不发射 vtable impl（itable
  条目由具体实现类承担），但静态类型值的隐式上转（Writer→Appendable 形态）经 Object 边界
  保持对象身份。宏 interface.rs 的 `From<Object>`/`From<Self> for Object` 双向已备。
- **载体 instanceof 运行时化**（sim/control）：载体化后接口局部值（`Consumer<Object>` 变量）
  的 `instanceof` 不再按「互不为子类型」折叠编译期 false——实现者开放，运行时对象可同时
  实现目标接口。**实证**：不修则 streams 的 instanceof 快路径整体消失
  （RangeIntSpliterator.forEachRemaining 的 `action::accept` 装箱链 CCE：
  `Integer cannot be cast to Integer`——裸 i32 装箱的 Object 类名是 Integer 但 as_any 是 i32，
  Consumer 条目桥接 `Integer::from(Object)` 精确命中失败），TestStreamAdvanced 红线破；
  修复后快路径复活（`Object::from(action).is_instance_of("java/util/function/IntConsumer")`）。
- **SAM 条目差集桥接**（sam_objects）：default 条目体按「条目形参 → 声明形参」实际差集——
  同为载体直传，Object→载体才 `From<Object>` 包装；载体返回 Result 直接透传。
- **调用点实参载体例外**（invoke_sig）：类型变量代入（构造器 turbofish / 接收者实参映射）与
  描述符直签同名两类保持载体；手写边界方法（`_impl.rs` 伴生，签名先于载体化）接口位回退
  Object——`_handwritten_boundary_method` 存在性探测与 project_writer 同判据。

### 批次 5 绿门

- 42 例回归矩阵 32 PASS，10 失败全数定责基线：TestCollectionsUtil（Unsafe stub）/ 
  TestMethodRefKinds（E0308）/ TestOptional / TestOptionalChain（NPE 族）沿 §6 既有；
  TestLinkedHash / TestMapIteration / TestPriorityQueue / TestTreeMapSet 沿批次 4 复核；
  **TestStringEdge 与 TestHashMapOps 本任新增对照**：基线 5757b73 同型同点
  （Charset.<clinit> stub + internSame=false / null 数组 NPE 族），main 已由其他分支治愈
  （sun/nio/cs 手写层等），合并即消。golden（expected/*.txt 对比）零差异——32 PASS 输出
  逐字一致。
- 审计线：TSB `[bfs-audit] 22/2734/0` 不变；downcast_ref=0；双种子归零（TSB + TestIterator）；
  `type_surgery_sites` 62→69（+7 为批次 5 边界适配新增静态位点）；
  TSB from_any 101→105（+4 全为 `.apply(..)` 返回位 val-typevar——A-1 取值端结构性残余，
  Function/BiFunction 载体化后返回显式 Object，禁区不追）。

### 遗留登记（终态 None 的次序依据）

1. ~~**CharSequence/Appendable——被手写 decimal 层阻塞**~~（**批次 6 已解决**，见 §9）：
   原阻塞两面——(a) 调用点发射载体而手写签名收 Object；(b) 手写体 append_seq 传 Object
   而载体签名要 CharSequence——批次 6 按终态口径「手写层按载体签名书写」双侧同步落地。
2. **Spliterator 族**（批次 5 沿暂缓，批次 6 沿评估维持）：特化桥接
   forEachRemaining(Object)↔(LongConsumer) 名/型解析发散（既有登记）；批次 6 的字符族
   载体化与 iface_carrier_views 机制不触及该阻塞面（发散在方法名/形参型的桥接解析，
   非接口类型位置），维持暂缓不改判定。
3. **TSB 残余 1010 分布**：Temporal 族 ~211 / Spliterator 族 ~126 / CharSequence+Appendable
   ~100（上述阻塞）/ Node+Stream+Sink ~136（streams 内部接口族，Sink extends Consumer 的
   父链 upcast 机制已备）/ Pattern_CharPredicate 45 / 已铺设族残余位（Consumer 41、
   Comparable 30——Object 途径流入的结构残余，终态收敛）。

### 终态全量测量（45 测集，@29893be）

```
test                                   from_any  mrg  let  inl  Into<I> Into<Obj> cast_if
test_ArrayList                                0    0    0    0        5        92       2
test_ArraysUtil                             103    0  102    1      975      1082     130
test_AutoboxEdge                              2    0    1    1        2        74       0
test_Autoboxing                               0    0    0    0        2        51       0
test_BoundedGenerics                          0    0    0    0        3        43       0
test_Casting                                  0    0    0    0        2        49       0
test_CollectionFactory                      103    0  102    1      997      1093     133
test_Collections                              0    0    0    0        5        90       2
test_Comparable                               0    0    0    0       19        43      15
test_Comparator                               0    0    0    0       25        75      20
test_DefaultMethods                           0    0    0    0        2        41       0
test_EnumMethods                            103    0  102    1      975      1078     130
test_ForEach                                  1    0    1    0        2        44       0
test_FunctionalInterface                    103    0  102    1     1004      1080     130
test_GenericMethod                            0    0    0    0        3        43       0
test_HashMapOps                              40    0   40    0       22       170      17
test_InheritedMethod                          0    0    0    0        2        48       0
test_InterfaceStatic                        103    0  102    1      975      1082     130
test_Interfaces                               0    0    0    0        3        43       0
test_Iterator                                 0    0    0    0        2        46       0
test_Lambda                                   0    0    0    0       20        81      15
test_LambdaCapture                          103    0  102    1      975      1078     130
test_LambdaVar                              104    0  103    1      986      1082     130
test_LinkedHash                               2    0    1    1        5       127       2
test_LinkedList                               2    0    2    0        2        49       0
test_ListOf                                   2    0    1    1       19       135      15
test_MapIteration                             5    0    5    0       22       137      17
test_MethodRef                                0    0    0    0        2        48       0
test_MultiCatch                               0    0    0    0        2        41       0
test_Optional                                 0    0    0    0        2        48       0
test_OptionalFull                           103    0  102    1      987      1081     130
test_PatternMatch                           103    0  102    1      975      1078     130
test_PriorityQueue                            4    0    3    1       23       120      18
test_Sorting                                  0    0    0    0        2        41       0
test_StreamAdvanced                         107    0  105    2     1044      1106     146
test_StreamBasic                            105    0  104    1     1010      1099     131
test_StreamCollectors                       111    0  110    1     1009      1173     131
test_StreamMore                             108    0  106    2     1031      1103     144
test_StringBuilder                          103    0  102    1      981      1084     130
test_StringBuilderOps                         0    0    0    0        2        43       0
test_StringEdge                               4    0    4    0        8       143       2
test_StringRegex                            103    0  102    1      981      1082     130
test_StringSearch                           103    0  102    1      981      1078     130
test_TreeMapSet                             106    0  105    1     1010      1141     136
test_Varargs                                  0    0    0    0        2        45       0
TOTAL                                      1836    0 1813   23    17106     20610    2376
```

全测集口径前后对照：**Into::<I> 32931（§7 基线 @74f70dd）→ 17106（本表 @29893be，
−48.0%）**；from_any 2538 → 1836（−27.7%，残余 let-align 为主，A-1 取值端域）；
try_cast_iface 4827（批次 3 后快照）→ 2376（−50.8%，载体 checkcast 接管）。
记分牌曲线（TSB 口径）：1879（基线）→ 1744（批次 3）→ 1211（批次 4）→ 1010（批次 5），
累计 −46.3%；全测集口径 32931 →（批次 3 后 30199）→ 17106。

## 9. 批次 6 落地（CharSequence/Appendable 载体化 + 手写 decimal 层双侧适配）

接管背景：前任代理静默死亡（WIP 六文件已保护提交 `bca0f46`，未验证）；本任自实测复现
起步，WIP 验证为完整可用——九例定向回归面全绿，无需追加修复。基点 `cfb1783`，先并
main（`548c08b`：TypeIR 批次 2 六消费点 + 散点三件 + macros 异常 upcast `__erased_vtable`
机制 + docs `485fc19`），merge 干净无冲突，两机制（`__erased_vtable` 委托 /
iface_carrier_views 臂）在 wrapper.rs 共存验证。

### 批次 6 要点（第五任，前 WIP 作者+本任验证）

- **铺设**：`CARRIER_TYPE_POSITIONS` + `java/lang/CharSequence`、`java/lang/Appendable`
  （jvm_type.py 单一决策点）。
- **sig_parse 第五擦除点**：`_parse_one_type` 对启用接口的早返回载体发射——必须早于
  `_CLASSNAME_MAP` 查表（`java/lang/CharSequence → 'Object'` 的批次前旧擦除会把载体
  短路掉）；既有 240 行分支处理未铺设接口，无双重应用。
- **接口视图臂（新机制）**：`#[iface_carrier_views]` 属性（attrs.py 生成本类实现且
  闭包内的接口之擦除载体 Rust 类型清单）→ macros parse.rs 解析 → wrapper.rs 的
  `__view_into` 探针生成接口载体臂（`From<Object>` 包装填充，UFCS 显式防 `static from`
  工厂遮蔽）。动机：JLS 4.10.3 子类型关系含接口——数组协变（String[] → CharSequence[]）
  与 `try_checkcast::<载体>` 此前对接口位一律 false（祖先臂只覆盖父类链）。铺设门无关：
  未铺设接口的臂是死代码（槽位形态 Object 永不匹配），门宽化后自动激活。
- **interface_gen**：闭包内接口的载体 use 行先行（标记接口无 impl 关系但臂引用其类型，
  晚登记将 E0433）。
- **手写 decimal 层双侧适配（§8 遗留 1 的终态口径）**：double/float/floating 三个
  `appendTo` 签名 `Object → Appendable`（入+返回位），体内 `Into::<Appendable>::into`
  显式转换全部摘除（形参已是载体）；`append_seq` 实参改推断式 `Into::into`——
  CharSequence 实参位的发射形态取决于其是否入闭包，两形态下 `Object: Into<_>` 均成立。
  生成侧调用点（abstract_string_builder.rs:592/606）发射 `<Appendable as From<_>>::from`
  载体实参，两侧对齐。
- **语义冲突扫描（先例两处的同型检查）**：runtime/java_runtime/src 全量 grep——
  CharSequence/Appendable 仅 decimal 三文件，无其他手写/stub 签名需对齐。

### 批次 6 绿门（定向，全量待服务器）

- 九例建运对金标全绿：TestStringBuilder / TestStreamBasic / TestStreamCollectors /
  TestDouble（Ryū appendTo 直达路径）/ TestStringBuilderOps / TestStringCodePoints /
  TestPatternMatch（checkcast/instanceof 臂面）/ TestInterfaces / TestIterator。
- `[bfs-audit]`：TSB `22/2734/0` 不变，九例 unresolved 全 0；readability：TSB
  from_any=105 不变、downcast=downcast_ref=0；type_surgery_sites=27（TypeIR 批次 2 后值）。
- 双种子（PYTHONHASHSEED 1/2 生成树 diff）：TSB + TestIterator 双双一致。
- 记分牌（对照 §8 终态表）：TSB 1010→913、TSC 1009→912、TPM 975→878、
  TSBuiider 981→878（各 −97~−103，即 TSB 残余分布里 CharSequence+Appendable ~100 的
  兑现）；小闭包（Double/SBOps/SCP/Iter/Interfaces）Into<I> 持平（≈2，Object 途径结构
  残余非本批对象）；from_any/cast_if 逐例持平。
- Python 单测：tests.unit.test_erased_queries 10/10（发现 tests/unit 有 8 例既有
  `st.walk` 缺属性错误，main 上同现——非本批引入，非本域不越界修）。

### 批次 6 后 TSB 残余展望（913 的去向，供下一批排期）

Temporal 族 ~211 / Spliterator 族 ~126（沿暂缓）/ Node+Stream+Sink ~136 /
Pattern_CharPredicate 45 / 已铺设族残余位（Consumer 41、Comparable 30）。
