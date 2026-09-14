#!/usr/bin/env python3
"""
方法级调用链分析：对比「当前类级 BFS」vs「方法级调用链」发现的 JDK 类数量。

用法：
    python3 scripts/analyze_callchain.py tests/e2e/01_basics/HelloWorld.java
"""

import sys
import os
import subprocess
from collections import deque, defaultdict

sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))
from codegen.classfile import parse_class_bytes
from codegen.jdk_resolver import JdkResolver
from codegen.transpile import _JDK_PREFIXES, _JDK_STUB_ONLY_PREFIXES


# ── 工具函数 ──────────────────────────────────────────────────────────────────

def _parse_instr_refs(instrs):
    """从指令注释中提取方法引用和字段所属类引用（仅 JDK 类）。

    Returns:
        (method_refs, field_classes):
          method_refs   - [(cls, method, descriptor), ...]，用于 BFS 展开
          field_classes - [cls, ...]，通过 Field 指令发现，只生成存根不展开方法体
    """
    method_refs = []
    field_classes = []
    for instr in (instrs or []):
        c = instr.comment
        if not c:
            continue
        if c.startswith(('Method ', 'InterfaceMethod ')):
            # "Method java/io/PrintStream.println:(Ljava/lang/String;)V"
            rest = c.split(' ', 1)[1]
            dot = rest.find('.')
            colon = rest.find(':', dot)
            if dot > 0 and colon > dot:
                cls = rest[:dot]
                meth = rest[dot+1:colon]
                desc = rest[colon+1:]
                if cls.startswith(_JDK_PREFIXES) and '[' not in cls:
                    method_refs.append((cls, meth, desc))
                elif cls.startswith(_JDK_STUB_ONLY_PREFIXES) and '[' not in cls:
                    # jdk/ 内部类方法调用 → 只生成类型存根，不展开方法体
                    field_classes.append(cls)
        elif c.startswith('Field '):
            # "Field java/nio/charset/CodingErrorAction.REPLACE:..."
            rest = c[6:]
            dot = rest.find('.')
            if dot > 0:
                cls = rest[:dot]
                if (cls.startswith(_JDK_PREFIXES) or cls.startswith(_JDK_STUB_ONLY_PREFIXES)) and '[' not in cls:
                    field_classes.append(cls)
        elif c.startswith(_JDK_PREFIXES) and '[' not in c:
            # new / checkcast / anewarray: comment = class binary name
            cls = c.split()[0]
            method_refs.append((cls, '<init>', '()V'))
        elif c.startswith(_JDK_STUB_ONLY_PREFIXES) and '[' not in c:
            # jdk/ 内部类的 new/checkcast 指令 → 仅生成存根，不展开方法体
            cls = c.split()[0]
            field_classes.append(cls)
    return method_refs, field_classes


def _collect_class_refs_all_methods(ci):
    """当前类级 BFS 策略：扫描类的所有方法，返回引用的类名集合。"""
    classes = set()
    for m in ci.methods:
        if m.name == '<clinit>':
            continue
        method_refs, _ = _parse_instr_refs(m.instrs)
        for cls, _, _ in method_refs:
            classes.add(cls)
    return classes


# ── 当前类级 BFS（复现现有逻辑）────────────────────────────────────────────────

def class_level_bfs(user_class_infos, resolver):
    """复现 transpile.py _discover_jdk_classes 的类级 BFS。"""
    initial = set()
    for ci in user_class_infos:
        for m in ci.methods:
            method_refs, _ = _parse_instr_refs(m.instrs)
            for cls, _, _ in method_refs:
                initial.add(cls)

    visited_classes = set()
    queue = deque(initial)
    infos = {}

    while queue:
        cls = queue.popleft()
        if cls in visited_classes:
            continue
        visited_classes.add(cls)
        data = resolver.resolve(cls)
        if data is None:
            continue
        try:
            ci = parse_class_bytes(data, cls)
            infos[cls] = ci
            for ref_cls in _collect_class_refs_all_methods(ci):
                if ref_cls not in visited_classes:
                    queue.append(ref_cls)
        except Exception:
            pass

    return infos


