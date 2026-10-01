# 分布式全量第二轮基线回归（8 例）定位记录

分支 regress2（基于 rust-closure-analyzer@f685c7b5）。复现与验证均单例运行。

## 1. TestForNameInit —— 已修

- 现象：`Class.forName("[I").getName()` 处 null_recv 违约 panic。
- 根因：闭包分析器按名取类（`engine/class_lookup.rs`）对描述符形式的数组类名直接跳过，候选集为空且非 top，
  调用点结果只剩空集 → 后续 `getName` 被判接收者恒 null（null_recv 折叠）。
- 修法：数组类名的元素类型可解析（单字符基本类型或类路径上的类）时，结果取数组类镜像（同 ldc 数组类常量，
  只 touch 不初始化元素类）；有实例化期望类型时不保留（数组类不可实例化）。
- 验证：单例输出与期望一致；HelloWorld / TestForNameInit 的 classes / methods / instantiated 集合不变。

## 2. TestNestedGeneric —— 已修

- 现象：`Box<Integer>.map(Object::toString)` 内 `new Box<>(f.apply(value))` 生成 `Box::<T>::new(From::from(_t0))`，
  T = Integer 时对 String 做 checkcast → ClassCastException。
- 根因：Rust 生成器擦除方法级类型变量（`map` 的 R → Object），构造实参是顶层 Object；
  `resolve_ctor_turbofish_args` 规则 1 视顶层 Object 为「无约束」，落到规则 2（同顶层类同名形参）取调用方的 T。
  Python 版本保留方法级类型变量（实参类型为 R，规则 1 直接绑定），故基线通过。
- 修法（`instr/src/invoke/bind.rs`）：裸类型变量形参收到顶层 Object 实参且别处未绑定时绑定为 Object
  （存储擦除下实例化只是视图，Object 恒可行；取调用方 T 会引入 Java 不做的检查转换）。
- 验证：TestNestedGeneric 通过；HelloWorld / GenericClassDemo / GenericMethodTest / TestRawTypes / ArrayListTest 单例通过（编译 + 输出）。

## 3. TestClassNaming / TestClassLiteral —— 已修

- 现象：TestClassNaming 编译 E0599（main.rs 类初始化钩子引用不透明类 `__class_init`）；修掉后 / TestClassLiteral
  输出不符：`isAssignableFrom` 恒 false、`getEnclosingClass` / `isMemberClass` 等反射结果缺失。
- 根因：L1 不透明类发射（C3 第 6 项）只发 `#[binary_name]`，丢了类级元数据（all_supertypes / inner_classes /
  enclosing_method / modifiers / super_class / interfaces 等），java_meta 构建脚本建不出层次表与内部类表；
  而类镜像是非 null 的 Class，镜像上的反射查询读的正是这些表。另 `class_init_hooks` 未排除不透明类。
- 修法（emit）：`head::opaque_metadata_lines` 对不透明类照发类级元数据（去掉 has_clinit / 注解原始字节），
  `opaque.rs` 写入 `java_class_opaque!`（宏解析本就忽略未知外层属性）；`entry.rs` 钩子跳过不透明类（用户 / JDK 两侧）。
- 验证：TestClassNaming / TestClassLiteral / HelloWorld 单例与期望一致；`cargo test --release -- --test-threads=1` 通过。
  TestAnnoValues 编译错误随之消失，运行期另有根因（见 4）。
