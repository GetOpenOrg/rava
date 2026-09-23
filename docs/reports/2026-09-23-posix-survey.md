# sun/nio/fs POSIX 原生族手写前置盘点（FilesApi 解锁方案）

> 2026-09-23，纯调查代理交付（主会话入档）。手段：本机 JDK21（21.0.11 macOS）javap + src.zip；
> Linux 变体从 openjdk/jdk21u 原始源核对；--no-run transpile TestFilesApi 1 次（54s，1498 类）。

## 1. native 方法全集（JDK21 java.base）

### UnixNativeDispatcher（跨平台共享核心）— 49 个 native（javap 确认）

| 语义组 | native 方法 | 数量 |
|---|---|---|
| 进程/fd 基础 | getcwd, dup, init(capabilities 位图) | 3 |
| open/close + FILE* 配套 | open0, openat0, close0, rewind, getlinelen | 5 |
| 目录项操作 | link0, unlink0, unlinkat0, mknod0, rename0, renameat0, mkdir0, rmdir0, readlink0, realpath0, symlink0 | 11 |
| stat 族 | stat0, lstat0, fstat0, fstatat0 | 4 |
| 属主/权限/时间 | chown0, lchown0, fchown0, chmod0, fchmod0, utimes0, futimes0, futimens0, lutimes0 | 9 |
| 目录流 | opendir0, fdopendir, closedir, readdir0 | 4 |
| raw I/O（xattr 缓冲用） | read0, write0 | 2 |
| 权限探测 | access0 | 1 |
| 用户/组数据库 | getpwuid, getgrgid, getpwnam0, getgrnam0 | 4 |
| 文件系统信息 | statvfs0 | 1 |
| 错误消息 | strerror | 1 |
| 扩展属性 | fgetxattr0, fsetxattr0, fremovexattr0, flistxattr | 4 |

每个 native 外层都有非 native 包装（open(UnixPath,int,int) 等，做 NativeBuffer 拷贝 + Blocker begin/end）。

### 平台变体

| 类 | native 数 | 方法 |
|---|---|---|
| LinuxNativeDispatcher（服务器） | 6 | setmntent0, getmntent0, endmntent, posix_fadvise, directCopy0, init |
| BsdNativeDispatcher（本地 macOS） | 8 | getfsstat, fsstatEntry, endfsstat, getmntonname0, clonefile0, setattrlist0, fsetattrlist0, initIDs |
| MacOSXNativeDispatcher | 1 | normalizepath（NFD 规范化） |
| UnixFileSystem 自带 | 1 | bufferedCopy0（Files.copy 驱动） |

**全平台合计 = 65 个 native**。另：UnixConstants 的 60 个 static final 常量（O_CREAT 等）在 JDK 构建时已烧进 classfile（macOS: O_CREAT=0x200；Linux: 0x40），无 native，转译自动按目标平台 JDK 取对值，不需手写。

## 2. 消费面映射（源码静态核对）

| 消费类 | 调用的 UnixNativeDispatcher 包装 |
|---|---|
| UnixPath | open, opendir, realpath |
| UnixFileSystem | access, chmod, chown, close, fchmod, fchown, futimes, **getcwd**, lchown, mkdir, mknod, open, opendir, readlink, rename, rmdir, symlink, unlink, utimes（大多是 Files.* 静态委托的实现体） |
| UnixFileSystemProvider | access, close, dup, fdopendir, link, mkdir, open, opendir, readlink, rmdir, symlink, unlink |
| UnixChannelFactory | **open**, openat, unlink, unlinkat |
| UnixFileAttributes | **stat**, **stat2**, lstat, fstat, fstatat, birthtimeSupported |
| UnixFileAttributeViews | chmod, chown, close, fchmod, futimens, futimes, lchown, lutimes, utimes |
| UnixDirectoryStream | close, closedir, readdir |
| UnixSecureDirectoryStream | close, dup, fchmod, fchown, fdopendir, futimes, open, openat, renameat, unlinkat |
| UnixUserPrincipals | getgrgid, getgrnam, getpwnam, getpwuid |
| UnixUserDefinedFileAttributeView | close, fgetxattr, flistxattr, fremovexattr, fsetxattr, read, write |
| UnixFileStore | close, fgetxattr, xattrSupported |
| UnixFileStoreAttributes | statvfs |
| UnixException | strerror |
| BsdFileSystem / BsdFileStore / BsdFileAttributeViews | chown/unlink；realpath；lutimes/futimens |

注意：JDK21 无 UnixCopyFile（copy 由 UnixFileSystem.bufferedCopy/directCopy + Files.copy 驱动）。

