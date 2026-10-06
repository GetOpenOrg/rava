//! `jdk.internal.util.SystemProps$Raw` 的 native 层（类 ①：`ACC_NATIVE`）。
//!
//! HotSpot 由 `java_props_md.c` / `Arguments` 填两张原始表：`platformProperties()` 按类内常量
//! `_<名>_NDX` 定位（长度 `FIXED_LENGTH`，未知项为 null），`vmProperties()` 为键值交替的数组。
//! 构建期引导映像把这两张表里随宿主变化的项物化为「宿主相关」内容，启动序列调用本 native 一次、以
//! 结果改写映像字符串（计划 2026-10-05-boot-image-evaluator §5.5.2 D3）——下标即类文件常量，
//! `vmProperties` 的键序与构建期模型（vm_intrinsics.toml `[concrete.boot] vm_props`）一致。

use crate::prelude::*;
use super::system_props_raw::SystemProps_Raw;

fn s(v: &str) -> String {
    String::from(v)
}

fn owned(v: std::string::String) -> String {
    String::from_owned(v)
}

/// 标准流编码：只在该流是终端时设置（HotSpot `isatty` 判定），否则 null
fn tty_encoding(is_tty: bool) -> String {
    if is_tty {
        owned(crate::posix::native_encoding())
    } else {
        String::default()
    }
}

impl SystemProps_Raw {
    /// native `platformProperties()`：平台属性原始表（下标为类文件常量 `_<名>_NDX`）
    #[jvm_native]
    pub fn platformProperties() -> Result<JArray<String>> {
        use std::io::IsTerminal;
        let mut t: Vec<String> = (0..Self::FIXED_LENGTH).map(|_| String::default()).collect();
        let mut put = |i: i32, v: String| t[i as usize] = v;
        let (language, country) = crate::posix::locale();
        let country = if country.is_empty() { String::default() } else { owned(country) };
        put(Self::_display_language_NDX, owned(language.clone()));
        put(Self::_display_country_NDX, country.clone());
        put(Self::_format_language_NDX, owned(language));
        put(Self::_format_country_NDX, country);
        put(Self::_file_separator_NDX, s("/"));
        put(Self::_path_separator_NDX, s(":"));
        put(Self::_line_separator_NDX, s("\n"));
        put(Self::_java_io_tmpdir_NDX, owned(std::env::var("TMPDIR").unwrap_or_else(|_| "/tmp".into())));
        put(Self::_os_arch_NDX, s(crate::posix::os_arch()));
        put(Self::_os_name_NDX, s(crate::posix::os_name()));
        put(Self::_os_version_NDX, owned(crate::posix::os_release()));
        put(Self::_sun_arch_data_model_NDX, s(if cfg!(target_pointer_width = "64") { "64" } else { "32" }));
        put(Self::_sun_cpu_endian_NDX, s(if cfg!(target_endian = "little") { "little" } else { "big" }));
        put(Self::_sun_io_unicode_encoding_NDX, s(if cfg!(target_endian = "little") { "UnicodeLittle" } else { "UnicodeBig" }));
        put(Self::_sun_jnu_encoding_NDX, owned(crate::posix::native_encoding()));
        put(Self::_stdout_encoding_NDX, tty_encoding(std::io::stdout().is_terminal()));
        put(Self::_stderr_encoding_NDX, tty_encoding(std::io::stderr().is_terminal()));
        #[cfg(jdk_ge_25)]
        {
            put(Self::_native_encoding_NDX, owned(crate::posix::native_encoding()));
            put(Self::_stdin_encoding_NDX, tty_encoding(std::io::stdin().is_terminal()));
        }
        #[cfg(not(jdk_ge_25))]
        put(Self::_file_encoding_NDX, owned(crate::posix::native_encoding()));
        put(Self::_user_dir_NDX, owned(std::env::current_dir().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default()));
        put(Self::_user_home_NDX, owned(std::env::var("HOME").unwrap_or_default()));
        put(Self::_user_name_NDX, owned(crate::posix::current_user_name()));
        Ok(JArray::from(t))
    }

    /// native `vmProperties()`：VM 属性（键值交替）。键序与构建期模型一致：宿主相关的两项在前
    #[jvm_native]
    pub fn vmProperties() -> Result<JArray<String>> {
        let home = crate::jdk_resources::JAVA_RUNTIME_HOME;
        let feature = crate::jdk_feature().to_string();
        let pairs: [(&str, std::string::String); 13] = [
            ("java.home", home.into()),
            ("sun.boot.library.path", format!("{home}/lib")),
            ("java.library.path", "".into()),
            ("java.class.path", "".into()),
            ("java.vm.specification.name", "Java Virtual Machine Specification".into()),
            ("java.vm.specification.vendor", "Oracle Corporation".into()),
            ("java.vm.specification.version", feature.clone()),
            ("java.vm.name", "rava native runtime".into()),
            ("java.vm.vendor", "rava".into()),
            ("java.vm.version", feature),
            ("java.vm.info", "native image".into()),
            ("jdk.debug", "release".into()),
            ("jdk.reflect.useNativeAccessorOnly", "true".into()),
        ];
        Ok(JArray::from(pairs.into_iter().flat_map(|(k, v)| [s(k), owned(v)]).collect::<Vec<_>>()))
    }
}
