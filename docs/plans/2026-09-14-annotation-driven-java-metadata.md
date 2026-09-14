# 注解驱动的 Java 元数据方案

**日期**：2026-09-14  
**目标**：通过在生成的 Rust 文件上标注完整 Java 元数据，使任何后续处理逻辑（继承展开、接口 dispatch、instanceof、代码生成）都能从注解中获取所需信息，无需重新解析 `.class` 文件或维护独立数据结构。

---

## 一、两种继承实现方案的分析与选择

在确定实施路径前，先把两个备选方案的边界和缺陷说清楚。

### 1.1 方案 A：ClassHierarchy（Python 侧字段展平）

Python 转译器在运行期维护全局继承图，转译时把祖先字段递归复制进子类 struct：

```rust
// Python 把 Throwable/Exception/IOException 的字段全部复制进来
pub struct FileNotFoundException {
    pub detailMessage: JField<String>,  // 来自 Throwable（复制）
    pub cause:         JField<Object>,  // 来自 Throwable（复制）
    pub fileName:      JField<String>,  // 自身字段
}
```

**核心缺陷：字段语义错误。**

当 `FileNotFoundException` 向上转型为 `IOException` 时，需要把字段值复制出去，
两个对象各持有独立的 `detailMessage` 副本。通过 `FileNotFoundException` 修改字段，
再以 `IOException` 视角读取，读到的是另一个副本——这违反 Java 的对象语义（对象是单一实体）。

另一个问题：ClassHierarchy 信息只存在于 Python 进程生命周期内，Rust 编译器、proc-macro、build.rs 都无法访问，无法在 Rust 编译期做任何基于继承的自动代码生成。

### 1.2 方案 B：注解驱动 + _super 嵌套 + Deref

注解把继承信息嵌入 .rs 文件，struct 嵌入 `_super` 字段，proc-macro 生成 `Deref`：

```rust
impl Deref for FileNotFoundException {
    type Target = IOException;
    fn deref(&self) -> &IOException { &self._super }
}
```

`_super` 嵌套解决了字段语义问题（物理上只有一份），但 **Deref 模拟继承是 Rust 社区明确的反模式**（违反 Deref 的语义预期：它表示"智能指针解引用"，不表示"继承向上转型"）。主要问题：

- `Deref` 只能有一个 `Target`，接口方法不走 Deref，仍需显式 trait impl
- 深层继承时（5 层）错误信息极难读懂（`expected IOException, found FileNotFoundException` 出现在多处）
- 与 Rust API Guidelines 冲突，生成代码对 Rust 开发者反直觉

### 1.3 选定方案：注解驱动 + _super 嵌套 + 显式 upcast 方法

结合两个方案的优点，规避各自的缺陷：

| 维度 | ClassHierarchy | Deref 方案 | **选定方案** |
|------|---------------|-----------|------------|
| 字段语义 | ❌ 有副本语义缺陷 | ✅ 单一物理副本 | ✅ 单一物理副本 |
| 继承信息持久化 | ❌ 仅 Python 进程内 | ✅ 持久在 .rs 文件 | ✅ 持久在 .rs 文件 |
| Rust 惯用法 | ✅ 普通 struct | ❌ Deref 反模式 | ✅ 普通方法调用 |
| 跨层访问 | ✅ 直接访问 | ✅ 自动穿透 | ⚠️ 需显式写一层 |
| 实现复杂度 | 中 | 高（proc-macro）| 中（emitter.py）|

**核心思路**：

- `_super` 嵌套字段保留（解决字段语义）
- 用**显式 upcast 方法**替代 Deref（解决 Rust 反模式）
- Python 侧 ClassHierarchy 仍然存在（作为生成期内部数据结构），但输出改为 `_super` 嵌套，而非字段展平
- 注解持久化继承元数据（供 build.rs 和未来工具使用）

```rust
// emitter.py 生成 upcast 方法（不生成 Deref impl）：
impl FileNotFoundException {
    pub fn as_io_exception(&self) -> &IOException     { &self._super }
    pub fn into_io_exception(self)  -> IOException    { self._super }
    pub fn as_exception(&self)      -> &Exception     { &self._super._super }
    pub fn into_exception(self)     -> Exception      { self._super._super }
}
```

调用侧：`fnfe.as_io_exception().getMessage()?` — 完全显式，无隐式穿透。

---

## 二、设计原则

