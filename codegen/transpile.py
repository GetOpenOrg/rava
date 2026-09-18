"""
转译流水线：javac → parse → jdk_scan → codegen → Cargo 项目。
"""

import subprocess
import sys
import os
import re
from collections import deque
from .classfile import parse_class
from .emitter import write_cargo_project
from .constants import OBJECT_CLASS as _OBJECT_CLASS, RUNTIME_JAVA_RUNTIME as _RUNTIME_JAVA_RUNTIME


# JDK 包前缀（binary name 斜线分隔）- 这些类的方法会被 BFS 展开并翻译
# 只展开公开 API（java/ javax/）；内部实现包（sun/ jdk/ com.sun/ com.oracle/）截断为 stub
_JDK_PREFIXES = ('java/', 'javax/')

# 内部包边界：方法体全部为 panic! stub，不展开调用链
# 这是内部边界截断策略的核心——sun/ 等包的实现细节不翻译，只生成类型占位符
# java/security/ 是 JVM 安全服务层（JDK 21 的 SecurityManager 始终为 null），视为内部边界
_JDK_STUB_ONLY_PREFIXES = ('sun/', 'jdk/', 'com/sun/', 'com/oracle/', 'java/security/')



def _read_manifest(name: str) -> list[str]:
    """读取 runtime/java_runtime/ 下的 VM 清单文件（每行一项，`#` 注释）。"""
    _path = os.path.join(_RUNTIME_JAVA_RUNTIME, name)
    if not os.path.exists(_path):
        return []
    with open(_path, encoding='utf-8') as _f:
        return [_l.strip() for _l in _f if _l.strip() and not _l.lstrip().startswith('#')]


# VM 耦合边界类：公开包里由 JVM 自身引导 / 承载 VM 设施（模块系统、类加载、安全管理器等）的类。
# 它们在原生二进制里没有字节码层面的对应物，与内部包同规则：BFS 在此截断，整体手写、按需实现。
# 清单在 runtime/（手写层真源）维护，生成器不出现任何 JDK 类名。
_VM_BOUNDARY_CLASSES: frozenset[str] = frozenset(_read_manifest('vm_boundary.txt'))


def _is_boundary_class(cls: str) -> bool:
    """内部包（前缀）或 VM 耦合边界类（清单，含其嵌套类）。"""
    if cls.startswith(_JDK_STUB_ONLY_PREFIXES):
        return True
    return cls.split('$', 1)[0] in _VM_BOUNDARY_CLASSES


