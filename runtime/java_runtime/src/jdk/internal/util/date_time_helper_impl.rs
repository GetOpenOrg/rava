//! `jdk/internal/util/DateTimeHelper` 手写伴生：内部边界类（JDK 25 新增），按调用链
//! 按需实现（K-2 规则）。
//!
//! JDK 25 起 `LocalDate` / `LocalTime` / `LocalDateTime.toString` 改经本工具类输出
//! ISO-8601 文本。按 javap 字节码逐分支照写，文本在 Rust 侧拼好后一次追加
//!（`StringBuilder.append(String)`，与逐段 append 的可观察结果相同）：
//!   - 日期：|year| < 1000 → 负号 + 补零到 4 位；year > 9999 → 前缀 `+`；
//!     月 / 日不足两位补 `0`；
//!   - 时间：时:分 必出；秒或纳秒非零才出 `:ss`；纳秒非零时出 `.` + 前导零
//!    （9 − 位数）+ 按 毫秒 / 微秒 / 纳秒 截去尾零后的值。

use crate::prelude::*;
use super::date_time_helper::DateTimeHelper;
use crate::java::lang::StringBuilder;
use crate::java::time::{LocalDate, LocalDateTime, LocalTime};

fn date_text(year: i32, month: i32, day: i32) -> std::string::String {
    let mut s = std::string::String::new();
    let abs = year.unsigned_abs();
    if abs < 1000 {
        if year < 0 {
            s.push('-');
        }
        let zeros = if abs < 10 { 3 } else if abs < 100 { 2 } else { 1 };
        s.push_str(&"0".repeat(zeros));
        s.push_str(&abs.to_string());
    } else {
        if year > 9999 {
            s.push('+');
        }
        s.push_str(&year.to_string());
    }
    s.push_str(if month < 10 { "-0" } else { "-" });
    s.push_str(&month.to_string());
    s.push_str(if day < 10 { "-0" } else { "-" });
    s.push_str(&day.to_string());
    s
}

fn time_text(hour: i32, minute: i32, second: i32, nano: i32) -> std::string::String {
    let mut s = std::string::String::new();
    s.push_str(if hour < 10 { "0" } else { "" });
    s.push_str(&hour.to_string());
    s.push_str(if minute < 10 { ":0" } else { ":" });
    s.push_str(&minute.to_string());
    if (second | nano) > 0 {
        s.push_str(if second < 10 { ":0" } else { ":" });
        s.push_str(&second.to_string());
        if nano > 0 {
            s.push('.');
            let zeros = 9 - nano.to_string().len() as i32;   // DecimalDigits.stringSize
            if zeros > 0 {
                s.push_str(&"0".repeat(zeros as usize));
            }
            let v = if nano % 1_000_000 == 0 {
                nano / 1_000_000
            } else if nano % 1000 == 0 {
                nano / 1000
            } else {
                nano
            };
            s.push_str(&v.to_string());
        }
    }
    s
}

impl DateTimeHelper {
    /// static `formatTo(StringBuilder, LocalDateTime)`：日期 + `T` + 时间。
    #[jvm_boundary(upcalls = "java/time/LocalDateTime.toLocalDate:()Ljava/time/LocalDate; java/time/LocalDateTime.toLocalTime:()Ljava/time/LocalTime; java/time/LocalDate.getYear:()I java/time/LocalDate.getMonthValue:()I java/time/LocalDate.getDayOfMonth:()I java/time/LocalTime.getHour:()I java/time/LocalTime.getMinute:()I java/time/LocalTime.getSecond:()I java/time/LocalTime.getNano:()I java/lang/StringBuilder.append:(Ljava/lang/String;)Ljava/lang/StringBuilder;")]
    pub fn formatTo_sb_localdatetime(buf: StringBuilder, dateTime: LocalDateTime) -> Result<()> {
        Self::formatTo_sb_localdate(Clone::clone(&buf), dateTime.toLocalDate()?)?;
        buf.append_str(String::from("T"))?;
        Self::formatTo_sb_localtime(buf, dateTime.toLocalTime()?)
    }

    /// static `formatTo(StringBuilder, LocalDate)`：`[±]yyyy-MM-dd`。
    #[jvm_boundary(upcalls = "java/time/LocalDate.getYear:()I java/time/LocalDate.getMonthValue:()I java/time/LocalDate.getDayOfMonth:()I java/lang/StringBuilder.append:(Ljava/lang/String;)Ljava/lang/StringBuilder;")]
    pub fn formatTo_sb_localdate(buf: StringBuilder, date: LocalDate) -> Result<()> {
        let text = date_text(date.getYear()?, date.getMonthValue()?, date.getDayOfMonth()?);
        buf.append_str(String::from(text.as_str()))?;
        Ok(())
    }

    /// static `formatTo(StringBuilder, LocalTime)`：`HH:mm[:ss[.fraction]]`。
    #[jvm_boundary(upcalls = "java/time/LocalTime.getHour:()I java/time/LocalTime.getMinute:()I java/time/LocalTime.getSecond:()I java/time/LocalTime.getNano:()I java/lang/StringBuilder.append:(Ljava/lang/String;)Ljava/lang/StringBuilder;")]
    pub fn formatTo_sb_localtime(buf: StringBuilder, time: LocalTime) -> Result<()> {
        let text = time_text(time.getHour()?, time.getMinute()?, time.getSecond()?, time.getNano()?);
        buf.append_str(String::from(text.as_str()))?;
        Ok(())
    }
}
