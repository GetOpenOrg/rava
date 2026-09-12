#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/VersionProps",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "",
    source      = "VersionProps.java",
))]
pub struct VersionProps;

impl VersionProps {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "init", descriptor = "(Ljava/util/Map;)V", access = "public static"))]
    pub fn init(props: Object) -> Result<()> {
        let _t0 = props.put(String::from("java.version"), String::from("21.0.11"))?;
        let _t1 = props.put(String::from("java.version.date"), String::from("2026-04-21"))?;
        let _t2 = props.put(String::from("java.runtime.version"), String::from("21.0.11"))?;
        let _t3 = props.put(String::from("java.runtime.name"), String::from("OpenJDK Runtime Environment"))?;
        let _t4 = VersionProps::VENDOR_VERSION().isEmpty()?;
        let _t5 = props.put(String::from("java.vendor.version"), VersionProps::VENDOR_VERSION())?;
        let _t6 = props.put(String::from("java.class.version"), String::from("65.0"))?;
        let _t7 = props.put(String::from("java.specification.version"), String::from("21"))?;
        let _t8 = props.put(String::from("java.specification.name"), String::from("Java Platform API Specification"))?;
        let _t9 = props.put(String::from("java.specification.vendor"), String::from("Oracle Corporation"))?;
        let _t10 = props.put(String::from("java.vendor"), String::from("Homebrew"))?;
        let _t11 = props.put(String::from("java.vendor.url"), String::from("https://github.com/Homebrew/homebrew-core/issues"))?;
        let _t12 = props.put(String::from("java.vendor.url.bug"), VersionProps::VENDOR_URL_BUG())?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "parseVersionNumber", descriptor = "(Ljava/lang/String;II)I", access = "private static"))]
    pub fn parseVersionNumber(version: String, prevIndex: i32, index: i32) -> Result<i32> {
        let _t0 = version.charAt(prevIndex)?;
        let _t1: i32 = Character::digit(_t0, 10i32)?;
        String::new().append(&String::from("Leading zeros not supported ("))?;
        let _t2 = version.substring(prevIndex, index)?;
        String::new().append(&_t2)?;
        String::new().append(&String::from(")"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t3: i32 = Integer::parseInt(version, prevIndex, index, 10i32)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "parseVersionNumbers", descriptor = "(Ljava/lang/String;)Ljava/util/List;", access = "static"))]
    pub fn parseVersionNumbers(version: String) -> Result<Object> {
        let mut size: i32 = 0i32;
        let mut prevIndex: i32 = 0i32;
        let _t0 = version.indexOf(46i32, prevIndex)?;
        prevIndex = (_t0).wrapping_add(1i32);
        size = size.wrapping_add(1i32);
        let mut _arr1: Vec<Object> = Vec::with_capacity(size as usize);
        let mut verNumbers: Vec<Object> = _arr1;
        let mut n: i32 = 0i32;
        prevIndex = 0i32;
        let _t2 = version.indexOf(46i32)?;
        let mut index: i32 = _t2;
        loop {
            if index <= -1i32 { break; }
            let _t0: i32 = VersionProps::parseVersionNumber(version, prevIndex, index)?;
            verNumbers[n as usize] = _t0;
            prevIndex = (index).wrapping_add(1i32);
            let _t1 = version.indexOf(46i32, prevIndex)?;
            index = _t1;
            n = n.wrapping_add(1i32);
        }
        let _t3 = version.length()?;
        let _t4: i32 = VersionProps::parseVersionNumber(version, prevIndex, _t3)?;
        verNumbers[n as usize] = _t4;
        String::new().append(&String::from("Leading/trailing zeros not allowed ("))?;
        let _t5: String = Arrays::toString(&verNumbers)?;
        String::new().append(&_t5)?;
        String::new().append(&String::from(")"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t6: Object = List::of(&verNumbers)?;
        Ok(_t6)
    }

    #[cfg_attr(any(), java_method(name = "versionNumbers", descriptor = "()Ljava/util/List;", access = "static"))]
    pub fn versionNumbers() -> Result<Object> {
        let _t0: Object = VersionProps::parseVersionNumbers(String::from("21.0.11"))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "pre", descriptor = "()Ljava/util/Optional;", access = "static"))]
    pub fn pre() -> Result<Object> {
        let _t0: Object = VersionProps::optionalOf(String::from(""))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "build", descriptor = "()Ljava/util/Optional;", access = "static"))]
    pub fn build() -> Result<Object> {
        let _t0 = String::from("").isEmpty()?;
        let _t1: Object = Optional::empty()?;
        let _t2: i32 = Integer::parseInt(String::from(""))?;
        let _t3: Object = Optional::of(_t2)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "optional", descriptor = "()Ljava/util/Optional;", access = "static"))]
    pub fn optional() -> Result<Object> {
        let _t0: Object = VersionProps::optionalOf(String::from(""))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "optionalOf", descriptor = "(Ljava/lang/String;)Ljava/util/Optional;", access = "private static"))]
    pub fn optionalOf(value: String) -> Result<Object> {
        let _t0 = value.isEmpty()?;
        let _t1: Object = Optional::of(value)?;
        return Ok(_t1);
        let _t2: Object = Optional::empty()?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Z)V", access = "private static"))]
    pub fn print(err: bool) -> Result<()> {
        let mut ps: Object = System::out();
        String::new().append(&String::from("openjdk version "21.0.11" 2026-04-21"))?;
        String::from("LTS").append(&String::from(""))?;
        VersionProps::isLTS().println(String::from("LTS"))?;
        String::new().append(&String::from("openjdk 21.0.11 2026-04-21"))?;
        String::from("LTS").append(&String::from(""))?;
        VersionProps::isLTS().println(String::from("LTS"))?;
        let _t0: String = System::getProperty(String::from("jdk.debug"), String::from("release"))?;
        let mut jdk_debug_level: String = _t0;
        let _t1 = String::from("release").equals(jdk_debug_level)?;
        jdk_debug_level = String::from("");
        String::new().append(&jdk_debug_level)?;
        String::new().append(&String::from(""))?;
        jdk_debug_level = String::new();
        let _t2 = VersionProps::VENDOR_VERSION().isEmpty()?;
        String::new().append(&String::from(""))?;
        String::new().append(&VersionProps::VENDOR_VERSION())?;
        let mut vendor_version: String = String::new();
        String::new().append(&String::from("OpenJDK Runtime Environment"))?;
        String::new().append(&vendor_version)?;
        String::new().append(&String::from("("))?;
        String::new().append(&jdk_debug_level)?;
        String::new().append(&String::from("build"))?;
        String::new().append(&String::from("21.0.11"))?;
        String::new().append(&String::from(")"))?;
        ps.println(String::new())?;
        let _t3: String = System::getProperty(String::from("java.vm.name"))?;
        let mut java_vm_name: String = _t3;
        let _t4: String = System::getProperty(String::from("java.vm.version"))?;
        let mut java_vm_version: String = _t4;
        let _t5: String = System::getProperty(String::from("java.vm.info"))?;
        let mut java_vm_info: String = _t5;
        String::new().append(&java_vm_name)?;
        String::new().append(&vendor_version)?;
        String::new().append(&String::from("("))?;
        String::new().append(&jdk_debug_level)?;
        String::new().append(&String::from("build"))?;
        String::new().append(&java_vm_version)?;
        String::new().append(&String::from(","))?;
        String::new().append(&java_vm_info)?;
        String::new().append(&String::from(")"))?;
        ps.println(String::new())?;
        Ok(())
    }
}
