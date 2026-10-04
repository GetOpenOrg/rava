//! 链接产物的 DWARF 遍历：逐函数按行表分段，每段给出内联链（自内向外的（文件, 行），闭包帧已剔除）。
//!
//! - Mach-O：调试信息留在各目标文件（可执行文件只带调试映射 STABS）。按调试映射逐目标文件建
//!   DWARF 上下文，函数地址经目标文件内同名符号换算；系统 rustlib 下的预编译库跳过（不可能内联
//!   生成代码）。
//! - ELF：DWARF 在可执行文件内，函数取带大小的代码符号。
//!
//! 成帧口径与旧版运行期回溯一致：内联帧各自成帧；函数名（反修饰后）含 `{closure` 的帧不成帧；
//! 无位置的帧不成帧。

use std::borrow::Cow;
use std::collections::HashMap;

use object::{Object, ObjectSection, ObjectSymbol, SymbolKind};
use typed_arena::Arena;

type Reader<'a> = gimli::RelocateReader<gimli::EndianSlice<'a, gimli::RunTimeEndian>, &'a RelocMap>;

#[derive(Default, Debug)]
pub struct RelocMap(object::read::RelocationMap);

impl gimli::read::Relocate for &'_ RelocMap {
    fn relocate_address(&self, offset: usize, value: u64) -> gimli::Result<u64> {
        Ok(self.0.relocate(offset as u64, value))
    }

    fn relocate_offset(&self, offset: usize, value: usize) -> gimli::Result<usize> {
        <usize as gimli::ReaderOffset>::from_u64(self.0.relocate(offset as u64, value as u64))
    }
}

/// 一段代码：可执行文件地址、长度、内联链（自内向外的 (DWARF 文件路径, 行)）
pub trait Sink {
    fn piece(&mut self, start: u64, len: u64, frames: &[(&str, u32)]);
}

/// 一个函数：可执行文件地址、大小、在其 DWARF 上下文中的地址
struct Func {
    exe: u64,
    size: u64,
    obj: u64,
}

/// 遍历统计
#[derive(Default, Debug)]
pub struct WalkStats {
    pub objects: usize,
    pub objects_missing: usize,
    pub functions: usize,
    pub pieces: usize,
}

/// 遍历可执行文件 `exe` 全部有 DWARF 的函数
pub fn walk(exe: &object::File<'_>, exe_data: &[u8], sink: &mut dyn Sink) -> Result<WalkStats, String> {
    let mut stats = WalkStats::default();
    if exe.format() == object::BinaryFormat::MachO {
        walk_debug_map(exe, sink, &mut stats)?;
    } else {
        let mut funcs: Vec<Func> = exe
            .symbols()
            .filter(|s| s.kind() == SymbolKind::Text && s.size() > 0 && s.is_definition())
            .map(|s| Func { exe: s.address(), size: s.size(), obj: s.address() })
            .collect();
        funcs.sort_by_key(|f| (f.exe, f.size));
        funcs.dedup_by_key(|f| f.exe);
        stats.objects = 1;
        with_context(exe_data, |_, ctx| walk_funcs(ctx, &funcs, sink, &mut stats))??;
    }
    Ok(stats)
}

/// Mach-O：按调试映射逐目标文件（同一归档的成员相邻处理，归档只读一次）
fn walk_debug_map(exe: &object::File<'_>, sink: &mut dyn Sink, stats: &mut WalkStats) -> Result<(), String> {
    let map = exe.object_map();
    let mut by_object: HashMap<usize, Vec<&object::ObjectMapEntry<'_>>> = HashMap::new();
    for e in map.symbols() {
        let path = e.object(&map).path();
        if e.size() == 0 || contains(path, b"/lib/rustlib/") {
            continue;
        }
        by_object.entry(e.object_index()).or_default().push(e);
    }
    let mut order: Vec<usize> = by_object.keys().copied().collect();
    order.sort_by_key(|&i| (map.objects()[i].path(), map.objects()[i].member(), i));
    let mut archive: Option<(Vec<u8>, Vec<u8>)> = None; // (路径, 内容)
    for i in order {
        let file = &map.objects()[i];
        let path = String::from_utf8_lossy(file.path()).into_owned();
        let owned;
        let data: &[u8] = match file.member() {
            Some(member) => {
                if archive.as_ref().map(|a| a.0.as_slice()) != Some(file.path()) {
                    archive = std::fs::read(&path).ok().map(|d| (file.path().to_vec(), d));
                }
                let Some((_, bytes)) = archive.as_ref() else {
                    stats.objects_missing += 1;
                    continue;
                };
                match archive_member(bytes, member) {
                    Some(d) => d,
                    None => {
                        stats.objects_missing += 1;
                        continue;
                    }
                }
            }
            None => match std::fs::read(&path) {
                Ok(d) => {
                    owned = d;
                    &owned
                }
                Err(_) => {
                    stats.objects_missing += 1;
                    continue;
                }
            },
        };
        stats.objects += 1;
        let entries = &by_object[&i];
        with_context(data, |obj, ctx| {
            // 目标文件内符号名 → 地址（调试映射以符号名对位）
            let mut addr: HashMap<&[u8], u64> = HashMap::new();
            for s in obj.symbols() {
                if let Ok(n) = s.name_bytes() {
                    addr.entry(n).or_insert(s.address());
                }
            }
            let funcs: Vec<Func> = entries
                .iter()
                .filter_map(|e| addr.get(e.name()).map(|&a| Func { exe: e.address(), size: e.size(), obj: a }))
                .collect();
            walk_funcs(ctx, &funcs, sink, stats)
        })??;
    }
    Ok(())
}

