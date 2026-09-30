use crate::prelude::*;
use super::static_property::StaticProperty;

// 内部边界类 jdk.internal.util.StaticProperty：VM 启动时快照的系统属性。
// 按 java.util.Locale 调用链按需实现默认区域相关属性，其余保持 panic 存根。
// 取值与 JDK 的 System.initPhase1 一致：user.language / user.country 来自进程环境
// （LC_ALL / LC_MESSAGES / LANG，形如 `en_US.UTF-8`），缺省为 en / 空；
// *_DISPLAY / *_FORMAT 未单独设置时回落到基础属性。

fn posix_locale() -> (std::string::String, std::string::String) {
    let raw = ["LC_ALL", "LC_MESSAGES", "LANG"].iter()
        .filter_map(|name| std::env::var(name).ok())
        .find(|value| !value.is_empty())
        .unwrap_or_default();
    let tag = raw.split(['.', '@']).next().unwrap_or("");
    if tag.is_empty() || tag == "C" || tag == "POSIX" {
        return ("en".to_owned(), std::string::String::new());
    }
    let mut parts = tag.splitn(2, '_');
    let language = parts.next().unwrap_or("en").to_owned();
    let country = parts.next().unwrap_or("").to_owned();
    (language, country)
}

impl StaticProperty {
    /// java.home：原生二进制无真实 JDK 安装目录，返回稳定伪值锚定嵌入资源
    /// 路径协议（`<JAVA_RUNTIME_HOME>/lib/tzdb.dat` 在 FileInputStream.open0
    /// 的资源重定向层命中，见 jdk_resources 模块）。与 JDK 快照属性语义一致：
    /// 恒定、进程内不变（System.props 的 java.home 同源同值）。
    #[jvm_boundary]
    pub fn javaHome() -> Result<String> {
        Ok(String::from(crate::jdk_resources::JAVA_RUNTIME_HOME))
    }

    /// user.dir：进程工作目录（JDK 快照自 initPhase1 的 user.dir 属性）。
    #[jvm_boundary]
    pub fn userDir() -> Result<String> {
        Ok(String::from(std::env::current_dir()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default()
            .as_str()))
    }

    /// user.home：$HOME（POSIX 语义）。
    #[jvm_boundary]
    pub fn userHome() -> Result<String> {
        Ok(String::from(std::env::var("HOME").unwrap_or_default().as_str()))
    }

    /// user.name：$USER，回落 $LOGNAME。
    #[jvm_boundary]
    pub fn userName() -> Result<String> {
        Ok(String::from(std::env::var("USER")
            .or_else(|_| std::env::var("LOGNAME"))
            .unwrap_or_default()
            .as_str()))
    }

    /// java.io.tmpdir：$TMPDIR，回落 /tmp。
    #[jvm_boundary]
    pub fn javaIoTmpDir() -> Result<String> {
        Ok(String::from(std::env::var("TMPDIR")
            .unwrap_or_else(|_| std::string::String::from("/tmp"))
            .as_str()))
    }

    #[jvm_boundary]
    pub fn USER_LANGUAGE() -> Result<String> { Ok(String::from(posix_locale().0.as_str())) }
    #[jvm_boundary]
    pub fn USER_LANGUAGE_DISPLAY() -> Result<String> { Self::USER_LANGUAGE() }
    #[jvm_boundary]
    pub fn USER_LANGUAGE_FORMAT() -> Result<String> { Self::USER_LANGUAGE() }

    #[jvm_boundary]
    pub fn USER_SCRIPT() -> Result<String> { Ok(String::from("")) }
    #[jvm_boundary]
    pub fn USER_SCRIPT_DISPLAY() -> Result<String> { Self::USER_SCRIPT() }
    #[jvm_boundary]
    pub fn USER_SCRIPT_FORMAT() -> Result<String> { Self::USER_SCRIPT() }

    #[jvm_boundary]
    pub fn USER_COUNTRY() -> Result<String> { Ok(String::from(posix_locale().1.as_str())) }
    #[jvm_boundary]
    pub fn USER_COUNTRY_DISPLAY() -> Result<String> { Self::USER_COUNTRY() }
    #[jvm_boundary]
    pub fn USER_COUNTRY_FORMAT() -> Result<String> { Self::USER_COUNTRY() }

    #[jvm_boundary]
    pub fn USER_VARIANT() -> Result<String> { Ok(String::from("")) }
    #[jvm_boundary]
    pub fn USER_VARIANT_DISPLAY() -> Result<String> { Self::USER_VARIANT() }
    #[jvm_boundary]
    pub fn USER_VARIANT_FORMAT() -> Result<String> { Self::USER_VARIANT() }

