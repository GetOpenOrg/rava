//! `java/lang/ProcessHandleImpl$Info` 的 native 方法（与生成的 process_handle_impl_info.rs 共置）。
//!
//! 对标 HotSpot ProcessHandleImpl_unix.c / ProcessHandleImpl_linux.c：
//! - 计时（startTime / totalTime）取 /proc/<pid>/stat，与 isAlive0 同一口径（Info.info 以启动时刻
//!   校验结果，不一致即清空）；
//! - command = readlink /proc/<pid>/exe；arguments = /proc/<pid>/cmdline 去掉 argv[0]；
//!   commandLine = cmdline 以空格连接（command / arguments 缺失时的回退）；
//! - user = /proc/<pid> 属主 uid 经 getpwuid_r 取用户名（取不到时为 uid 数字串，JDK 同形）。
//! 无权读取的项保持 null / -1（其它用户的进程）。

use crate::prelude::*;
use super::process_handle_impl_info::ProcessHandleImpl_Info;
use super::process_handle_impl_impl::proc_stat;

/// uid → 用户名；无对应条目时为数字串（JDK ProcessHandleImpl_unix.c 同形）。
fn user_name(uid: u32) -> std::string::String {
    crate::posix::passwd_name(uid).unwrap_or_else(|| uid.to_string())
}

impl ProcessHandleImpl_Info {
    /// native `initIDs()`：缓存 JNI 字段 ID——无需初始化。
    #[jvm_native]
    pub fn initIDs() -> Result<()> {
        Ok(())
    }

    /// native `info0(long pid)`：采集进程信息写入本对象字段。
    #[jvm_native]
    pub fn info0(&self, pid: i64) -> Result<()> {
        use std::os::unix::fs::MetadataExt;
        if let Some(st) = proc_stat(pid) {
            self.__set_totalTime(st.total_time);
            self.__set_startTime(st.start_time);
        }
        if let Ok(meta) = std::fs::metadata(format!("/proc/{}", pid)) {
            self.__set_user(String::from(user_name(meta.uid()).as_str()));
        }
        let exe = std::fs::read_link(format!("/proc/{}/exe", pid)).ok();
        if let Some(exe) = &exe {
            self.__set_command(String::from(exe.to_string_lossy().as_ref()));
        }
        if let Ok(raw) = std::fs::read(format!("/proc/{}/cmdline", pid)) {
            let args: Vec<std::string::String> = raw
                .split(|b| *b == 0)
                .map(|a| std::string::String::from_utf8_lossy(a).into_owned())
                .collect::<Vec<_>>();
            // cmdline 以 NUL 结尾：末尾空段不是参数
            let args: Vec<std::string::String> = match args.split_last() {
                Some((last, init)) if last.is_empty() => init.to_vec(),
                _ => args,
            };
            if !args.is_empty() {
                let rest: Vec<String> = args[1..].iter().map(|a| String::from(a.as_str())).collect();
                self.__set_arguments(JArray::from(rest));
                self.__set_commandLine(String::from(args.join(" ").as_str()));
            }
        }
        Ok(())
    }
}
