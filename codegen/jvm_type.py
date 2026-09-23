"""
JvmType 类型代数 —— Python 侧最小 JVM 类型语义层（收敛路线图 L1-a）。

规格来源：docs/plans/2026-09-21-codegen-type-convergence.md §六评审采纳 2——
「Python 侧必须有最小 JvmType 语义层（类型代数 + erasure()/is_subtype_of()/
substitute() + registry 缓存）……它不是弃件：就是 Rust `ty` crate 的规格本体，
照抄即可移植」。完全体（全部查询方法 + TypeFacts 不可变化）在 Rust 定型。

代数（frozen dataclass，结构等价 + 可 hash —— 缓存键的基础）：

    Primitive(kind)                       基本类型（kind ∈ boolean/byte/char/
                                          short/int/long/float/double/void）
    ClassRef(binary, args, is_interface)  类/接口引用；binary = JVM binary name，
                                          args = 泛型实参（raw 类型 args 为空）
    Array(elem)                           数组（元素协变）
    TypeVar(name, bound)                  类型变量；bound = 上界（None = 无界）
    Wildcard(kind, bound)                 通配符；kind ∈ '*'（无界）/ '+'（extends）
                                          / '-'（super）
    Null                                  null 底类型（<: 一切引用类型）

与 Rust `ty` crate 的映射（移植规格）：
    - 六个变体 → `ty::Ty` 同构 enum 变体（binary 全名 / DefId 为身份）；
    - erasure / substitute / is_subtype_of → Ty 固有方法（registry 参数 → TyCtx）；
    - _super_closure 的 (id, len, binary) 缓存键 → TyCtx 内 per-crate 祖先集
      （registry 在 Rust 侧是不可变 TypeFacts，无需长度失效位——Python 侧沿
      type_map._SHORT_INDEX_CACHE 的既有约定）；
    - 短名回退匹配（_closure_hit）是 Python 侧特有：registry 域外祖先只有
      Rust 短名可比较；Rust 侧 binary 全名即身份，无此回退。

与 sig_parse.py 的关系：from_signature 与 sig_parse 解析同一文法
（JVMS §4.7.9），但 sig_parse 产出 Rust 类型字符串（发射侧），无法无损回填
类型对象——本函数产出类型化结果（查询侧）。两套语法实现暂并行，M-3 /
字符串路径退役后由本层唯一持有。

依赖方向（禁止回环）：本模块 → constants / type_map（均无本地反向依赖）。
"""

from __future__ import annotations

import re
from collections.abc import Mapping
from dataclasses import dataclass

from .constants import OBJECT_CLASS
from .type_map import short_cls

# 数组类型固定实现的接口（JLS 4.10.3）：数组 <: 这三个 + Object
_ARRAY_SUPERTYPES = frozenset({
    OBJECT_CLASS, 'java/lang/Cloneable', 'java/io/Serializable',
})

# 描述符字符 → 基本类型名
_PRIM_DESC: dict[str, str] = {
    'Z': 'boolean', 'B': 'byte', 'C': 'char', 'S': 'short',
    'I': 'int', 'J': 'long', 'F': 'float', 'D': 'double', 'V': 'void',
}

# 泛型签名里的基本类型字符（JVMS §4.7.9 与字段描述符同字符集，V 除外）
_SIG_PRIM: dict[str, str] = {k: v for k, v in _PRIM_DESC.items() if k != 'V'}


# ── 代数变体 ────────────────────────────────────────────────────

