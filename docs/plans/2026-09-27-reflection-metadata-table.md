# FS-R：反射元数据表（Class / Field / Method / Constructor 回到字节码）

> 状态：R1 / R2 / R3 完成（2026-09-27，3892415 起）；R4 进行中。属 FS-H0「反射元数据」组（过渡态清单 §〇 第一行，33 处越界覆盖）。
> 前置：MH-native 管线（N11 / S-66：MH Direct / Records / BmhDynamicSpecies PASS）。

## 一、现状

反射族以手写越界覆盖承载（`_audit_override` 计数）：

| 类 | 覆盖 | 手写实现依据 |
|---|---|---|
| `Class` | 24：getDeclaredField(s) / Method(s) / Constructor(s)、getField(s) / getMethod / getConstructor(s)、getName / getSimpleName / getPackageName / isMemberClass / isRecord、getAnnotation(s) / isAnnotationPresent、cast / descriptorString / desiredAssertionStatus / getComponentType / getClassLoader / getModule | build.rs 元数据表（CLASS_FIELDS / CLASS_METHODS）直查 |
| `Field` | get / getInt / getLong / set、getAnnotation / getDeclaredAnnotations | L3 字段闭包（reflect_field） |
| `Method` | invoke、getAnnotation / getDeclaredAnnotations | L3 分派闭包（reflect_invoke） |
| `Constructor` | newInstance、getAnnotation | L3 分派闭包 + 序列化构造器登记 |

JDK 的公开方法体可以翻译，被挡住的是它们下方的三件事：

1. **native 成员集**：`getDeclaredFields0/Methods0/Constructors0(boolean publicOnly)`、`getDeclaringClass0`、`getSimpleBinaryName0`、`isRecord0`、`getRecordComponents0`、`getEnclosingMethod0`、`getGenericSignature0`、`getRawAnnotations` + `getConstantPool`。HotSpot 从 class 文件结构取值；原生二进制没有 class 文件。
2. **访问器工厂**：JDK 21 的 `ReflectionFactory.newFieldAccessor/newMethodAccessor/newConstructorAccessor` 一律经 `MethodHandleAccessorFactory`，由 `DirectMethodHandle` 组装。MH-native 管线可以承接，但它们在 `jdk/internal/reflect/`（内部边界），需要按字节码放行。
3. **序列化构造器**：`ReflectionFactory.generateConstructor` 经 `SerializationConstructorAccessorGenerator` 现场生成字节码，这是运行期类定义点，不可翻译。

## 二、方案

### 2.1 元数据表扩展（build.rs，单一来源 = java_class! 属性）

现有 `CLASS_FIELDS`（name / descriptor / modifiers / is_static / constant）与 `CLASS_METHODS`（name / descriptor / modifiers / static / native / abstract / exceptions）按声明序生成，声明序即 slot。补齐以下内容：

- **字段 / 方法 / 构造器的泛型签名**：`generic_signature` 属性已在宏输入中，建表时带出。
- **类级属性**（新增 `CLASS_META`）：
  - 修饰位
  - declaringClass / simpleBinaryName（InnerClasses 属性）
  - enclosingMethod
  - isRecord 与记录组件（Record 属性：name / descriptor / signature）
  - nestHost
  - 泛型签名

  由 class_writer 发射为 java_class! 块属性，build.rs 扫描建表。
- **注解**：保留原始字节（`RuntimeVisibleAnnotations` 等）。常量池只取被引用的 Utf8 / 数值条目，作为该类的稀疏常量表。`jdk/internal/reflect/ConstantPool` 是边界类，手写实现按稀疏表应答 `getUTF8At` / `getIntAt` 等。这样 `AnnotationParser` 可以原样翻译（第 2.4 节，阶段 R4）。

### 2.2 native 层（class_impl.rs，只写 ACC_NATIVE）

- `getDeclaredFields0(publicOnly)`：按表构造 `Field`，走字节码构造器 `Field(Class, String, Class, int, boolean trustedFinal, int slot, String signature, byte[] annotations)`（upcall 入链）。publicOnly 过滤 `ACC_PUBLIC`。trustedFinal 与 HotSpot 同判定：static final、record 字段、隐藏类字段。
- `getDeclaredMethods0` / `getDeclaredConstructors0`：同形，`Method(...)` / `Constructor(...)`，参数类型与异常类型按描述符经 `for_class` 或基本类型镜像解析。
- `getDeclaringClass0` / `getSimpleBinaryName0` / `isRecord0` / `getRecordComponents0` / `getEnclosingMethod0` / `getGenericSignature0` / `getModifiers`：查 `CLASS_META`。
- VM 填充的镜像字段在 `for_class` 建镜像时写入：`componentType`（已完成，`192b9e1`）、`classLoader`（启动类为 null，用户类在 FS-C 组）、`module`（FS-C 组）。

### 2.3 访问器工厂放行（boundary_release.txt）

放行 `jdk/internal/reflect/` 的纯 Java 访问器族，按字节码翻译：
- `MethodHandleAccessorFactory`
- `DirectMethodHandleAccessor`
- `DirectConstructorHandleAccessor`
- `MethodHandle*FieldAccessorImpl`
- `FieldAccessorImpl` / `MethodAccessorImpl` / `ConstructorAccessorImpl`

`Reflection.getCallerClass` / `getClassAccessFlags` 保留 native（已手写）。`useNativeAccessor` 恒走 MH 分支（`-Djdk.reflect.useNativeAccessorOnly` 缺省）。

由此，`Field.get` → `MethodHandle` getter → `reflect_field`，`Method.invoke` → `DirectMethodHandle` → `reflect_invoke`，与 MH Direct 同一管线。

