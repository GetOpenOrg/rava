#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/channels/FileChannel",
    super_class = "java/nio/channels/spi/AbstractInterruptibleChannel",
    interfaces  = "java/nio/channels/SeekableByteChannel,java/nio/channels/GatheringByteChannel,java/nio/channels/ScatteringByteChannel",
    access      = "public abstract",
    source      = "FileChannel.java",
))]
pub struct FileChannel;

impl FileChannel {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "protected"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/nio/channels/spi/AbstractInterruptibleChannel.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "open", descriptor = "(Ljava/nio/file/Path;Ljava/util/Set;[Ljava/nio/file/attribute/FileAttribute;)Ljava/nio/channels/FileChannel;", access = "public static"))]
    // java: open(Ljava/nio/file/Path;Ljava/util/Set;[Ljava/nio/file/attribute/FileAttribute;)Ljava/nio/channels/FileChannel;
    pub fn open__path_set_arr_fil(path: Object, options: Object, attrs: &[Object]) -> Result<Object> {
        let _t0 = path.getFileSystem()?;
        let _t1 = _t0.provider()?;
        let mut provider: Object = _t1;
        let _t2 = provider.newFileChannel(path, options, attrs)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "open", descriptor = "(Ljava/nio/file/Path;[Ljava/nio/file/OpenOption;)Ljava/nio/channels/FileChannel;", access = "public static"))]
    // java: open(Ljava/nio/file/Path;[Ljava/nio/file/OpenOption;)Ljava/nio/channels/FileChannel;
    pub fn open__path_arr_ope(path: Object, options: &[Object]) -> Result<Object> {
        let _t0: Object = Collections::emptySet()?;
        let mut set: Object = _t0;
        set = HashSet::<_>::new()?;
        let _t1: bool = Collections::addAll(set, &options)?;
        let _t2: Object = FileChannel::open(path, set, &FileChannel::NO_ATTRIBUTES())?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_native(name = "read", descriptor = "(Ljava/nio/ByteBuffer;)I", access = "public abstract"))]
    pub fn read__bytebu(&self, arg0: Object) -> Result<i32> {
        todo!("abstract java/nio/channels/FileChannel.read")
    }

    #[cfg_attr(any(), java_native(name = "read", descriptor = "([Ljava/nio/ByteBuffer;II)J", access = "public abstract"))]
    pub fn read__arr_byt_i_i(&self, arg0: Vec<Object>, arg1: i32, arg2: i32) -> Result<i64> {
        todo!("abstract java/nio/channels/FileChannel.read")
    }

    #[cfg_attr(any(), java_method(name = "read", descriptor = "([Ljava/nio/ByteBuffer;)J", access = "public final"))]
    // java: read([Ljava/nio/ByteBuffer;)J
    pub fn read__arr_byt(&self, dsts: Vec<Object>) -> Result<i64> {
        let this = self;
        let _t0 = this.read(dsts, 0i32, (dsts.len() as i32))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_native(name = "write", descriptor = "(Ljava/nio/ByteBuffer;)I", access = "public abstract"))]
    pub fn write__bytebu(&self, arg0: Object) -> Result<i32> {
        todo!("abstract java/nio/channels/FileChannel.write")
    }

    #[cfg_attr(any(), java_native(name = "write", descriptor = "([Ljava/nio/ByteBuffer;II)J", access = "public abstract"))]
    pub fn write__arr_byt_i_i(&self, arg0: Vec<Object>, arg1: i32, arg2: i32) -> Result<i64> {
        todo!("abstract java/nio/channels/FileChannel.write")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([Ljava/nio/ByteBuffer;)J", access = "public final"))]
    // java: write([Ljava/nio/ByteBuffer;)J
    pub fn write__arr_byt(&self, srcs: Vec<Object>) -> Result<i64> {
        let this = self;
        let _t0 = this.write(srcs, 0i32, (srcs.len() as i32))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_native(name = "position", descriptor = "()J", access = "public abstract"))]
    pub fn position(&self) -> Result<i64> {
        todo!("abstract java/nio/channels/FileChannel.position")
    }

    #[cfg_attr(any(), java_native(name = "position", descriptor = "(J)Ljava/nio/channels/FileChannel;", access = "public abstract"))]
    pub fn position__l(&self, arg0: i64) -> Result<Object> {
        todo!("abstract java/nio/channels/FileChannel.position")
    }

    #[cfg_attr(any(), java_native(name = "size", descriptor = "()J", access = "public abstract"))]
    pub fn size(&self) -> Result<i64> {
        todo!("abstract java/nio/channels/FileChannel.size")
    }

    #[cfg_attr(any(), java_native(name = "truncate", descriptor = "(J)Ljava/nio/channels/FileChannel;", access = "public abstract"))]
    pub fn truncate(&self, arg0: i64) -> Result<Object> {
        todo!("abstract java/nio/channels/FileChannel.truncate")
    }

    #[cfg_attr(any(), java_native(name = "force", descriptor = "(Z)V", access = "public abstract"))]
    pub fn force(&self, arg0: bool) -> Result<()> {
        todo!("abstract java/nio/channels/FileChannel.force")
    }

    #[cfg_attr(any(), java_native(name = "transferTo", descriptor = "(JJLjava/nio/channels/WritableByteChannel;)J", access = "public abstract"))]
    pub fn transferTo(&self, arg0: i64, arg1: i64, arg2: Object) -> Result<i64> {
        todo!("abstract java/nio/channels/FileChannel.transferTo")
    }

    #[cfg_attr(any(), java_native(name = "transferFrom", descriptor = "(Ljava/nio/channels/ReadableByteChannel;JJ)J", access = "public abstract"))]
    pub fn transferFrom(&self, arg0: Object, arg1: i64, arg2: i64) -> Result<i64> {
        todo!("abstract java/nio/channels/FileChannel.transferFrom")
    }

    #[cfg_attr(any(), java_native(name = "read", descriptor = "(Ljava/nio/ByteBuffer;J)I", access = "public abstract"))]
    pub fn read__bytebu_l(&self, arg0: Object, arg1: i64) -> Result<i32> {
        todo!("abstract java/nio/channels/FileChannel.read")
    }

    #[cfg_attr(any(), java_native(name = "write", descriptor = "(Ljava/nio/ByteBuffer;J)I", access = "public abstract"))]
    pub fn write__bytebu_l(&self, arg0: Object, arg1: i64) -> Result<i32> {
        todo!("abstract java/nio/channels/FileChannel.write")
    }

    #[cfg_attr(any(), java_native(name = "map", descriptor = "(Ljava/nio/channels/FileChannel$MapMode;JJ)Ljava/nio/MappedByteBuffer;", access = "public abstract"))]
    pub fn map__filech_l_l(&self, arg0: Object, arg1: i64, arg2: i64) -> Result<Object> {
        todo!("abstract java/nio/channels/FileChannel.map")
    }

    #[cfg_attr(any(), java_method(name = "map", descriptor = "(Ljava/nio/channels/FileChannel$MapMode;JJLjava/lang/foreign/Arena;)Ljava/lang/foreign/MemorySegment;", access = "public"))]
    // java: map(Ljava/nio/channels/FileChannel$MapMode;JJLjava/lang/foreign/Arena;)Ljava/lang/foreign/MemorySegment;
    pub fn map__filech_l_l_arena(&self, mode: Object, offset: i64, arg_2: i64, size: Object) -> Result<Object> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_native(name = "lock", descriptor = "(JJZ)Ljava/nio/channels/FileLock;", access = "public abstract"))]
    pub fn lock__l_l_z(&self, arg0: i64, arg1: i64, arg2: bool) -> Result<Object> {
        todo!("abstract java/nio/channels/FileChannel.lock")
    }

    #[cfg_attr(any(), java_method(name = "lock", descriptor = "()Ljava/nio/channels/FileLock;", access = "public final"))]
    // java: lock()Ljava/nio/channels/FileLock;
    pub fn lock(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.lock(0i64, 9223372036854775807i64, 0i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_native(name = "tryLock", descriptor = "(JJZ)Ljava/nio/channels/FileLock;", access = "public abstract"))]
    pub fn tryLock__l_l_z(&self, arg0: i64, arg1: i64, arg2: bool) -> Result<Object> {
        todo!("abstract java/nio/channels/FileChannel.tryLock")
    }

    #[cfg_attr(any(), java_method(name = "tryLock", descriptor = "()Ljava/nio/channels/FileLock;", access = "public final"))]
    // java: tryLock()Ljava/nio/channels/FileLock;
    pub fn tryLock(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.tryLock(0i64, 9223372036854775807i64, 0i32)?;
        Ok(_t0)
    }
}