class JvmType:
    """类型代数基类：查询方法在此单点实现，变体只承载数据。"""

    __slots__ = ()

    # -- 构造便捷入口 --------------------------------------------------

    @staticmethod
    def object_type() -> 'ClassRef':
        return ClassRef(OBJECT_CLASS)

    @staticmethod
    def class_of(binary: str, registry: 'dict | None' = None) -> 'ClassRef':
        """从 binary name 构造 ClassRef；registry 可查时补全 is_interface 标志。"""
        ci = registry.get(binary) if registry else None
        return ClassRef(binary, (), bool(ci.is_interface) if ci is not None else False)

    # -- 查询方法 ------------------------------------------------------

    def erasure(self) -> 'JvmType':
        """擦除到 raw：实参 → 擦除；TypeVar → 上界擦除（无界 → Object）；
        Wildcard → 上界擦除（无界 → Object）；数组擦除元素；其余不动。幂等。"""
        if isinstance(self, ClassRef):
            return ClassRef(self.binary, (), self.is_interface) if self.args else self
        if isinstance(self, Array):
            elem = self.elem.erasure()
            return Array(elem) if elem is not self.elem else self
        if isinstance(self, TypeVar):
            return self.bound.erasure() if self.bound is not None else self.object_type()
        if isinstance(self, Wildcard):
            return self.bound.erasure() if self.bound is not None else self.object_type()
        return self  # Primitive / Null

    def substitute(self, mapping: Mapping[str, 'JvmType']) -> 'JvmType':
        """类型变量代入（fmap）：命中映射表的 TypeVar 换成目标；否则递归进
        实参 / 数组元素 / 上界。命中值不再二次代入（单遍语义，
        substitute(f) ∘ substitute(g) = substitute(g∘f)，g 定义在 f 的像上）。"""
        if isinstance(self, TypeVar):
            if self.name in mapping:
                return mapping[self.name]
            if self.bound is None:
                return self
            nb = self.bound.substitute(mapping)
            return TypeVar(self.name, nb) if nb is not self.bound else self
        if isinstance(self, ClassRef):
            if not self.args:
                return self
            return ClassRef(self.binary,
                            tuple(a.substitute(mapping) for a in self.args),
                            self.is_interface)
        if isinstance(self, Array):
            return Array(self.elem.substitute(mapping))
        if isinstance(self, Wildcard):
            if self.bound is None:
                return self
            return Wildcard(self.kind, self.bound.substitute(mapping))
        return self  # Primitive / Null

    def is_subtype_of(self, other: 'JvmType',
                      registry: 'dict | None' = None) -> bool:
        """子类型判定（自反 + 传递；registry = dict[binary, ClassInfo]，
        走 super_class/interfaces 闭包，闭包集按 (id, len, binary) 缓存）。

        最小层语义（与 JVM 运行时/reflexive 子类型化一致）：
          - Primitive：仅自反（隐式拓宽是转换不是子类型，归 coerce 层）；
          - ClassRef：按 binary 闭包判定；带实参的目标额外要求实参包含
            （containment，通配符感知）；raw 目标不做实参检查（unchecked）；
          - Array：元素协变；且 <: Object/Cloneable/Serializable；
          - TypeVar：经上界判定（无界 → Object）；
          - Wildcard 作目标 = 包含关系（+ 上界 / - 下界 / * 万有），
            作源 = 按其上界的擦除形态判定；
          - Null <: 一切引用类型（ClassRef/Array/TypeVar/Wildcard）。
        """
        if self is other or self == other:
            return True
        # null 底类型
        if isinstance(self, Null):
            return isinstance(other, (ClassRef, Array, TypeVar, Wildcard))
        # 类型变量：经上界判定
        if isinstance(self, TypeVar):
            bound = self.bound if self.bound is not None else self.object_type()
            return bound.is_subtype_of(other, registry)
        # 通配符作源：按静态形态（上界；- 的静态形态是 Object）
        if isinstance(self, Wildcard):
            if self.kind == '+':
                return (self.bound if self.bound is not None
                        else self.object_type()).is_subtype_of(other, registry)
            return self.object_type().is_subtype_of(other, registry)
        # 通配符作目标：包含关系
        if isinstance(other, Wildcard):
            if other.kind == '*':
                return True
            if other.kind == '+':
                ob = other.bound if other.bound is not None else self.object_type()
                return self.is_subtype_of(ob, registry)
            ob = other.bound if other.bound is not None else self.object_type()
            return ob.is_subtype_of(self, registry)  # super：下界 <: self
        # 类型变量作目标：X <: T 等价于 X <: bound(T)（无界 → Object）
        if isinstance(other, TypeVar):
            ob = other.bound if other.bound is not None else self.object_type()
            return self.is_subtype_of(ob, registry)
        # 数组
        if isinstance(self, Array):
            if isinstance(other, Array):
                return self.elem.is_subtype_of(other.elem, registry)
            if isinstance(other, ClassRef):
                return other.binary in _ARRAY_SUPERTYPES
            return False
        # ClassRef
        if isinstance(self, ClassRef):
            if not isinstance(other, ClassRef):
                return False
            if other.binary in _super_closure(self.binary, registry) \
                    or _closure_hit(self.binary, other, registry):
                if not other.args:
                    return True  # raw 目标：unchecked 接受
                if len(self.args) != len(other.args):
                    return False
                return all(_contained(a, b, registry)
                           for a, b in zip(self.args, other.args))
            return False
        return False  # Primitive 只剩自反（已在开头相等判定覆盖）

    def to_display(self) -> str:
        """人读串（诊断用；非 Rust 类型串——JVM→Rust 映射仍归 type_map）。"""
        if isinstance(self, Primitive):
            return self.kind
        if isinstance(self, ClassRef):
            base = self.binary.replace('/', '.').replace('$', '.')
            if self.args:
                return f"{base}<{', '.join(a.to_display() for a in self.args)}>"
            return base
        if isinstance(self, Array):
            return f"{self.elem.to_display()}[]"
        if isinstance(self, TypeVar):
            if self.bound is not None:
                return f"{self.name} extends {self.bound.to_display()}"
            return self.name
        if isinstance(self, Wildcard):
            if self.kind == '+':
                return f"? extends {self.bound.to_display() if self.bound else 'Object'}"
            if self.kind == '-':
                return f"? super {self.bound.to_display() if self.bound else 'Object'}"
            return '?'
        return 'null'


