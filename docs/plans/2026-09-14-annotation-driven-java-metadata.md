# 注解驱动的 Java 元数据方案

**日期**：2026-09-14  
**目标**：通过在生成的 Rust 文件上标注完整 Java 元数据，使任何后续处理逻辑（继承展开、接口 dispatch、instanceof、代码生成）都能从注解中获取所需信息，无需重新解析 `.class` 文件或维护独立数据结构。

---

## 一、设计原则

### 1.1 注解 = Java 源文件信息的完整镜像

每个生成的 Rust 文件中的注解，与对应 Java 文件的声明信息完全一致：

- 类级别：继承类、实现接口、访问修饰符
- 字段级别：名称、JVM 类型描述符、访问权限、修饰符（static/final/volatile/transient）
- 方法级别：名称、JVM 描述符、访问权限、修饰符（static/native/abstract/final/synchronized）

### 1.2 只标注直接声明，链路靠遍历

每个类只记录自己直接声明的父类和接口，不展开完整继承链。完整链路由使用方（proc-macro / build.rs）递归遍历获得：

```
FileNotFoundException（super_class=IOException）
  → IOException（super_class=Exception）
    → Exception（super_class=Throwable）
      → Throwable（super_class=Object）
        → Object（super_class=""）→ 停止
```

### 1.3 struct 嵌入 `_super` 字段，继承通过 Deref 穿透

每个有父类的 struct 的第一个字段固定为 `pub _super: ParentType`。Rust 通过 `Deref` 链自动向上查找字段和方法，与 Java 继承语义一致：

```rust
let fnfe: FileNotFoundException = ...;
fnfe.fileName.get()       // 自身字段
fnfe.detailMessage.get()  // Deref → IOException → Exception → Throwable.detailMessage ✓
```

---

## 二、注解格式规范

### 2.1 类注解（`#[java_class(...)]`）

```rust
#[java_rta_macros::java_class(
    binary_name = "java/util/LinkedHashMap",   // JVM 二进制类名（/分隔）
    super_class  = "java/util/HashMap",         // 直接父类，无则为空串
    interfaces   = "java/util/Map,java/util/SequencedMap",  // 直接实现的接口（逗号分隔）
    access       = "public",                    // public / protected / private / package
    modifiers    = "final",                     // abstract / final / enum / interface / synthetic（空格分隔）
    source       = "LinkedHashMap.java",        // 源文件名
)]
pub struct LinkedHashMap<K, V> {
    pub _super: HashMap<K, V>,                  // 对应 extends HashMap
    // ... 自身字段
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

无父类示例（Object 本身 / 接口）：

```rust
// Object 无父类
#[java_rta_macros::java_class(
    binary_name = "java/lang/Object",
    super_class  = "",
    interfaces   = "",
    access       = "public",
    modifiers    = "",
)]
pub struct Object { ... }

// 接口生成为 Rust trait
#[java_rta_macros::java_interface(
    binary_name  = "java/io/Serializable",
    super_interfaces = "",    // extends OtherInterface 时填写
    access       = "public",
    modifiers    = "interface abstract",
)]
pub trait Serializable {}
```

### 2.2 字段注解（`#[java_field(...)]`）

```rust
#[java_field(
    name       = "elementData",          // Java 字段名
    descriptor = "[Ljava/lang/Object;",  // JVM 类型描述符
    access     = "private",              // public / protected / private / package
    modifiers  = "transient",            // static / final / volatile / transient / synthetic（空格分隔）
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

### 2.3 方法注解（`#[java_method(...)]`）

```rust
#[java_method(
    name       = "add",
    descriptor = "(Ljava/lang/Object;)Z",   // JVM 方法描述符
    access     = "public",
    modifiers  = "",                         // static / final / synchronized / native / abstract / bridge / varargs
)]
pub fn add(&self, e: Object) -> Result<bool> { ... }

// native 方法（无字节码，body 为 stub）
#[java_method(
    name       = "arraycopy",
    descriptor = "(Ljava/lang/Object;ILjava/lang/Object;II)V",
    access     = "public",
    modifiers  = "static native",
)]
pub fn arraycopy(src: Object, srcPos: i32, dest: Object, destPos: i32, length: i32) -> Result<()> {
    panic!("stub: java/lang/System.arraycopy:(Ljava/lang/Object;ILjava/lang/Object;II)V")
}

// abstract 方法（生成为 panic stub，由子类实现）
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

## 三、proc-macro 根据注解自动生成的内容

`java_rta_macros::java_class` 读取注解后，自动派生以下代码：

### 3.1 继承相关（从 `super_class` 生成）

```rust
// 输入（标注了 super_class = "java/io/IOException"）：
#[java_rta_macros::java_class(
    binary_name = "java/io/FileNotFoundException",
    super_class  = "java/io/IOException",
    interfaces   = "java/io/Serializable",
)]
pub struct FileNotFoundException {
    pub _super: IOException,
    pub fileName: JField<String>,
}

