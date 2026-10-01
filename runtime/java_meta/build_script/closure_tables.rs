//! closure.json 派生表。模块服务表（seeds.services）：(服务, provider) 二元组，事实序，供
//! BootLoader.getServicesCatalog 装填引导服务目录（boundary-narrowing §6.11）。
//! 表以 `__java_meta_MODULE_SERVICES` 符号导出，java_runtime::meta 以同名 extern 声明读取——
//! VM 初始系统属性表（system_properties）同法以 `__java_meta_VM_CONST_PROPERTIES` /
//! `__java_meta_VM_DYNAMIC_PROPERTIES` 导出。
//! 这些事实随闭包（即用户代码）变化，放在本 crate 才不连带重编 java_runtime。

use std::fs;
use std::path::Path;

// 事实形如 `{"service": S, "providers": [{"class": P, "module": M | null}]}`；module 为 null 的是类路径
// provider（META-INF/services），JDK 经 LazyClassPathLookupIterator 而非服务目录发现，不入表。
// closure.json 缺席（旧 scratch / 手工构建）→ 空表。build 依赖只有 std，故就地极简 JSON 解析。

enum Json { Null, Other, Str(String), Arr(Vec<Json>), Obj(Vec<(String, Json)>) }

impl Json {
    fn get(&self, key: &str) -> Option<&Json> {
        match self { Json::Obj(kv) => kv.iter().find(|(k, _)| k == key).map(|(_, v)| v), _ => None }
    }
    fn str(&self) -> Option<&str> { match self { Json::Str(s) => Some(s), _ => None } }
    fn arr(&self) -> &[Json] { match self { Json::Arr(a) => a, _ => &[] } }
}

struct JsonCur<'a> { b: &'a [u8], i: usize }

impl JsonCur<'_> {
    fn ws(&mut self) { while self.i < self.b.len() && self.b[self.i].is_ascii_whitespace() { self.i += 1; } }
    fn eat(&mut self, c: u8) -> bool { self.ws(); if self.b.get(self.i) == Some(&c) { self.i += 1; true } else { false } }
    fn string(&mut self) -> Option<String> {
        let mut out = Vec::new();
        loop {
            let c = *self.b.get(self.i)?;
            self.i += 1;
            match c {
                b'"' => return String::from_utf8(out).ok(),
                b'\\' => {
                    let e = *self.b.get(self.i)?;
                    self.i += 1;
                    match e {
                        b'n' => out.push(b'\n'), b't' => out.push(b'\t'), b'r' => out.push(b'\r'),
                        b'b' => out.push(8), b'f' => out.push(12),
                        b'u' => {
                            let h = std::str::from_utf8(self.b.get(self.i..self.i + 4)?).ok()?;
                            self.i += 4;
                            let ch = char::from_u32(u32::from_str_radix(h, 16).ok()?).unwrap_or('\u{fffd}');
                            out.extend_from_slice(ch.encode_utf8(&mut [0; 4]).as_bytes());
                        }
                        other => out.push(other),
                    }
                }
                _ => out.push(c),
            }
        }
    }
    fn value(&mut self) -> Option<Json> {
        self.ws();
        match *self.b.get(self.i)? {
            b'"' => { self.i += 1; self.string().map(Json::Str) }
            b'[' => {
                self.i += 1;
                let mut a = Vec::new();
                if self.eat(b']') { return Some(Json::Arr(a)); }
                loop { a.push(self.value()?); if self.eat(b']') { return Some(Json::Arr(a)); } if !self.eat(b',') { return None; } }
            }
            b'{' => {
                self.i += 1;
                let mut kv = Vec::new();
                if self.eat(b'}') { return Some(Json::Obj(kv)); }
                loop {
                    if !self.eat(b'"') { return None; }
                    let k = self.string()?;
                    if !self.eat(b':') { return None; }
                    kv.push((k, self.value()?));
                    if self.eat(b'}') { return Some(Json::Obj(kv)); }
                    if !self.eat(b',') { return None; }
                }
            }
            _ => {
                let st = self.i;
                while self.i < self.b.len() && !b",]} \t\r\n".contains(&self.b[self.i]) { self.i += 1; }
                Some(if &self.b[st..self.i] == b"null" { Json::Null } else { Json::Other })
            }
        }
    }
}