@dataclass(frozen=True)
class Primitive(JvmType):
    kind: str  # 'int' | 'long' | ... | 'void'


@dataclass(frozen=True)
class ClassRef(JvmType):
    binary: str
    args: tuple[JvmType, ...] = ()
    is_interface: bool = False


@dataclass(frozen=True)
class Array(JvmType):
    elem: JvmType


@dataclass(frozen=True)
class TypeVar(JvmType):
    name: str
    bound: 'JvmType | None' = None


@dataclass(frozen=True)
class Wildcard(JvmType):
    kind: str = '*'        # '*' | '+' | '-'
    bound: 'JvmType | None' = None


@dataclass(frozen=True)
class Null(JvmType):
    pass


# ── 构造入口：描述符 / 泛型签名 ─────────────────────────────────

# Rust 基本类型拼写 → JVM 基本类型名（from_rust_type 的反向映射；
# u8/u32/u64/usize 无 JVM 对应物，不在表内——按域外占位处理）
_RUST_PRIM: dict[str, str] = {
    'bool': 'boolean', 'i8': 'byte', 'u16': 'char', 'i16': 'short',
    'i32': 'int', 'i64': 'long', 'f32': 'float', 'f64': 'double',
    '()': 'void',
}

_PRIM_TO_RUST: dict[str, str] = {v: k for k, v in _RUST_PRIM.items()}

# (id(registry), len(registry), rust_ty) → 解析结果。失效约定同 _CLOSURE_CACHE。
_RUST_PARSE_CACHE: dict[tuple, JvmType] = {}