// proc-macro 自动生成：

// 1. Deref → 透明访问父类字段和方法
impl std::ops::Deref for FileNotFoundException {
    type Target = IOException;
    fn deref(&self) -> &IOException { &self._super }
}
impl std::ops::DerefMut for FileNotFoundException {
    fn deref_mut(&mut self) -> &mut IOException { &mut self._super }
}

// 2. 直接父类上转型
impl From<FileNotFoundException> for IOException {
    fn from(v: FileNotFoundException) -> IOException { v._super }
}

// 3. Object 上转型（已有，确保保留）
impl From<FileNotFoundException> for Object {
    fn from(v: FileNotFoundException) -> Object { Object::from_any(v) }
}
```

### 3.2 接口相关（从 `interfaces` 生成）

```rust
// proc-macro 为每个直接接口生成 trait impl（空实现或转发）：
impl Serializable for FileNotFoundException {}
```

若接口 trait 有方法，且类本身有对应方法注解，则自动转发：

```rust
impl List for ArrayList {
    fn size(&self) -> Result<i32> { self.size() }
    fn get(&self, index: i32) -> Result<Object> { self.get(index) }
    // ...
}
```

### 3.3 跨层 From impl（从注解链路遍历生成）

build.rs（而非 proc-macro，因为需要跨文件信息）扫描所有 `#[java_class]` 注解，
为每对祖先-后代类生成 From impl：

```rust
// build.rs 生成到 OUT_DIR/java_from_impls.rs：
impl From<FileNotFoundException> for Exception {
    fn from(v: FileNotFoundException) -> Exception { v._super.into() }
}
impl From<FileNotFoundException> for Throwable {
    fn from(v: FileNotFoundException) -> Throwable { v._super.into().into() }
}
```

---

## 四、emitter.py 需要的改动

### 4.1 补全 `java_class` 注解中的 `modifiers` 字段

```python
# codegen/emitter.py: _java_class_attr()
def _modifiers_str(flags: int) -> str:
    """从 access_flags 提取修饰符（区别于 access 权限）"""
    parts = []
    if flags & _ACC_FINAL:     parts.append('final')
    if flags & _ACC_ABSTRACT:  parts.append('abstract')
    if flags & _ACC_INTERFACE: parts.append('interface')
    if flags & _ACC_ENUM:      parts.append('enum')
    if flags & 0x1000:         parts.append('synthetic')  # ACC_SYNTHETIC
    return ' '.join(parts)

# 在 _java_class_attr 中增加：
inner_lines.append(f'    modifiers   = "{_modifiers_str(ci.access_flags)}",')
```

### 4.2 补全 `java_field` 注解中的 `modifiers` 字段

```python
# codegen/emitter.py: _java_field_attr()
def _field_modifiers_str(flags: int) -> str:
    parts = []
    if flags & _ACC_STATIC:    parts.append('static')
    if flags & _ACC_FINAL:     parts.append('final')
    if flags & 0x0040:         parts.append('volatile')   # ACC_VOLATILE
    if flags & 0x0080:         parts.append('transient')  # ACC_TRANSIENT
    if flags & 0x1000:         parts.append('synthetic')
    return ' '.join(parts)
```

### 4.3 补全 `java_method` 注解中的 `modifiers` 字段

```python
# codegen/emitter.py: _java_method_attr()
def _method_modifiers_str(flags: int) -> str:
    parts = []
    if flags & _ACC_STATIC:    parts.append('static')
    if flags & _ACC_FINAL:     parts.append('final')
    if flags & _ACC_NATIVE:    parts.append('native')
    if flags & _ACC_ABSTRACT:  parts.append('abstract')
    if flags & 0x0020:         parts.append('synchronized')  # ACC_SYNCHRONIZED
    if flags & 0x0040:         parts.append('bridge')        # ACC_BRIDGE
    if flags & 0x0080:         parts.append('varargs')       # ACC_VARARGS
    if flags & 0x1000:         parts.append('synthetic')
    return ' '.join(parts)
```