# ── 方法级调用链 BFS ──────────────────────────────────────────────────────────

def method_level_bfs(user_class_infos, resolver):
    """方法级 BFS：只追踪实际被调用的方法，未被调用的方法不展开其依赖。
    同时收集 Field 指令发现的类作为 field-only stub（与 transpile.py 对齐）。
    """
    reachable_methods = set()
    field_discover_classes = set()
    queue = deque()

    def enqueue(instrs):
        method_refs, f_classes = _parse_instr_refs(instrs)
        for cls in f_classes:
            field_discover_classes.add(cls)
        for ref in method_refs:
            if ref not in reachable_methods:
                reachable_methods.add(ref)
                queue.append(ref)

    for ci in user_class_infos:
        for m in ci.methods:
            enqueue(m.instrs or [])

    class_cache = {}  # binary_name → ClassInfo，value 附带 _method_index

    def get_ci(cls):
        if cls in class_cache:
            return class_cache[cls]
        data = resolver.resolve(cls)
        if data is None:
            class_cache[cls] = None
            return None
        try:
            ci = parse_class_bytes(data, cls)
            ci._method_index = defaultdict(list)
            for m in ci.methods:
                ci._method_index[(m.name, m.descriptor)].append(m)
            class_cache[cls] = ci
            return ci
        except Exception:
            class_cache[cls] = None
            return None

    while queue:
        cls, meth, desc = queue.popleft()
        ci = get_ci(cls)
        if ci is None:
            continue
        for m in ci._method_index.get((meth, desc), []):
            enqueue(m.instrs or [])

    reachable_classes = {cls for cls, _, _ in reachable_methods}

    # 补充 field-only stub 类：在 BFS 调用链中未出现、但通过 Field 指令引用的类
    field_only_classes = {}
    for cls in field_discover_classes:
        if cls not in reachable_classes:
            ci = get_ci(cls)
            if ci is not None:
                field_only_classes[cls] = ci

    return reachable_methods, reachable_classes, field_only_classes, class_cache


# ── 主程序 ────────────────────────────────────────────────────────────────────