def from_rust_type(rust_ty: str, registry: 'dict | None' = None) -> JvmType:
    """Rust 类型串（codegen 表面语言，如 ``ArrayList<String>`` / ``JArray<i32>`` /
    ``Object`` / ``K``）→ JvmType——发射侧 jvm_to_rust 的反向边界解析器（查询侧）。

    消费点（invoke 域类型决策，收敛路线图 L1-a 批次 3）以本入口替代
    ``split('<')[0]`` 头部文本解剖：类型身份（binary / 是否接口）与形态
    （数组 / 参数化 / 基本类型）经类型对象查询。

    文法：``'&'? head ('<' arg (',' arg)* '>')?``，head = 标识符或 ``'()'``，
    arg 递归同文法且可为 ``'?'``（通配符）。映射：

    - ``i32`` 等 Rust 基本类型拼写（``'()'`` → void）→ ``Primitive``；
    - ``JArray<E>`` → ``Array``（裸 ``JArray`` 无实参时按占位 ClassRef 处理）；
    - ``Object`` → ``ClassRef(java/lang/Object)``；
    - 其余 head 经 ``_registry_short_index`` 反查 binary（is_interface 同步补全；
      configure_short_names 保证短名 → binary 单射）；反查不到 → ``ClassRef(head)``
      占位（binary = 短名，registry 域外身份，同 _is_subtype 的父类占位口径）；
    - ``'?'`` → ``Wildcard('*')``；空串 / 无前导标识符 → ``Null()``（全函数哨兵，
      消费点按「无类身份」处理，与旧头部解剖对空串的行为一致）。

    宽容性：取最长有效前缀——head 后无 ``'<'`` 即裸形态；实参段畸形（未闭合 /
    尾随残片）时降级为裸 head 占位（旧路径对畸形输入同样只取头部文本，无更严格
    语义可失去）。幂等可缓存。"""
    key = (id(registry), len(registry) if registry else 0, rust_ty)
    hit = _RUST_PARSE_CACHE.get(key)
    if hit is not None:
        return hit
    ty = _parse_rust_type(rust_ty, registry)
    if len(_RUST_PARSE_CACHE) > 8192:  # 防御性上限（正常转译远小于此）
        _RUST_PARSE_CACHE.clear()
    _RUST_PARSE_CACHE[key] = ty
    return ty


def _parse_rust_type(s: str, registry: 'dict | None') -> JvmType:
    s = s.strip()
    while s.startswith('&'):
        s = s[1:].lstrip()
    if not s:
        return Null()
    if s.startswith('?'):
        return Wildcard('*')
    m = re.match(r'[A-Za-z_][A-Za-z0-9_]*|\(\)', s)
    if m is None:
        return Null()
    head = m.group(0)
    rest = s[m.end():].lstrip()
    args: 'tuple[JvmType, ...] | None' = None
    if rest.startswith('<'):
        args = _parse_rust_args(rest, registry)
    prim = _RUST_PRIM.get(head)
    if prim is not None:
        return Primitive(prim)
    if head == 'JArray':
        if args is not None and len(args) == 1:
            return Array(args[0])
        return ClassRef(head)  # 裸/畸形 JArray：占位（域外包装名）
    binary = head
    is_iface = False
    if head == 'Object':
        binary = OBJECT_CLASS
    elif registry:
        from .type_map import _registry_short_index
        ci = _registry_short_index(registry).get(head)
        if ci is not None:
            binary, is_iface = ci.name, bool(ci.is_interface)
    if args is None:
        return ClassRef(binary, (), is_iface)
    return ClassRef(binary, args, is_iface)


def _parse_rust_args(s: str, registry: 'dict | None') -> 'tuple[JvmType, ...] | None':
    """解析 ``<A, B, ...>`` 实参段（s 以 '<' 开头）。畸形（未闭合）→ None。"""
    args: list[JvmType] = []
    i = 1
    while i < len(s):
        if s[i] == '>':
            return tuple(args)
        # 逐个实参：递归取一个类型后应见 ',' 或 '>'
        one, j = _scan_rust_one(s, i, registry)
        if one is None:
            return None
        args.append(one)
        while j < len(s) and s[j] in ' \t':
            j += 1
        if j >= len(s):
            return None
        if s[j] == ',':
            i = j + 1
            continue
        if s[j] == '>':
            return tuple(args)
        return None
    return None