    #[jvm_boundary]
    pub fn USER_EXTENSIONS() -> Result<String> { Ok(String::from("")) }
    #[jvm_boundary]
    pub fn USER_EXTENSIONS_DISPLAY() -> Result<String> { Self::USER_EXTENSIONS() }
    #[jvm_boundary]
    pub fn USER_EXTENSIONS_FORMAT() -> Result<String> { Self::USER_EXTENSIONS() }

    /// `user.dir`：VM 启动时的工作目录快照（System.initPhase1 与
    /// StaticProperty.userDir() 同源）。原生二进制语义 = 进程启动时的
    /// 当前目录；目录形态不规约（JDK 不做 canonicalize——UnixFileSystem
    /// 构造侧的 Util.normalize 负责规约）。
    #[jvm_boundary]
    pub fn USER_DIR() -> Result<String> {
        let cwd = std::env::current_dir().unwrap_or_default();
        Ok(String::from(cwd.to_str().unwrap_or("")))
    }

    // ── 编码族快照（System.initPhase1 → StaticProperty.<clinit>；边界类 <clinit> 不翻译，
    //   静态字段由同名访问器承载，取值与 System 属性 native.encoding / sun.jnu.encoding /
    //   file.encoding 同源：posix::native_encoding）────────────────────────

    /// `sun.jnu.encoding`：文件名 / 环境变量 / 命令行的平台编码（ProcessImpl.JNU_CHARSET、
    /// ProcessEnvironment 的消费方）。
    #[jvm_boundary]
    pub fn SUN_JNU_ENCODING() -> Result<String> {
        Ok(String::from(crate::posix::native_encoding().as_str()))
    }

    /// `native.encoding`：宿主区域 codeset（JEP 400）。
    #[jvm_boundary]
    pub fn NATIVE_ENCODING() -> Result<String> {
        Self::SUN_JNU_ENCODING()
    }

    /// `file.encoding`：JDK 18+ 缺省 UTF-8（JEP 400）。
    #[jvm_boundary]
    pub fn FILE_ENCODING() -> Result<String> {
        Ok(String::from("UTF-8"))
    }

    // ── 公开访问器（字段快照同值；JDK 的 StaticProperty.xxx() 直接返回对应静态字段）────

    #[jvm_boundary]
    pub fn nativeEncoding() -> Result<String> {
        Self::NATIVE_ENCODING()
    }

    #[jvm_boundary]
    pub fn fileEncoding() -> Result<String> {
        Self::FILE_ENCODING()
    }

    #[jvm_boundary]
    pub fn jnuEncoding() -> Result<String> {
        Self::SUN_JNU_ENCODING()
    }

    /// `java.library.path`：原生单二进制无 JNI 库搜索路径（System 属性同值：空串）。
    #[jvm_boundary]
    pub fn javaLibraryPath() -> Result<String> {
        Ok(String::from(""))
    }

    /// `sun.boot.library.path`：`<java.home>/lib`（System 属性同值）。
    #[jvm_boundary]
    pub fn sunBootLibraryPath() -> Result<String> {
        Ok(String::from(format!("{}/lib", crate::jdk_resources::JAVA_RUNTIME_HOME).as_str()))
    }

    /// `jdk.serialFilter`：未设置 → null。
    #[jvm_boundary]
    pub fn jdkSerialFilter() -> Result<String> {
        Ok(String::default())
    }

    /// `jdk.serialFilterFactory`：未设置 → null。
    #[jvm_boundary]
    pub fn jdkSerialFilterFactory() -> Result<String> {
        Ok(String::default())
    }

    /// `java.properties.date`：未设置 → null（Properties.store 写当前时间注释）。
    #[jvm_boundary]
    pub fn javaPropertiesDate() -> Result<String> {
        Ok(String::default())
    }

    /// `java.locale.useOldISOCodes`：缺省空串（JDK getProperty(props, key, "")）。
    #[jvm_boundary]
    pub fn javaLocaleUseOldISOCodes() -> Result<String> {
        Ok(String::from(""))
    }

    #[jvm_boundary]
    pub fn osName() -> Result<String> {
        Ok(String::from(crate::posix::os_name()))
    }

    #[jvm_boundary]
    pub fn osArch() -> Result<String> {
        Ok(String::from(crate::posix::os_arch()))
    }

    #[jvm_boundary]
    pub fn osVersion() -> Result<String> {
        Ok(String::from(crate::posix::os_release().as_str()))
    }
}
