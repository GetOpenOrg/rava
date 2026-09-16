#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::math::*;
use crate::java::nio::*;
use crate::java::nio::charset::*;
use crate::java::security::*;
use crate::java::text::*;
use crate::java::text::spi::*;
use crate::java::time::*;
use crate::java::time::chrono::*;
use crate::java::time::temporal::*;
use crate::java::time::zone::*;
use crate::java::util::*;
use crate::java::util::concurrent::*;
use crate::java::util::concurrent::atomic::*;
use crate::java::util::concurrent::locks::*;
use crate::java::util::function::*;
use crate::java::util::regex::*;
use crate::java::util::spi::*;
use crate::java::util::stream::*;
use crate::java::util::zip::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::reflect::misc::*;
use crate::sun::security::action::*;
use crate::sun::security::util::*;
use crate::sun::text::*;
use crate::sun::util::*;
use crate::sun::util::calendar::*;
use crate::sun::util::locale::*;
use crate::sun::util::locale::provider::*;
use crate::sun::util::spi::*;
use crate::java::text::Normalizer;
use crate::jdk::internal::misc::Unsafe;
use crate::jdk::internal::misc::VM;
use crate::jdk::internal::reflect::Reflection;
use crate::jdk::internal::util::StaticProperty;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/System"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "System.java"]
    #[inner_classes     = "java/lang/System$CallersHolder:java/lang/System:CallersHolder:10;java/lang/System$1:::0;java/lang/System$Logger:java/lang/System:Logger:1545;java/lang/System$LoggerFinder:java/lang/System:LoggerFinder:1033;java/util/Map$Entry:java/util/Map:Entry:1545;java/lang/System$2:::0;java/lang/System$Logger$Level:java/lang/System$Logger:Level:16409;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/lang/System"]

    pub struct System;

    impl System {
        #[cfg_attr(any(), java_field(name = "in", descriptor = "Ljava/io/InputStream;", access = "public", modifiers = "static final", is_static = true))]
        // static field: in:Ljava/io/InputStream;
        pub fn in_() -> InputStream {
            panic!("stub: java/lang/System.in:Ljava/io/InputStream;")
        }

        #[cfg_attr(any(), java_field(name = "initialIn", descriptor = "Ljava/io/InputStream;", access = "private", modifiers = "static", is_static = true))]
        // static field: initialIn:Ljava/io/InputStream;
        pub fn initialIn() -> InputStream {
            panic!("stub: java/lang/System.initialIn:Ljava/io/InputStream;")
        }

        #[cfg_attr(any(), java_field(name = "NEVER", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: NEVER:I
        pub fn NEVER() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "MAYBE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: MAYBE:I
        pub fn MAYBE() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "allowSecurityManager", descriptor = "I", access = "private", modifiers = "static", is_static = true))]
        // static field: allowSecurityManager:I
        pub fn allowSecurityManager_field() -> i32 {
            panic!("stub: java/lang/System.allowSecurityManager:I")
        }

        #[cfg_attr(any(), java_field(name = "security", descriptor = "Ljava/lang/SecurityManager;", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: security:Ljava/lang/SecurityManager;
        pub fn security() -> SecurityManager {
            panic!("stub: java/lang/System.security:Ljava/lang/SecurityManager;")
        }

        #[cfg_attr(any(), java_field(name = "notSupportedJnuEncoding", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static", is_static = true))]
        // static field: notSupportedJnuEncoding:Ljava/lang/String;
        pub fn notSupportedJnuEncoding() -> String {
            panic!("stub: java/lang/System.notSupportedJnuEncoding:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "cons", descriptor = "Ljava/io/Console;", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: cons:Ljava/io/Console;
        pub fn cons() -> Object {
            panic!("stub: java/lang/System.cons:Ljava/io/Console;")
        }

        #[cfg_attr(any(), java_field(name = "initialErrStream", descriptor = "Ljava/io/PrintStream;", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: initialErrStream:Ljava/io/PrintStream;
        pub fn initialErrStream() -> PrintStream {
            panic!("stub: java/lang/System.initialErrStream:Ljava/io/PrintStream;")
        }

        #[cfg_attr(any(), java_field(name = "props", descriptor = "Ljava/util/Properties;", access = "private", modifiers = "static", is_static = true))]
        // static field: props:Ljava/util/Properties;
        pub fn props() -> Properties {
            panic!("stub: java/lang/System.props:Ljava/util/Properties;")
        }

        #[cfg_attr(any(), java_field(name = "lineSeparator", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static", is_static = true))]
        // static field: lineSeparator:Ljava/lang/String;
        pub fn lineSeparator_field() -> String {
            panic!("stub: java/lang/System.lineSeparator:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "bootLayer", descriptor = "Ljava/lang/ModuleLayer;", access = "package", modifiers = "static", is_static = true))]
        // static field: bootLayer:Ljava/lang/ModuleLayer;
        pub fn bootLayer() -> Object {
            panic!("stub: java/lang/System.bootLayer:Ljava/lang/ModuleLayer;")
        }

        #[native]
        #[java_native(name = "registerNatives", descriptor = "()V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn registerNatives() -> Result<()> {
            panic!("native: java/lang/System.registerNatives:()V")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/System.<init>:()V")
        }

        #[java_method(name = "allowSecurityManager", descriptor = "()Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn allowSecurityManager() -> Result<bool> {
            Ok(System::allowSecurityManager_field() != 1i32)
        }

        #[java_method(name = "setIn", descriptor = "(Ljava/io/InputStream;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setIn(in_: InputStream) -> Result<()> {
            panic!("stub: java/lang/System.setIn:(Ljava/io/InputStream;)V")
        }

        #[java_method(name = "setOut", descriptor = "(Ljava/io/PrintStream;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setOut(out: PrintStream) -> Result<()> {
            panic!("stub: java/lang/System.setOut:(Ljava/io/PrintStream;)V")
        }

        #[java_method(name = "setErr", descriptor = "(Ljava/io/PrintStream;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setErr(err: PrintStream) -> Result<()> {
            panic!("stub: java/lang/System.setErr:(Ljava/io/PrintStream;)V")
        }

        #[java_method(name = "console", descriptor = "()Ljava/io/Console;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn console() -> Result<Object> {
            panic!("stub: java/lang/System.console:()Ljava/io/Console;")
        }

        #[java_method(name = "inheritedChannel", descriptor = "()Ljava/nio/channels/Channel;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn inheritedChannel() -> Result<Object> {
            panic!("stub: java/lang/System.inheritedChannel:()Ljava/nio/channels/Channel;")
        }

        #[java_method(name = "checkIO", descriptor = "()V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkIO() -> Result<()> {
            panic!("stub: java/lang/System.checkIO:()V")
        }

        #[native]
        #[java_native(name = "setIn0", descriptor = "(Ljava/io/InputStream;)V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn setIn0(arg0: InputStream) -> Result<()> {
            panic!("native: java/lang/System.setIn0:(Ljava/io/InputStream;)V")
        }

        #[native]
        #[java_native(name = "setOut0", descriptor = "(Ljava/io/PrintStream;)V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn setOut0(arg0: PrintStream) -> Result<()> {
            panic!("native: java/lang/System.setOut0:(Ljava/io/PrintStream;)V")
        }

        #[native]
        #[java_native(name = "setErr0", descriptor = "(Ljava/io/PrintStream;)V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn setErr0(arg0: PrintStream) -> Result<()> {
            panic!("native: java/lang/System.setErr0:(Ljava/io/PrintStream;)V")
        }

        #[java_method(name = "codeSource", descriptor = "(Ljava/lang/Class;)Ljava/net/URL;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Ljava/net/URL;")]
        pub fn codeSource(clazz: Object) -> Result<Object> {
            panic!("stub: java/lang/System.codeSource:(Ljava/lang/Class;)Ljava/net/URL;")
        }

        #[java_method(name = "setSecurityManager", descriptor = "(Ljava/lang/SecurityManager;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn setSecurityManager(sm: SecurityManager) -> Result<()> {
            panic!("stub: java/lang/System.setSecurityManager:(Ljava/lang/SecurityManager;)V")
        }

        #[java_method(name = "implSetSecurityManager", descriptor = "(Ljava/lang/SecurityManager;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn implSetSecurityManager(sm: SecurityManager) -> Result<()> {
            panic!("stub: java/lang/System.implSetSecurityManager:(Ljava/lang/SecurityManager;)V")
        }

        #[java_method(name = "setSecurityManager0", descriptor = "(Ljava/lang/SecurityManager;)V", access = "private", modifiers = "static synchronized", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setSecurityManager0(s: SecurityManager) -> Result<()> {
            panic!("stub: java/lang/System.setSecurityManager0:(Ljava/lang/SecurityManager;)V")
        }

        #[java_method(name = "getSecurityManager", descriptor = "()Ljava/lang/SecurityManager;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getSecurityManager() -> Result<SecurityManager> {
            let _t0: bool = System::allowSecurityManager()?;
            if _t0 {
                return Ok(System::security());
            }
            Ok(Default::default())
        }

        #[native]
        #[java_native(name = "identityHashCode", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn identityHashCode(arg0: Object) -> Result<i32> {
            panic!("native: java/lang/System.identityHashCode:(Ljava/lang/Object;)I")
        }

        #[java_method(name = "getProperties", descriptor = "()Ljava/util/Properties;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getProperties() -> Result<Properties> {
            panic!("stub: java/lang/System.getProperties:()Ljava/util/Properties;")
        }

        #[java_method(name = "lineSeparator", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lineSeparator() -> Result<String> {
            Ok(System::lineSeparator_field())
        }

        #[java_method(name = "setProperties", descriptor = "(Ljava/util/Properties;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setProperties(props: Properties) -> Result<()> {
            panic!("stub: java/lang/System.setProperties:(Ljava/util/Properties;)V")
        }

        #[java_method(name = "getProperty", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getProperty_str(key: String) -> Result<String> {
            panic!("stub: java/lang/System.getProperty:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "getProperty", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getProperty_str_str(key: String, def: String) -> Result<String> {
            panic!("stub: java/lang/System.getProperty:(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "setProperty", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setProperty(key: String, value: String) -> Result<String> {
            panic!("stub: java/lang/System.setProperty:(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "clearProperty", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clearProperty(key: String) -> Result<String> {
            panic!("stub: java/lang/System.clearProperty:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "checkKey", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkKey(key: String) -> Result<()> {
            panic!("stub: java/lang/System.checkKey:(Ljava/lang/String;)V")
        }

        #[java_method(name = "getenv", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getenv_str(name: String) -> Result<String> {
            panic!("stub: java/lang/System.getenv:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "getenv", descriptor = "()Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Map<Ljava/lang/String;Ljava/lang/String;>;")]
        pub fn getenv() -> Result<Object> {
            panic!("stub: java/lang/System.getenv:()Ljava/util/Map;")
        }

        #[java_method(name = "getLogger", descriptor = "(Ljava/lang/String;)Ljava/lang/System$Logger;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLogger_str(name: String) -> Result<Object> {
            panic!("stub: java/lang/System.getLogger:(Ljava/lang/String;)Ljava/lang/System$Logger;")
        }

        #[java_method(name = "getLogger", descriptor = "(Ljava/lang/String;Ljava/util/ResourceBundle;)Ljava/lang/System$Logger;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLogger_str_resour(name: String, bundle: Object) -> Result<Object> {
            panic!("stub: java/lang/System.getLogger:(Ljava/lang/String;Ljava/util/ResourceBundle;)Ljava/lang/System$Logger;")
        }

        #[java_method(name = "exit", descriptor = "(I)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn exit(status: i32) -> Result<()> {
            panic!("stub: java/lang/System.exit:(I)V")
        }

        #[java_method(name = "gc", descriptor = "()V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn gc() -> Result<()> {
            panic!("stub: java/lang/System.gc:()V")
        }

        #[java_method(name = "runFinalization", descriptor = "()V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn runFinalization() -> Result<()> {
            panic!("stub: java/lang/System.runFinalization:()V")
        }

        #[java_method(name = "load", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn load(filename: String) -> Result<()> {
            panic!("stub: java/lang/System.load:(Ljava/lang/String;)V")
        }

        #[java_method(name = "loadLibrary", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn loadLibrary(libname: String) -> Result<()> {
            panic!("stub: java/lang/System.loadLibrary:(Ljava/lang/String;)V")
        }

        #[native]
        #[java_native(name = "mapLibraryName", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn mapLibraryName(arg0: String) -> Result<String> {
            panic!("native: java/lang/System.mapLibraryName:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "newPrintStream", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;)Ljava/io/PrintStream;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newPrintStream(out: OutputStream, enc: String) -> Result<PrintStream> {
            panic!("stub: java/lang/System.newPrintStream:(Ljava/io/OutputStream;Ljava/lang/String;)Ljava/io/PrintStream;")
        }

        #[java_method(name = "logInitException", descriptor = "(ZZLjava/lang/String;Ljava/lang/Throwable;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn logInitException(printToStderr: bool, printStackTrace: bool, msg: String, e: Throwable) -> Result<()> {
            panic!("stub: java/lang/System.logInitException:(ZZLjava/lang/String;Ljava/lang/Throwable;)V")
        }

        #[java_method(name = "createProperties", descriptor = "(Ljava/util/Map;)Ljava/util/Properties;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/lang/String;Ljava/lang/String;>;)Ljava/util/Properties;")]
        pub fn createProperties(initialProps: Object) -> Result<Properties> {
            panic!("stub: java/lang/System.createProperties:(Ljava/util/Map;)Ljava/util/Properties;")
        }

        #[java_method(name = "initPhase1", descriptor = "()V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initPhase1() -> Result<()> {
            panic!("stub: java/lang/System.initPhase1:()V")
        }

        #[java_method(name = "initPhase2", descriptor = "(ZZ)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initPhase2(printToStderr: bool, printStackTrace: bool) -> Result<i32> {
            panic!("stub: java/lang/System.initPhase2:(ZZ)I")
        }

        #[java_method(name = "initPhase3", descriptor = "()V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initPhase3() -> Result<()> {
            panic!("stub: java/lang/System.initPhase3:()V")
        }

        #[java_method(name = "setJavaLangAccess", descriptor = "()V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setJavaLangAccess() -> Result<()> {
            panic!("stub: java/lang/System.setJavaLangAccess:()V")
        }
    }
}
