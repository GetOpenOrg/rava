//! seeds.toml 反射种子：经类名反射 / VM 装载进入运行期、没有静态调用边的类与成员的结构判定。
//! 引擎侧的补种时机见 `engine/seeds.rs`。

pub mod annotation;
pub mod data_bundle;
pub mod jca;
pub mod locale;
pub mod services;

use annotation::AnnoCfg;
use data_bundle::Carriers;
use jca::JcaCfg;
use locale::LocaleCfg;
use services::ServicesCfg;

#[derive(Debug, Default)]
pub struct SeedCfg {
    pub annotation: AnnoCfg,
    pub locale: LocaleCfg,
    pub jca: JcaCfg,
    pub carriers: Carriers,
    pub services: ServicesCfg,
}

impl SeedCfg {
    pub fn from_toml(t: &toml::Table) -> Self {
        let carriers: Vec<String> = t
            .get("data_bundle")
            .and_then(|s| s.get("carriers"))
            .and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
            .unwrap_or_default();
        SeedCfg {
            annotation: AnnoCfg::from_toml(t.get("annotation")),
            locale: LocaleCfg::from_toml(t.get("locale")),
            jca: JcaCfg::from_toml(t.get("jca")),
            carriers: Carriers::new(&carriers),
            services: ServicesCfg::from_toml(t.get("services")),
        }
    }
}