def _scan_rust_one(s: str, i: int, registry: 'dict | None') -> 'tuple[JvmType, int] | None':
    """从 i 起扫描一个完整类型（含嵌套实参段），返回 (类型, 结束位置)。"""
    n = len(s)
    while i < n and s[i] in ' \t':
        i += 1
    if i >= n:
        return None
    if s[i] == '?':
        return Wildcard('*'), i + 1
    m = re.match(r'[A-Za-z_][A-Za-z0-9_]*|\(\)', s[i:])
    if m is None:
        return None
    head = m.group(0)
    j = i + m.end()
    if j < n and s[j] == '<':
        # 嵌套实参段：配平尖括号后整段递归（复用主文法，保证嵌套形态一致）
        depth = 0
        k = j
        while k < n:
            if s[k] == '<':
                depth += 1
            elif s[k] == '>':
                depth -= 1
                if depth == 0:
                    break
            k += 1
        if k >= n:
            return None
        inner_t = _parse_rust_type(s[i:k + 1], registry)
        return inner_t, k + 1
    prim = _RUST_PRIM.get(head)
    if prim is not None:
        return Primitive(prim), j
    return from_rust_type(head, registry), j


def rust_head_name(ty: JvmType) -> str:
    """类型对象的 Rust 基名（去实参的头标识符）——被替代位点
    ``split('<')[0]`` 文本解剖的类型对象等价物：

    - ``ClassRef`` → ``short_cls(binary)``（registry 域内即解析输入的头标识符
      ——短名 → binary 单射的逆；域外占位的 binary 即短名本身）；
    - ``Array`` → ``'JArray'``（数组包装族名，裸名口径与旧文本解剖一致）；
    - ``Primitive`` → Rust 拼写（void → ``'()'``）；
    - ``TypeVar`` → 变量名；``Wildcard`` → ``'?'``；``Null`` → ``''``。"""
    if isinstance(ty, ClassRef):
        return short_cls(ty.binary)
    if isinstance(ty, Array):
        return 'JArray'
    if isinstance(ty, Primitive):
        return _PRIM_TO_RUST.get(ty.kind, ty.kind)
    if isinstance(ty, TypeVar):
        return ty.name
    if isinstance(ty, Wildcard):
        return '?'
    return ''


def strict_erased_subtype(actual: JvmType, expected: JvmType,
                          registry: 'dict | None') -> bool:
    """erasure 基名级的严格子类型（invoke 实参上转判定，TypeIR 批次 3 S5）。

    与 hierarchy._is_subtype（字符串键适配层，TypeIR 试点已委托本模块）的
    可观察语义逐点一致——调用侧从 Rust 类型串解析出类型对象后经本查询决策，
    替代 actual 基名的头部文本解剖：

    - 不自反：erasure 后同 binary（或解析前即同短名）→ False；
    - java/lang/Object 恒不作为成立目标（上转 Object 归装箱路径，不是
      __into_super 链）；
    - actual 基名须 registry 域内（占位/数组/基本类型 → False）；
      expected 域外时按短名占位匹配闭包（is_subtype_of 的 _closure_hit 路径）。
    """
    a, e = actual.erasure(), expected.erasure()
    if not (isinstance(a, ClassRef) and isinstance(e, ClassRef)):
        return False
    if a.binary == e.binary:
        return False
    if e.binary == OBJECT_CLASS:
        return False
    if registry is not None and a.binary not in registry:
        return False
    return a.is_subtype_of(e, registry)


def from_descriptor(desc: str) -> JvmType:
    """JVM 字段描述符（JVMS §4.3.2）→ JvmType：'I'、'[I'、'[[Ljava/lang/String;'、
    'Ljava/util/List;'。非法描述符抛 ValueError（类型层宁严不宽）。"""
    if not desc:
        raise ValueError('empty descriptor')
    c = desc[0]
    if c in _PRIM_DESC:
        if len(desc) != 1:
            raise ValueError(f'trailing text after primitive descriptor: {desc!r}')
        return Primitive(_PRIM_DESC[c])
    if c == '[':
        return Array(from_descriptor(desc[1:]))
    if c == 'L':
        end = desc.find(';')
        if end < 0 or end != len(desc) - 1:
            raise ValueError(f'malformed class descriptor: {desc!r}')
        return ClassRef(desc[1:end])
    raise ValueError(f'unknown descriptor: {desc!r}')


