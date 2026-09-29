"""转译选项：由 scripts/main.py 的命令行参数单点写入（项目不设自有环境变量）。

- DEBUG（--debug）：兜底 / 未解析调用 / 迟到 static 边 / cfg 结构化判定的逐条明细；
- STRICT（--strict）：兜底路径改为硬失败，并写入 scratch 的 java_runtime/strict.txt，
  build.rs 据此把缺手写实现的 native 方法升级为 cargo error；
- TRACE_CLASS（--trace-class）：打印该类（斜线形态 binary name）各方法的入链路径；
- RAW_SITES（--raw-sites）：Raw 逃生舱构造位点剖面输出文件（见 raw_audit）；
- PRECHECK_ONLY（--precheck-only）：转译后打印完整编译前预检明细（[precheck]）即结束，不编译不运行。
"""

DEBUG = False
STRICT = False
TRACE_CLASS = ''
RAW_SITES = ''
PRECHECK_ONLY = False