## 3. TestFilesApi 用例面实际触达子集

用例（Path.of / writeString / readString / readAllLines / mismatch / exists / deleteIfExists）。实测 BFS 闭包中 sun/nio/fs 只进了 7 个类，全 stub：AbstractFileSystemProvider、BsdFileSystemProvider、DefaultFileSystemProvider、MacOSXFileSystemProvider、UnixFileSystem、UnixFileSystemProvider、UnixPath（UnixPath 53 方法 52 stub、UnixFileSystemProvider 42/40、UnixFileSystem 39/41）。UnixChannelFactory / UnixNativeDispatcher / UnixFileAttributes / UnixConstants / UnixException 尚不在闭包中——stub 修复后 BFS 前沿推进会拉入。

Files.* 层的 writeString/readString/readAllLines/mismatch/newByteChannel/newInputStream/newOutputStream 均已生成实现体（只有 exists→checkAccess→readAttributesIfExists、deleteIfExists→implDelete 落到 sun/nio/fs stub）。**用例面真正需要的 sun/nio/fs native 子集只有 4 个**：

- open0（writeString/readString/mismatch 的 newByteChannel → UnixChannelFactory）
- stat0（exists → FileSystemProvider.exists 默认法 → checkAccess(0 modes)→stat）
- unlink0（deleteIfExists → implDelete，靠 errno==ENOENT 判定 false）
- strerror（UnixException 消息；语义可用 io::Error::from_raw_os_error 替代）
- 加 init()（capabilities 位，返回常量即可）

**先行依赖**：UnixFileSystemProvider 构造调 StaticProperty.userDir()，而 static_property_impl.rs 目前没有 USER_DIR（只有 locale 族）。

## 4. Rust 等价物映射（关键架构发现：runtime 目前零 libc 依赖，只有 parking_lot + 宏 crate）

| 类别 | native → Rust |
|---|---|
| std 直接承载 | getcwd→env::current_dir；link0→fs::hard_link；unlink0→fs::remove_file；rename0→fs::rename；mkdir0→DirBuilderExt::mode；rmdir0→fs::remove_dir；symlink0→os::unix::fs::symlink；readlink0→fs::read_link；realpath0→fs::canonicalize；stat0/lstat0→metadata/symlink_metadata+MetadataExt；chmod0→set_permissions+PermissionsExt；chown0/lchown0(chown)→os::unix::fs::chown；opendir/readdir/closedir→fs::read_dir；strerror→io::Error::from_raw_os_error().to_string()；dup→File::try_clone；utimes0/futimes0/futimens0→File::set_times/set_modified/set_accessed(1.75+) |
| 需封装 | **open0**→OpenOptions+OpenOptionsExt::custom_flags(raw O_flags)+mode()（fd 以 i32 进 FileDescriptor，沿用 FileOutputStream 惯例）；**fstat0**→ManuallyDrop<File::from_raw_fd(fd)>.metadata()；access0→无 std 等价（需 euid 自查或 libc） |
| 无 std 等价（档 B 需 libc 或手写 syscall） | openat0/unlinkat0/renameat0/fstatat0/fdopendir（SecureDirectoryStream 族）、mknod0、fchmod0/fchown0、lutimes0、xattr 四件套、statvfs0、getpwuid/getgrgid/getpwnam0/getgrnam0、posix_fadvise、directCopy0(copy_file_range)、setmntent/getmntent/endmntent(或解析 /proc/self/mounts)；macOS 专属 getfsstat 族/clonefile0/setattrlist0/getmntonname0/normalizepath |

**平台差异点**：① 单例类型不同（Linux: LinuxFileSystemProvider 直接继承 UnixFileSystemProvider；macOS: MacOSX→Bsd→Unix 三层）——生成代码类型名随平台 JDK classfile 变化，impl 文件须按平台分；② birthtime（macOS st_birthtime 支持 / Linux stat 无 → creationTime 回落 ctime）；③ xattr/statvfs vs getfsstat；④ UnixPath 的 macOS NFD normalizepath；⑤ UnixConstants 值差异由平台 JDK classfile 自动解决。

## 5. JVM 抽象层最小手写形态 + 惯例对照

链路：FileSystems$DefaultFileSystemHolder.clinit → DefaultFileSystemProvider.theFileSystem() → INSTANCE(静态单例，clinit 时 new 平台 provider) → UnixFileSystemProvider.<init>(theFileSystem=newFileSystem(StaticProperty.userDir())) → UnixFileSystem(defaultDirectory=Util.normalize(dir))。DefaultFileSystemProvider 本体极小（INSTANCE 字段 + instance() + theFileSystem() 两行委托），手写形态 = #[jvm_boundary] 静态单例方法（参照 static_property_impl.rs 惯例），JVM 是 eager 构造（ctor 里建 theFileSystem）。

