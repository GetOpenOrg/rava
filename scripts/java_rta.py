"""
java_rta.py  ——  TeaVM 风格 Java 可达性分析器

原理：模拟 TeaVM Dependency Checker 的核心算法
  1. 解析 Java 源码 AST（javalang）
  2. 收集所有类的方法定义与方法体内的调用关系
  3. 从入口点 main() 出发，执行 BFS/DFS 可达性分析
  4. 用 RTA（Reaching Type Analysis）处理虚方法：
     只有被 NEW 实例化过的类，其方法覆写才被纳入可达集合
  5. 打印调用链树

用法：
    python3 java_rta.py HelloWorld.java
    python3 java_rta.py HelloWorld.java --json      # 输出 JSON 供前端使用
    python3 java_rta.py HelloWorld.java --entry MyClass.myMethod  # 自定义入口
"""

import sys
import json
import argparse
from pathlib import Path
from collections import defaultdict
from dataclasses import dataclass, field
from typing import Optional

try:
    import tomllib          # Python 3.11+
except ImportError:
    try:
        import tomli as tomllib  # pip install tomli
    except ImportError:
        tomllib = None      # cutoff.toml 加载失败时优雅降级

import javalang
import javalang.tree as T


# ──────────────────────────────────────────────
# 数据结构
# ──────────────────────────────────────────────

@dataclass
class MethodRef:
    """方法的唯一标识：类名 + 方法名 + 参数数量（简化签名）"""
    class_name: str
    method_name: str
    param_count: int = -1          # -1 表示未知

    def key(self) -> str:
        pc = f"/{self.param_count}" if self.param_count >= 0 else ""
        return f"{self.class_name}#{self.method_name}{pc}"

    def short(self) -> str:
        return f"{self.class_name}.{self.method_name}()"

    def __hash__(self):
        return hash(self.key())

    def __eq__(self, other):
        return self.key() == other.key()

    def __repr__(self):
        return self.key()


@dataclass
class CallEdge:
    """一条调用边：caller → callee，附带调用类型"""
    caller: MethodRef
    callee: MethodRef
    call_type: str = "static"      # static | virtual | constructor | jdk


@dataclass
class ClassInfo:
    """类的元信息"""
    name: str
    superclass: Optional[str] = None
    interfaces: list = field(default_factory=list)
    methods: dict = field(default_factory=dict)      # method_name -> MethodInfo
    is_jdk: bool = False


@dataclass
class MethodInfo:
    """方法元信息 + 方法体内的调用列表"""
    name: str
    class_name: str
    param_count: int = 0
    calls: list = field(default_factory=list)        # list[CallEdge]


# ──────────────────────────────────────────────
# JDK 已知 API 数据库（仅覆盖常用类，用于链路显示）
# ──────────────────────────────────────────────

