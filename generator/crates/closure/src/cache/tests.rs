//! 跨运行缓存单测：键覆盖每一项输入、损坏即删重建、中断写入可恢复、命中产物逐字节不变。

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use classfile::MemberRef;
use resolve::{ClassPath, Origin};
use serde_json::json;

use super::hash::{hex_of, Fp};
use super::*;

fn tmp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("rava-cache-test-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&d);
    fs::create_dir_all(&d).unwrap();
    d
}

fn write(p: &Path, data: &[u8]) {
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(p, data).unwrap();
}

fn mref(s: &str) -> MemberRef {
    MemberRef { owner: s.into(), name: "main".into(), desc: "([Ljava/lang/String;)V".into() }
}

/// 一组可变的分析输入
#[derive(Clone)]
struct Case {
    analyzer: Vec<u8>,
    user_origin: Origin,
    roots: Vec<MemberRef>,
    seed_roots: Vec<MemberRef>,
    locales: Vec<String>,
    cold_cut: bool,
    flow_batch: Option<usize>,
    release: Vec<String>,
    dropped: Vec<String>,
}

fn base() -> Case {
    Case {
        analyzer: b"rava-1".to_vec(),
        user_origin: Origin::User,
        roots: vec![mref("Main")],
        seed_roots: vec![],
        locales: vec![],
        cold_cut: false,
        flow_batch: None,
        release: vec![],
        dropped: vec![],
    }
}

fn key_of(c: &Case, user: &Path, jar: &Path, rt: &Path) -> String {
    let mut cp = ClassPath::new(21);
    cp.add(c.user_origin, user).unwrap();
    cp.add(Origin::Jdk, jar).unwrap();
    let input = Input {
        cp: &cp,
        runtime_dir: rt,
        roots: c.roots.clone(),
        seed_roots: c.seed_roots.clone(),
        locales: c.locales.clone(),
        diag: Default::default(),
        cold_cut: c.cold_cut,
        flow_batch: c.flow_batch,
    };
    key_with(&c.analyzer, &input, (&c.release, &c.dropped)).unwrap()
}

/// 最小 zip（单个存储条目），作 jmod / jar 档案
fn zip_bytes(name: &str, data: &[u8]) -> Vec<u8> {
    let crc = crc32(data);
    let mut out = Vec::new();
    let n = name.as_bytes();
    let le16 = |v: u16| v.to_le_bytes();
    let le32 = |v: u32| v.to_le_bytes();
    out.extend(le32(0x0403_4b50));
    out.extend(le16(20));
    out.extend(le16(0));
    out.extend(le16(0));
    out.extend(le32(0));
    out.extend(le32(crc));
    out.extend(le32(data.len() as u32));
    out.extend(le32(data.len() as u32));
    out.extend(le16(n.len() as u16));
    out.extend(le16(0));
    out.extend(n);
    out.extend(data);
    let cd = out.len() as u32;
    out.extend(le32(0x0201_4b50));
    out.extend(le16(20));
    out.extend(le16(20));
    out.extend(le16(0));
    out.extend(le16(0));
    out.extend(le32(0));
    out.extend(le32(crc));
    out.extend(le32(data.len() as u32));
    out.extend(le32(data.len() as u32));
    out.extend(le16(n.len() as u16));
    out.extend([0u8; 12]);
    out.extend(le32(0));
    out.extend(n);
    let cd_len = out.len() as u32 - cd;
    out.extend(le32(0x0605_4b50));
    out.extend([0u8; 4]);
    out.extend(le16(1));
    out.extend(le16(1));
    out.extend(le32(cd_len));
    out.extend(le32(cd));
    out.extend(le16(0));
    out
}

fn crc32(data: &[u8]) -> u32 {
    let mut c = !0u32;
    for &b in data {
        c ^= b as u32;
        for _ in 0..8 {
            c = if c & 1 != 0 { (c >> 1) ^ 0xEDB8_8320 } else { c >> 1 };
        }
    }
    !c
}

#[test]
fn hash_is_streaming_and_framed() {
    let data: Vec<u8> = (0..1000u32).map(|i| (i * 7) as u8).collect();
    let mut a = Fp::default();
    a.update(&data);
    let mut b = Fp::default();
    for c in data.chunks(13) {
        b.update(c);
    }
    assert_eq!(a.hex(), b.hex());
    assert_eq!(a.hex(), hex_of(&data));
    assert_ne!(hex_of(b"ab"), hex_of(b"ba"));
    assert_ne!(hex_of(b""), hex_of(&[0]));
    let (mut x, mut y) = (Fp::default(), Fp::default());
    x.field("a", b"bc");
    y.field("ab", b"c");
    assert_ne!(x.hex(), y.hex());
}

