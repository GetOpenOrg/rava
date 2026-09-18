# P0 Python 文件拆分方案

> 创建时间：2026-09-18  
> 状态：草案

---

## 1. 背景与优先级

三个 Python 文件在当前规模下已成为瓶颈：每次修改都需要读取整个文件，Context 占用极高，且逻辑边界模糊导致改动范围难以控制。

| 文件 | 当前行数 | 主要问题 |
|------|---------|---------|
| `codegen/emitter/class_writer.py` | 1149 | `_gen_class_rs` 单函数 935 行，内含大量内联局部函数 |
| `codegen/instr/invoke.py` | 950 | `_gen_invokevirtual` 单函数 292 行，签名查找辅助函数与生成函数混杂 |
| `codegen/instr/sim.py` | 841 | `sim_instr` 单函数 841 行 elif 链，无任何子函数可直接移走 |

**原则**：按 block.rs Phase 1 节奏——先移出支撑代码，主函数最后拆分；零逻辑改动，只挪代码。

---

## 2. class_writer.py 拆分方案

### 2.1 现状分析

`_gen_class_rs`（行 218–1152，935 行）内含 7 个局部函数（无法直接提取）和 10 个内联逻辑阶段（无函数边界）：

**局部函数（需先提升为模块级）**：
- `_strip_generic`（L244）— 去掉 JVM 类名中的 `<…>` 泛型
- `_add_precise_import`（L321）— 生成精确 use 语句
- `_pkg_to_use`（L368）— pkg_slash → use 路径
- `_extract_type_names`（L556）— 从 Rust 类型字符串提取类型名
- `_validate_field_type`（L573）— 递归检查 rust_ty 可用性
- `_resolve_field_rust`（L606）— 字段 Rust 类型（generic_sig 优先）
- `_resolve_anc_field_rust`（L617）— 祖先字段 Rust 类型（泛型参数映射）

**顶层支撑函数（可直接移走）**：
- `_bin_to_rust`（L33）— JVM 二进制名 → Rust 类型名
- `_find_virtual_in`（L38）— 沿祖先链查虚方法归属
- `_extract_clinit_consts`（L96）— 扫描 `<clinit>` 提取常量
- `_push_int_value`（L145）— push 指令 → 整数字面值
- `_extract_clinit_arrays`（L154）— 扫描 `<clinit>` 提取常量数组

### 2.2 目标结构

```
codegen/emitter/
├── class_writer.py          ← 只保留 _gen_class_rs 主干（目标 ~250 行）
├── vtable_util.py     (新)  ← vtable 分析工具
├── clinit_extract.py  (新)  ← <clinit> 常量提取
├── import_gen.py      (新)  ← 引用收集 + cross_imports 生成
├── field_gen.py       (新)  ← 字段类型解析 + superclass_fields + static getter
└── method_inherit.py  (新)  ← 接口 default + 超类虚方法继承
```

### 2.3 各子文件内容

**`vtable_util.py`（~70 行）**
```python
def _bin_to_rust(name: str) -> str: ...
def _find_virtual_in(method_name, descriptor, ci, registry) -> str | None: ...
```

**`clinit_extract.py`（~130 行）**
```python
_CONST_PUSH_OPCODES = {...}
def _push_int_value(ins) -> int | None: ...
def _extract_clinit_consts(ci) -> dict[str, ...]: ...
def _extract_clinit_arrays(ci) -> dict[str, ...]: ...
```

**`import_gen.py`（~200 行）**
提取 Step1（引用收集）+ Step2（cross_imports 生成）为顶层函数：
```python
# 从局部提升为模块级
def _strip_generic(name: str) -> str: ...
def _add_precise_import(jvm_name, ...) -> str: ...
def _pkg_to_use(pkg_slash: str) -> str: ...

# Step1+Step2 提取
def collect_cross_imports(
    ci, registry, jdk_crate_pkg_paths, generated_classes,
    call_chain, skipped_classes, conflict_map,
    user_sibling_imports, user_crate_prefix
) -> list[str]: ...
```

**`field_gen.py`（~200 行）**
```python
# 从局部提升为模块级
def _extract_type_names(rust_ty: str) -> list[str]: ...
def _validate_field_type(rust_ty, available_types) -> bool: ...
def _resolve_field_rust(field, class_type_params) -> str: ...
def _resolve_anc_field_rust(field, anc_type_args, anc_type_params) -> str: ...

# 阶段提取
def build_superclass_fields(ci, registry, class_type_params) -> list[tuple[str, str]]: ...
def gen_static_field_getters(ci, class_type_params, registry, ...) -> tuple[list[str], list[str]]: ...
    # 返回 (module_statics, method_blocks)
```