def from_signature(sig: str, registry: 'dict | None' = None) -> JvmType:
    """泛型签名单类型（JVMS §4.7.9：FieldTypeSignature）→ JvmType。

    文法与 sig_parse._parse_one_type 同源，但产出类型对象：
      'I'                        → Primitive('int')
      'TT;'                      → TypeVar('T')
      '[TT;'                     → Array(TypeVar('T'))
      '*' / '+TT;' / '-TT;'      → Wildcard
      'Ljava/util/List<TT;>;'    → ClassRef('java/util/List', (TypeVar('T'),))
      'LOuter<TT;>.Inner<...>;'  → ClassRef('Outer$Inner', (…,))——内部类
                                   binary 拼接 $，实参取最内段（与 sig_parse 同）
    registry 可查时补全 is_interface。非法签名抛 ValueError。"""
    ty, i = _parse_sig_type(sig, 0, registry)
    if i != len(sig):
        raise ValueError(f'trailing text after signature: {sig!r}')
    return ty


def _parse_sig_type(sig: str, i: int, registry: 'dict | None') -> 'tuple[JvmType, int]':
    if i >= len(sig):
        raise ValueError(f'unexpected end of signature: {sig!r}')
    c = sig[i]
    if c in _SIG_PRIM:
        return Primitive(_SIG_PRIM[c]), i + 1
    if c == 'T':
        end = sig.find(';', i)
        if end < 0:
            raise ValueError(f'unterminated type variable: {sig!r}')
        return TypeVar(sig[i + 1:end]), end + 1
    if c == '[':
        elem, j = _parse_sig_type(sig, i + 1, registry)
        return Array(elem), j
    if c == '*':
        return Wildcard('*'), i + 1
    if c in '+-':
        bound, j = _parse_sig_type(sig, i + 1, registry)
        return Wildcard(c, bound), j
    if c == 'L':
        j = i + 1
        while j < len(sig) and sig[j] not in ('<', ';', '.'):
            j += 1
        binary = sig[i + 1:j]
        args: tuple[JvmType, ...] = ()
        if j < len(sig) and sig[j] == '<':
            args, j = _parse_sig_args(sig, j, registry)
        # ClassTypeSigSuffix：LOuter<A;>.Inner<B;>; → Outer$Inner，实参取最内段
        while j < len(sig) and sig[j] == '.':
            k = j + 1
            while k < len(sig) and sig[k] not in ('<', ';', '.'):
                k += 1
            binary = binary + '$' + sig[j + 1:k]
            j = k
            if j < len(sig) and sig[j] == '<':
                args, j = _parse_sig_args(sig, j, registry)
        if j >= len(sig) or sig[j] != ';':
            raise ValueError(f'malformed class signature: {sig!r}')
        ci = registry.get(binary) if registry else None
        return ClassRef(binary, args,
                        bool(ci.is_interface) if ci is not None else False), j + 1
    raise ValueError(f'unknown signature char {c!r} at {i} in {sig!r}')


def _parse_sig_args(sig: str, i: int, registry: 'dict | None') -> 'tuple[tuple[JvmType, ...], int]':
    """解析 <TypeArgument+>，i 指向 '<'。返回 (实参元组, '>' 之后位置)。"""
    i += 1  # 跳过 '<'
    args: list[JvmType] = []
    while i < len(sig) and sig[i] != '>':
        arg, i = _parse_sig_type(sig, i, registry)
        args.append(arg)
    if i >= len(sig):
        raise ValueError(f'unterminated type arguments in {sig!r}')
    return tuple(args), i + 1  # 跳过 '>'


# ── registry 闭包与缓存 ─────────────────────────────────────────

# (id(registry), len(registry), binary) → 祖先 binary 全名集（含自身）。
# 失效约定与 type_map._SHORT_INDEX_CACHE 一致：registry 身份 + 长度。
_CLOSURE_CACHE: dict[tuple, frozenset] = {}