#[test]
fn every_input_changes_the_key() {
    let d = tmp("key");
    let (user, jar, rt) = (d.join("user"), d.join("jdk.jmod"), d.join("rt"));
    write(&user.join("Main.class"), b"\xCA\xFE\xBA\xBEmain");
    write(&user.join("p/Helper.class"), b"\xCA\xFE\xBA\xBEhelper");
    fs::write(&jar, zip_bytes("classes/java/lang/Object.class", b"object")).unwrap();
    write(&rt.join("closure.toml"), b"[boundary]\n");
    write(&rt.join("src/java/lang/object_impl.rs"), b"impl Object {}\n");

    let b = base();
    let k0 = key_of(&b, &user, &jar, &rt);
    assert_eq!(k0, key_of(&b, &user, &jar, &rt), "同一输入的键必须稳定");
    assert_eq!(k0.len(), 32);

    let mut seen = vec![k0.clone()];
    let mut differs = |what: &str, k: String| {
        assert!(!seen.contains(&k), "{what} 变化后键未变（或与其它变化撞键）");
        seen.push(k);
    };
    let variants: Vec<(&str, Case)> = vec![
        ("分析器", Case { analyzer: b"rava-2".to_vec(), ..b.clone() }),
        ("档案来源角色", Case { user_origin: Origin::Lib, ..b.clone() }),
        ("入口", Case { roots: vec![mref("Other")], ..b.clone() }),
        ("种子", Case { seed_roots: vec![mref("Seed")], ..b.clone() }),
        ("locale", Case { locales: vec!["zh-CN".into()], ..b.clone() }),
        ("cold_cut", Case { cold_cut: true, ..b.clone() }),
        ("flow_batch", Case { flow_batch: Some(7), ..b.clone() }),
        ("放行", Case { release: vec!["a/b/".into()], ..b.clone() }),
        ("按字节码建模", Case { dropped: vec!["a/b/C".into()], ..b.clone() }),
    ];
    for (what, c) in &variants {
        differs(what, key_of(c, &user, &jar, &rt));
    }
    // flow_batch 缺省值与显式写出缺省值等价（同一次分析）
    let explicit = Case { flow_batch: Some(crate::engine::FLOW_BATCH), ..b.clone() };
    assert_eq!(k0, key_of(&explicit, &user, &jar, &rt));

    let file_edits: Vec<(&str, PathBuf, Option<&[u8]>)> = vec![
        ("用户类内容", user.join("Main.class"), Some(b"\xCA\xFE\xBA\xBEmain2")),
        ("新增用户类", user.join("p/Extra.class"), Some(b"\xCA\xFE\xBA\xBEextra")),
        ("删除用户类", user.join("p/Helper.class"), None),
        ("清单", rt.join("closure.toml"), Some(b"[boundary]\npackages = []\n")),
        ("手写源", rt.join("src/java/lang/object_impl.rs"), Some(b"impl Object { }\n")),
        ("新增手写文件", rt.join("src/java/lang/string_ext.rs"), Some(b"\n")),
        ("jmod", jar.clone(), None),
    ];
    for (what, p, data) in file_edits {
        let old = fs::read(&p).ok();
        match (data, what) {
            (_, "jmod") => fs::write(&p, zip_bytes("classes/java/lang/Object.class", b"object2")).unwrap(),
            (Some(x), _) => write(&p, x),
            (None, _) => fs::remove_file(&p).unwrap(),
        }
        differs(what, key_of(&b, &user, &jar, &rt));
        match old {
            Some(o) => fs::write(&p, o).unwrap(),
            None => fs::remove_file(&p).unwrap(),
        }
        assert_eq!(k0, key_of(&b, &user, &jar, &rt), "{what} 还原后键应复原");
    }
    // 同内容换路径：路径也进键
    let user2 = d.join("user2");
    fs::rename(&user, &user2).unwrap();
    differs("用户类目录路径", key_of(&b, &user2, &jar, &rt));
    let _ = fs::remove_dir_all(&d);
}

fn entry() -> Entry {
    Entry {
        diag: vec!["[closure] 类解析失败：A：x".into()],
        closure: json!({"summary": {"classes": 2, "elapsed_ms": 17, "perf": {"phases_ms": {"flows": 3}}}, "classes": [{"name": "A", "via": null}], "z": "汉字\u{1}"}),
    }
}

