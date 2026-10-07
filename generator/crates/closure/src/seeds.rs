//! seeds.toml 反射种子：经类名反射 / VM 装载进入运行期、没有静态调用边的类与成员的结构判定。
//! 引擎侧的补种时机见 `engine/seeds.rs`。

pub mod annotation;
pub mod bundles;
pub mod jca;
pub mod jca_order;
pub mod locale;
pub mod services;

use annotation::AnnoCfg;
use bundles::BundleCfg;
use jca::JcaCfg;
use locale::LocaleCfg;
use services::ServicesCfg;

#[derive(Debug, Default)]
pub struct SeedCfg {
    pub annotation: AnnoCfg,
    pub locale: LocaleCfg,
    pub jca: JcaCfg,
    pub services: ServicesCfg,
    pub bundles: BundleCfg,
}

impl SeedCfg {
    pub fn from_toml(t: &toml::Table) -> Self {
        SeedCfg {
            annotation: AnnoCfg::from_toml(t.get("annotation")),
            locale: LocaleCfg::from_toml(t.get("locale")),
            jca: JcaCfg::from_toml(t.get("jca")),
            services: ServicesCfg::from_toml(t.get("services")),
            bundles: BundleCfg::from_toml(t.get("bundles")),
        }
    }
}
