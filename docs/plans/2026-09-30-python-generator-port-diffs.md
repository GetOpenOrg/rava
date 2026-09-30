# Python 生成器移植期差异清单（待并入 Rust 生成器）

> 状态（2026-09-30）：D1 / D3 ✅ 已并入 instr（cacea617，golden 增补 TestProcessHandleInfo / CheckOutputDeviceIsATerminal，逐字节一致；当前闭包下 D3 形态无 golden 样本，待 P5b 后以 e2e 验证）；D2 ✅ 闭包按条目输出 `dead_catches`、input 消费（b7f76452，folds v2）。
>
> 2026-09-30（落点经代码审核修正，见 `docs/reports/2026-09-30-rust-emitter-review.md`）。Rust 生成器计划（`2026-09-30-rust-emitter.md` §三）规定：P5 切换前以 Python 产物为对照真源，
> 移植期的 Python 改动须登记，由 Rust 侧同步。本文列出 **Rust 各 crate 移植基线之后** Python 生成器
> （`codegen/`）的全部语义改动，供移植线逐条并入。

## 一、基线判定

各 crate 首个移植提交的祖先关系（`git merge-base --is-ancestor`）：

| crate | 首个移植提交 | 基线已含 |
|---|---|---|
| input / ty / ir / sim / cfg / instr | 41df307 / fd78886 / c9b87b8 / 7d9f1fd / ea0d4c1 / 0f52b6e | c448bc9、8cb36bd、420a929、216f74f、b893e9a（此前的 Python 改动均已在 golden 真源内） |

**基线之后**的 Python 生成器提交共 3 个：bca2140、eb038d0、b8c182a。下表逐条给出位置、语义、Rust 落点与验证。
（同期的 runtime/ 手写层改动不属于生成器移植范围，Rust 生成器经 overlay 直接复用，不列出。）

## 二、差异明细

### D1 — invokedynamic：实现句柄是接口重声明的根类方法（bca2140）

| 项 | 内容 |
|---|---|
| Python | `codegen/instr/sim/dynamic.py` `sim_dynamic`（invokedynamic 通用分支，解析 `impl:` 之后、`lambda_impl_rust_name` 之前）；同函数「实现类不在 registry」的实例判定 |
| 触发 | 方法引用 `handle::equals`（`ProcessHandle.of(pid).map(child::equals)`）：javac 把实现句柄记为 `REF_invokeInterface ProcessHandle.equals(Object)`。接口不发射根类公开方法（经根类 vtable 分派），G-10 账本断言「调用点引用 equals 但该类未输出定义」 |
| 改动 1 | 实现类是接口，且 `(方法名, 参数描述符)` ∈ 根类虚方法集（`_root_virtual_methods()`：equals / hashCode / toString）时，实现类改写为根类（`constants.OBJECT_CLASS`）——与 JVM 方法解析一致 |
| 改动 2 | 实现类不在 registry（手写根类）时的实例判定由两个特例改为通用式：**捕获数 + SAM 实参数 = 描述符形参数 + 1**（接收者占一位）。原式只覆盖「无形参方法」，`recv::equals`（1 捕获 + 1 SAM 实参 = 1 形参 + 1）漏判为静态，生成 `Object::equals(Object::from(..), _la0)`（E0308） |
| 改动 3 | 同函数 SAM 实参适配里「形参是声明类类型变量」分支访问 `_impl_ci.is_interface` 前判空（改写为根类后 `_impl_ci` 为 None，AttributeError） |
| Rust 落点 | ① `instr/src/sim/dynamic/lambda.rs` 143 行 `lambda_impl_rust_name(...)` 之前改写 `impl_cls`（条件用 `env.ctx.facts.root_virtual`，目标 `ty::consts::OBJECT`；`LambdaRef` 记改写后的类）；② 同文件 180 行实例判定换成 `lam.cap_names.len() + lam.sam_params.len() == n + 1`；③ `instr/src/sim/dynamic/lambda_args.rs` 60–63 行 `ci == None` 时现返回 `unported`（按 Python 修复前的 AttributeError 标注）——须改为跳过类型变量判定、继续后续分支。前提已核实：Root 域类不进 Rust Registry（`input/src/build.rs:148`），改写后 `ci = None`，与 Python 同路径（审核报告 `docs/reports/2026-09-30-rust-emitter-review.md` §三 D1） |
| 验证 | e2e TestProcessHandleInfo（本机转译 + cargo check 通过）；golden 需含一处 `X::equals` 方法引用（建议把 TestProcessHandleInfo 加入 instr golden 转储集） |

