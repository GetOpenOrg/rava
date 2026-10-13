//! 元数据表解码（二进制体积 B1(c)，`docs/plans/2026-10-04-binary-size.md`）：表在二进制中是「字符串池 +
//! 字节流」（发射层 `rava_meta_tables::codec`：整数 LEB128、有符号先 zigzag，串记池下标，列表记长度），
//! 首次查询时解码为 [`crate::meta`] 的元素类型；`&'static str` / `&'static [u8]` 直接指向池内字节，
//! 解码出的切片进程内常驻（每表一次）。各解码函数与发射层同名表的渲染函数字形一一对应。

use crate::meta::{CpVal, FieldMeta, LineMethod, LineNumbers, MethodMeta, NestMeta};

type Names = &'static [&'static str];

/// 池的项索引：池字节为各项「LEB128 长度 + 内容」首尾相接（进程内一次）
pub fn pool_index(cell: &'static std::sync::OnceLock<Vec<&'static [u8]>>, bytes: &'static [u8]) -> &'static [&'static [u8]] {
    cell.get_or_init(|| {
        let mut items = Vec::new();
        let mut at = 0usize;
        while at < bytes.len() {
            let n = leb128(bytes, &mut at) as usize;
            items.push(&bytes[at..at + n]);
            at += n;
        }
        items
    })
}

fn leb128(bytes: &[u8], at: &mut usize) -> u64 {
    let (mut v, mut shift) = (0u64, 0u32);
    loop {
        let b = bytes[*at];
        *at += 1;
        v |= ((b & 0x7f) as u64) << shift;
        if b & 0x80 == 0 {
            return v;
        }
        shift += 7;
    }
}

/// 一张表字节流的顺序读取器
pub struct Reader {
    pool: &'static [&'static [u8]],
    bytes: &'static [u8],
    at: usize,
}

impl Reader {
    pub fn new(pool: &'static [&'static [u8]], bytes: &'static [u8]) -> Self {
        Reader { pool, bytes, at: 0 }
    }

    fn u32(&mut self) -> u32 {
        leb128(self.bytes, &mut self.at) as u32
    }

    fn u64(&mut self) -> u64 {
        leb128(self.bytes, &mut self.at)
    }

    fn i32(&mut self) -> i32 {
        let v = self.u32();
        (v >> 1) as i32 ^ -((v & 1) as i32)
    }

    fn i64(&mut self) -> i64 {
        let v = self.u64();
        (v >> 1) as i64 ^ -((v & 1) as i64)
    }

    fn bytes(&mut self) -> &'static [u8] {
        let id = self.u32() as usize;
        self.pool[id]
    }

    fn str(&mut self) -> &'static str {
        // SAFETY：池中字符串项由发射层以 `str::as_bytes` 写入（UTF-8），按项边界切取
        unsafe { std::str::from_utf8_unchecked(self.bytes()) }
    }

    /// 长度 + 各元素
    fn list<T>(&mut self, mut item: impl FnMut(&mut Reader) -> T) -> &'static [T] {
        let n = self.u32() as usize;
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(item(self));
        }
        Vec::leak(v)
    }

    fn strs(&mut self) -> Names {
        self.list(|r| r.str())
    }

    /// 读到字节流结束的顶层行
    fn rows<T>(mut self, mut item: impl FnMut(&mut Reader) -> T) -> &'static [T] {
        let mut v = Vec::new();
        while self.at < self.bytes.len() {
            v.push(item(&mut self));
        }
        Vec::leak(v)
    }
}

/// 串集行（CLINIT_CLASSES / HIDDEN_CLASSES / RECORD_CLASSES）
pub fn names(r: Reader) -> Names {
    r.rows(|r| r.str())
}

/// (串, 串) 行（CLASS_DIRECT_SUPER / CLASS_SOURCE_FILE / CLASS_DEFINING_LOADER）
pub fn pairs(r: Reader) -> &'static [(&'static str, &'static str)] {
    r.rows(|r| (r.str(), r.str()))
}

/// (类, 整数) 行（CLASS_MODIFIERS / CLASS_ACCESS_FLAGS）
pub fn ints(r: Reader) -> &'static [(&'static str, i32)] {
    r.rows(|r| (r.str(), r.i32()))
}

/// (类, [串]) 行（CLASS_HIERARCHY / CLASS_INTERFACES / PERMITTED_SUBCLASSES / NEST_MEMBERS）
pub fn name_lists(r: Reader) -> &'static [(&'static str, Names)] {
    r.rows(|r| (r.str(), r.strs()))
}

/// CLASS_FIELDS 行：类, [名, 描述符, 位集, 标志（bit0 static / bit1 有常量）, [常量], 注解, Signature]
pub fn fields(r: Reader) -> &'static [(&'static str, &'static [FieldMeta])] {
    r.rows(|r| {
        let class = r.str();
        let list = r.list(|r| {
            let (name, descriptor, modifiers, flags) = (r.str(), r.str(), r.i32(), r.u32());
            let constant = if flags & 2 != 0 { Some(r.i64()) } else { None };
            FieldMeta { name, descriptor, modifiers, is_static: flags & 1 != 0, constant, annotations: r.bytes(), signature: r.str() }
        });
        (class, list)
    })
}

