//! 本程序的 jimage（`lib/modules` 格式，计划 docs/plans/2026-10-05-boot-image-evaluator.md §5.7）。
//!
//! 模块资源（闭包读取到的 JDK 模块资源，含 `<类名>.class`）写成 JDK 运行时映像格式的字节块，嵌入用户侧
//! 元数据；运行期 `NativeImageBuffer.getNativeMap`（native，手写）把它作为 `${java.home}/lib/modules`
//! 的内存映射交给翻译的 `BasicImageReader` / `ImageReader`，`SystemModuleReader`、jrt 协议照 JDK 字节码执行。
//!
//! 格式与 JDK 21 / 25 的 `jdk.internal.jimage` 读侧一致（写侧对照 jlink 的 `BasicImageWriter` /
//! `ImageResourcesTree`）：
//! - 头 7 个 int（本机字节序，目标平台小端）：magic、版本、flags、资源数、表长、位置区长、字符串区长；
//! - 重定向表与位置偏移表各 `表长` 个 int（完美哈希，读侧 `BasicImageReader.getLocationIndex`）；
//! - 位置区：首字节保留为 `ATTRIBUTE_END`，每个位置为压缩属性流（`ImageLocation.compress`）；
//! - 字符串区：MUTF-8、NUL 结尾，偏移 0 为空串；
//! - 内容区：资源字节（不压缩）与目录内容（`/modules/…` 子项位置偏移、`/packages/<包>` 的 (isEmpty, 模块名串偏移) 对）。

use std::collections::{BTreeMap, BTreeSet, HashMap};

use input::ModuleResource;

const MAGIC: u32 = 0xCAFE_DADA;
const MAJOR_VERSION: u32 = 1;
const MINOR_VERSION: u32 = 0;
const HEADER_SLOTS: usize = 7;
/// 哈希乘子与缺省种子（`ImageStringsReader.HASH_MULTIPLIER`）
const HASH_MULTIPLIER: i32 = 0x0100_0193;
const POSITIVE_MASK: i32 = 0x7FFF_FFFF;
/// 单桶找种子的上限：超过即放大表长重来（jlink `PerfectHashBuilder` 同一策略）
const SEED_RETRIES: i32 = 1000;

const ATTRIBUTE_MODULE: u8 = 1;
const ATTRIBUTE_PARENT: u8 = 2;
const ATTRIBUTE_BASE: u8 = 3;
const ATTRIBUTE_EXTENSION: u8 = 4;
const ATTRIBUTE_OFFSET: u8 = 5;
const ATTRIBUTE_COMPRESSED: u8 = 6;
const ATTRIBUTE_UNCOMPRESSED: u8 = 7;

/// 一个 UTF-16 码元的 MUTF-8 编码（NUL 为 `C0 80`）
fn mutf8_unit(u: u16, out: &mut Vec<u8>) {
    match u {
        0x0001..=0x007F => out.push(u as u8),
        0x0000 | 0x0080..=0x07FF => {
            out.push(0xC0 | (u >> 6) as u8);
            out.push(0x80 | (u & 0x3F) as u8);
        }
        _ => {
            out.push(0xE0 | (u >> 12) as u8);
            out.push(0x80 | ((u >> 6) & 0x3F) as u8);
            out.push(0x80 | (u & 0x3F) as u8);
        }
    }
}

fn mutf8(s: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(s.len());
    for u in s.encode_utf16() {
        mutf8_unit(u, &mut out);
    }
    out
}

/// `ImageStringsReader.hashCode(s, seed)`：逐 MUTF-8 字节 `h = h * 乘子 ^ b`，取正
fn hash(s: &str, seed: i32) -> i32 {
    mutf8(s).into_iter().fold(seed, |h, b| h.wrapping_mul(HASH_MULTIPLIER) ^ i32::from(b)) & POSITIVE_MASK
}

/// 字符串区（`ImageStringsWriter`）：偏移 0 为空串，偏移 1 为 "class"（与 jlink 同形）
struct Strings {
    bytes: Vec<u8>,
    offsets: HashMap<String, u32>,
}

impl Strings {
    fn new() -> Self {
        let mut s = Strings { bytes: Vec::new(), offsets: HashMap::new() };
        s.add("");
        s.add("class");
        s
    }

    fn add(&mut self, s: &str) -> u32 {
        if let Some(&o) = self.offsets.get(s) {
            return o;
        }
        let o = self.bytes.len() as u32;
        self.bytes.extend(mutf8(s));
        self.bytes.push(0);
        self.offsets.insert(s.to_string(), o);
        o
    }
}

