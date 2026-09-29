//! `java/lang/ProcessHandleImpl` 的 native 方法（与生成的 process_handle_impl.rs 共置）。
//!
//! 对标 HotSpot ProcessHandleImpl_unix.c / ProcessHandleImpl_linux.c：pid 查询、存活探测
//! （kill(pid, 0)）、父进程（/proc/<pid>/stat 第 4 列）、信号终止。子进程等待与进程枚举
//! 依赖进程派生（ProcessImpl.forkAndExec），随其一并实现，当前保持存根。

use crate::prelude::*;
use super::process_handle_impl::ProcessHandleImpl;

/// /proc/<pid>/stat 的父进程号（第 4 字段；comm 字段可含空格，按最后一个 ')' 之后切分）。
fn proc_parent(pid: i64) -> Option<i64> {
    let stat = std::fs::read_to_string(format!("/proc/{}/stat", pid)).ok()?;
    let rest = &stat[stat.rfind(')')? + 1..];
    rest.split_whitespace().nth(1)?.parse().ok()
}

impl ProcessHandleImpl {
    /// native `initNative()`：缓存时钟频率 / 启动时间基准等 JNI 状态——无需初始化。
    #[jvm_native]
    pub fn initNative() -> Result<()> {
        Ok(())
    }

    /// native `getCurrentPid0()`：当前进程号（getpid）。
    #[jvm_native]
    pub fn getCurrentPid0() -> Result<i64> {
        Ok(std::process::id() as i64)
    }

    /// native `isAlive0(long pid)`：进程启动时间（毫秒）；不可得返回 0，进程不存在返回 -1
    /// （JDK 以启动时间区分 pid 复用；此处不采集启动时间，恒以 0 表示「存活、时间未知」）。
    #[jvm_native]
    pub fn isAlive0(pid: i64) -> Result<i64> {
        // SAFETY: kill(pid, 0) 只做存在性 / 权限探测，不发送信号
        let r = unsafe { libc::kill(pid as libc::pid_t, 0) };
        if r == 0 || std::io::Error::last_os_error().raw_os_error() == Some(libc::EPERM) {
            Ok(0)
        } else {
            Ok(-1)
        }
    }

    /// native `parent0(long pid, long startTime)`：父进程号；不可得返回 -1。
    #[jvm_native]
    pub fn parent0(pid: i64, _start_time: i64) -> Result<i64> {
        if pid == std::process::id() as i64 {
            // SAFETY: getppid 无副作用
            return Ok(unsafe { libc::getppid() } as i64);
        }
        Ok(proc_parent(pid).unwrap_or(-1))
    }

    /// native `destroy0(long pid, long startTime, boolean forcibly)`：SIGKILL / SIGTERM；
    /// 发送成功返回 true。
    #[jvm_native]
    pub fn destroy0(pid: i64, _start_time: i64, forcibly: bool) -> Result<bool> {
        let sig = if forcibly { libc::SIGKILL } else { libc::SIGTERM };
        // SAFETY: 向指定进程发送终止信号（ProcessHandle.destroy 的 JDK 语义）
        Ok(unsafe { libc::kill(pid as libc::pid_t, sig) } == 0)
    }
}