JDK_KNOWN = {
    # class -> {method -> [calls]}
    "System": {
        "out": [],
        "err": [],
        "exit": [],
        "currentTimeMillis": [],
        "arraycopy": [],
    },
    "PrintStream": {
        "println":    ["java.io.PrintStream#write", "java.io.PrintStream#newLine"],
        "print":      ["java.io.PrintStream#write"],
        "printf":     ["java.io.PrintStream#format"],
        "format":     [],
        "flush":      [],
        "close":      [],
    },
    "String": {
        "valueOf":    [],
        "length":     [],
        "charAt":     [],
        "substring":  [],
        "indexOf":    [],
        "contains":   [],
        "replace":    [],
        "split":      [],
        "trim":       [],
        "toUpperCase": [],
        "toLowerCase": [],
        "equals":     ["Object#equals"],
        "hashCode":   ["Object#hashCode"],
        "toString":   ["Object#toString"],
        "format":     [],
        "concat":     [],
        "isEmpty":    [],
        "startsWith": [],
        "endsWith":   [],
        "intern":     [],
        "<init>":     [],
    },
    "StringBuilder": {
        "<init>":  [],
        "append":  [],
        "toString": [],
        "length":  [],
        "insert":  [],
        "delete":  [],
        "reverse": [],
    },
    "Object": {
        "<init>":   [],
        "toString": [],
        "equals":   [],
        "hashCode": [],
        "getClass": [],
        "clone":    [],
        "wait":     [],
        "notify":   [],
        "notifyAll": [],
    },
    "Integer": {
        "parseInt":   [],
        "valueOf":    [],
        "toString":   [],
        "intValue":   [],
        "compareTo":  [],
        "MAX_VALUE":  [],
        "MIN_VALUE":  [],
    },
    "Math": {
        "abs":   [],
        "max":   [],
        "min":   [],
        "sqrt":  [],
        "pow":   [],
        "floor": [],
        "ceil":  [],
        "round": [],
        "random": [],
        "log":   [],
    },
    "ArrayList": {
        "<init>": [],
        "add":    ["AbstractList#add", "ArrayList#ensureCapacityInternal"],
        "get":    ["ArrayList#rangeCheck"],
        "size":   [],
        "remove": ["ArrayList#fastRemove"],
        "clear":  [],
        "contains": ["AbstractList#contains"],
        "isEmpty":  [],
        "iterator": ["ArrayList#Itr.<init>"],
        "toArray":  [],
        "set":      [],
    },
    "HashMap": {
        "<init>": [],
        "put":    ["HashMap#putVal", "HashMap#hash"],
        "get":    ["HashMap#getNode", "HashMap#hash"],
        "containsKey": ["HashMap#getNode"],
        "remove": ["HashMap#removeNode"],
        "size":   [],
        "isEmpty": [],
        "keySet": ["HashMap#KeySet.<init>"],
        "values": ["HashMap#Values.<init>"],
    },
    "Arrays": {
        "sort":      [],
        "copyOf":    [],
        "asList":    [],
        "fill":      [],
        "toString":  [],
        "equals":    [],
        "binarySearch": [],
    },
    "Collections": {
        "sort":           [],
        "unmodifiableList": [],
        "singletonList":  [],
        "emptyList":      [],
        "reverse":        [],
        "shuffle":        [],
    },
}

# 常用类的完整 JDK 路径映射
CLASS_PACKAGE = {
    "System":       "java.lang",
    "String":       "java.lang",
    "StringBuilder": "java.lang",
    "Object":       "java.lang",
    "Integer":      "java.lang",
    "Long":         "java.lang",
    "Double":       "java.lang",
    "Boolean":      "java.lang",
    "Character":    "java.lang",
    "Float":        "java.lang",
    "Short":        "java.lang",
    "Byte":         "java.lang",
    "Math":         "java.lang",
    "Runtime":      "java.lang",
    "Thread":       "java.lang",
    "Runnable":     "java.lang",
    "Exception":    "java.lang",
    "RuntimeException": "java.lang",
    "PrintStream":  "java.io",
    "InputStream":  "java.io",
    "OutputStream": "java.io",
    "BufferedReader": "java.io",
    "FileReader":   "java.io",
    "File":         "java.io",
    "ArrayList":    "java.util",
    "HashMap":      "java.util",
    "HashSet":      "java.util",
    "LinkedList":   "java.util",
    "Arrays":       "java.util",
    "Collections":  "java.util",
    "List":         "java.util",
    "Map":          "java.util",
    "Set":          "java.util",
    "Iterator":     "java.util",
    "Optional":     "java.util",
}


# ──────────────────────────────────────────────
# 阶段 1：AST 解析，构建类 → 方法 → 调用 的完整图
# ──────────────────────────────────────────────