/// 一个位置：全名（哈希键）、内容区偏移与长度
struct Location {
    name: String,
    offset: u64,
    size: u64,
}

/// `ImageLocationWriter.newLocation` 的名字拆分：(模块, 父路径, 基名, 扩展名)
fn split_name(full: &str) -> (&str, &str, &str, &str) {
    if let Some(rest) = full.strip_prefix("/modules/") {
        return ("modules", "", rest, "");
    }
    if let Some(rest) = full.strip_prefix("/packages/") {
        return ("packages", "", rest, "");
    }
    let mut module = "";
    let mut name = full;
    if full.len() >= 2 && full.starts_with('/') {
        if let Some(i) = full[1..].find('/') {
            module = &full[1..i + 1];
            name = &full[i + 2..];
        }
    }
    let mut parent = "";
    if let Some(i) = name.rfind('/').filter(|&i| i > 1) {
        parent = &name[..i];
        name = &name[i + 1..];
    }
    match name.rfind('.') {
        Some(i) => (module, parent, &name[..i], &name[i + 1..]),
        None => (module, parent, name, ""),
    }
}

/// `ImageLocation.compress`：非零属性逐个写 `(kind << 3) | (字节数 - 1)` + 大端值，0 结尾
fn compress(attrs: &[(u8, u64)], out: &mut Vec<u8>) {
    for &(kind, value) in attrs {
        if value == 0 {
            continue;
        }
        let n = ((63 - value.leading_zeros()) >> 3) as u8;
        out.push((kind << 3) | n);
        for i in (0..=n).rev() {
            out.push((value >> (8 * u32::from(i))) as u8);
        }
    }
    out.push(0);
}

/// 完美哈希（读侧规则：`redirect[hash(name) % n]` 为负 → 下标 `-1 - v`，为正 → 种子，下标
/// `hash(name, v) % n`，为 0 → 不存在）。返回 (重定向表, 各下标对应的输入序号)
fn perfect_hash(names: &[&str]) -> (Vec<i32>, Vec<usize>) {
    let mut count = names.len().max(1);
    'grow: loop {
        let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); count];
        for (i, n) in names.iter().enumerate() {
            buckets[hash(n, HASH_MULTIPLIER) as usize % count].push(i);
        }
        let mut order: Vec<usize> = (0..count).filter(|&b| !buckets[b].is_empty()).collect();
        order.sort_by(|a, b| buckets[*b].len().cmp(&buckets[*a].len()).then(a.cmp(b)));
        let mut redirect = vec![0i32; count];
        let mut slots: Vec<Option<usize>> = vec![None; count];
        for &b in &order {
            let members = &buckets[b];
            if members.len() == 1 {
                let Some(free) = slots.iter().position(Option::is_none) else { unreachable!("槽数不少于名字数") };
                slots[free] = Some(members[0]);
                redirect[b] = -1 - free as i32;
                continue;
            }
            let found = (1..=SEED_RETRIES).map(|k| HASH_MULTIPLIER + k).find_map(|seed| {
                let picked: Vec<usize> = members.iter().map(|&i| hash(names[i], seed) as usize % count).collect();
                let distinct: BTreeSet<usize> = picked.iter().copied().collect();
                (distinct.len() == picked.len() && picked.iter().all(|&s| slots[s].is_none())).then_some((seed, picked))
            });
            let Some((seed, picked)) = found else {
                count = (count + 1) | 1;
                continue 'grow;
            };
            redirect[b] = seed;
            for (&i, s) in members.iter().zip(picked) {
                slots[s] = Some(i);
            }
        }
        // 空槽（表长大于名字数时）指向首个位置：读侧经名字校验判定不存在
        let index = slots.into_iter().map(|s| s.unwrap_or(0)).collect();
        return (redirect, index);
    }
}

/// 目录树节点内容：子目录（`/modules/...` 全名）与资源（`/<模块>/<名>` 全名）
#[derive(Default)]
struct Dir {
    children: BTreeSet<String>,
}

