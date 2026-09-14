"""
转译流水线：javac → parse → jdk_scan → codegen → Cargo 项目。
"""

import subprocess
import sys
import os
from collections import deque
from .classfile import parse_class
from .emitter import write_cargo_project
from .type_map import load_ergonomic_renames

# JDK 包前缀（binary name 斜线分隔）- 这些类的方法会被 BFS 展开并翻译
# 只展开公开 API（java/ javax/）；内部实现包（sun/ jdk/ com.sun/ com.oracle/）截断为 stub
_JDK_PREFIXES = ('java/', 'javax/')

# 内部包边界：方法体全部为 panic! stub，不展开调用链
# 这是内部边界截断策略的核心——sun/ 等包的实现细节不翻译，只生成类型占位符
# java/security/ 是 JVM 安全服务层（JDK 21 的 SecurityManager 始终为 null），视为内部边界
_JDK_STUB_ONLY_PREFIXES = ('sun/', 'jdk/', 'com/sun/', 'com/oracle/', 'java/security/')

# java_runtime 已手写实现的类：这些类不再由 jdk_classes 翻译，避免重复定义和命名冲突
_JAVA_RUNTIME_CLASSES: frozenset[str] = frozenset({
    'java/lang/Object',
})


def transpile(java_files: list[str], out_dir: str, batch_bin: bool = False):
    for jf in java_files:
        if not os.path.exists(jf):
            sys.exit(f"File not found: {jf}")

    # .class 统一输出到 <源码目录>/classes/，与 .java 隔离
    src_dir   = os.path.dirname(os.path.abspath(java_files[0]))
    class_dir = os.path.join(src_dir, 'classes')
    os.makedirs(class_dir, exist_ok=True)

    # 1. javac
    print(f"[1/4] javac {' '.join(java_files)}")
    r = subprocess.run(['javac', '-g', '-d', class_dir] + java_files,
                       capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f"javac failed:\n{r.stderr}")

    # 2. 解析用户 .class 二进制（含内部类 $ 文件）
    class_infos = []
    _seen_class_files: set[str] = set()
    for jf in java_files:
        class_name = os.path.splitext(os.path.basename(jf))[0]
        class_file = os.path.join(class_dir, class_name + '.class')

        print(f"[2/4] 解析 {class_name}.class")
        ci = parse_class(class_file)
        ci.methods = [m for m in ci.methods if m.name != '<clinit>']
        print(f"      字段: {[f.name for f in ci.fields]}")
        print(f"      方法: {[m.name for m in ci.methods]}")
        class_infos.append(ci)
        _seen_class_files.add(class_file)

        # 发现同目录下的内部类（OuterClass$Inner.class）
        for fname in sorted(os.listdir(class_dir)):
            if fname.startswith(class_name + '$') and fname.endswith('.class'):
                inner_file = os.path.join(class_dir, fname)
                if inner_file in _seen_class_files:
                    continue
                _seen_class_files.add(inner_file)
                inner_ci = parse_class(inner_file)
                inner_ci.methods = [m for m in inner_ci.methods if m.name != '<clinit>']
                print(f"      内部类: {fname[:-6]} (字段: {[f.name for f in inner_ci.fields]}, 方法: {[m.name for m in inner_ci.methods]})")
                class_infos.append(inner_ci)

    # 3. 方法级调用链 BFS 发现 JDK 类
    print(f"[3/4] 扫描 JDK 类引用...", end=' ', flush=True)
    jdk_class_infos, visited_methods, field_stubs = _discover_jdk_classes_method_level(class_infos)
    field_stub_count = sum(1 for ci in jdk_class_infos
                           if any(ci.name == cls for cls in field_stubs))
    bfs_count = len(jdk_class_infos) - field_stub_count
    print(f"完成，{bfs_count} 个调用链类 + {field_stub_count} 个 field stub = {len(jdk_class_infos)} 个")

    # 写 JDK 扫描报告
    project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    reports_dir = os.path.join(project_root, 'docs', 'reports')
    os.makedirs(reports_dir, exist_ok=True)
    stem0 = os.path.splitext(os.path.basename(java_files[0]))[0]
    report_path = os.path.join(reports_dir, f"jdk-scan-{stem0}.md")
    _write_jdk_scan_report(report_path, jdk_class_infos, field_stubs)
    print(f"      JDK 扫描报告 → {report_path}")

    # 4. 生成 Rust
    print(f"[4/4] 生成 Rust → {out_dir}/")
    # 预加载 ergonomic @jvm_rename 指令（T39：为 ergonomic 层腾出干净方法名）
    load_ergonomic_renames(os.path.abspath(out_dir))
    write_cargo_project(out_dir, class_infos, jdk_class_infos, java_files,
                        batch_bin=batch_bin, visited_methods=visited_methods)
    print(f"\n✓ 完成。运行方式：\n  cd {out_dir} && cargo run --release")


