# JVM Class File 属性检查清单

> 基于 JVMS §4.7（Java SE 21）  
> 最后更新：2026-09-13

## 说明

| 标志 | 含义 |
|------|------|
| ✅ 已解析 | `classfile.py` 中有完整解析逻辑 |
| ⚠️ 部分解析 | 仅读取部分字段，其余跳过 |
| ❌ 未解析 | 用 `_skip_attribute` 直接跳过 |
| 🟢 已用于 codegen | 解析结果被生成代码逻辑使用 |
| 🟡 已存储未使用 | 解析后存入数据结构，但 codegen 未使用 |
| ⚪ 不适用 | 对本项目 Java→Rust 代码生成无意义 |

---

## 一、预定义属性总表

### 1. 代码与字节码相关

| 属性名 | 作用域 | 解析状态 | 使用状态 | 说明 | 优先级 |
|--------|--------|----------|----------|------|--------|
| `Code` | Method | ✅ 已解析 | 🟢 已用 | 核心属性，包含字节码指令、异常表、子属性 | — |
| `StackMapTable` | Code | ❌ 未解析 | ⚪ 不适用 | JVM 类型验证用，转译时不需要 | 低 |
| `LineNumberTable` | Code | ❌ 未解析 | ⚪ 不适用 | 调试信息，转译不需要 | 低 |
| `LocalVariableTable` | Code | ✅ 已解析 | 🟢 已用 | 局部变量名（slot→name 映射），用于生成可读变量名 | — |
| `LocalVariableTypeTable` | Code | ✅ 已解析 | 🟢 已用 | 泛型版本的局部变量类型，slot→Signature 映射，覆盖 Object 为精确类型（TE; → E，Ljava/lang/String; → String） | — |

### 2. 泛型与签名

| 属性名 | 作用域 | 解析状态 | 使用状态 | 说明 | 优先级 |
|--------|--------|----------|----------|------|--------|
| `Signature` | Class | ✅ 已解析 | 🟢 已用 | `ClassInfo.generic_signature` → `parse_class_type_params()` → 生成 `struct ArrayList<E>` 等泛型结构 | — |
| `Signature` | Method | ✅ 已解析 | 🟢 已用 | `ParsedMethod.generic_signature` → method.py 推断泛型参数/返回类型 | — |
| `Signature` | Field | ✅ 已解析 | 🟡 已存储未使用 | `FieldInfo.generic_signature` 存有字段泛型类型（如 `E`），但 emitter 生成字段时仍用裸描述符，`elementData` 生成为 `JField<Object>` 而非 `JField<E>` | **中** |

### 3. 常量与初始化

| 属性名 | 作用域 | 解析状态 | 使用状态 | 说明 | 优先级 |
|--------|--------|----------|----------|------|--------|
| `ConstantValue` | Field | ❌ 未解析 | 🟡 待用 | `static final` 字段的编译期常量值（int/long/float/double/String） | **中** |

> **影响**：`static final` 字段目前生成 `Default::default()`，有了此属性可生成正确的常量初始值。

### 4. 异常声明

| 属性名 | 作用域 | 解析状态 | 使用状态 | 说明 | 优先级 |
|--------|--------|----------|----------|------|--------|
| `Exceptions` | Method | ✅ 已解析 | 🟡 已存储未使用 | 方法 `throws` 声明的受检异常列表，存入 `ParsedMethod.exceptions` | 低 |

> **影响**：Rust 的 `Result<T, E>` 中 E 类型目前固定，有了此属性可生成更具体的错误类型。

### 5. 内部类与嵌套类型

| 属性名 | 作用域 | 解析状态 | 使用状态 | 说明 | 优先级 |
|--------|--------|----------|----------|------|--------|
| `InnerClasses` | Class | ❌ 未解析 | 🟡 待用 | 内部类/外部类关系映射，生成嵌套 mod 结构需要 | **中** |
| `EnclosingMethod` | Class | ❌ 未解析 | ⚪ 不适用 | 匿名类/局部类所在方法，转译时通常不需要 | 低 |
| `NestHost` | Class | ❌ 未解析 | ⚪ 不适用 | Java 11+ 嵌套访问控制 | 低 |
| `NestMembers` | Class | ❌ 未解析 | ⚪ 不适用 | Java 11+ 嵌套成员列表 | 低 |

