//! `sun/nio/fs/UnixPath` 手写伴生：POSIX 原生族档 A（切入序 3）。
//!
//! 用例面：构造与编码（jnuEncoding=UTF-8 直编码，档 A 形态）、系统调用字节取用
//! （getByteArrayForSysCalls）、Path 接口槽位（getFileSystem/toString/equals/
//! hashCode——`__impl_` 形态保声明进 vtable）、安全管理器检查（JDK21 恒 null，
//! no-op）。其余成员（子路径/relativize/toUri/迭代器等）保持 panic 存根。

use crate::prelude::*;
use super::unix_file_system::UnixFileSystem;
use super::unix_path::UnixPath;
use crate::java::nio::file::InvalidPathException;
use crate::java::nio::file::ProviderMismatchException;

/// jnu 编码字符串 → 字节（macOS/Linux 默认 UTF-8；POSIX locale 非 UTF-8 形态留档 B）。
fn jnu_encode(s: &String) -> Vec<u8> {
    format!("{}", s).into_bytes()
}

/// 字节 → jnu 编码字符串（toString 侧解码）。
fn jnu_decode(bytes: &[u8]) -> std::string::String {
    std::string::String::from_utf8_lossy(bytes).into_owned()
}

/// `JArray<i8>` ↔ `Vec<u8>`（byte[] 的 JVM 语义是有符号字节，路径字节按无符号语义比较）。
fn to_u8(a: &JArray<i8>) -> Vec<u8> {
    a.to_vec().into_iter().map(|b| b as u8).collect()
}