**`method_inherit.py`（~130 行）**
```python
def gen_interface_default_methods(ci, registry, ...) -> list[str]: ...
def gen_superclass_virtual_methods(ci, registry, ...) -> list[str]: ...
```

**`class_writer.py` 保留（目标 ~250 行）**：
`_gen_class_rs` 精简为 10 个调用步骤，不含任何内联逻辑块。

### 2.4 执行顺序

1. `clinit_extract.py` — 纯函数，无依赖，0 风险
2. `vtable_util.py` — `_bin_to_rust` + `_find_virtual_in` 同文件迁移
3. `field_gen.py` — 局部函数提升为模块级，再移文件（需通过参数传递 registry）
4. `import_gen.py` — Step1+Step2 提取，参数多，需仔细列举
5. `method_inherit.py` — 两个大代码块提取为独立函数后移文件
6. **最后**：`class_writer.py` `_gen_class_rs` 替换为调用链，精简至 ~250 行

---

## 3. invoke.py 拆分方案

### 3.1 现状分析

| 函数 | 行号 | 职责 |
|------|------|------|
| `_lookup_method_sig_params` | 29–84 | 查方法 generic_signature 参数类型列表 |
| `_registry_iface_shorts` | 90–104 | registry 中所有接口的 Rust 短名 |
| `_concrete_class_shorts` | 110–124 | registry 中非接口类的 Rust 短名 |
| `_downcast_target_valid` | 127–137 | 验证 downcast 目标类型合法性 |
| `_lookup_method_sig_ret` | 140–218 | 查方法 generic_signature 真实返回类型 |
| `_coerce_arg` | 221–281 | 参数强制转换（装箱、上转、downcast、扩宽） |
| `_gen_string_concat` | 284–332 | invokedynamic 字符串拼接 → `format!` |
| `_split_type_args` | 335–351 | 按顶层逗号切泛型实参串 |
| `_substitute_tvars` | 354–362 | 类型变量 → 实参替换 |
| `_resolve_ctor_turbofish_args` | 365–410 | 推导泛型构造器 turbofish 实参 |
| `_gen_invokespecial` | 413–555 | invokespecial：构造器 + super 调用 |
| `_gen_invokestatic` | 558–656 | invokestatic：静态方法调用 |
| `_gen_invokevirtual` | 659–950 | invokevirtual/invokeinterface：虚分发（292 行） |

**依赖关系**：
```
_gen_invokespecial ──► _lookup_method_sig_params, _coerce_arg, _resolve_ctor_turbofish_args
_gen_invokestatic  ──► _lookup_method_sig_params, _lookup_method_sig_ret, _coerce_arg
_gen_invokevirtual ──► _lookup_method_sig_params, _lookup_method_sig_ret, _coerce_arg, _downcast_target_valid

_lookup_method_sig_params ──► _registry_iface_shorts
_lookup_method_sig_ret    ──► _split_type_args, _substitute_tvars
_coerce_arg               ──► _downcast_target_valid, _into_super_chain (coerce.py)
```

### 3.2 目标结构

```
codegen/instr/
├── invoke.py               ← 保留 3 个入口 + _gen_string_concat + _resolve_ctor_turbofish_args
├── invoke_sig.py     (新)  ← 签名查找工具（纯函数，无 sim 操作）
├── invoke_virtual.py (新)  ← _gen_invokevirtual（292 行单函数）
└── coerce.py               ← _coerce_arg 合并进来（语义上属于参数强制转换层）
```

### 3.3 各子文件内容

**`invoke_sig.py`（~350 行，纯函数，可独立测试）**：
- `_registry_iface_shorts`
- `_concrete_class_shorts`
- `_downcast_target_valid`
- `_lookup_method_sig_params`
- `_lookup_method_sig_ret`
- `_split_type_args`
- `_substitute_tvars`

**`invoke_virtual.py`（~292 行）**：
- `_gen_invokevirtual`（从 invoke.py 整体迁移）

**`coerce.py` 扩展**：
- 将 `_coerce_arg` 合并（已大量依赖 coerce.py 内的函数，语义上同属一层）

**`invoke.py` 保留（目标 ~300 行）**：
- `_gen_string_concat`
- `_resolve_ctor_turbofish_args`
- `_gen_invokespecial`
- `_gen_invokestatic`
- 从子文件 import，对外暴露入口

### 3.4 执行顺序

