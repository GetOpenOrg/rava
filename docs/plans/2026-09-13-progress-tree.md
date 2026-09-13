# HelloWorld 目标运行路径：问题树与推进层次

**日期**：2026-09-13（持续更新）  
**目标**：HelloWorld 使用 JDK `.class` 字节码翻译出的 Rust 代码运行，不依赖任何手写 runtime

---

## 阅读说明

```
✅ 已解决      ⚠️ 部分解决      ❌ 尚未解决      🔜 待到达
```

每个节点格式：
```
[状态] 问题描述
      原因：...
      解决方案：...
```

---

## 问题树

```
目标：HelloWorld 使用 JDK 字节码翻译的 Rust 代码运行
│
├── 第 1 层：基础设施
│   ├── ✅ javac 编译 + -g 调试信息提取
│   ├── ✅ JDK .class 文件解析（classfile.py）
│   │       解析：常量池、字段、方法、字节码指令、LocalVariableTable
│   ├── ✅ JDK jmod 文件读取（jdk_resolver.py）
│   │       从 $JAVA_HOME/jmods/java.base.jmod 解析 .class 字节流
│   └── ✅ Cargo workspace 生成
│           三层 crate：java_runtime / jdk_classes / user
│           java_runtime 提交到 git（VM 基础设施）
│           jdk_classes / user gitignored（代码生成产物）
│           native_impls/ 在 workspace 根，提交到 git
│
├── 第 2 层：用户类翻译（HelloWorld.java → hello_world.rs）
│   ├── ✅ 类结构生成（struct + impl）
│   ├── ✅ 字段生成（Field<T> 封装）
│   ├── ✅ 泛型参数（PhantomData 补全）
│   │       原因：无实例字段的泛型 struct 触发 E0392
│   │       解决：检测 class_tparams，注入 PhantomData 字段或 tuple struct
│   ├── ✅ 构造器生成（PhantomData 初始化）
│   │       原因：新增 _phantom 字段后构造器未初始化触发 E0063
│   │       解决：method.py 检测 class_tparams，在 Self{} 中注入 _phantom
│   ├── ✅ 方法重载 mangle（overloaded_names + mangle_name）
│   ├── ✅ 标识符 $ 转义（safe_ident 统一处理）
│   │       原因：Java 内部类用 $ 分隔（如 ArrayList$Itr），Rust 不接受
│   │       解决：constants.py 统一 safe_ident()，所有标识符经此过滤
│   └── ✅ 局部变量名安全化（关键字、$ 处理）
│
├── 第 3 层：JDK 类发现（BFS 调用链扫描）
│   ├── ✅ 从用户类指令注释提取 JDK 类引用
│   ├── ✅ BFS 传递闭包扫描（transpile.py _discover_jdk_classes）
│   ├── ✅ 截断规则（_CUTOFF_PREFIXES / _CUTOFF_CLASSES）
│   │       防止 NIO/reflect/Unsafe/并发框架导致依赖爆炸
│   ├── ✅ 手写运行时排除（_JAVA_RUNTIME_CLASSES）
│   │       排除 Object/Math/ArrayList 等，避免与 java_runtime 重复定义
│   │       已迁移：String / System / PrintStream 已从排除集移除
│   └── ⚠️ 扫描粒度：类级别，未到方法级别
│           现状：BFS 按类扫描，把整个 ArrayList 拉进来
│                 即使 HelloWorld 只用了 ArrayList.add，排序相关类也被拉入
│           目标：按方法调用链扫描，未调用的方法其依赖类不纳入 BFS
│           解决方案：解析每个方法的 invokevirtual/invokestatic/invokespecial 指令，
│                     构建方法级调用图，BFS 只追踪实际调用的方法
│
├── 第 4 层：jdk_classes 生成（150 个 JDK 类的 Rust 存根）
│   ├── ✅ 跨包 glob import（use crate::java::pkg::*）
│   │       原因：Objects/Integer 等类引用同 crate 其他类型，编译报"找不到类型"
│   │       解决：mod.rs 加 pub use module::*，每个文件头加 use crate::pkg::*
│   ├── ✅ 方法体存根（panic!("stub/native: ClassName.method:descriptor")）
│   │       替代 todo!()，携带完整描述符，运行时精确定位未覆盖的规则
│   ├── ✅ 方法名去重（used_rust_names 加序号后缀）
│   │       原因：不同描述符 mangle 后可能碰撞（如 new__abstra_i_i）
│   │       解决：emitter.py 维护 used_rust_names dict，碰撞时追加 _N
│   ├── ✅ 存根参数名安全化（_safe_param_name，关键字加 _ 后缀）
│   │       原因：JDK 方法参数名含 type/in 等 Rust 关键字触发 E0178
│   ├── ✅ 调用链路上方法调用 mangle（_mangle_if_overloaded）
│   │       原因：invokestatic Objects.requireNonNull 有两个重载，调用名需 mangle
│   │       解决：instr.py 查 registry 检测重载数，手写运行时类跳过 mangle
│   ├── ✅ $-命名的 .rs 文件（array_list$itr.rs 等）残留
│   │       解决：to_snake() 已有 name.replace('$', '_')，0 个残留文件已验证
│   ├── ✅ #[derive(Default)] 添加到所有 struct
│   │       原因：PrintStream::default()、String::default() 被 native_impls 调用
│   │       解决：emitter._gen_class_rs 对所有生成 struct 加 #[derive(Clone, Default)]
│   └── ✅ 公开静态字段 getter 方法生成（System::out()）
│           原因：getstatic 渲染为 System::out() 调用，但 jdk_classes System 无此方法
│           解决：emitter 扫描 public static 字段，为每个生成 pub fn field_name() 存根
│                 若 native_impls 有对应实现则 dispatch 到 _native::field_name()
│
├── 第 5 层：invokevirtual / invokestatic 指令翻译
│   ├── ✅ invokevirtual 统一路径（obj.method(args)?）
│   ├── ✅ invokestatic 通用解析
│   ├── ✅ invokespecial（构造器 + super 调用）
│   ├── ⚠️ StringBuilder.append / String.append 特殊分支（硬编码）
│   │       现状：instr.py 对 StringBuilder/StringBuffer 做特判，映射到 String 类型
│   │       问题：Python 代码中出现 JDK 类名（违反架构原则）
│   │       缓解：String 已迁移到 jdk_classes，append 通过 @synthetic native_impls 实现
│   │       剩余：消除 instr.py 中 'StringBuilder', 'StringBuffer' 字面量，
│   │             需先完成数组类型支持（第 11 层），使 StringBuilder 可从 JDK 字节码翻译
│   ├── ⚠️ _COLL_IR_TYPES 硬编码（ArrayList/HashMap/HashSet new）
│   │       现状：instr.py 对集合类构造做特判
│   │       问题：Python 代码中出现 JDK 类名（违反架构原则）
│   │       解决方案：第 11 层（数组类型支持）完成后，集合类可从 JDK 字节码翻译
│   └── ⚠️ _JAVA_RUNTIME_SHORT_NAMES 硬编码
│           现状：Object/String/Math 等在此集合，方法调用不加 mangle suffix
│           已清理：System / PrintStream / String 已从此集合移除
│           剩余：Object / Math / ArrayList / HashMap / HashSet / StringBuilder
│           解决方案：对应类迁移到 jdk_classes 后逐步移除
│
├── 第 6 层：JDK 方法体字节码翻译（stub_bodies=False）
│   │       ← 实验结果：全量开启曾触发 3808 个编译错误，逐步修复
│   │
│   ├── ✅ loop 变量先 assign 后 let（300+ 错误 → 0）
│   │       原因：JVM 循环中局部变量先被 istore 赋值（生成 AssignStmt），
│   │             后在 LocalVariableTable 里才有 let 声明（生成 LetStmt）
│   │             导致 Rust 看到变量在声明前被使用
│   │       解决：method.py _promote_undeclared_assigns()：追踪嵌套深度，
│   │             将出作用域后再 assign 的 AssignStmt 提升为 LetStmt(mutable=True)
│   │
│   ├── ✅ Object 类型接收方调用具体方法（大幅减少）
│   │       原因：JVM 类型擦除，接口参数/返回值类型降级为 Object
│   │       已修复：
│   │         - gen_method_body 传 registry → 参数 list:List, c:Comparator 等
│   │         - _gen_invokevirtual/_gen_invokestatic 传 registry → 返回类型更精确
│   │         - getfield/getstatic 传 registry → 字段类型更精确
│   │         - checkcast 指令实现类型更新 → midVal:Comparable 等
│   │       剩余：泛型集合 get() 返回 Object（真实类型擦除），待后续层解决
│   │
│   ├── ✅ 内部类引用（Preconditions 等）生成 panic! 存根
│   │       原因：Objects.checkIndex 调用 jdk.internal.util.Preconditions（被截断）
│   │       解决：instr.py _class_known() + _gen_invokestatic 内部类检测
│   │
│   ├── ✅ aconst_null 指令推送 Object::default() 到操作数栈
│   │       原因：null 参数导致 todo!("stack underflow") 生成
│   │       解决：instr.py 添加 aconst_null 处理
│   │
│   ├── ✅ 泛型类型参数推断（Comparator → Comparator<Object>）
│   │       原因：jvm_to_rust 对泛型类只返回短名，函数签名中触发 E0107
│   │       解决：type_map.py jvm_to_rust() 读 generic_signature 附加 <Object,...>
│   │
│   ├── ✅ bool 返回类型转换（iconst_1/0 → true/false）
│   │       原因：ireturn 将 1i32/0i32 写入 Ok()，与 Result<bool> 不符（E0308）
│   │       解决：method.py _fix_bool_returns() 后处理 Ok(1i32)→Ok(true)
│   │
│   ├── ✅ 静态方法数组参数双重引用（Vec<T> → &&[T]）
│   │       原因：sig_type 把 Vec<T> 转为 &[T]（签名），但 sim 仍用 Vec<T>
│   │       解决：method.py sim 初始化用 sig_type 类型
│   │
│   ├── ✅ bool vs i32 比较（ifne 条件 {a}!=0i32 当 a: bool 时 E0308）
│   │       原因：cfg.py cmp_op 生成 "a!=0i32"，不区分 bool/int 操作数类型
│   │       解决：method.py bool_cond_map handler 检测 a_type 是 bool，
│   │             ifne → 直接用布尔值，ifeq → !bool，跳过 !=0i32 比较
│   │
│   ├── ✅ Object 缺少 null 语义方法（is_none/get）
│   │       原因：ifnull/ifnonnull 生成 .is_none() 调用，Object 无此方法
│   │       解决：runtime.py object.rs 模板添加 is_none()→false, get()→Ok(self.clone())
│   │
│   ├── ✅ Object 缺少 Display trait
│   │       原因：toIdentityString 用 format!("{}", o) 但 Object 无 Display
│   │       解决：runtime.py object.rs 模板添加 impl Display for Object
│   │
│   ├── ✅ _TRANSLATE_BODIES 测试残留（java/util/Objects 留在集合里）
│   │       原因：调试期间向 _TRANSLATE_BODIES 添加了 'java/util/Objects'，未清理
│   │       后果：Objects.toString() 被翻译，引用了 getName/identityHashCode 触发 E0599
│   │       解决：emitter.py _TRANSLATE_BODIES 恢复为 set()（空集合）
│   │
│   └── ⚠️ 泛型集合 get() 返回 Object（类型擦除根因）
│           现状：ArrayList<E>.get() 在字节码中返回 Object，Rust 类型为 Object
│                 用户代码期望 String 但得到 Object → 类型不匹配
│           解决方案：第 11 层（泛型类型推断）或保留手写集合
│
├── 第 7 层：native_impls/ 链接机制
│   ├── ✅ 目录结构与文件建立
│   │       ✅ native_impls/java/lang/system.rs（currentTimeMillis/nanoTime/out/err）
│   │       ✅ native_impls/java/io/print_stream.rs（println__str/println__i/flush）
│   │       ✅ native_impls/java/lang/string.rs（from_owned/append/Display/From<&str>）
│   │       ✅ native_impls/java/lang/double.rs（doubleToRawLongBits/longBitsToDouble）
│   │       ✅ native_impls/java/lang/float.rs（floatToRawIntBits/intBitsToFloat）
│   │       ✅ native_impls/java/lang/throwable.rs（fillInStackTrace no-op）
│   │       ✅ native_impls/java/lang/null_pointer_exception.rs（getExtendedNPEMessage）
│   │
│   ├── ✅ mod _native { include!(...) } 链接机制
│   │       原因：native_impls/ 文件此前只被 build.rs 追踪，不被任何 crate 编译
│   │       解决：emitter 在有 native_impls 的类文件中插入
│   │             mod _native { use java_runtime::prelude::*; use super::*; include!("../..../file.rs"); }
│   │             方法 stub 改为 _native::fn_name(args) dispatch
│   │       include! 路径公式：(pkg_depth + 2) 个 "../" + "native_impls/pkg/class.rs"
│   │
│   ├── ✅ /// @synthetic 合成方法机制
│   │       原因：native_impls 需要提供不对应任何 Java 方法的 Rust 辅助函数
│   │             （如 String::from_owned, String::append）
│   │       解决：native_impls 文件中用 /// @synthetic 标注的 pub fn，
│   │             emitter 在生成的 impl 块中自动生成 wrapper 方法
│   │             静态（无 _this 参数）→ pub fn name(args) { _native::name(args) }
│   │             实例（_this: &T）→ pub fn name(&self, args) { _native::name(self, args) }
│   │             实例可变（_this: &mut T）→ pub fn name(&mut self, ...) { ... }
│   │
│   └── ✅ build.rs native_status.toml 追踪
│           所有 native 方法实现状态：implemented / needed / stub / not-needed
│           当前：所有 needed 方法已标记 implemented
│
├── 第 8 层：_JAVA_RUNTIME_CLASSES 迁移（手写 → 字节码翻译）
│   │       ← HelloWorld 目标架构的核心工作
│   │
│   ├── ✅ java/io/PrintStream 迁移（P4）
│   │       方案：JDK PrintStream.class 字节码翻译（stub 方法体）
│   │             native_impls 短路 println__str/println__i 直接调用 Rust println!
│   │             System::out() 通过 native_impls 返回 PrintStream::default()
│   │       已移除：_JAVA_RUNTIME_CLASSES / _JAVA_RUNTIME_SHORT_NAMES / runtime.py
│   │
│   ├── ✅ java/lang/System 迁移（P4）
│   │       方案：JDK System.class 字节码翻译（unit struct，无实例字段）
│   │             静态字段 out/err 通过 getter stub + native_impls 实现
│   │       已移除：_JAVA_RUNTIME_CLASSES / _JAVA_RUNTIME_SHORT_NAMES / runtime.py
│   │
│   ├── ✅ java/lang/String 迁移（P5）
│   │       关键挑战：
│   │         1. 循环依赖：java_runtime::Object 内部使用 String 返回类型
│   │            解决：Object.getClass/toString 改为返回 Result<Object>，
│   │                  java_runtime 不再引用 String，String 可放入 jdk_classes
│   │         2. byte[] value 字段被 cut-off 为 Field<Object>
│   │            解决：native_impls 通过 Object(Rc::new(rust_string)) 将
│   │                  真实字符串数据存入 Object 字段，Display/append 通过 downcast 读回
│   │         3. String::new() 构造器不兼容（jdk_classes 版本是 stub）
│   │            解决：instr.py StringBuilder 映射改用 String::from("")，
│   │                  type_map.py String 默认值改为 String::default()
│   │         4. from_owned / append 不是 Java 方法
│   │            解决：/// @synthetic 机制，emitter 生成 wrapper
│   │       已移除：_JAVA_RUNTIME_CLASSES / runtime.py string.rs 条目
│   │       保留：_JAVA_RUNTIME_SHORT_NAMES 中的 String（避免方法名加 suffix）
│   │
│   ├── ✅ java/util/ArrayList / HashMap / HashSet 迁移（P7）
│   │       方案：JDK 字节码翻译生成 struct 骨架（存根方法体）
│   │             native_impls 通过 /// @synthetic 提供 new_default/add__obj/get__i/size 等
│   │             impl Into<Object> for String 解决 JVM 引用协变问题
│   │             _gen_invokevirtual 自动为 Object 参数插入 .into() 转换
│   │       已移除：_JAVA_RUNTIME_CLASSES / runtime.py array_list/hash_map/hash_set 条目
│   │       已移除：prelude 不再导出 ArrayList / HashMap / HashSet
│   │
│   └── ✅ java/lang/Math 迁移（P8）
│           Math：全部 native 方法，通过 native_impls/java/lang/math.rs 实现 30+ 方法
│           已移除：_JAVA_RUNTIME_CLASSES / runtime.py math.rs 条目
│           保留：Object 在 java_runtime（VM 基础设施，永久保留）
│
├── 第 9 层：手写 runtime 清理（最终态）
│   ├── ✅ System 从 runtime.py 移除
│   ├── ✅ PrintStream 从 runtime.py 移除
│   ├── ✅ String 从 runtime.py 移除
│   ├── ✅ java_runtime prelude 不再导出 System / PrintStream / String
│   ├── ✅ ArrayList / HashMap / HashSet 从 runtime.py 移除（P7）
│   ├── ✅ Math 从 runtime.py 移除（P8）
│   ├── ❌ Object 仍在 runtime.py（VM 基础设施，部分永久保留）
│   │       object.rs: lock/unlock/hashCode/equals 是 VM 设施，永久保留
│   │       getClass/toString 已改为返回 Object（不再依赖 String）
│   └── ❌ _COLL_IR_TYPES / StringBuilder 特判仍在 instr.py
│
├── 第 10 层：当前验收状态
│   ├── ✅ cargo check 0 errors
│   ├── ✅ HelloWorld 输出 "Hello, World / hahaha / 2"
│   ├── ✅ System → jdk_classes（JDK 字节码翻译 + native_impls）
│   ├── ✅ PrintStream → jdk_classes（JDK 字节码翻译 + native_impls）
│   ├── ✅ String → jdk_classes（JDK 字节码翻译 + native_impls）
│   ├── ✅ ArrayList → jdk_classes + native_impls（P7 完成）
│   ├── ✅ Math → jdk_classes + native_impls（P8 完成）
│   └── ⚠️ Object → java_runtime（VM 基础设施，lock/unlock/hashCode 永久保留）
│
└── 第 11 层：后续架构工作（达到完全目标）
    │
    ├── ✅ 数组类型支持（P6 完成）
    │       类型：所有 Java 数组 Vec<T> → Rc<RefCell<Vec<T>>>（引用语义）
    │       指令：newarray/anewarray、aaload/aastore/arraylength 全部更新
    │       影响：ArrayList 内部 Object[] elementData 正确映射，P7 得以完成
    │
    ├── ✅ Math 迁移（P8 完成）
    │       native_impls/java/lang/math.rs 实现 sin/cos/sqrt/pow/log 等 30+ 方法
    │       已移除：_JAVA_RUNTIME_CLASSES / runtime.py math.rs 条目
    │
    ├── ❌ 泛型类型推断（集合 get() 返回 Object 问题）
    │       问题：ArrayList<E>.get() 字节码返回 Object，
    │             checkcast 后才知道真实类型，但 Rust 不支持运行时类型强制
    │       方案 A：翻译时追踪 checkcast 指令，将变量类型 narrow 到 cast 目标类型
    │       方案 B：保持 Object 类型，但 Object 实现 Into<T> via downcast
    │
    ├── ❌ BFS 方法级扫描（减少 jdk_classes 类数量）
    │       现状：BFS 按类展开，150 个类上限可能不够
    │       目标：只扫描调用链上的方法，未调用方法的依赖类不纳入
    │
    ├── ❌ 单 crate 架构（最终目标态）
    │       现状：三层 crate（java_runtime / jdk_classes / user）
    │       目标：合并为单 crate（消除循环依赖问题，简化构建）
    │       前提：所有手写 runtime 完成迁移
    │
    └── ❌ _JAVA_RUNTIME_SHORT_NAMES / _COLL_IR_TYPES 清理
            当对应类完成迁移后，从 instr.py 移除相关硬编码
```