fn contains(hay: &[u8], needle: &[u8]) -> bool {
    hay.windows(needle.len()).any(|w| w == needle)
}

fn archive_member<'a>(data: &'a [u8], member: &[u8]) -> Option<&'a [u8]> {
    let archive = object::read::archive::ArchiveFile::parse(data).ok()?;
    archive.members().filter_map(Result::ok).find(|m| m.name() == member).and_then(|m| m.data(data).ok())
}

/// 在 `data`（目标文件或可执行文件）的 DWARF 上下文中执行 `f`
fn with_context<R>(
    data: &[u8],
    f: impl for<'a> FnOnce(&object::File<'a>, &addr2line::Context<Reader<'a>>) -> R,
) -> Result<R, String> {
    let file = object::File::parse(data).map_err(|e| format!("目标文件解析：{e}"))?;
    let endian = if file.is_little_endian() { gimli::RunTimeEndian::Little } else { gimli::RunTimeEndian::Big };
    let sections: Arena<Vec<u8>> = Arena::new();
    let relocs: Arena<RelocMap> = Arena::new();
    let dwarf = gimli::Dwarf::load(|id| -> Result<Reader<'_>, String> {
        let mut map = RelocMap::default();
        let bytes: &[u8] = match file.section_by_name(id.name()) {
            Some(section) => {
                for (offset, relocation) in section.relocations() {
                    let _ = map.0.add(&file, offset, relocation);
                }
                match section.uncompressed_data().map_err(|e| format!("{}：{e}", id.name()))? {
                    Cow::Borrowed(b) => b,
                    Cow::Owned(v) => sections.alloc(v),
                }
            }
            None => &[],
        };
        Ok(gimli::RelocateReader::new(gimli::EndianSlice::new(bytes, endian), &*relocs.alloc(map)))
    })?;
    let ctx = addr2line::Context::from_dwarf(dwarf).map_err(|e| format!("DWARF：{e}"))?;
    Ok(f(&file, &ctx))
}

/// 逐函数按行表分段，每段取内联链
fn walk_funcs(ctx: &addr2line::Context<Reader<'_>>, funcs: &[Func], sink: &mut dyn Sink, stats: &mut WalkStats) -> Result<(), String> {
    let mut frames: Vec<(&str, u32)> = Vec::new();
    for f in funcs {
        stats.functions += 1;
        let end = f.obj + f.size;
        let pieces = ctx.find_location_range(f.obj, end).map_err(|e| format!("DWARF 行表：{e}"))?;
        for (addr, len, _) in pieces {
            let lo = addr.max(f.obj);
            let hi = addr.saturating_add(len).min(end);
            if lo >= hi {
                continue;
            }
            stats.pieces += 1;
            frames.clear();
            let mut it = ctx.find_frames(lo).skip_all_loads().map_err(|e| format!("DWARF 内联链：{e}"))?;
            while let Some(frame) = it.next().map_err(|e| format!("DWARF 内联链：{e}"))? {
                if frame.function.as_ref().is_some_and(|n| is_closure(n)) {
                    continue;
                }
                if let Some(loc) = &frame.location {
                    if let (Some(file), Some(line)) = (loc.file, loc.line) {
                        frames.push((file, line));
                    }
                }
            }
            sink.piece(f.exe + (lo - f.obj), hi - lo, &frames);
        }
    }
    Ok(())
}

/// 闭包函数（反修饰名含 `{closure`）：原位闭包与外层方法同一行，延迟闭包的位置是创建点，均不成帧
fn is_closure<R: gimli::Reader>(name: &addr2line::FunctionName<R>) -> bool {
    let Ok(raw) = name.raw_name() else { return false };
    // 旧式修饰名含 `closure` 字样，v0 修饰名以 `_R` 开头：其余不必反修饰
    if !raw.contains("closure") && !raw.starts_with("_R") {
        return false;
    }
    name.demangle().is_ok_and(|d| d.contains("{closure"))
}