def _write_jdk_scan_report(path: str, jdk_class_infos: list, field_stubs: set):
    """将 JDK 扫描结果写成 Markdown 报告。"""
    from datetime import date
    field_stub_names = {ci.name for ci in jdk_class_infos if ci.name in field_stubs}
    callchain_infos = [ci for ci in jdk_class_infos if ci.name not in field_stubs]
    with open(path, 'w', encoding='utf-8') as f:
        f.write(f"# JDK 扫描报告\n\n生成时间：{date.today()}\n\n")
        f.write("## 摘要\n\n")
        f.write(f"| 来源 | 类数 |\n|------|-----:|\n")
        f.write(f"| 调用链 BFS | {len(callchain_infos)} |\n")
        f.write(f"| Field-only stub | {len(field_stub_names)} |\n")
        f.write(f"| 合计 | {len(jdk_class_infos)} |\n\n")
        f.write("## 调用链 BFS 发现的类\n\n")
        f.write("| 类名 | 方法数 | native 数 |\n|------|-------:|----------:|\n")
        for ci in sorted(callchain_infos, key=lambda c: c.name):
            native = sum(1 for m in ci.methods if m.is_native)
            f.write(f"| `{ci.name}` | {len(ci.methods)} | {native} |\n")
        f.write("\n## Field-only Stub 类\n\n")
        f.write("| 类名 | 方法数 | native 数 |\n|------|-------:|----------:|\n")
        for ci in sorted((ci for ci in jdk_class_infos if ci.name in field_stubs),
                         key=lambda c: c.name):
            native = sum(1 for m in ci.methods if m.is_native)
            f.write(f"| `{ci.name}` | {len(ci.methods)} | {native} |\n")


