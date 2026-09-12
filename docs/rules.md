# 代码生成架构规则

**日期**：2026-09-12  
**适用范围**：`scripts/codegen/` 目录下的所有代码生成模块  
**来源**：架构审计与重新设计讨论（参见 `docs/plans/2026-09-12-architecture-redesign.md`）

---

## 规则一：字节码是唯一数据源

**规则**：凡是字节码操作数或注释中已有的信息，一律从字节码解析获取，Python 代码中不重复写一遍。

**适用场景**：

| 指令 | 字节码注释已有的信息 | 正确做法 |
|------|------------------|---------|
| `new` | 类的 binary name | `parse_class_ref(comment)` → `NewPendingExpr(name)` |
| `getstatic / putstatic` | 类名、字段名、字段类型 | `parse_field_ref(comment)` → `StaticFieldRef(cls, field, ty)` |
| `getfield / putfield` | 类名、字段名、字段类型 | `parse_field_ref(comment)` → `FieldAccess` |
| `invokevirtual` | 类名、方法名、参数描述符、返回描述符 | `parse_method_ref(comment)` → `MethodCall` |
| `invokestatic` | 同上 | 同上 |

**违规示例（禁止）**：
```python
# 禁止：在 Python 代码中写死类名
if 'System.out' in comment:
    sim.push('__stdout__', 'PrintStream')   # ❌

# 禁止：从类名推断行为
if cls == 'StringBuilder':                  # ❌
    ...
```

**合规示例**：
```python
# 正确：从注释通用解析
elif op == 'getstatic':
    cls, field, descriptor = parse_field_ref(comment)   # ✅
    ty = jvm_to_rust(descriptor)
    sim.push(StaticFieldRef(cls, field, RsNamed(ty)), RsNamed(ty))
```

---

## 规则二：最终态 Python 代码中不包含任何 JDK 类名

**规则**：`scripts/codegen/` 下的最终态代码中，不得出现任何 JDK 类名字符串常量（如 `'ArrayList'`、`'java/util/HashMap'`、`'PrintStream'` 等）。

**例外（仅在过渡期 Phase B-C 有效）**：`jdk_dispatch.py` 中的 `JDK_INTERFACE_GROUPS`，仅保存接口名（非实现类名），且该文件在 Phase D 完成后整个删除。

**原因**：JDK 有数千个类，任何枚举都不完整；类名写死在代码里是扩展的瓶颈。

---

## 规则三：用接口判断类行为，不枚举实现类

**规则**：需要判断某个类是否具备某种行为时，查询该类实现的接口，而不是枚举已知实现类的名单。

**错误模式**：
```python
JDK_CLASS_GROUPS = {
    'ArrayList': 'list',
    'LinkedList': 'list',   # 要手动加
    'Vector':    'list',    # 要手动加...
}
```

**正确模式**：
```python
JDK_INTERFACE_GROUPS = {
    'java/util/List': 'list',   # 接口数量有限且稳定
}

def get_dispatch_group(cls, registry):
    for iface in registry.get(cls).all_interfaces():    # 从已解析的 ClassInfo 获取
        if group := JDK_INTERFACE_GROUPS.get(iface):
            return group
```

**原因**：实现类数量无限，接口数量有限。新增 `LinkedList`/`TreeMap` 支持时零改动。

---

## 规则四：生成的 `.rs` 文件是 Java 元数据的唯一持久化载体

**规则**：从 `.class` 文件解析出的所有 Java 元信息（类层次、接口实现、字段签名、方法描述符、访问标志）必须通过 `#[java_*]` 属性嵌入到生成的 `.rs` 文件中。`build.rs` 只从这些属性中读取元信息，不维护任何手工表。

**属性格式**：
```rust
#[java_class(
    binary_name = "java/util/ArrayList",
    super_class = "java/util/AbstractList",
    interfaces  = "java/util/List,java/util/RandomAccess,...",
    access      = "public",
    source      = "ArrayList.java",
)]
pub struct ArrayList<E> { ... }

#[java_native(name = "arraycopy", descriptor = "(...)", access = "public static native")]
pub fn arraycopy(...) -> Result<()> { todo!(...) }
```

**`build.rs` 从属性自动推导**：
- 类层次图（`super_class` 链）
- 接口实现关系（`interfaces` 列表递归展开）
- native 方法追踪状态
- 接口实现完整性验证

**违规示例（禁止）**：
```rust
// build.rs 中禁止出现手工维护的类名
let jdk_classes = ["java/util/ArrayList", "java/lang/System", ...];  // ❌
```

---

## 规则五：IR 优先，`render.py` 是唯一产生字符串的地方

