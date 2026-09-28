"""闭包分类统计：转译时按「角色」与「包」细分列出引入的类（控制台汇总 + jdk-scan 报告全表）。

角色（每个 JDK 类恰属其一）：
  翻译方法体        调用链上至少一个方法入链，方法体由字节码翻译
  仅类型存根        只作为类型 / 字段 / 签名被引用，无方法入链（方法全部为 panic 存根）
  手写边界·内部包   closure.toml [boundary] 前缀内、未放行的类（jdk/ sun/ …），整体手写
  手写边界·VM 耦合  closure.toml [vm_boundary] 的公开包类（Class / ClassLoader …），整体手写

来源标注（与角色正交，只标边界前缀内按字节码翻译的类为何被放行）：
  JCA 服务          seeds.toml [jca] 放行的 provider / 算法实现族
  数据资源束        结构判定为纯数据的资源束（CLDR locale 数据等）
  边界放行          closure.toml [release] 放行条目
"""
from __future__ import annotations

from collections import defaultdict

ROLE_TRANSLATED = '翻译方法体'
ROLE_STUB = '仅类型存根'
ROLE_BOUNDARY_INTERNAL = '手写边界·内部包'
ROLE_BOUNDARY_VM = '手写边界·VM 耦合'
ROLES = (ROLE_TRANSLATED, ROLE_STUB, ROLE_BOUNDARY_INTERNAL, ROLE_BOUNDARY_VM)

SRC_JCA = 'JCA 服务'
SRC_BUNDLE = '数据资源束'
SRC_RELEASE = '边界放行'


def _methods_by_class(visited_methods) -> dict[str, int]:
    counts: dict[str, int] = defaultdict(int)
    for cls, meth, _desc in visited_methods or ():
        if meth != '<clinit>':
            counts[cls] += 1
    return counts


def classify(jdk_class_infos, visited_methods) -> list[dict]:
    """每个 JDK 类一条记录：name / package / role / source / methods（入链方法数）。"""
    from . import callchain as cc
    per_cls = _methods_by_class(visited_methods)
    rows = []
    for ci in jdk_class_infos:
        name = ci.name
        n = per_cls.get(name, 0)
        if cc._is_boundary_class(name):
            role = (ROLE_BOUNDARY_VM if name.split('$', 1)[0] in cc._VM_BOUNDARY_CLASSES
                    else ROLE_BOUNDARY_INTERNAL)
        else:
            role = ROLE_TRANSLATED if n else ROLE_STUB
        source = ''
        if name.startswith(cc._JDK_STUB_ONLY_PREFIXES) and not cc._is_boundary_class(name):
            if cc._jca_released(name, cc._JCA_MANIFEST):
                source = SRC_JCA
            elif cc._is_data_bundle(name):
                source = SRC_BUNDLE
            elif cc._released_general(name):
                source = SRC_RELEASE
        rows.append({'name': name, 'package': name.rsplit('/', 1)[0] if '/' in name else '(默认包)',
                     'role': role, 'source': source, 'methods': n})
    return rows


def _package_table(rows) -> list[tuple[str, dict]]:
    pk: dict[str, dict] = {}
    for r in rows:
        d = pk.setdefault(r['package'], {'classes': 0, 'methods': 0, **{k: 0 for k in ROLES}})
        d['classes'] += 1
        d['methods'] += r['methods']
        d[r['role']] += 1
    return sorted(pk.items(), key=lambda kv: (-kv[1]['classes'], kv[0]))


def print_summary(user_count: int, rows, top: int = 20) -> None:
    """控制台汇总：按角色 / 来源 / 包（前 top 个）。"""
    by_role = {k: [r for r in rows if r['role'] == k] for k in ROLES}
    total_m = sum(r['methods'] for r in rows)
    print(f"      闭包分类：用户类 {user_count} / JDK 类 {len(rows)}（入链方法 {total_m}）")
    print("        按角色：")
    for k in ROLES:
        rs = by_role[k]
        m = sum(r['methods'] for r in rs)
        print(f"          {k:<14}{len(rs):>6} 类" + (f"{m:>8} 方法" if m else ''))
    srcs = defaultdict(int)
    for r in rows:
        if r['source']:
            srcs[r['source']] += 1
    if srcs:
        print("        边界前缀内按字节码翻译：" + ' · '.join(f"{k} {v}" for k, v in sorted(srcs.items())))
    table = _package_table(rows)
    print(f"        按包（共 {len(table)} 个包，前 {min(top, len(table))} 个；全表见 jdk-scan 报告）：")
    for pkg, d in table[:top]:
        parts = [f"翻译 {d[ROLE_TRANSLATED]}", f"存根 {d[ROLE_STUB]}"]
        if d[ROLE_BOUNDARY_INTERNAL] or d[ROLE_BOUNDARY_VM]:
            parts.append(f"手写 {d[ROLE_BOUNDARY_INTERNAL] + d[ROLE_BOUNDARY_VM]}")
        print(f"          {pkg:<34}{d['classes']:>5} 类（{' / '.join(parts)}）{d['methods']:>7} 方法")


def write_report_sections(f, rows) -> None:
    """jdk-scan 报告：角色汇总、按包全表、按角色的类清单。"""
    f.write("## 按角色\n\n| 角色 | 类数 | 入链方法 |\n|------|-----:|--------:|\n")
    for k in ROLES:
        rs = [r for r in rows if r['role'] == k]
        f.write(f"| {k} | {len(rs)} | {sum(r['methods'] for r in rs)} |\n")
    f.write("\n## 按包\n\n| 包 | 类数 | 翻译方法体 | 仅类型存根 | 手写边界 | 入链方法 |\n"
            "|----|-----:|-----:|-----:|-----:|-----:|\n")
    for pkg, d in _package_table(rows):
        f.write(f"| `{pkg}` | {d['classes']} | {d[ROLE_TRANSLATED]} | {d[ROLE_STUB]} | "
                f"{d[ROLE_BOUNDARY_INTERNAL] + d[ROLE_BOUNDARY_VM]} | {d['methods']} |\n")
    for k in ROLES:
        rs = sorted((r for r in rows if r['role'] == k), key=lambda r: r['name'])
        if not rs:
            continue
        f.write(f"\n## {k}（{len(rs)}）\n\n| 类 | 入链方法 | 来源 |\n|----|-----:|------|\n")
        for r in rs:
            f.write(f"| `{r['name']}` | {r['methods']} | {r['source']} |\n")