### 2.1 注解 = Java 声明的完整镜像

每个生成的 Rust 文件中的注解与对应 Java 文件的声明信息完全一致：

- 类级别：继承类、实现接口、访问修饰符、类修饰符
- 字段级别：名称、JVM 类型描述符、访问权限、修饰符（static/final/volatile/transient）
- 方法级别：名称、JVM 描述符、访问权限、修饰符（static/native/abstract/final/synchronized）

### 2.2 只标注直接声明，链路靠遍历

每个类只记录自己直接声明的父类和接口，不展开完整继承链：

```
FileNotFoundException（super_class=IOException, interfaces=Serializable）
  → IOException（super_class=Exception, interfaces=""）
    → Exception（super_class=Throwable, interfaces=Serializable）
      → Throwable（super_class=Object, interfaces=Serializable）
        → Object（super_class=""）→ 停止
```

### 2.3 _super 字段语义

当前 java_rta 的对象模型是**值语义**：struct 持有 `JField<T>`（`Box<RefCell<T>>`），
对象以值方式传递或 clone，**没有 JRef/JRc 堆引用层**。
因此 `_super: IOException`（值嵌套）是可行的，不存在引用层冲突。

---

## 三、注解格式规范

### 3.1 类注解（`#[java_class(...)]`）

```rust
#[java_rta_macros::java_class(
    binary_name = "java/util/LinkedHashMap",
    super_class  = "java/util/HashMap",
    interfaces   = "java/util/Map,java/util/SequencedMap",
    access       = "public",
    modifiers    = "final",
    source       = "LinkedHashMap.java",
)]
pub struct LinkedHashMap<K, V> {
    pub _super: HashMap<K, V>,
    // 自身字段...
}
```

多接口示例：

```rust
#[java_rta_macros::java_class(
    binary_name = "java/util/ArrayList",
    super_class  = "java/util/AbstractList",
    interfaces   = "java/util/List,java/util/RandomAccess,java/lang/Cloneable,java/io/Serializable",
    access       = "public",
    modifiers    = "",
    source       = "ArrayList.java",
)]
pub struct ArrayList<E> {
    pub _super: AbstractList<E>,
    pub elementData: JField<Object>,
    pub size: JField<i32>,
}
```

无父类（Object 本身）：

```rust
#[java_rta_macros::java_class(
    binary_name = "java/lang/Object",
    super_class  = "",
    interfaces   = "",
    access       = "public",
    modifiers    = "",
)]
pub struct Object { ... }
```

接口生成为 Rust trait：

```rust
#[java_rta_macros::java_interface(
    binary_name      = "java/io/Serializable",
    super_interfaces = "",
    access           = "public",
    modifiers        = "interface abstract",
)]
pub trait Serializable {}
```

### 3.2 字段注解（`#[java_field(...)]`）

```rust
#[java_field(
    name       = "elementData",
    descriptor = "[Ljava/lang/Object;",
    access     = "private",
    modifiers  = "transient",
)]
pub elementData: JField<Object>,

#[java_field(
    name       = "SIZE",
    descriptor = "I",
    access     = "public",
    modifiers  = "static final",
)]
pub SIZE: JField<i32>,
```

### 3.3 方法注解（`#[java_method(...)]`）

```rust
#[java_method(
    name       = "add",
    descriptor = "(Ljava/lang/Object;)Z",
    access     = "public",
    modifiers  = "",
)]
pub fn add(&self, e: Object) -> Result<bool> { ... }

#[java_method(
    name       = "arraycopy",
    descriptor = "(Ljava/lang/Object;ILjava/lang/Object;II)V",
    access     = "public",
    modifiers  = "static native",
)]
pub fn arraycopy(src: Object, srcPos: i32, dest: Object, destPos: i32, length: i32) -> Result<()> {
    panic!("stub: java/lang/System.arraycopy:(Ljava/lang/Object;ILjava/lang/Object;II)V")
}

#[java_method(
    name       = "size",
    descriptor = "()I",
    access     = "public",
    modifiers  = "abstract",
)]
pub fn size(&self) -> Result<i32> {
    panic!("stub: abstract java/util/AbstractCollection.size:()I")
}
```

---

## 四、emitter.py 生成 `_super` 字段和 upcast 方法

### 4.1 生成 `_super` 字段