def _super_closure(binary: str, registry: 'dict | None') -> frozenset:
    """binary 的全部祖先 binary 集合（含自身）：super_class 链 + interfaces
    闭包（BFS）。未注册节点是闭包叶子（无出边）。含 java/lang/Object
    （正确的 Java 语义：一切类 <: Object）。带缓存。"""
    key = (id(registry), len(registry) if registry else 0, binary)
    hit = _CLOSURE_CACHE.get(key)
    if hit is not None:
        return hit
    seen: set[str] = {binary}
    if registry:
        queue: list[str] = [binary]
        while queue:
            cur = queue.pop(0)
            ci = registry.get(cur)
            if ci is None:
                continue
            edges = []
            if ci.super_class:
                edges.append(ci.super_class)
            edges.extend(ci.interfaces or [])
            for nxt in edges:
                if nxt not in seen:
                    seen.add(nxt)
                    queue.append(nxt)
    result = frozenset(seen)
    if len(_CLOSURE_CACHE) > 4096:  # 防御性上限（正常转译远小于此）
        _CLOSURE_CACHE.clear()
    _CLOSURE_CACHE[key] = result
    return result


def _closure_hit(child_bin: str, target: ClassRef, registry: 'dict | None') -> bool:
    """闭包短名回退：闭包成员与目标按 Rust 短名匹配。registry 域内短名经
    configure_short_names 消歧后即 binary 身份（回退退化为恒等比较）；此回退
    只对域外节点生效——闭包上未注册祖先（binary 来自 ClassInfo 边）与
    短名占位目标（hierarchy._is_subtype 的 registry 外 parent）只有 Rust
    短名这一跨域可比身份。Rust ty crate 无此层（binary/DefId 即身份）。"""
    tgt_short = short_cls(target.binary)
    return any(short_cls(b) == tgt_short
               for b in _super_closure(child_bin, registry))


def _contained(actual: JvmType, formal: JvmType, registry: 'dict | None') -> bool:
    """泛型实参包含关系（JLS 4.5.1，最小实现）：
    ? extends B 接受 actual <: B；? super B 接受 B <: actual；
    ? 接受一切；其余要求结构相等（Java 泛型不协变）。"""
    if isinstance(formal, Wildcard):
        if formal.kind == '*':
            return True
        fb = formal.bound if formal.bound is not None else JvmType.object_type()
        if formal.kind == '+':
            return actual.is_subtype_of(fb, registry)
        return fb.is_subtype_of(actual, registry)
    return actual == formal


# ── A-4 批次 3+：接口类型位置载体化（单一决策点）────────────────────────────
#
# 接口名进入类型位置（形参 / 返回 / 局部 / 字段）时发射擦除载体形态
# `I<Object, ..>`（接口载体：持有 Object 引用的真实 Rust 类型，方法经
# `__interface` itable 分派），使调用点从 `Into::<I<Object>>::into(obj).m()`
# 变为 `it.m()` 直调（主度量 32931 处的消灭对象）。
#
# 五个擦除点（sig_types.emitted_method_sig_types / method_gen 存根 /
# invoke_sig 调用点实参解析 / type_map.jvm_to_rust / sig_parse._parse_one_type）
# 一律经本组函数取判定，禁止平行实现：
#   carrier_type(binary, registry)          binary → 载体串（未启用 → None）
#   carrier_type_for_ident(rust_ty, registry) Rust 类型串首标识符 → 载体串
#
# 铺设控制：CARRIER_TYPE_POSITIONS 列出已启用接口的 binary name；
# None = 全部接口（终态）。批次逐批宽化，全部批次落地后置 None。

