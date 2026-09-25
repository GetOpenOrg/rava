"""Locale 种子（L-1 L1-b）：从用户字节码推出需要编入的 locale 集及其 CLDR 资源束类。

资源束经 `ResourceBundle.getBundle` 按类名反射装载，调用链上没有静态边；参照
GraalVM native-image `-H:IncludeLocales`，入选 locale =

- 默认集：ROOT + en（运行时默认 locale 的语言）；
- 用户字节码中静态可见的 locale 引用：
  - `const` 类的 static 自类型常量字段（`getstatic Locale.FRANCE`）——沿其 `<clinit>`
    溯源到字符串字面量：`<int k>; invokestatic f:(B)L; putstatic F` → f 体内
    `getstatic T.arr; iload; aaload` → T.<clinit> 中 `aload; <k>; ldc 语言; ldc 地区;
    invoke…; aastore`。各步只按结构匹配（JDK21/25 常量表下标顺序不同，溯源自然携带）；
  - `factory` 调用且实参全为字符串字面量（`Locale.of("fr", "FR")`）；
  - `tag` 调用且实参为字面量语言标签（`Locale.forLanguageTag("fr-FR")`）；
- 显式配置 `--locales fr,it_IT,zh-Hant-TW`。

每个 locale 连同父链（fr_FR → fr → ROOT）入闭包——ResourceBundle 回退语义所需。
类名全部来自 runtime 清单 `locale_seeds.txt`（原则 4）。
"""
from __future__ import annotations

from dataclasses import dataclass

from .runtime_manifest import read_list

_INT_CONSTS = {'iconst_m1': -1, 'iconst_0': 0, 'iconst_1': 1, 'iconst_2': 2,
               'iconst_3': 3, 'iconst_4': 4, 'iconst_5': 5}


@dataclass(frozen=True)
class LocaleSeeds:
    const_classes: frozenset
    factories: frozenset          # "类.方法:描述符"
    tags: frozenset
    bundle_bases: tuple
    triggers: frozenset = frozenset()   # (类, 成员名)


def load_manifest() -> LocaleSeeds:
    consts, facts, tags, bundles, triggers = set(), set(), set(), [], set()
    for ln in read_list('locale_seeds.txt'):
        kind, val = ln.split(None, 1)
        val = val.strip()
        if kind == 'const':
            consts.add(val)
        elif kind == 'factory':
            facts.add(val)
        elif kind == 'tag':
            tags.add(val)
        elif kind == 'bundle':
            bundles.append(val)
        elif kind == 'trigger':
            cls, _, member = val.rpartition('.')
            triggers.add((cls, member))
    return LocaleSeeds(frozenset(consts), frozenset(facts), frozenset(tags), tuple(bundles),
                       frozenset(triggers))


# ── 字面量识别 ──────────────────────────────────────────────────────────────

def _int_lit(ins):
    op = ins.opcode or ''
    if op in _INT_CONSTS:
        return _INT_CONSTS[op]
    if op in ('bipush', 'sipush'):
        try:
            return int(ins.operand)
        except (TypeError, ValueError):
            return None
    return None


def _str_lit(ins):
    if (ins.opcode or '') not in ('ldc', 'ldc_w'):
        return None
    c = ins.comment or ''
    if c == 'String':
        return ''
    if c.startswith('String '):
        return c[len('String '):]
    return None


def _member_ref(comment: str):
    """'Method a/B.m:(…)R' / 'InterfaceMethod …' / 'Field a/B.f:T' → (类, 成员, 描述符)。"""
    if not comment:
        return None
    parts = comment.split(' ', 1)
    if len(parts) != 2 or parts[0] not in ('Method', 'InterfaceMethod', 'Field'):
        return None
    rest = parts[1]
    colon = rest.find(':')
    if colon < 0:
        return None
    dot = rest.rfind('.', 0, colon)
    if dot < 0:
        return None
    return rest[:dot], rest[dot + 1:colon].strip('"'), rest[colon + 1:]


def _param_count(desc: str) -> int:
    n, i = 0, 1
    while i < len(desc) and desc[i] != ')':
        while desc[i] == '[':
            i += 1
        if desc[i] == 'L':
            i = desc.index(';', i)
        n += 1
        i += 1
    return n


