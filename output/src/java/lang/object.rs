// 此文件由 java_rta 自动生成，仅供 build.rs 扫描。不参与 Rust 模块编译。

#[java_class(
    binary_name = "java/lang/Object",
    super_class = "",
    interfaces  = "",
    access      = "public",
    source      = "Object.java",
)]
struct _JavaClassMarker;

#[java_native(name = "getClass", descriptor = "()Ljava/lang/Class;", access = "public final native")]
fn _getClass() {}

#[java_native(name = "hashCode", descriptor = "()I", access = "public native")]
fn _hashCode() {}

#[java_native(name = "clone", descriptor = "()Ljava/lang/Object;", access = "protected native")]
fn _clone() {}

#[java_native(name = "notify", descriptor = "()V", access = "public final native")]
fn _notify() {}

#[java_native(name = "notifyAll", descriptor = "()V", access = "public final native")]
fn _notifyAll() {}

#[java_native(name = "wait0", descriptor = "(J)V", access = "private final native")]
fn _wait0() {}