def _collect_method_refs(instrs) -> tuple[list[tuple[str, str, str]], list[str]]:
    """从指令注释中提取方法引用和字段所属类引用（仅 JDK 类）。

    Returns:
        (method_refs, field_classes):
          method_refs   - (cls, method, descriptor) 三元组，用于 BFS 展开
          field_classes - 通过 getstatic/Field 指令发现的类名，只生成存根不展开方法体
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
                # stub-only 优先检查（java/security/ 等是 java/ 的子前缀，必须先匹配）
                if cls.startswith(_JDK_STUB_ONLY_PREFIXES) and '[' not in cls:
                    field_classes.append(cls)
                elif cls.startswith(_JDK_PREFIXES) and '[' not in cls:
                    method_refs.append((cls, meth, desc))
        elif c.startswith('Field '):
            # "Field java/nio/charset/CodingErrorAction.REPLACE:Ljava/nio/charset/CodingErrorAction;"
            # getstatic/putstatic/getfield/putfield - 只发现声明类，不展开其方法体
            rest = c[6:]  # 去掉 "Field "
            dot = rest.find('.')
            if dot > 0:
                cls = rest[:dot]
                if (cls.startswith(_JDK_PREFIXES) or cls.startswith(_JDK_STUB_ONLY_PREFIXES)) and '[' not in cls:
                    field_classes.append(cls)
        elif c.startswith(_JDK_STUB_ONLY_PREFIXES) and '[' not in c:
            # stub-only 内部类的 new/checkcast 指令 → 仅生成存根，不展开方法体
            cls = c.split()[0]
            field_classes.append(cls)
        elif c.startswith(_JDK_PREFIXES) and '[' not in c:
            # new / checkcast / anewarray: comment = class binary name
            cls = c.split()[0]
            method_refs.append((cls, '<init>', '()V'))
    return method_refs, field_classes


def _discover_jdk_classes_method_level(class_infos: list) -> list:
    """方法级调用链 BFS：只追踪实际被调用的方法，不展开未调用方法的依赖类。"""
    from .classfile import parse_class_bytes
    from .jdk_resolver import JdkResolver

    visited_methods: set[tuple[str, str, str]] = set()
    queue: deque[tuple[str, str, str]] = deque()
    # 通过 getstatic/Field 指令发现的类：只生成存根，不展开方法体
    field_discover_classes: set[str] = set()

    def enqueue_refs(instrs):
        method_refs, f_classes = _collect_method_refs(instrs)
        for cls in f_classes:
            if cls not in _JAVA_RUNTIME_CLASSES:
                field_discover_classes.add(cls)
        for key in method_refs:
            cls = key[0]
            if cls in _JAVA_RUNTIME_CLASSES:
                continue
            if key not in visited_methods:
                visited_methods.add(key)
                queue.append(key)

    # 初始种子：用户类所有方法的引用
    for ci in class_infos:
        for m in ci.methods:
            enqueue_refs(m.instrs or [])

    if not queue:
        print("      无 JDK 类引用")
        return [], set()

    class_cache: dict[str, object] = {}
    jdk_infos: dict[str, object] = {}

    try:
        resolver = JdkResolver()
    except RuntimeError as e:
        print(f"      警告：{e}，跳过 JDK 元数据生成")
        return [], set()

    with resolver:
        while queue:
            cls, meth, desc = queue.popleft()

            # 解析类（首次遇到时）
            if cls not in class_cache:
                data = resolver.resolve(cls)
                if data is None:
                    class_cache[cls] = None
                    continue
                try:
                    ci = parse_class_bytes(data, cls)
                    class_cache[cls] = ci
                    if cls not in jdk_infos:
                        jdk_infos[cls] = ci
                except Exception as e:
                    class_cache[cls] = None
                    continue

            ci = class_cache.get(cls)
            if ci is None:
                continue

            if cls not in jdk_infos:
                jdk_infos[cls] = ci

            # 追踪该方法的指令引用（精确匹配名字+描述符，避免重载方法误展开）
            for m in ci.methods:
                if m.name == meth and m.descriptor == desc:
                    enqueue_refs(m.instrs or [])

        # field_discover_classes + T76 父类链：BFS 处理，递归包含所有父类
        # T76 生成 pub _super: ParentType，需要父类类型存在于 jdk_infos
        _stub_queue: deque[str] = deque(field_discover_classes)
        _stub_visited: set[str] = set(field_discover_classes)
        while _stub_queue:
            cls = _stub_queue.popleft()
            if cls in jdk_infos:
                continue
            data = resolver.resolve(cls)
            if data is None:
                continue
            try:
                ci = parse_class_bytes(data, cls)
                jdk_infos[cls] = ci
                # 递归添加父类（_super 字段需要父类类型存在）
                if (ci.super_class and ci.super_class != 'java/lang/Object'
                        and ci.super_class not in _JAVA_RUNTIME_CLASSES
                        and ci.super_class not in jdk_infos
                        and ci.super_class not in _stub_visited):
                    _stub_visited.add(ci.super_class)
                    _stub_queue.append(ci.super_class)
            except Exception:
                pass

        # 同样为 BFS 调用链中发现的类递归添加父类
        _parent_queue: deque[str] = deque()
        for _ci in list(jdk_infos.values()):
            if (_ci and _ci.super_class and _ci.super_class != 'java/lang/Object'
                    and _ci.super_class not in _JAVA_RUNTIME_CLASSES
                    and _ci.super_class not in jdk_infos):
                _parent_queue.append(_ci.super_class)
        while _parent_queue:
            cls = _parent_queue.popleft()
            if cls in jdk_infos:
                continue
            data = resolver.resolve(cls)
            if data is None:
                continue
            try:
                ci = parse_class_bytes(data, cls)
                jdk_infos[cls] = ci
                if (ci.super_class and ci.super_class != 'java/lang/Object'
                        and ci.super_class not in _JAVA_RUNTIME_CLASSES
                        and ci.super_class not in jdk_infos):
                    _parent_queue.append(ci.super_class)
            except Exception:
                pass

    return list(jdk_infos.values()), visited_methods, field_discover_classes
