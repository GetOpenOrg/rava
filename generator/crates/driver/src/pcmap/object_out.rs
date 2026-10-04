//! 表目标文件：一个数据节，与运行时 `pc_map` 的占位项同名同段，链接器把两者首尾拼接。
//! - Mach-O：`__DATA,__rava_pcmap`，`S_ATTR_NO_DEAD_STRIP`（`-dead_strip` 下保留），平台版本取自产物；
//! - ELF：`rava_pcmap`，`SHF_ALLOC | SHF_WRITE | SHF_GNU_RETAIN`（`--gc-sections` 下保留；节名是 C 标识符，
//!   `__start_` / `__stop_` 引用亦保留），另带空 `.note.GNU-stack`（不要求可执行栈）。

use object::write::Object;
use object::{elf, macho, BinaryFormat, Object as _, SectionFlags, SectionKind};

/// 为产物 `exe` 生成承载 `blob` 的目标文件字节
pub fn object_file(exe: &object::File<'_>, exe_data: &[u8], blob: &[u8]) -> Result<Vec<u8>, String> {
    let format = exe.format();
    let mut obj = Object::new(format, exe.architecture(), exe.endianness());
    let section = match format {
        BinaryFormat::MachO => {
            if let Some(v) = macho_build_version(exe_data)? {
                obj.set_macho_build_version(v);
            }
            let id = obj.add_section(b"__DATA".to_vec(), b"__rava_pcmap".to_vec(), SectionKind::Data);
            obj.section_mut(id).flags =
                SectionFlags::MachO { flags: macho::S_REGULAR | macho::S_ATTR_NO_DEAD_STRIP, reserved2: 0 };
            id
        }
        BinaryFormat::Elf => {
            let id = obj.add_section(Vec::new(), b"rava_pcmap".to_vec(), SectionKind::Data);
            obj.section_mut(id).flags = SectionFlags::Elf {
                sh_type: elf::SHT_PROGBITS,
                sh_flags: elf::SHF_ALLOC | elf::SHF_WRITE | elf::SHF_GNU_RETAIN,
            };
            let note = obj.add_section(Vec::new(), b".note.GNU-stack".to_vec(), SectionKind::Other);
            obj.section_mut(note).flags = SectionFlags::Elf { sh_type: elf::SHT_PROGBITS, sh_flags: elf::SectionFlags::default() };
            id
        }
        other => return Err(format!("不支持的产物格式 {other:?}")),
    };
    obj.set_section_data(section, blob.to_vec(), 8);
    obj.write().map_err(|e| format!("表目标文件：{e}"))
}

/// 产物的 LC_BUILD_VERSION（目标文件缺平台版本时 ld64 告警）
fn macho_build_version(exe_data: &[u8]) -> Result<Option<object::write::MachOBuildVersion>, String> {
    use object::read::macho::MachOFile64;
    let file = MachOFile64::<object::Endianness>::parse(exe_data).map_err(|e| format!("Mach-O：{e}"))?;
    let endian = file.endian();
    Ok(file.build_version().map_err(|e| format!("Mach-O：{e}"))?.map(|(v, _)| {
        let mut out = object::write::MachOBuildVersion::default();
        out.platform = v.platform.get(endian);
        out.minos = v.minos.get(endian);
        out.sdk = v.sdk.get(endian);
        out
    }))
}