---

## 当前推进状态（2026-09-13）

### 已完成轮次

| 轮次 | 内容 | 提交 |
|------|------|------|
| P1 | loop 变量先 assign 后 let | 早期提交 |
| P2 | Object 类型接收方、BFS/存根修复 | 早期提交 |
| P3 | native_impls 基础设施建立 | 早期提交 |
| P4 | PrintStream + System 迁移、mod _native include! 机制 | e61b651 |
| P5-a | Object 模板缺失方法、bool 条件类型错误、_TRANSLATE_BODIES 清理 | 984be02 |
| P5-b | String 迁移（@synthetic 机制 + jdk_classes String）| b43669d |
| P8 | Math 迁移（native_impls 实现 30+ 方法）| 89e1cab |
| P6 | 数组类型 Vec<T> → Rc<RefCell<Vec<T>>>，引用语义修复 | c667875 |
| P7 | ArrayList/HashMap/HashSet 迁移，impl Into<Object> for String | 548df61 |

### 当前状态（P7 完成后）

**已迁移到 jdk_classes + native_impls：**
- System、PrintStream（P4）
- String（P5）
- Math（P8）
- ArrayList、HashMap、HashSet（P7）

**仍在 java_runtime（永久保留基础设施）：**
- Object（lock/unlock/hashCode/equals 是 VM 基础设施）
- error.rs（JvmError/Result）
- types.rs（Field<T>）

