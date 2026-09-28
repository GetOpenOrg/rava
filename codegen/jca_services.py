"""JCA 服务注册（K-JCA，docs/plans/2026-09-25-jca-service-registry.md）。

`Cipher.getInstance("DES")` / `MessageDigest.getInstance("MD5")` 经 provider 服务表按类名
反射构造实现类（`Provider$Service.newInstance`），调用链上没有静态边。与 L-1 资源束同构：

- 服务表：provider 注册方法的字节码里 `ldc 类型; ldc 算法; ldc 实现类名` 三连字符串常量
  （SunJCE `ps(..)` / SunEntries `add(..)` / `addWithAlias(..)` 同形态），实现类名须可加载；
- 种子：服务类型的 engine 类（JCA 约定：engine 类简单名 == 服务类型）有方法在调用链上，
  且算法名（大小写不敏感）出现在用户类 String 常量里——transformation
  `DES/ECB/PKCS5Padding` 取首段；
- 放行：清单 `release` 行（算法实现包 + engine / SPI 类）从边界前缀放行，按字节码翻译。

类名全部来自 runtime 清单 `seeds.toml` [jca]（原则 4）。
"""
from __future__ import annotations

from dataclasses import dataclass

from .runtime_manifest import seed_section, jca_release_entries


@dataclass(frozen=True)
class JcaManifest:
    providers: tuple = ()          # (provider 名, 注册类, Provider 子类)
    release: tuple = ()            # 放行的类 / 包前缀（`/` 结尾为包）
    triggers: frozenset = frozenset()   # (类, 成员名)
    defaults: tuple = ()           # ((触发类, 成员名), 类型, 算法)：运行期配置决定的缺省服务
    alias_sources: tuple = ()      # 算法同义名源类（类初始化器中每个构造调用的字符串实参组）


@dataclass(frozen=True, order=True)
class Service:
    type: str
    algorithm: str
    impl: str          # 斜线形态 binary name
    provider: str


def load_manifest() -> JcaManifest:
    """seeds.toml [jca]：provider 注册类、放行条目、服务查找触发成员。"""
    sec = seed_section('jca')
    provs = tuple((p['name'], p['class'], p['provider']) for p in sec.get('providers', []))
    triggers = frozenset(tuple(t.rsplit('.', 1)) for t in sec.get('triggers', []))
    defaults = tuple((tuple(d['trigger'].rsplit('.', 1)), *d['service'].split('.', 1))
                     for d in sec.get('defaults', []))
    return JcaManifest(provs, tuple(jca_release_entries()), triggers, defaults,
                       tuple(sec.get('alias_sources', [])))


def _str_lit(ins):
    if (ins.opcode or '') not in ('ldc', 'ldc_w'):
        return None
    c = ins.comment or ''
    if c.startswith('String '):
        return c[len('String '):]
    return None


def extract_services(load, manifest: JcaManifest | None = None) -> list:
    """全部 provider 注册类的服务三元组（去重、排序）。load(binary_name) → ClassInfo|None。"""
    mf = manifest or load_manifest()
    out: set = set()
    for prov, cls, _pcls in mf.providers:
        ci = load(cls)
        for m in (ci.methods if ci else ()):
            ins = m.instrs or []
            for i in range(len(ins) - 2):
                a, b, c = (_str_lit(x) for x in ins[i:i + 3])
                if not (a and b and c) or '.' not in c or ' ' in c:
                    continue
                impl = c.replace('.', '/')
                if load(impl) is None:
                    continue
                out.add(Service(a, b, impl, prov))
    return sorted(out)


def provider_class(name: str, manifest: JcaManifest | None = None) -> str | None:
    """provider 名 → Provider 子类 binary name（清单 providers.provider）。"""
    mf = manifest or load_manifest()
    for prov, _cls, pcls in mf.providers:
        if prov == name:
            return pcls
    return None


def _algorithm_key(s: str) -> str:
    """transformation `DES/ECB/PKCS5Padding` → `des`；普通算法名小写。"""
    return s.split('/', 1)[0].strip().lower()


