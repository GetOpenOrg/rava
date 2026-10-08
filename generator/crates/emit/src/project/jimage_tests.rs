//! jimage 写侧单测：按 JDK 读侧规则（`BasicImageReader.getLocationIndex` / `ImageLocation.decompress`）
//! 回读全部位置，校验名字、内容与目录结构

use super::*;

fn res(module: &str, name: &str, bytes: &[u8]) -> ModuleResource {
    ModuleResource { module: module.to_string(), name: name.to_string(), bytes: bytes.to_vec() }
}

fn int_at(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(b[off..off + 4].try_into().unwrap())
}

fn string_at(strings: &[u8], off: usize) -> String {
    let end = strings[off..].iter().position(|&b| b == 0).unwrap() + off;
    String::from_utf8(strings[off..end].to_vec()).unwrap()
}

/// 读侧：全名 → (属性数组)；未命中为 None
fn find(image: &[u8], name: &str) -> Option<[u64; 8]> {
    let len = int_at(image, 16) as usize;
    let loc_size = int_at(image, 20) as usize;
    let redirect = |i: usize| int_at(image, 28 + 4 * i) as i32;
    let offsets = |i: usize| int_at(image, 28 + 4 * len + 4 * i) as usize;
    let loc_base = 28 + 8 * len;
    let str_base = loc_base + loc_size;
    let v = redirect(hash(name, HASH_MULTIPLIER) as usize % len);
    let index = match v {
        0 => return None,
        v if v < 0 => (-1 - v) as usize,
        v => hash(name, v) as usize % len,
    };
    let mut p = loc_base + offsets(index);
    let mut attrs = [0u64; 8];
    loop {
        let data = image[p];
        p += 1;
        if data <= 7 {
            break;
        }
        let kind = (data >> 3) as usize;
        let n = (data & 7) as usize + 1;
        attrs[kind] = image[p..p + n].iter().fold(0u64, |a, &b| (a << 8) | u64::from(b));
        p += n;
    }
    let s = |k: usize| string_at(&image[str_base..], attrs[k] as usize);
    let mut full = String::new();
    if attrs[1] != 0 {
        full += &format!("/{}/", s(1));
    }
    if attrs[2] != 0 {
        full += &format!("{}/", s(2));
    }
    full += &s(3);
    if attrs[4] != 0 {
        full += &format!(".{}", s(4));
    }
    (full == name).then_some(attrs)
}

fn content<'a>(image: &'a [u8], attrs: &[u64; 8]) -> &'a [u8] {
    let len = int_at(image, 16) as usize;
    let index_size = 28 + 8 * len + int_at(image, 20) as usize + int_at(image, 24) as usize;
    let start = index_size + attrs[5] as usize;
    &image[start..start + attrs[7] as usize]
}

#[test]
fn hash_matches_jdk_rules() {
    // h = 0x01000193；逐字节 h = h * 0x01000193 ^ b，取正
    let manual = [b'/', b'a'].iter().fold(HASH_MULTIPLIER, |h, &b| h.wrapping_mul(HASH_MULTIPLIER) ^ i32::from(b)) & POSITIVE_MASK;
    assert_eq!(hash("/a", HASH_MULTIPLIER), manual);
    assert_eq!(mutf8("\u{0}é"), vec![0xC0, 0x80, 0xC3, 0xA9]);
}

#[test]
fn split_follows_location_writer() {
    assert_eq!(split_name("/modules/java.base/java/lang"), ("modules", "", "java.base/java/lang", ""));
    assert_eq!(split_name("/packages/java.lang"), ("packages", "", "java.lang", ""));
    assert_eq!(split_name("/java.base/java/lang/Object.class"), ("java.base", "java/lang", "Object", "class"));
    assert_eq!(split_name("/java.base/module-info.class"), ("java.base", "", "module-info", "class"));
    assert_eq!(split_name("/modules"), ("", "", "/modules", ""));
}

#[test]
fn roundtrip_resources_and_tree() {
    let rs = vec![
        res("java.base", "java/lang/Object.class", b"cafe"),
        res("java.base", "java/lang/invoke/x.dat", b"12345"),
        res("java.base", "META-INF/services/p.Q", b"q"),
        res("jdk.zipfs", "jdk/nio/zipfs/a.properties", b""),
    ];
    let image = write(&rs);
    assert_eq!(int_at(&image, 0), MAGIC);
    for r in &rs {
        let a = find(&image, &format!("/{}/{}", r.module, r.name)).expect("资源可查");
        assert_eq!(content(&image, &a), r.bytes.as_slice());
    }
    assert!(find(&image, "/java.base/java/lang/String.class").is_none());
    // 目录：/modules/java.base/java/lang 含子目录 invoke 与资源 Object.class
    let dir = find(&image, "/modules/java.base/java/lang").expect("目录可查");
    assert_eq!(content(&image, &dir).len(), 8);
    // 包：java.lang 在 java.base 有直接资源（isEmpty = 0），java 只有子目录（isEmpty = 1）
    let pkg = find(&image, "/packages/java.lang").expect("包可查");
    assert_eq!(int_at(content(&image, &pkg), 0), 0);
    let java = find(&image, "/packages/java").expect("父包可查");
    assert_eq!(int_at(content(&image, &java), 0), 1);
    assert!(find(&image, "/packages/META-INF").is_none());
    assert!(find(&image, "/modules").is_some() && find(&image, "/packages").is_some());
}

#[test]
fn empty_image_has_root_directories() {
    let image = write(&[]);
    assert!(find(&image, "/modules").is_some());
    assert!(find(&image, "/java.base/x").is_none());
}

#[test]
fn many_names_hash_perfectly() {
    let rs: Vec<ModuleResource> = (0..2000).map(|i| res("m", &format!("p{}/r{i}.bin", i % 37), &[i as u8])).collect();
    let image = write(&rs);
    for r in &rs {
        let a = find(&image, &format!("/m/{}", r.name)).expect("资源可查");
        assert_eq!(content(&image, &a), r.bytes.as_slice());
    }
}
