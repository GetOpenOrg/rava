"""[fallback-audit] 转译器静默兜底审计（docs/reports/2026-09-23-fallback-audit.md §四）。

背景：转译器异常兜底过宽吞掉了代码 bug（K-6b 相对导入笔误 → ImportError 被
吞成大面积 stub；typeir-b3 V3 别名 NameError 被吞成 8 次 stub 回退、仅一个
测试可达——安全网吞 bug 的第三例实证）。审计线分两族：

- **A 组（gen_method_body 九吞点 → panic! stub）并入 [cfg-audit]**：与 stub
  语义同族，同生命周期、同 AuditStats。九点收窄为只兜 ``CfgError`` 家族
  （含并入该家族的栈下溢），``CfgAuditError`` 与 ImportError / AttributeError /
  NameError / TypeError / KeyError 等代码 bug 全部穿透硬失败。本模块提供
  九点统一的埋点形态（:func:`stub_fallback`——位点标记 + 异常类型 +
  JAVA_RTA_DEBUG 全点 traceback）。
- **B 组（15 处非 stub 静默降级）新增 [fallback-audit]**：语义是「非 stub
  质量降级」（类型回退描述符 / 渲染空串 / 类解析失败静默丢类），塞进
  cfg-audit 会混淆既有契约，按 equiv_audit 同款 record/summary/reset 模式
  独立成线（见 :func:`record` / :func:`summary`）。

B 组各 ID 的口径与埋点：

==================  =========================================================
ID                  口径（触发条件 / 埋点 / 收窄形态）
==================  =========================================================
sig-parse-field     sig_parse.parse_field_type：字段级 Signature 解析失败 →
                    返回 ''（类型回退描述符）。收窄为 (ValueError, IndexError)。
sig-parse-method    sig_parse.parse_method_param_types：方法级 Signature 解析
                    失败 → ([], '')。同上收窄。
type-map-params     type_map.parse_class_type_params：类级 Signature 形参表
                    解析失败 → 返回部分形参表。同上收窄。
vars-render-loop    vars.py _hoist_loop_vars Pass2：render_stmt 失败 → 空串
                    （提升判定的嵌套深度会算错）。保留 except Exception +
                    计数（render 失败必是 bug；JAVA_RTA_STRICT=1 下穿透）。
vars-render-if      vars.py _hoist_if_vars Pass2：同上。
vars-type-decl      vars.py _hoist_if_vars 提升类型 render_type 失败 → None
                    （对齐检查跳过）。
vars-type-outer     vars.py 顶层后到 let 的类型渲染失败 → None（拆分判定跳过）。
vars-type-later     vars.py 降级前值侧类型渲染失败 → None（对齐检查跳过）。
sam-functional      sam_objects._functional_sam：SAM 描述符无法映射 Rust 类型
                    → None（站点回落闭包装箱）。收窄为 (ValueError, IndexError)。
sam-prescan         sam_objects.prescan：invokedynamic 站点返回描述符解析失败
                    → 跳过该站点。同上收窄。
cc-load-class       callchain._load_class：类字节码解析失败 → 置 None 静默
                    移出层次遍历（调用链缺口源）。保留 except Exception +
                    计数 + 警告清单（解析 JDK 类失败的形态谱不可预枚举）。
cc-root-names       callchain._object_method_names：Object.class 解析失败 →
                    根方法名集合为空（根继承判定全数落入 unresolved）。
cc-root-desc        callchain 根方法描述符类型闭包入队失败 → 丢类型边。
cc-stub-chan        callchain stub 通道类处理失败 → 丢类（此前完全无信号）。
cc-parent-queue     callchain 父类补全队列处理失败 → 丢父类。
==================  =========================================================
"""

from __future__ import annotations

import os

from .cfg import CfgError, STATS

# JAVA_RTA_STRICT 分级（fallback-audit 方案 §4.2，复用 build.rs 既有钩子语义——
# build.rs:30,72 已消费该变量把 needed-native 从 warning 升 cargo::error）：
#   默认          A 组白名单兜底（CfgError → stub）+ B 组计数
#   STRICT=1      A 组九点全穿（无 stub 兜底）+ B 组 vars 渲染失败穿透
#   DEBUG=1       默认之上：九点全打 traceback + B 组逐触发明细
# env 读取只在本模块一处（单点），转译器各吞点经 FALLBACK_EXC / STRICT 消费
STRICT = os.environ.get('JAVA_RTA_STRICT', '') == '1'

