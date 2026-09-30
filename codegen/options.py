"""转译选项：由 scripts/main.py 的命令行参数单点写入（项目不设自有环境变量）。

- DEBUG（--debug）：兜底 / 闭包未解析调用 / cfg 结构化判定的逐条明细；
- STRICT（--strict）：兜底路径改为硬失败，并写入 scratch 的 java_runtime/strict.txt，
  build.rs 据此把缺手写实现的 native 方法升级为 cargo error；
- TRACE_CLASS（--trace-class）：打印该类或方法入闭包的最短 provenance 链（转交 `rava closure --why`）；
- RAW_SITES（--raw-sites）：Raw 逃生舱构造位点剖面输出文件（见 raw_audit）；
- PRECHECK_ONLY（--precheck-only）：转译后打印完整编译前预检明细（[precheck]）即结束，不编译不运行；
- JDK_SEEDS（scripts/gap_scan.py api 模式）：额外的 JDK 方法入口 (类, 方法, 描述符)，等价于用户程序调用了它们。
"""

DEBUG = False
STRICT = False
TRACE_CLASS = ''
RAW_SITES = ''
PRECHECK_ONLY = False
JDK_SEEDS: list = []
