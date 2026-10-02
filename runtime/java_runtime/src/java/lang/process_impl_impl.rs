//! `java.lang.ProcessImpl`（Unix）的 native 层：对标 JDK ProcessImpl_md.c / childproc.c。
//!
//! 派生方式：父进程在 fork 前备好全部 C 字符串与指针表（PATH 候选路径、argv、envp），
//! 子进程只做 async-signal-safe 调用（dup2 / close / chdir / sigprocmask / signal / execve /
//! write / _exit）。exec 失败时子进程经 CLOEXEC 错误管道回传 errno，父进程读到即回收子进程并抛
//! `IOException("Exec failed, error: <errno> (<strerror>) ")`（ProcessBuilder 再包成 `Cannot run program …`）。
//! JDK 的 launchMechanism（posix_spawn + jspawnhelper / vfork / fork）只影响派生手段，
//! 不影响可观测行为，统一按 fork 处理。

use crate::prelude::*;
use super::*;
use std::ffi::{c_char, CString};

/// Java `toCString` 产出的字节数组 → C 字符串（去掉末尾 NUL；null 数组 → None）。
fn c_bytes(a: &JArray<i8>) -> Option<Vec<u8>> {
    if a.is_jvm_null() {
        return None;
    }
    let mut v: Vec<u8> = a.to_vec().into_iter().map(|b| b as u8).collect();
    while v.last() == Some(&0) {
        v.pop();
    }
    Some(v)
}

/// NUL 分隔的字符串块（argBlock / envBlock）→ 各项。
fn split_block(a: &JArray<i8>, count: i32) -> Vec<CString> {
    if a.is_jvm_null() || count <= 0 {
        return Vec::new();
    }
    let bytes: Vec<u8> = a.to_vec().into_iter().map(|b| b as u8).collect();
    bytes
        .split(|b| *b == 0)
        .take(count as usize)
        .map(|s| CString::new(s).unwrap_or_default())
        .collect()
}

/// 可执行文件候选路径：含 `/` 直接执行；否则按父进程 PATH 逐目录拼接
/// （JDK 同形：用父进程的 PATH，缺省 `:/bin:/usr/bin`；空目录项表示当前目录）。
fn exec_candidates(prog: &[u8]) -> Vec<CString> {
    if prog.contains(&b'/') {
        return vec![CString::new(prog).unwrap_or_default()];
    }
    let path = std::env::var_os("PATH")
        .map(|p| std::os::unix::ffi::OsStrExt::as_bytes(p.as_os_str()).to_vec())
        .unwrap_or_else(|| b":/bin:/usr/bin".to_vec());
    path.split(|b| *b == b':')
        .map(|dir| {
            let mut full = if dir.is_empty() { b".".to_vec() } else { dir.to_vec() };
            full.push(b'/');
            full.extend_from_slice(prog);
            CString::new(full).unwrap_or_default()
        })
        .collect()
}

fn set_cloexec(fd: i32) {
    // SAFETY: 对自有描述符设置 FD_CLOEXEC
    unsafe {
        let flags = libc::fcntl(fd, libc::F_GETFD);
        libc::fcntl(fd, libc::F_SETFD, flags | libc::FD_CLOEXEC);
    }
}

fn make_pipe() -> std::io::Result<[i32; 2]> {
    let mut p = [-1i32; 2];
    // SAFETY: pipe 写入两个新描述符
    if unsafe { libc::pipe(p.as_mut_ptr()) } != 0 {
        return Err(std::io::Error::last_os_error());
    }
    set_cloexec(p[0]);
    set_cloexec(p[1]);
    Ok(p)
}

fn close_fd(fd: i32) {
    if fd >= 0 {
        // SAFETY: 关闭自有描述符
        unsafe { libc::close(fd) };
    }
}

/// JDK 21 throwIOException 形态（IOE_FORMAT `"%s, error: %d (%s) "`，末尾含空格）：
/// `<阶段>, error: <errno> (<strerror>) `，阶段为 `Exec failed` / `fork failed` / `pipe failed`。
fn io_error(stage: &str, errno: i32) -> JvmError {
    let text = std::io::Error::from_raw_os_error(errno).to_string();
    let text = match text.find(" (os error ") {
        Some(i) => text[..i].to_owned(),
        None => text,
    };
    match crate::java::io::IOException::new_str(String::from(format!("{}, error: {} ({}) ", stage, errno, text))) {
        Ok(e) => JvmError::from(e),
        Err(e) => e,
    }
}