**与既有手写惯例对齐点**（runtime/java_runtime/src/ 下 67 个 *_impl.rs）：
- co-located *_impl.rs + codegen 自动 wiring mod xxx_impl;；#[jvm_native]（native）/#[jvm_boundary]（普通方法）覆盖生成的 panic stub——build.rs 维护 native_status.toml 的 implemented/needed 状态
- FileDescriptor/FileOutputStream 先例（fd 为 i32、std::io 承载、upcalls 属性声明异常构造）——fd→FileChannelImpl 可沿用 fd-int 直通，但真实 std::fs::File 需句柄表或 ManuallyDrop 管理
- Unsafe 常量自洽原则 → capabilities 位按宿主真实能力返回；initIDs 类无语义 native 一行 Ok(())；isBigEndian 式 cfg(target_os)（birthtime/xattr supported 同法）
- 「按调用链按需实现，其余保持 panic 存根」——档 A 只实现用例面

## 6. 工作量两档估算与切入序

**档 A（TestFilesApi 解锁，双平台）**：sun/nio/fs 侧约 10 个 *_impl.rs：
1. default_file_system_provider_impl（单例，~40 行×2 平台）；2. unix_file_system_provider_impl（init/newFileSystem/newByteChannel/readAttributesIfExists/implDelete/checkAccess，~300）；3. 平台 provider 薄层（~30/平台）；4. unix_file_system_impl（getPath/defaultDirectory，~150）；5. unix_path_impl（ctor/toString/equals/hashCode/checkRead/checkWrite/getByteArrayForSysCalls，~300）；6. unix_channel_factory_impl（newFileChannel+Flags 解码，~250）；7. unix_native_dispatcher_impl（open0/close0/stat0/unlink0/strerror/init，~200）；8. unix_file_attributes_impl（16 个 st_* 字段填充 + BasicFileAttributes 视图，~250）；9. unix_exception_impl（errno+rethrow 族，~120）；10. static_property_impl 追加 USER_DIR（~10）。
**小计 ≈ 1,600–1,900 行**。另有**邻接依赖（sun/nio/ch 族，数据真正流动必需）**：FileDispatcherImpl natives（read0/write0/pread0/close0/size/force0，~250 行）+ FileChannelImpl 读写/size/close（~350）+ FileDescriptor fd 登记扩展（~60）≈ **700–800 行**。**档 A 总计 ≈ 2,300–2,700 行 / 约 13 个文件**。

**档 B（POSIX 原生族全集）**：65 native 全实现 + 消费者：UnixDirectoryStream(~200)、UnixSecureDirectoryStream(~250)、UnixFileAttributeViews 四视图(~400)、UnixFileStore+Linux/Bsd 变体(~350)、UnixUserPrincipals(~150)、UnixUserDefinedFileAttributeView(~250)、copy 族 bufferedCopy0/directCopy0/posix_fadvise(~150)、mknod/symlink API(~100)；需引入 **libc 依赖**（openat/xattr/statvfs/passwd 等 std 全无）。不含 watch service（inotify/kqueue，独立大坑 ~600+，建议后置）。**档 B ≈ 档 A + 2,500–3,500 行 ≈ 5,000–6,000 行**。

**建议切入序**（每步后重跑 transpile，BFS 前沿推进刷新 stub 清单）：
1. StaticProperty.USER_DIR（10 行先行）→ 2. DefaultFileSystemProvider 单例 + 平台 provider 薄层（打通 Path.of）→ 3. UnixFileSystem.getPath + UnixPath 基础方法 → 4. UnixException + UnixFileAttributes + stat0（打通 Files.exists）→ 5. UnixChannelFactory + open0/close0/unlink0（打通 newByteChannel/deleteIfExists）→ 6. （邻接）FileChannelImpl + FileDispatcherImpl（writeString/readString 数据落地）→ 7. e2e 全绿。

**关键文件路径**：测试 tests/e2e/53_io_api/TestFilesApi.java；手写真源 runtime/java_runtime/src/（jdk/internal/misc/unsafe__impl.rs、java/io/file_output_stream_impl.rs、jdk/internal/util/static_property_impl.rs 为惯例参照）；scratch build/test_files_api/java_runtime/src/sun/nio/fs/；JDK 提取源 /tmp/jdksrc21/java.base/sun/nio/fs/、/tmp/jdkcmp/j21/java.base/sun/nio/fs/。
