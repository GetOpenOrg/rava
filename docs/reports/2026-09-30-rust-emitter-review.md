# Rust 生成器代码审核：Python 移植期差异的落点核对

> 2026-09-30。对象：`generator/crates/{instr,input}`（合并提交 cb4c473 时的状态）。
> 目的：核对 `docs/plans/2026-09-30-python-generator-port-diffs.md` 的 D1–D3 在 Rust 侧的实际代码形态、
> 移植落点与行为前提，修正该清单初稿中的错误。

## 一、方法与范围

- **逐行阅读**（不是 grep 命中行）：
  - `instr/src/sim/dynamic/lambda.rs`（全文 196 行）
  - `instr/src/sim/dynamic/lambda_args.rs`（全文 164 行）
  - `instr/src/sim/dynamic/lambda_body.rs`（`iface_call` 部分）
  - `instr/src/invoke/virtual_/vtable.rs`（`__virtual_view` 发射体 30–80 行）
  - `instr/src/naming.rs`（`mangle_if_overloaded` / `lambda_impl_rust_name`）
  - `input/src/build.rs`（`closure_classes` / `registry` / `normalize`）
  - `input/src/norm.rs`（`apply_fold` 的异常表处理）
  - `classfile/src/class.rs`（`ExceptionEntry`）
- **对照对象**：Python 修复后的 `codegen/instr/sim/dynamic.py`、`codegen/instr/invoke_virtual.py`、`codegen/method/codegen.py`；
  触发用例的真实字节码（`javap -c -p`）。
- **全仓扫描**：`format!("<{}>"` 形态的类型实参拼接（D3 同类问题）、`root_virtual` 的使用点、Object 锚点常量。
- **运行**：`cargo test -p instr` 通过，共 15 个（10 + 3 + 2 + 0）。
- **未覆盖**：
  - 本地没有 golden 转储（`build/golden/`），golden 对照测试按设计跳过，不构成逐字节一致的证据。
  - 未审 P4c（method）与 P5（emit），这两部分尚未合入。

## 二、结论摘要

| 项 | 初稿判断 | 核对结论 |
|---|---|---|
| D1 改动 1（根类改写） | 落点 lambda.rs 约 143 行 | ✅ 正确。前提成立：Root 域类不进 Rust Registry（`input/src/build.rs:148`），改写后 `ci = None`，与 Python 同路径 |
| D1 改动 2（实例判定通用式） | 落点约 178 行 | ✅ 正确。Rust 现为 Python 修复前的两个特例式，原样照抄 |
| D1 改动 3（`_impl_ci` 判空） | 「Rust 侧 Option，无须对应」 | ❌ **错误**。`lambda_args.rs:60-63` 在 `ci == None` 时返回 `unported(...)` 错误——D1 改动 1 之后这条路径必然走到，必须改为「不适用、继续后续分支」 |
| D2（死处理器剔除） | 落点「P4c method crate 方法体入口」 | ⚠️ **落点修正**。已合入的 `input/src/build.rs` `normalize()` 同时持有 Registry 与规范化后的 `exception_table`，是更合适、且现在即可实现的落点 |
| D3（turbofish） | 落点 vtable.rs 约 56 行 | ✅ 正确，且是全仓唯一一处。另一处同形拼接 `lambda_body.rs:28` 用在 `Into::<X<..>>` 类型位置，合法，不需要改 |

## 三、逐项核对

### D1 — lambda 实现句柄为接口重声明的根类方法

**现状（Rust = Python 修复前）**

- `lambda.rs:143`：`lambda_impl_rust_name(&env.ctx, impl_cls, ..)` 直接使用 `impl_cls`，没有改写。
  `impl_cls` 来自引导方法的 `MethodHandle.member`（`lambda.rs:64-78`）。
- `lambda.rs:176-184`：实例判定为

  ```
  n + 1 == sam_params.len() || (!cap_names.is_empty() && cap_names.len() == n + 1)
  ```

  用 `ProcessHandle::equals` 实测：1 个捕获（child）、1 个 SAM 实参、1 个形参（Object）。两式都不成立，`is_instance = false`。
  结果与 Python 修复前相同：`call_args` 不取 `&recv`，最终生成 `Object::equals(Object::from(..), _la0)`（E0308）。
- `lambda_args.rs:60-63`：SAM 实参是擦除引用且 `lam.ci == None` 时返回 `unported`，注释写明「Python 在此对 None 取 is_interface（AttributeError）」。
  这是移植时按 Python 当时的行为（崩溃）显式标注的未移植分支。

**行为前提核对**