### D2 — 方法体：catch 类型不在闭包内的异常表条目按死处理器剔除（eb038d0）

| 项 | 内容 |
|---|---|
| Python | `codegen/method/codegen.py` `gen_method_body` 入口（签名计算之前） |
| 触发 | `jdk/internal/reflect/MethodAccessorGenerator$1.run` 的 `catch (InstantiationException \| IllegalAccessException)`：`NativeMethodAccessorImpl.invoke0` 实现后该类入闭包，但 `InstantiationException` 不在闭包 → catch 头引用未生成类型（E0425，SendAnUnknownMethodCall） |
| 改动 | 异常表条目的 `catch_type` 不在 registry 时剔除该条目（方法对象浅拷贝后替换 `exception_table`）；多类型 catch 逐类型剔除，剩余类型照常；catch-any（`catch_type is None`）保留。语义与闭包分析 folds v1 的 `dead_handlers` 相同——类不在闭包即运行期不可能被抛出 |
| Rust 落点 | **终态（2026-09-30 拍板）：闭包分析器按条目判定，发射层只消费。** closure crate 的 folds 新增按条目的死异常表条目（`dead_catches`：catch_type 不在闭包类集的条目；某处理器全部条目皆死时仍并入 `dead_handlers`），`input/src/norm.rs` `apply_fold` 按其删除条目。不在 `input` 的 `normalize()` 里按 Registry 自行过滤——那会成为「死异常条目」的第二判定源（审核报告 §三 D2 指出 `dead_handlers` 按处理器判定、不覆盖多类型 catch 部分缺失，此缺口由分析器按条目输出补上）。Python `gen_method_body` 入口的 registry 过滤随 P5 切换退役 |
| 验证 | e2e SendAnUnknownMethodCall（本机 cargo check 通过）；golden 对照需含 MethodAccessorGenerator$1（该例闭包内） |

### D3 — 类 vtable 分派的擦除视图调用补 turbofish（b8c182a）

| 项 | 内容 |
|---|---|
| Python | `codegen/instr/invoke_virtual.py` 类 vtable 分派发射体（`_view_recv` 构造处） |
| 触发 | 泛型类接收者经裸 Object 的类虚方法分派：`WeakPairMap.containsKeyPair` 里 `map.containsKey(..)`（map 为 `ConcurrentHashMap<K,V>`）生成 `ConcurrentHashMap<Object, Object>::__virtual_view(&..)`——表达式位置的类型实参必须写成 turbofish（语法错误，CheckOutputDeviceIsATerminal） |
| 改动 | 擦除实参串由 `<Object, ..>` 改为 `::<Object, ..>` |
| Rust 落点 | `generator/crates/instr/src/invoke/virtual_/vtable.rs` 56 行：`format!("<{}>", ...)` → `format!("::<{}>", ...)`（全仓唯一一处；`lambda_body.rs:28` 同形拼接在类型位置，合法） |
| 验证 | e2e CheckOutputDeviceIsATerminal（本机 cargo check 通过）；golden 需含泛型类接收者的 `__virtual_view` 调用点（现有 3 例若无，golden 不会暴露此差异） |

## 三、并入顺序建议

1. D3、D1：落在已合入的 instr crate，改动局部；并入后重跑 instr golden（转储前先把 Python 同步到含本三提交的版本，golden 自然全等）。
2. D2：闭包精度线实现（folds 按条目输出 + input 消费，含 Python `closure_folds.py` 读取端同步），与 D1 / D3 独立。
3. 此后 Python 生成器如再有改动，续记本文（按提交号追加 D4…），直至 P5 切换。

## 四、相关

- 触发这些改动的用户批量失败与修复记录：`docs/plans/2026-09-29-gap-closure.md` §五—§七。
- 基线之前、已含于移植真源的 Python 改动（无需处理）：c448bc9（jdk_resources 复跑覆写、acmp 形参、擦除名集位置标记等）、8cb36bd（`[vm_boundary]` 按方法划分）、420a929 / 216f74f（桥方法形参对齐）、b893e9a（不重发射只承载接口槽位的祖先桥）。
