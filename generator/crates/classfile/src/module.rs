//! 模块描述符（`module-info.class` 的 `Module` / `ModuleResolution` 属性，JVMS §4.7.25）。
//!
//! 只取引导层解析、服务目录与模块图需要的部分：模块名、requires（运行期 / static 分列）、无限定 exports 的有无、
//! uses、provides，以及 `DO_NOT_RESOLVE_BY_DEFAULT`。类名为内部形式（`/` 分隔）。

use crate::constant::{ConstantPool, CpEntry};
use crate::reader::Reader;
use crate::Error;

/// `requires` 的 `ACC_STATIC_PHASE`：编译期依赖，运行期解析不跟随
const REQUIRES_STATIC: u16 = 0x0040;
/// `ModuleResolution` 的 `DO_NOT_RESOLVE_BY_DEFAULT`
const DO_NOT_RESOLVE_BY_DEFAULT: u16 = 0x0001;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ModuleDecl {
    pub name: String,
    /// 运行期 requires（已去掉 `requires static`）
    pub requires: Vec<String>,
    /// `requires static`：编译期依赖（运行期解析不跟随；代码可引用，计入模块可读性）
    pub requires_static: Vec<String>,
    /// 至少有一个无限定 exports
    pub exports_api: bool,
    /// exports：(包（内部形式，`/` 分隔）, 目标模块；空 = 无限定)。访问判定用
    pub exports: Vec<(String, Vec<String>)>,
    /// opens：同 exports 形
    pub opens: Vec<(String, Vec<String>)>,
    pub uses: Vec<String>,
    /// (服务接口, 按声明序的实现类)
    pub provides: Vec<(String, Vec<String>)>,
    pub do_not_resolve_by_default: bool,
}

fn module_name(pool: &ConstantPool, idx: u16) -> Result<String, Error> {
    match pool.get(idx)? {
        CpEntry::Module(n) => Ok(pool.utf8(*n)?.to_string()),
        _ => Err(Error::BadIndex(idx)),
    }
}

/// 解析 `module-info.class`；不是模块描述符（无 `Module` 属性）时为 None
pub fn parse_module_info(data: &[u8]) -> Result<Option<ModuleDecl>, Error> {
    let mut r = Reader::new(data);
    if r.u4()? != 0xCAFE_BABE {
        return Err(Error::BadMagic);
    }
    r.u2()?;
    r.u2()?;
    let pool = ConstantPool::parse(&mut r)?;
    r.u2()?; // access
    r.u2()?; // this_class
    r.u2()?; // super_class
    let n_if = r.u2()?;
    r.skip(2 * n_if as usize)?;
    // module-info 无字段 / 方法（JVMS §4.1）；仍按通用布局跳过
    for _ in 0..2 {
        let n = r.u2()?;
        for _ in 0..n {
            r.skip(6)?;
            let na = r.u2()?;
            for _ in 0..na {
                r.u2()?;
                let len = r.u4()? as usize;
                r.skip(len)?;
            }
        }
    }
    let mut out: Option<ModuleDecl> = None;
    let mut dnr = false;
    let n_attr = r.u2()?;
    for _ in 0..n_attr {
        let name = pool.utf8(r.u2()?)?;
        let len = r.u4()? as usize;
        let body = r.bytes(len)?;
        let mut a = Reader::new(body);
        match name {
            "Module" => out = Some(module_attr(&mut a, &pool)?),
            "ModuleResolution" => dnr = a.u2()? & DO_NOT_RESOLVE_BY_DEFAULT != 0,
            _ => {}
        }
    }
    Ok(out.map(|mut m| {
        m.do_not_resolve_by_default = dnr;
        m
    }))
}