1. `invoke_sig.py` — 全部纯函数，直接剪切，无逻辑改动
2. `_coerce_arg` → `coerce.py` — 单函数迁移
3. `invoke_virtual.py` — 整体迁移 `_gen_invokevirtual`

---

## 4. sim.py 拆分方案

### 4.1 现状分析

`sim_instr`（行 70–841，771 行）是一条巨型 elif 链，包含 14 个逻辑段。没有独立子函数可以直接移走。

**拆分策略**：先将 elif 段提取为独立函数，再按职责分组到子文件，`sim.py` 只保留顶层 dispatch。

| elif 段 | 行范围 | 目标文件 |
|---------|--------|---------|
| 整型常量/null/ldc | 75–130 | `sim/consts.py` |
| load/store/iinc | 131–179 | `sim/locals.py` |
| 整数算术 + long/float/double + 比较 + 类型转换 | 180–308 | `sim/arith.py` |
| 栈操作（dup/pop/swap） | 310–352 | `sim/stack.py` |
| 对象/字段（new/getfield/putfield/getstatic/putstatic） | 354–510 | `sim/fields.py` |
| 数组（newarray/aastore/aaload 等） | 512–606 | `sim/arrays.py` |
| 方法调用（委托到 invoke.py） | 608–612 | `sim/methods.py` |
| 返回 | 614–659 | `sim/returns.py` |
| 类型检查/控制流（if/goto/checkcast/instanceof/tableswitch） | 661–726 | `sim/control.py` |
| invokedynamic/monitor/athrow/nop | 728–841 | `sim/dynamic.py` |

### 4.2 目标结构

```
codegen/instr/
├── sim.py               ← 只保留 sim_instr 顶层 dispatch（目标 ~50 行）
└── sim/
    ├── __init__.py
    ├── consts.py        ← 整型常量、null、ldc
    ├── locals.py        ← load/store/iinc
    ├── arith.py         ← 所有算术 + 比较 + 类型转换
    ├── stack.py         ← dup/pop/swap
    ├── fields.py        ← 对象/字段指令 + _restore_field_declared_type
    ├── arrays.py        ← 所有数组指令
    ├── methods.py       ← 方法调用 dispatch（委托 invoke.py）
    ├── returns.py       ← return 系列
    ├── control.py       ← if/goto/checkcast/instanceof/switch
    └── dynamic.py       ← invokedynamic/monitor/athrow
```

**子函数统一接口**：
```python
def sim_xxx(ins, sim, class_name, registry) -> bool:
    """处理匹配的指令，返回 True 表示已处理，False 表示跳过。"""
```

`sim_instr` 精简为：
```python
def sim_instr(ins, sim, class_name, registry):
    handlers = [sim_consts, sim_locals, sim_arith, sim_stack,
                sim_fields, sim_arrays, sim_methods, sim_returns,
                sim_control, sim_dynamic]
    for handler in handlers:
        if handler(ins, sim, class_name, registry):
            return
    raise NotImplementedError(f"未处理的指令: {ins['op']}")
```

### 4.3 执行顺序

1. `sim/dynamic.py` — invokedynamic 约 90 行，逻辑最内聚，无外部状态依赖
2. `sim/fields.py` — 含 `_restore_field_declared_type` helper，可携带整体迁移（~148 行）
3. `sim/arith.py` — 最大段，但全是纯算术模板，无外部查询
4. `sim/control.py` — if/goto 控制流，~65 行
5. 其余段依次提取

---

## 5. 总体执行优先级

| 优先级 | 文件 | 拆分步骤 | 收益 |
|--------|------|---------|------|
| P0.1 | `class_writer.py` | 提取 `clinit_extract.py`（无依赖叶子） | 立即减小主文件读取量 |
| P0.1 | `invoke.py` | 提取 `invoke_sig.py`（纯函数，无 sim 操作） | 签名查找逻辑独立可测 |
| P0.2 | `sim.py` | 提取 `sim/dynamic.py` + `sim/fields.py` | 最内聚段，最低风险 |
| P0.3 | `class_writer.py` | 依次提取 `vtable_util`, `field_gen`, `import_gen`, `method_inherit` | 主函数从 935→250 行 |
| P0.3 | `invoke.py` | 提取 `invoke_virtual.py` + 合并 `_coerce_arg` | 虚分发逻辑独立 |
| P0.4 | `sim.py` | 提取剩余 8 个段，主函数精简为 dispatch | 841 行 → 50 行 dispatch |

**验证方式**：每步完成后运行 `python3 scripts/run_tests.py` 确认 e2e 测试不退步（只是挪代码，无逻辑改动，测试通过率应保持不变）。
