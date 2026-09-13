# Java 风格 Ergonomic API：推进树

**日期**：2026-09-13（持续更新）  
**目标**：用户用 Rust 调用生成 API 时，代码写法与 Java 高度相似；差异通过宏/VM 层吸收，对用户透明

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

## 推进树

```
目标：Rust API 使用方式与 Java 高度相似
│
├── 第 1 层：类型转换基础设施（T35/T36）
│   │
│   ├── ✅ Boxing：primitive → Object（T35）
│   │       原因：Java 自动装箱，JVM 字节码传入 Object 参数时 i32/bool 等须包装
│   │       解决：impl From<i32/i64/f32/f64/bool/i8/i16/u16> for Object
│   │             _gen_invokevirtual 对所有传入 Object 的值插入 .into()
│   │
│   ├── ✅ Unboxing：Object → primitive（T36）
│   │       原因：ArrayList<i32>.get(0) 返回 Object，用户期望直接得到 i32
│   │             缺少 From<Object> for i32 导致无法做 let n: i32 = obj
│   │       解决：impl From<Object> for i32/i64/f32/f64/bool/i8/i16/u16
│   │             利用已有 Object::downcast::<T>() 方法实现
│   │
│   └── ✅ Object::PartialEq（T35）
│           原因：HashMap/HashSet 键比较需要 Object 值相等语义
│           解决：macro try_eq! 对每个 primitive 类型比较，回退到 Rc::ptr_eq
│
├── 第 2 层：集合 ergonomic 泛型方法（T39）
│   │       目标：list.add(e) 而非 list.add__obj(e.into())
│   │             list.get_item(i) -> E 而非 list.get(i) -> Object + downcast
│   │
│   ├── ✅ emitter.py _ergonomic.rs 自动 include 机制
│   │       原因：ergonomic 方法需要更强的泛型约束（E: Into<Object> + From<Object>）
│   │             不能放在 mod _native（会被包在内部模块）
│   │       解决：emitter 检测 <class_snake>_ergonomic.rs，直接 include! 到生成文件顶层
│   │
│   ├── ✅ ArrayList ergonomic 方法（array_list_ergonomic.rs）
│   │       add(e: E) → add__obj(e.into())
│   │       get_item(i: i32) -> Result<E>（避免与 JVM-exact get -> Object 冲突）
│   │       contains_e(e: E)、iter_typed() -> Vec<E>
│   │
│   ├── ✅ HashMap ergonomic 方法（hash_map_ergonomic.rs）
│   │       put_kv(k: K, v: V)、get_value(k: K) -> Result<V>
│   │       contains_key_e(k: K)
│   │
│   ├── ✅ HashSet ergonomic 方法（hash_set_ergonomic.rs）
│   │       add_e(e: E)、contains_e(e: E)
│   │
│   └── ❌ 方法名冲突（T37 解决）
│           现状：get_item vs Java get / add vs add__obj 已解决，但
│                 put_kv vs put / get_value vs get 仍有歧义名称
│           原因：JVM-exact 方法占用了 put/get 原始名称
│           解决方案：T37 将 JVM-exact 方法重命名为 put_obj/get_obj
│
├── 第 3 层：println 统一派发（T38）
│   │       目标：System::out().println(42) 而非 println__i / println__str / println__obj
│   │
│   ├── ✅ Printable trait 定义（java_runtime/src/lib.rs）
│   │       impl Printable for i32/i64/f32/f64/bool/i8/i16/u16/Object
│   │       Printable 导出到 prelude
│   │
│   ├── ✅ impl Printable for String（print_stream.rs，jdk_classes 上下文）
│   │       原因：String 类型在 jdk_classes，无法在 java_runtime 直接 impl
│   │       解决：在 native_impls/java/io/print_stream.rs 的 mod _native 中定义
│   │
│   └── ❌ instr.py 生成统一 println(x) 调用（T38 完整版）
│           现状：生成代码仍是 println__i / println__str / println__obj 分散调用
│           需要：instr.py 中所有 invokevirtual println:* 统一生成 println(x)
│           前提：PrintStream 添加 pub fn println<T: Printable>(&self, v: T)
│
├── 第 4 层：方法名去 mangle（T37）
│   │       目标：list.add__obj(x) → list.add(x)，new_default() → new()
│   │
│   ├── ❌ add__obj → add（最短参数重载保留原名）
│   │       规则：每组重载中参数最少的版本保留 Java 原始名
│   │             其余重载保留 mangle 名（add__i_obj 等）
│   │
│   ├── ❌ new_default() → new()
│   │       原因：Java <init>() 在 Rust 中占用了 new 名称（编译器自动命名）
│   │             @synthetic 工厂函数命名为 new_default 以避免冲突
│   │       方案：Java 构造器 stub 改名 __init() 或 _ctor()，释放 new 给工厂函数
│   │
│   └── ❌ JVM-exact 方法加 _obj 后缀（为 ergonomic 方法让路）
│           目标：put_obj(Object, Object) + put_kv(K, V) 共存
│                 get_obj(i32) -> Object + get_item(i32) -> E 共存
│
├── 第 5 层：用户自定义类集成（T40）
│   │       目标：用户自定义 Rust struct 加 #[java_class] 后可放入 ArrayList<MyStruct>
│   │
│   ├── 🔜 java_rta_macros proc-macro crate
│   │       展开内容：impl Into<Object> + From<Object> + Display + Debug
│   │
│   ├── 🔜 替换 cfg_attr(any(), java_class(...)) → #[java_class(...)]
│   │       现状：生成代码的 java_class 属性是死属性（cfg(any()) 永远 false）
│   │       目标：真实 proc-macro 激活，自动生成样板代码
│   │
│   └── 🔜 用户侧验证
│           目标代码：
│           #[java_class(binary_name = "com/example/Person")]
│           struct Person { pub name: JField<String> }
│           let list: ArrayList<Person> = ArrayList::new()?;
│           list.add(Person::default())?;
│           let p: Person = list.get(0)?;
│
└── 第 6 层：当前验收状态
    ├── ✅ TestArrayList.java 编译并运行正确
    ├── ✅ 生成 Rust 代码（user/src/test_array_list.rs）输出与 Java 完全一致（14 行）
    ├── ✅ cargo check 0 errors（含 ergonomic impl 块）
    ├── ⚠️ for-each 增强循环代码生成（T42 新发现）
    │       原因：for-each 编译为 iterator() + hasNext() + next() 调用序列
    │             JVM 局部变量 slot 复用（iterator 变量与后续 HashMap 共用 slot 3）
    │             导致 Rust 类型冲突：let ages: Iterator<Object> = ...; ages = HashMap...;
    │       临时处理：TestArrayList.java 注释掉 for-each 段落（待 T42 修复）
    ├── ⚠️ 方法名仍为 mangle 形式（T37 待完成）
    │       当前：list.add__obj(x.into())?
    │       目标：list.add(x)?
    └── ⚠️ println 仍分散调用（T38 完整版待完成）
            当前：System::out().println__i(n)?
            目标：System::out().println(n)?
```