#[test]
fn roundtrip_is_byte_identical() {
    let d = tmp("roundtrip");
    let s = Store::open(&d, u64::MAX).unwrap();
    let e = entry();
    assert!(matches!(s.load("k1"), Load::Miss));
    s.save("k1", &e).unwrap();
    let Load::Hit(got) = s.load("k1") else { panic!("应命中") };
    assert_eq!(got, e);
    let pretty = |v: &serde_json::Value| serde_json::to_string_pretty(v).unwrap();
    assert_eq!(pretty(&got.closure), pretty(&e.closure));
    // 命中只换计时字段
    let mut hit = got.closure.clone();
    mark_hit(&mut hit, 5, 1);
    let strip = |mut v: serde_json::Value| {
        let s = v["summary"].as_object_mut().unwrap();
        s.remove("elapsed_ms");
        s.remove("perf");
        v
    };
    assert_eq!(pretty(&strip(hit.clone())), pretty(&strip(e.closure.clone())));
    assert_eq!(hit["summary"]["perf"]["cache"]["hit"], json!(true));
    let _ = fs::remove_dir_all(&d);
}

#[test]
fn corrupt_entries_are_removed_and_rebuilt() {
    let d = tmp("corrupt");
    let s = Store::open(&d, u64::MAX).unwrap();
    let e = entry();
    let p = d.join("k1.entry");
    let good = {
        s.save("k1", &e).unwrap();
        fs::read(&p).unwrap()
    };
    let n = good.len();
    let mut flipped = good.clone();
    flipped[n - 3] ^= 0x20;
    let mut wrong_key = b"rava-closure-cache 1 k2".to_vec();
    wrong_key.extend(&good[good.iter().position(|&b| b == b' ').unwrap() + 5..]);
    let damages: Vec<(&str, Vec<u8>)> = vec![
        ("截断", good[..n - 10].to_vec()),
        ("载荷翻位", flipped),
        ("空文件", vec![]),
        ("无头部", b"{}".to_vec()),
        ("他键头部", wrong_key),
        ("格式版本不符", String::from_utf8_lossy(&good).replacen(" 1 k1 ", " 999 k1 ", 1).into_bytes()),
    ];
    for (what, bytes) in damages {
        fs::write(&p, &bytes).unwrap();
        assert!(matches!(s.load("k1"), Load::Corrupt(_)), "{what}：应判损坏");
        assert!(!p.exists(), "{what}：损坏条目应删除");
        assert!(matches!(s.load("k1"), Load::Miss));
        s.save("k1", &e).unwrap();
        assert!(matches!(s.load("k1"), Load::Hit(ref g) if *g == e), "{what}：重写后应命中");
    }
    let _ = fs::remove_dir_all(&d);
}

#[test]
fn interrupted_write_recovers() {
    let d = tmp("interrupt");
    let s = Store::open(&d, u64::MAX).unwrap();
    // 写到一半中断：只留下临时文件，条目不存在
    let stale = d.join(".k1.4242.0.tmp");
    fs::write(&stale, b"rava-closure-cache 1 k1 999 0\n{\"di").unwrap();
    let fresh = d.join(".k9.4243.0.tmp");
    fs::write(&fresh, b"partial").unwrap();
    assert!(matches!(s.load("k1"), Load::Miss));
    let f = fs::File::options().write(true).open(&stale).unwrap();
    f.set_modified(SystemTime::now() - Duration::from_secs(7200)).unwrap();
    s.save("k1", &entry()).unwrap();
    assert!(matches!(s.load("k1"), Load::Hit(_)));
    assert!(!stale.exists(), "超时的临时文件应清掉");
    assert!(fresh.exists(), "未超时的临时文件（可能是并行进程正在写）保留");
    let _ = fs::remove_dir_all(&d);
}

#[test]
fn eviction_keeps_recent_entries_under_cap() {
    let d = tmp("evict");
    let e = entry();
    let one = {
        let s = Store::open(&d, u64::MAX).unwrap();
        s.save("probe", &e).unwrap();
        let n = fs::metadata(d.join("probe.entry")).unwrap().len();
        fs::remove_file(d.join("probe.entry")).unwrap();
        n
    };
    let s = Store::open(&d, one * 2).unwrap();
    let t0 = SystemTime::now() - Duration::from_secs(100);
    for (i, k) in ["a", "b"].iter().enumerate() {
        s.save(k, &e).unwrap();
        let f = fs::File::options().write(true).open(d.join(format!("{k}.entry"))).unwrap();
        f.set_modified(t0 + Duration::from_secs(i as u64 * 10)).unwrap();
    }
    // 命中刷新 a 的时间：淘汰时 b 最旧
    assert!(matches!(s.load("a"), Load::Hit(_)));
    s.save("c", &e).unwrap();
    assert!(d.join("a.entry").exists());
    assert!(!d.join("b.entry").exists());
    assert!(d.join("c.entry").exists());
    let _ = fs::remove_dir_all(&d);
}