**规则**：`instr.py` 和 `stack.py` 中的所有代码生成操作必须产生 IR 节点（`RsExpr`/`RsStmt`），最终统一由 `render.py` 转换为字符串。除 `render.py` 外，任何模块不得直接拼接 Rust 代码字符串。

**违规示例（禁止）**：
```python
sim.push(f"({a}).wrapping_add({b})")          # ❌ f-string 拼接
sim.emit(f"let {v}: {ty} = {expr};")          # ❌ f-string 拼接
sim.push('__stdout__', 'PrintStream')         # ❌ 魔法字符串
```

**合规示例**：
```python
sim.push(BinOp('wrapping_add', a, b), I32)    # ✅ IR 节点
sim.emit(LetStmt(v, ty, mutable=False, value=expr))  # ✅ IR 节点
sim.push(StaticFieldRef(cls, field, ty), ty)  # ✅ IR 节点
```

---

## 规则六：类型信息在 IR 层持有，不用字符串标注

**规则**：操作数栈存储 `(RsExpr, RsType)` 元组，类型必须是 `RsType` 节点（`RsPrimitive`、`RsNamed`、`RsGeneric` 等），不得使用裸字符串（`'i32'`、`'ArrayList<String>'`）作为类型标注。

**泛型占位**：集合类型初始化时使用 `RsInfer`（对应 Rust 的 `_`），在第一次方法调用时从参数类型确定实际元素类型，不做事后正则修补。

**违规示例（禁止）**：
```python
sim.push(expr, 'ArrayList<String>')           # ❌ 字符串类型
sim.push(expr, 'i32')                         # ❌ 字符串类型
```

**合规示例**：
```python
sim.push(expr, RsGeneric('ArrayList', [RsInfer()]))  # ✅ 初始占位
sim.push(expr, I32)                                  # ✅ 类型常量
```

---

## 规则七：不允许对生成代码做正则后处理

**规则**：代码生成的正确性必须在 IR 构建阶段保证。不得对已生成的字符串做正则修补来修正错误。

**禁止的后处理模式**：
- 用正则修改变量类型（`_fix_coll_types`）
- 用正则删除/添加 `mut`（`_remove_unnecessary_mut`）
- 用正则合并冗余变量（`_merge_aliases`）
- 用正则简化表达式（`_simplify_wrapping_add`）

**正确做法**：
- 变量的 `mutable` 属性由 IR mutation 分析（扫描 `AssignStmt` 目标）在渲染前正确设置
- 类型从 IR 节点的类型字段中读取，永远正确
- 冗余变量在 `store_local` 阶段直接使用目标名，不产生中间别名

---

## 规则八：`jdk_dispatch.py` 是过渡产物，Phase D 后删除

**规则**：`jdk_dispatch.py` 仅在 Phase B-C 期间（JDK 字节码尚未完整翻译时）有效。Phase D（完整 JDK 字节码翻译）完成后，该文件整个删除，`invokevirtual` 对所有类（JDK 或用户类）统一处理。

**Phase D 后的 `_gen_invokevirtual` 最终形态**：
```python
def _gen_invokevirtual(sim, comment, class_name):
    cls, mname, param_types, ret_type = parse_method_ref(comment)
    args = [sim.pop() for _ in range(len(param_types))][::-1]
    obj_expr, obj_ty = sim.pop()
    # 所有类统一处理，无任何 JDK 类名判断，无分派表查询
    rust_name = to_snake(mname)
    call = MethodCall(obj_expr, rust_name, [e for e, _ in args], with_try=True)
    ...
```

**判断标准**：如果某段代码在 Phase D 完成后仍然必要，它才是真正的最终态架构。如果 Phase D 完成后可以删除，它就是过渡代码，需要标注清楚生命周期。

---

## 规则速查表

| 规则 | 核心要求 | 反面检查项 |
|------|---------|-----------|
| 规则一 | 从字节码注释解析，不重复写 | Python 代码中出现 JDK 类名 |
| 规则二 | 最终态无 JDK 类名常量 | `'ArrayList'`、`'PrintStream'` 等字符串 |
| 规则三 | 接口判断行为，不枚举实现类 | 列举 ArrayList/LinkedList/... 的表 |
| 规则四 | `.rs` 文件嵌入完整 Java 属性 | `build.rs` 中有手工类名表 |
| 规则五 | IR 节点，`render.py` 唯一出口 | `instr.py` 中出现 f-string |
| 规则六 | `RsType` 节点，不用字符串类型 | `push(expr, 'i32')` |
| 规则七 | IR 层保证正确性，不做正则修补 | `import re` 出现在 `method.py` 中 |
| 规则八 | `jdk_dispatch.py` 是过渡产物 | Phase D 后该文件仍存在 |