class ASTParser:
    """从 javalang AST 中提取类信息、方法信息和调用关系"""

    def __init__(self):
        self.classes: dict[str, ClassInfo] = {}
        # 全局 NEW 实例化集合（用于 RTA）
        self.instantiated: set[str] = set()

    def parse_file(self, source: str):
        tree = javalang.parse.parse(source)
        for _, cls_node in tree.filter(T.ClassDeclaration):
            self._parse_class(cls_node)

    def _parse_class(self, cls_node: T.ClassDeclaration):
        cname = cls_node.name
        superclass = None
        if cls_node.extends:
            superclass = cls_node.extends.name

        interfaces = []
        if cls_node.implements:
            for iface in cls_node.implements:
                interfaces.append(iface.name)

        ci = ClassInfo(
            name=cname,
            superclass=superclass,
            interfaces=interfaces,
        )
        self.classes[cname] = ci

        # 解析方法
        for method_node in cls_node.methods:
            mi = self._parse_method(cname, method_node)
            ci.methods[method_node.name] = mi

        # 解析构造器
        for ctor_node in cls_node.constructors:
            mi = self._parse_constructor(cname, ctor_node)
            key = f"<init>"
            if key not in ci.methods:
                ci.methods[key] = mi
            # 多个构造器：后缀区分
            ci.methods[f"<init>/{len(ctor_node.parameters)}"] = mi

    def _parse_method(self, class_name: str, node: T.MethodDeclaration) -> MethodInfo:
        mi = MethodInfo(
            name=node.name,
            class_name=class_name,
            param_count=len(node.parameters) if node.parameters else 0,
        )
        caller_ref = MethodRef(class_name, node.name, mi.param_count)

        # 局部变量类型映射（用于 qualifier 解析）
        var_types: dict[str, str] = {}
        if node.parameters:
            for p in node.parameters:
                var_types[p.name] = p.type.name

        # 从方法体收集调用和 NEW
        if node.body:
            self._collect_from_body(node.body, caller_ref, mi, var_types)

        return mi

    def _parse_constructor(self, class_name: str, node: T.ConstructorDeclaration) -> MethodInfo:
        mi = MethodInfo(
            name="<init>",
            class_name=class_name,
            param_count=len(node.parameters) if node.parameters else 0,
        )
        caller_ref = MethodRef(class_name, "<init>", mi.param_count)
        var_types: dict[str, str] = {}
        if node.parameters:
            for p in node.parameters:
                var_types[p.name] = p.type.name
        if node.body:
            self._collect_from_body(node.body, caller_ref, mi, var_types)
        return mi

    def _collect_from_body(self, body, caller_ref: MethodRef,
                           mi: MethodInfo, var_types: dict):
        """递归遍历方法体 AST，提取调用边和 NEW 指令"""

        def walk(nodes):
            if nodes is None:
                return
            if isinstance(nodes, list):
                for n in nodes:
                    walk(n)
                return
            if not isinstance(nodes, T.Node):
                return

            # ── NEW 指令 ──
            if isinstance(nodes, T.ClassCreator):
                created = nodes.type.name
                self.instantiated.add(created)
                callee_ref = MethodRef(created, "<init>",
                                       len(nodes.arguments) if nodes.arguments else 0)
                edge = CallEdge(caller=caller_ref, callee=callee_ref,
                                call_type="constructor")
                mi.calls.append(edge)
                # 继续走参数
                walk(nodes.arguments)
                return

            # ── 局部变量声明（收集变量类型）──
            if isinstance(nodes, T.LocalVariableDeclaration):
                type_name = nodes.type.name if hasattr(nodes.type, 'name') else str(nodes.type)
                for decl in nodes.declarators:
                    var_types[decl.name] = type_name
                # 继续走右值
                walk(nodes.declarators)
                return

            # ── 方法调用 ──
            if isinstance(nodes, T.MethodInvocation):
                self._resolve_call(nodes, caller_ref, mi, var_types)
                walk(nodes.arguments)
                walk(getattr(nodes, 'selectors', None))
                return

            # 递归子节点
            for attr in nodes.attrs:
                walk(getattr(nodes, attr, None))

        walk(body)

    def _resolve_call(self, node: T.MethodInvocation,
                      caller_ref: MethodRef, mi: MethodInfo,
                      var_types: dict):
        """
        解析方法调用的目标类和调用类型

        qualifier 有几种情况：
          - None / ''          → this.method() 或同类静态方法
          - 'ClassName'        → 静态调用（如 String.valueOf）
          - 'varName'          → 虚调用，类型从 var_types 查
          - 'ClassName.field'  → 字段访问后调用（如 System.out.println）
        """
        q = node.qualifier or ""
        method = node.member
        args_n = len(node.arguments) if node.arguments else 0

        target_class = None
        call_type = "virtual"

        if not q:
            # this 调用 / 同类静态
            target_class = caller_ref.class_name
            call_type = "static"

        elif q == "super":
            # super() 调用
            target_class = self._get_superclass(caller_ref.class_name)
            call_type = "virtual"

        else:
            parts = q.split(".")
            # System.out → 特殊处理：System 是类，out 是字段
            if len(parts) == 2 and parts[0] in self.classes or parts[0] in JDK_KNOWN:
                first = parts[0]
                # 判断 first 是否是已知类
                if first in JDK_KNOWN or first in self.classes:
                    target_class = self._resolve_field_type(first, parts[1])
                    call_type = "static" if first in JDK_KNOWN else "virtual"
                else:
                    target_class = first
                    call_type = "static"

            elif len(parts) == 1:
                v = parts[0]
                # 可能是变量名或类名
                if v in var_types:
                    # 变量 → 虚调用
                    target_class = var_types[v]
                    call_type = "virtual"
                elif v[0].isupper():
                    # 首字母大写 → 类名静态调用
                    target_class = v
                    call_type = "static"
                else:
                    # 小写变量但未声明（可能是字段）
                    target_class = caller_ref.class_name
                    call_type = "virtual"
            else:
                # 复杂 qualifier，取最后一段
                target_class = q
                call_type = "virtual"

        if not target_class:
            target_class = caller_ref.class_name

        is_jdk = (target_class in JDK_KNOWN or
                  target_class in CLASS_PACKAGE and target_class not in self.classes)

        callee_ref = MethodRef(target_class, method, args_n)
        ct = "jdk" if is_jdk else call_type
        edge = CallEdge(caller=caller_ref, callee=callee_ref, call_type=ct)
        mi.calls.append(edge)

    def _resolve_field_type(self, class_name: str, field_name: str) -> str:
        """推断字段的类型（System.out → PrintStream）"""
        field_type_map = {
            ("System", "out"): "PrintStream",
            ("System", "err"): "PrintStream",
            ("System", "in"):  "InputStream",
        }
        return field_type_map.get((class_name, field_name), class_name)

    def _get_superclass(self, class_name: str) -> Optional[str]:
        ci = self.classes.get(class_name)
        if ci:
            return ci.superclass or "Object"
        return "Object"


