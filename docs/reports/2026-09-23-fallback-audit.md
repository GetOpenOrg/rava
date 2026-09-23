# 转译器异常兜底过宽审计报告（纯审计，未修改任何文件）

> 2026-09-23，纯审计代理交付（主会话入档）。方法：全量 grep + 逐点读码分类；
> 7 次语料 transpile（`--no-run`，scratch 与日志全部落 `/tmp/java_rta_audit/`，
> 其中 4 次带进程内 monkeypatch 仪表计数「异常逃逸点」）+ 1 次 K-6b 注入模拟
> 实验，共 8 次转译调用。副作用说明：transpile 会重写
> `docs/reports/jdk-scan-<Test>.md`（既有机制，内容同源覆写）。

## 一、吞异常点全清单（codegen/ 共 62 条 except 子句，43 个独立吞点）

### A 组 · gen_method_body 九大吞点（`except Exception` → panic! stub）——K-6b 事故同型

统一形态：`except CfgAuditError: raise` / `except Exception as e: _CFG_STATS.record_stub_fallback(...) + _gen_native_stub(...)`。全部在 `codegen/emitter/class_writer.py` 与 `clinit_extract.py`：

| # | 位置（catch Exception 行） | 入口形态 | 降级行为 | 可观测信号 |
|---|---|---|---|---|
| 1 | class_writer.py:232 | `_emit_method_blocks` 接口 lambda 体（G-10） | stub | cfg-audit 计数，**无位点标记、无 traceback** |
| 2 | class_writer.py:267 | 接口私有实例方法 | stub | 标记 `(iface-private)`，无 traceback |
| 3 | class_writer.py:363 | 接口 default 载体体 | 降为 None→纯声明 | 标记 `(iface-default)`，无 traceback |
| 4 | class_writer.py:427 | **主循环普通方法体** | stub | 无标记；JAVA_RTA_DEBUG 有 traceback |
| 5 | class_writer.py:584 | `_emit_interface_default_inheritance` | stub | 无标记 |
| 6 | class_writer.py:657 | `_emit_interface_special_members`（Iface.super.m） | stub | 无标记 |
| 7 | class_writer.py:825 | 超类虚继承 **bridge 体** | 跳过或 stub | 标记 `(bridge)` |
| 8 | class_writer.py:848 | 超类虚继承普通体 | stub | 无标记 |
| 9 | clinit_extract.py:129 | `<clinit>` → `__clinit` | stub | 无标记；JAVA_RTA_DEBUG 有 traceback |

附加缺陷：`audit.py:71 record_stub_fallback` 按 method_id 去重——同一方法在多个位点触发只计 1 次，且抹掉位点信息。

### B 组 · 静默非 stub 吞点（重点可疑，全部零可观测）

| 位置 | catch | 降级行为 | 风险 |
|---|---|---|---|
| sig_parse.py:265 `parse_field_type` | Exception | 返回 `''` → 类型回退描述符/Object | **吞 AttributeError/NameError（代码 bug）** |
| sig_parse.py:354 `parse_method_param_types` | Exception | 返回 `([], '')` → 调用方回退描述符 | 同上 |
| sig_types.py:449 `jvm_to_rs_type` | Exception | 泛型签名→描述符回退 | 宽容与 bug 不可区分 |
| sig_types.py:472 `infer_type_args_from_declared` | Exception | None（菱形推断跳过） | 同上 |
| type_map.py:337 `parse_class_type_params` | Exception | `pass`，返回部分形参表 | 泛型参数错误静默 |
| vars.py:287 / 406（hoist Pass2） | Exception | `rendered.append('')` | **渲染 bug 被吞→提升判定错乱（嵌套深度算错）** |
| vars.py:477 / 590 / 661 | Exception | 类型串 None→对齐检查跳过 | 静默 |
| callchain.py:272 `_load_class` | Exception | 类置 None，静默移出层次遍历 | 解析 bug→调用链缺口 |
| callchain.py:291 / 512 | Exception | 根类方法名空 / 根描述符类型闭包丢边 | 部分可经 bfs-audit 间接观测 |
| callchain.py:707 / 731 | Exception | stub 通道/父类队列丢类 | **完全无信号** |
| sam_objects.py:194 / 242 | Exception | SAM 合成跳过→闭包装箱回退 | 语义 None 与 bug 混同 |

