"""JCA 服务注册（K-JCA，docs/plans/2026-09-25-jca-service-registry.md）。

`Cipher.getInstance("DES")` / `MessageDigest.getInstance("MD5")` 经 provider 服务表按类名
反射构造实现类（`Provider$Service.newInstance`），调用链上没有静态边。与 L-1 资源束同构：

- 服务表：provider 注册方法的字节码里 `ldc 类型; ldc 算法; ldc 实现类名` 三连字符串常量
  （SunJCE `ps(..)` / SunEntries `add(..)` / `addWithAlias(..)` 同形态），实现类名须可加载；
- 种子：服务类型的 engine 类（JCA 约定：engine 类简单名 == 服务类型）有方法在调用链上，
  且算法名（大小写不敏感）出现在用户类 String 常量里——transformation
  `DES/ECB/PKCS5Padding` 取首段；
- 放行：清单 `release` 行（算法实现包 + engine / SPI 类）从边界前缀放行，按字节码翻译。

类名全部来自 runtime 清单 `jca_providers.txt`（原则 4）。
"""
from __future__ import annotations

from dataclasses import dataclass

from .runtime_manifest import read_list


@dataclass(frozen=True)
class JcaManifest:
    providers: tuple = ()          # (provider 名, 注册类)
    release: tuple = ()            # 放行的类 / 包前缀（`/` 结尾为包）
    triggers: frozenset = frozenset()   # (类, 成员名)


@dataclass(frozen=True, order=True)
class Service:
    type: str
    algorithm: str
    impl: str          # 斜线形态 binary name
    provider: str


def load_manifest() -> JcaManifest:
    provs, release, triggers = [], [], set()
    for ln in read_list('jca_providers.txt'):
        kind, val = ln.split(None, 1)
        val = val.strip()
        if kind == 'provider':
            name, cls = val.split()
            provs.append((name, cls))
        elif kind == 'release':
            release.append(val)
        elif kind == 'trigger':
            cls, _, member = val.rpartition('.')
            triggers.add((cls, member))
    return JcaManifest(tuple(provs), tuple(release), frozenset(triggers))


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
    for prov, cls in mf.providers:
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


def select_services(services, algorithms: set, live_types: set) -> list:
    """入选服务：类型的 engine 类在调用链上 且 算法名在用户字符串常量中。"""
    return [s for s in services
            if s.type in live_types and s.algorithm.lower() in algorithms]


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
