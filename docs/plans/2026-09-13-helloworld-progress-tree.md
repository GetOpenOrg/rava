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
│           jdk_classes / user 的 src/ gitignored（代码生成产物）
│           Cargo.toml / build.rs 由 git 直接管理，emitter 不再写入
│           native_impls/ 在 workspace 根，提交到 git
│
├── 第 2 层：用户类翻译（HelloWorld.java → hello_world.rs）
│   ├── ✅ 类结构生成（struct + impl）
│   ├── ✅ 字段生成（JField<T> 封装）
│   │       注：Field<T> 已重命名为 JField<T>，避免与 java/lang/reflect/Field 命名冲突
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
├── 第 3 层：JDK 类发现（方法级调用链 BFS）
│   ├── ✅ 从用户类指令注释提取 JDK 方法引用（class + method + descriptor）
│   ├── ✅ 方法级调用链 BFS（_discover_jdk_classes_method_level）
│   │       只追踪实际被调用的方法，未调用方法的依赖类不展开
│   │       HelloWorld 自然收敛到 879 个 JDK 类，无需任何人工截断
│   ├── ✅ 截断规则已删除（_CUTOFF_CLASSES / _CUTOFF_PREFIXES / _CUTOFF_EXTRA_CLASSES）
│   │       原因：这些规则是类级 BFS 爆炸的补丁（类级无截断→3129 类）
│   │       解决：方法级 BFS 自然收敛，无需人工黑名单
│   └── ✅ 手写运行时排除（_JAVA_RUNTIME_CLASSES）
│           仅保留 java/lang/Object（VM 基础设施，永久由 java_runtime 提供）
│
├── 第 4 层：jdk_classes 生成（879 个 JDK 类的 Rust 存根）
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
│   ├── ✅ 公开静态字段 getter 方法生成（System::out()）
│   │       原因：getstatic 渲染为 System::out() 调用，但 jdk_classes System 无此方法
│   │       解决：emitter 扫描 public static 字段，为每个生成 pub fn field_name() 存根
│   │             若 native_impls 有对应实现则 dispatch 到 _native::field_name()
│   └── ✅ 编译警告全部消除
│           non_camel_case_types（内部类 $ → _ 后名称不符合 CamelCase）
│           unused_imports（glob import 引入未用类型）
│           ambiguous_glob_reexports（多子模块导出同名类型）
│           → 生成文件头 #![allow(...)] 覆盖所有情况
│
├── 第 5 层：invokevirtual / invokestatic 指令翻译
│   ├── ✅ invokevirtual 统一路径（obj.method(args)?）
│   ├── ✅ invokestatic 通用解析
│   ├── ✅ invokespecial（构造器 + super 调用）
│   ├── ✅ StringBuilder 迁移（P10）
│   │       StringBuilder 从 java_runtime 迁移到 jdk_classes + native_impls
│   │       /// @field 机制：native_impls 向生成 struct 注入额外字段（_sb）
│   │       instr.py 中 StringBuilder/StringBuffer 字面量已全部删除
│   ├── ✅ _COLL_IR_TYPES 硬编码删除（P9）
│   │       invokespecial ClassName.<init>:()V 改为动态逻辑
│   │       '/' in full_cls 判断 JDK 类 → 生成 ClassName::new_default()
│   └── ✅ _JAVA_RUNTIME_SHORT_NAMES 清理完成
│           仅保留 Object（java_runtime 永久基础设施）
│           已移除：String / Math / ArrayList / HashMap / HashSet / StringBuilder
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
│   ├── ✅ 内部类引用生成 panic! 存根
│   │       原因：Objects.checkIndex 调用 jdk.internal.util.Preconditions 等内部类
│   │       解决：instr.py _class_known() 检测，未知类生成 panic! 存根
│   │
│   ├── ✅ aconst_null 指令推送 Object::default() 到操作数栈
│   ├── ✅ 泛型类型参数推断（Comparator → Comparator<Object>）
│   ├── ✅ bool 返回类型转换（iconst_1/0 → true/false）
│   ├── ✅ bool vs i32 比较（ifne/ifeq 条件区分 bool/int 操作数类型）
│   ├── ✅ Object 缺少 null 语义方法（is_none/get）
│   ├── ✅ Object 缺少 Display trait
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
│   │       ✅ native_impls/java/lang/string_builder.rs（new_default/append__str/toString）
│   │       ✅ native_impls/java/lang/math.rs（sin/cos/sqrt/pow/log 等 30+ 方法）
│   │       ✅ native_impls/java/lang/double.rs / float.rs / throwable.rs 等
│   │       ✅ native_impls/java/util/array_list.rs / hash_map.rs / hash_set.rs
│   │
│   ├── ✅ mod _native { include!(...) } 链接机制
│   │       解决：emitter 在有 native_impls 的类文件中插入
│   │             mod _native { use java_runtime::prelude::*; use super::*; include!("../..../file.rs"); }
│   │             方法 stub 改为 _native::fn_name(args) dispatch
│   │       include! 路径公式：(pkg_depth + 2) 个 "../" + "native_impls/pkg/class.rs"
│   │
│   ├── ✅ /// @synthetic 合成方法机制
│   │       native_impls 文件中用 /// @synthetic 标注的 pub fn，
│   │       emitter 在生成的 impl 块中自动生成 wrapper 方法
│   │
│   ├── ✅ /// @field name: RustType 字段注入机制
│   │       native_impls 向生成 struct 注入额外字段（如 StringBuilder._sb）
│   │
│   └── ✅ build.rs native_status.toml 追踪
│           所有 native 方法实现状态：implemented / needed / stub / not-needed
│
├── 第 8 层：_JAVA_RUNTIME_CLASSES 迁移（手写 → 字节码翻译）
│   │       ← HelloWorld 目标架构的核心工作
│   │
│   ├── ✅ java/io/PrintStream 迁移（P4）
│   ├── ✅ java/lang/System 迁移（P4）
│   ├── ✅ java/lang/String 迁移（P5）
│   ├── ✅ java/util/ArrayList / HashMap / HashSet 迁移（P7）
│   │       impl Into<Object> for String 解决 JVM 引用协变问题
│   ├── ✅ java/lang/Math 迁移（P8）
│   └── ✅ java/lang/StringBuilder 迁移（P10）
│           /// @field 机制注入 _sb 字段，native_impls 完整实现
│           保留：Object 在 java_runtime（VM 基础设施，永久保留）
│
├── 第 9 层：代码架构清理
│   ├── ✅ runtime.py 删除（P11）
│   │       java_runtime/src/ 文件由 git 直接管理，不再由转译器写入
│   ├── ✅ Cargo.toml / build.rs 由 git 管理（P12）
│   │       .gitignore 改为只忽略 jdk_classes/src/ 和 user/src/
│   │       output/Cargo.toml、java_runtime/Cargo.toml、jdk_classes/Cargo.toml、
│   │       user/Cargo.toml、jdk_classes/build.rs 全部直接提交到 git
│   │       emitter.py 删除所有静态文件常量（-297 行）
│   ├── ✅ JField<T> 重命名（消除 reflect.Field 命名冲突）
│   │       java_runtime::types::Field → JField，删除 emitter is_named_field 特判
│   ├── ✅ 废弃脚本清理
│   │       删除：scripts/javalang/（旧 AST 解析器）、classify.py（无调用方）
│   │             gen_manifest.py、gen_stubs.py、java_rta.py、javap.py
│   │             analyze_callchain.py（验证完成）
│   └── ✅ Python 代码中 JDK 类名字面量清零
│           删除 _COLL_IR_TYPES、StringBuilder 特判、Math 特判
│           instr.py 中活跃代码零 JDK 类名字面量
│
├── 第 10 层：当前验收状态
│   ├── ✅ cargo check 0 errors，0 code-quality warnings
│   ├── ✅ HelloWorld 输出 "Hello, World / hahaha / 2"
│   ├── ✅ python3 scripts/main.py 默认转译并运行（无参数即完整流程）
│   ├── ✅ System / PrintStream / String / Math / ArrayList / HashMap / HashSet / StringBuilder
│   │       全部来自 JDK 字节码翻译 + native_impls
│   ├── ✅ BFS 方法级调用链（879 类，自然收敛，无截断规则）
│   └── ⚠️ Object → java_runtime（VM 基础设施，lock/unlock/hashCode 永久保留）
│
└── 第 11 层：后续架构工作（达到完全目标）
    │
    ├── ❌ 泛型类型推断（集合 get() 返回 Object 问题）
    │       问题：ArrayList<E>.get() 字节码返回 Object，
    │             checkcast 后才知道真实类型，但 Rust 不支持运行时类型强制
    │       方案 A：翻译时追踪 checkcast 指令，将变量类型 narrow 到 cast 目标类型
    │       方案 B：保持 Object 类型，但 Object 实现 Into<T> via downcast
    │
    └── ❌ 单 crate 架构（最终目标态）
            现状：三层 crate（java_runtime / jdk_classes / user）
            目标：合并为单 crate（消除循环依赖问题，简化构建）
            前提：所有手写 runtime 完成迁移（已满足）
