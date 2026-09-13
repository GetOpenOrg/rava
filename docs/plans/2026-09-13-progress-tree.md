# HelloWorld 目标运行路径：问题树与推进层次

**日期**：2026-09-13  
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
      影响范围：...
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
│   │       排除 String/System/ArrayList 等，避免与 java_runtime 重复定义
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
│   └── ✅ $-命名的 .rs 文件（array_list$itr.rs 等）残留
│           解决：to_snake() 已有 name.replace('$', '_')，0 个残留文件已验证
│
├── 第 5 层：invokevirtual / invokestatic 指令翻译
│   ├── ✅ invokevirtual 统一路径（obj.method(args)?）
│   ├── ✅ invokestatic 通用解析
│   ├── ✅ invokespecial（构造器 + super 调用）
│   ├── ⚠️ StringBuilder.append / String.append 特殊分支（硬编码）
│   │       现状：instr.py 第 629-640 行对 StringBuilder/StringBuffer 做特判
│   │       问题：Python 代码中出现 JDK 类名（违反架构原则）
│   │       解决方案：Phase B（数据驱动分派）后通过接口映射自动处理
│   ├── ⚠️ _COLL_IR_TYPES 硬编码（ArrayList/HashMap/HashSet new）
│   │       现状：instr.py 第 76-81 行对集合类构造做特判
│   │       问题：Python 代码中出现 JDK 类名（违反架构原则）
│   │       解决方案：Phase B 后通过接口映射自动处理
│   └── ⚠️ _JAVA_RUNTIME_SHORT_NAMES 硬编码
│           现状：用于避免对手写 runtime 类的方法名做 mangle
│           问题：手写 runtime 消除后此集合也需删除
│           解决方案：Phase E 后删除
│
├── 第 6 层：JDK 方法体字节码翻译（stub_bodies=False）
│   │       ← 实验结果：全量开启触发 3808 个编译错误
│   │
│   ├── ❌ loop 变量先 assign 后 let（300+ 错误）
│   │       原因：JVM 循环中局部变量先被 istore 赋值（生成 AssignStmt），
│   │             后在 LocalVariableTable 里才有 let 声明（生成 LetStmt）
│   │             导致 Rust 看到变量在声明前被使用
│   │       示例：
│   │           i = 0i32;        // AssignStmt：i 未声明
│   │           loop {
│   │               let mut i: i32 = 0i32;  // LetStmt：声明在赋值之后
│   │           }
│   │       解决方案：method.py 在渲染前做"首次赋值提升"：
│   │                 如果 AssignStmt 的目标变量在当前作用域没有对应 LetStmt，
│   │                 将该 AssignStmt 替换为 LetStmt（mutable=True）
│   │
│   ├── ❌ Object 类型接收方调用具体方法（1071+ 错误）
│   │       原因：JVM 类型擦除后泛型集合元素类型为 Object，
│   │             但翻译后的 Rust 变量类型也是 Object，
│   │             而 Object struct 上没有 size/hasNext/next/compare 等方法
│   │       示例：
│   │           let _t0: Object = list.get(0)?;
│   │           _t0.hasNext()?;  // Object 上没有 hasNext
│   │       解决方案：
│   │             短期：泛型擦除处的 Object 类型用 RsInfer（_）替代，让 Rust 推断
│   │             长期：Phase B 数据驱动分派 + 接口映射
│   │
│   ├── ❌ 内部类引用（ArraysSupport 等）找不到（67+ 错误）
│   │       原因：Arrays.copyOf 内部调用 ArraysSupport，
│   │             而 ArraysSupport 被 BFS 截断规则排除，未生成代码
│   │       解决方案：对这类工具类保持 stub_bodies=True（永远 panic! 存根），
│   │                 不翻译字节码，从而不引入其依赖的类
│   │
│   └── ❌ 数组通过 & 引用赋值（39 错误）
│           原因：JVM 数组是引用语义，可通过不可变引用修改元素；
│                 Rust 数组切片 &[T] 不能赋值，需要 &mut [T]
│           解决方案：array 类型生成 Field<Vec<T>> 而不是 &[T]，
│                     或将接收 array 的方法参数改为 &mut Vec<T>
│
├── 第 7 层：_JAVA_RUNTIME_CLASSES 迁移（手写 → 字节码翻译）
│   │       ← 这是 HelloWorld 目标架构的核心障碍
│   │
│   ├── 🔜 java/lang/String 迁移
│   │       现状：手写 java_runtime::String（Rust 字符串封装）
│   │       JDK String 的 native 方法：charAt/length/indexOf 等约 20 个
│   │       迁移步骤：
│   │         1. 从 _JAVA_RUNTIME_CLASSES 移除 java/lang/String
│   │         2. BFS 扫描 String.class，生成带 panic! 存根的 Rust 代码
│   │         3. native_impls/java/lang/string.rs 实现 charAt/length 等
│   │         4. 删除 runtime.py 中 string.rs 内容
│   │
│   ├── 🔜 java/io/PrintStream 迁移
│   │       现状：手写 java_runtime::PrintStream（直接 println! 宏）
│   │       JDK PrintStream.println(String) 调用链：
│   │         println → print → textOut.print → BufferedWriter.write
│   │         → OutputStreamWriter.write → StreamEncoder.write → native
│   │       依赖的额外类：BufferedWriter / OutputStreamWriter / StreamEncoder
│   │       ← 这些类目前被 _CUTOFF_PREFIXES 的 sun/ 前缀截断
│   │       迁移步骤：
│   │         1. 扩展 BFS：允许 java/io/ 包（当前未截断，但 PrintStream 在排除集）
│   │         2. 翻译 PrintStream 字节码（方法有 bytecode，但依赖 BufferedWriter）
│   │         3. native_impls/java/io/print_stream.rs 实现最底层 write native
│   │         4. 或采用"短路"方案：PrintStream.println 的 native 实现直接调用 Rust println!
│   │
│   ├── 🔜 java/lang/System 迁移
│   │       现状：手写 java_runtime::System（System::out() 直接返回 PrintStream）
│   │       JDK System.out 是 JVM 初始化时注入的静态字段（setOut0 是 native）
│   │       迁移步骤：
│   │         1. 翻译 System.class（System.out 是 static final PrintStream 字段）
│   │         2. native_impls/java/lang/system.rs 实现 setOut0/setErr0/setIn0
│   │         3. 或"短路"：System::out() 的 native 实现直接返回预初始化的 PrintStream
│   │
│   ├── 🔜 java/util/ArrayList / HashMap / HashSet 迁移
│   │       现状：手写 java_runtime 集合类（Rc<RefCell<Vec<T>>>）
│   │       JDK 集合类的 native 方法极少，大部分是纯 Java 实现
│   │       迁移步骤：
│   │         1. 从 _JAVA_RUNTIME_CLASSES 移除，加入 BFS 扫描
│   │         2. stub_bodies=False 翻译字节码（需第 6 层错误先修复）
│   │         3. 少量 native 方法放入 native_impls/
│   │
│   └── 🔜 java/lang/Object / java/lang/Math 迁移
│           Object：hashCode/clone/wait/notify 均为 native，较难
│           Math：全部是 native，需要 native_impls/ 逐一实现
│
├── 第 8 层：native_impls/ 基础设施（Phase E）
│   ├── ⚠️ 目录结构建立
│   │       ✅ output/native_impls/java/lang/system.rs（currentTimeMillis/nanoTime/arraycopy）
│   │       ⚠️ output/native_impls/java/lang/string.rs（文件存在但实现为空）
│   │       ❌ output/native_impls/java/io/print_stream.rs（尚未建立）
│   │
│   ├── ⚠️ HelloWorld 最小 native 集合实现
│   │       ✅ System.currentTimeMillis → std::time::SystemTime
│   │       ✅ System.arraycopy → 标记 not-needed（HelloWorld 不调用）
│   │       ❌ PrintStream.write(byte[]) → 尚未实现
│   │       ❌ String.charAt / String.length → 尚未实现
│   │
│   └── ⚠️ build.rs 构建阻断
│           ✅ build.rs 已从模板生成，扫描 ../native_impls/，维护 ../native_status.toml
│           ❌ JAVA_RTA_STRICT=1 严格模式未默认开启（needed 方法只警告，不阻断编译）
│
├── 第 9 层：手写 runtime 清理（最终态）
│   ├── 🔜 删除 runtime.py 中所有 Rust 字符串内容
│   ├── 🔜 删除 java_runtime/src/java/ 子目录（手写 Java 类实现）
│   │       注：java_runtime/src/error.rs 和 types.rs 是 VM 基础设施，永久保留
│   ├── 🔜 删除 _JAVA_RUNTIME_CLASSES 排除集（不再需要）
│   ├── 🔜 删除 _JAVA_RUNTIME_SHORT_NAMES（不再需要 mangle 豁免）
│   └── 🔜 删除 _COLL_IR_TYPES 硬编码
│
└── 第 10 层：验收
        ✅ 当前：cargo check 0 errors，HelloWorld 输出 "Hello, World / hahaha / 2"
        🔜 目标：
            - output/Cargo.toml 无 java_runtime 依赖
            - output/src/java/lang/system.rs 存在且有真实实现
            - output/native_impls/ 存在
            - cargo run 输出 "Hello, World"
