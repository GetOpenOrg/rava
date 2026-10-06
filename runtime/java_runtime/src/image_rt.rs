//! 构建期引导映像启动序列的运行期原语（计划 2026-10-05-boot-image-evaluator §5.5.2 D4 / D5）。
//!
//! 生成的根门面模块 `boot_image` 在 `main` 入口调用 `__boot_image_start`：登记映像区、写 VM 单元、
//! 以宿主值改写宿主相关内容、驻留字符串、链接类镜像、写静态字段初值、按构建期次序重放残差。
//! 本模块只给出与具体类无关的原语；名字带 `__` 语义的细节不出现在可读层。

use crate::error::Result;
use crate::java::lang::{Class, Object, String, Thread};

/// 类镜像（描述符：`L..;` / `[..` / 基本类型字母）
pub fn mirror(desc: &str) -> Object {
    Object::from(Class::__class_for_descriptor(desc))
}

/// 映像中的 VM 初始线程绑定为 OS 主线程的当前线程
pub fn bind_initial_thread(t: Object) {
    Thread::from(t).__vm_bind_initial();
}

/// 驻留映像字符串（构建期驻留表的内容：运行期驻留表以映像对象为规范实例）
pub fn intern(s: Object) {
    let _ = String::from(s).__interned();
}

/// 引导初始化档位（`VM.initLevel` 的执行流视图）：`None` = 引导完成
pub fn set_level(level: Option<i32>) {
    crate::exec_context::state().boot_level.set(level);
}

/// VM 原生单元的初值
pub fn vm_cell(name: &str, bits: i64) {
    cell(name).store(bits, std::sync::atomic::Ordering::Relaxed);
}

/// VM 原生单元的运行期地址（重定位槽）
pub fn vm_cell_addr(name: &str) -> i64 {
    cell(name).as_ptr() as i64
}

fn cell(name: &str) -> &'static std::sync::atomic::AtomicI64 {
    match name {
        "next_thread_id" => Thread::__next_thread_id_cell(),
        _ => panic!("stub: 引导映像 VM 单元 {name} 无运行期承载"),
    }
}

/// 实例字段偏移（重定位槽）：与 `Unsafe.objectFieldOffset` 同一登记表
pub fn field_offset(decl: &str, name: &str) -> i64 {
    crate::reflect_dispatch::instance_field_id(decl.replace('/', "."), name.to_string())
}

/// 启动序列的异常出口：引导期抛出的异常不可恢复（与 JVM 初始化失败同为致命）
pub fn run(f: impl FnOnce() -> Result<()>) {
    if let Err(e) = f() {
        e.report_uncaught();
    }
}