/// 写出 jimage 字节（`resources` 中同一 (模块, 名) 至多一份）
pub fn write(resources: &[ModuleResource]) -> Vec<u8> {
    // 目录树（ImageResourcesTree）：/modules、/modules/<模块>/<目录…>；/packages、/packages/<包>
    let mut dirs: BTreeMap<String, Dir> = BTreeMap::new();
    dirs.entry("/modules".to_string()).or_default();
    dirs.entry("/packages".to_string()).or_default();
    // 包 → 模块 → 该模块的包目录是否没有直接资源（isEmpty）
    let mut packages: BTreeMap<String, BTreeMap<String, bool>> = BTreeMap::new();
    for r in resources {
        let segs: Vec<&str> = r.name.split('/').collect();
        let mut path = format!("/modules/{}", r.module);
        dirs.get_mut("/modules").map(|d| d.children.insert(path.clone()));
        for (k, seg) in segs[..segs.len() - 1].iter().enumerate() {
            let child = format!("{path}/{seg}");
            dirs.entry(path.clone()).or_default().children.insert(child.clone());
            path = child;
            let pkg = segs[..=k].join(".");
            if !pkg.starts_with("META-INF") {
                let leaf = k == segs.len() - 2;
                let e = packages.entry(pkg).or_default().entry(r.module.clone()).or_insert(true);
                *e &= !leaf;
            }
        }
        dirs.entry(path).or_default().children.insert(format!("/{}/{}", r.module, r.name));
    }
    for pkg in packages.keys() {
        dirs.get_mut("/packages").map(|d| d.children.insert(format!("/packages/{pkg}")));
    }

    // 内容布局：资源在前（输入序），目录与包随后（名字序）；长度先定，偏移随之确定
    let mut locs: Vec<Location> = Vec::new();
    let mut offset = 0u64;
    for r in resources {
        locs.push(Location { name: format!("/{}/{}", r.module, r.name), offset, size: r.bytes.len() as u64 });
        offset += r.bytes.len() as u64;
    }
    for (name, d) in &dirs {
        let size = 4 * d.children.len() as u64;
        locs.push(Location { name: name.clone(), offset, size });
        offset += size;
    }
    for (pkg, mods) in &packages {
        let size = 8 * mods.len() as u64;
        locs.push(Location { name: format!("/packages/{pkg}"), offset, size });
        offset += size;
    }

    // 位置区与字符串区
    let mut strings = Strings::new();
    for (_, mods) in &packages {
        for m in mods.keys() {
            strings.add(m);
        }
    }
    let mut location_bytes = vec![0u8];
    let mut location_at: HashMap<&str, u32> = HashMap::new();
    for l in &locs {
        let (module, parent, base, ext) = split_name(&l.name);
        let attrs = [
            (ATTRIBUTE_MODULE, u64::from(strings.add(module))),
            (ATTRIBUTE_PARENT, u64::from(strings.add(parent))),
            (ATTRIBUTE_BASE, u64::from(strings.add(base))),
            (ATTRIBUTE_EXTENSION, u64::from(strings.add(ext))),
            (ATTRIBUTE_OFFSET, l.offset),
            (ATTRIBUTE_COMPRESSED, 0),
            (ATTRIBUTE_UNCOMPRESSED, l.size),
        ];
        location_at.insert(l.name.as_str(), location_bytes.len() as u32);
        compress(&attrs, &mut location_bytes);
    }
    while location_bytes.len() % 2 != 0 {
        location_bytes.push(0);
    }
    while strings.bytes.len() % 2 != 0 {
        strings.bytes.push(0);
    }

    // 哈希表
    let names: Vec<&str> = locs.iter().map(|l| l.name.as_str()).collect();
    let (redirect, index) = perfect_hash(&names);

    let mut out: Vec<u8> = Vec::with_capacity(offset as usize + location_bytes.len() + strings.bytes.len() + 64);
    let put = |out: &mut Vec<u8>, v: u32| out.extend_from_slice(&v.to_le_bytes());
    put(&mut out, MAGIC);
    put(&mut out, (MAJOR_VERSION << 16) | MINOR_VERSION);
    put(&mut out, 0);
    put(&mut out, locs.len() as u32);
    put(&mut out, redirect.len() as u32);
    put(&mut out, location_bytes.len() as u32);
    put(&mut out, strings.bytes.len() as u32);
    for r in &redirect {
        put(&mut out, *r as u32);
    }
    for &i in &index {
        put(&mut out, location_at[names[i]]);
    }
    out.extend_from_slice(&location_bytes);
    out.extend_from_slice(&strings.bytes);
    debug_assert_eq!(out.len(), 4 * (HEADER_SLOTS + 2 * redirect.len()) + location_bytes.len() + strings.bytes.len());

    // 内容区（与布局同序）
    for r in resources {
        out.extend_from_slice(&r.bytes);
    }
    for d in dirs.values() {
        for c in &d.children {
            put(&mut out, location_at[c.as_str()]);
        }
    }
    for mods in packages.values() {
        for (m, empty) in mods {
            put(&mut out, u32::from(*empty));
            put(&mut out, strings.offsets[m.as_str()]);
        }
    }
    out
}

#[cfg(test)]
#[path = "jimage_tests.rs"]
mod tests;
