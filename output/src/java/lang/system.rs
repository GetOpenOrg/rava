// 此文件由 java_rta 自动生成，仅供 build.rs 扫描。不参与 Rust 模块编译。

#[java_class(
    binary_name = "java/lang/System",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "System.java",
)]
struct _JavaClassMarker;

#[java_native(name = "registerNatives", descriptor = "()V", access = "private static native")]
fn _registerNatives() {}

#[java_native(name = "setIn0", descriptor = "(Ljava/io/InputStream;)V", access = "private static native")]
fn _setIn0() {}

#[java_native(name = "setOut0", descriptor = "(Ljava/io/PrintStream;)V", access = "private static native")]
fn _setOut0() {}

#[java_native(name = "setErr0", descriptor = "(Ljava/io/PrintStream;)V", access = "private static native")]
fn _setErr0() {}

#[java_native(name = "currentTimeMillis", descriptor = "()J", access = "public static native")]
fn _currentTimeMillis() {}

#[java_native(name = "nanoTime", descriptor = "()J", access = "public static native")]
fn _nanoTime() {}

#[java_native(name = "arraycopy", descriptor = "(Ljava/lang/Object;ILjava/lang/Object;II)V", access = "public static native")]
fn _arraycopy() {}

#[java_native(name = "identityHashCode", descriptor = "(Ljava/lang/Object;)I", access = "public static native")]
fn _identityHashCode() {}

#[java_native(name = "mapLibraryName", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public static native")]
fn _mapLibraryName() {}
