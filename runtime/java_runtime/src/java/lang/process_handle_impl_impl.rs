//! `java/lang/ProcessHandleImpl` 的 native 方法（与生成的 process_handle_impl.rs 共置）。
//!
//! 对标 HotSpot ProcessHandleImpl_unix.c / ProcessHandleImpl_linux.c：pid 查询、存活探测
//! （kill(pid, 0)）、父进程（/proc/<pid>/stat 第 4 列）、信号终止、子进程等待（waitpid /
//! waitid）、进程枚举（getProcessPids0，遍历 /proc）；进程信息 Info.info0 在
//! process_handle_impl_info_impl.rs，共用本文件的 /proc/<pid>/stat 解析。

use crate::prelude::*;
use super::process_handle_impl::ProcessHandleImpl;

/// /proc/<pid>/stat 的父进程号与计时（对标 ProcessHandleImpl_linux.c os_getParentPidAndTimings）。
pub(super) struct ProcStat {
    pub ppid: i64,
    /// 用户态 + 内核态 CPU 时间（纳秒）
    pub total_time: i64,
    /// 启动时刻（epoch 毫秒）；不可得为 0
    pub start_time: i64,
}

/// 时钟节拍频率（sysconf(_SC_CLK_TCK)）。
fn clock_ticks_per_second() -> i64 {
    // SAFETY: sysconf 只读系统配置
    let t = unsafe { libc::sysconf(libc::_SC_CLK_TCK) } as i64;
    if t > 0 { t } else { 100 }
}

/// 系统启动时刻（epoch 毫秒，/proc/stat 的 btime 行）；不可得为 0。
fn boot_time_ms() -> i64 {
    std::fs::read_to_string("/proc/stat").ok()
        .and_then(|s| s.lines().find_map(|l| l.strip_prefix("btime ").map(str::to_owned)))
        .and_then(|v| v.trim().parse::<i64>().ok())
        .map(|secs| secs * 1000)
        .unwrap_or(0)
}

/// 解析 /proc/<pid>/stat：comm 字段可含空格与括号，按最后一个 ')' 之后切分；其后第 0 列为
/// state（第 3 字段），ppid 为第 4 字段，utime / stime 为第 14 / 15 字段，starttime 为第 22 字段。
pub(super) fn proc_stat(pid: i64) -> Option<ProcStat> {
    let stat = std::fs::read_to_string(format!("/proc/{}/stat", pid)).ok()?;
    let rest: Vec<&str> = stat[stat.rfind(')')? + 1..].split_whitespace().collect();
    let field = |i: usize| rest.get(i).and_then(|v| v.parse::<i64>().ok());
    let ppid = field(1)?;
    let hz = clock_ticks_per_second();
    let total_time = match (field(11), field(12)) {
        (Some(u), Some(s)) => (u + s) * (1_000_000_000 / hz),
        _ => -1,
    };
    let boot = boot_time_ms();
    let start_time = match field(19) {
        Some(ticks) if boot != 0 => boot + ticks * 1000 / hz,
        _ => 0,
    };
    Some(ProcStat { ppid, total_time, start_time })
}

fn proc_parent(pid: i64) -> Option<i64> {
    proc_stat(pid).map(|s| s.ppid)
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

    /// native `isAlive0(long pid)`：进程启动时刻（epoch 毫秒）；存活但时间不可得返回 0，
    /// 进程不存在返回 -1。JDK 以启动时刻区分 pid 复用，`Info.info` 也以它校验 info0 的结果，
    /// 两处须同一口径（proc_stat）。
    #[jvm_native]
    pub fn isAlive0(pid: i64) -> Result<i64> {
        if let Some(st) = proc_stat(pid) {
            return Ok(st.start_time);
        }
        // SAFETY: kill(pid, 0) 只做存在性 / 权限探测，不发送信号
        let r = unsafe { libc::kill(pid as libc::pid_t, 0) };
        if r == 0 || std::io::Error::last_os_error().raw_os_error() == Some(libc::EPERM) {
            Ok(0)
        } else {
            Ok(-1)
        }
    }

    /// native `parent0(long pid, long startTime)`：父进程号；不可得返回 -1。
    /// 调用方给出的启动时刻与进程实际启动时刻不符（pid 已复用）时同样返回 -1。
    #[jvm_native]
    pub fn parent0(pid: i64, start_time: i64) -> Result<i64> {
        if pid == std::process::id() as i64 {
            // SAFETY: getppid 无副作用
            return Ok(unsafe { libc::getppid() } as i64);
        }
        Ok(match proc_stat(pid) {
            Some(st) if st.start_time != start_time && st.start_time != 0 && start_time != 0 => -1,
            Some(st) => st.ppid,
            None => -1,
        })
    }

    /// native `getProcessPids0(long pid, long[] pids, long[] ppids, long[] stimes)`：
    /// pid == 0 枚举全部进程，否则只取父进程为 pid 的子进程；逐项写入三个数组（ppids / stimes
    /// 可为 null），返回匹配总数——超过数组长度时只写前 len 项，调用方按返回值扩容重试。
    #[jvm_native(upcalls = "java/lang/IllegalArgumentException.<init>:(Ljava/lang/String;)V")]
    pub fn getProcessPids0(pid: i64, pids: JArray<i64>, ppids: JArray<i64>, stimes: JArray<i64>) -> Result<i32> {
        let size = pids.len()?;
        if (!ppids.is_jvm_null() && ppids.len()? != size)
            || (!stimes.is_jvm_null() && stimes.len()? != size) {
            return Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
                String::from("array sizes not equal"))?));
        }
        let mut count: i32 = 0;
        let dir = match std::fs::read_dir("/proc") {
            Ok(d) => d,
            Err(_) => return Ok(0),
        };
        for entry in dir.flatten() {
            let child: i64 = match entry.file_name().to_str().and_then(|n| n.parse().ok()) {
                Some(p) if p > 0 => p,
                _ => continue,
            };
            let st = match proc_stat(child) {
                Some(st) => st,
                None => continue,
            };
            if pid == 0 || st.ppid == pid {
                if count < size {
                    pids.set(count, child)?;
                    if !ppids.is_jvm_null() {
                        ppids.set(count, st.ppid)?;
                    }
                    if !stimes.is_jvm_null() {
                        stimes.set(count, st.start_time)?;
                    }
                }
                count += 1;
            }
        }
        Ok(count)
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
