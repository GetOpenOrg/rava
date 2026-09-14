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
from codegen.transpile import _JDK_PREFIXES


# ── 工具函数 ──────────────────────────────────────────────────────────────────

def _parse_instr_refs(instrs):
    """从指令注释中提取 (class, method, descriptor) 三元组。"""
    refs = []
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
                    refs.append((cls, meth, desc))
        elif c.startswith(_JDK_PREFIXES) and '[' not in c:
            # new / checkcast / anewarray: comment = class binary name
            cls = c.split()[0]
            refs.append((cls, '<init>', '()V'))
    return refs


def _collect_class_refs_all_methods(ci):
    """当前类级 BFS 策略：扫描类的所有方法，返回引用的类名集合。"""
    classes = set()
    for m in ci.methods:
        if m.name == '<clinit>':
            continue
        for cls, _, _ in _parse_instr_refs(m.instrs):
            classes.add(cls)
    return classes


# ── 当前类级 BFS（复现现有逻辑）────────────────────────────────────────────────

def class_level_bfs(user_class_infos, resolver):
    """复现 transpile.py _discover_jdk_classes 的类级 BFS。"""
    initial = set()
    for ci in user_class_infos:
        for m in ci.methods:
            for cls, _, _ in _parse_instr_refs(m.instrs):
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
    """方法级 BFS：只追踪实际被调用的方法，未被调用的方法不展开其依赖。"""
    reachable_methods = set()
    queue = deque()
    for ci in user_class_infos:
        for m in ci.methods:
            for ref in _parse_instr_refs(m.instrs):
                if ref not in reachable_methods:
                    reachable_methods.add(ref)
                    queue.append(ref)

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
            # 建立方法名索引，避免后续每次线性扫描
            ci._method_index = defaultdict(list)
            for m in ci.methods:
                ci._method_index[m.name].append(m)
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
        for m in ci._method_index.get(meth, []):
            for ref in _parse_instr_refs(m.instrs):
                if ref not in reachable_methods:
                    reachable_methods.add(ref)
                    queue.append(ref)

    reachable_classes = {cls for cls, _, _ in reachable_methods}
    return reachable_methods, reachable_classes, class_cache


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

    # 输出文件放在第一个 .java 文件同目录
    out_dir = os.path.dirname(os.path.abspath(java_files[0]))
    report_path = os.path.join(out_dir, 'callchain_report.txt')

    user_names = [ci.name for ci in user_infos]
    print(f"用户类：{user_names}")

    with JdkResolver() as resolver:
        # 1. 类级 BFS
        print("运行类级 BFS ...", end=' ', flush=True)
        cls_infos = class_level_bfs(user_infos, resolver)
        print(f"完成，{len(cls_infos)} 个类")

        # 2. 方法级 BFS
        print("运行方法级 BFS ...", end=' ', flush=True)
        reach_methods, reach_classes, cache = method_level_bfs(user_infos, resolver)
        print(f"完成，{len(reach_classes)} 个类 / {len(reach_methods)} 个方法")

        only_in_class_bfs = set(cls_infos) - reach_classes
        only_in_method_bfs = reach_classes - set(cls_infos)

        # 控制台摘要
        print()
        print(f"【摘要】")
        print(f"  类级 BFS  : {len(cls_infos)} 个类")
        print(f"  方法级 BFS: {len(reach_classes)} 个类，{len(reach_methods)} 个方法")
        print(f"  节省       : {len(only_in_class_bfs)} 个类（方法级不需要）")
        if only_in_method_bfs:
            print(f"  方法级额外发现: {len(only_in_method_bfs)} 个类")
        print(f"\n详细报告 → {report_path}")

        # 写详细报告到文件
        with open(report_path, 'w', encoding='utf-8') as rpt:
            rpt.write(f"用户类：{user_names}\n")
            rpt.write("=" * 70 + "\n")

            rpt.write("\n【类级 BFS】\n")
            for name in sorted(cls_infos):
                ci = cls_infos[name]
                total = len(ci.methods)
                native = sum(1 for m in ci.methods if m.is_native)
                rpt.write(f"  {name}  ({total} 方法, {native} native)\n")

            rpt.write("\n【方法级调用链 BFS】\n")
            by_cls = defaultdict(list)
            for cls, meth, desc in sorted(reach_methods):
                by_cls[cls].append(f"{meth}{desc}")
            for cls in sorted(by_cls):
                rpt.write(f"  {cls}\n")
                for sig in sorted(by_cls[cls]):
                    rpt.write(f"    ↳ {sig}\n")

            rpt.write("\n【仅类级 BFS 拉入（方法级不需要）】\n")
            for c in sorted(only_in_class_bfs):
                ci = cls_infos[c]
                rpt.write(f"  {c}  ({len(ci.methods)} 方法)\n")

            if only_in_method_bfs:
                rpt.write("\n【仅方法级 BFS 发现（类级未发现）】\n")
                for c in sorted(only_in_method_bfs):
                    rpt.write(f"  {c}\n")


if __name__ == '__main__':
    main()