# ──────────────────────────────────────────────
# 阶段 2：RTA 可达性分析
# ──────────────────────────────────────────────

class RTAnalyzer:
    """
    从入口方法出发，BFS 展开调用图。
    虚方法规则：只有在 instantiated 集合里的类，其 override 才被认为可达。
    <clinit> 隔离：静态初始化方法只在类确实被实例化时才跟随。
    cutoff 截断：进入 cutoff.toml 中声明的类时停止追踪，记录为边界。
    """

    def __init__(self, parser: ASTParser, cutoff_classes: set[str] | None = None):
        self.parser = parser
        self.classes = parser.classes
        self.instantiated = parser.instantiated  # NEW 指令收集的类型集合

        # 可达方法集合（key() → MethodRef）
        self.reachable: dict[str, MethodRef] = {}
        # 调用边列表（用于输出调用树）
        self.call_edges: list[CallEdge] = []
        # caller → [callee] 邻接表
        self.call_graph: dict[str, list[str]] = defaultdict(list)
        # cutoff 边界集合（记录但不展开）
        self.cutoff_frontier: set[str] = set()
        # cutoff 截断类集合（JVM 内部名，如 java/util/ServiceLoader）
        self._cutoff_classes: set[str] = cutoff_classes or set()

    def _should_follow(self, callee: MethodRef, call_type: str) -> bool:
        """判断是否应继续追踪该被调用方法。"""
        # cutoff 截断：记录边界，不继续追踪
        cls_jvm = callee.class_name.replace('.', '/')
        if cls_jvm in self._cutoff_classes or callee.class_name in self._cutoff_classes:
            self.cutoff_frontier.add(callee.key())
            return False
        # <clinit> 隔离：只有类被 new 过才跟随
        if callee.method_name == '<clinit>':
            return callee.class_name in self.instantiated
        return True

    def analyze(self, entry_class: str, entry_method: str = "main"):
        entry = MethodRef(entry_class, entry_method)
        self._bfs(entry)

    def _bfs(self, start: MethodRef):
        queue = [start]
        self.reachable[start.key()] = start

        while queue:
            curr = queue.pop(0)
            method_info = self._lookup_method(curr)

            if method_info is None:
                continue

            for edge in method_info.calls:
                callee = edge.callee

                # cutoff / clinit 隔离检查
                if not self._should_follow(callee, edge.call_type):
                    continue

                # RTA 虚方法过滤
                if edge.call_type == "virtual":
                    resolved = self._resolve_virtual(callee)
                else:
                    resolved = [callee]

                for r in resolved:
                    self.call_edges.append(
                        CallEdge(curr, r, edge.call_type)
                    )
                    self.call_graph[curr.key()].append(r.key())

                    if r.key() not in self.reachable:
                        self.reachable[r.key()] = r
                        queue.append(r)

    def _lookup_method(self, ref: MethodRef) -> Optional[MethodInfo]:
        """在用户代码或 JDK 已知库中查找方法"""
        ci = self.classes.get(ref.class_name)
        if ci:
            mi = ci.methods.get(ref.method_name)
            if mi:
                return mi
            # 继承链查找
            parent = ci.superclass
            while parent:
                pci = self.classes.get(parent)
                if pci:
                    mi = pci.methods.get(ref.method_name)
                    if mi:
                        return mi
                    parent = pci.superclass
                else:
                    break

        # JDK 已知库：构造虚拟 MethodInfo
        if ref.class_name in JDK_KNOWN:
            jdk_class = JDK_KNOWN[ref.class_name]
            if ref.method_name in jdk_class:
                internal_calls = jdk_class[ref.method_name]
                mi = MethodInfo(
                    name=ref.method_name,
                    class_name=ref.class_name,
                )
                # 将内部调用字符串转换为 CallEdge
                for ic in internal_calls:
                    parts = ic.split("#")
                    if len(parts) == 2:
                        ic_ref = MethodRef(parts[0], parts[1])
                        mi.calls.append(
                            CallEdge(
                                caller=ref,
                                callee=ic_ref,
                                call_type="jdk"
                            )
                        )
                return mi
        return None

    def _resolve_virtual(self, ref: MethodRef) -> list[MethodRef]:
        """
        RTA 虚方法解析：
        只有 instantiated 中的类（及其子类）才作为候选实现
        """
        candidates = []

        # 检查 ref 本身（可能是已知类的直接调用）
        if ref.class_name in self.classes or ref.class_name in JDK_KNOWN:
            candidates.append(ref)

        # 查找 instantiated 中实现了该方法的类
        for inst_class in self.instantiated:
            if inst_class == ref.class_name:
                continue
            # 检查 inst_class 是否是 ref.class_name 的子类或实现类
            if self._implements_or_extends(inst_class, ref.class_name):
                override_ref = MethodRef(inst_class, ref.method_name, ref.param_count)
                if self._has_method(inst_class, ref.method_name):
                    candidates.append(override_ref)

        return candidates if candidates else [ref]

    def _implements_or_extends(self, child: str, parent: str) -> bool:
        """检查 child 是否是 parent 的子类/实现类"""
        ci = self.classes.get(child)
        if not ci:
            return False
        if ci.superclass == parent:
            return True
        if parent in ci.interfaces:
            return True
        if ci.superclass:
            return self._implements_or_extends(ci.superclass, parent)
        return False

    def _has_method(self, class_name: str, method_name: str) -> bool:
        ci = self.classes.get(class_name)
        if ci:
            return method_name in ci.methods
        if class_name in JDK_KNOWN:
            return method_name in JDK_KNOWN[class_name]
        return False


