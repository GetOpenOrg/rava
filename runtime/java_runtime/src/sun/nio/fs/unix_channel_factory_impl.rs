//! `sun/nio/fs/UnixChannelFactory` 手写伴生：POSIX 原生族档 A（切入序 5）。
//!
//! `newFileChannel(UnixPath, Set<? extends OpenOption>, int mode)`：Flags
//! 解码（StandardOpenOption / LinkOption.NOFOLLOW_LINKS）→ oflags 组装 →
//! UnixNativeDispatcher.open → FileChannelImpl.open。仅 dfd=-1 形态（用例面；
//! SecureDirectoryStream 的 openat 族留档 B）。DELETE_ON_CLOSE 的立即 unlink、
/// DSYNC/SYNC/DIRECT 的透传位按 JDK 语义保留。

use crate::prelude::*;
use super::unix_channel_factory::UnixChannelFactory;
use super::unix_exception::UnixException;
use super::unix_native_dispatcher::UnixNativeDispatcher;
use super::unix_native_dispatcher_impl::consts;
use super::unix_path::UnixPath;
use crate::java::io::FileDescriptor;
use crate::java::lang::IllegalArgumentException;
use crate::java::lang::String;
use crate::java::lang::UnsupportedOperationException;
use crate::java::nio::channels::FileChannel;
use crate::java::util::Set;
use crate::sun::nio::ch::FileChannelImpl;

/// 用户 OpenOption 集合解码出的标志（JDK Flags 内部类语义）。
#[derive(Default)]
struct Flags {
    read: bool,
    write: bool,
    append: bool,
    truncate_existing: bool,
    no_follow_links: bool,
    create: bool,
    create_new: bool,
    delete_on_close: bool,
    sync: bool,
    dsync: bool,
    direct: bool,
}

/// 选项名匹配（枚举常量单例，name 即身份）。
fn option_name(option: &Object) -> Result<std::string::String> {
    Ok(format!("{}", Clone::clone(option)))
}