### 4.4 生成 `_super` 字段

```python
# codegen/emitter.py: struct 字段生成段
def _gen_struct_fields(ci: ClassInfo, ...) -> list[str]:
    lines = []
    # 若有父类（且父类不是 Object），第一个字段为 _super
    if ci.super_class and ci.super_class != 'java/lang/Object':
        parent_rust = short_cls(ci.super_class)
        # 若父类有类型参数，需要通配符占位（暂用 Object）
        lines.append(f'    pub _super: {parent_rust},')
    # 再生成自身字段
    for f in ci.fields:
        lines.append(_gen_field_line(f, ...))
    return lines
```

---

## 五、信息获取路径

有了完整注解，任何工具都可以通过以下方式获取 Java 类型系统信息：

| 需求 | 获取方式 |
|------|---------|
| 父类是谁 | 读本文件 `java_class.super_class` |
| 直接接口 | 读本文件 `java_class.interfaces` |
| 完整祖先链 | 递归跳转：本文件 → 父类文件 → 祖父类文件 → ... |
| 字段是否静态 | 读字段 `java_field.modifiers` 含 `static` |
| 方法是否 native | 读方法 `java_method.modifiers` 含 `native` |
| 方法是否 abstract | 读方法 `java_method.modifiers` 含 `abstract` |
| 是否实现了接口 X | 读本文件 `interfaces`，或递归查祖先的 `interfaces` |
| 字段类型 | 读 `java_field.descriptor`（JVM 描述符）|
| 方法签名 | 读 `java_method.descriptor`（JVM 描述符）|

---

## 六、实施顺序

### Phase 1：注解完整化（emitter.py，无风险）
1. 在 `_java_class_attr` 加 `modifiers` 字段
2. 在 `_java_field_attr` 加 `modifiers` 字段
3. 在 `_java_method_attr` 加 `modifiers` 字段
4. 在 struct 生成段加 `pub _super: ParentType` 字段

**验收**：重新生成代码，检查任意类文件的注解是否包含完整元数据。

### Phase 2：proc-macro 读注解派生代码（java_rta_macros）
1. `java_class` 宏：读 `super_class` → 生成 `Deref` + `DerefMut` + `From<Self> for Parent`
2. `java_class` 宏：读 `interfaces` → 生成空 `trait impl`
3. `java_interface` 宏（新建）：生成对应 Rust `trait`

**验收**：`FileNotFoundException` 可以 `.deref()` 访问 `IOException` 的字段；
         `let e: IOException = fnfe.into()` 编译通过。

### Phase 3：build.rs 扫描注解，生成跨层 From impl
1. build.rs 遍历所有 `jdk_classes/src/**/*.rs`
2. 提取 `java_class` 注解，构建内存中的继承图
3. 为每对祖先-后代生成 `From<Child> for Ancestor` impl，写入 `OUT_DIR`
4. `mod.rs` 中 `include!(concat!(env!("OUT_DIR"), "/java_from_impls.rs"))`

**验收**：`FileNotFoundException` 可以直接 `into()` 转为 `Exception` 或 `Throwable`；
         E0308 错误中"expected Ancestor, found Descendant"类型全部消除。

---

## 七、任务对应关系

| 任务 | 本方案对应内容 |
|------|--------------|
| T78（新）| Phase 1：注解完整化，补全 modifiers |
| T55 → 重定义 | Phase 3：build.rs 扫描注解构建继承图，不再是 Python 侧 ClassHierarchy |
| T76 → 重定义 | Phase 1 的 `_super` 字段 + Phase 2 的 proc-macro Deref/From，替代字段展平 |
| T77 | 不变，T76 完成后评估 |

---

## 八、与现有代码的兼容性

- **`#[java_class(...)]` 属性**：已存在，只需补 `modifiers` 字段，向后兼容
- **`#[java_field(...)]`**：已存在，只需补 `modifiers` 字段
- **`#[java_method(...)]`**：已存在，只需补 `modifiers` 字段
- **`_super` 字段**：新增，不影响现有不依赖父类字段的代码
- **proc-macro `java_rta_macros`**：现有宏已生成 `From<X> for Object`，扩展为也生成 `From<X> for Parent`
- **build.rs**：现有 build.rs 已有扫描逻辑，扩展注解解析即可

---

*本方案面向最终架构，无过渡方案。所有步骤完成后，Java 类型系统信息完全由 Rust 注解承载，Python 转译器不再需要维护任何运行时的继承关系数据结构。*
