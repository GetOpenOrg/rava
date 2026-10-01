//! 缓存条目的落盘与读取。
//!
//! 文件 `<dir>/<键>.entry`：首行头部 `rava-closure-cache <格式版本> <键> <载荷字节数> <载荷摘要>`，其后为载荷
//! （紧凑 JSON：`{"diag": [...], "closure": {...}}`）。
//!
//! - 写入：先写同目录临时文件 `.<键>.<pid>.<序号>.tmp`，写完再改名——中断只留下临时文件，条目要么完整、要么不存在；
//!   遗留超过 [`TMP_STALE`] 的临时文件在下次写入时清掉（并行进程正在写的临时文件不受影响）。
//! - 读取：头部、长度、摘要、JSON 任一不符即视为损坏，删除条目并按未命中处理（调用方冷算后重写）。
//! - 命中时刷新条目修改时间；写入后总量超过上限即按修改时间从旧到新淘汰。

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

use serde_json::Value;

use super::hash::hex_of;

/// 条目格式版本：载荷结构变化时递增（同时进入缓存键）
pub const FORMAT: u32 = 1;
const MAGIC: &str = "rava-closure-cache";
const TMP_STALE: Duration = Duration::from_secs(3600);

/// 一次分析的可缓存产物
#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    /// 分析期诊断行（标准错误，按原顺序重放）
    pub diag: Vec<String>,
    /// `Closure::to_json()`
    pub closure: Value,
}

pub struct Store {
    dir: PathBuf,
    max_bytes: u64,
}

/// 读取结果
#[derive(Debug)]
pub enum Load {
    Hit(Entry),
    Miss,
    /// 条目损坏（已删除），原因
    Corrupt(String),
}

static SEQ: AtomicU64 = AtomicU64::new(0);

impl Store {
    pub fn open(dir: &Path, max_bytes: u64) -> Result<Store, String> {
        fs::create_dir_all(dir).map_err(|e| format!("{}：{e}", dir.display()))?;
        Ok(Store { dir: dir.to_path_buf(), max_bytes })
    }

    fn path(&self, key: &str) -> PathBuf {
        self.dir.join(format!("{key}.entry"))
    }

    pub fn load(&self, key: &str) -> Load {
        let p = self.path(key);
        let Ok(bytes) = fs::read(&p) else { return Load::Miss };
        match parse(key, &bytes) {
            Ok(e) => {
                if let Ok(f) = fs::File::options().write(true).open(&p) {
                    let _ = f.set_modified(SystemTime::now());
                }
                Load::Hit(e)
            }
            Err(why) => {
                let _ = fs::remove_file(&p);
                Load::Corrupt(why)
            }
        }
    }

    pub fn save(&self, key: &str, e: &Entry) -> Result<(), String> {
        self.save_parts(key, &e.diag, &e.closure)
    }

    /// 同 [`Store::save`]，产物以借用传入（免整份克隆）
    pub fn save_parts(&self, key: &str, diag: &[String], closure: &Value) -> Result<(), String> {
        self.sweep_tmp();
        let mut payload = b"{\"diag\":".to_vec();
        serde_json::to_writer(&mut payload, diag).map_err(|e| e.to_string())?;
        payload.extend_from_slice(b",\"closure\":");
        serde_json::to_writer(&mut payload, closure).map_err(|e| e.to_string())?;
        payload.push(b'}');
        let head = format!("{MAGIC} {FORMAT} {key} {} {}\n", payload.len(), hex_of(&payload));
        let tmp = self.dir.join(format!(".{key}.{}.{}.tmp", std::process::id(), SEQ.fetch_add(1, Ordering::Relaxed)));
        let io = |e: std::io::Error| format!("{}：{e}", tmp.display());
        let r = (|| {
            let mut f = fs::File::create(&tmp)?;
            f.write_all(head.as_bytes())?;
            f.write_all(&payload)?;
            f.sync_all()?;
            fs::rename(&tmp, self.path(key))
        })();
        if let Err(err) = r {
            let _ = fs::remove_file(&tmp);
            return Err(io(err));
        }
        self.evict(key);
        Ok(())
    }

    /// 遗留的临时文件（写入中断）：超过时限的删除
    fn sweep_tmp(&self) {
        let now = SystemTime::now();
        for d in fs::read_dir(&self.dir).into_iter().flatten().flatten() {
            let n = d.file_name();
            let n = n.to_string_lossy();
            if !(n.starts_with('.') && n.ends_with(".tmp")) {
                continue;
            }
            let old = d.metadata().and_then(|m| m.modified()).ok().and_then(|t| now.duration_since(t).ok());
            if old.is_some_and(|a| a >= TMP_STALE) {
                let _ = fs::remove_file(d.path());
            }
        }
    }

    /// 总量超过上限：按修改时间从旧到新淘汰（刚写入的条目保留）
    fn evict(&self, keep: &str) {
        let mut all: Vec<(SystemTime, u64, PathBuf)> = fs::read_dir(&self.dir)
            .into_iter()
            .flatten()
            .flatten()
            .filter(|d| d.file_name().to_string_lossy().ends_with(".entry"))
            .filter_map(|d| {
                let m = d.metadata().ok()?;
                Some((m.modified().ok()?, m.len(), d.path()))
            })
            .collect();
        let mut total: u64 = all.iter().map(|x| x.1).sum();
        if total <= self.max_bytes {
            return;
        }
        all.sort();
        let keep = self.path(keep);
        for (_, len, p) in all {
            if total <= self.max_bytes {
                break;
            }
            if p != keep && fs::remove_file(&p).is_ok() {
                total -= len;
            }
        }
    }
}

fn parse(key: &str, bytes: &[u8]) -> Result<Entry, String> {
    let nl = bytes.iter().position(|&b| b == b'\n').ok_or("缺头部")?;
    let head = std::str::from_utf8(&bytes[..nl]).map_err(|_| "头部非 UTF-8")?;
    let f: Vec<&str> = head.split(' ').collect();
    let fmt = FORMAT.to_string();
    if f.len() != 5 || f[0] != MAGIC || f[1] != fmt || f[2] != key {
        return Err(format!("头部不符：{head}"));
    }
    let payload = &bytes[nl + 1..];
    if f[3] != payload.len().to_string() {
        return Err(format!("载荷长度 {} ≠ 头部 {}", payload.len(), f[3]));
    }
    if f[4] != hex_of(payload) {
        return Err("载荷摘要不符".into());
    }
    let mut v: Value = serde_json::from_slice(payload).map_err(|e| format!("载荷 JSON：{e}"))?;
    let diag = match v.get_mut("diag").map(Value::take) {
        Some(Value::Array(a)) => a.into_iter().map(|x| x.as_str().map(str::to_string).ok_or("diag 项非字符串")).collect::<Result<_, _>>()?,
        _ => return Err("缺 diag".into()),
    };
    let closure = match v.get_mut("closure").map(Value::take) {
        Some(c @ Value::Object(_)) => c,
        _ => return Err("缺 closure".into()),
    };
    Ok(Entry { diag, closure })
}