/// CLASS_METHODS 行：类, [名, 描述符, 位集, 标志（bit0 static / bit1 native / bit2 abstract / bit3 inherited）,
/// [throws], 注解, 参数注解, AnnotationDefault, Signature, declared_by]
pub fn methods(r: Reader) -> &'static [(&'static str, &'static [MethodMeta])] {
    r.rows(|r| {
        let class = r.str();
        let list = r.list(|r| {
            let (name, descriptor, modifiers, flags) = (r.str(), r.str(), r.i32(), r.u32());
            MethodMeta {
                name,
                descriptor,
                modifiers,
                is_static: flags & 1 != 0,
                is_native: flags & 2 != 0,
                is_abstract: flags & 4 != 0,
                exceptions: r.strs(),
                annotations: r.bytes(),
                param_annotations: r.bytes(),
                annotation_default: r.bytes(),
                signature: r.str(),
                inherited: flags & 8 != 0,
                declared_by: r.str(),
            }
        });
        (class, list)
    })
}

/// CLASS_NEST 行：类, 外层类, 简单名, 标志（bit0 本类条目 / bit1 有封闭方法）, [封闭方法三元组], [成员类]
pub fn nest(r: Reader) -> &'static [(&'static str, NestMeta)] {
    r.rows(|r| {
        let (class, outer, simple, flags) = (r.str(), r.str(), r.str(), r.u32());
        let enclosing = if flags & 2 != 0 { Some((r.str(), r.str(), r.str())) } else { None };
        (class, NestMeta { outer, simple, self_entry: flags & 1 != 0, enclosing, members: r.strs() })
    })
}

/// CLASS_ANNO 行：类, 注解原始字节, [(常量池下标, 标签, 值)]（标签 0 U / 1 W / 2 I / 3 J / 4 F / 5 D）
pub fn class_anno(r: Reader) -> &'static [(&'static str, &'static [u8], &'static [(i32, CpVal)])] {
    r.rows(|r| {
        let (class, raw) = (r.str(), r.bytes());
        let cp = r.list(|r| {
            let idx = r.i32();
            let val = match r.u32() {
                0 => CpVal::U(r.str()),
                1 => CpVal::W(r.list(|r| r.u32() as u16)),
                2 => CpVal::I(r.i32()),
                3 => CpVal::J(r.i64()),
                4 => CpVal::F(f32::from_bits(r.u32())),
                _ => CpVal::D(f64::from_bits(r.u64())),
            };
            (idx, val)
        });
        (class, raw, cp)
    })
}

/// RECORD_COMPONENTS 行：record 类, [(名, 描述符, Signature)]
pub fn record_components(r: Reader) -> &'static [(&'static str, &'static [(&'static str, &'static str, &'static str)])] {
    r.rows(|r| (r.str(), r.list(|r| (r.str(), r.str(), r.str()))))
}

/// 地址表（`pc_map`，字形见该模块文档）：池与流在映像的 `__rava_pcmap` 节内
pub fn pc_map(pool: &'static [u8], stream: &'static [u8]) -> crate::pc_map::PcMap {
    static POOL: std::sync::OnceLock<Vec<&'static [u8]>> = std::sync::OnceLock::new();
    let mut r = Reader::new(pool_index(&POOL, pool), stream);
    let methods: &'static [LineMethod] = r.list(|r| (r.str(), r.str(), r.str(), r.str(), r.u32(), r.bytes()));
    let lists = r.list(|r| {
        r.list(|r| {
            let method = r.u32();
            let line = match r.u32() {
                0 => crate::pc_map::LINE_NATIVE,
                1 => crate::pc_map::LINE_UNKNOWN,
                j => j as i32 - 2,
            };
            (method, line)
        })
    });
    let mut start = 0i64;
    let mut first = true;
    let ranges = r.list(|r| {
        if first {
            start = r.i64();
            first = false;
        } else {
            start += r.u64() as i64;
        }
        let list = match r.u32() {
            0 => u32::MAX,
            l => l - 1,
        };
        (start, list)
    });
    crate::pc_map::PcMap { methods, lists, ranges }
}

/// LINE_NUMBERS 行：类, 方法名, 描述符, [start_pc 增量, 行号增量]（均 zigzag）
pub fn line_numbers(r: Reader) -> &'static [LineNumbers] {
    r.rows(|r| {
        let (class, name, descriptor) = (r.str(), r.str(), r.str());
        let (mut pc, mut line) = (0i32, 0i32);
        let pairs = r.list(|r| {
            pc += r.i32();
            line += r.i32();
            (pc as u16, line as u16)
        });
        (class, name, descriptor, pairs)
    })
}
