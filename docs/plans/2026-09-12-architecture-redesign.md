# 架构重新设计计划

**日期**：2026-09-12  
**状态**：待实现  
**前置文档**：`2026-09-12-jdk-bytecode-translation.md`（原始计划，本文档替代之）  
**目标**：消除代码生成层的全部技术债，建立最终态架构：IR 优先 + 数据驱动分派 + 完整 JDK 翻译流水线。

---

## 目录

1. [现状差距分析](#1-现状差距分析)
2. [最终态架构](#2-最终态架构)
3. [阶段 A：IR 全量迁移](#3-阶段-a-ir-全量迁移)
4. [阶段 B：数据驱动 JDK 分派](#4-阶段-b-数据驱动-jdk-分派)
5. [阶段 C：属性格式升级](#5-阶段-c-属性格式升级)
6. [阶段 D：完整 JDK 字节码翻译](#6-阶段-d-完整-jdk-字节码翻译)
7. [阶段 E：native_impls 与构建阻断](#7-阶段-e-native_impls-与构建阻断)
8. [里程碑汇总](#8-里程碑汇总)

---

## 1. 现状差距分析

### 1.1 原始计划已完成的部分

| 原始任务 | 状态 | 备注 |
|---------|------|------|
| P0：`jdk_resolver.py` | ✅ 完成 | 能从 jmod 读取 JDK `.class` 文件 |
| P0：`classfile.py` 扩展（super_class, interfaces, ACC_NATIVE）| ✅ 完成 | ClassInfo 含完整元信息 |
| P1：JDK 类元数据文件生成 | ⚠️ 部分完成 | 格式是注释 `// @java_class`，非属性 `#[java_class]` |
| P2：`build.rs` 扫描注解、维护 `native_status.toml` | ⚠️ 部分完成 | 只报 warning，未阻断构建 |
| P3：`native_impls/` 手写实现目录 | ❌ 未实现 | |
| P4：删除 `instr.py` 中的 JDK 硬编码分派 | ❌ 未实现 | 被代码审计揭示为核心问题 |

### 1.2 原始计划未覆盖的问题（代码审计新发现）

原始计划完全聚焦 JDK 解析流水线，忽略了代码生成层的内部质量。审计发现 5 个根本性问题：

**问题 1：魔法字符串 token（`instr.py`）**

`new` 指令将 `"__new__ClassName__"` 压栈；`getstatic System.out` 将 `"__stdout__"` 压栈。这些是编码在字符串里的语义信息，在下游 `_gen_invokevirtual` 中被字符串匹配识别。任何类名包含这些模式时会静默出错。

**问题 2：字符串类型标注（`stack.py` + `instr.py`）**

栈存 `(str_expr, str_type)` 二元组，类型信息是 `'i32'`、`'ArrayList<String>'` 这样的裸字符串。`pop_str()` 返回渲染后的字符串，类型信息在传递过程中被丢弃或误用。

**问题 3：if-chain JDK 分派（`instr.py`）**

`_gen_invokevirtual` 里有 4 段 `if cls in (...): _dispatch_xxx(...)` 链。扩展新集合类需改 if-chain，不具备开放/封闭性。

**问题 4：正则后处理（`method.py`）**

`_fix_coll_types`、`_remove_unnecessary_mut`、`_merge_aliases`、`_simplify_wrapping_add` 4 个函数对生成后的字符串做正则修补。信息在 IR 层就存在，但被序列化成字符串后再用正则逆向推断，脆弱且不可靠。

**问题 5：`RawExpr`/`RawStmt` 无处不在**

`rs_ir.py` + `render.py` 已完整实现，但 `instr.py` 中 99% 的指令仍直接调用 `sim.push(f"...", 'i32')` 和 `sim.emit(f"...")` 产生原始字符串。IR 层形同虚设。

### 1.3 依赖关系

这 5 个问题是原始计划 P4（删除 JDK 硬编码）的先决条件：

```
问题 1-5 修复（阶段 A + B）
    ↓
P4：删除 instr.py 中 JDK 分派 if-chain
    ↓
P1 格式升级（阶段 C）
    ↓
P1 完整 JDK 字节码翻译（阶段 D）
    ↓
P3 native_impls（阶段 E）
```

---

## 2. 最终态架构

### 2.1 架构层次与硬编码边界

**设计原则：凡是字节码操作数或注释里已有的信息，一律从字节码解析，Python 代码中不重复写一遍。只有字节码里没有答案的行为映射，才允许显式定义。**

| 指令类型 | 数据来自哪里 | 正确处理方式 |
|---------|------------|------------|
| `getstatic / putstatic` | 注释中的 `Field ClassName.field:Type` | 通用解析 → `StaticFieldRef`，无硬编码 |
| `getfield / putfield` | 同上 | 通用解析 → `FieldAccess`，无硬编码 |
| `new` | 注释中的类名 | 通用解析 → `NewPendingExpr`，无硬编码 |
| `invokevirtual`（JDK 类，Phase D 之前）| 字节码没有 Rust 翻译方式 | 查 `jdk_dispatch.py` 过渡表（见下方说明）|
| `invokevirtual`（JDK 类，Phase D 之后）| JDK 字节码已翻译为 Rust 方法 | 与用户类方法完全相同的通用处理 |
| `invokevirtual`（用户类）| 字节码中的方法签名 | 通用解析 → `MethodCall`，无硬编码 |

**关于 `jdk_dispatch.py` 的定位：过渡桥梁，非最终态。**

Phase B 引入 `jdk_dispatch.py` 是因为 Phase D（完整 JDK 字节码翻译）尚未完成。在 JDK 类没有被翻译成 Rust 之前，`ArrayList.add` 该生成什么 Rust 代码无法从字节码中自动推导，必须手工定义。

Phase D 完成后，`jdk_dispatch.py` 整个文件删除——此时 JDK 类的 Rust 实现已经存在，`invokevirtual` 统一走通用路径，不再需要任何手工注册表。

### 2.2 模块职责（最终态，Phase D 完成后）

```
classfile.py      解析 .class 字节码 → ClassInfo
jdk_resolver.py   从 jmods 读取 JDK .class → bytes
type_map.py       JVM 原始类型 → Rust 类型（仅原始类型，无任何 JDK 类名）
rs_ir.py          IR 节点定义
render.py         IR → Rust 字符串（唯一产生字符串的地方）
stack.py          操作数栈，存 (RsExpr, RsType)
instr.py          JVM 指令 → IR 节点，无 JDK 类名常量，无字符串拼接
method.py         IR 收集 + mutation 分析 + render，无正则后处理
emitter.py        ClassInfo → .rs 文件
transpile.py      翻译流水线

[过渡期存在，Phase D 后删除]
jdk_dispatch.py   JDK 类行为映射表（仅在 Phase B-C 期间有效）
```

### 2.2 数据流（最终态）

```
JVM 字节码
    │ sim_instr() → IR 节点
    ▼
StackSim 栈：[(RsExpr, RsType)]
    │ flush() → 收集 RsStmt 列表
    ▼
method.py：mutation 分析（扫描 AssignStmt 目标）
    │ render_stmt()
    ▼
Rust 源码字符串（仅此一处产生）
```

### 2.3 类型系统（最终态）

```
RsType
├── RsPrimitive('i32' | 'i64' | 'f32' | 'f64' | 'bool' | '()')
├── RsNamed(name: str)                 # 用户类、JDK 类
├── RsGeneric(base: str, params: list[RsType])   # ArrayList<i32>
├── RsRef(inner: RsType, mutable: bool)
├── RsSlice(elem: RsType)
├── RsTuple(elems: list[RsType])
└── RsInfer                            # 新增：_ 占位，用于泛型推断
```

```
RsExpr（扩展新增）
├── NewPendingExpr(class_name: str)    # new 指令压栈，等待 <init> 调用
├── StaticFieldRef(class_name: str, field_name: str, ty: RsType)  # System.out
└── ... （已有节点不变）
```

---

## 3. 阶段 A：IR 全量迁移

目标：`instr.py` 中全部指令产生 IR 节点，消灭魔法字符串，消灭 `method.py` 中的正则后处理。

### 3.1 `rs_ir.py` 扩展（3 个新节点）

```python
@dataclass
class NewPendingExpr(RsExpr):
    """对应 'new ClassName' 指令，等待 invokespecial <init> 确定构造参数。"""
    class_name: str

@dataclass
class StaticFieldRef(RsExpr):
    """对应 'getstatic ClassName.fieldName' 指令。"""
    class_name: str
    field_name: str
    ty: 'RsType'

@dataclass
class RsInfer(RsType):
    """对应 Rust 的 _ 类型推断占位符，用于集合泛型初始定型。"""
    pass
```

### 3.2 `stack.py` 改造

`stack.py` 已经导入 `rs_ir.py`，但 `push()` 和 `store_local()` 仍接受字符串做兼容：

**删除向后兼容路径**，全部要求 IR 节点：

```python
def push(self, expr: RsExpr, ty: RsType = I32):
    # 不再接受 str，如有 str 则在调用方修改
    assert isinstance(expr, RsExpr)
    assert isinstance(ty, RsType)
    self.stack.append((expr, ty))

def pop(self) -> tuple[RsExpr, RsType]:
    return self.stack.pop() if self.stack else (RawExpr('/* UNDERFLOW */'), I32)
```

删除 `pop_str()`、`load_local_str()`、`push_str()`（这些方法存在是为了向 `instr.py` 的字符串代码提供兼容性）。

`store_local` 改为直接生成 `LetStmt(mutable=False, ...)`，mutation 分析在 `method.py` 中统一做。

### 3.3 `instr.py` 全量改造

对每个 JVM 指令分组依次迁移：

**整型常量（示例）：**
```python
# 旧
sim.push(f"{val}i32")

# 新
sim.push(Lit(val, I32), I32)
```

**算术运算（示例）：**
```python
# 旧
b,_=sim.pop_str(); a,_=sim.pop_str()
sim.push(f"({a}).wrapping_add({b})")

# 新
b, b_ty = sim.pop(); a, a_ty = sim.pop()
sim.push(BinOp('wrapping_add', a, b), I32)
```

**`new` 指令（消灭魔法字符串）：**
```python
# 旧
cls = comment.split('/')[-1]
sim.push(f"__new__{cls}__", f"__pending__{cls}")

# 新
cls = comment  # 保留完整 binary name
sim.push(NewPendingExpr(cls), RsNamed(cls))
```

**`getstatic`（消灭魔法字符串，通用解析）：**
```python
# 旧（只处理 System.out，写死类名）
sim.push('__stdout__', 'PrintStream')

# 新（通用解析注释中的字段引用，无任何 JDK 类名写死）
elif op == 'getstatic':
    cls, field, descriptor = parse_field_ref(comment)  # 解析 "Field java/lang/System.out:Ljava/io/PrintStream;"
    ty = jvm_to_rust(descriptor)
    sim.push(StaticFieldRef(cls, field, RsNamed(ty)), RsNamed(ty))
```

`putstatic` / `getfield` / `putfield` 同理，全部通用解析，**Python 代码中不出现任何 JDK 类名常量**。

**store/load：**
```python
# 旧
e, _ = sim.pop_str()
sim.store_local(slot, e, 'i32')

# 新
e, ty = sim.pop()
sim.store_local(slot, e, ty)
```

### 3.4 `method.py` 删除正则后处理

删除以下函数，替代方案在 IR 层实现：

| 删除函数 | 原因 | IR 替代 |
|---------|------|--------|
| `_fix_coll_types` | 类型在构建时由 `RsInfer` + `jdk_dispatch.py` 确定 | 见阶段 B |
| `_remove_unnecessary_mut` | `LetStmt.mutable` 由 IR mutation 分析正确设置 | 见下方 |
| `_merge_aliases` | `store_local` 直接用目标变量名，无冗余临时变量 | `store_local` 改造 |
| `_simplify_wrapping_add` | `iinc` 直接生成 `AssignStmt(BinOp("+="))` | `iinc` 指令改造 |

**mutation 分析（替代 `_remove_unnecessary_mut`）：**

在 `gen_method_body` 中，收集所有 IR 节点后：

```python
def _analyze_mutation(stmts: list[RsStmt]) -> set[str]:
    """返回被赋值的变量名集合（有 AssignStmt 或 index 写入）。"""
    mutated = set()
    for stmt in stmts:
        if isinstance(stmt, AssignStmt):
            if isinstance(stmt.target, Var):
                mutated.add(stmt.target.name)
            elif isinstance(stmt.target, Index):
                if isinstance(stmt.target.base, Var):
                    mutated.add(stmt.target.base.name)
    return mutated
```

`store_local` 生成 `LetStmt(mutable=False, ...)`；`gen_method_body` 在渲染前遍历 IR，将出现在 `mutated` 集合中的变量的 `LetStmt.mutable` 改为 `True`。

---

## 4. 阶段 B：数据驱动 JDK 分派（过渡态）

**定位：Phase B 引入 `jdk_dispatch.py` 是 Phase D 完成之前的过渡桥梁。Phase D（完整 JDK 字节码翻译）完成后，`jdk_dispatch.py` 整个文件删除，`invokevirtual` 对所有类统一处理，无任何特殊逻辑。**

目标：新建 `jdk_dispatch.py`，以注册表替代 `instr.py` 中的 if-chain，所有分派函数返回 IR 节点。

### 4.1 注册表结构

`JDK_CLASS_GROUPS`（枚举所有实现类）被完全删除，替换为 `JDK_INTERFACE_GROUPS`（只枚举接口）。
组 ID 在运行时通过查询 `ClassRegistry` 中的继承关系动态推导，无需人工维护实现类列表。

```python
# jdk_dispatch.py

# 接口 binary name → 组 ID
# 接口数量少且极稳定；实现类（ArrayList/LinkedList/TreeMap/...）无需列出，自动覆盖
JDK_INTERFACE_GROUPS: dict[str, str] = {
    'java/util/List':       'list',
    'java/util/Map':        'map',
    'java/util/Set':        'set',
    'java/lang/Appendable': 'appendable',  # StringBuilder / StringBuffer
}


def get_dispatch_group(class_binary_name: str, registry: 'ClassRegistry') -> str | None:
    """从类的接口实现关系推导组 ID，无任何类名硬编码。"""
    cls_info = registry.get(class_binary_name)
    if not cls_info:
        return None
    for iface in cls_info.all_interfaces():   # 递归展开：自身接口 + 父类接口
        if group := JDK_INTERFACE_GROUPS.get(iface):
            return group
    return None


# Handler 签名: (sim, obj_expr, obj_ty, args) -> None
Handler = Callable[[StackSim, RsExpr, RsType, list[tuple[RsExpr, RsType]]], None]

JDK_VIRTUAL_METHODS: dict[tuple[str, str], Handler] = {
    ('list',       'add'):        _list_add,
    ('list',       'get'):        _list_get,
    ('list',       'size'):       _list_size,
    ('list',       'remove'):     _list_remove,
    ('map',        'put'):        _map_put,
    ('map',        'get'):        _map_get,
    ('map',        'size'):       _map_size,
    ('map',        'containsKey'):_map_contains_key,
    ('set',        'add'):        _set_add,
    ('set',        'size'):       _set_size,
    ('set',        'contains'):   _set_contains,
    ('appendable', 'append'):     _appendable_append,
    ('appendable', 'toString'):   _appendable_to_string,
    # 扩展新方法：只加这里，不动 instr.py
    # 扩展新实现类（LinkedList/TreeMap/...）：零改动，自动继承接口映射
}
```

**扩展性对比：**

| 场景 | 旧方案（枚举类名） | 新方案（接口映射）|
|------|-----------------|----------------|
| 支持 `LinkedList` | 手动加 2 行 | 自动（`implements List`）|
| 支持 `TreeMap` | 手动加 2 行 | 自动（`implements Map`）|
| 支持用户自定义 `List` | 不支持 | 自动 |
| 表的行数 | 随实现类线性增长 | 固定（接口数量有限）|

### 4.2 分派函数签名（IR 节点）

```python
def _list_add(sim: StackSim, obj: RsExpr, obj_ty: RsType,
              args: list[tuple[RsExpr, RsType]]) -> None:
    elem_expr, elem_ty = args[0]
    sim.emit(ExprStmt(MethodCall(obj, 'add', [elem_expr], with_try=True)))
    # 同时更新 obj_ty 的泛型参数（若为 RsInfer）
    _resolve_coll_type(sim, obj, elem_ty)

def _list_get(sim: StackSim, obj: RsExpr, obj_ty: RsType,
              args: list[tuple[RsExpr, RsType]]) -> None:
    idx_expr, _ = args[0]
    elem_ty = _elem_type_of(obj_ty)  # 从已确定的泛型参数取
    v = sim.fresh('_e')
    sim.emit(LetStmt(v, elem_ty, mutable=False,
                     value=MethodCall(obj, 'get', [Cast(idx_expr, RsNamed('usize'))],
                                      with_try=True)))
    sim.push(Var(v), elem_ty)
```

### 4.3 泛型类型实时确定（替代 `_fix_coll_types`）

当 `new ArrayList` 时压 `NewPendingExpr('ArrayList')`，类型为 `RsGeneric('ArrayList', [RsInfer()])`。

当 `invokespecial ArrayList.<init>` 时，将该 `LetStmt` 的类型固定。

当 `list.add(10i32)` 时，`_list_add` 调用 `_resolve_coll_type(sim, obj, I32)`：查找栈/局部变量中该对象的 `LetStmt`，将 `RsInfer` 替换为 `I32`。

### 4.4 `instr.py` 的 `_gen_invokevirtual` 改造

```python
def _gen_invokevirtual(sim: StackSim, comment: str, class_name: str):
    cls, mname, param_jvm_types, ret_jvm_type = parse_method_ref(comment)
    n_args = len(param_jvm_types)
    args = [sim.pop() for _ in range(n_args)][::-1]
    obj_expr, obj_ty = sim.pop()

    # JDK 分派：从类的接口实现关系推导组 ID，无类名硬编码
    full_name = _full_binary_name(cls, comment)   # 从 invokevirtual 注释获取完整 binary name
    group = get_dispatch_group(full_name, registry)
    if group:
        handler = JDK_VIRTUAL_METHODS.get((group, mname))
        if handler:
            handler(sim, obj_expr, obj_ty, args)
            return

    # 用户类方法调用
    arg_exprs = [e for e, _ in args]
    ret_ty = jvm_to_rust(ret_jvm_type)
    call = MethodCall(obj_expr, mname, arg_exprs, with_try=True)
    if ret_jvm_type == 'V':
        sim.emit(ExprStmt(call))
    else:
        v = sim.fresh('_t')
        sim.emit(LetStmt(v, RsNamed(ret_ty), mutable=False, value=call))
        sim.push(Var(v), RsNamed(ret_ty))
```

---

## 5. 阶段 C：属性格式升级与元数据完整化

目标：生成的 `.rs` 文件成为 Java 元数据的**唯一持久化载体**。所有类层次、接口实现、字段、方法签名信息通过 Rust 属性嵌入，`build.rs` 扫描属性即可自动重建完整 `ClassRegistry`，无需任何手工维护的类名表。

### 5.1 设计原则

生成的 `.rs` 文件具有双重角色：
1. **Rust 实现**：可编译的结构体、方法（Phase D 完成后）
2. **元数据载体**：通过 `#[java_*]` 属性保存 Java 类的全部结构信息，供 `build.rs` 在构建期扫描

`build.rs` 只读属性，不依赖任何手工维护的类名/接口名常量。

### 5.2 完整属性格式定义

**普通类（Phase D 后含完整实现；Phase D 前为元数据存根）：**

```rust
#[java_class(
    binary_name  = "java/util/ArrayList",
    super_class  = "java/util/AbstractList",
    interfaces   = "java/util/List,java/util/RandomAccess,java/lang/Cloneable,java/io/Serializable",
    generic_sig  = "<E:Ljava/lang/Object;>Ljava/util/AbstractList<TE;>;Ljava/util/List<TE;>;",
    access       = "public",
    source       = "ArrayList.java",
)]
pub struct ArrayList<E> { ... }
```

**接口：**

```rust
#[java_interface(
    binary_name       = "java/util/List",
    super_interfaces  = "java/util/Collection",
    generic_sig       = "<E:Ljava/lang/Object;>Ljava/util/Collection<TE;>;",
    access            = "public abstract interface",
)]
pub trait List<E>: Collection<E> { ... }
```

**方法（普通）：**

```rust
#[java_method(
    name        = "add",
    descriptor  = "(Ljava/lang/Object;)Z",
    access      = "public",
    throws      = "",
)]
pub fn add(&self, e: E) -> Result<bool> { ... }
```

**方法（native，无字节码，生成 `todo!` 存根）：**

```rust
#[java_native(
    name        = "arraycopy",
    descriptor  = "(Ljava/lang/Object;ILjava/lang/Object;II)V",
    access      = "public static native",
)]
pub fn arraycopy(...) -> Result<()> {
    todo!("native java/lang/System.arraycopy:(Ljava/lang/Object;ILjava/lang/Object;II)V")
}
```

**Phase D 前的 JDK 元数据存根（仅含属性，无实现）：**

```rust
// 此文件由 java_rta 自动生成，仅供 build.rs 扫描。不参与 Rust 模块编译。

#[java_class(
    binary_name = "java/util/ArrayList",
    super_class = "java/util/AbstractList",
    interfaces  = "java/util/List,java/util/RandomAccess,java/lang/Cloneable,java/io/Serializable",
    access      = "public",
    source      = "ArrayList.java",
)]
struct _JavaClassMarker;

#[java_native(name = "...", descriptor = "...", access = "...")]
fn _method_stub() {}
```

### 5.3 `build.rs` 自动建立 ClassRegistry

`build.rs` 扫描所有 `.rs` 文件，从属性中提取 `binary_name`、`super_class`、`interfaces` 等字段，自动重建完整类层次图：

```rust
struct ClassMeta {
    binary_name:  String,
    super_class:  String,
    interfaces:   Vec<String>,      // 直接实现的接口
    all_ifaces:   Vec<String>,      // 递归展开后的全部接口（含父类、父接口）
    is_interface: bool,
    native_methods: Vec<NativeMethodMeta>,
}

impl ClassRegistry {
    /// 扫描 src/ 下所有 .rs 文件，从 #[java_class] / #[java_interface] 属性重建类层次
    pub fn scan(src_dir: &str) -> Self {
        // 1. 从属性提取原始 ClassMeta（含直接接口列表）
        // 2. 递归展开 all_ifaces：遍历 super_class 和每个接口的 super_interfaces
        // 3. 无任何硬编码类名/接口名
    }

    /// 判断某个类是否实现了指定接口
    pub fn implements(&self, binary_name: &str, interface: &str) -> bool {
        self.get(binary_name)
            .map(|m| m.all_ifaces.iter().any(|i| i == interface))
            .unwrap_or(false)
    }
}
```

`build.rs` 使用 `ClassRegistry` 做的所有分析（接口实现验证、native 状态追踪）全部从属性数据自动推导，不维护任何类名常量。

### 5.4 `emitter.py` 修改

```python
def _java_class_attr(ci: ClassInfo) -> str:
    """将 ClassInfo 的全部元信息序列化为 #[java_class(...)] 属性字符串。"""
    parts = [f'binary_name = "{ci.name}"']
    if ci.super_class:
        parts.append(f'super_class = "{ci.super_class}"')
    if ci.interfaces:
        parts.append(f'interfaces  = "{",".join(ci.interfaces)}"')
    if ci.generic_signature:
        parts.append(f'generic_sig = "{ci.generic_signature}"')
    parts.append(f'access = "{_access_str(ci.access_flags)}"')
    if ci.source_file:
        parts.append(f'source = "{ci.source_file}"')
    inner = ',\n    '.join(parts)
    return f'#[java_class(\n    {inner},\n)]'
```

`ClassInfo` 中的字段（`super_class`、`interfaces`、`generic_signature`）均来自 `.class` 文件解析，不在 Python 代码中硬编码任何 JDK 类名。

---

## 6. 阶段 D：完整 JDK 字节码翻译

目标：JDK 类文件从"仅含元数据注解"升级为"完整翻译字节码"，完成原始计划的 P1 核心目标。

### 6.1 前提条件

阶段 A、B 完成后，`instr.py` 已能对任意字节码产生干净的 IR 输出。JDK 字节码翻译复用完全相同的流程，无需特殊处理。

### 6.2 翻译流程变更

当前 `_gen_jdk_class_rs()` 生成元数据存根。完成本阶段后改为调用完整翻译流程：

```python
def _gen_jdk_class_rs(ci: ClassInfo) -> str:
    # 与用户类翻译完全相同，唯一区别是路径在 src/java/ 下
    return gen_class_rs(ci)
```

仅有 `is_native=True` 的方法生成 `todo!` 存根（因为 native 方法没有字节码）。

### 6.3 JDK 类与用户类的统一处理

`emitter.py` 中 `write_cargo_project()` 的当前逻辑：

```python
# 用户类 → src/ClassName.rs
for ci in class_infos:
    write_user_class(out_dir, ci)

# JDK 类 → src/java/.../ClassName.rs（仅元数据）
for jdk_ci in jdk_class_infos:
    write_jdk_metadata(out_dir, jdk_ci)
```

阶段 D 后统一为：

```python
# 用户类和 JDK 类使用相同的翻译路径
for ci in class_infos + jdk_class_infos:
    write_class(out_dir, ci)
```

### 6.4 `jdk_dispatch.py` 删除，`invokevirtual` 统一化

阶段 D 完成后，JDK 类已有完整 Rust 实现。`JDK_VIRTUAL_METHODS` 中每个 handler 所定义的"该调用什么 Rust 方法"，此时已由翻译后的 Rust 方法签名自动表达。

`_gen_invokevirtual` 退化为最简形式，对所有类（JDK 或用户类）完全一致：

```python
def _gen_invokevirtual(sim, comment, class_name):
    cls, mname, param_jvm_types, ret_jvm_type = parse_method_ref(comment)
    args = [sim.pop() for _ in range(len(param_jvm_types))][::-1]
    obj_expr, obj_ty = sim.pop()

    # 所有类统一处理，无任何 JDK 类名判断，无 dispatch 表查询
    rust_name = to_snake(mname)
    call = MethodCall(obj_expr, rust_name, [e for e, _ in args], with_try=True)
    if ret_jvm_type == 'V':
        sim.emit(ExprStmt(call))
    else:
        v = sim.fresh('_t')
        ret_ty = jvm_to_rust(ret_jvm_type)
        sim.emit(LetStmt(v, RsNamed(ret_ty), mutable=False, value=call))
        sim.push(Var(v), RsNamed(ret_ty))
```

删除内容：
- `jdk_dispatch.py` 整个文件（含 `JDK_INTERFACE_GROUPS`、`get_dispatch_group()`、`JDK_VIRTUAL_METHODS`、所有 handler 函数）
- `type_map.py` 中剩余的任何 JDK 类名常量

**此后 Python 代码中不包含任何 JDK 类名。所有信息均从字节码注释解析。**

---

## 7. 阶段 E：native_impls 与构建阻断

目标：完成原始计划的 P3，建立手写 native 实现目录，`build.rs` 在有未实现 native 方法时阻断构建。

### 7.1 目录结构

```
output/
└── native_impls/
    └── java/
        ├── lang/
        │   ├── object.rs        # Object.hashCode, Object.clone 等
        │   ├── system.rs        # System.arraycopy, System.currentTimeMillis 等
        │   └── string.rs        # String.charAt, String.length 等
        └── io/
            └── print_stream.rs  # 如有 native 方法
```

### 7.2 `native_status.toml` 语义

```toml
# status 值：
#   "implemented" — native_impls/ 中有签名匹配的实现
#   "needed"      — 当前 RTA 可达集合中有调用，尚无实现，阻断编译
#   "not-needed"  — 不可达或已确认不需要实现（人工设置，build.rs 保留）

[java.lang.Object]
hashCode    = "implemented"
clone       = "needed"

[java.lang.System]
arraycopy   = "needed"
```

### 7.3 构建阻断

`build.rs` 遇到 `status = "needed"` 时输出 `cargo:error=` 终止构建：

```rust
if let Some(needed) = new_status.get(&class_key)
    .and_then(|m| m.get(&method_key))
    .filter(|s| s.as_str() == "needed")
{
    println!("cargo:error=Native method needs implementation: {class_key}.{method_key}");
    println!("cargo:error=  Add impl to native_impls/{}", class_path);
}
```

---

## 8. 里程碑汇总

| 阶段 | 任务 | 完成标志 |
|------|------|---------|
| **A1** | `rs_ir.py` 扩展：`NewPendingExpr`, `StaticFieldRef`, `RsInfer` | 新节点通过 `render.py` 渲染正确 |
| **A2** | `stack.py` 删除字符串兼容路径，全量使用 IR | `pop_str()` / `load_local_str()` 方法删除 |
| **A3** | `instr.py` 全部指令迁移到 IR 节点（含消灭 `__new__` / `__stdout__`）| `instr.py` 中无 f-string 拼接 |
| **A4** | `method.py` 删除 `_fix_coll_types` / `_remove_unnecessary_mut` / `_merge_aliases` / `_simplify_wrapping_add`，改为 IR mutation 分析 | `method.py` 无 `import re` |
| **A5** | 全部测试（HelloWorld + P1 + P2 + P3）通过 `cargo check` | `cargo check` 0 errors |
| **B1** | 新建 `jdk_dispatch.py`：`JDK_CLASS_GROUPS` + `JDK_VIRTUAL_METHODS` 注册表 | 注册表覆盖 list/map/set/sb/print_stream/math |
| **B2** | `instr.py` 的 `_gen_invokevirtual` 改为查注册表 | 删除所有 `_dispatch_list/map/set` 函数 |
| **B3** | 泛型类型实时确定，`_fix_coll_types` 彻底删除 | `type_map.py` 无 `JDK_COLL_TYPES` |
| **B4** | 全部测试通过 `cargo check` + 生成代码正确性验证 | P3 测试输出与 Java 一致 |
| **C1** | `emitter.py` 升级 JDK 元数据文件格式为 `#[java_class(...)]` 属性 | 生成文件使用 `#[...]` 语法 |
| **C2** | `build.rs` 更新正则，扫描属性格式 | `native_status.toml` 内容不变，由新格式生成 |
| **D1** | `_gen_jdk_class_rs` 调用完整字节码翻译流程 | JDK 类 `.rs` 文件有完整 Rust 方法体 |
| **D2** | 用户类与 JDK 类统一翻译入口 | `write_cargo_project` 无用户/JDK 分支 |
| **E1** | 建立 `native_impls/java/lang/system.rs`，实现 HelloWorld 最小 native 集合 | `cargo run` 输出正确 |
| **E2** | `build.rs` 将 `needed` native 方法升级为 `cargo:error=` 阻断构建 | 有未实现方法时构建报错，无时构建成功 |

---

## 附：受影响文件清单

| 文件 | 阶段 | 变更性质 |
|------|------|---------|
| `scripts/codegen/rs_ir.py` | A1 | 扩展新节点 |
| `scripts/codegen/render.py` | A1 | 新节点渲染（小改） |
| `scripts/codegen/stack.py` | A2 | 删除兼容路径，严格 IR |
| `scripts/codegen/instr.py` | A3, B2 | 全量 IR 化，删 if-chain |
| `scripts/codegen/method.py` | A4 | 删除所有正则后处理 |
| `scripts/codegen/jdk_dispatch.py` | B1 | 新建 |
| `scripts/codegen/type_map.py` | B3 | 删除 `JDK_COLL_TYPES` |
| `scripts/codegen/emitter.py` | C1 | 属性格式升级 |
| `output/build.rs` | C2, E2 | 正则更新 + 构建阻断 |
| `output/native_impls/` | E1 | 新建目录与实现文件 |