# ──────────────────────────────────────────────
# 阶段 3：调用链树输出
# ──────────────────────────────────────────────

CALL_TYPE_COLOR = {
    "static":      "\033[36m",   # 青色
    "virtual":     "\033[33m",   # 黄色
    "constructor": "\033[32m",   # 绿色
    "jdk":         "\033[35m",   # 紫色
}
RESET = "\033[0m"
BOLD = "\033[1m"
DIM = "\033[2m"


def build_tree(analyzer: RTAnalyzer, entry_key: str) -> dict:
    """构建调用树的字典结构（供 JSON 输出或递归打印）"""
    visited = set()

    def _build(key: str, depth: int = 0) -> dict:
        ref = analyzer.reachable.get(key)
        if not ref:
            # 可能是 JDK 方法
            parts = key.split("#")
            ref = MethodRef(parts[0], parts[1] if len(parts) > 1 else "?")

        node = {
            "key": key,
            "class": ref.class_name,
            "method": ref.method_name,
            "is_jdk": ref.class_name in JDK_KNOWN or (
                ref.class_name in CLASS_PACKAGE and
                ref.class_name not in analyzer.classes
            ),
            "children": []
        }

        if key in visited or depth > 12:
            node["truncated"] = True
            return node
        visited.add(key)

        callees = analyzer.call_graph.get(key, [])
        seen_children = set()
        for ckey in callees:
            if ckey not in seen_children:
                seen_children.add(ckey)
                # 找到对应的 call_type
                ct = "virtual"
                for e in analyzer.call_edges:
                    if e.caller.key() == key and e.callee.key() == ckey:
                        ct = e.call_type
                        break
                child = _build(ckey, depth + 1)
                child["call_type"] = ct
                node["children"].append(child)

        return node

    return _build(entry_key)


