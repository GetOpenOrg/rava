//! `sun/nio/fs/BsdFileSystem` 的 ACC_NATIVE（类 1，libnio `BsdFileSystem.c`）。
//!
//! 本类只在 macOS 的 JDK 中存在，唯一 native 为 `directCopy0`：Files.copy 的内核内数据拷贝，
//! 走 fcopyfile(3) 的 COPYFILE_DATA；有取消标记时挂进度回调，回调见标记置位即令 fcopyfile 退出。

use crate::prelude::*;
use super::bsd_file_system::BsdFileSystem;
#[cfg(target_os = "macos")]
use super::unix_native_dispatcher_impl::{errno, unix_exception};

impl BsdFileSystem {
    /// native `directCopy0(int dst, int src, long cancelAddress)`：成功返回 0，失败抛 UnixException(errno)。
    #[jvm_native]
    pub fn directCopy0(dst: i32, src: i32, cancel: i64) -> Result<i32> {
        direct_copy(dst, src, cancel)
    }
}

/// JNI `fcopyfile_callback`：数据拷贝阶段出错、或进度回调时取消标记非 0，返回 COPYFILE_QUIT，否则继续
#[cfg(target_os = "macos")]
extern "C" fn copy_progress(
    what: libc::c_int,
    stage: libc::c_int,
    _state: libc::copyfile_state_t,
    _src: *const libc::c_char,
    _dst: *const libc::c_char,
    ctx: *mut libc::c_void,
) -> libc::c_int {
    // SAFETY: ctx 为 directCopy0 传入的取消标记地址（调用方持有的直接内存 int）
    if what == libc::COPYFILE_COPY_DATA
        && (stage == libc::COPYFILE_ERR
            || (stage == libc::COPYFILE_PROGRESS && unsafe { std::ptr::read_volatile(ctx as *const i32) } != 0))
    {
        return libc::COPYFILE_QUIT;
    }
    libc::COPYFILE_CONTINUE
}

#[cfg(target_os = "macos")]
fn direct_copy(dst: i32, src: i32, cancel: i64) -> Result<i32> {
    let state = if cancel != 0 {
        // SAFETY: copyfile_state_alloc 返回新状态；回调与上下文按 copyfile(3) 约定设置
        unsafe {
            let s = libc::copyfile_state_alloc();
            let cb: libc::copyfile_callback_t = Some(copy_progress);
            libc::copyfile_state_set(s, libc::COPYFILE_STATE_STATUS_CB as u32, cb.map_or(std::ptr::null(), |f| f as *const libc::c_void));
            libc::copyfile_state_set(s, libc::COPYFILE_STATE_STATUS_CTX as u32, cancel as *const libc::c_void);
            s
        }
    } else {
        std::ptr::null_mut()
    };
    // SAFETY: src / dst 为调用方持有的描述符；state 为空或上面分配的状态
    let rc = unsafe { libc::fcopyfile(src, dst, state, libc::COPYFILE_DATA) };
    let err = errno();
    if !state.is_null() {
        // SAFETY: state 由 copyfile_state_alloc 分配，仅此处释放
        unsafe { libc::copyfile_state_free(state) };
    }
    if rc < 0 {
        return Err(unix_exception(err));
    }
    Ok(0)
}

/// 非 macOS 平台不存在 BsdFileSystem；保持签名可编译
#[cfg(not(target_os = "macos"))]
fn direct_copy(_dst: i32, _src: i32, _cancel: i64) -> Result<i32> {
    Ok(-6)
}
