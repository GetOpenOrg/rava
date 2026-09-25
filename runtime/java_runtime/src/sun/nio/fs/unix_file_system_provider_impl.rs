//! `sun/nio/fs/UnixFileSystemProvider` 手写伴生：POSIX 原生族档 A（切入序 2/4/5）。
//!
//! 按用例面实现（TestFilesApi：writeString/readString/readAllLines/mismatch/
//! exists/deleteIfExists）：构造链、theFileSystem/getFileSystem/getScheme 与
//! checkAccess/exists/readAttributesIfExists/implDelete/newByteChannel 的 Unix
//! 语义体；其余成员保持 panic 存根（档 A 纪律）。

use crate::prelude::*;
use super::unix_file_system_provider::UnixFileSystemProvider;
use super::unix_file_system::implref::UnixFileSystem;
use super::unix_channel_factory::UnixChannelFactory;
use super::unix_exception::UnixException;
use super::unix_file_attributes::UnixFileAttributes;
use super::unix_native_dispatcher::UnixNativeDispatcher;
use super::unix_native_dispatcher_impl::consts;
use super::unix_path::UnixPath;
use crate::java::lang::String;
use crate::java::net::URI;
use crate::java::nio::channels::FileChannel;
use crate::java::nio::file::AccessMode;
use crate::java::nio::file::LinkOption;
use crate::java::util::Set;
use crate::jdk::internal::util::StaticProperty;

/// `Util.followLinks(LinkOption...)` 等价：无 NOFOLLOW_LINKS 即跟随。
fn follow_links(options: &JArray<LinkOption>) -> Result<bool> {
    let n = options.len()?;
    for i in 0..n {
        let opt = options.get(i)?;
        if Object::from(Clone::clone(&opt)) == Object::from(Clone::clone(&LinkOption::NOFOLLOW_LINKS()?)) {
            return Ok(false);
        }
    }
    Ok(true)
}

impl UnixFileSystemProvider {
    /// `<init>()`: `theFileSystem = newFileSystem(StaticProperty.userDir())`。
    /// 抽象基类——实际由平台薄层（macOS/Linux）构造，newFileSystem 虚分派
    /// 解析到平台覆写；本形态以基类 UnixFileSystem 承载（档 A 落差见
    /// mac_osx_file_system_provider_impl 的注释）。
    #[jvm_boundary(upcalls = "sun/nio/fs/UnixFileSystem.<init>:(Lsun/nio/fs/UnixFileSystemProvider;Ljava/lang/String;)V")]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        let dir = StaticProperty::USER_DIR()?;
        let fs = UnixFileSystem::new(Clone::clone(&this), dir)?;
        this.__set_theFileSystem(fs);
        Ok(this)
    }

    /// `theFileSystem()`：默认文件系统访问器（本包内直调形态）。
    #[jvm_boundary]
    pub fn theFileSystem(&self) -> Result<UnixFileSystem> {
        Ok(self.__get_theFileSystem())
    }

    /// `getScheme()`: 恒为 "file"。
    #[jvm_boundary]
    pub fn getScheme(&self) -> Result<String> {
        Ok(String::from("file"))
    }

    /// `checkUri(URI)`: file URI 形态校验（scheme/authority/path/query/fragment）。
    /// 四个 URI 组件均为 String wrapper：null 判定走 vtable 钩子 `is_jvm_null()`
    /// （`_is_jnull` 只对 `Object` 载体生效、对 wrapper 恒 false——原判定全死，
    /// 组件判空形同虚设）。
    #[jvm_boundary(upcalls = "java/net/URI.getScheme:()Ljava/lang/String; java/net/URI.getRawAuthority:()Ljava/lang/String; java/net/URI.getPath:()Ljava/lang/String; java/net/URI.getRawQuery:()Ljava/lang/String; java/net/URI.getRawFragment:()Ljava/lang/String; java/lang/String.equalsIgnoreCase:(Ljava/lang/String;)Z java/lang/IllegalArgumentException.<init>:(Ljava/lang/String;)V")]
    pub fn checkUri(&self, uri: URI) -> Result<()> {
        let scheme_ok = uri.getScheme()?
            .equalsIgnoreCase(Clone::clone(&self.getScheme()?))?;
        if !scheme_ok {
            return Err(JvmError::from(
                crate::java::lang::IllegalArgumentException::new_str(String::from(
                    "URI does not match this provider",
                ))?,
            ));
        }
        let authority = uri.getRawAuthority()?;
        if !authority.is_jvm_null() {
            return Err(JvmError::from(
                crate::java::lang::IllegalArgumentException::new_str(String::from(
                    "Authority component present",
                ))?,
            ));
        }
        let path = uri.getPath()?;
        if path.is_jvm_null() {
            return Err(JvmError::from(
                crate::java::lang::IllegalArgumentException::new_str(String::from(
                    "Path component is undefined",
                ))?,
            ));
        }
        if !String::from("/") .equals(Object::from(Clone::clone(&path)))? {
            return Err(JvmError::from(
                crate::java::lang::IllegalArgumentException::new_str(String::from(
                    "Path component should be '/'",
                ))?,
            ));
        }
        let query = uri.getRawQuery()?;
        if !query.is_jvm_null() {
            return Err(JvmError::from(
                crate::java::lang::IllegalArgumentException::new_str(String::from(
                    "Query component present",
                ))?,
            ));
        }
        let fragment = uri.getRawFragment()?;
        if !fragment.is_jvm_null() {
            return Err(JvmError::from(
                crate::java::lang::IllegalArgumentException::new_str(String::from(
                    "Fragment component present",
                ))?,
            ));
        }
        Ok(())
    }

    /// `getFileSystem(URI)`: checkUri 后返回 theFileSystem（final，声明类即实现体）。
    #[jvm_boundary(upcalls = "java/nio/file/FileSystemAlreadyExistsException.<init>:()V")]
    pub fn __impl_getFileSystem(
        &self,
        uri: URI,
    ) -> Result<crate::java::nio::file::file_system::implref::FileSystem> {
        self.checkUri(uri)?;
        Ok(Clone::clone(&self.__get_theFileSystem()).into())
    }
}

