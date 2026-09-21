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
