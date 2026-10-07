use crate::prelude::*;
use super::native_image_buffer::NativeImageBuffer;
use crate::java::lang::String;
use crate::java::nio::{ByteBuffer, DirectByteBuffer};

// jdk.internal.jimage.NativeImageBuffer 伴生（手写边界 ①：ACC_NATIVE）。
//
// HotSpot 的 `JVM_NativeImageBuffer_getNativeMap`（libjimage）在 VM 启动时已映射运行时映像
// `${java.home}/lib/modules`，按路径返回该映射的 DirectByteBuffer（JNI `NewDirectByteBuffer`，即
// `DirectByteBuffer(long addr, long cap)`）。原生二进制的「运行时映像」是构建期写入用户侧元数据的本程序
// jimage（`meta::module_image`，闭包读取的模块资源，计划 boot-image §5.7）：路径等于嵌入树的
// `lib/modules` 时返回它的直接缓冲区，其余路径 null（与 HotSpot 对未打开映像的回答一致）。
// 其后 `BasicImageReader` / `ImageReader` / `SystemModuleReader` / jrt 协议全部按字节码执行。
impl NativeImageBuffer {
    /// native `getNativeMap(String)`：名为 `${java.home}/lib/modules` → 本程序 jimage 的直接缓冲区；否则 null
    #[jvm_native]
    pub fn getNativeMap(imagePath: String) -> Result<ByteBuffer> {
        if imagePath.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let image = crate::meta::module_image();
        let path = format!("{}", imagePath);
        if image.is_empty() || path != format!("{}/lib/modules", crate::jdk_resources::JAVA_RUNTIME_HOME) {
            return Ok(ByteBuffer::default());
        }
        // 映像在只读数据段、进程内常驻：地址与长度即 JNI NewDirectByteBuffer 的实参；读侧只经只读视图访问
        Ok(DirectByteBuffer::new_l_l(image.as_ptr() as i64, image.len() as i64)?.into())
    }
}