```

---

## 推进历史

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
| P9 | 消除 instr.py JDK 类名硬编码（_COLL_IR_TYPES / StringBuilder）| 9d5346e |
| P10 | StringBuilder 迁移，/// @field 机制，instr.py 零类名字面量 | 72c5f48 |
| P11 | runtime.py 固化删除，java_runtime 由 git 直接管理 | 02b9128 |
| P12 | Cargo.toml/build.rs 由 git 管理，gitignore 只忽略 src/ | df35729 |
| — | 方法级 BFS，删除截断规则（879 类自然收敛）| ed8818c |
| — | JField<T> 重命名，消除 reflect.Field 命名冲突 | e3adee8 |
| — | 废弃脚本清理（javalang/classify/gen_manifest 等）| 55697dc |
| — | main.py 默认运行 HelloWorld，消除所有编译警告 | ae34a89 |

---

## 当前状态

**java_runtime 只保留 VM 基础设施（永久）：**
- `error.rs`（JvmError / Result）
- `types.rs`（JField<T>）
- `java/lang/object.rs`（Object，lock/unlock/hashCode 等 VM 语义）

**所有 JDK 标准库类来自字节码翻译 + native_impls：**
System、PrintStream、String、Math、ArrayList、HashMap、HashSet、StringBuilder

**转译器产物完全由 git 管理：**
- `java_runtime/`、`native_impls/`、`Cargo.toml`×4、`jdk_classes/build.rs` 提交到 git
- 仅 `jdk_classes/src/` 和 `user/src/` gitignored（生成产物）

## 各层依赖关系

```
第 1-4 层（基础设施）✅
    ↓
第 5-6 层（指令翻译）✅（剩余：泛型集合类型擦除）
    ↓
第 7 层（native_impls 链接）✅
    ↓
第 8 层（全部 JDK 标准库迁移）✅
    ↓
第 9 层（代码架构清理）✅
    ↓
第 10 层：HelloWorld 完全运行在 JDK 字节码翻译 + native_impls ✅
    ↓
第 11 层（泛型推断 → 单 crate → 完全目标）❌
```