def print_tree(node: dict, prefix: str = "", is_last: bool = True, is_root: bool = True):
    """终端彩色调用链树打印"""
    connector = "" if is_root else ("└── " if is_last else "├── ")
    call_type = node.get("call_type", "static")

    # 颜色
    color = CALL_TYPE_COLOR.get(call_type, "")
    dim = DIM if node.get("is_jdk") else ""

    # 类型标签
    label = {
        "static":      "[static]",
        "virtual":     "[virtual]",
        "constructor": "[new]",
        "jdk":         "[JDK]",
    }.get(call_type, "")

    # 方法显示
    class_part = f"{dim}{node['class']}{RESET}"
    method_part = f"{BOLD}{node['method']}(){RESET}"
    tag = f" {color}{label}{RESET}"

    truncated = " ..." if node.get("truncated") else ""

    print(f"{prefix}{connector}{class_part}.{method_part}{tag}{truncated}")

    children = node.get("children", [])
    new_prefix = prefix + ("    " if is_root or is_last else "│   ")

    for i, child in enumerate(children):
        print_tree(child, new_prefix, i == len(children) - 1, False)


def print_summary(analyzer: RTAnalyzer):
    """打印可达集合摘要"""
    user_methods = [(r.class_name, r.method_name)
                    for r in analyzer.reachable.values()
                    if r.class_name in analyzer.classes]
    jdk_methods = [(r.class_name, r.method_name)
                   for r in analyzer.reachable.values()
                   if r.class_name not in analyzer.classes]

    print()
    print("━" * 60)
    print(f"{BOLD}📊 可达性分析摘要{RESET}")
    print("━" * 60)

    print(f"\n{BOLD}✦ 实例化类型集合（RTA NEW 指令收集）：{RESET}")
    for cls in sorted(analyzer.instantiated):
        pkg = CLASS_PACKAGE.get(cls, "用户代码")
        print(f"   • {cls}  {DIM}({pkg}){RESET}")

    print(f"\n{BOLD}✦ 用户代码可达方法（{len(user_methods)} 个）：{RESET}")
    by_class = defaultdict(list)
    for cls, mth in user_methods:
        by_class[cls].append(mth)
    for cls, methods in sorted(by_class.items()):
        print(f"   {cls}")
        for m in sorted(set(methods)):
            print(f"     └ {m}()")

    print(f"\n{BOLD}✦ JDK 可达方法（{len(jdk_methods)} 个）：{RESET}")
    jdk_by_class = defaultdict(list)
    for cls, mth in jdk_methods:
        jdk_by_class[cls].append(mth)
    for cls, methods in sorted(jdk_by_class.items()):
        pkg = CLASS_PACKAGE.get(cls, "?")
        print(f"   {DIM}{pkg}.{RESET}{cls}")
        for m in sorted(set(methods)):
            print(f"     {DIM}└ {m}(){RESET}")

    print()
    print(f"  总计：{len(analyzer.reachable)} 个可达方法")
    print()


