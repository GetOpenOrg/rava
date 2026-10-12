# 反射强转同类风险核查：coerce 装箱文本与方法句柄 invoke 路径（tasks.md「剩余工作」第 10 条）

- 日期：2026-10-12；分支 `coerce-check`（基线 main 99de3268 + 测例提交）。
- 任务背景：fix-invoke-box-cast（84d9468f）修复了 `Method.invoke` 基本类型返回值未按返回描述符
  经包装类 `valueOf` 装箱的缺陷（`(Integer) mi.invoke(...)` 强转失败）。本步核查其同类风险面：
  ① `generator/crates/instr/src/coerce.rs` 的 `to_object_text`（checkcast / coerce 路径上基本值 →
  包装对象的装箱文本生成）；② 方法句柄（`MethodHandle.invoke` / `invokeWithArguments`）路径的
  入参与返回装箱。结论先行：**全部探针分支与 JVM 一致，无失配；见证测例入库**（本分支仅新增
  测例与文档，无代码改动）。

## 一、风险面分析（为什么本应可疑、为何无失配）

原生值盒（`From<i32> for Object` = `Object::__alloc`，`object.rs` `impl_from_primitive!`）只报
包装类名、不是翻译包装类实例——`try_cast::<Integer>` 的 `is_instance_of` 通过后视图构造失败，
是 fix-invoke-box-cast 的失配形态。本步把两处可疑面逐一落到代码路径：

1. **checkcast / coerce 路径（`coerce.rs` + `instr/sim/control.rs` checkcast 翻译）**：源级
   `(Integer) intExpr` / `(Object) intExpr` / 三元 / 比较产物装箱在字节码里 javac 已显式发
   `Integer.valueOf` 等工厂调用，进入 checkcast 翻译（`try_cast::<T>` / `box_first` 的
   `Object::from(src).try_cast` / `From<Object>` 视图）的值已是翻译包装类实例，视图构造成立。
   `to_object_text` Prim 分支的 `.into()` 原生盒只出现在转译器内部的擦除边界（块级合并值 /
   条件原子 / lambda 体文本拼接 / sigpoly 实参装箱），这些边界不与用户可见的 checkcast 直接
   相接。分支覆盖见 `TestCoerceBoxCast`。
2. **sigpoly 调用点（`instr/sim/methods.rs::gen_signature_polymorphic`）**：基本类型实参按
   `coerce::to_object` 装成原生盒传入 `invoke__site`——这是方法句柄解释器
   （`method_handle_ext.rs`：实参与返回值一律以 Object 流动）的**设计内内部载体**，与
   fix-invoke-box-cast 修复口径一致（原生盒只作 VM 内部擦除载体）。返回侧三种形态各得其所：
   - 调用点返回基本类型（`(I)I`）：`from_object_push` 走 `impl From<Object> for i32`（=
     `downcast::<i32>`），等型句柄经分派臂拿到原生盒、适配句柄经 JDK asType 字节码拆箱成原生盒，
     两种来源都命中；
   - 调用点返回引用（`(I)Ljava/lang/Integer;`）且句柄类型不同：`invoke__site` 先
     `asType(调用点类型)`，装箱由翻译的 JDK 组合子字节码（`ValueConversions.box*` → 翻译
     `valueOf`）完成，产出翻译包装类实例；
   - 等型引用返回：目标成员本身返回引用，直接是翻译对象。
   分支覆盖见 `TestMhBoxCast`。
3. **拒收面语义（JVM 侧实测校准后写入期望）**：
   - `invoke` 引用上下文不可转换（`(Long) intHandle.invoke(1)`：int 不可转 Long）在 JVM 是
     **asType 适配时抛 WrongMethodTypeException**，不是 CCE；
   - `invokeWithArguments` 返回 Object 后强转错误包装类是 **CCE**（装箱已完成，强转失败）；
   - `invokeExact` 调用点类型（含返回）必须与句柄类型完全一致，引用上下文套在基本返回句柄上
     即 WMTE。另见证一个 javac 细节：sigpoly 返回类型只在**显式强转上下文**细化，普通赋值上下文
     按擦除 Object 发描述符（`(I)Object` ≠ `(I)Integer` → WMTE），rava 与 JVM 一致。

## 二、见证测例（期望输出 JDK 21 本地生成，同 TestReflectBoxCast 口径）

- `tests/e2e/21_casting/TestCoerceBoxCast.java`：八种基本类型表达式 / 方法返回值直接
  `(Object)` / `(Integer)` / `(Double)` 等强转与隐式上转；byte/short/char 拓宽后装箱（种类随
  拓宽结果）与原种类装箱（`(Byte)` / `(Short)` / `(Character)`）；三元 / 比较产物装箱；
  `instanceof` / `getClass`；泛型边界取回；`Object[]` 变参与数组元素装箱；`Comparable` 视图；
  拒收面四种错误包装类强转 → ClassCastException。
- `tests/e2e/59_method_handles/TestMhBoxCast.java`：八种基本类型返回值 × 引用上下文
  （`(Integer)` / `(Number)` / `(Object)` 强转、`instanceof`、`getClass`）经 asType 装箱；
  基本类型上下文拆箱对照；子字实参（char / byte / short / boolean + int / long / float /
  double）`invoke` / `invokeExact` / `invokeWithArguments` 三入口；`invokeWithArguments`
  泛型适配链；getter / identity 句柄装箱边界；引用返回句柄 `invokeExact` 合法面；拒收面
  （`invoke` 不可转换 WMTE × 2、`invokeWithArguments` 强转 CCE、`invokeExact` 类型不符
  WMTE × 2）。

## 三、验证

- 集群抽查（spot ccx-new2，ref = coerce-check fb2825d85b5a，JDK 21，服务器 kr2 / sg2 / us1 /
  jp1，避开 dev）：15/15 全 PASS、0 失败——新增两例 + 回归族（TestReflectBoxCast、
  TestWrapperValueFaces / TestUnboxWrapperFaces / TestWidenFacesInvoke、TestMethodHandleDirect /
  Combinators / TestBmhDynamicSpecies / TestFieldHandleOffsets / TestMethodHandleStaticInit、
  TestAutoboxing / BooleanBoxingTest / BoxingEnhancedTest / TestAutoboxEdge）。逐例含转译 +
  构建 + 运行三段真实耗时（MH 族单例 268–299 s，见 spot/ccx-new2/timings_jdk21.tsv）。
  期间本地派发进程一次被环境回收，凭 state 租约原 tag 续跑无损（基础设施既有能力）。
- 本机 `cargo check --release --tests`（generator，经 heavy_lock）通过。本分支无运行时代码改动；
  java_runtime 的编译验证由 15 例端到端构建覆盖（裸检出不可 cargo check 为既有形态）。
- 未发现失配，无 failure_patterns 登记项。

## 四、残余说明

- 本核查是**探针级**结论：覆盖了任务书列出的全部分支（checkcast 装箱族、MH 入参 / 返回 /
  拒收面），但「转译器内部擦除边界不再外溢原生盒」这一设计不变量没有静态检查器兜底——若后续
  改动把 `to_object` / `to_object_text` 的 Prim 分支产物接到用户可见 checkcast，仍可能复发
  fix-invoke-box-cast 形态；两例测例即为此不变量的常驻回归 witness。
- 与 `unbox-root-virtual-stub`（batch-1012 748e0987，unbox 方向）无交互：本步测例全部直呼
  包装类方法（println 拼接 / `getClass` / `compareTo` 等直呼站点入链），未触达闭包外根虚槽
  存根现象。