### C 组 · 合法兜底（保留）
- **OSError 家族可选文件探测**（13 处）：callchain.py:51,459；attrs.py:53；invoke_sig.py:229；member_naming.py:105；native_upcalls.py:56；method_gen.py:130,135；project_writer.py:93,318,344；raw_audit.py:52；transpile.py:78（杂散 .class 跳过）。语义=「可选产物不存在」。
- **jdk_resolver.py:112,167,195,208,237**：JDK 发现/zip 查找，KeyError=条目不存在。
- callchain.py:257（RuntimeError 无 JDK→**打印警告**后跳过）、member_owner.py:352（RuntimeError→跳过根方法集）。
- graph.py:328（`CfgError`→`function_always_returns=False`→保守补 return）：语义决策，且已是窄类型。
- type_map.py:194/201/212/218/277（ValueError 容错推进）：描述符残缺时逐字符前进。

## 二、gen_method_body 主链 try/except 结构图

```
write_cargo_project (project_writer.py:148)
 ├─ _scan_impl_files ─────────────────────── [C 组 OSError 探测 ×2]
 ├─ sam_objects.prescan ──────────────────── [B 组 ×2 静默]
 ├─ 每类 _gen_class_rs (class_writer.py:945)
 │   ├─ _gen_clinit_block ──try── gen_method_body ──┐except CfgAuditError→raise
 │   ├─ _emit_method_blocks ──try── gen_method_body ×4 形态（lambda/私有/载体default/普通）
 │   ├─ _emit_interface_default_inheritance ──try──  ┤except Exception→计数+panic! stub
 │   ├─ _emit_interface_special_members ──try──      │   ← A 组九吞点
 │   └─ _emit_superclass_virtual_inheritance ──try──×2（bridge/普通）
 ├─ LAMBDA_NAME_LEDGER.check()  ★硬断言，不吞（好范本）
 └─ 落盘

gen_method_body (method/codegen.py:228) 内部：
 ├─ emitted_method_sig_types → parse_method_param_types → _parse_one_type/_parse_type_args
 │                                                ↑ [B 组：except Exception→空返回，异常到此为止]
 ├─ _structured_entries (codegen.py:173)
 │   ├─ simulate_blocks (blocks.py:954) ── 15 处 raise CfgError（栈深/try 形态/状态机）
 │   │    └─ sim_instr (instr/sim/*.py)：**无任何 except，异常全部上浮** ← K-6b 笔误在此上浮后被 A 组吞掉
 │   ├─ _split_disjoint_try_ranges ── raise CfgError(L144)【实测唯一真实触发源】
 │   ├─ analyze/structure/simplify ── raise CfgError ×10
 │   └─ _verify_tree / ledger.verify ── raise CfgAuditError（必须穿透，已保证）
 ├─ underflow → RuntimeError(codegen.py:426)
 ├─ _hoist_loop_vars/_hoist_if_vars (vars.py) ── [B 组：render_stmt/render_type 异常被吞 ×5]
 └─ 主渲染 render_stmt (codegen.py:447)：异常上浮（正确）
```

**JAVA_RTA_DEBUG=1 现状**：① class_writer.py:431 主路径 traceback；② clinit_extract.py:132 traceback；③ main.py:175-177 stub_fallback 全列（method_id+repr）；④ callchain.py:638 unresolved 调用列。**缺口**：其余 6 吞点无 traceback；`run_tests.py` 汇总只解析 readability/equiv/raw，**stub_fallback 计数未进 runner 聚合与 --deny 体系**；B 组零计数。

## 三、量化抽样（7 语料 + 1 注入实验）