```

---

## 推进优先级

### 立即可做（解除第 6 层阻塞）

**P1：修复 loop 变量先 assign 后 let**

```
files: scripts/codegen/method.py
修改：在渲染 entries 之前扫描所有 AssignStmt，
      对目标变量没有对应 LetStmt 的情况，将 AssignStmt 提升为 LetStmt(mutable=True)
预期：消除约 300 个 E0425 错误
```

**P2：Object 类型接收方 → RsInfer 推断**

```
files: scripts/codegen/instr.py, type_map.py
修改：invokevirtual 时若接收方类型为 Object，
      改为 let _tN = ... （不写显式类型），让 Rust 从方法签名推断
预期：消除大量 E0308/E0599 错误
```

### 中期（第 7-8 层）

**P3：建立 native_impls/ 目录 + HelloWorld 最小实现**

```
新建：output/native_impls/java/lang/system.rs（currentTimeMillis, arraycopy）
      output/native_impls/java/io/print_stream.rs（write）
      output/native_impls/java/lang/string.rs（charAt, length, intern）
```

**P4：PrintStream 迁移（最小路径：短路 native）**

```
方案：PrintStream.println → native 实现直接调用 Rust println!
      不翻译 BufferedWriter/StreamEncoder 这条深依赖链
      移除 _JAVA_RUNTIME_CLASSES 中的 java/io/PrintStream
      jdk_classes 生成 PrintStream 的字节码翻译（方法有字节码部分翻译）
      native 方法由 native_impls/java/io/print_stream.rs 实现
```

**P5：System 迁移**

```
方案：System.out 由 native_impls 短路为直接返回 PrintStream 实例
      移除 _JAVA_RUNTIME_CLASSES 中的 java/lang/System
```

### 长期（第 9 层清理）

**P6：删除 java_runtime 手写实现**

```
当 P3-P5 完成，java_runtime 中的 String/System/PrintStream 实现已无用
逐步删除，替换为 jdk_classes + native_impls 的组合
```

---

## 各层依赖关系

```
P1（loop 变量）
    ↓
P2（Object 类型推断）
    ↓
第 6 层错误大幅减少 → stub_bodies=False 对大部分 JDK 类可用
    ↓
P3（native_impls 基础设施）
    ↓
P4（PrintStream 迁移）→ P5（System 迁移）→ P6（清理 java_runtime）
    ↓
目标架构达成：HelloWorld 完全运行在字节码翻译的 Rust 代码上
```
