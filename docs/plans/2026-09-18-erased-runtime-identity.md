# 泛型擦除 vs 单态化：运行时类型标识终态方案

> 日期：2026-09-18
> 关联：`2026-09-17-java-rust-type-1to1.md`（T-2 接口泛型透明）、`2026-09-16-java-class-macro-unified.md`、`java-rust-translation-reference.md` §16

## 1 问题

Java 泛型在运行时被擦除：`ArrayList<String>` 与 `ArrayList<Object>` 是同一个运行时类。
生成的 Rust 是单态化的：`X<String>` 与 `X<Object>` 的 `TypeId` 不同。
现状中凡是「运行时判定类型」的位置都依赖带类型实参的精确 `TypeId`（`as_any().downcast_ref::<X<Args>>()`），后果：

1. 接口/虚方法分派链落空。for-each 生成 `downcast_ref::<ArrayList_Itr<Object>>()`，运行时对象是 `ArrayList_Itr<String>`，`hasNext` 落到默认 `false`，循环体 0 次执行。
2. `_reinstantiate_generic` 走 `Object::from_any(..).downcast::<X<T>>()`，实参不同即 ClassCastException。
3. checkcast / instanceof 对泛型类有同样的问题。
4. 这些调用出现在可读层，违反 §16。

## 2 终态指标

| 指标 | 终态值 |
|------|--------|
| 依赖「带类型实参 TypeId」的运行时判定 | 0 |
| instanceof / checkcast / 虚分派 / 接口分派的判定依据 | 仅擦除后的类（与 JVM 一致） |
| 可读层 `downcast_ref` / `as_any` / `Object::from_any` / `.vtable` / `.downcast::<` | 0 |
| 接口调用的可读形态 | `it.hasNext()` / `it.next()` |

## 3 候选方案与论证

### (a) 运行时 `__inner` 完全擦除，类型实参只存在于 wrapper 的 PhantomData

- 字段存储：类型变量位置的字段统一存 `Object`，访问器在 wrapper 层按 `From<Object>` / `Into<Object>` 转换。
- 运行时标识：`X__inner` 非泛型，`TypeId` 天然与类型实参无关；`From<Object> for X<A>` 对任意 `A` 成立。
- 代价：宏需要对 struct 字段、vtable trait、所有方法体所在的 impl 做擦除/重建，属于对 `java_class!` 的整体改造；方法体目前运行在 `__inner<Args>` 的上下文里，需整体迁移到 wrapper 上下文。

### (b) 不带类型实参的类标识 + 对象安全的擦除 vtable 查询

- 每个接口一个**非泛型**的 `I__VTable: 'static`（擦除签名），对象通过 `ObjectVTable::__interface` 按接口回答「我的 itable 项」。
- 判定依据是「对象是否实现该接口」，与类型实参无关——这正是 JVM 的 itable 语义。
- 类型实参只在载体 `I<E>` 的方法签名里出现，用 `Into`/`From` 在边界转换（等价 javac 的桥接方法 + checkcast）。

### (c) 全局类型注册表（`TypeId → 擦除类名`）+ unsafe 重解释

否决：需要 unsafe transmute 在不同单态化布局之间转换，布局并不保证一致（类型变量位置存的是具体类型而非 `Object`）。

### 结论：(b) 作为分派层终态，(a) 作为存储层终态，两者正交、分两阶段落地

- **分派**（问题 1、4）只需要 (b)：调用方永远不需要知道对象的具体类，也就不需要任何 TypeId。
- **类身份转换**（问题 2、3 中 `Object → X<A>`）需要 (a)：只要 `__inner` 仍带类型实参，`Object → X<A>` 就必须命中精确 TypeId。
- (b) 先行的理由：它独立可落地，形态已是终态（阶段 2 不会改动接口 vtable / 载体 / `__interface` 的任何形状），且立刻修复最大面积的运行时错误（所有 invokeinterface）。

## 4 影响评估

| 维度 | 评估 |
|------|------|
| 泛型字段存储 | 阶段 1 不变；阶段 2 类型变量字段存 `Object`，访问器在 wrapper 层转换 |
| 基本类型实参 | Java 泛型实参恒为引用类型，Rust 侧的 `i32` 等经 `From<i32> for Object` / `From<Object> for i32` 参与同一套边界转换，无特例 |
| 性能 | 接口调用 = 一次 `__interface` 虚调用（`TypeId` 比较 + `Rc` 克隆）+ 一次 itable 虚调用，与 JVM invokeinterface 同阶；替换掉的是 O(实现类数) 的 downcast 链 |
| 宏复杂度 | 新增 `expand_interface` 的 vtable/载体方法与 `expand_interface_impl`；类侧只多一个 `__interface` 查询。签名擦除是纯语法变换（类型形参出现处 → `Object`） |
| 0 编译错误状态 | 三个验证测试保持 0 错误 |
| T-2 兼容 | 载体 `I<E>` 即 T-2 目标中的 `Iterator<E>` 类型；阶段 2 把它放进签名/局部变量类型位置后，调用点自然成为 `it.hasNext()` |