- Root 域类不进 Registry：`closure_classes` 跳过 `Domain::Root`（`input/src/build.rs:148`），因此改写成 `java/lang/Object` 后 `env.ctx.reg().get(..)` 为 None。
  后续依次成立：
  - `impl_shape` 返回 `(false, ..)`（`lambda.rs:120-122`）；
  - 根类手写分支按通用式判定实例；
  - `call_args` 生成 `&Object::from(Clone::clone(&cap0))`（`lambda_args.rs:146-150`）。

  这与 Python 修复后的产物同形。
- 名字：`mangle_if_overloaded` 对 Registry 外的类返回原名（`naming.rs:56-58`），`equals` 不改名，与 Python 一致。
- 根类虚方法集：`env.ctx.facts.root_virtual`（`ctx.rs:50-51`）即 Python `_root_virtual_methods()` 的等价物，可以直接用；
  Object 的 binary name 用 `ty::consts::OBJECT`（`ty/src/consts.rs:17`）。

**移植要求（三处）**

1. `lambda.rs` 在 `split_impl` 之后、`lambda_impl_rust_name` 之前改写实现类：
   - 条件：`reg().get(impl_cls)` 是接口，且 `(impl_mname, 参数描述符)` ∈ `root_virtual`；
   - 动作：`impl_cls = ty::consts::OBJECT`。
   - 注意 `LambdaRef` 日志要记改写后的类，与 Python 账本一致。
2. `lambda.rs:180` 的实例判定改为 `lam.cap_names.len() + lam.sam_params.len() == n + 1`。
3. `lambda_args.rs:60-63` 在 `ci == None` 时不返回 `unported`，而是跳过「类型变量形参」判定，继续后续的装箱分支（等价于 Python 的 `_impl_ci is not None and ...`）。

**P5 附带要求**：Rust 侧 G-10 账本目前只记日志（`log.rs:51` `Effect::LambdaRef`），断言由 emit 实现。
断言必须与 Python `_LambdaNameLedger.check` 一样跳过「本轮未生成的类」，否则登记到 Object 的引用会误报。

### D2 — catch 类型不在闭包内的异常表条目

**字节码核实**：`jdk.internal.reflect.MethodAccessorGenerator$1.run()` 的异常表是两条共用处理器 31 的条目：

```
0 30 31 Class java/lang/InstantiationException
0 30 31 Class java/lang/IllegalAccessException
```

多类型 catch 中，只要有一个类型在闭包内，处理器就存活。

**现状**

- 闭包分析的 `dead_handlers` 按**处理器**判定。`input/src/norm.rs:282-285` 只在 `fold.dead_handlers.contains(&ent.handler)` 时删除条目。
  处理器存活时，缺失的那个类型条目仍保留，所以 D2 不是 folds 的重复。
- Rust 侧还没有按 catch 类型的过滤。

**落点修正**：`input/src/build.rs` `Build::normalize()`（266 行起）。

- 该函数遍历 Registry 全部方法，拿到折叠后的 `NormCode { exception_table, .. }`，而且只输出「改动过」的方法体。
- 在此过滤 `catch_type = Some(t)` 且 `reg.get(t).is_none()` 的条目，并把「有条目被剔除」计入改动标记（`folded || insns.len() != before` 之外再加一个条件）。
- 下游 TryCatchPlan 等消费方不需要任何改动；与 Python 在 `gen_method_body` 入口过滤的语义等价，两边用的都是发射用 Registry。

### D3 — 类 vtable 分派的擦除视图调用

- **现状**：`vtable.rs:56` 为 `format!("<{}>", ..)`，拼进 `{base}{erased_targs}::__virtual_view(..)`（57 行），表达式位置缺少 `::`，与 Python 修复前完全同形。
- **移植**：改为 `format!("::<{}>", ..)`。
- **同类扫描**：全仓 `format!("<{}>"` 只有两处。另一处 `lambda_body.rs:28` 用在 `Into::<{short}{targs}>::into(..)`，位于类型位置，正确。

## 四、对差异清单的修正

已同步修改 `docs/plans/2026-09-30-python-generator-port-diffs.md`：

- D1 的 Rust 落点补上 `lambda_args.rs:60-63` 的 `unported` 分支，删除「无须对应」的错误表述。
- D2 的 Rust 落点由「P4c method crate」改为 `input/src/build.rs` `normalize()`。

## 五、建议

1. **golden 覆盖**：现有 instr golden 的 3 例不含 D1 / D3 的触发形态，golden 全等不能证明已同步。建议把以下两例加入 `scripts/golden/dump_instr.py` 的转储集：
   - TestProcessHandleInfo（`X::equals` 方法引用）；
   - CheckOutputDeviceIsATerminal（泛型类接收者的 `__virtual_view`）。
2. **`unported` 清点**：D1 改动 3 表明，Rust 侧按 Python 当时的崩溃行为标注了 `unported` 分支。Python 之后修掉的崩溃，对应的 `unported` 需要随差异清单一起更新。
   建议移植线用 `grep -rn "unported(" generator/crates` 对照差异清单做一次清点。