/// (服务, provider) 二元组：模块 provider（module 非 null），事实序
fn module_service_pairs(root: &Json) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let services = root.get("seeds").and_then(|s| s.get("services"));
    for svc in services.map(Json::arr).unwrap_or(&[]) {
        let Some(service) = svc.get("service").and_then(Json::str) else { continue };
        for p in svc.get("providers").map(Json::arr).unwrap_or(&[]) {
            let module_named = p.get("module").is_some_and(|m| !matches!(m, Json::Null));
            if let (true, Some(class)) = (module_named, p.get("class").and_then(Json::str)) {
                out.push((service.to_owned(), class.to_owned()));
            }
        }
    }
    out
}

fn services_src(root: Option<&Json>) -> String {
    let pairs = root.map(module_service_pairs).unwrap_or_default();
    let mut src = String::from(
        "#[export_name = \"__java_meta_MODULE_SERVICES\"] pub static MODULE_SERVICES: &[(&str, &str)] = &[\n");
    for (s, p) in &pairs {
        src += &format!("    ({s:?}, {p:?}),\n");
    }
    src + "];\n"
}

// ── VM 初始系统属性表（closure.json system_properties）──────────────────────────────
// 事实形如 `{"values": {键: 值}, "dynamic": [键…]}`，即分析器折叠属性读点所用的清单表
// （vm_intrinsics.toml `[facts.system_properties]`）原样导出。运行时 System.registerNatives
// 只写入这两类键：常量键按表中值写入，动态键由手写层取宿主值。两者之外的键不写入
// （分析器按缺省 null 折叠），运行时属性表与折叠结论同源。

fn system_properties_src(root: Option<&Json>) -> String {
    let sp = root.and_then(|r| r.get("system_properties"));
    if root.is_some() && sp.is_none() {
        println!("cargo:warning=java_meta 构建脚本: closure.json 缺 system_properties，初始系统属性表为空");
    }
    let mut src = String::from(
        "#[export_name = \"__java_meta_VM_CONST_PROPERTIES\"] pub static VM_CONST_PROPERTIES: &[(&str, &str)] = &[\n");
    if let Some(Json::Obj(kv)) = sp.and_then(|s| s.get("values")) {
        for (k, v) in kv {
            if let Some(v) = v.str() {
                src += &format!("    ({k:?}, {v:?}),\n");
            }
        }
    }
    src += "];\n#[export_name = \"__java_meta_VM_DYNAMIC_PROPERTIES\"] pub static VM_DYNAMIC_PROPERTIES: &[&str] = &[\n";
    for k in sp.and_then(|s| s.get("dynamic")).map(Json::arr).unwrap_or(&[]) {
        if let Some(k) = k.str() {
            src += &format!("    {k:?},\n");
        }
    }
    src + "];\n"
}

fn write_if_changed(path: &Path, src: &str) {
    if fs::read_to_string(path).ok().as_deref() != Some(src) {
        fs::write(path, src).unwrap();
    }
}

/// closure.json 派生的服务表与系统属性表。closure.json 缺席（旧 scratch / 手工构建）或解析失败 → 空表
pub(crate) fn write_closure_tables(closure_json: &Path) {
    println!("cargo:rerun-if-changed={}", closure_json.display());
    let root = fs::read_to_string(closure_json).ok().and_then(|t| {
        let r = (JsonCur { b: t.as_bytes(), i: 0 }).value();
        if r.is_none() {
            println!("cargo:warning=java_meta 构建脚本: closure.json 解析失败，服务表与系统属性表为空");
        }
        r
    });
    let Ok(out_dir) = std::env::var("OUT_DIR") else { return };
    let out_dir = Path::new(&out_dir);
    write_if_changed(&out_dir.join("services_table.rs"), &services_src(root.as_ref()));
    write_if_changed(&out_dir.join("system_properties.rs"), &system_properties_src(root.as_ref()));
}