---

## 当前 API 对比状态

| Java 写法 | 当前 Rust 写法 | 目标 Rust 写法 | 状态 | 解决任务 |
|-----------|---------------|---------------|------|---------|
| `new ArrayList<>()` | `ArrayList::<Object>::new_default()?` | `ArrayList::<String>::new()?` | ⚠️ | T37 |
| `list.add("Alice")` | `list.add__obj(String::from("Alice").into())?` | `list.add(String::from("Alice"))?` | ⚠️ | T37+T39 |
| `String s = list.get(0)` | `list.get(0)?.downcast::<String>()` | `let s: String = list.get_item(0)?` | ⚠️ | T39✅ 名称待 T37 |
| `int n = scores.get(0)` | `scores.get(0)?.downcast::<i32>()` | `let n: i32 = scores.get_item(0)?` | ⚠️ | T36✅ T39✅ |
| `map.put("k", 42)` | `map.put("k".into(), 42i32.into())?` | `map.put_kv(s, 42)?` | ⚠️ | T39✅ 名称待 T37 |
| `map.get("k")` | `map.get("k".into())?` 返回 Object | `let v: i32 = map.get_value(s)?` | ⚠️ | T39✅ 名称待 T37 |
| `set.add("x")` | `set.add("x".into())?` | `set.add_e(s)?` | ⚠️ | T39✅ 名称待 T37 |
| `System.out.println(42)` | `System::out().println__i(42)?` | `System::out().println(42)?` | ⚠️ | T38 |
| `for (S s : list)` | 编译报错（T42）| `for s in list.iter_typed()` | ❌ | T42 |
| `new Person(...)` + `list.add(p)` | 需手动 `.into()` | `list.add(p)?` 无需转换 | 🔜 | T40 |

**整体进展：** 类型转换基础设施 ✅ → 集合 ergonomic 方法 ✅ → 方法名/println ⚠️ → proc-macro 🔜

---

## 下一步优先级

```
立即可做（无依赖）：
  T37-a  new_default() → new()：@synthetic 标注 + emitter @synthetic 名称逻辑
  T38-b  PrintStream::println<T: Printable> native_impl + instr.py 统一生成

中优先（依赖 T37-a 完成）：
  T37-b  add__obj → add 去 mangle（emitter overloaded_names 最短版本保留原名）

长期（依赖 T37 全部完成）：
  T40    java_rta_macros proc-macro crate

独立修复（可随时做）：
  T42    for-each 增强循环 slot 复用 bug（instr.py / method.py 局部变量重声明逻辑）
```

---

## 推进历史

| 提交 | 内容 |
|------|------|
| T35 (`2f692f1`) | Boxing（From<primitive> for Object）、HashMap/HashSet native 实现、PartialEq |
| T36 (本 session) | Unboxing（From<Object> for primitive）、object.rs |
| T38-基础 (本 session) | Printable trait 定义、impl for String（print_stream.rs）|
| T39 (本 session) | ergonomic _rs 机制（emitter.py）、array_list/hash_map/hash_set ergonomic 文件 |
| 测试 (本 session) | TestArrayList.java 14 行输出与 Rust 生成代码完全一致验证 |