# ──────────────────────────────────────────────
# 主入口
# ──────────────────────────────────────────────

def analyze(source_path: str,
            entry_class: str = None,
            entry_method: str = "main",
            output_json: bool = False) -> dict:

    source = Path(source_path).read_text(encoding="utf-8")

    # 阶段 1：解析 AST
    parser = ASTParser()
    parser.parse_file(source)

    if not parser.classes:
        raise ValueError("没有找到任何类定义")

    # 确定入口类
    if not entry_class:
        # 找包含 main 方法的类
        for cname, ci in parser.classes.items():
            if "main" in ci.methods:
                entry_class = cname
                break
        if not entry_class:
            entry_class = list(parser.classes.keys())[0]
            entry_method = list(parser.classes[entry_class].methods.keys())[0]

    if not output_json:
        print(f"\n{BOLD}🔍 TeaVM 风格 Java 可达性分析器{RESET}")
        print(f"  源文件：{source_path}")
        print(f"  入口：  {entry_class}.{entry_method}()")
        print(f"  实例化类型集合（从 NEW 指令收集）：", end="")

    # 阶段 2：RTA 分析（加载 cutoff.toml）
    cutoff_classes: set[str] = set()
    cutoff_path = Path(__file__).parent.parent / 'config' / 'cutoff.toml'
    if cutoff_path.exists() and tomllib is not None:
        with open(cutoff_path, 'rb') as _f:
            _cutoff_data = tomllib.load(_f)
        cutoff_classes = set(_cutoff_data.get('cutoff', {}).get('classes', []))
    analyzer = RTAnalyzer(parser, cutoff_classes=cutoff_classes)
    analyzer.analyze(entry_class, entry_method)

    if not output_json:
        print(", ".join(sorted(analyzer.instantiated)) or "(空)")

    # 阶段 3：构建并输出调用树
    entry_ref = MethodRef(entry_class, entry_method)
    tree = build_tree(analyzer, entry_ref.key())

    if output_json:
        result = {
            "entry": entry_ref.key(),
            "tree": tree,
            "reachable": [
                {
                    "key": r.key(),
                    "class": r.class_name,
                    "method": r.method_name,
                    "is_jdk": r.class_name not in analyzer.classes,
                    "package": CLASS_PACKAGE.get(r.class_name, "用户代码"),
                }
                for r in analyzer.reachable.values()
            ],
            "instantiated": sorted(analyzer.instantiated),
            "stats": {
                "total_reachable": len(analyzer.reachable),
                "user_methods": sum(
                    1 for r in analyzer.reachable.values()
                    if r.class_name in analyzer.classes
                ),
                "jdk_methods": sum(
                    1 for r in analyzer.reachable.values()
                    if r.class_name not in analyzer.classes
                ),
            }
        }
        return result

    # 终端输出
    print(f"\n{BOLD}📞 调用链树{RESET}  {DIM}（颜色图例：{RESET}" +
          f"{CALL_TYPE_COLOR['constructor']}[new]{RESET} " +
          f"{CALL_TYPE_COLOR['static']}[static]{RESET} " +
          f"{CALL_TYPE_COLOR['virtual']}[virtual]{RESET} " +
          f"{CALL_TYPE_COLOR['jdk']}[JDK]{RESET}{DIM}）{RESET}\n")
    print_tree(tree)
    print_summary(analyzer)

    return {"tree": tree}


def main():
    ap = argparse.ArgumentParser(
        description="TeaVM 风格 Java 可达性分析器（方法级 RTA）"
    )
    ap.add_argument("source", help=".java 源文件路径")
    ap.add_argument("--entry", help="入口方法，格式 ClassName.methodName，默认找 main()")
    ap.add_argument("--json", action="store_true", help="输出 JSON 格式（供前端使用）")
    args = ap.parse_args()

    entry_class = entry_method = None
    if args.entry:
        parts = args.entry.split(".")
        if len(parts) == 2:
            entry_class, entry_method = parts
        else:
            entry_class = parts[0]
            entry_method = "main"

    result = analyze(
        args.source,
        entry_class=entry_class,
        entry_method=entry_method or "main",
        output_json=args.json,
    )

    if args.json:
        print(json.dumps(result, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