def _find_method(ci, name, desc=None):
    for m in (ci.methods if ci else ()):
        if m.name == name and (desc is None or m.descriptor == desc):
            return m
    return None


# ── Locale 值 ──────────────────────────────────────────────────────────────

def _norm(lang='', script='', region='', variant=''):
    return (lang.lower(), script.title(), region.upper(), variant)


def parse_tag(tag: str):
    """BCP 47 标签或下划线形式 → (语言, 文字, 地区, 变体)；无法识别 → None。"""
    parts = [p for p in tag.replace('_', '-').split('-') if p]
    if not parts or not parts[0].isalpha():
        return None
    lang, script, region, variant = parts[0], '', '', ''
    rest = parts[1:]
    if rest and len(rest[0]) == 4 and rest[0].isalpha():
        script = rest.pop(0)
    if rest and ((len(rest[0]) == 2 and rest[0].isalpha())
                 or (len(rest[0]) == 3 and rest[0].isdigit())):
        region = rest.pop(0)
    if rest:
        variant = '_'.join(rest)
    return _norm(lang, script, region, variant)


def parent_chain(loc) -> list[str]:
    """ResourceBundle 候选后缀（由具体到一般，不含 ROOT）：fr_FR → [fr_FR, fr]。"""
    lang, script, region, variant = loc
    if not lang:
        return []
    cands = []
    if script:
        if region and variant:
            cands.append(f'{lang}_{script}_{region}_{variant}')
        if region:
            cands.append(f'{lang}_{script}_{region}')
        cands.append(f'{lang}_{script}')
    if region and variant:
        cands.append(f'{lang}_{region}_{variant}')
    if region:
        cands.append(f'{lang}_{region}')
    cands.append(lang)
    out = []
    for c in cands:
        if c not in out:
            out.append(c)
    return out


# ── const 字段溯源 ─────────────────────────────────────────────────────────

class _Tracer:
    def __init__(self, load):
        self._load = load
        self._memo: dict = {}

    def static_field(self, cls: str, fname: str):
        key = ('f', cls, fname)
        if key not in self._memo:
            self._memo[key] = None
            self._memo[key] = self._static_field(cls, fname)
        return self._memo[key]

    def _static_field(self, cls, fname):
        clinit = _find_method(self._load(cls), '<clinit>')
        if clinit is None:
            return None
        ins = clinit.instrs or []
        for i, x in enumerate(ins):
            if x.opcode != 'putstatic':
                continue
            ref = _member_ref(x.comment)
            if not ref or ref[0] != cls or ref[1] != fname:
                continue
            return self._value_before(ins, i)
        return None

    def _value_before(self, ins, i):
        """ins[i] 消费的栈顶值：invokestatic 且实参全为字面量 → 溯源其返回值。"""
        if i == 0 or ins[i - 1].opcode != 'invokestatic':
            return None
        ref = _member_ref(ins[i - 1].comment)
        if not ref:
            return None
        n = _param_count(ref[2])
        if i - 1 - n < 0:
            return None
        args = []
        for a in ins[i - 1 - n:i - 1]:
            v = _str_lit(a)
            if v is None:
                v = _int_lit(a)
            if v is None:
                return None
            args.append(v)
        if args and all(isinstance(a, str) for a in args):
            return tuple(args)
        return self._method_return(ref[0], ref[1], ref[2], args)

    def _method_return(self, cls, name, desc, args):
        """静态方法体内 `getstatic T.arr; iload_k; aaload` → 常量表 T.arr[args[k]]。"""
        m = _find_method(self._load(cls), name, desc)
        if m is None:
            return None
        ins = m.instrs or []
        for i in range(len(ins) - 2):
            if ins[i].opcode != 'getstatic' or ins[i + 2].opcode != 'aaload':
                continue
            op = ins[i + 1].opcode or ''
            if op.startswith('iload_'):
                k = int(op[len('iload_'):])
            elif op == 'iload':
                k = int(ins[i + 1].operand)
            else:
                continue
            ref = _member_ref(ins[i].comment)
            if not ref or k >= len(args) or not isinstance(args[k], int):
                continue
            return self._array_elem(ref[0], ref[1], args[k])
        return None

    def _array_elem(self, cls, arr, idx):
        """T.<clinit>：`aload; <idx>; 字面量…; invokestatic; aastore`，且该方法写 T.arr。"""
        key = ('a', cls, arr)
        table = self._memo.get(key)
        if table is None:
            table = self._memo[key] = self._array_table(cls, arr)
        return table.get(idx)

    def _array_table(self, cls, arr):
        clinit = _find_method(self._load(cls), '<clinit>')
        if clinit is None:
            return {}
        ins = clinit.instrs or []
        writes = any(x.opcode == 'putstatic' and (_member_ref(x.comment) or ())[:2] == (cls, arr)
                     for x in ins)
        if not writes:
            return {}
        table = {}
        for j, x in enumerate(ins):
            if x.opcode != 'aastore' or j < 2 or ins[j - 1].opcode != 'invokestatic':
                continue
            ref = _member_ref(ins[j - 1].comment)
            if not ref:
                continue
            n = _param_count(ref[2])
            lits = [_str_lit(a) for a in ins[j - 1 - n:j - 1]]
            if len(lits) != n or any(v is None for v in lits):
                continue
            k = _int_lit(ins[j - 2 - n])
            if k is not None and ins[j - 3 - n].opcode.startswith('aload'):
                table.setdefault(k, tuple(lits))
        return table