_FUNCTION_IFACES = {
    'java/util/function/Consumer', 'java/util/function/BiConsumer',
    'java/util/function/Supplier', 'java/util/function/Function',
    'java/util/function/BiFunction', 'java/util/function/Predicate',
    'java/util/function/BiPredicate', 'java/util/function/UnaryOperator',
    'java/util/function/BinaryOperator',
    'java/util/function/IntFunction', 'java/util/function/IntPredicate',
    'java/util/function/IntSupplier',
    'java/util/function/IntUnaryOperator', 'java/util/function/IntBinaryOperator',
    'java/util/function/ToIntFunction', 'java/util/function/ToLongFunction',
    'java/util/function/ToDoubleFunction', 'java/util/function/LongFunction',
    'java/util/function/DoubleFunction', 'java/util/function/BooleanSupplier',
    'java/util/function/ObjIntConsumer', 'java/util/function/ObjDoubleConsumer',
    'java/util/function/ObjLongConsumer', 'java/util/function/DoubleBinaryOperator',
    'java/util/function/DoublePredicate',
    'java/util/function/DoubleSupplier', 'java/util/function/DoubleUnaryOperator',
    'java/util/function/LongBinaryOperator', 
    'java/util/function/LongPredicate', 'java/util/function/LongSupplier',
    'java/util/function/LongUnaryOperator', 'java/util/function/ToIntBiFunction',
    'java/util/function/ToLongBiFunction', 'java/util/function/ToDoubleBiFunction',
    'java/util/function/LongToIntFunction', 'java/util/function/LongToDoubleFunction',
    'java/util/function/DoubleToIntFunction', 'java/util/function/DoubleToLongFunction',
    'java/util/function/IntToLongFunction', 'java/util/function/IntToDoubleFunction',
}

CARRIER_TYPE_POSITIONS: 'frozenset[str] | None' = frozenset({
    'java/util/Iterator',      # 批次 3 穿透起步（证据文档 §6 建议）
    'java/util/ListIterator',  # 批次 4：集合族（Iterator 直系，hasNext/next 同族分派）
    'java/util/List',
    'java/util/Collection',
    'java/util/Set',
    'java/util/Map',
    'java/util/Map$Entry',
    'java/util/Queue',
    'java/util/Deque',
    # 批次 5：函数式接口族 + 比较器（Spliterator 族暂缓：特化桥接
    # forEachRemaining(Object)↔(LongConsumer) 的名/型解析发散，见证据文档遗留节）
    'java/util/Comparator',
    'java/util/Comparable',
    'java/util/stream/Collector',
    *_FUNCTION_IFACES,
    # 批次 6：CharSequence/Appendable 字符族（前次试做因手写 decimal 层阻塞回退，
    # 本次两侧同步：runtime 手写体 appendTo/append_seq 按载体边界适配，
    # 见 docs/reports/2026-09-21-a4-phase0-evidence.md §8 遗留 1）
    'java/lang/CharSequence',
    'java/lang/Appendable',
})


def iface_carrier_enabled(binary: str) -> bool:
    """接口是否已进入类型位置载体化铺设。"""
    gate = CARRIER_TYPE_POSITIONS
    return gate is None or binary in gate


def carrier_type(binary: str, registry: 'dict | None') -> 'str | None':
    """启用接口 → 擦除载体 Rust 类型串 `Short<Object, ..>`；否则 None。

    非接口 / 不在 registry（闭包外）/ 未启用 → None，调用方回退既有擦除路径
    （jvm_to_rust 接口分支的 Object 化）。形参数取 effective_class_type_params
    （含内部类从外围继承的形参——与 jvm_to_rust 的类泛型分支同一口径）。"""
    if not registry or not iface_carrier_enabled(binary):
        return None
    ci = registry.get(binary)
    if ci is None or not getattr(ci, 'is_interface', False):
        return None
    from .type_map import effective_class_type_params
    params = effective_class_type_params(ci, registry)
    if params:
        return f"{short_cls(binary)}<{', '.join('Object' for _ in params)}>"
    return short_cls(binary)


def carrier_type_for_ident(rust_ty: str, registry: 'dict | None') -> 'str | None':
    """Rust 类型串（`Iterator<E>` / `List<String>` / `Consumer` …）的首标识符
    命中启用接口 → 载体串；否则 None。调用点实参解析与签名过滤共用。"""
    if not registry or not rust_ty:
        return None
    m = re.match(r'^(\w+)', rust_ty)
    if m is None:
        return None
    from .type_map import _registry_short_index
    ci = _registry_short_index(registry).get(m.group(1))
    if ci is None or not getattr(ci, 'is_interface', False):
        return None
    return carrier_type(ci.name, registry)