/// 子进程：重定向标准流、切目录、逐候选 execve；全部失败时经错误管道写 errno 后 _exit。
/// 只含 async-signal-safe 调用（fork 之后多线程进程的子进程约束）。
unsafe fn child(stdio: [i32; 3], dir: Option<&CString>, candidates: &[CString],
                argv: &[*const c_char], envp: &[*const c_char], err_fd: i32) -> ! {
    for (target, fd) in stdio.iter().enumerate() {
        if *fd != target as i32 {
            if libc::dup2(*fd, target as i32) < 0 {
                report(err_fd);
            }
        } else {
            // 已在目标位置：清掉可能的 CLOEXEC
            let flags = libc::fcntl(*fd, libc::F_GETFD);
            libc::fcntl(*fd, libc::F_SETFD, flags & !libc::FD_CLOEXEC);
        }
    }
    if let Some(d) = dir {
        if libc::chdir(d.as_ptr()) != 0 {
            report(err_fd);
        }
    }
    // 恢复默认信号处置与空信号掩码（Rust 运行时忽略 SIGPIPE；JVM 子进程为默认处置）
    libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    let mut empty: libc::sigset_t = std::mem::zeroed();
    libc::sigemptyset(&mut empty);
    libc::sigprocmask(libc::SIG_SETMASK, &empty, std::ptr::null_mut());

    let mut saw_eacces = false;
    for c in candidates {
        libc::execve(c.as_ptr(), argv.as_ptr(), envp.as_ptr());
        let e = *errno_location();
        if e == libc::EACCES {
            saw_eacces = true;
        } else if e != libc::ENOENT && e != libc::ENOTDIR {
            report_errno(err_fd, e);
        }
    }
    report_errno(err_fd, if saw_eacces { libc::EACCES } else { libc::ENOENT })
}

#[cfg(target_os = "linux")]
unsafe fn errno_location() -> *mut i32 {
    libc::__errno_location()
}

#[cfg(target_os = "macos")]
unsafe fn errno_location() -> *mut i32 {
    libc::__error()
}

unsafe fn report(err_fd: i32) -> ! {
    report_errno(err_fd, *errno_location())
}

unsafe fn report_errno(err_fd: i32, e: i32) -> ! {
    let b = e.to_ne_bytes();
    libc::write(err_fd, b.as_ptr() as *const libc::c_void, b.len());
    libc::_exit(127)
}

impl ProcessImpl {
    /// native `init()`（`<clinit>` 末尾调用）：JDK ProcessImpl_md.c 在此缓存 PATH 拆分
    /// 结果（parentPathv）与 JNI 字段 ID，供 forkAndExec 解析可执行文件。原生二进制无 JNI
    /// 字段缓存；PATH 在 forkAndExec 落地时按调用时刻的环境解析（与 JDK 在 <clinit> 快照
    /// 的差异仅在运行期修改 PATH 时可观察，而 Java 无修改进程环境的 API）。无可观察状态。
    #[jvm_native]
    pub fn init() -> Result<()> {
        Ok(())
    }