## 5 阶段 1（本轮已落地）

### 5.1 运行时 / 宏

- `ObjectVTable::__interface(self: Rc<Self>, slot: &mut dyn Any)`：调用方传入 `Option<Rc<dyn I__VTable>>` 槽，对象若实现该接口则填入。
- 接口块（`java_class!` 接口形态）展开：
  - 非泛型 `I__VTable: 'static`，方法为擦除签名，默认体 `panic!("stub: ...")`；
  - 载体 `I<E> { __ref: Object, PhantomData }`，`From<Object>` / `Into<Object>` / `Deref<Target = Object>`；
  - 载体实例方法：查询 `__interface` → 调 itable；函数式接口（恰一个抽象方法）回落到闭包；否则 `AbstractMethodError`；
  - `#[java_method(inherited_from = "Owner<args>")]` 声明：父接口方法在子接口载体上可见，展开为向 owner 载体的向上转型后调用。
- 类块：`impl<E> Iface for Class<E> { 擦除签名声明; }` → 宏生成 `impl Iface__VTable for Class__inner`（重建 wrapper 后调同名/`target` 成员，保持多态）与 `__interface` 查询；wrapper 转发到 `self.vtable`。
- 类的类型形参补 `From<Object> + Into<Object>` bound。
- `JArray<T>`：`ObjectVTable` 与 `From<Object>`。

### 5.2 codegen

- `codegen/emitter/interface_gen.py`（新）：解析每个具体类的全部接口方法 → 本类成员 / `_impl.rs` 提供的成员 / 祖先类成员（登记继承成员请求）/ 桥接方法真实描述符；输出 `impl Iface for Class` 声明与 `use`。接口间继承成员声明同在此生成。
- 接口的实例方法（抽象与 default）进入接口块，不再被丢弃。
- `invokeinterface` 在 `Object` 接收者上：`Into::<I<Object>>::into(Clone::clone(&obj)).m(args)?`，不再枚举实现类。
- 向上转型保持对象身份：类实例 `Object::from(x)`，类型变量 `Into::<Object>::into(x)`，基本类型 `.into()`；`Object::from_any` 仅用于闭包等无运行时类的值。
- `areturn` 处 `Object → 类型变量 / 类` 经 `From::from`（javac 的隐式 checkcast），不再返回 `Default::default()`。

### 5.3 验证结果

三个测试 `--clean` 生成后均 0 编译错误。TestArrayList 的 for-each 打印 Alice/Bob/Charlie（基线为 0 次迭代）。
可读层计数（仅统计 `java_class!` 生成文件，前 → 后）：

| 测试 | downcast_ref | .as_any() | Object::from_any | .downcast::< |
|------|------|------|------|------|
| test_array_list | 16 → 5 | 16 → 5 | 96 → 57 | 23 → 20 |
| test_collections | 12 → 5 | 12 → 5 | 85 → 51 | 18 → 15 |
| test_string_builder | 1630 → 126 | 1630 → 126 | 1568 → 469 | 237 → 153 |

## 6 阶段 2（后续步骤，全部以 §2 指标为验收）

1. **`__inner` 与类 vtable 擦除**（方案 a）：`X__inner`、`X__VTable` 去掉类型形参；类型变量字段存 `Object`；`From<Object> for X<A>` 改为向对象查询类 vtable（与 `__interface` 同机制，按擦除类），对任意 `A` 成立且支持向上转型目标。完成后删除 `_reinstantiate_generic`。
2. **checkcast 统一为 `Into::<T>::into(obj)`**：当前 `.downcast::<T>()` 字符串形态被 `stack.py`（hint 重定向）、`fields.py`、`invoke_sig.py`、`codegen.py` 共 15+ 处模式匹配依赖，且 `downcast(&self)` 与 `into(self)` 的所有权语义不同；本轮试改后 TestStringBuilder 出现 16 个错误，已回退。需在步骤 1 之后以 IR 节点（`CastExpr`）替代字符串形态一次性替换。
3. **instanceof 在「静态类型是目标祖先」时走运行时判定**：本轮试改后原先被静态 `false` 屏蔽的分支变活，暴露出子类值赋给父类局部变量缺少向上转型（`e = _t1`，TreeNode → Node）以及三元合并的类型不一致（位于控制流结构化区域，非本任务范围）。需与控制流改造合并后启用。
4. **移除 `invokevirtual` 在 `Object` 接收者上的类 downcast 链**（剩余 `downcast_ref` 的主要来源）：依赖步骤 1 的类 vtable 查询。
5. **T-2 载体进入类型位置**：接口类型的参数/返回值/局部变量/字段用 `I<E>` 而非 `Object`，调用点成为 `it.hasNext()`。
6. **lambda 对象实现 `I__VTable`**：替换载体里的闭包 `downcast_ref` 回落，使 default 方法可在 lambda 上调用。
7. **抽象类、枚举、手写类的接口实现**：抽象类当前跳过（由具体子类承担）；手写类与枚举尚无 `__interface`。
8. `Object::from_any` 剩余站点（闭包装箱、未知类型）随步骤 5、6 归零。