```python
# codegen/emitter.py: _gen_struct_fields()
def _gen_struct_fields(ci: ClassInfo, ...) -> list[str]:
    lines = []
    # 若有父类（且不是 Object），第一个字段为 _super
    if ci.super_class and ci.super_class != 'java/lang/Object':
        parent_rust = short_cls(ci.super_class)
        lines.append(f'    pub _super: {parent_rust},')
    for f in ci.fields:
        lines.append(_gen_field_line(f, ...))
    return lines
```

### 4.2 生成显式 upcast 方法（不使用 Deref）

```python
# codegen/emitter.py: _gen_upcast_methods()
def _gen_upcast_methods(ci: ClassInfo, registry: dict) -> str:
    """为有父类的类生成显式 upcast 方法（替代 Deref）"""
    if not ci.super_class or ci.super_class == 'java/lang/Object':
        return ''
    lines = [f'impl {short_cls(ci.name)} {{']
    # 沿祖先链逐层生成 as_xxx / into_xxx
    chain = []   # [(rust_name, access_path)]
    cur_super = ci.super_class
    access_path = '_super'
    while cur_super and cur_super != 'java/lang/Object':
        rust_name = short_cls(cur_super)
        snake = to_snake(rust_name)
        lines.append(f'    pub fn as_{snake}(&self) -> &{rust_name} {{ &self.{access_path} }}')
        lines.append(f'    pub fn into_{snake}(self) -> {rust_name} {{ self.{access_path} }}')
        parent_ci = registry.get(cur_super)
        cur_super = parent_ci.super_class if parent_ci else None
        access_path += '._super'
    lines.append('}')
    return '\n'.join(lines)
```

生成结果示例：

```rust
impl FileNotFoundException {
    pub fn as_io_exception(&self)  -> &IOException  { &self._super }
    pub fn into_io_exception(self) ->  IOException  { self._super }
    pub fn as_exception(&self)     -> &Exception    { &self._super._super }
    pub fn into_exception(self)    ->  Exception    { self._super._super }
    pub fn as_throwable(&self)     -> &Throwable    { &self._super._super._super }
    pub fn into_throwable(self)    ->  Throwable    { self._super._super._super }
}
```

### 4.3 From impl（直接父类）

proc-macro 读 `super_class` 注解，只生成**直接父类**的 `From` impl（不跨层）：

```rust
// proc-macro java_class 自动派生（仅直接父类）：
impl From<FileNotFoundException> for IOException {
    fn from(v: FileNotFoundException) -> IOException { v._super }
}
```

跨层的 `From<FileNotFoundException> for Throwable` 由 build.rs 扫描注解链路生成（T55）。

---

## 五、注解完整化：modifiers 字段

### 5.1 类 modifiers

```python
_ACC_SYNTHETIC  = 0x1000
_ACC_ANNOTATION = 0x2000

def _class_modifiers_str(flags: int) -> str:
    parts = []
    if flags & _ACC_FINAL:      parts.append('final')
    if flags & _ACC_ABSTRACT:   parts.append('abstract')
    if flags & _ACC_INTERFACE:  parts.append('interface')
    if flags & _ACC_ENUM:       parts.append('enum')
    if flags & _ACC_SYNTHETIC:  parts.append('synthetic')
    if flags & _ACC_ANNOTATION: parts.append('annotation')
    return ' '.join(parts)
```

### 5.2 字段 modifiers

```python
_ACC_VOLATILE  = 0x0040
_ACC_TRANSIENT = 0x0080

def _field_modifiers_str(flags: int) -> str:
    parts = []
    if flags & _ACC_STATIC:    parts.append('static')
    if flags & _ACC_FINAL:     parts.append('final')
    if flags & _ACC_VOLATILE:  parts.append('volatile')
    if flags & _ACC_TRANSIENT: parts.append('transient')
    if flags & _ACC_SYNTHETIC: parts.append('synthetic')
    return ' '.join(parts)
```

### 5.3 方法 modifiers

```python
_ACC_SYNCHRONIZED = 0x0020
_ACC_BRIDGE       = 0x0040
_ACC_VARARGS      = 0x0080

def _method_modifiers_str(flags: int) -> str:
    parts = []
    if flags & _ACC_STATIC:       parts.append('static')
    if flags & _ACC_FINAL:        parts.append('final')
    if flags & _ACC_SYNCHRONIZED: parts.append('synchronized')
    if flags & _ACC_NATIVE:       parts.append('native')
    if flags & _ACC_ABSTRACT:     parts.append('abstract')
    if flags & _ACC_BRIDGE:       parts.append('bridge')
    if flags & _ACC_VARARGS:      parts.append('varargs')
    if flags & _ACC_SYNTHETIC:    parts.append('synthetic')
    return ' '.join(parts)
```

