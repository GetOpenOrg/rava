//! `sun/nio/fs/UnixFileSystem` 手写伴生：POSIX 原生族档 A（切入序 3）。
//!
//! 用例面：构造（默认目录快照 + 根目录）、provider()/getPath()（Path.of 链的
//! vtable 槽位）。getSeparator/isOpen/isReadOnly 等保持 panic 存根。

use crate::prelude::*;
use super::unix_file_system_provider::UnixFileSystemProvider;
use super::unix_path::UnixPath;
use crate::java::lang::String;
use crate::java::nio::file::spi::FileSystemProvider;
use crate::sun::security::action::GetPropertyAction;

/// `Util.toBytes(String)` 等价：jnuEncoding（macOS/Linux 默认 UTF-8）编码。
/// 档 A 以 UTF-8 直编码承载（JLA.getBytesNoRepl 同语；POSIX locale 下 jnu
/// 编码非 UTF-8 的形态留档 B）。
fn to_bytes(s: &String) -> Vec<u8> {
    format!("{}", s).into_bytes()
}

impl super::unix_file_system::implref::UnixFileSystem {
    /// `<init>(UnixFileSystemProvider, String)`：JDK 构造语义——
    /// defaultDirectory = normalizeAndCheck(dir) 的 jnu 编码；必须以 '/' 开头；
    /// chdirAllowed（sun.nio.fs.chdirAllowed，缺省 false）为真或 cwd 与默认目录
    /// 不一致时 needToResolveAgainstDefaultDirectory = true；rootDirectory = "/"。
    #[jvm_boundary]
    pub fn new(provider: UnixFileSystemProvider, dir: String) -> Result<Self> {
        let normalized = UnixPath::normalizeAndCheck(Clone::clone(&dir))?;
        let default_dir = to_bytes(&normalized);
        if default_dir.first().copied() != Some(b'/') {
            return Err(JvmError::from(crate::java::lang::RuntimeException::new_str(
                String::from("default directory must be absolute"),
            )?));
        }

        let prop_value = GetPropertyAction::privilegedGetProperty_str_str(
            String::from("sun.nio.fs.chdirAllowed"),
            String::from("false"),
        )?;
        let prop_str = format!("{}", prop_value);
        let chdir_allowed = if prop_str.is_empty() {
            true
        } else {
            prop_str.eq_ignore_ascii_case("true")
        };
        let need_resolve = if chdir_allowed {
            true
        } else {
            // UnixNativeDispatcher.getcwd() 的 std 等价物（报告 §4）
            let cwd = std::env::current_dir().unwrap_or_default();
            let cwd_bytes = cwd.to_str().unwrap_or("").as_bytes().to_vec();
            cwd_bytes != default_dir
        };

        let mut this = Self::default();
        this._init_not_null();
        this.__set_provider(provider);
        this.__set_defaultDirectory(JArray::from(
            default_dir.into_iter().map(|b| b as i8).collect::<Vec<i8>>(),
        ));
        this.__set_needToResolveAgainstDefaultDirectory(need_resolve);
        let root = UnixPath::new_unixfilesystem_str(
            Clone::clone(&this),
            String::from("/"),
        )?;
        this.__set_rootDirectory(root);
        Ok(this)
    }

    /// `defaultDirectory()`：默认目录（jnu 编码字节）。
    #[jvm_boundary]
    pub fn defaultDirectory(&self) -> Result<JArray<i8>> {
        Ok(Clone::clone(&self.__get_defaultDirectory()))
    }

    /// `needToResolveAgainstDefaultDirectory()`：相对路径是否需对默认目录解析。
    #[jvm_boundary]
    pub fn needToResolveAgainstDefaultDirectory(&self) -> Result<bool> {
        Ok(self.__get_needToResolveAgainstDefaultDirectory())
    }

    /// `provider()`：本文件系统的 provider（java.nio.file.FileSystem 槽位，
    /// Files.provider 链的 vtable 分派目标——`__impl_` 形态保声明进 vtable）。
    #[jvm_boundary]
    pub fn __impl_provider(&self) -> Result<FileSystemProvider> {
        Ok(Clone::clone(&self.__get_provider()).into())
    }

    /// `getPath(String, String...)`：Path.of 的最终落点（同上 vtable 槽位形态）。
    /// JDK 语义：more 为空取 first；否则以 '/' 连接非空段。
    #[jvm_boundary]
    pub fn __impl_getPath(&self, first: String, more: JArray<String>) -> Result<Object> {
        let mut path = format!("{}", first);
        let n = more.len()?;
        for i in 0..n {
            let segment = more.get(i)?;
            let seg = format!("{}", segment);
            if !seg.is_empty() {
                if !path.is_empty() {
                    path.push('/');
                }
                path.push_str(&seg);
            }
        }
        let p = UnixPath::new_unixfilesystem_str(Clone::clone(self), String::from(path.as_str()))?;
        Ok(Object::from(p))
    }
}