# A 组九吞点的兜底异常白名单：默认 (CfgError,)——全语料唯一合法降级流是 CfgError
# （含并入该家族的栈下溢），需要兜底的「可选数据缺失」在源头显式 raise CfgError，
# 不搭便车于偶发 KeyError；strict 下为空元组（全部穿透硬失败，catch 不到任何东西）
FALLBACK_EXC: tuple[type[BaseException], ...] = () if STRICT else (CfgError,)

# B 组静默兜底点的 ID 全集（口径表见模块头）
IDS: tuple[str, ...] = (
    'sig-parse-field', 'sig-parse-method', 'type-map-params',
    'vars-render-loop', 'vars-render-if', 'vars-type-decl',
    'vars-type-outer', 'vars-type-later',
    'sam-functional', 'sam-prescan',
    'cc-load-class', 'cc-root-names', 'cc-root-desc',
    'cc-stub-chan', 'cc-parent-queue',
)

_counts: dict[str, int] = {i: 0 for i in IDS}
_warnings: list[str] = []   # callchain 丢类明细（site: detail），供调用侧打印警告清单


def record(site_id: str, detail: str = '') -> None:
    """B 组某静默兜底点触发 +1（只计数与明细，不改变降级行为本身）。

    detail 非空时进警告清单（callchain 丢类需指名道姓——哪个类被静默移出
    遍历，光有计数无法排查调用链缺口）；JAVA_RTA_DEBUG=1 时逐触发打印明细
    （报告 §4.2：DEBUG=默认之上 + B 组计数明细）。"""
    _counts[site_id] = _counts.get(site_id, 0) + 1
    if detail:
        _warnings.append(f"{site_id}: {detail}")
    if os.environ.get('JAVA_RTA_DEBUG'):
        import sys
        print(f"[fallback-audit] {site_id} {detail}".rstrip(), file=sys.stderr)


def summary() -> str:
    """[fallback-audit] 行：只列非零项（equiv-audit 风格）；全零输出 none。

    2026-09-23 审计实证 B 组 15 点在全语料零触发（死代码）——收窄后任何非零
    都极可能是真 bug（K-6b 型），runner 可经 --deny fallback 升级为整体失败。"""
    items = ' '.join(f"{k}={v}" for k, v in _counts.items() if v)
    return f"[fallback-audit] {items or 'none'}"


def warnings() -> list[str]:
    """callchain 丢类明细的拷贝（调用侧打印警告清单用）。"""
    return list(_warnings)


def reset() -> None:
    """清零（与 cfg AuditStats.reset 同约定，供复用进程的场景）。"""
    for k in _counts:
        _counts[k] = 0
    _warnings.clear()


def stub_fallback(method_id: str, site: str, e: BaseException) -> None:
    """A 组九吞点的 stub 兜底统一埋点（计数写入 cfg AuditStats，归 [cfg-audit]）。

    - 位点标记 ``site``：九点各一名（iface-lambda / iface-private / iface-default /
      main / iface-inherit / iface-special / bridge / super-inherit / clinit），
      AuditStats 去重键为 ``(method_id, site)``——同一方法在多个位点触发各计一次。
    - 异常类型 ``type(e).__name__`` 进 summary 的 ``exc.`` 分解（收窄后默认只有
      CfgError 可达，其余形态出现即白名单漏网，一眼可辨）。
    - ``JAVA_RTA_DEBUG=1``：九点全部打印 traceback（此前仅主循环与 <clinit>
      两点有，其余六点无堆栈——审计报告 §二缺口③）。
    """
    STATS.record_stub_fallback(method_id, repr(e), site=site, exc=type(e).__name__)
    if os.environ.get('JAVA_RTA_DEBUG'):
        import sys
        import traceback
        print(f"[DEBUG] stub fallback ({site}) {method_id}: {e}", file=sys.stderr)
        traceback.print_exc()
