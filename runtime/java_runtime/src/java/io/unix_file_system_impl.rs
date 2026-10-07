use crate::prelude::*;
use super::unix_file_system::UnixFileSystem;
use super::file::File;
use super::file_system::FileSystem;

// UnixFileSystem 的 native 层：File 语义在 Unix 上的系统调用面。
// stat 族（getBooleanAttributes0/getLastModifiedTime0/getLength0）映射
// std::fs::metadata；命名空间操作（delete0/rename0/create*0）映射对应
// std::fs API；权限位（checkAccess0/setPermission0/setReadOnly0）经
// Unix PermissionsExt 模式位操作。checkAccess0 的 access(2) 语义无 std
// 等价（需真实 uid/gid 判定），按「任一权限类授予即通过」的宽容近似。
// getSpace0 无 statvfs 等价、getNameMax0 无 pathconf 等价——分别返回 0
// 与 255（APFS/ext4 等现代文件系统的通用 NAME_MAX），语料内无消费方。

/// File → 宿主路径（`File.path` Java String → UTF-8）。
fn file_path(f: &File) -> std::string::String {
    format!("{}", f.__get_path())
}

/// 路径不存在时 Java 侧约定的「null 数组」形态。
fn null_str_array() -> JArray<String> {
    JArray::default()
}

/// realpath(3)（`std::fs::canonicalize` 在 Unix 上即 realpath）。
fn realpath(path: &str) -> std::io::Result<std::string::String> {
    std::fs::canonicalize(path).map(|p| p.to_string_lossy().into_owned())
}

/// `JDK_Canonicalize(orig, out, PATH_MAX)` 的逐句移植。
fn jdk_canonicalize(orig: &str) -> std::io::Result<std::string::String> {
    let path_max = libc::PATH_MAX as usize;
    if orig.len() > path_max {
        return Err(std::io::Error::from_raw_os_error(libc::ENAMETOOLONG));
    }
    // 先对整条路径 realpath
    if let Ok(r) = realpath(orig) {
        return Ok(collapse(&r));
    }
    // 原路径某处不成立：从末尾逐段去名，直到某个子路径可解析或名字用尽
    let bytes = orig.as_bytes();
    let mut p = bytes.len();
    let mut resolved: Option<std::string::String> = None;
    while p > 0 {
        // 跳过最后一个名字：`while ((--p > path) && (*p != '/'));`
        loop {
            p -= 1;
            if p == 0 || bytes[p] == b'/' {
                break;
            }
        }
        if p == 0 {
            break;
        }
        match realpath(&orig[..p]) {
            Ok(r) => {
                resolved = Some(r);
                break;
            }
            // 不存在 / 类型不对 / 无权访问：再去一个名字；其他 I/O 错误直接失败
            Err(e) if matches!(e.raw_os_error(),
                Some(libc::ENOENT) | Some(libc::ENOTDIR) | Some(libc::EACCES)) => continue,
            Err(e) => return Err(e),
        }
    }
    match resolved {
        Some(mut r) => {
            // 把未解析的尾部接到已解析前缀后（避免重复斜杠）
            let mut tail = &orig[p..];
            if r.len() + tail.len() >= path_max {
                return Err(std::io::Error::from_raw_os_error(libc::ENAMETOOLONG));
            }
            if r.ends_with('/') && tail.starts_with('/') {
                tail = &tail[1..];
            }
            r.push_str(tail);
            Ok(collapse(&r))
        }
        // 一段也解析不了：返回原路径
        None => Ok(collapse(orig)),
    }
}

/// `splitNames` 的名字切分：每个名字从当前位置起（首字符不作分隔符），到下一个 `/` 为止。
fn split_names(s: &str) -> Vec<&str> {
    let b = s.as_bytes();
    let mut names = Vec::new();
    let mut p = 0;
    while p < b.len() {
        let start = p;
        p += 1;
        let mut end = b.len();
        while p < b.len() {
            if b[p] == b'/' {
                end = p;
                p += 1;
                break;
            }
            p += 1;
        }
        names.push(&s[start..end]);
    }
    names
}