def _from_literals(lits):
    lits = list(lits) + ['', '', '']
    return _norm(lits[0], '', lits[1], lits[2])


# ── 入口 ───────────────────────────────────────────────────────────────────

def collect_locales(user_infos, load, extra=(), manifest: LocaleSeeds | None = None):
    """入选 locale 集（(语言, 文字, 地区, 变体) 元组，已排序）。

    user_infos：可迭代 ClassInfo（用户类）；load(binary_name) → ClassInfo|None；
    extra：`--locales` 给出的标签串。ROOT 不在集合内（束基名总是入选）。"""
    mf = manifest or load_manifest()
    tracer = _Tracer(load)
    found = {_norm('en')}
    for t in extra:
        loc = parse_tag(t)
        if loc:
            found.add(loc)
    for ci in user_infos:
        for m in ci.methods:
            ins = m.instrs or []
            for i, x in enumerate(ins):
                op = x.opcode or ''
                ref = _member_ref(x.comment)
                if not ref:
                    continue
                if op == 'getstatic' and ref[0] in mf.const_classes \
                        and ref[2] == f'L{ref[0]};':
                    lits = tracer.static_field(ref[0], ref[1])
                    if lits:
                        loc = _from_literals(lits)
                        if loc[0]:
                            found.add(loc)
                elif op.startswith('invoke'):
                    key = f'{ref[0]}.{ref[1]}:{ref[2]}'
                    if key not in mf.factories and key not in mf.tags:
                        continue
                    n = _param_count(ref[2])
                    lits = [_str_lit(a) for a in ins[max(0, i - n):i]]
                    if len(lits) != n or any(v is None for v in lits):
                        continue
                    loc = parse_tag(lits[0]) if key in mf.tags else _from_literals(lits)
                    if loc and loc[0]:
                        found.add(loc)
    return sorted(found)


def bundle_classes(locales, load, manifest: LocaleSeeds | None = None) -> list[str]:
    """入选 locale（含父链）对应的、可解析且通过纯数据判定的资源束类（已排序）。"""
    from .data_bundle import is_pure_data_bundle
    mf = manifest or load_manifest()
    suffixes = set()
    for loc in locales:
        suffixes.update(parent_chain(loc))
    out = set()
    for base in mf.bundle_bases:
        pkg, _, simple = base.rpartition('/')
        names = [base]
        for suf in sorted(suffixes):
            names.append(f'{base}_{suf}')
            names.append(f'{pkg}/ext/{simple}_{suf}')
        for n in names:
            ci = load(n)
            if ci is not None and is_pure_data_bundle(ci, load):
                out.add(n)
    return sorted(out)