### 2.4 序列化构造器（intrinsics.txt 第二类准入）

`ReflectionFactory.generateConstructor` 是运行期类定义点，改由 VM 内建承载，返回的访问器落到现有 `<alloc>` + `<init_on>` 协议（N2）。这与 N11 的 `generateConcreteSpeciesCode` 同一准入类别。

### 2.5 阶段与验收

| 阶段 | 内容 | 删除的覆盖 | 验收 |
|---|---|---|---|
| R1 | CLASS_META + getDeclaringClass0 / getSimpleBinaryName0 / isRecord0 / getModifiers / getGenericSignature0 | getSimpleName / getPackageName / isMemberClass / isRecord / getName / descriptorString / cast / desiredAssertionStatus | 反射命名族测试、Records |
| R2 | getDeclaredFields0 / Methods0 / Constructors0 + 2.3 访问器放行 | Class 的 12 个成员查询、Field.get/set/getInt/getLong、Method.invoke、Constructor.newInstance | 反射调用 / 字段族、MH Direct、Records（序列化字段回填） |
| R3 | 2.4 序列化构造器内建 | 序列化构造器登记表（serialization_target） | 序列化族（Records / SerializationTest） |
| R4 | 注解原始字节 + 稀疏常量表 + ConstantPool 边界类 + 动态代理 VM 承载（§2.6） | getAnnotation(s) / isAnnotationPresent / getDeclaredAnnotations（5 处）+ 注解代理合成 | 注解族测试 |

每阶段新增 e2e（期望由 JVM 生成），并同步删除对应越界覆盖。越界覆盖只有归零一条路，不另设豁免。

### 2.6 R4 细化：注解 = 注解字节 + 动态代理 VM 承载（2026-09-27 补）

JDK 注解实例链：`Class.getAnnotation` → `annotationData()` → `AnnotationParser.parseAnnotations(
getRawAnnotations(), getConstantPool(), cls)` → `annotationForMap` → **`Proxy.newProxyInstance`**。
动态代理的类由 `Proxy$ProxyBuilder.defineProxyClass` → `ProxyGenerator.generateProxyClass`
在运行期生成，与 N11 物种、R3 序列化访问器同属运行期类定义点。
而且用户代码直接使用 `Proxy.newProxyInstance` 属常见形态，所以动态代理单独作为一项通用能力实现：

| 分项 | 内容 |
|---|---|
| R4a 代理载体 | VM 支持类 `java/lang/reflect/Proxy$Dyn extends Proxy`（字节码翻译，实例携带接口列表），`defineProxyClass` 内建返回之（intrinsics 第二类）。宏新增「代理载体」形态：类属性 `vm_proxy_carrier` 的类，其 `ObjectVTable::is_instance_of` / 接口视图按实例的接口列表应答；接口载体分派的最终回退（现为 AbstractMethodError）前增加 `ObjectVTable::__proxy_invoke(iface, name, desc, args)`，代理载体实现为经 `InvocationHandler.invoke(proxy, Method, args)` 转发（Method 经元数据表按 (iface, name, desc) 取），实参 / 返回按描述符装箱拆箱 |
| R4b 注解字节 | class_writer 发射 RuntimeVisible(Parameter)Annotations 原始字节 + 被引用常量池条目的稀疏表；`getRawAnnotations` / `getConstantPool` native 应答；`jdk/internal/reflect/ConstantPool` 边界类按稀疏表实现 `getUTF8At` / `getIntAt` 等 |
| R4c 删除 | Class / Field / Method / Constructor 的注解查询族越界覆盖、`annotation_meta` / `anno_objects` 翻译期注解代理合成 |

验收：新增 TestDynamicProxy（多接口、default 方法、equals/hashCode/toString、异常透传 / UndeclaredThrowableException）与 TestAnnoReflect 回归。

### 2.7 R2 / R3 实施补记（2026-09-27）

| 项 | 落点 |
|---|---|
| 访问器族放行 | `jdk/internal/reflect/` 整包 + `java/lang/Class$ReflectionData` 等嵌套辅助类（boundary_release.txt） |
| `useNativeAccessor` 分支 | `DirectMethod/ConstructorHandleAccessor$NativeAccessor.invoke0 / newInstance0` native：按 Method / Constructor 声明键经 L3 分派 |
| 实参不符 vs 目标抛出 | `AccessorUtils.isIllegalArgument` 读栈帧——intrinsics.txt 第三类准入「栈帧查询点」：L3 分派登记经用户成员逃逸的异常身份（java.base 帧继续、用户帧判 ITE 的等价物） |
| 引导期时序 | `vm_boot_init.txt`（HotSpot initPhase1 对应物）：`AccessibleObject` 先于应用代码初始化，避免 `ReflectionFactory.<clinit>` 反向触发时重入读到 null 单例 |
| 元数据补齐 | 修饰符表补手写根类 Object 行；直接超接口表 + `getInterfaces0`；`SharedSecrets.getJavaLangInvokeAccess` 槽位 |
| 删除 | Class 成员查询 / 命名族覆盖、Field.get/set、Method.invoke、Constructor.newInstance、序列化构造器登记表、`member_accessible`、`constructor_rows` |

## 三、不做什么

- 不为反射引入 `Jvm*` trait / 自造命名空间；元数据表是 build.rs 生成物，只在 native 层消费。
- 不把全部类的全部成员放进反射面：元数据表记录全部声明，但 L3 分派臂仍按反射引用面（REFLECT_CONSTS）发射。表中有、臂中无的成员被调用时，如实命中存根（`panic!("stub: ...")`）。