fn module_attr(a: &mut Reader, pool: &ConstantPool) -> Result<ModuleDecl, Error> {
    let mut m = ModuleDecl { name: module_name(pool, a.u2()?)?, ..Default::default() };
    a.u2()?; // flags
    a.u2()?; // version
    for _ in 0..a.u2()? {
        let target = module_name(pool, a.u2()?)?;
        let flags = a.u2()?;
        a.u2()?;
        if flags & REQUIRES_STATIC == 0 {
            m.requires.push(target);
        } else {
            m.requires_static.push(target);
        }
    }
    // exports 与 opens 同形：(包, 标志, to 列表)；exports 的无限定形态计入 API
    for kind in 0..2 {
        for _ in 0..a.u2()? {
            let pkg = match pool.get(a.u2()?)? {
                CpEntry::Package(n) => pool.utf8(*n)?.to_string(),
                _ => return Err(Error::BadIndex(0)),
            };
            a.u2()?; // flags
            let n_to = a.u2()?;
            let mut to = Vec::with_capacity(n_to as usize);
            for _ in 0..n_to {
                to.push(module_name(pool, a.u2()?)?);
            }
            if kind == 0 {
                if n_to == 0 {
                    m.exports_api = true;
                }
                m.exports.push((pkg, to));
            } else {
                m.opens.push((pkg, to));
            }
        }
    }
    for _ in 0..a.u2()? {
        m.uses.push(pool.class_name(a.u2()?)?.to_string());
    }
    for _ in 0..a.u2()? {
        let svc = pool.class_name(a.u2()?)?.to_string();
        let n = a.u2()?;
        let mut with = Vec::with_capacity(n as usize);
        for _ in 0..n {
            with.push(pool.class_name(a.u2()?)?.to_string());
        }
        m.provides.push((svc, with));
    }
    Ok(m)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// 手工组装的最小 module-info：常量池 1 Utf8"Module" 2 Utf8"m" 3 Module#2 4 Utf8"a/S" 5 Class#4
    /// 6 Utf8"a/Impl" 7 Class#6 8 Utf8"java.base" 9 Module#8 10 Utf8"x" 11 Module#10
    /// 12 Utf8"a" 13 Package#12 14 Utf8"ModuleResolution" 15 Utf8"b" 16 Package#15
    pub(crate) fn sample() -> Vec<u8> {
        let mut b: Vec<u8> = vec![0xCA, 0xFE, 0xBA, 0xBE, 0, 0, 0, 53, 0, 17];
        let utf = |b: &mut Vec<u8>, s: &str| {
            b.push(1);
            b.extend((s.len() as u16).to_be_bytes());
            b.extend(s.as_bytes());
        };
        let refc = |b: &mut Vec<u8>, tag: u8, i: u16| {
            b.push(tag);
            b.extend(i.to_be_bytes());
        };
        utf(&mut b, "Module");
        utf(&mut b, "m");
        refc(&mut b, 19, 2);
        utf(&mut b, "a/S");
        refc(&mut b, 7, 4);
        utf(&mut b, "a/Impl");
        refc(&mut b, 7, 6);
        utf(&mut b, "java.base");
        refc(&mut b, 19, 8);
        utf(&mut b, "x");
        refc(&mut b, 19, 10);
        utf(&mut b, "a");
        refc(&mut b, 20, 12);
        utf(&mut b, "ModuleResolution");
        utf(&mut b, "b");
        refc(&mut b, 20, 15);
        // access this super interfaces fields methods
        b.extend([0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        b.extend(2u16.to_be_bytes());
        let body: Vec<u16> = vec![
            3, 0, 0, // name flags version
            2, 9, 0x8000, 0, 11, REQUIRES_STATIC, 0, // requires java.base（mandated）、requires static x
            2, 13, 0, 0, 16, 0, 1, 11, // exports a（无限定）、exports b to x
            1, 13, 0, 1, 11, // opens a to x
            1, 5, // uses a/S
            1, 5, 1, 7, // provides a/S with a/Impl
        ];
        b.extend(1u16.to_be_bytes());
        b.extend(((body.len() * 2) as u32).to_be_bytes());
        for x in body {
            b.extend(x.to_be_bytes());
        }
        b.extend(14u16.to_be_bytes());
        b.extend(2u32.to_be_bytes());
        b.extend(DO_NOT_RESOLVE_BY_DEFAULT.to_be_bytes());
        b
    }

    #[test]
    fn module_info_parse() {
        let m = parse_module_info(&sample()).unwrap().unwrap();
        assert_eq!(m.name, "m");
        assert_eq!(m.requires, vec!["java.base".to_string()]);
        assert_eq!(m.requires_static, vec!["x".to_string()]);
        assert!(m.exports_api);
        assert_eq!(m.exports, vec![("a".to_string(), vec![]), ("b".to_string(), vec!["x".to_string()])]);
        assert_eq!(m.opens, vec![("a".to_string(), vec!["x".to_string()])]);
        assert_eq!(m.uses, vec!["a/S".to_string()]);
        assert_eq!(m.provides, vec![("a/S".to_string(), vec!["a/Impl".to_string()])]);
        assert!(m.do_not_resolve_by_default);
    }
}