### 6. 注解

| 属性名 | 作用域 | 解析状态 | 使用状态 | 说明 | 优先级 |
|--------|--------|----------|----------|------|--------|
| `RuntimeVisibleAnnotations` | Class/Field/Method | ❌ 未解析 | ⚪ 不适用 | 运行时可见注解（如 `@Override`） | 低 |
| `RuntimeInvisibleAnnotations` | Class/Field/Method | ❌ 未解析 | ⚪ 不适用 | 编译期注解 | 低 |
| `RuntimeVisibleParameterAnnotations` | Method | ❌ 未解析 | ⚪ 不适用 | 参数注解 | 低 |
| `RuntimeInvisibleParameterAnnotations` | Method | ❌ 未解析 | ⚪ 不适用 | 参数注解 | 低 |
| `RuntimeVisibleTypeAnnotations` | Class/Field/Method/Code | ❌ 未解析 | ⚪ 不适用 | Java 8+ 类型注解（如 `@NotNull`） | 低 |
| `RuntimeInvisibleTypeAnnotations` | Class/Field/Method/Code | ❌ 未解析 | ⚪ 不适用 | 类型注解 | 低 |
| `AnnotationDefault` | Method | ❌ 未解析 | ⚪ 不适用 | 注解接口的默认值 | 低 |

### 7. 反射与元数据

| 属性名 | 作用域 | 解析状态 | 使用状态 | 说明 | 优先级 |
|--------|--------|----------|----------|------|--------|
| `MethodParameters` | Method | ❌ 未解析 | 🟡 待用 | 方法参数名（需编译器 `-parameters` 选项），可补全 LocalVariableTable 缺少参数名的情况 | **中** |
| `SourceFile` | Class | ✅ 已解析 | 🟡 已存储未使用 | 原始 .java 文件名，存入 `ClassInfo.source_file` | 低 |
| `SourceDebugExtension` | Class | ❌ 未解析 | ⚪ 不适用 | JSR 45 调试扩展（JSP 等） | 低 |
| `Deprecated` | Class/Field/Method | ❌ 未解析 | 🟡 待用 | `@Deprecated` 标记，可生成 `#[deprecated]` 属性 | 低 |
| `Synthetic` | Class/Field/Method | ✅ 已解析 | 🟢 已用 | 编译器合成成员；与 `ACC_SYNTHETIC`（0x1000）访问标志等价，两者均设置 `is_synthetic=True`，emitter 用 `m.is_synthetic` 过滤合成方法 | — |

### 8. 动态调用与 lambda

| 属性名 | 作用域 | 解析状态 | 使用状态 | 说明 | 优先级 |
|--------|--------|----------|----------|------|--------|
| `BootstrapMethods` | Class | ✅ 已解析 | 🟢 已用 | `invokedynamic` 引导方法（lambda、字符串拼接等） | — |

### 9. 记录类（Java 16+）

| 属性名 | 作用域 | 解析状态 | 使用状态 | 说明 | 优先级 |
|--------|--------|----------|----------|------|--------|
| `Record` | Class | ❌ 未解析 | 🟡 待用 | record 类的组件列表，可映射为 Rust struct + 字段 | 低 |

### 10. 封闭类（Java 17+）

| 属性名 | 作用域 | 解析状态 | 使用状态 | 说明 | 优先级 |
|--------|--------|----------|----------|------|--------|
| `PermittedSubclasses` | Class | ❌ 未解析 | ⚪ 不适用 | sealed class 允许的子类列表，Rust 用 enum 表达 | 低 |

### 11. 模块系统（Java 9+）

