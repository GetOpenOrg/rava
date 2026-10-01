# 分布式全量第二轮基线回归（8 例）定位记录

分支 regress2（基于 rust-closure-analyzer@f685c7b5）。复现与验证均单例运行。

## 1. TestForNameInit —— 已修

- 现象：`Class.forName("[I").getName()` 处 null_recv 违约 panic。
- 根因：闭包分析器按名取类（`engine/class_lookup.rs`）对描述符形式的数组类名直接跳过，候选集为空且非 top，
  调用点结果只剩空集 → 后续 `getName` 被判接收者恒 null（null_recv 折叠）。
- 修法：数组类名的元素类型可解析（单字符基本类型或类路径上的类）时，结果取数组类镜像（同 ldc 数组类常量，
  只 touch 不初始化元素类）；有实例化期望类型时不保留（数组类不可实例化）。
- 验证：单例输出与期望一致；HelloWorld / TestForNameInit 的 classes / methods / instantiated 集合不变。
