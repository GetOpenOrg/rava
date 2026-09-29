//! `java/lang/ProcessHandleImpl` 的 native 方法（与生成的 process_handle_impl.rs 共置）。
//!
//! 对标 HotSpot ProcessHandleImpl_unix.c / ProcessHandleImpl_linux.c：pid 查询、存活探测
//! （kill(pid, 0)）、父进程（/proc/<pid>/stat 第 4 列）、信号终止、子进程等待（waitpid /
//! waitid）。进程枚举（getProcessPids0）与进程信息（Info.info0）保持存根。

use crate::prelude::*;
use super::process_handle_impl::ProcessHandleImpl;

/// /proc/<pid>/stat 的父进程号（第 4 字段；comm 字段可含空格，按最后一个 ')' 之后切分）。
fn proc_parent(pid: i64) -> Option<i64> {
    let stat = std::fs::read_to_string(format!("/proc/{}/stat", pid)).ok()?;
    let rest = &stat[stat.rfind(')')? + 1..];
    rest.split_whitespace().nth(1)?.parse().ok()
}

/// waitid(WNOWAIT)：观察子进程退出而不回收（ProcessHandle.onExit 对非本进程直接子进程的等待）。
#[cfg(target_os = "linux")]
fn wait_no_reap(pid: i64) -> Result<i32> {
    // SAFETY: siginfo_t 全零为合法初值；waitid 只写入该结构
    let mut info: libc::siginfo_t = unsafe { std::mem::zeroed() };
    loop {
        let r = unsafe {
            libc::waitid(libc::P_PID, pid as libc::id_t, &mut info, libc::WEXITED | libc::WNOWAIT)
        };
        if r >= 0 {
            break;
        }
        if std::io::Error::last_os_error().raw_os_error() != Some(libc::EINTR) {
            return Ok(0);
        }
    }
    // SAFETY: waitid 成功后 si_code / si_status 有效
    let (code, st) = unsafe { (info.si_code, info.si_status()) };
    Ok(if code == libc::CLD_KILLED || code == libc::CLD_DUMPED { 0x80 + st } else { st })
}

#[cfg(not(target_os = "linux"))]
fn wait_no_reap(_pid: i64) -> Result<i32> {
    unreachable!("非 Linux 平台按回收等待")
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

    /// native `waitForProcessExit0(long pid, boolean reap)`：阻塞等待子进程退出。
    /// 正常退出返回退出码，被信号终止返回 `0x80 + 信号号`（shell 约定，JDK 同形）；
    /// reap=false 时用 waitid(WNOWAIT) 只观察不回收。非本进程子进程（ECHILD）等错误返回 0。
    #[jvm_native]
    pub fn waitForProcessExit0(pid: i64, reap: bool) -> Result<i32> {
        const SIGNAL_BASE: i32 = 0x80;
        // waitid(WNOWAIT) 的 siginfo 访问器仅 Linux 可移植；其余平台按回收等待
        if reap || !cfg!(target_os = "linux") {
            let mut status = 0;
            loop {
                // SAFETY: 等待指定子进程，状态写入本地变量
                if unsafe { libc::waitpid(pid as libc::pid_t, &mut status, 0) } >= 0 {
                    break;
                }
                if std::io::Error::last_os_error().raw_os_error() != Some(libc::EINTR) {
                    return Ok(0);
                }
            }
            if libc::WIFEXITED(status) {
                Ok(libc::WEXITSTATUS(status))
            } else if libc::WIFSIGNALED(status) {
                Ok(SIGNAL_BASE + libc::WTERMSIG(status))
            } else {
                Ok(status)
            }
        } else {
            wait_no_reap(pid)
        }
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