# java_runtime 已手写实现的类：这些类不再由 jdk_classes 翻译，避免重复定义和命名冲突
_JAVA_RUNTIME_CLASSES: frozenset[str] = frozenset({
    _OBJECT_CLASS,
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
                print(f"      内部类: {fname[:-6]} (字段: {[f.name for f in inner_ci.fields]}, 方法: {[m.name for m in inner_ci.methods]})")
                class_infos.append(inner_ci)

    # 3. 方法级调用链 BFS 发现 JDK 类
    print(f"[3/4] 扫描 JDK 类引用...", end=' ', flush=True)
    jdk_class_infos, visited_methods, field_stubs = _discover_jdk_classes_method_level(
        class_infos, runtime_src=os.path.join(out_dir, 'java_runtime', 'src'))
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


def _desc_class_refs(desc: str) -> list[str]:
    """从方法/字段 descriptor 提取类引用（Lxxx/yyy; 形式，含数组元素类型）。"""
    return re.findall(r'L([^;]+);', desc or '')


def _collect_method_refs(instrs) -> tuple:
    """从指令注释中提取方法引用和字段所属类引用（仅 JDK 类）。

    Returns:
        (method_refs, field_classes, member_refs, boundary_refs, new_classes, static_field_refs):
          method_refs   - (cls, method, descriptor) 三元组，用于 BFS 展开
          field_classes - 通过 getstatic/Field 指令发现的类名，只生成存根不展开方法体
          member_refs   - (cls, member) 被触达的成员（方法含内部边界类的方法、字段），
                          用于查询手写实现声明的 Java 回调（见 native_upcalls.py）
          static_field_refs - getstatic/putstatic 的 (cls, field)：类初始化触发点（JVMS §5.5）
    """

    def _add_type_refs(desc: str) -> None:
        """T88：签名类型引用进闭包（type-only）。

        被调方法的参数/返回类型、字段声明类型里的接口（如
        AbstractCollection.iterator() 的 Ljava/util/Iterator;）若不进闭包，
        签名引用的接口类型不会生成 → 名字落到 prelude trait 报 E0782。
        走 field_classes 通道：只生成类型存根，不展开方法体。
        """
        for tcls in _desc_class_refs(desc):
            if _is_boundary_class(tcls):
                field_classes.append(tcls)
            elif tcls.startswith(_JDK_PREFIXES):
                field_classes.append(tcls)

    method_refs = []
    field_classes = []
    member_refs = []
    boundary_refs = []   # 以内部边界类为常量池类的方法引用（边界类型上的虚调用目标）
    new_classes = []     # new 指令 / 构造器引用实例化的类
    static_field_refs = []
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
                if '[' not in cls:
                    member_refs.append((cls, meth))
                # stub-only 优先检查（java/security/ 等是 java/ 的子前缀，必须先匹配）
                if _is_boundary_class(cls) and '[' not in cls:
                    field_classes.append(cls)
                    boundary_refs.append((cls, meth, desc))
                elif cls.startswith(_JDK_PREFIXES) and '[' not in cls:
                    method_refs.append((cls, meth, desc))
                _add_type_refs(desc)
        elif c.startswith('InvokeDynamic '):
            # "InvokeDynamic samName:dynDesc impl:Cls.method:implDesc samtype:samDesc"
            # lambda / 方法引用的实现方法由闭包直接调用，属于调用链的一部分
            for tok in c.split(' ')[2:]:
                if not tok.startswith('impl:'):
                    continue
                rest = tok[5:]
                colon = rest.find(':')
                dot = rest.rfind('.', 0, colon) if colon > 0 else -1
                if dot > 0:
                    cls = rest[:dot]
                    meth = rest[dot+1:colon]
                    desc = rest[colon+1:]
                    if '[' not in cls:
                        member_refs.append((cls, meth))
                    if _is_boundary_class(cls) and '[' not in cls:
                        field_classes.append(cls)
                    elif cls.startswith(_JDK_PREFIXES) and '[' not in cls:
                        method_refs.append((cls, meth, desc))
                        if meth == '<init>':
                            new_classes.append(cls)   # 构造器引用 X::new
                    _add_type_refs(desc)
        elif c.startswith('Field '):
            # "Field java/nio/charset/CodingErrorAction.REPLACE:Ljava/nio/charset/CodingErrorAction;"
            # getstatic/putstatic/getfield/putfield - 只发现声明类，不展开其方法体
            rest = c[6:]  # 去掉 "Field "
            dot = rest.find('.')
            if dot > 0:
                cls = rest[:dot]
                if (cls.startswith(_JDK_PREFIXES) or cls.startswith(_JDK_STUB_ONLY_PREFIXES)) and '[' not in cls:
                    field_classes.append(cls)
                    if instr.opcode in ('getstatic', 'putstatic'):
                        _fend = rest.find(':', dot)
                        static_field_refs.append((cls, rest[dot+1:_fend if _fend > dot else len(rest)]))
                colon = rest.find(':', dot)
                if colon > dot:
                    if '[' not in cls:
                        member_refs.append((cls, rest[dot+1:colon]))
                    _add_type_refs(rest[colon+1:])
        elif c.startswith(_JDK_PREFIXES + _JDK_STUB_ONLY_PREFIXES) and '[' not in c and _is_boundary_class(c.split()[0]):
            # stub-only 内部类的 new/checkcast 指令 → 仅生成存根，不展开方法体
            cls = c.split()[0]
            field_classes.append(cls)
        elif c.startswith(_JDK_PREFIXES) and '[' not in c:
            # new / checkcast / instanceof / anewarray: comment = class binary name
            cls = c.split()[0]
            if instr.opcode == 'new':
                # 只有 new 产生该类的运行期实例（RTA 的实例化集合）；
                # 构造器本身由紧随其后的 invokespecial <init> 方法引用入队
                new_classes.append(cls)
            field_classes.append(cls)
    return method_refs, field_classes, member_refs, boundary_refs, new_classes, static_field_refs


def _discover_jdk_classes_method_level(class_infos: list, runtime_src: str | None = None) -> list:
    """方法级调用链 BFS：只追踪实际被调用的方法，不展开未调用方法的依赖类。

    调用边的三个来源：
      1. 字节码方法引用，按 JVMS §5.4.3.3 解析到声明者（父类链 → 父接口 default）
      2. 虚调用的运行期目标（RTA）：已实例化类（其 <init> 在调用链上）对虚调用目标的覆盖版本
      3. 手写 native / 内部边界方法声明的 Java 回调（runtime_src 下共置 `_impl.rs` 的 upcalls）
    """
    from .classfile import parse_class_bytes
    from .jdk_resolver import JdkResolver
    from .native_upcalls import NativeUpcalls

    upcalls = NativeUpcalls(runtime_src) if runtime_src else None
    # 以根类为常量池类的虚方法引用 (name, descriptor)：根类手写、不入 visited_methods
    root_virtual_targets: set[tuple[str, str]] = set()
    # 以内部边界类（接口/抽象类）为常量池类的方法引用：边界类自身手写、不入 visited_methods，
    # 但其公开 API 包内的已实例化子类型（如匿名访问器类）的覆盖版本在调用链上
    boundary_virtual_targets: set[tuple[str, str, str]] = set()

    visited_methods: set[tuple[str, str, str]] = set()
    queue: deque[tuple[str, str, str]] = deque()
    # 通过 getstatic/Field 指令发现的类：只生成存根，不展开方法体
    field_discover_classes: set[str] = set()

    def _enqueue_desc_types(desc: str) -> None:
        """T88：方法描述符的参数/返回类型也是类型依赖（type-only）。

        abstract 方法无方法体（instrs 为空），其签名引用的接口类型
        （如 AbstractCollection.iterator() 的 Ljava/util/Iterator;）若不收集
        则不会进闭包 → 签名引用的名字落到 prelude trait 报 E0782。
        走 field_discover_classes 通道：只生成类型存根，不展开方法体。
        与 trace_callchain.py 的 _extract_desc_classes 思路对齐。
        """
        for tcls in _desc_class_refs(desc):
            if (tcls not in _JAVA_RUNTIME_CLASSES
                    and (tcls.startswith(_JDK_PREFIXES)
                         or tcls.startswith(_JDK_STUB_ONLY_PREFIXES))):
                field_discover_classes.add(tcls)

    # 类初始化（JVMS §5.5）已入队的类
    init_enqueued: set[str] = set()
    # static 字段访问触发点：(常量池类, 字段名)，待 resolver 可用后解析到声明类
    pending_static_fields: deque[tuple[str, str]] = deque()

    # 调用链溯源：每个入队方法记录其来源方法（JAVA_RTA_BFS_TRACE=<类 binary name> 时输出入链路径）
    enqueued_from: dict[tuple[str, str, str], tuple[str, str, str] | None] = {}
    origin: list = [None]

    seen_members: set[tuple[str, str]] = set()
    instantiated_classes: set[str] = set()   # RTA：调用链上被 new 出来的类

    def _enqueue_method(key: tuple[str, str, str]) -> None:
        if key[0] in _JAVA_RUNTIME_CLASSES:
            return
        if key not in visited_methods:
            visited_methods.add(key)
            enqueued_from[key] = origin[0]
            queue.append(key)

    def _enqueue_upcalls(cls: str, member: str) -> None:
        """被触达成员的手写实现声明的 Java 回调目标入队（native → Java 的调用边）。"""
        if upcalls is None or (cls, member) in seen_members:
            return
        seen_members.add((cls, member))
        for tcls, tmeth, tdesc in upcalls.lookup(cls, member):
            # 回调目标自身也是被触达的成员：其手写实现可继续声明回调
            _enqueue_upcalls(tcls, tmeth)
            if tcls == _OBJECT_CLASS:
                if tmeth not in ('<init>', '<clinit>'):
                    root_virtual_targets.add((tmeth, tdesc))
            elif _is_boundary_class(tcls):
                if tcls not in _JAVA_RUNTIME_CLASSES:
                    field_discover_classes.add(tcls)
            elif tcls.startswith(_JDK_PREFIXES):
                _enqueue_method((tcls, tmeth, tdesc))
                if tmeth == '<init>':
                    instantiated_classes.add(tcls)   # 手写实现构造的 Java 对象
            _enqueue_desc_types(tdesc)

    def enqueue_refs(instrs, exception_table=()):
        (method_refs, f_classes, member_refs, boundary_refs, new_classes,
         static_refs) = _collect_method_refs(instrs)
        boundary_virtual_targets.update(boundary_refs)
        instantiated_classes.update(new_classes)
        pending_static_fields.extend(static_refs)
        # 异常表的 catch_type：catch 分派按类层次匹配，只需类型存根（不引入任何方法）
        for _entry in (exception_table or ()):
            _catch = _entry[3]
            if _catch and _catch.startswith(_JDK_PREFIXES + _JDK_STUB_ONLY_PREFIXES):
                f_classes.append(_catch)
        for cls in f_classes:
            if cls not in _JAVA_RUNTIME_CLASSES:
                field_discover_classes.add(cls)
        for key in method_refs:
            if key[0] == _OBJECT_CLASS and key[1] not in ('<init>', '<clinit>'):
                # 根类虚方法（toString/hashCode/equals…）：实际目标是已实例化类的覆盖版本
                root_virtual_targets.add((key[1], key[2]))
            _enqueue_method(key)
        for cls, member in member_refs:
            _enqueue_upcalls(cls, member)

    # 初始种子：用户类所有方法的引用
    for ci in class_infos:
        for m in ci.methods:
            enqueue_refs(m.instrs or [], m.exception_table)
            # T88：用户方法自身描述符里的参数/返回类型也是类型依赖
            # （abstract 方法无 instrs，但其签名引用的接口类型要进闭包）
            _enqueue_desc_types(m.descriptor)

    class_cache: dict[str, object] = {}
    jdk_infos: dict[str, object] = {}

    try:
        resolver = JdkResolver()
    except RuntimeError as e:
        print(f"      警告：{e}，跳过 JDK 元数据生成")
        return [], set(), set()

    def _load_class(name: str):
        """按需解析类（不加入生成范围，仅供方法解析沿继承层次查找）。"""
        if name not in class_cache:
            _data = resolver.resolve(name)
            try:
                class_cache[name] = parse_class_bytes(_data, name) if _data is not None else None
            except Exception:
                class_cache[name] = None
        return class_cache[name]

    def _enqueue_declaring_method(ci, meth: str, desc: str) -> None:
        """JVM 方法解析（JVMS §5.4.3.3）的接口分支：常量池类及其父类链都未声明
        (meth, desc) 时，实际执行的是父接口的 default 方法。把声明接口的该方法入队，
        使接口进入 registry、default 方法体注入实现类并进入 vtable。
        abstract 声明无方法体，不入队（不为无体方法扩大生成范围）。
        """
        if meth in ('<init>', '<clinit>'):
            return
        _chain = []
        _cur = ci
        _seen: set[str] = set()
        while _cur is not None and _cur.name not in _seen:
            _seen.add(_cur.name)
            _chain.append(_cur)
            _sc = _cur.super_class
            if not _sc or _sc in _JAVA_RUNTIME_CLASSES:
                break
            _cur = _load_class(_sc)
        # 本类及父类链任一处声明了 (meth, desc) → 由类层次自身承载，不属于 default 方法解析
        for _anc in _chain[1:]:
            if any(m.name == meth and m.descriptor == desc for m in _anc.methods):
                # JVMS §5.4.3.3 步骤 2：实际执行的是最近祖先类声明的方法 → 该方法入队
                if _is_boundary_class(_anc.name):
                    field_discover_classes.add(_anc.name)
                else:
                    _enqueue_method((_anc.name, meth, desc))
                return
        _owner = None
        if _owner is None:
            _iq = deque(i for _c in _chain for i in (_c.interfaces or []))
            _iseen: set[str] = set()
            _via: dict[str, str] = {}   # 接口 → 经由哪个子接口到达（回溯继承路径用）
            while _iq and _owner is None:
                _in = _iq.popleft()
                if _in in _iseen:
                    continue
                _iseen.add(_in)
                _ici = _load_class(_in)
                if _ici is None:
                    continue
                _hit = next((m for m in _ici.methods
                             if m.name == meth and m.descriptor == desc and not m.is_abstract), None)
                if _hit is not None:
                    _owner = (_ici, _hit)
                    break
                for _sup in (_ici.interfaces or []):
                    _via.setdefault(_sup, _in)
                    _iq.append(_sup)
        if _owner is None:
            return
        # 实现类 → 声明接口之间的中间接口必须进 registry（仅类型别名级 stub），
        # 否则 default 方法注入沿 interfaces 向上遍历时在缺失节点处断链。
        _step = _via.get(_owner[0].name)
        while _step is not None:
            if (_step not in _JAVA_RUNTIME_CLASSES
                    and not _is_boundary_class(_step)):
                field_discover_classes.add(_step)
            _step = _via.get(_step)
        _oci, _om = _owner
        if _om.is_abstract or _om.is_static or _oci.name in _JAVA_RUNTIME_CLASSES:
            return
        if _is_boundary_class(_oci.name):
            field_discover_classes.add(_oci.name)
            return
        _key = (_oci.name, meth, desc)
        if _key not in visited_methods:
            visited_methods.add(_key)
            queue.append(_key)

    def _translatable(name: str) -> bool:
        return (bool(name) and name not in _JAVA_RUNTIME_CLASSES
                and name.startswith(_JDK_PREFIXES)
                and not _is_boundary_class(name))

    def _enqueue_class_init(name: str) -> None:
        """类初始化触发（getstatic / putstatic / invokestatic / new）：该类及其父类链的
        <clinit> 进入调用链。内部边界类无字节码翻译，在边界处截断。"""
        _cur = name
        while _translatable(_cur) and _cur not in init_enqueued:
            init_enqueued.add(_cur)
            _ci = _load_class(_cur)
            if _ci is None:
                return
            if any(m.name == '<clinit>' for m in _ci.methods):
                _key = (_cur, '<clinit>', '()V')
                if _key not in visited_methods:
                    visited_methods.add(_key)
                    enqueued_from[_key] = origin[0]
                    queue.append(_key)
            _cur = _ci.super_class

    def _static_field_owner(name: str, field: str) -> str:
        """JVMS §5.4.3.2 字段解析：本类 → 父接口 → 父类，返回声明类。"""
        _seen: set[str] = set()
        _stack = [name]
        while _stack:
            _n = _stack.pop(0)
            if not _n or _n in _seen:
                continue
            _seen.add(_n)
            _ci = _load_class(_n) if _translatable(_n) else None
            if _ci is None:
                continue
            if any(f.is_static and f.name == field for f in _ci.fields):
                return _n
            _stack.extend(_ci.interfaces or [])
            _stack.append(_ci.super_class)
        return name

    def _drain_static_fields() -> None:
        while pending_static_fields:
            _cls, _field = pending_static_fields.popleft()
            _enqueue_class_init(_static_field_owner(_cls, _field))

    def _load_vm_roots() -> list[tuple[str, str, str]]:
        """VM 根方法清单：手写运行时直接调用的已翻译方法（见清单文件头注释）。"""
        _roots = []
        for _line in _read_manifest('vm_roots.txt'):
            _owner, _, _desc = _line.partition(':')
            _cls, _, _meth = _owner.rpartition('.')
            _roots.append((_cls, _meth, _desc))
        return _roots

    def _process(cls: str, meth: str, desc: str) -> None:
        # 解析类（首次遇到时）
        ci = _load_class(cls)
        if ci is None:
            return
        if cls not in jdk_infos:
            jdk_infos[cls] = ci
        origin[0] = (cls, meth, desc)

        # 追踪该方法的指令引用（精确匹配名字+描述符，避免重载方法误展开）
        _declared = False
        for m in ci.methods:
            if m.name == meth and m.descriptor == desc:
                _declared = True
                enqueue_refs(m.instrs or [], m.exception_table)
                # T88：被调方法的描述符参数/返回类型也是类型依赖
                # （abstract/native 方法无 instrs，签名引用的接口类型
                # 如 iterator()Ljava/util/Iterator; 仍需进闭包生成）
                _enqueue_desc_types(m.descriptor)
                # invokestatic / new 是类初始化触发点
                if m.is_static or m.name == '<init>':
                    _enqueue_class_init(cls)
        if not _declared:
            _enqueue_declaring_method(ci, meth, desc)
        _drain_static_fields()

    with resolver:
        for _root in _load_vm_roots():
            if _root not in visited_methods:
                visited_methods.add(_root)
                queue.append(_root)
        # 用户类的父类链：用户类初始化先初始化其 JDK 父类
        for _uci in class_infos:
            _enqueue_class_init(_uci.super_class)
        _drain_static_fields()

        # 根类的方法被所有类继承（手写 ObjectVTable 的签名与字节码一致），
        # 其描述符中的参数/返回类型是全局类型依赖，必须进闭包
        _root_data = resolver.resolve(_OBJECT_CLASS)
        if _root_data is not None:
            try:
                for _rm in parse_class_bytes(_root_data, _OBJECT_CLASS).methods:
                    _enqueue_desc_types(_rm.descriptor)
            except Exception:
                pass


        _supertype_cache: dict[str, frozenset] = {}

        def _supertypes(name: str) -> frozenset:
            """name 的全部超类型（含自身、父类链、传递父接口）。"""
            if name in _supertype_cache:
                return _supertype_cache[name]
            _supertype_cache[name] = frozenset({name})   # 环保护
            acc = {name}
            _ci = _load_class(name) if name not in _JAVA_RUNTIME_CLASSES else None
            if _ci is not None:
                for _up in ([_ci.super_class] if _ci.super_class else []) + list(_ci.interfaces or []):
                    acc |= _supertypes(_up)
            _supertype_cache[name] = frozenset(acc)
            return _supertype_cache[name]

        _ACC_PRIVATE, _ACC_FINAL = 0x0002, 0x0010

        def _propagate_virtual_targets() -> None:
            """虚调用目标 → 运行期实际接收者类的覆盖版本。

            - 接口方法：闭包内直接实现该接口的具体类
            - RTA：已实例化类 X（(X, <init>, *) 在调用链上）是虚调用目标声明类的子类型时，
              (X, m, d) 入队；X 未声明则由 _process 按方法解析规则落到最近声明者
            """
            instantiated = sorted(
                x for x in instantiated_classes
                if class_cache.get(x) is not None
                and not class_cache[x].is_interface and not class_cache[x].is_abstract
            )
            for cls, meth, desc in list(visited_methods):
                if meth in ('<init>', '<clinit>'):
                    continue
                ci = class_cache.get(cls)
                if ci is None:
                    continue
                decl = next((m for m in ci.methods
                             if m.name == meth and m.descriptor == desc), None)
                if decl is None or decl.is_static or (decl.access_flags & (_ACC_PRIVATE | _ACC_FINAL)):
                    continue
                if ci.is_interface:
                    for concrete_name, concrete_ci in list(jdk_infos.items()):
                        if concrete_ci is None or concrete_ci.is_interface:
                            continue
                        if cls in (concrete_ci.interfaces or []):
                            _enqueue_method((concrete_name, meth, desc))
                for x in instantiated:
                    if x != cls and cls in _supertypes(x):
                        _enqueue_method((x, meth, desc))
            for meth, desc in sorted(root_virtual_targets):
                for x in instantiated:
                    _enqueue_method((x, meth, desc))
            for cls, meth, desc in sorted(boundary_virtual_targets):
                if meth in ('<init>', '<clinit>'):
                    continue
                for x in instantiated:
                    if cls in _supertypes(x):
                        _enqueue_method((x, meth, desc))

        # 不动点：排空队列 → 传播虚调用目标 → 有新方法则继续
        while True:
            while queue:
                _process(*queue.popleft())
            _propagate_virtual_targets()
            if not queue:
                break

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
                if (ci.super_class and ci.super_class != _OBJECT_CLASS
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
            if (_ci and _ci.super_class and _ci.super_class != _OBJECT_CLASS
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
                if (ci.super_class and ci.super_class != _OBJECT_CLASS
                        and ci.super_class not in _JAVA_RUNTIME_CLASSES
                        and ci.super_class not in jdk_infos):
                    _parent_queue.append(ci.super_class)
            except Exception:
                pass

    _trace_cls = os.environ.get('JAVA_RTA_BFS_TRACE')
    if _trace_cls:
        for _key in sorted(k for k in visited_methods if k[0] == _trace_cls):
            _path = []
            _step = _key
            while _step is not None and len(_path) < 64:
                _path.append(f"{_step[0]}.{_step[1]}:{_step[2]}")
                _step = enqueued_from.get(_step)
            print("      [bfs-trace] " + "\n          <- ".join(_path))

    return list(jdk_infos.values()), visited_methods, field_discover_classes
