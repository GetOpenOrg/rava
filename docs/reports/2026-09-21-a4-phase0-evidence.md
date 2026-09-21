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
