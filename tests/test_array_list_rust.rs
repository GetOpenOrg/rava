// Rust API 对比文档：当前写法 vs 最终目标写法
// 对应 Java 文件：tests/TestArrayList.java
//
// 说明：
//   - 「当前写法」是现在可编译运行的代码（已在 output/ 验证输出与 Java 完全一致）
//   - 「目标写法」是 T37/T38/T39/T40 完成后的最终 ergonomic API 写法
//   - 🔒 不可消除的差异：String::from("...")、? 操作符（Rust 语言本质，无法用宏消除）
//
// 运行方式（当前写法已集成在 output/ workspace）：
//   cd output && cargo run --release
//
// ══════════════════════════════════════════════════════════════════════
// Java                              │ Rust（当前）        │ Rust（目标，T37-T40 后）
// ══════════════════════════════════════════════════════════════════════

// ── 1. ArrayList<String> 基本操作 ─────────────────────────────────────

// Java:
//   ArrayList<String> names = new ArrayList<>();
//
// Rust（当前）：
//   let names = ArrayList::<Object>::new_default()?;
//
// Rust（目标，T37）：
//   let names: ArrayList<String> = ArrayList::new()?;  // ← new_default→new, 类型参数

// ─────────────────────────────────────────────────────────────────────

// Java:
//   names.add("Alice");
//
// Rust（当前）：
//   names.add__obj(String::from("Alice").into())?;
//
// Rust（目标，T37+T39）：
//   names.add(String::from("Alice"))?;   // ← add__obj→add，无需 .into()
//   // 🔒 String::from("...")  不可消除（Java 字面量 vs Rust 构造）

// ─────────────────────────────────────────────────────────────────────

// Java:
//   System.out.println(names.size());
//
// Rust（当前）：
//   System::out().println__i(names.size()?)?;
//
// Rust（目标，T38）：
//   System::out().println(names.size()?)?;   // ← 统一 println<T: Printable>

// ─────────────────────────────────────────────────────────────────────

// Java:
//   System.out.println(names.get(0));
//
// Rust（当前）：
//   System::out().println__obj(names.get(0i32)?)?;
//
// Rust（目标，T37+T38+T39）：
//   let s: String = names.get_item(0)?;     // ← T39: get_item 返回 E（无需 downcast）
//   System::out().println(s)?;              // ← T38: 统一 println
//   // T37 完成后 get_item 改名为 get

// ══════════════════════════════════════════════════════════════════════

// ── 2. ArrayList<Integer>：autoboxing / unboxing ───────────────────────

// Java:
//   ArrayList<Integer> scores = new ArrayList<>();
//   scores.add(100);                 // autoboxing: int → Integer
//   int first = scores.get(0);       // unboxing: Integer → int
//
// Rust（当前）：
//   let scores = ArrayList::<Object>::new_default()?;
//   scores.add__obj(100i32.into())?;                      // 手动 .into()
//   let first: i32 = scores.get(0i32)?.downcast::<i32>(); // 手动 downcast
//
// Rust（目标，T37+T36+T39）：
//   let scores: ArrayList<i32> = ArrayList::new()?;
//   scores.add(100)?;                // ← 无需 .into()（T39 + i32: Into<Object>）
//   let first: i32 = scores.get_item(0)?;   // ← 无需 downcast（T39 + From<Object> for i32）
//   // T37 完成后 get_item 改名为 get

// ══════════════════════════════════════════════════════════════════════

// ── 3. HashMap<String, Integer> ────────────────────────────────────────

// Java:
//   HashMap<String, Integer> ages = new HashMap<>();
//   ages.put("Alice", 30);
//   int v = ages.get("Alice");
//   boolean has = ages.containsKey("Charlie");
//
// Rust（当前）：
//   let ages = HashMap::<Object, Object>::new_default()?;
//   ages.put(String::from("Alice").into(), 30i32.into())?;
//   System::out().println__obj(ages.get(String::from("Alice").into())?)?;
//   System::out().println__z(ages.containsKey(String::from("Charlie").into())?)?;
//
// Rust（目标，T37+T38+T39）：
//   let ages: HashMap<String, i32> = HashMap::new()?;
//   ages.put_kv(String::from("Alice"), 30)?;         // ← T39: put_kv，无需 .into()
//   let v: i32 = ages.get_value(String::from("Alice"))?;  // ← T39: get_value，无需 downcast
//   System::out().println(v)?;                       // ← T38: 统一 println
//   let has = ages.contains_key_e(String::from("Charlie"))?;  // ← T39
//   System::out().println(has)?;

// ══════════════════════════════════════════════════════════════════════

// ── 4. HashSet<String> 去重 ────────────────────────────────────────────

// Java:
//   HashSet<String> unique = new HashSet<>();
//   unique.add("apple");
//   System.out.println(unique.size());
//   System.out.println(unique.contains("apple"));
//
// Rust（当前）：
//   let unique = HashSet::<Object>::new_default()?;
//   unique.add(String::from("apple").into())?;
//   System::out().println__i(unique.size()?)?;
//   System::out().println__z(unique.contains(String::from("apple").into())?)?;
//
// Rust（目标，T37+T38+T39）：
//   let unique: HashSet<String> = HashSet::new()?;
//   unique.add_e(String::from("apple"))?;            // ← T39: add_e，无需 .into()
//   System::out().println(unique.size()?)?;          // ← T38: 统一 println
//   System::out().println(unique.contains_e(String::from("apple"))?)?; // ← T39

// ══════════════════════════════════════════════════════════════════════

// ── 5. 不可消除的 Java vs Rust 差异（语言本质）─────────────────────────
//
// 完成 T37-T40 后，仍存在的差异（无法用宏消除）：
//
// | Java              | Rust                    | 原因                        |
// |-------------------|-------------------------|-----------------------------|
// | "Alice"           | String::from("Alice")   | Rust 有自己的 &str/String   |
// | list.add("Alice") | list.add(String::from("Alice"))? | 构造 + ?        |
// | String s = ...    | let s: String = ...     | 类型声明语法差异            |
// | (无需处理异常)    | ?                       | Rust 无 checked exception  |
// | new Foo()         | Foo::new()?             | 构造函数语法 + ?            |

// ══════════════════════════════════════════════════════════════════════
// 验证结果：
//   Java 输出（javac + java TestArrayList）：
//     3 / Alice / Bob / Charlie / 3 / 100 / 95 / 3 / 30 / 25 / true / false / 2 / true / false
//   Rust 输出（cargo run）：
//     3 / Alice / Bob / Charlie / 3 / 100 / 95 / 3 / 30 / 25 / true / false / 2 / true / false
//   ✅ 逐行一致
