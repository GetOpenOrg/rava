//! `jdk/internal/loader/NativeLibraries` 的 ACC_NATIVE（类 1，libjava `NativeLibraries.c`）。
//!
//! 原生二进制里 JDK 本地库的 native 方法全部静态链接在 java_runtime 内（`*_impl.rs`），
//! 对应 JDK 的「内建库」（statically linked library，`JNI_OnLoad_<lib>` 存在于进程中）：
//! `System.loadLibrary("net")` 等经 findBuiltinLib 认定为内建，load 只登记句柄与 JNI 版本。
//! 非 JDK 库需要 dlopen + JNI 调用约定，本运行时不提供 JNI，按加载失败处理。

use crate::prelude::*;
use super::native_libraries::NativeLibraries;
use super::native_libraries_native_library_impl::NativeLibraries_NativeLibraryImpl;

/// JDK 21 随附的本地库（macOS / Linux 两平台的并集，去掉 `lib` 前缀与平台后缀）：
/// 其 native 方法由 java_runtime 静态提供，即 JDK 意义上的内建库。
const BUILTIN_LIBRARIES: &[&str] = &[
    "attach", "awt", "awt_headless", "awt_lwawt", "awt_xawt", "dt_socket", "extnet", "fontmanager",
    "freetype", "instrument", "j2gss", "j2pcsc", "j2pkcs11", "jaas", "java", "javajpeg", "jawt", "jdwp",
    "jimage", "jli", "jsig", "jsound", "lcms", "le", "management", "management_agent", "management_ext",
    "mlib_image", "net", "nio", "osx", "osxapp", "osxkrb5", "osxsecurity", "osxui", "prefs", "rmi",
    "saproc", "sctp", "splashscreen", "syslookup", "verify", "zip",
];

#[cfg(target_os = "macos")]
const LIB_SUFFIX: &str = ".dylib";
#[cfg(not(target_os = "macos"))]
const LIB_SUFFIX: &str = ".so";
const LIB_PREFIX: &str = "lib";

/// JNI_VERSION_1_8：内建库 JNI_OnLoad_<lib> 须返回的最低版本
const JNI_VERSION_1_8: i32 = 0x0001_0008;
/// 内建库的句柄即进程句柄；Java 侧只要求非 0（Unloader 拒绝 0 句柄）
const PROCESS_HANDLE: i64 = -1;

fn error<E>(e: Result<E>) -> JvmError
where
    E: Into<JvmError>,
{
    e.map(Into::into).unwrap_or_else(|e| e)
}

impl NativeLibraries {
    /// native `findBuiltinLib(String filename)`：去掉平台前缀 / 后缀（JNI 只按长度截取，不校验内容）后
    /// 为内建库则返回库名，否则 null；filename 为 null 抛 InternalError。
    #[jvm_native]
    pub fn findBuiltinLib(filename: String) -> Result<String> {
        if filename.is_jvm_null() {
            let e = crate::java::lang::InternalError::new_str(String::from("NULL filename for native library"));
            return Err(error(e));
        }
        let file = filename.to_string();
        if file.len() <= LIB_PREFIX.len() + LIB_SUFFIX.len() {
            return Ok(String::default());
        }
        let Some(name) = file.get(LIB_PREFIX.len()..file.len() - LIB_SUFFIX.len()) else {
            return Ok(String::default());
        };
        Ok(if BUILTIN_LIBRARIES.contains(&name) { String::from(name) } else { String::default() })
    }

    /// native `load(NativeLibraryImpl, String name, boolean isBuiltin, boolean throwExceptionIfFail)`：
    /// 内建库登记进程句柄与 JNI 1.8 后返回 true；非内建库无法以 JNI 加载——
    /// throwExceptionIfFail 时抛 UnsatisfiedLinkError（「Can't load library: <name>」），否则返回 false。
    #[jvm_native]
    pub fn load(lib: NativeLibraries_NativeLibraryImpl, name: String, isBuiltin: bool, throwExceptionIfFail: bool) -> Result<bool> {
        if isBuiltin {
            lib.__set_handle(PROCESS_HANDLE);
            lib.__set_jniVersion(JNI_VERSION_1_8);
            return Ok(true);
        }
        if throwExceptionIfFail {
            let msg = String::from(format!("Can't load library: {}", name.to_string()));
            return Err(error(crate::java::lang::UnsatisfiedLinkError::new_str(msg)));
        }
        Ok(false)
    }

    /// native `unload(String name, boolean isBuiltin, long handle)`：内建库的 JNI_OnUnload_<lib> 不存在，无事可做。
    #[jvm_native]
    pub fn unload(name: String, isBuiltin: bool, handle: i64) -> Result<()> {
        Ok(())
    }
}