    /// native `forkAndExec(mode, helperpath, prog, argBlock, argc, envBlock, envc, dir, fds,
    /// redirectErrorStream)`：派生子进程，返回 pid。
    ///
    /// `fds[i] == -1` 表示该标准流走管道（父进程保留另一端，回写到 `fds[i]`），否则是子进程
    /// 直接使用的描述符（INHERIT 为 0/1/2，文件重定向为已打开的描述符），回写 -1。
    /// redirectErrorStream 时子进程 stderr 复制 stdout，`fds[2]` 回写 -1（getErrorStream 为空流）。
    #[jvm_native]
    #[allow(clippy::too_many_arguments)]
    pub fn forkAndExec(&self, _mode: i32, _helperpath: JArray<i8>, prog: JArray<i8>,
                       arg_block: JArray<i8>, argc: i32, env_block: JArray<i8>, envc: i32,
                       dir: JArray<i8>, fds: JArray<i32>, redirect_error_stream: bool) -> Result<i32> {
        let prog = c_bytes(&prog).unwrap_or_default();
        let prog_c = CString::new(prog.clone()).unwrap_or_default();
        let args = split_block(&arg_block, argc);
        let mut argv: Vec<*const c_char> = vec![prog_c.as_ptr()];
        argv.extend(args.iter().map(|a| a.as_ptr()));
        argv.push(std::ptr::null());
        // envBlock 为 null → 继承父进程环境（fork 前在父进程展开，子进程不读全局 environ）
        let envs: Vec<CString> = if env_block.is_jvm_null() {
            use std::os::unix::ffi::OsStrExt;
            std::env::vars_os()
                .map(|(k, v)| {
                    let mut kv = k.as_bytes().to_vec();
                    kv.push(b'=');
                    kv.extend_from_slice(v.as_bytes());
                    CString::new(kv).unwrap_or_default()
                })
                .collect()
        } else {
            split_block(&env_block, envc)
        };
        let mut envp: Vec<*const c_char> = envs.iter().map(|e| e.as_ptr()).collect();
        envp.push(std::ptr::null());
        let dir_c = c_bytes(&dir).map(|d| CString::new(d).unwrap_or_default());
        let candidates = exec_candidates(&prog);

        let req = [fds.get(0)?, fds.get(1)?, fds.get(2)?];
        let mut pipes: [Option<[i32; 2]>; 3] = [None, None, None];
        let mut opened: Vec<i32> = Vec::new();
        let result = (|| -> std::result::Result<i32, (&'static str, i32)> {
            for i in 0..3 {
                if req[i] == -1 && !(i == 2 && redirect_error_stream) {
                    let p = make_pipe().map_err(|e| ("pipe failed", e.raw_os_error().unwrap_or(libc::EIO)))?;
                    opened.extend_from_slice(&p);
                    pipes[i] = Some(p);
                }
            }
            let err_pipe = make_pipe().map_err(|e| ("pipe failed", e.raw_os_error().unwrap_or(libc::EIO)))?;
            // 子进程视角的 0/1/2：管道取子进程一端（stdin 读端、stdout/stderr 写端）
            let child_in = pipes[0].map(|p| p[0]).unwrap_or(req[0]);
            let child_out = pipes[1].map(|p| p[1]).unwrap_or(req[1]);
            let child_err = if redirect_error_stream {
                child_out
            } else {
                pipes[2].map(|p| p[1]).unwrap_or(req[2])
            };
            // SAFETY: fork 后子进程只执行 child() 中的 async-signal-safe 调用
            let pid = unsafe { libc::fork() };
            if pid < 0 {
                close_fd(err_pipe[0]);
                close_fd(err_pipe[1]);
                return Err(("fork failed", std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EAGAIN)));
            }
            if pid == 0 {
                unsafe {
                    child([child_in, child_out, child_err], dir_c.as_ref(), &candidates,
                          &argv, &envp, err_pipe[1])
                }
            }
            close_fd(err_pipe[1]);
            let mut buf = [0u8; 4];
            let mut got = 0usize;
            loop {
                // SAFETY: 读自有管道到本地缓冲
                let n = unsafe {
                    libc::read(err_pipe[0], buf[got..].as_mut_ptr() as *mut libc::c_void, 4 - got)
                };
                if n > 0 {
                    got += n as usize;
                    if got == 4 { break; }
                } else if n == 0 || std::io::Error::last_os_error().raw_os_error() != Some(libc::EINTR) {
                    break;
                }
            }
            close_fd(err_pipe[0]);
            if got == 4 {
                let mut status = 0;
                // SAFETY: 回收 exec 失败的子进程
                unsafe { libc::waitpid(pid, &mut status, 0) };
                return Err(("Exec failed", i32::from_ne_bytes(buf)));
            }
            Ok(pid)
        })();

        match result {
            Ok(pid) => {
                // 关闭子进程一端，父进程一端回写 fds
                if let Some(p) = pipes[0] { close_fd(p[0]); }
                if let Some(p) = pipes[1] { close_fd(p[1]); }
                if let Some(p) = pipes[2] { close_fd(p[1]); }
                fds.set(0, pipes[0].map(|p| p[1]).unwrap_or(-1))?;
                fds.set(1, pipes[1].map(|p| p[0]).unwrap_or(-1))?;
                fds.set(2, pipes[2].map(|p| p[0]).unwrap_or(-1))?;
                Ok(pid)
            }
            Err((stage, errno)) => {
                for fd in opened {
                    close_fd(fd);
                }
                Err(io_error(stage, errno))
            }
        }
    }
}