def main():
    java_files = sys.argv[1:]
    if not java_files:
        sys.exit("用法: python3 scripts/analyze_callchain.py tests/HelloWorld.java")

    # 编译
    src_dir = os.path.dirname(os.path.abspath(java_files[0]))
    class_dir = os.path.join(src_dir, 'classes')
    os.makedirs(class_dir, exist_ok=True)
    r = subprocess.run(['javac', '-g', '-d', class_dir] + java_files,
                       capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f"javac failed:\n{r.stderr}")

    # 解析用户类
    user_infos = []
    for jf in java_files:
        stem = os.path.splitext(os.path.basename(jf))[0]
        class_file = os.path.join(class_dir, stem + '.class')
        if os.path.exists(class_file):
            with open(class_file, 'rb') as f:
                user_infos.append(parse_class_bytes(f.read(), stem))

    # 报告写到项目根目录 docs/reports/
    project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    reports_dir = os.path.join(project_root, 'docs', 'reports')
    os.makedirs(reports_dir, exist_ok=True)
    stem0 = os.path.splitext(os.path.basename(java_files[0]))[0]
    from datetime import date
    report_path = os.path.join(reports_dir, f"callchain-{stem0}.md")

    user_names = [ci.name for ci in user_infos]
    print(f"用户类：{user_names}")

    with JdkResolver() as resolver:
        # 1. 类级 BFS
        print("运行类级 BFS ...", end=' ', flush=True)
        cls_infos = class_level_bfs(user_infos, resolver)
        print(f"完成，{len(cls_infos)} 个类")

        # 2. 方法级 BFS（含 field-only stub）
        print("运行方法级 BFS ...", end=' ', flush=True)
        reach_methods, reach_classes, field_only, cache = method_level_bfs(user_infos, resolver)
        total_method_classes = len(reach_classes) + len(field_only)
        print(f"完成，{len(reach_classes)} 个调用链类 + {len(field_only)} 个 field-only stub = {total_method_classes} 个")

        all_method_classes = reach_classes | set(field_only)
        only_in_class_bfs = set(cls_infos) - all_method_classes
        only_in_method_bfs = all_method_classes - set(cls_infos)

        # 控制台摘要
        print()
        print(f"【摘要】")
        print(f"  类级 BFS  : {len(cls_infos)} 个类")
        print(f"  方法级 BFS: {total_method_classes} 个类（{len(reach_classes)} 调用链 + {len(field_only)} field stub）/ {len(reach_methods)} 个方法")
        print(f"  节省       : {len(only_in_class_bfs)} 个类（方法级不需要）")
        if only_in_method_bfs:
            print(f"  方法级额外发现: {len(only_in_method_bfs)} 个类")
        print(f"\n详细报告 → {report_path}")

        # 写详细报告（Markdown）
        with open(report_path, 'w', encoding='utf-8') as rpt:
            rpt.write(f"# 调用链分析报告：{', '.join(user_names)}\n\n")
            rpt.write(f"生成时间：{date.today()}\n\n")

            rpt.write("## 摘要\n\n")
            rpt.write(f"| 策略 | 类数 | 方法数 |\n")
            rpt.write(f"|------|-----:|-------:|\n")
            rpt.write(f"| 类级 BFS | {len(cls_infos)} | — |\n")
            rpt.write(f"| 方法级 BFS（调用链） | {len(reach_classes)} | {len(reach_methods)} |\n")
            rpt.write(f"| 方法级 BFS（field-only stub） | {len(field_only)} | — |\n")
            rpt.write(f"| 方法级 BFS 合计 | {total_method_classes} | {len(reach_methods)} |\n")
            rpt.write(f"| 节省（方法级不需要） | {len(only_in_class_bfs)} | — |\n\n")

            rpt.write("## 类级 BFS 发现的类\n\n")
            rpt.write("| 类名 | 方法数 | native 数 |\n")
            rpt.write("|------|-------:|----------:|\n")
            for name in sorted(cls_infos):
                ci = cls_infos[name]
                total = len(ci.methods)
                native = sum(1 for m in ci.methods if m.is_native)
                rpt.write(f"| `{name}` | {total} | {native} |\n")

            rpt.write("\n## 方法级 BFS 可达方法\n\n")
            by_cls = defaultdict(list)
            for cls, meth, desc in sorted(reach_methods):
                by_cls[cls].append(f"{meth}{desc}")
            for cls in sorted(by_cls):
                rpt.write(f"### `{cls}`\n\n")
                for sig in sorted(by_cls[cls]):
                    rpt.write(f"- `{sig}`\n")
                rpt.write("\n")

            rpt.write("## Field-only Stub 类（仅通过字段访问发现）\n\n")
            rpt.write("| 类名 | 方法数 | native 数 |\n")
            rpt.write("|------|-------:|----------:|\n")
            for name in sorted(field_only):
                ci = field_only[name]
                total = len(ci.methods)
                native = sum(1 for m in ci.methods if m.is_native)
                rpt.write(f"| `{name}` | {total} | {native} |\n")

            rpt.write("\n## 仅类级 BFS 拉入（方法级不需要）\n\n")
            rpt.write("| 类名 | 方法数 |\n")
            rpt.write("|------|-------:|\n")
            for c in sorted(only_in_class_bfs):
                ci = cls_infos[c]
                rpt.write(f"| `{c}` | {len(ci.methods)} |\n")

            if only_in_method_bfs:
                rpt.write("\n## 仅方法级 BFS 发现（类级未发现）\n\n")
                for c in sorted(only_in_method_bfs):
                    rpt.write(f"- `{c}`\n")


if __name__ == '__main__':
    main()