| 属性名 | 作用域 | 解析状态 | 使用状态 | 说明 | 优先级 |
|--------|--------|----------|----------|------|--------|
| `Module` | Class | ❌ 未解析 | ⚪ 不适用 | 模块声明（requires/exports/opens 等） | 低 |
| `ModulePackages` | Class | ❌ 未解析 | ⚪ 不适用 | 模块中所有包列表 | 低 |
| `ModuleMainClass` | Class | ❌ 未解析 | ⚪ 不适用 | 模块的主类 | 低 |

---

## 二、汇总统计

| 类别 | 数量 |
|------|------|
| 总属性数（JVMS Java 21） | 30 |
| ✅ 已解析 | 7（`Code`、`LocalVariableTable`、`Signature`×3、`Exceptions`、`BootstrapMethods`、`SourceFile`、`Synthetic`） |
| 🟢 已用于 codegen | 5（`Code`、`LocalVariableTable`、`BootstrapMethods`、`Signature`[Class+Method]、`Synthetic`） |
| 🟡 已存储未使用 | 3（`Signature`[Field]、`Exceptions`、`SourceFile`） |
| ❌ 未解析 | 23 |

---

## 三、优先实现路线图

### 高优先级（影响当前功能正确性）

> 无——此类已全部完成。

### 中优先级（改善代码生成质量）

1. **`Signature`（Field 级，使用）** — 生成字段的精确泛型类型
   - 影响：`elementData: JField<Object>` → `JField<E>`，struct 内部类型更准确
   - 改动：emitter 在生成字段时优先用 `FieldInfo.generic_signature`，回退到 `descriptor`

2. **`ConstantValue`（解析+使用）** — `static final` 字段正确初始化
   - 影响：`public static final int MAX = 100` 等常量

4. **`MethodParameters`（解析+使用）** — 补全方法参数名
   - 影响：LocalVariableTable 有时不含参数名（如编译时未带 `-g`）

5. **`InnerClasses`（解析+使用）** — 生成正确的嵌套 mod 结构
   - 影响：匿名类、内部类的翻译

6. ~~**`LocalVariableTypeTable`**~~ — ✅ **已完成（T34）**：slot→Signature 映射，覆盖 Object 为精确类型（TE; → E，Ljava/lang/String; → String）

### 低优先级（调试信息、历史遗留）

7. `Exceptions`（使用）— 生成更精确的 `Result<T, E>` 错误类型
8. `Deprecated`（解析+使用）— 生成 `#[deprecated]` 属性
9. `Record`（解析+使用）— Java record 类映射为 Rust struct
10. 注解相关属性 — 大多数对代码生成无实际影响

---

## 四、当前 classfile.py 解析状态速查

```
classfile.py 解析的属性（按出现位置）：

字段属性：
  ✅ Signature → FieldInfo.generic_signature（已存储，emitter 尚未使用，字段类型仍用裸描述符）

方法属性：
  ✅ Code     → ParsedMethod（字节码 + 异常表）
  ✅ Exceptions → ParsedMethod.exceptions
  ✅ Signature → ParsedMethod.generic_signature
  ✅ Synthetic → ParsedMethod.is_synthetic（与 ACC_SYNTHETIC 标志等价，二者均处理）

Code 子属性：
  ✅ LocalVariableTable → ParsedMethod.local_names
  ❌ StackMapTable      → _skip_attribute
  ❌ LineNumberTable    → _skip_attribute
  ✅ LocalVariableTypeTable → ParsedMethod.local_types（slot → Signature）
  ❌ 其他              → _skip_attribute

类属性：
  ✅ BootstrapMethods → bootstrap_methods 列表
  ✅ Signature        → ClassInfo.generic_signature
  ✅ SourceFile       → ClassInfo.source_file
  ❌ InnerClasses     → _skip_attribute
  ❌ 其他             → _skip_attribute

注：字段/类级别的 Synthetic 属性目前未解析（现代 Java 均用 ACC_SYNTHETIC 标志），
    方法级 Synthetic 属性已解析（兼容旧版 class 文件）。
```