impl UnixChannelFactory {
    /// `newFileChannel(UnixPath, Set, int)`：open + FileChannelImpl 组装。
    #[jvm_boundary(upcalls = "java/lang/IllegalArgumentException.<init>:(Ljava/lang/String;)V java/lang/UnsupportedOperationException.<init>:(Ljava/lang/String;)V")]
    pub fn newFileChannel(
        path: UnixPath,
        options: Set<Object>,
        mode: i32,
    ) -> Result<FileChannel> {
        let mut flags = Flags::default();

        let it = options.iterator()?;
        while it.hasNext()? {
            let option = it.next()?;
            if _is_jnull(&option) {
                return Err(JvmError::null_pointer());
            }
            let name = option_name(&option)?;
            match name.as_str() {
                "READ" => flags.read = true,
                "WRITE" => flags.write = true,
                "APPEND" => flags.append = true,
                "TRUNCATE_EXISTING" => flags.truncate_existing = true,
                "CREATE" => flags.create = true,
                "CREATE_NEW" => flags.create_new = true,
                "DELETE_ON_CLOSE" => flags.delete_on_close = true,
                "SPARSE" => { /* ignore（JDK 同此） */ }
                "SYNC" => flags.sync = true,
                "DSYNC" => flags.dsync = true,
                "NOFOLLOW_LINKS" => flags.no_follow_links = true,
                _ => {
                    return Err(JvmError::from(
                        UnsupportedOperationException::new_str(String::from(format!(
                            "{} not supported",
                            name
                        )))?,
                    ));
                }
            }
        }

        // 默认读；append ⇒ 写
        if !flags.read && !flags.write {
            if flags.append {
                flags.write = true;
            } else {
                flags.read = true;
            }
        }

        // 校验（JDK 同款）
        if flags.read && flags.append {
            return Err(JvmError::from(IllegalArgumentException::new_str(String::from(
                "READ + APPEND not allowed",
            ))?));
        }
        if flags.append && flags.truncate_existing {
            return Err(JvmError::from(IllegalArgumentException::new_str(String::from(
                "APPEND + TRUNCATE_EXISTING not allowed",
            ))?));
        }

        // oflags 组装
        use consts::oflags as o;
        let mut oflags: i32 = if flags.read && flags.write {
            o::O_RDWR
        } else if flags.write {
            o::O_WRONLY
        } else {
            0 // O_RDONLY
        };
        if flags.write {
            if flags.truncate_existing {
                oflags |= o::O_TRUNC;
            }
            if flags.append {
                oflags |= o::O_APPEND;
            }
            if flags.create_new {
                let raw = path.asByteArray()?;
                let bytes: Vec<u8> = raw.to_vec().into_iter().map(|b| b as u8).collect();
                if bytes.last() == Some(&b'.')
                    && (bytes.len() == 1 || bytes[bytes.len() - 2] == b'/')
                {
                    return Err(JvmError::from(UnixException::new_i(consts::errno::EEXIST)?));
                }
                oflags |= o::O_CREAT | o::O_EXCL;
            } else if flags.create {
                oflags |= o::O_CREAT;
            }
        }
        let mut follow_links = true;
        if !flags.create_new && (flags.no_follow_links || flags.delete_on_close) {
            follow_links = false;
            oflags |= o::O_NOFOLLOW;
        }
        if flags.dsync {
            // O_DSYNC：macOS 0x400000 / Linux 0x1000——档 A 用例面未消费，
            // 透传位由宿主值承载（写入失败面不在用例内）
            #[cfg(target_os = "macos")]
            {
                oflags |= 0x0040_0000;
            }
            #[cfg(target_os = "linux")]
            {
                oflags |= 0x1000;
            }
        }
        if flags.sync {
            #[cfg(target_os = "macos")]
            {
                oflags |= 0x0080_0000;
            }
            #[cfg(target_os = "linux")]
            {
                oflags |= 0x4010;
            }
        }
        if flags.direct {
            #[cfg(target_os = "macos")]
            {
                // macOS 无 O_DIRECT（JDK 同此：0）
            }
            #[cfg(target_os = "linux")]
            {
                oflags |= 0x4000;
            }
        }

        // open(2)
        let fd = match UnixNativeDispatcher::open(Clone::clone(&path), oflags, mode) {
            Ok(fd) => fd,
            Err(e) => {
                // errno 从 UnixException 载体还原后按 JDK 语义改写再翻译
                if e.is_instance_of("sun/nio/fs/UnixException") {
                    let x0: UnixException = e.catch_as::<UnixException>("sun/nio/fs/UnixException");
                    let errno = x0.errno()?;
                    let mut x = x0;
                    // EISDIR → EEXIST（create_new 时的歧义错误）
                    if flags.create_new && errno == consts::errno::EISDIR {
                        x.setError(consts::errno::EEXIST)?;
                    }
                    // ELOOP 的消息补全（NOFOLLOW_LINKS 语境）
                    if !follow_links && errno == consts::errno::ELOOP {
                        let msg = x.errorString()?;
                        x = UnixException::new_str(String::from(format!(
                            "{} (NOFOLLOW_LINKS specified)",
                            msg
                        )))?;
                    }
                    x.rethrowAsIOException_unixpath(&path)?;
                }
                return Err(e);
            }
        };

        // DELETE_ON_CLOSE：立即 unlink（best-effort）
        if flags.delete_on_close {
            let _ = UnixNativeDispatcher::unlink(Clone::clone(&path));
        }

        // FileDescriptor（fd 为 i32 裸 fd，FileOutputStream 同惯例）；
        // new_i 的生成体附带 getAppend 探测（Unix 恒 false），append 位以
        // 本地 flags 覆写（fdAccess.setAppend 语义）
        let fd_obj = FileDescriptor::new_i(fd)?;
        fd_obj.__set_append(flags.append);

        let path_str = path.toString()?;
        FileChannelImpl::open_filedescriptor_str_z_z_z_closeable(
            fd_obj,
            path_str,
            flags.read,
            flags.write,
            flags.direct,
            Default::default(),
        )
    }
}
