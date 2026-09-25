//! 纯数据资源束注册表（L-1，`docs/plans/2026-09-25-l1-cldr-locale-data.md`）。
//!
//! JDK 的 `ResourceBundle.getBundle` 按类名反射实例化 CLDR 资源束类（无静态调用边）。
//! 原生侧在「装载」这一跳截断：codegen 对入选 locale（用户字节码静态可见的 locale
//! 引用 + 父链，见 `codegen/locale_seed.py`）的束类照常翻译字节码，并在生成 main
//! 启动时经 [`register_data_bundles`] 登记 `(binary name, 构造闭包)`——与
//! reflect_dispatch / 类初始化钩子同一登记模式。束内数据消费（getObject → 父链回退 →
//! ListResourceBundle.handleGetObject → getContents）全部走翻译字节码。
//!
//! 构造闭包返回 `Object`（束类的运行时对象）：本模块属 VM 基础设施，不依赖
//! `ResourceBundle` 是否进入闭包；消费方（`LocaleData` 手写边界）自行取视图。

use crate::error::Result;
use crate::java::lang::Object;
use std::collections::HashMap;

/// 束类构造闭包（调用翻译出的无参构造器）。
pub type BundleCtor = fn() -> Result<Object>;

std::thread_local! {
    static BUNDLES: std::cell::RefCell<HashMap<&'static str, BundleCtor>> =
        std::cell::RefCell::new(HashMap::new());
}

/// 生成项目 main 启动时登记入选束（binary name 斜线形态；重登记幂等）。
pub fn register_data_bundles(bundles: &[(&'static str, BundleCtor)]) {
    BUNDLES.with(|b| {
        let mut b = b.borrow_mut();
        for (name, ctor) in bundles {
            b.insert(*name, *ctor);
        }
    });
}

/// 按 binary name 取束类构造闭包；未入选 → None（调用方按 ResourceBundle 回退语义
/// 继续尝试更一般的候选）。
pub fn lookup(name: &str) -> Option<BundleCtor> {
    BUNDLES.with(|b| b.borrow().get(name).copied())
}