### 下一步（按优先级）

**P9：消除 Python 代码中的 JDK 类名常量（架构规范要求）**
```
files: scripts/codegen/instr.py
修改：_COLL_IR_TYPES 硬编码（ArrayList/HashMap/HashSet new 的初始化表达式）
      _JAVA_RUNTIME_SHORT_NAMES 硬编码（Object/Math/ArrayList 等）
      StringBuilder 特殊分支（映射到 String 类型）
目标：instr.py 中不出现任何 JDK 类名字面量
```

**P10：单 crate 架构（最终目标态）**
```
前提：所有手写 runtime 迁移完成（P7/P8 后满足）
目标：合并 java_runtime/jdk_classes/user 为单 crate
```

---

## 各层依赖关系

```
第 1-4 层（基础设施）✅
    ↓
第 5-6 层（指令翻译）✅（大部分，剩余集合类型擦除问题）
    ↓
第 7 层（native_impls 链接）✅
    ↓
第 8 层（_JAVA_RUNTIME_CLASSES 迁移）
    ├── PrintStream ✅（P4）→ System ✅（P4）→ String ✅（P5）
    ├── Math ✅（P8）
    └── ArrayList/HashMap/HashSet ✅（P7，@synthetic native_impls 方案）
        ↓
第 9 层（runtime 清理）✅（所有临时实现已清除，只保留 VM 基础设施）
    ↓
第 10 层当前状态：HelloWorld 运行正确，所有 JDK 类使用字节码翻译 + native_impls
    ↓
第 11 层（代码质量：消除硬编码类名 → 单 crate → 完全目标）
```