---

## 六、build.rs：扫描注解生成跨层 From impl（T55）

proc-macro 只生成直接父类的 `From`。跨层 `From`（`FileNotFoundException → Throwable`）由 build.rs 在编译前生成：

```rust
// output/jdk_classes/build.rs（扩展）
fn main() {
    // 1. 扫描所有 .rs 文件，提取 java_class 注解
    let hierarchy = scan_annotations("src");  // 返回 HashMap<binary_name, super_class>
    // 2. 对每个类，沿祖先链生成跨层 From impl
    let mut code = String::new();
    for (cls, super_chain) in &hierarchy {
        let child_rust = rust_name(cls);
        for (depth, anc) in super_chain.iter().enumerate().skip(1) {
            // skip(1) 跳过直接父类（proc-macro 已生成）
            let anc_rust = rust_name(anc);
            let via = "_super".repeat(depth + 1);  // "_super._super._super"
            code += &format!(
                "impl From<{child_rust}> for {anc_rust} {{\n\
                 fn from(v: {child_rust}) -> {anc_rust} {{ v.{via} }}\n}}\n"
            );
        }
    }
    // 3. 写入 OUT_DIR
    let out = std::env::var("OUT_DIR").unwrap();
    std::fs::write(format!("{out}/java_from_impls.rs"), code).unwrap();
}
```

**注意**：build.rs 需要解析 `#[java_class(...)]` 属性，用正则或 syn crate 均可，后者更健壮但需要增加依赖。

---

## 七、信息获取路径

| 需求 | 获取方式 |
|------|---------|
| 父类是谁 | 读本文件 `java_class.super_class` |
| 直接接口 | 读本文件 `java_class.interfaces` |
| 完整祖先链 | 递归跳转：本文件 → 父类文件 → 祖父类文件 → ... |
| 字段是否静态 | 读字段 `java_field.modifiers` 含 `static` |
| 方法是否 native | 读方法 `java_method.modifiers` 含 `native` |
| 方法是否 abstract | 读方法 `java_method.modifiers` 含 `abstract` |
| 是否实现了接口 X | 读本文件 `interfaces`，或递归查祖先的 `interfaces` |

---

## 八、实施顺序

### Phase 1：注解完整化（T78，emitter.py，低风险，优先完成）

1. `_java_class_attr` 加 `modifiers` 字段
2. `_java_field_attr` 加 `modifiers` 字段
3. `_java_method_attr` 加 `modifiers` 字段

验收：重新生成代码，任意类文件的注解包含完整元数据。

### Phase 2：_super 字段 + upcast 方法（T76，emitter.py）

1. struct 生成时插入 `pub _super: ParentType` 作为第一个字段
2. 为每个有父类的类生成显式 `as_xxx` / `into_xxx` upcast 方法
3. proc-macro 读 `super_class` 生成直接父类的 `From` impl（不生成 Deref）

验收：E0609 消除；`fnfe.as_throwable()` 可调用；直接父类 `From` 编译通过。

### Phase 3：build.rs 跨层 From impl（T55）

1. build.rs 扫描 .rs 文件提取 `java_class` 注解
2. 构建内存继承图，生成跨层 `From` impl
3. `include!` 引入生成代码

验收：`FileNotFoundException` 可直接 `.into()` 得到 `Throwable`；E0308 中跨层类型不匹配消除。

---

## 九、已知边界与后续工作

- **接口 trait 方法**：`java_class.interfaces` 生成空 trait impl（stub），接口方法的实现由各类自己的字节码翻译完成，不在本方案范围内
- **泛型父类**（`HashMap<K,V>` extends `AbstractMap<K,V>`）：`_super` 字段的类型参数处理由 T66（泛型精化）负责，本阶段用 `_super: AbstractMap<K, V>` 占位
- **Python 侧 ClassHierarchy**：仍然作为 emitter.py 内部数据结构存在，用于生成 upcast 方法（需要沿祖先链遍历），但不再做字段展平

---

*本方案面向最终架构，无过渡方案。*
