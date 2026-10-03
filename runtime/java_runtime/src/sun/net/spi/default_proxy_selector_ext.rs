//! DefaultProxySelector 系统代理查询的平台实现（libnet `DefaultProxySelector.c` 的 macOS / unix 两版）。
//!
//! - macOS：CFNetwork 系统代理设置 → `CFNetworkCopyProxiesForURL`，PAC 条目以 5 秒运行循环执行脚本展开；
//! - Linux：优先 GIO `GProxyResolver`（dlopen libgio-2.0），不可用时回落 GConf-2 手动代理配置；
//! - 其他平台：无系统代理设施。
//!
//! 只产出平台无关的 [`SystemProxy`] 列表；`None` 对应 JNI 返回 null。

/// 一条系统代理：直连，或未解析的代理主机与端口。
pub enum SystemProxy {
    Direct,
    Http(std::string::String, i32),
    Socks(std::string::String, i32),
}

#[cfg(target_os = "macos")]
pub use mac::{init, lookup};
#[cfg(target_os = "linux")]
pub use linux::{init, lookup};

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub fn init() -> bool {
    false
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub fn lookup(_proto: &str, _host: &str) -> Option<Vec<SystemProxy>> {
    None
}

#[cfg(target_os = "macos")]
mod mac {
    use super::SystemProxy;
    use std::ffi::{c_char, c_void, CStr};

    type CFTypeRef = *const c_void;
    const UTF8: u32 = 0x0800_0100; // kCFStringEncodingUTF8
    const SINT32: isize = 3; // kCFNumberSInt32Type
    const HOST_BUFFER: usize = 1024;
    const PAC_TIMEOUT_SECS: f64 = 5.0;

    #[repr(C)]
    struct StreamClientContext {
        version: isize,
        info: *mut c_void,
        retain: *const c_void,
        release: *const c_void,
        copy_description: *const c_void,
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    unsafe extern "C" {
        fn CFRelease(cf: CFTypeRef);
        fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
        fn CFEqual(a: CFTypeRef, b: CFTypeRef) -> u8;
        fn CFGetTypeID(cf: CFTypeRef) -> usize;
        fn CFArrayGetTypeID() -> usize;
        fn CFArrayGetCount(a: CFTypeRef) -> isize;
        fn CFArrayGetValueAtIndex(a: CFTypeRef, i: isize) -> CFTypeRef;
        fn CFDictionaryGetValue(d: CFTypeRef, key: CFTypeRef) -> CFTypeRef;
        fn CFNumberGetValue(n: CFTypeRef, ty: isize, out: *mut c_void) -> u8;
        fn CFStringGetCString(s: CFTypeRef, buf: *mut c_char, size: isize, enc: u32) -> u8;
        fn CFStringCreateWithCString(alloc: CFTypeRef, s: *const c_char, enc: u32) -> CFTypeRef;
        fn CFURLCreateWithBytes(alloc: CFTypeRef, bytes: *const u8, len: isize, enc: u32, base: CFTypeRef) -> CFTypeRef;
        fn CFRunLoopGetCurrent() -> CFTypeRef;
        fn CFRunLoopAddSource(rl: CFTypeRef, src: CFTypeRef, mode: CFTypeRef);
        fn CFRunLoopRemoveSource(rl: CFTypeRef, src: CFTypeRef, mode: CFTypeRef);
        fn CFRunLoopRunInMode(mode: CFTypeRef, seconds: f64, return_after_source: u8) -> i32;
        fn CFRunLoopStop(rl: CFTypeRef);
    }

    #[link(name = "CFNetwork", kind = "framework")]
    unsafe extern "C" {
        static kCFProxyTypeKey: CFTypeRef;
        static kCFProxyTypeNone: CFTypeRef;
        static kCFProxyTypeSOCKS: CFTypeRef;
        static kCFProxyTypeAutoConfigurationURL: CFTypeRef;
        static kCFProxyAutoConfigurationURLKey: CFTypeRef;
        static kCFProxyHostNameKey: CFTypeRef;
        static kCFProxyPortNumberKey: CFTypeRef;
        fn CFNetworkCopySystemProxySettings() -> CFTypeRef;
        fn CFNetworkCopyProxiesForURL(url: CFTypeRef, settings: CFTypeRef) -> CFTypeRef;
        fn CFNetworkExecuteProxyAutoConfigurationURL(
            script: CFTypeRef,
            url: CFTypeRef,
            cb: extern "C" fn(*mut c_void, CFTypeRef, CFTypeRef),
            ctx: *mut StreamClientContext,
        ) -> CFTypeRef;
    }

    /// 持有一个 Create / Copy 得到的 CF 对象，离开作用域时释放。
    struct Owned(CFTypeRef);

    impl Owned {
        fn new(cf: CFTypeRef) -> Option<Owned> {
            (!cf.is_null()).then_some(Owned(cf))
        }
    }

    impl Drop for Owned {
        fn drop(&mut self) {
            // SAFETY: 构造时持有一次引用
            unsafe { CFRelease(self.0) };
        }
    }

    /// PAC 执行回调：client 指向结果槽，保留 proxies 或 error 后停止当前运行循环。
    extern "C" fn pac_callback(client: *mut c_void, proxies: CFTypeRef, error: CFTypeRef) {
        // SAFETY: client 为 expand 中的 CFTypeRef 结果槽
        unsafe {
            let slot = client as *mut CFTypeRef;
            *slot = CFRetain(if error.is_null() { proxies } else { error });
            CFRunLoopStop(CFRunLoopGetCurrent());
        }
    }

    pub fn init() -> bool {
        true
    }

    /// 执行 PAC 脚本，把结果数组中的条目追加到 out（保留引用）。
    unsafe fn expand_pac(entry: CFTypeRef, url: CFTypeRef, out: &mut Vec<Owned>) {
        let script = CFDictionaryGetValue(entry, kCFProxyAutoConfigurationURLKey);
        let mut result: CFTypeRef = std::ptr::null();
        let mut ctx = StreamClientContext {
            version: 0,
            info: &mut result as *mut CFTypeRef as *mut c_void,
            retain: std::ptr::null(),
            release: std::ptr::null(),
            copy_description: std::ptr::null(),
        };
        let Some(source) = Owned::new(CFNetworkExecuteProxyAutoConfigurationURL(script, url, pac_callback, &mut ctx)) else {
            return;
        };
        let Some(mode) = Owned::new(CFStringCreateWithCString(
            std::ptr::null(),
            c"sun.net.spi.DefaultProxySelector".as_ptr(),
            UTF8,
        )) else {
            return;
        };
        let rl = CFRunLoopGetCurrent();
        CFRunLoopAddSource(rl, source.0, mode.0);
        CFRunLoopRunInMode(mode.0, PAC_TIMEOUT_SECS, 0);
        CFRunLoopRemoveSource(rl, source.0, mode.0);
        if let Some(result) = Owned::new(result) {
            if CFGetTypeID(result.0) == CFArrayGetTypeID() {
                for i in 0..CFArrayGetCount(result.0) {
                    out.push(Owned(CFRetain(CFArrayGetValueAtIndex(result.0, i))));
                }
            }
        }
    }

    /// 一条非 PAC 代理字典 → SystemProxy；字段缺失为 None。
    unsafe fn convert(entry: CFTypeRef) -> Option<SystemProxy> {
        let ty = CFDictionaryGetValue(entry, kCFProxyTypeKey);
        if ty.is_null() {
            return None;
        }
        if CFEqual(ty, kCFProxyTypeNone) != 0 {
            return Some(SystemProxy::Direct);
        }
        let port_ref = CFDictionaryGetValue(entry, kCFProxyPortNumberKey);
        let mut port: i32 = 0;
        if port_ref.is_null() || CFNumberGetValue(port_ref, SINT32, &mut port as *mut i32 as *mut c_void) == 0 {
            return None;
        }
        let host_ref = CFDictionaryGetValue(entry, kCFProxyHostNameKey);
        let mut buf = [0 as c_char; HOST_BUFFER];
        if host_ref.is_null() || CFStringGetCString(host_ref, buf.as_mut_ptr(), HOST_BUFFER as isize, UTF8) == 0 {
            return None;
        }
        let host = CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned();
        Some(if CFEqual(ty, kCFProxyTypeSOCKS) != 0 {
            SystemProxy::Socks(host, port)
        } else {
            SystemProxy::Http(host, port)
        })
    }

    /// `proto://host` 的系统代理列表；PAC 条目就地展开为脚本给出的条目。
    pub fn lookup(proto: &str, host: &str) -> Option<Vec<SystemProxy>> {
        // SAFETY: 全部 CF 对象经 Owned 管理引用计数；字典取值为 Get 语义不释放
        unsafe {
            let settings = Owned::new(CFNetworkCopySystemProxySettings())?;
            let uri = format!("{proto}://{host}");
            let url = Owned::new(CFURLCreateWithBytes(std::ptr::null(), uri.as_ptr(), uri.len() as isize, UTF8, std::ptr::null()))?;
            let proxies = Owned::new(CFNetworkCopyProxiesForURL(url.0, settings.0))?;
            let mut expanded: Vec<Owned> = Vec::new();
            for i in 0..CFArrayGetCount(proxies.0) {
                let entry = CFArrayGetValueAtIndex(proxies.0, i);
                if entry.is_null() {
                    return None;
                }
                let ty = CFDictionaryGetValue(entry, kCFProxyTypeKey);
                if ty.is_null() {
                    return None;
                }
                if CFEqual(ty, kCFProxyTypeAutoConfigurationURL) == 0 {
                    expanded.push(Owned(CFRetain(entry)));
                } else {
                    expand_pac(entry, url.0, &mut expanded);
                }
            }
            expanded.iter().map(|e| convert(e.0)).collect()
        }
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use super::SystemProxy;
    use std::ffi::{c_char, c_int, c_void, CStr, CString};
    use std::sync::OnceLock;

    type GError = c_void;

    struct Gio {
        get_default: unsafe extern "C" fn() -> *mut c_void,
        lookup: unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_void, *mut *mut GError) -> *mut *mut c_char,
        parse_uri: unsafe extern "C" fn(*const c_char, u16, *mut *mut GError) -> *mut c_void,
        get_hostname: unsafe extern "C" fn(*mut c_void) -> *const c_char,
        get_port: unsafe extern "C" fn(*mut c_void) -> u16,
        strfreev: unsafe extern "C" fn(*mut *mut c_char),
        clear_error: unsafe extern "C" fn(*mut *mut GError),
        object_unref: Option<unsafe extern "C" fn(*mut c_void)>,
    }

    struct GConf {
        client: usize,
        get_string: unsafe extern "C" fn(*mut c_void, *const c_char, *mut *mut GError) -> *mut c_char,
        get_int: unsafe extern "C" fn(*mut c_void, *const c_char, *mut *mut GError) -> c_int,
        get_bool: unsafe extern "C" fn(*mut c_void, *const c_char, *mut *mut GError) -> c_int,
        free: Option<unsafe extern "C" fn(*mut c_void)>,
    }

    enum Backend {
        Gio(Gio),
        GConf(GConf),
        Unavailable,
    }

    static BACKEND: OnceLock<Backend> = OnceLock::new();

    /// dlsym 并转换为函数指针类型 F；符号缺失为 None。
    unsafe fn sym<F: Copy>(handle: *mut c_void, name: &CStr) -> Option<F> {
        let p = libc::dlsym(handle, name.as_ptr());
        (!p.is_null()).then(|| std::mem::transmute_copy::<*mut c_void, F>(&p))
    }

    unsafe fn open(names: &[&CStr], flags: c_int) -> Option<*mut c_void> {
        names.iter().map(|n| libc::dlopen(n.as_ptr(), flags)).find(|h| !h.is_null())
    }

    unsafe fn init_gio() -> Option<Gio> {
        let h = open(&[c"libgio-2.0.so", c"libgio-2.0.so.0"], libc::RTLD_LAZY)?;
        let type_init = sym::<unsafe extern "C" fn()>(h, c"g_type_init");
        let gio = (|| {
            Some(Gio {
                get_default: sym(h, c"g_proxy_resolver_get_default")?,
                lookup: sym(h, c"g_proxy_resolver_lookup")?,
                parse_uri: sym(h, c"g_network_address_parse_uri")?,
                get_hostname: sym(h, c"g_network_address_get_hostname")?,
                get_port: sym(h, c"g_network_address_get_port")?,
                strfreev: sym(h, c"g_strfreev")?,
                clear_error: sym(h, c"g_clear_error")?,
                object_unref: sym(h, c"g_object_unref"),
            })
        })();
        match (type_init, gio) {
            (Some(type_init), Some(gio)) => {
                type_init();
                Some(gio)
            }
            _ => {
                libc::dlclose(h);
                None
            }
        }
    }

    unsafe fn init_gconf() -> Option<GConf> {
        open(&[c"libgconf-2.so", c"libgconf-2.so.4"], libc::RTLD_GLOBAL | libc::RTLD_LAZY)?;
        let all = libc::RTLD_DEFAULT;
        let type_init = sym::<unsafe extern "C" fn()>(all, c"g_type_init")?;
        let get_default = sym::<unsafe extern "C" fn() -> *mut c_void>(all, c"gconf_client_get_default")?;
        type_init();
        let client = get_default();
        if client.is_null() {
            return None;
        }
        Some(GConf {
            client: client as usize,
            get_string: sym(all, c"gconf_client_get_string")?,
            get_int: sym(all, c"gconf_client_get_int")?,
            get_bool: sym(all, c"gconf_client_get_bool")?,
            free: sym(all, c"g_free"),
        })
    }

    fn backend() -> &'static Backend {
        // SAFETY: dlopen / dlsym 只读取库符号；函数指针类型与 GLib / GConf 声明一致
        BACKEND.get_or_init(|| unsafe {
            if let Some(gio) = init_gio() {
                Backend::Gio(gio)
            } else if let Some(gconf) = init_gconf() {
                Backend::GConf(gconf)
            } else {
                Backend::Unavailable
            }
        })
    }

    pub fn init() -> bool {
        !matches!(backend(), Backend::Unavailable)
    }

    pub fn lookup(proto: &str, host: &str) -> Option<Vec<SystemProxy>> {
        match backend() {
            Backend::Gio(gio) => by_gio(gio, proto, host),
            Backend::GConf(gconf) => by_gconf(gconf, proto, host),
            Backend::Unavailable => None,
        }
    }

    /// GProxyResolver：返回 `<protocol>://[user[:password]@]host:port` 或 `direct://` 列表。
    fn by_gio(gio: &Gio, proto: &str, host: &str) -> Option<Vec<SystemProxy>> {
        let uri = CString::new(format!("{proto}://{host}")).ok()?;
        // SAFETY: 按 GIO API 约定调用；proxies 由 g_strfreev 释放，error 由 g_clear_error 清理
        unsafe {
            let resolver = (gio.get_default)();
            if resolver.is_null() {
                return None;
            }
            let mut error: *mut GError = std::ptr::null_mut();
            let proxies = (gio.lookup)(resolver, uri.as_ptr(), std::ptr::null_mut(), &mut error);
            if proxies.is_null() {
                return None;
            }
            let result = if error.is_null() { parse_gio(gio, proxies, &mut error) } else { None };
            (gio.strfreev)(proxies);
            (gio.clear_error)(&mut error);
            result
        }
    }

    unsafe fn parse_gio(gio: &Gio, proxies: *mut *mut c_char, error: &mut *mut GError) -> Option<Vec<SystemProxy>> {
        let mut out = Vec::new();
        let mut i = 0;
        while !(*proxies.add(i)).is_null() {
            let entry = *proxies.add(i);
            i += 1;
            let text = CStr::from_ptr(entry).to_bytes();
            if text.starts_with(b"direct://") {
                out.push(SystemProxy::Direct);
                continue;
            }
            let conn = (gio.parse_uri)(entry, 0, error);
            if conn.is_null() || !error.is_null() {
                return None;
            }
            let host_ptr = (gio.get_hostname)(conn);
            let port = (gio.get_port)(conn);
            let host = (!host_ptr.is_null()).then(|| CStr::from_ptr(host_ptr).to_string_lossy().into_owned());
            if let Some(unref) = gio.object_unref {
                unref(conn);
            }
            let host = host.filter(|_| port > 0)?;
            out.push(if text.starts_with(b"socks") {
                SystemProxy::Socks(host, port as i32)
            } else {
                SystemProxy::Http(host, port as i32)
            });
        }
        Some(out)
    }

    impl GConf {
        fn string(&self, key: &CStr) -> Option<std::string::String> {
            // SAFETY: gconf_client_get_string 返回新分配的串或 NULL，读完以 g_free 释放
            unsafe {
                let p = (self.get_string)(self.client as *mut c_void, key.as_ptr(), std::ptr::null_mut());
                if p.is_null() {
                    return None;
                }
                let s = CStr::from_ptr(p).to_string_lossy().into_owned();
                if let Some(free) = self.free {
                    free(p as *mut c_void);
                }
                Some(s)
            }
        }

        fn int(&self, key: &CStr) -> i32 {
            // SAFETY: 按 GConf API 读取整型键
            unsafe { (self.get_int)(self.client as *mut c_void, key.as_ptr(), std::ptr::null_mut()) }
        }

        fn bool(&self, key: &CStr) -> bool {
            // SAFETY: 按 GConf API 读取布尔键
            unsafe { (self.get_bool)(self.client as *mut c_void, key.as_ptr(), std::ptr::null_mut()) != 0 }
        }

        /// 主机串非空且端口非 0 时给出代理端点。
        fn endpoint(&self, host_key: &CStr, port_key: &CStr) -> Option<(std::string::String, i32)> {
            let host = self.string(host_key);
            let port = self.int(port_key);
            host.filter(|_| port != 0).map(|h| (h, port))
        }
    }

    /// GConf：只认 `/system/proxy/mode = manual` 的手动配置，按协议取主机 / 端口，再查 no_proxy_for 后缀表。
    fn by_gconf(g: &GConf, proto: &str, host: &str) -> Option<Vec<SystemProxy>> {
        let mode = g.string(c"/system/proxy/mode")?;
        if !mode.eq_ignore_ascii_case("manual") {
            return None;
        }
        let mut socks = false;
        let mut endpoint = None;
        if g.bool(c"/system/http_proxy/use_same_proxy") {
            endpoint = g.endpoint(c"/system/http_proxy/host", c"/system/http_proxy/port");
        }
        if endpoint.is_none() {
            let p = proto.to_ascii_lowercase();
            endpoint = match p.as_str() {
                "http" => g.endpoint(c"/system/http_proxy/host", c"/system/http_proxy/port"),
                "https" => g.endpoint(c"/system/proxy/secure_host", c"/system/proxy/secure_port"),
                "ftp" => g.endpoint(c"/system/proxy/ftp_host", c"/system/proxy/ftp_port"),
                "socks" => {
                    let e = g.endpoint(c"/system/proxy/socks_host", c"/system/proxy/socks_port");
                    socks = e.is_some();
                    e
                }
                _ => None,
            };
        }
        let (phost, pport) = endpoint?;
        if let Some(list) = g.string(c"/system/proxy/no_proxy_for") {
            // 与 JNI 一致：逗号 / 空格分隔的后缀，遇到长于主机名的后缀即停止比较
            for suffix in list.split([',', ' ']).filter(|s| !s.is_empty()) {
                if suffix.len() > host.len() {
                    break;
                }
                if host.as_bytes()[host.len() - suffix.len()..].eq_ignore_ascii_case(suffix.as_bytes()) {
                    return None;
                }
            }
        }
        Some(vec![if socks { SystemProxy::Socks(phost, pport) } else { SystemProxy::Http(phost, pport) }])
    }
}