impl UnixFileSystemProvider {
    /// `checkAccess(Path, AccessMode...)`：modes 空 → F_OK；否则 R/W/X 组合
    /// （JDK 语义）。access 返回 errno，非 0 以 UnixException 翻译抛出。
    /// R_OK/W_OK/X_OK 的档 A 近似见 UnixNativeDispatcher::access 注释
    /// （用例面：exists → 空 modes → F_OK 精确）。
    #[jvm_boundary]
    pub fn __impl_checkAccess(&self, obj: Object, modes: JArray<AccessMode>) -> Result<()> {
        let file = UnixPath::toUnixPath(Clone::clone(&obj))?;
        let mut e = false;
        let mut r = false;
        let mut w = false;
        let mut x = false;
        let n = modes.len()?;
        if n == 0 {
            e = true;
        } else {
            for i in 0..n {
                let mode = modes.get(i)?;
                match format!("{}", Clone::clone(&Object::from(Clone::clone(&mode)))).as_str() {
                    "READ" => r = true,
                    "WRITE" => w = true,
                    "EXECUTE" => x = true,
                    _ => {}
                }
            }
        }
        let mut mode = 0;
        if e || r {
            file.checkRead()?;
            if r {
                mode |= consts::errno::R_OK;
            }
        }
        if w {
            file.checkWrite()?;
            mode |= consts::errno::W_OK;
        }
        if x {
            mode |= consts::errno::X_OK;
        }
        let errno = UnixNativeDispatcher::access(Clone::clone(&file), mode)?;
        if errno != 0 {
            let x = UnixException::new_i(errno)?;
            x.rethrowAsIOException_unixpath(&file)?;
        }
        Ok(())
    }

    /// `exists(Path, LinkOption...)`：跟随链接 → checkRead + access(F_OK)==0；
    /// 否则 super.exists 语义（lstat 成败）。JDK21 macOS 形态。
    #[jvm_boundary]
    pub fn __impl_exists(&self, path: Object, options: JArray<LinkOption>) -> Result<bool> {
        if follow_links(&options)? {
            let file = UnixPath::toUnixPath(Clone::clone(&path))?;
            file.checkRead()?;
            Ok(UnixNativeDispatcher::access(Clone::clone(&file), consts::errno::F_OK)? == 0)
        } else {
            // AbstractFileSystemProvider/FileSystemProvider 默认法：lstat 成败
            let file = UnixPath::toUnixPath(Clone::clone(&path))?;
            match UnixFileAttributes::get(Clone::clone(&file), false) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        }
    }

    /// `readAttributesIfExists(Path, Class, LinkOption...)`：BasicFileAttributes +
    /// followLinks → stat（不存在返回 null 载体）；其余形态走 super 默认法的
    /// 可观测语义（lstat/stat，不存在 null）。
    #[jvm_boundary]
    pub fn __impl_readAttributesIfExists(
        &self,
        path: Object,
        _type_: crate::java::lang::Class,
        options: JArray<LinkOption>,
    ) -> Result<Object> {
        let file = UnixPath::toUnixPath(Clone::clone(&path))?;
        file.checkRead()?;
        let attrs = UnixFileAttributes::getIfExists(Clone::clone(&file))?;
        // attrs 为 UnixFileAttributes wrapper：null 判定走 vtable 钩子
        // （`_is_jnull` 对 wrapper 恒 false，不存在时会误返回非 null 载体）
        if !attrs.is_jvm_null() {
            return Ok(Object::from(attrs));
        }
        Ok(Object::default())
    }

