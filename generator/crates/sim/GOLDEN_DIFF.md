# sim golden 差异登记

golden：`python3 scripts/golden/dump_sim.py <Test.java> --jdk 21 --clean` → `build/golden/sim/<Test>.jsonl`
（+ `<Test>.names.json`），`cargo test -p sim --test golden -- --nocapture` 逐条重放比对。

每条记录以 Python 调用前的状态（栈带 dup 身份分组、局部变量、计数器、深度 / 绑定账、类型变量上界）
重建 `StackSim::from_state`（`init` 走 `StackSim::new`），回放同一调用，逐项比对：返回值（表达式文本 + 类型文本）、
新增语句（渲染文本 + slot + bind_off + value_ty）、后状态（栈文本与类型、非平凡条目的 dup 分组、局部变量
名 / 类型 / is_new、计数器、合成槽类别、深度账、绑定点、下溢标志）。外部钩子按日志中的（名, 实参）回答，
未命中即记为不等。

## 现状（2026-09-30）

| 记录（去重记录 / 调用次数） | TestHashMapOps | TestStreamBasic |
|---|---|---|
| init | 639/639 · 657/657 | 1266/1266 · 1307/1307 |
| pop | 14846/14846 · 14936/14936 | 20970/20970 · 21121/21121 |
| pop_for_store | 1704/1704 · 1720/1720 | 2135/2135 · 2151/2151 |
| store_local | 1704/1704 · 1720/1720 | 2135/2135 · 2151/2151 |
| load_local | 5562/5562 · 5630/5630 | 8517/8517 · 8614/8614 |
| fresh_let | 67/67 · 67/67 | 93/93 · 93/93 |

不等：0。钩子调用（两例合计）：is_interface 25、carrier 22（均返回 None）、is_subtype 17、box_object 1；
infer_type_args 0（两例无菱形构造存入泛型声明局部，该路径只有代码审阅覆盖）。

## §1 结构性偏离（不影响 golden 比对）

1. **类型模型**：Python `RsNamed(name 字符串)` / `RsPrimitive` → `ty::RsType`。以基本类型名构造的
   `RsNamed('i8')`（LVT 声明类型）在 Rust 侧同为 `Prim`，判定口径（`isinstance(RsNamed)`）随之按「非标量」处理；
   两例中此类局部只经文本比较路径，结果一致。
2. **不可达分支**：`_store_refine_hint` 的 `Vec<Object>` ↔ `Vec<E>` 重定向（RsType 无 Vec 形态，Python 注释亦称
   语料零触发）；`_store_emit` 的 `isinstance(ty, RsGeneric)` 省略 let 类型（golden 中无 RsGeneric 类型节点）。
3. **Raw 文本 → 结构节点**：int 族收窄（`(v) as i32` / `((v) as i32 != 0i32)` / `(((v) as i32) as T)`）、
   `Default::default()`、`From::from`、`Into::<B>::into(Into::<Object>::into(..))`、`Object::from(Clone::clone(this))`、
   `<T as ::std::convert::From<X>>::from(..)`、下溢占位 `(panic!("stack underflow") as i32)` 均以 `ir` 节点构造，
   渲染逐字节一致。对应判定（`is_default` / `is_trivial` / `contains_default`）同时识别结构节点与同文本 Raw
   （Python 只见 Raw）；`contains_default` 对内联块只看简单语句。
4. **菱形实参改写**：Python 按渲染前缀 `Head::<_, ..>::` 替换文本；Rust 沿渲染最左路径找到首个调用路径的
   首段替换 generics，未命中时同样退为强制 let 类型注解。
5. **dup 身份**：Python 以对象 `is` 判定，Rust 以 `ValueId`。store 后 Python 把同身份副本替换为各自新建的
   `Var`（身份分离），Rust 保留共享 id——替换结果是平凡的 `Var`，`pop` 不物化平凡值，行为一致；比对只看非平凡
   条目的分组。
6. **钩子实参**：Python 以短名 / 类型串调用，Rust 传擦除后的 `RsType`（`is_subtype` / `is_interface`）或完整
   `RsType`（`carrier_type` / `box_object`），由实现方映射到注册表。

## §2 回放装置的约定

- `names.json` 只含注册表 binary 名；注册表外的 binary（如 `NewPendingExpr` 的根类）按 Python `short_cls`
  缺省规则（末段、`$` → `_`）取短名。
- 记录 `cfg.class_type_params` 已按名排序（集合语义）；`init` 用实参中的声明序构造 `this` 的类型实参。
- Python 前状态的语句缓冲不重建，只比对本次调用新增语句。
