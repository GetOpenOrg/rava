//! 构建期引导映像的运行期原语（计划 2026-10-05-boot-image-evaluator §5.5.2 D4 / D5、§5.10 零拷贝）。
//!
//! 映像是常量：对象、静态字段初值、接口 / 数组视图、类镜像都在生成的根门面模块 `boot_image` 的静态区里。
//! 映像表（类镜像、驻留串、构建期初始化类、VM 模块表）是链接期符号，运行期查询直接查表，启动时不登记。本模块只给出与具体类无关的原语。

use crate::error::Result;
use crate::java::lang::{Object, ObjectVTable, String, Thread};
use crate::obj_ref::__Obj;

/// 映像中的 VM 模块（构建期 `defineModule0` 的登记；`loader` 为定义加载器，引导加载器为 None）
pub struct ImageModule {
    pub module: &'static dyn ObjectVTable,
    pub loader: Option<&'static dyn ObjectVTable>,
    pub open: bool,
    pub location: Option<&'static str>,
    pub packages: &'static [&'static str],
}

/// 映像表（生成的常量，链接期符号 `__rava_image_tables`）。大表按块给出（每块一个静态，初值规模有界），块间与块内整体有序
pub struct ImageTables {
    /// 类镜像：(镜像键, 镜像)，按键（字节序）升序。键为 binary name（`/` 分隔）/ 数组描述符 / 基本类型名
    /// （`int`、`void` 等 Java 关键字，不与类名冲突）
    pub mirrors: &'static [&'static [(&'static str, &'static dyn ObjectVTable)]],
    /// 构建期驻留表的规范实例，按 UTF-16 码元序列升序
    pub strings: &'static [&'static [&'static dyn ObjectVTable]],
    /// 构建期完成初始化的类（binary name 升序）
    pub build_time: &'static [&'static str],
    /// VM 模块表初值（defineModule0 次序）
    pub modules: &'static [ImageModule],
}

unsafe extern "Rust" {
    /// 映像表：生成的根门面 `boot_image` 以此符号定义（`IMAGE_TABLES`），链接期解析——不经启动期登记，
    /// 任何查询（包括启动序列之前）都看到同一张表
    #[link_name = "__rava_image_tables"]
    static IMAGE_TABLES: ImageTables;
}

#[inline]
fn tables() -> &'static ImageTables {
    // SAFETY: 符号由生成的根门面以不可变静态定义，类型同本声明
    unsafe { &IMAGE_TABLES }
}

/// 映像对象的句柄
#[inline]
pub fn object(v: &'static dyn ObjectVTable) -> Object {
    Object(__Obj::image(v))
}

/// 分块有序表的查找：`cmp` 给出元素相对目标的次序
fn chunk_search<E>(chunks: &'static [&'static [E]], cmp: impl Fn(&E) -> std::cmp::Ordering) -> Option<&'static E> {
    let k = chunks.partition_point(|c| c.last().is_some_and(|e| cmp(e) == std::cmp::Ordering::Less));
    let c = chunks.get(k)?;
    c.binary_search_by(|e| cmp(e)).ok().map(|i| &c[i])
}

/// 映像中的类镜像（键同 [`ImageTables::mirrors`]）
pub fn image_mirror(key: &str) -> Option<Object> {
    chunk_search(tables().mirrors, |(k, _)| (*k).cmp(key)).map(|e| object(e.1))
}

/// 映像驻留表中内容为 `units` 的规范实例；`cmp` 比较映像串的 UTF-16 码元序列与 `units`（不分配）
pub fn image_string(units: &[u16], cmp: impl Fn(&String, &[u16]) -> std::cmp::Ordering) -> Option<String> {
    chunk_search(tables().strings, |v| cmp(&String::from(object(*v)), units)).map(|v| String::from(object(*v)))
}

/// 类是否在构建期完成初始化（binary name，`/` 分隔）
pub fn build_time(class: &str) -> bool {
    tables().build_time.binary_search(&class).is_ok()
}

/// 映像中的 VM 模块（VM 模块表首次访问时作初值）
pub fn modules() -> &'static [ImageModule] {
    tables().modules
}

/// 映像中的 VM 初始线程绑定为 OS 主线程的当前线程
pub fn bind_initial_thread(t: Object) {
    Thread::from(t).__vm_bind_initial();
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