/// `collapse`：语法上消解 `.` 与 `..`（不查询文件系统，只用于 realpath 之后的整理）。
/// `.` 总是去掉；`..` 与它之前最近的未去名字一并去掉，之前没有名字时保留。保留首个 `/`。
fn collapse(path: &str) -> std::string::String {
    let (root, names_str) = match path.strip_prefix('/') {
        Some(rest) => ("/", rest),
        None => ("", path),
    };
    let names = split_names(names_str);
    let has_dots = names.iter().any(|n| *n == "." || *n == "..");
    if !has_dots || names.len() < 2 {
        return path.to_owned();
    }
    let mut kept: Vec<Option<&str>> = names.iter().map(|n| Some(*n)).collect();
    for i in 0..kept.len() {
        match names[i] {
            "." => kept[i] = None,
            ".." => {
                if let Some(j) = (0..i).rev().find(|&j| kept[j].is_some()) {
                    kept[j] = None;
                    kept[i] = None;
                }
            }
            _ => {}
        }
    }
    let joined: Vec<&str> = kept.into_iter().flatten().collect();
    format!("{}{}", root, joined.join("/"))
}

impl UnixFileSystem {
    /// native initIDs：HotSpot 缓存 JNI 字段 ID；原生二进制无此需要。
    #[jvm_native]
    pub fn initIDs() -> Result<()> {
        Ok(())
    }

    /// native getBooleanAttributes0(File)：BA_EXISTS(1) | BA_REGULAR(2) |
    /// BA_DIRECTORY(4)；不存在返回 0。JDK 用 stat（跟随符号链接）。
    #[jvm_native]
    pub fn getBooleanAttributes0(&self, f: File) -> Result<i32> {
        // 嵌入资源（伪 java.home 下的 JDK 数据文件）视为存在的普通文件（FileInputStream.open0 同一判定）
        let path = file_path(&f);
        if path.starts_with(crate::jdk_resources::JAVA_RUNTIME_HOME) && crate::jdk_resources::lookup(&path).is_some() {
            return Ok(FileSystem::BA_EXISTS()? | FileSystem::BA_REGULAR()?);
        }
        let Ok(md) = std::fs::metadata(path) else { return Ok(0) };
        let mut attrs = FileSystem::BA_EXISTS()?;
        let ft = md.file_type();
        if ft.is_file() {
            attrs |= FileSystem::BA_REGULAR()?;
        }
        if ft.is_dir() {
            attrs |= FileSystem::BA_DIRECTORY()?;
        }
        Ok(attrs)
    }

    /// native checkAccess0(File, access)：ACCESS_EXECUTE(1)/WRITE(2)/READ(4) 与 access(2) 的
    /// X_OK/W_OK/R_OK 同值——按有效用户权限精确判定（root 对只读文件可写等 JDK 同语义）。
    #[jvm_native]
    pub fn checkAccess0(&self, f: File, access: i32) -> Result<bool> {
        Ok(crate::posix::access(std::path::Path::new(&file_path(&f)), access) == 0)
    }

    /// native getLastModifiedTime0(File)：epoch 毫秒；不存在返回 0。
    #[jvm_native]
    pub fn getLastModifiedTime0(&self, f: File) -> Result<i64> {
        let Ok(md) = std::fs::metadata(file_path(&f)) else { return Ok(0) };
        let millis = md.modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
        Ok(millis)
    }

    /// native getLength0(File)：字节长度；不存在返回 0。
    #[jvm_native]
    pub fn getLength0(&self, f: File) -> Result<i64> {
        let Ok(md) = std::fs::metadata(file_path(&f)) else { return Ok(0) };
        Ok(md.len() as i64)
    }

    /// native canonicalize0(String)：`JDK_Canonicalize`（libjava `canonicalize_md.c`）语义——
    /// 整条路径 realpath 失败时（如路径不存在）从末尾逐段剥离，取能 realpath 的最长前缀再接回
    /// 未解析的尾部，最后语法上消解 `.` / `..`；路径不必存在。失败抛 IOException，消息同
    /// `JNU_ThrowIOExceptionWithLastError(env, "Bad pathname")`（errno 文案优先）。
    #[jvm_native]
    pub fn canonicalize0(&self, path: String) -> Result<String> {
        match jdk_canonicalize(&format!("{}", path)) {
            Ok(p) => Ok(String::from(p.as_str())),
            Err(e) => {
                let msg = e.raw_os_error()
                    .and_then(crate::net_posix::strerror)
                    .unwrap_or_else(|| "Bad pathname".to_owned());
                Err(JvmError::from(super::IOException::new_str(String::from(msg.as_str()))?))
            }
        }
    }

    /// native list0(File)：目录项名字数组；失败（不存在/非目录/IO 错）返回 null。
    #[jvm_native]
    pub fn list0(&self, f: File) -> Result<JArray<String>> {
        let names: Vec<std::string::String> = match std::fs::read_dir(file_path(&f)) {
            Ok(entries) => entries
                .filter_map(|e| e.ok())
                .filter_map(|e| e.file_name().into_string().ok())
                .collect(),
            Err(_) => return Ok(null_str_array()),
        };
        if names.is_empty() {
            // read_dir 对不存在路径同样 Err → 上分支；空目录返回空数组（非 null）
            return Ok(JArray::new(0));
        }
        let arr = JArray::new(names.len() as i32);
        for (i, n) in names.iter().enumerate() {
            arr.set(i as i32, String::from(n.as_str()))?;
        }
        Ok(arr)
    }