    /// `implDelete(Path, boolean failIfNotExists)`：stat → 目录 rmdir / 文件
    /// unlink；ENOENT 且 !failIfNotExists → false；EEXIST/ENOTEMPTY 的目录 →
    /// DirectoryNotEmptyException；其余翻译抛出。
    #[jvm_boundary(upcalls = "java/nio/file/DirectoryNotEmptyException.<init>:(Ljava/lang/String;)V")]
    pub fn __impl_implDelete(&self, obj: Object, fail_if_not_exists: bool) -> Result<bool> {
        let file = UnixPath::toUnixPath(Clone::clone(&obj))?;
        file.checkDelete()?;
        let attrs = UnixFileAttributes::get(Clone::clone(&file), false).ok();
        let is_dir = attrs.as_ref().map(|a| a.isDirectory().unwrap_or(false)).unwrap_or(false);
        let result: std::io::Result<()> = if is_dir {
            std::fs::remove_dir(
                std::string::String::from_utf8_lossy(&{
                    let bytes = file.getByteArrayForSysCalls()?;
                    bytes.to_vec().into_iter().map(|b| b as u8).collect::<Vec<u8>>()
                })
                .into_owned(),
            )
        } else {
            std::fs::remove_file(
                std::string::String::from_utf8_lossy(&{
                    let bytes = file.getByteArrayForSysCalls()?;
                    bytes.to_vec().into_iter().map(|b| b as u8).collect::<Vec<u8>>()
                })
                .into_owned(),
            )
        };
        match result {
            Ok(()) => Ok(true),
            Err(e) => {
                let errno = e.raw_os_error().unwrap_or(consts::errno::ENOENT);
                if !fail_if_not_exists && errno == consts::errno::ENOENT {
                    return Ok(false);
                }
                if is_dir && (errno == consts::errno::EEXIST || errno == consts::errno::ENOTEMPTY) {
                    return Err(JvmError::from(
                        crate::java::nio::file::DirectoryNotEmptyException::new(
                            file.getPathForExceptionMessage()?,
                        )?,
                    ));
                }
                UnixException::new_i(errno)?.rethrowAsIOException_unixpath(&file)?;
                Ok(false)
            }
        }
    }

    /// `isSameFile(Path, Path)`：equals 短路 → null/NPE → 类型不符 false →
    /// 双 stat 的 (st_ino, st_dev) 比较（JDK 同款）。
    #[jvm_boundary]
    pub fn __impl_isSameFile(&self, obj1: Object, obj2: Object) -> Result<bool> {
        let file1 = UnixPath::toUnixPath(Clone::clone(&obj1))?;
        if file1.equals(Clone::clone(&obj2))? {
            return Ok(true);
        }
        if _is_jnull(&obj2) {
            return Err(JvmError::null_pointer());
        }
        let Ok(file2) = Clone::clone(&obj2).try_cast::<UnixPath>("sun/nio/fs/UnixPath") else {
            return Ok(false);
        };
        file1.checkRead()?;
        file2.checkRead()?;
        let attrs1 = match UnixFileAttributes::get(Clone::clone(&file1), true) {
            Ok(a) => a,
            Err(e) => {
                if e.is_instance_of("sun/nio/fs/UnixException") {
                    let x: UnixException =
                        e.catch_as::<UnixException>("sun/nio/fs/UnixException");
                    x.rethrowAsIOException_unixpath(&file1)?;
                }
                return Err(e);
            }
        };
        let attrs2 = match UnixFileAttributes::get(Clone::clone(&file2), true) {
            Ok(a) => a,
            Err(e) => {
                if e.is_instance_of("sun/nio/fs/UnixException") {
                    let x: UnixException =
                        e.catch_as::<UnixException>("sun/nio/fs/UnixException");
                    x.rethrowAsIOException_unixpath(&file2)?;
                }
                return Err(e);
            }
        };
        attrs1.isSameFile(attrs2)
    }

    /// `newByteChannel(Path, Set, FileAttribute...)`：toUnixPath → mode
    /// （无属性 → ALL_READWRITE 0666，umask 由宿主 open(2) 施加）→
    /// UnixChannelFactory.newFileChannel；UnixException 翻译抛出。
    #[jvm_boundary]
    pub fn __impl_newByteChannel(
        &self,
        obj: Object,
        options: Set<Object>,
        _attrs: JArray<Object>,
    ) -> Result<Object> {
        let file = UnixPath::toUnixPath(Clone::clone(&obj))?;
        const ALL_READWRITE: i32 = 0o666;
        match UnixChannelFactory::newFileChannel(Clone::clone(&file), options, ALL_READWRITE) {
            Ok(channel) => Ok(Object::from(channel)),
            Err(e) => {
                if e.is_instance_of("sun/nio/fs/UnixException") {
                    let x: UnixException = e.catch_as::<UnixException>("sun/nio/fs/UnixException");
                    x.rethrowAsIOException_unixpath(&file)?;
                }
                Err(e)
            }
        }
    }
}
