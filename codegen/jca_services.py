"""JCA 服务注册（K-JCA，docs/plans/2026-09-25-jca-service-registry.md）——发射期清单查询。

`Cipher.getInstance("DES")` / `MessageDigest.getInstance("MD5")` 经 provider 服务表按类名
反射构造实现类（`Provider$Service.newInstance`），调用链上没有静态边。服务表抽取与种子选择
由 Rust 闭包分析器完成（generator/crates/closure/src/seeds/jca.rs，closure.json `seeds.jca`）；
本模块只保留发射层与边界判定需要的清单查询：

- 放行：清单 `release` 行（算法实现包 + engine / SPI 类）从边界前缀放行，按字节码翻译；
- provider 名 → Provider 子类（生成 main 的 provider 构造闭包登记）。

类名全部来自 runtime 清单 `seeds.toml` [jca]（原则 4）。
"""
from __future__ import annotations

from dataclasses import dataclass

from .runtime_manifest import seed_section, jca_release_entries


@dataclass(frozen=True)
class JcaManifest:
    providers: tuple = ()          # (provider 名, 注册类, Provider 子类)
    release: tuple = ()            # 放行的类 / 包前缀（`/` 结尾为包）


@dataclass(frozen=True, order=True)
class Service:
    type: str
    algorithm: str
    impl: str          # 斜线形态 binary name
    provider: str


def load_manifest() -> JcaManifest:
    """seeds.toml [jca]：provider 注册类与放行条目。"""
    sec = seed_section('jca')
    provs = tuple((p['name'], p['class'], p['provider']) for p in sec.get('providers', []))
    return JcaManifest(provs, tuple(jca_release_entries()))


def provider_class(name: str, manifest: JcaManifest | None = None) -> str | None:
    """provider 名 → Provider 子类 binary name（清单 providers.provider）。"""
    mf = manifest or load_manifest()
    for prov, _cls, pcls in mf.providers:
        if prov == name:
            return pcls
    return None


def released(cls: str, manifest: JcaManifest) -> bool:
    """边界前缀下的类是否按 K-JCA 放行（清单 release 行：`/` 结尾为包前缀，否则为类及其嵌套类）。

    放行集是静态清单而非「入选实现类同包」的动态推导：边界判定在多个发射决策点
    （clinit 提取、审计、闭包分类）经同一入口求值，须与触达先后无关。"""
    outer = cls.split('$', 1)[0]
    for r in manifest.release:
        if (r.endswith('/') and cls.startswith(r)) or outer == r:
            return True
    return False