| 测试 | methods | stub_fallback | 吞点实际异常 | B 组静默（仪表） | 红绿 |
|---|---|---|---|---|---|
| HelloWorld | 266 | 0 | — | 0（9 函数逃逸计数全 0） | GREEN |
| TestRecordPattern | 291 | 0 | — | 未仪表 | GREEN |
| TestInterfaceConflict | 281 | 0 | — | 未仪表 | GREEN |
| TestTryInLoop | 268 | 0 | — | 0 | GREEN |
| TestStreamAdvanced | 7,680 | 2 | CfgError×2 | 0 | GREEN |
| TestStringNewMethods | 7,631 | 2 | CfgError×2 | 0 | 审计时红（后续 unify3/rstrip 修复已绿） |
| TestVirtualThread | 7,749 | 2 | CfgError×2 | 0 | GREEN |
| K-6b 注入模拟（ImportError 进 gen_method_body） | 265 | **1** | **ImportError→stub** | — | **exit=0** |

**结论**：
1. 九吞点全部实际触发均为 **CfgError**（`_split_disjoint_try_ranges` 的「try 区间缺体入口块」），固定两个方法：`java/util/AbstractMap.equals` 与 `java/io/ObjectInputStream$BlockDataInputStream.readBlockHeader`——大闭包测试普遍携带这两个 stub（每闭包 2 个），是**语义限制伪装的质量缺口活案例**（当前被测试绿掩盖），建议单独跟进。
2. **B 组静默吞点在全部样本零触发**——其 `except Exception` 现为死代码，任何未来触发都极可能是真 bug（K-6b 类）。收窄它们在语料上零损失。
3. 注入实验逐字复现 K-6b 教训：typo 级 ImportError → transpile exit=0、生成物含 `panic!("stub: …")`、唯一信号 `stub_fallback=1`（且 runner 不聚合）。

## 四、收窄方案（不动码设计）

### 4.1 白名单兜底
- **A 组九点**：只 catch `(CfgError,)`（`CfgAuditError` 继续穿透；把 codegen.py:426 的 underflow `RuntimeError` 并入 CfgError 家族）。ImportError/AttributeError/NameError/TypeError/KeyError 全部穿透硬失败。量化依据：现行唯一合法流就是 CfgError。需要兜底的「可选数据缺失」在源头显式 `raise CfgError`，不搭便车于偶发 KeyError。
- **B 组**：sig_parse/type_map 三点收窄为 `(ValueError, IndexError)`；vars.py 五点先加计数（render 失败必是 bug，strict 下直接穿透）；sam_objects 两点收窄为 `(ValueError, IndexError)`；callchain 五点 parse 失败改计数+警告清单。
- **C 组**：保留不动。

### 4.2 JAVA_RTA_STRICT 分级（复用 build.rs 既有钩子）
build.rs 已消费 `JAVA_RTA_STRICT`（build.rs:30,72：needed-native 从 warning 升 cargo::error）。扩展到转译器：`strict=1` 九点全穿（无 stub 兜底）+ B 组白名单化；**默认**=白名单兜底+计数；`JAVA_RTA_DEBUG=1`=默认之上九点全打 traceback + B 组计数明细。落点：class_writer/clinit_extract 顶部单点读 env 决定 `_FALLBACK_EXC = tuple()` 或 `(CfgError,)`，main.py 零改动。

### 4.3 审计线归属
**A 组并入 [cfg-audit]**（同生命周期、同 AuditStats、与 stub 语义同族）：summary 扩为 `stub_fallback=N(main=a,clinit=b,lambda=c,…|exc.CfgError=x,exc.TypeError=y)`，九点统一加位点标记（现只有 3 个标记）；`record_stub_fallback` 去重键改 `(method_id, site)`。**B 组新增 [fallback-audit]**（equiv_audit 同款 record/summary/reset 模式，run_tests 加解析+汇总+`--deny fallback` 升级）——因其语义是「非 stub 质量降级」，塞进 cfg-audit 会混淆既有契约。

**关键文件**：`codegen/emitter/class_writer.py`、`codegen/emitter/clinit_extract.py`、`codegen/cfg/audit.py`、`codegen/method/codegen.py`、`codegen/method/vars.py`、`codegen/sig_parse.py`、`codegen/callchain.py`、`scripts/main.py`、`scripts/run_tests.py`、`runtime/java_runtime/build.rs`。抽样日志：`/tmp/java_rta_audit/logs/`，仪表脚本：`/tmp/java_rta_audit/instr_run.py`。