    /// native delete0(File)：remove(3) 语义——文件或空目录，失败返回 false。
    #[jvm_native]
    pub fn delete0(&self, f: File) -> Result<bool> {
        let path = file_path(&f);
        let Ok(md) = std::fs::symlink_metadata(&path) else { return Ok(false) };
        let r = if md.is_dir() {
            std::fs::remove_dir(&path)
        } else {
            std::fs::remove_file(&path)
        };
        Ok(r.is_ok())
    }

    /// native rename0(File, File)：rename(2)（同文件系统内原子改名）。
    #[jvm_native]
    pub fn rename0(&self, f1: File, f2: File) -> Result<bool> {
        Ok(std::fs::rename(file_path(&f1), file_path(&f2)).is_ok())
    }

    /// native createDirectory0(File)：mkdir(2)（单级，父须存在）。
    #[jvm_native]
    pub fn createDirectory0(&self, f: File) -> Result<bool> {
        Ok(std::fs::create_dir(file_path(&f)).is_ok())
    }

    /// native createFileExclusively0(String)：O_CREAT|O_EXCL 原子建空文件。
    #[jvm_native]
    pub fn createFileExclusively0(&self, path: String) -> Result<bool> {
        use std::io::Write;
        let created = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(format!("{}", path));
        match created {
            Ok(mut f) => Ok(f.flush().is_ok()),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(false),
            Err(_) => Ok(false),
        }
    }

    /// native setPermission0(File, access, enable, owneronly)：按权限类
    /// （u/g/o）设 r/w/x 位。
    #[jvm_native]
    pub fn setPermission0(&self, f: File, access: i32, enable: bool, owneronly: bool) -> Result<bool> {
        use std::os::unix::fs::PermissionsExt;
        let path = file_path(&f);
        let Ok(md) = std::fs::metadata(&path) else { return Ok(false) };
        let mut perms = md.permissions();
        let mut mode = perms.mode();
        let (set_bits, clear_bits) = if access == FileSystem::ACCESS_READ()? {
            (0o444, 0o444)
        } else if access == FileSystem::ACCESS_WRITE()? {
            (0o222, 0o222)
        } else if access == FileSystem::ACCESS_EXECUTE()? {
            (0o111, 0o111)
        } else {
            return Ok(false);
        };
        let (set_bits, clear_bits) = if owneronly {
            (set_bits & 0o700, clear_bits & 0o700)
        } else {
            (set_bits, clear_bits)
        };
        if enable { mode |= set_bits; } else { mode &= !clear_bits; }
        perms.set_mode(mode);
        Ok(std::fs::set_permissions(&path, perms).is_ok())
    }

    /// native setReadOnly0(File)：清除全部写位。
    #[jvm_native]
    pub fn setReadOnly0(&self, f: File) -> Result<bool> {
        use std::os::unix::fs::PermissionsExt;
        let path = file_path(&f);
        let Ok(md) = std::fs::metadata(&path) else { return Ok(false) };
        let mut perms = md.permissions();
        perms.set_mode(perms.mode() & !0o222);
        Ok(std::fs::set_permissions(&path, perms).is_ok())
    }

    /// native setLastModifiedTime0(File, time)：epoch 毫秒 → utimensat。
    #[jvm_native]
    pub fn setLastModifiedTime0(&self, f: File, time: i64) -> Result<bool> {
        use std::fs::FileTimes;
        let Ok(file) = std::fs::File::options().read(true).open(file_path(&f)) else {
            return Ok(false);
        };
        let t = std::time::UNIX_EPOCH + std::time::Duration::from_millis(time.max(0) as u64);
        Ok(file.set_times(FileTimes::new().set_modified(t)).is_ok())
    }

    /// native getNameMax0(String)：pathconf(_PC_NAME_MAX)。
    #[jvm_native]
    pub fn getNameMax0(&self, path: String) -> Result<i64> {
        Ok(crate::posix::name_max(std::path::Path::new(&format!("{}", path))))
    }

    /// native getSpace0(File, t)：statvfs——SPACE_TOTAL(0) / SPACE_FREE(1) / SPACE_USABLE(2)。
    #[jvm_native]
    pub fn getSpace0(&self, f: File, t: i32) -> Result<i64> {
        Ok(crate::posix::space(std::path::Path::new(&file_path(&f)), t))
    }
}