def user_algorithm_strings(user_infos) -> set:
    out: set = set()
    for ci in user_infos:
        for m in ci.methods:
            for x in (m.instrs or ()):
                v = _str_lit(x)
                if v:
                    out.add(_algorithm_key(v))
    return out


def alias_groups(load, manifest: JcaManifest | None = None) -> dict:
    """算法同义名表：小写名 → 同组全部小写名。

    数据源是清单 alias_sources 的类初始化器：每个构造调用（`invokespecial <init>`）之前、
    自上一个构造调用 / 静态写起累积的字符串常量即一组同义名（KnownOIDs 枚举常量：
    `SHA_1("1.3.14.3.2.26", "SHA-1", "SHA", "SHA1")`——常量名 / OID / 标准名 / 别名）。
    组内含非算法串（常量名、OID）只会让同组名多一个等价入口，不影响选择精度。"""
    mf = manifest or load_manifest()
    out: dict = {}
    for cls in mf.alias_sources:
        ci = load(cls)
        for m in (ci.methods if ci else ()):
            if m.name != '<clinit>':
                continue
            group: list = []
            for ins in (m.instrs or []):
                v = _str_lit(ins)
                if v:
                    group.append(v.lower())
                    continue
                op = ins.opcode or ''
                if op == 'invokespecial' and '<init>' in (ins.comment or ''):
                    names = set(group)
                    for n in names:
                        out.setdefault(n, set()).update(names)
                    group = []
                elif op == 'putstatic':
                    group = []
    return out


# engine 调用参数窗口：`ldc 算法; [ldc provider;] invokestatic <Engine>.getInstance`
_ENGINE_ARG_WINDOW = 3


def engine_call_strings(instrs, types: set) -> set:
    """方法体中紧邻 `<engine 类>.getInstance` 静态调用之前的字符串常量（算法名 / provider 名）。
    engine 类按 JCA 约定：简单名 == 服务类型。"""
    out: set = set()
    seq = [x for x in (instrs or []) if x.opcode]
    for k, ins in enumerate(seq):
        if ins.opcode != 'invokestatic':
            continue
        c = ins.comment or ''
        if not c.startswith('Method '):
            continue
        owner_member = c[len('Method '):].split(':', 1)[0]
        owner, _, member = owner_member.rpartition('.')
        if member != 'getInstance' or owner.rsplit('/', 1)[-1] not in types:
            continue
        for prev in seq[max(0, k - _ENGINE_ARG_WINDOW):k]:
            v = _str_lit(prev)
            if v:
                out.add(_algorithm_key(v))
    return out


def select_services(services, algorithms: set, live_types: set, aliases: dict | None = None,
                    forced: set | None = None) -> list:
    """入选服务：类型的 engine 类在调用链上 且 算法名（或其同义名）在候选算法串中；
    forced 为 (类型, 算法) 缺省服务（触发成员在链上即入选）。"""
    aliases = aliases or {}
    forced = forced or set()

    def _hit(s) -> bool:
        a = s.algorithm.lower()
        return a in algorithms or bool(aliases.get(a, set()) & algorithms)
    return [s for s in services
            if (s.type, s.algorithm) in forced or (s.type in live_types and _hit(s))]


def released(cls: str, manifest: JcaManifest) -> bool:
    """边界前缀下的类是否按 K-JCA 放行（清单 release 行：`/` 结尾为包前缀，否则为类及其嵌套类）。

    放行集是静态清单而非「入选实现类同包」的动态推导：边界判定在 BFS 多个决策点
    （入队、clinit 提取、字段发现）经同一入口求值，须与触达先后无关。放行类只有在
    调用链上才生成；它们对其余内部类（sun/security/util、sun/security/jca）的调用
    仍在边界截断。"""
    outer = cls.split('$', 1)[0]
    for r in manifest.release:
        if (r.endswith('/') and cls.startswith(r)) or outer == r:
            return True
    return False