impl UnixPath {
    /// `<init>(UnixFileSystem, byte[])`：内部表示直存。
    #[jvm_boundary]
    pub fn new_unixfilesystem_arr_b(
        fs: UnixFileSystem,
        path: JArray<i8>,
    ) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_fs(fs);
        this.__set_path(path);
        Ok(this)
    }

    /// `<init>(UnixFileSystem, String)`：`this(fs, encode(fs, normalizeAndCheck(input)))`。
    #[jvm_boundary]
    pub fn new_unixfilesystem_str(
        fs: UnixFileSystem,
        input: String,
    ) -> Result<Self> {
        let normalized = Self::normalizeAndCheck(Clone::clone(&input))?;
        let bytes = Self::encode(Clone::clone(&fs), normalized)?;
        Self::new_unixfilesystem_arr_b(fs, bytes)
    }

    /// `normalizeAndCheck(String)`：压缩重复 '/'，拒绝 NUL 字符。
    /// char 索引以 Unicode 标量遍历（'/' 与 NUL 不出现于增补字符内部，
    /// 切分点语义与 UTF-16 索引一致）。
    #[jvm_boundary(upcalls = "java/nio/file/InvalidPathException.<init>:(Ljava/lang/String;Ljava/lang/String;)V")]
    pub fn normalizeAndCheck(input: String) -> Result<String> {
        let s: std::string::String = format!("{}", input);
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut prev = '\0';
        for i in 0..n {
            let c = chars[i];
            if c == '/' && prev == '/' {
                return Self::normalize_str_i_i(Clone::clone(&input), n as i32, (i as i32) - 1);
            }
            Self::checkNotNul(Clone::clone(&input), c as u16)?;
            prev = c;
        }
        if prev == '/' {
            return Self::normalize_str_i_i(Clone::clone(&input), n as i32, n as i32 - 1);
        }
        Ok(input)
    }

    /// `checkNotNul(String, char)`：NUL 字符拒绝（InvalidPathException）。
    #[jvm_boundary]
    pub fn checkNotNul(input: String, c: u16) -> Result<()> {
        if c == 0 {
            return Err(JvmError::from(InvalidPathException::new_str_str(
                input,
                String::from("Nul character not allowed"),
            )?));
        }
        Ok(())
    }

    /// `normalize(String, int len, int off)`：自 off 起压缩重复 '/'。
    #[jvm_boundary]
    pub fn normalize_str_i_i(input: String, _len: i32, off: i32) -> Result<String> {
        let s: std::string::String = format!("{}", input);
        if s.is_empty() {
            return Ok(input);
        }
        let off = off as usize;
        let mut out = std::string::String::with_capacity(s.len());
        if off > 0 {
            out.push_str(&s[..off]);
        }
        let mut prev = '\0';
        for c in s[off..].chars() {
            if c == '/' && prev == '/' {
                continue;
            }
            Self::checkNotNul(Clone::clone(&input), c as u16)?;
            out.push(c);
            prev = c;
        }
        Ok(String::from(out.as_str()))
    }

    /// `encode(UnixFileSystem, String)`：jnu 编码（UTF-8 直编码；平台
    /// normalizeNativePath 的 macOS NFD 规范化档 A 以恒等承载）。
    #[jvm_boundary]
    pub fn encode(_fs: UnixFileSystem, input: String) -> Result<JArray<i8>> {
        Ok(JArray::from(
            jnu_encode(&input).into_iter().map(|b| b as i8).collect::<Vec<i8>>(),
        ))
    }

    /// `asByteArray()`：内部字节表示。
    #[jvm_boundary]
    pub fn asByteArray(&self) -> Result<JArray<i8>> {
        Ok(Clone::clone(&self.__get_path()))
    }

    /// `getByteArrayForSysCalls()`：系统调用路径字节——需要时对默认目录解析；
    /// 空路径以 "." 访问当前目录。
    #[jvm_boundary]
    pub fn getByteArrayForSysCalls(&self) -> Result<JArray<i8>> {
        let fs = self.getFileSystem()?;
        if fs.needToResolveAgainstDefaultDirectory()? {
            let base = fs.defaultDirectory()?;
            Ok(Self::resolve_arr_b_arr_b(base, Clone::clone(&self.__get_path()))?)
        } else if self.__get_path().len()? != 0 {
            Ok(Clone::clone(&self.__get_path()))
        } else {
            Ok(JArray::from(vec![b'.' as i8]))
        }
    }

    /// `getPathForExceptionMessage()`：异常消息用路径。
    #[jvm_boundary]
    pub fn getPathForExceptionMessage(&self) -> Result<String> {
        self.toString()
    }

    /// `getPathForPermissionCheck()`：权限检查用路径（安全管理器恒 null 的
    /// JDK21 下不被触达，语义完整保留）。
    #[jvm_boundary]
    pub fn getPathForPermissionCheck(&self) -> Result<String> {
        if self.getFileSystem()?.needToResolveAgainstDefaultDirectory()? {
            let bytes = self.getByteArrayForSysCalls()?;
            Ok(String::from(jnu_decode(&to_u8(&bytes)).as_str()))
        } else {
            self.toString()
        }
    }

    /// `toUnixPath(Path)`：null → NPE；非 UnixPath → ProviderMismatchException。
    #[jvm_boundary(upcalls = "java/nio/file/ProviderMismatchException.<init>:(Ljava/lang/String;)V")]
    pub fn toUnixPath(obj: Object) -> Result<UnixPath> {
        if _is_jnull(&obj) {
            return Err(JvmError::null_pointer());
        }
        match obj.try_cast::<UnixPath>("sun/nio/fs/UnixPath") {
            Ok(p) => Ok(p),
            Err(_) => Err(JvmError::from(ProviderMismatchException::new_str(
                String::from(format!("{}", obj)),
            )?)),
        }
    }

    /// `checkRead()`：安全管理器检查（JDK21 恒 null → no-op）。
    #[jvm_boundary]
    pub fn checkRead(&self) -> Result<()> {
        Ok(())
    }

    /// `checkWrite()`：同上 no-op。
    #[jvm_boundary]
    pub fn checkWrite(&self) -> Result<()> {
        Ok(())
    }

    /// `checkDelete()`：同上 no-op。
    #[jvm_boundary]
    pub fn checkDelete(&self) -> Result<()> {
        Ok(())
    }

    /// `isEmpty()`：内部表示是否为空。
    #[jvm_boundary]
    pub fn isEmpty(&self) -> Result<bool> {
        Ok(self.__get_path().len()? == 0)
    }

    /// `emptyPath()`：空路径（fs 上的 new UnixPath(fs, new byte[0])）。
    #[jvm_boundary]
    pub fn emptyPath(&self) -> Result<UnixPath> {
        Self::new_unixfilesystem_arr_b(
            self.getFileSystem()?,
            JArray::from(Vec::<i8>::new()),
        )
    }

    /// `resolve(byte[], byte[])`：base/child 连接（child 空 → base；child 绝对 →
    /// child；base 空 → child；base 为 "/" → "/child"；否则 "base/child"）。
    #[jvm_boundary]
    pub fn resolve_arr_b_arr_b(base: JArray<i8>, child: JArray<i8>) -> Result<JArray<i8>> {
        let b = to_u8(&base);
        let c = to_u8(&child);
        if c.is_empty() {
            return Ok(base);
        }
        if b.is_empty() || c[0] == b'/' {
            return Ok(child);
        }
        let mut result = Vec::with_capacity(b.len() + 1 + c.len());
        if b.len() == 1 && b[0] == b'/' {
            result.push(b'/');
        } else {
            result.extend_from_slice(&b);
            result.push(b'/');
        }
        result.extend_from_slice(&c);
        Ok(JArray::from(result.into_iter().map(|x| x as i8).collect::<Vec<i8>>()))
    }

    // ── Path 接口槽位（生成侧 vtable 分派目标，`__impl_` 形态）────────────

    /// `getFileSystem()`：所属 UnixFileSystem。
    #[jvm_boundary]
    pub fn __impl_getFileSystem(&self) -> Result<UnixFileSystem> {
        Ok(self.__get_fs())
    }

    /// `toString()`：jnu 解码（normalizeJavaPath 恒等承载），惰性缓存。
    #[jvm_boundary]
    pub fn __impl_toString(&self) -> Result<String> {
        let cached = self.__get_stringValue();
        if !_is_jnull(&cached) {
            return Ok(cached);
        }
        let s = String::from(jnu_decode(&to_u8(&self.__get_path())).as_str());
        self.__set_stringValue(Clone::clone(&s));
        Ok(s)
    }

    /// `equals(Object)`：ob instanceof UnixPath 时按字节序比较（compareTo == 0）。
    #[jvm_boundary]
    pub fn __impl_equals(&self, ob: Object) -> Result<bool> {
        if _is_jnull(&ob) {
            return Ok(false);
        }
        let Ok(other) = ob.try_cast::<UnixPath>("sun/nio/fs/UnixPath") else {
            return Ok(false);
        };
        Ok(to_u8(&self.__get_path()) == to_u8(&other.__get_path()))
    }

    /// `hashCode()`：字节序 31 进制散列（h==0 惰性计算，0 长度路径恒 0）。
    #[jvm_boundary]
    pub fn __impl_hashCode(&self) -> Result<i32> {
        let mut h = self.__get_hash();
        if h == 0 {
            for b in to_u8(&self.__get_path()) {
                h = h.wrapping_mul(31).wrapping_add(b as i32);
            }
            self.__set_hash(h);
        }
        Ok(h)
    }
}
