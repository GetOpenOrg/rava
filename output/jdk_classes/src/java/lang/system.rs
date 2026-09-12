#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/System",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "System.java",
))]
pub struct System;

impl System {
    #[cfg_attr(any(), java_native(name = "registerNatives", descriptor = "()V", access = "private static native"))]
    pub fn registerNatives() -> Result<()> {
        todo!("native java/lang/System.registerNatives")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "allowSecurityManager", descriptor = "()Z", access = "private static"))]
    pub fn allowSecurityManager() -> Result<bool> {
        Ok(System::allowSecurityManager() != 1i32)
    }

    #[cfg_attr(any(), java_method(name = "setIn", descriptor = "(Ljava/io/InputStream;)V", access = "public static"))]
    pub fn setIn(in_: JvmObject) -> Result<()> {
        System::checkIO()?;
        System::setIn0(in_)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "setOut", descriptor = "(Ljava/io/PrintStream;)V", access = "public static"))]
    pub fn setOut(out: JvmObject) -> Result<()> {
        System::checkIO()?;
        System::setOut0(out)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "setErr", descriptor = "(Ljava/io/PrintStream;)V", access = "public static"))]
    pub fn setErr(err: JvmObject) -> Result<()> {
        System::checkIO()?;
        System::setErr0(err)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "console", descriptor = "()Ljava/io/Console;", access = "public static"))]
    pub fn console() -> Result<JvmObject> {
        let mut c: JvmObject = System::cons();
        let mut local_1: i32 = 8i32;
        /* TODO: monitorenter  */
        c = System::cons();
        let _t0: JvmObject = SharedSecrets::getJavaIOAccess()?;
        let _t1 = _t0.console()?;
        c = _t1;
        System::cons(_t1);
        /* TODO: monitorexit  */
        let mut local_2: i32 = local_1;
        /* TODO: monitorexit  */
        panic!("{}", /* local_2 */);
        Ok(c)
    }

    #[cfg_attr(any(), java_method(name = "inheritedChannel", descriptor = "()Ljava/nio/channels/Channel;", access = "public static"))]
    pub fn inheritedChannel() -> Result<JvmObject> {
        let _t0: JvmObject = SelectorProvider::provider()?;
        let _t1 = _t0.inheritedChannel()?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "checkIO", descriptor = "()V", access = "private static"))]
    pub fn checkIO() -> Result<()> {
        let _t0: JvmObject = System::getSecurityManager()?;
        let mut sm: JvmObject = _t0;
        sm.checkPermission(RuntimePermission::new(String::from("setIO"))?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_native(name = "setIn0", descriptor = "(Ljava/io/InputStream;)V", access = "private static native"))]
    pub fn setIn0(arg0: JvmObject) -> Result<()> {
        todo!("native java/lang/System.setIn0")
    }

    #[cfg_attr(any(), java_native(name = "setOut0", descriptor = "(Ljava/io/PrintStream;)V", access = "private static native"))]
    pub fn setOut0(arg0: JvmObject) -> Result<()> {
        todo!("native java/lang/System.setOut0")
    }

    #[cfg_attr(any(), java_native(name = "setErr0", descriptor = "(Ljava/io/PrintStream;)V", access = "private static native"))]
    pub fn setErr0(arg0: JvmObject) -> Result<()> {
        todo!("native java/lang/System.setErr0")
    }

    #[cfg_attr(any(), java_method(name = "codeSource", descriptor = "(Ljava/lang/Class;)Ljava/net/URL;", access = "private static"))]
    pub fn codeSource(clazz: JvmObject) -> Result<JvmObject> {
        let _t0: JvmObject = Objects::requireNonNull(clazz)?;
        /* TODO: invokedynamic 76 */
        let mut pa: JvmObject = clazz;
        let _t1: JvmObject = AccessController::doPrivileged(pa)?;
        let _t2 = _t1.getCodeSource()?;
        let mut cs: JvmObject = _t2;
        let _t3 = cs.getLocation()?;
        /* TODO: aconst_null  */
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "setSecurityManager", descriptor = "(Ljava/lang/SecurityManager;)V", access = "public static"))]
    pub fn setSecurityManager(sm: JvmObject) -> Result<()> {
        let _t0: bool = System::allowSecurityManager()?;
        let _t1: JvmObject = Reflection::getCallerClass()?;
        let mut callerClass: JvmObject = _t1;
        let _t2 = System$CallersHolder::callers().putIfAbsent(callerClass, 1i32)?;
        let _t3: JvmObject = System::codeSource(callerClass)?;
        let mut url: JvmObject = _t3;
        let _t4 = callerClass.getName()?;
        let mut source: String = _t4;
        let _t5 = callerClass.getName()?;
        String::new().append(&_t5)?;
        String::new().append(&String::from("("))?;
        String::new().append(&url)?;
        String::new().append(&String::from(")"))?;
        source = String::new();
        let mut _arr6: Vec<JvmObject> = Vec::with_capacity(2i32 as usize);
        _arr6[0i32 as usize] = source;
        let _t7 = callerClass.getName()?;
        _arr6[1i32 as usize] = _t7;
        let _t8 = System::initialErrStream().printf(String::from("WARNING: A terminally deprecated method in java.lang.System has been called
    WARNING: System::setSecurityManager has been called by %s
    WARNING: Please consider reporting this to the maintainers of %s
    WARNING: System::setSecurityManager will be removed in a future release"), _arr6)?;
        System::implSetSecurityManager(sm)?;
        panic!("{}", /* UnsupportedOperationException::new(String::from("The Security Manager is deprecated and will be removed in a future release"))? */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implSetSecurityManager", descriptor = "(Ljava/lang/SecurityManager;)V", access = "private static"))]
    pub fn implSetSecurityManager(sm: JvmObject) -> Result<()> {
        let _t0 = 2i32.getResource(String::from("java/lang/ANY"))?;
        let _t1: JvmObject = DefaultFileSystemProvider::theFileSystem()?;
        sm.checkPackageAccess(String::from("java.lang"))?;
        let mut local_1: JvmObject = sm;
        System::setSecurityManager0(sm)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "setSecurityManager0", descriptor = "(Ljava/lang/SecurityManager;)V", access = "private static"))]
    pub fn setSecurityManager0(s: JvmObject) -> Result<()> {
        let _t0: JvmObject = System::getSecurityManager()?;
        let mut sm: JvmObject = _t0;
        sm.checkPermission(RuntimePermission::new(String::from("setSecurityManager"))?)?;
        let _t1 = s.getClass()?;
        let _t2 = _t1.getClassLoader()?;
        let _t3: JvmObject = AccessController::doPrivileged(System$1::new(s)?)?;
        System::security(s);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getSecurityManager", descriptor = "()Ljava/lang/SecurityManager;", access = "public static"))]
    pub fn getSecurityManager() -> Result<JvmObject> {
        let _t0: bool = System::allowSecurityManager()?;
        return Ok(System::security());
        /* TODO: aconst_null  */
        Ok(_t0)
    }

    #[cfg_attr(any(), java_native(name = "currentTimeMillis", descriptor = "()J", access = "public static native"))]
    pub fn currentTimeMillis() -> Result<i64> {
        todo!("native java/lang/System.currentTimeMillis")
    }

    #[cfg_attr(any(), java_native(name = "nanoTime", descriptor = "()J", access = "public static native"))]
    pub fn nanoTime() -> Result<i64> {
        todo!("native java/lang/System.nanoTime")
    }

    #[cfg_attr(any(), java_native(name = "arraycopy", descriptor = "(Ljava/lang/Object;ILjava/lang/Object;II)V", access = "public static native"))]
    pub fn arraycopy(arg0: JvmObject, arg1: i32, arg2: JvmObject, arg3: i32, arg4: i32) -> Result<()> {
        todo!("native java/lang/System.arraycopy")
    }

    #[cfg_attr(any(), java_native(name = "identityHashCode", descriptor = "(Ljava/lang/Object;)I", access = "public static native"))]
    pub fn identityHashCode(arg0: JvmObject) -> Result<i32> {
        todo!("native java/lang/System.identityHashCode")
    }

    #[cfg_attr(any(), java_method(name = "getProperties", descriptor = "()Ljava/util/Properties;", access = "public static"))]
    pub fn getProperties() -> Result<JvmObject> {
        let _t0: JvmObject = System::getSecurityManager()?;
        let mut sm: JvmObject = _t0;
        sm.checkPropertiesAccess()?;
        Ok(System::props())
    }

    #[cfg_attr(any(), java_method(name = "lineSeparator", descriptor = "()Ljava/lang/String;", access = "public static"))]
    pub fn lineSeparator() -> Result<String> {
        Ok(System::lineSeparator())
    }

    #[cfg_attr(any(), java_method(name = "setProperties", descriptor = "(Ljava/util/Properties;)V", access = "public static"))]
    pub fn setProperties(props: JvmObject) -> Result<()> {
        let _t0: JvmObject = System::getSecurityManager()?;
        let mut sm: JvmObject = _t0;
        sm.checkPropertiesAccess()?;
        let _t1: JvmObject = SystemProps::initProperties()?;
        let mut tempProps: JvmObject = _t1;
        VersionProps::init(tempProps)?;
        let _t2: JvmObject = System::createProperties(tempProps)?;
        props = _t2;
        System::props(props);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getProperty", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public static"))]
    // java: getProperty(Ljava/lang/String;)Ljava/lang/String;
    pub fn getProperty__str(key: String) -> Result<String> {
        System::checkKey(key)?;
        let _t0: JvmObject = System::getSecurityManager()?;
        let mut sm: JvmObject = _t0;
        sm.checkPropertyAccess(key)?;
        let _t1 = System::props().getProperty(key)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getProperty", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "public static"))]
    // java: getProperty(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
    pub fn getProperty__str_str(key: String, def: String) -> Result<String> {
        System::checkKey(key)?;
        let _t0: JvmObject = System::getSecurityManager()?;
        let mut sm: JvmObject = _t0;
        sm.checkPropertyAccess(key)?;
        let _t1 = System::props().getProperty(key, def)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "setProperty", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "public static"))]
    pub fn setProperty(key: String, value: String) -> Result<String> {
        System::checkKey(key)?;
        let _t0: JvmObject = System::getSecurityManager()?;
        let mut sm: JvmObject = _t0;
        sm.checkPermission(PropertyPermission::new(key, String::from("write"))?)?;
        let _t1 = System::props().setProperty(key, value)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "clearProperty", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public static"))]
    pub fn clearProperty(key: String) -> Result<String> {
        System::checkKey(key)?;
        let _t0: JvmObject = System::getSecurityManager()?;
        let mut sm: JvmObject = _t0;
        sm.checkPermission(PropertyPermission::new(key, String::from("write"))?)?;
        let _t1 = System::props().remove(key)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "checkKey", descriptor = "(Ljava/lang/String;)V", access = "private static"))]
    pub fn checkKey(key: String) -> Result<()> {
        panic!("{}", /* NullPointerException::new(String::from("key can't be null"))? */);
        let _t0 = key.isEmpty()?;
        panic!("{}", /* IllegalArgumentException::new(String::from("key can't be empty"))? */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getenv", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public static"))]
    // java: getenv(Ljava/lang/String;)Ljava/lang/String;
    pub fn getenv__str(name: String) -> Result<String> {
        let _t0: JvmObject = System::getSecurityManager()?;
        let mut sm: JvmObject = _t0;
        String::new().append(&String::from("getenv."))?;
        String::new().append(&name)?;
        sm.checkPermission(RuntimePermission::new(String::new())?)?;
        let _t1: String = ProcessEnvironment::getenv(name)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getenv", descriptor = "()Ljava/util/Map;", access = "public static"))]
    // java: getenv()Ljava/util/Map;
    pub fn getenv() -> Result<JvmObject> {
        let _t0: JvmObject = System::getSecurityManager()?;
        let mut sm: JvmObject = _t0;
        sm.checkPermission(RuntimePermission::new(String::from("getenv.*"))?)?;
        let _t1: JvmObject = ProcessEnvironment::getenv()?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getLogger", descriptor = "(Ljava/lang/String;)Ljava/lang/System$Logger;", access = "public static"))]
    // java: getLogger(Ljava/lang/String;)Ljava/lang/System$Logger;
    pub fn getLogger__str(name: String) -> Result<JvmObject> {
        let _t0: JvmObject = Objects::requireNonNull(name)?;
        let _t1: JvmObject = Reflection::getCallerClass()?;
        let mut caller: JvmObject = _t1;
        panic!("{}", /* IllegalCallerException::new(String::from("no caller frame"))? */);
        let _t2 = caller.getModule()?;
        let _t3: JvmObject = LazyLoggers::getLogger(name, _t2)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "getLogger", descriptor = "(Ljava/lang/String;Ljava/util/ResourceBundle;)Ljava/lang/System$Logger;", access = "public static"))]
    // java: getLogger(Ljava/lang/String;Ljava/util/ResourceBundle;)Ljava/lang/System$Logger;
    pub fn getLogger__str_resour(name: String, bundle: JvmObject) -> Result<JvmObject> {
        let _t0: JvmObject = Objects::requireNonNull(bundle)?;
        let mut rb: JvmObject = _t0;
        let _t1: JvmObject = Objects::requireNonNull(name)?;
        let _t2: JvmObject = Reflection::getCallerClass()?;
        let mut caller: JvmObject = _t2;
        panic!("{}", /* IllegalCallerException::new(String::from("no caller frame"))? */);
        let _t3: JvmObject = System::getSecurityManager()?;
        let mut sm: JvmObject = _t3;
        /* TODO: invokedynamic 313 */
        let mut pa: JvmObject = caller;
        /* TODO: aconst_null  */
        let mut _arr4: Vec<JvmObject> = Vec::with_capacity(1i32 as usize);
        _arr4[0i32 as usize] = System$LoggerFinder::LOGGERFINDER_PERMISSION();
        let _t5: JvmObject = AccessController::doPrivileged(rb, pa, &_arr4)?;
        return Ok(_t5);
        let _t6: JvmObject = System$LoggerFinder::accessProvider()?;
        let _t7 = caller.getModule()?;
        let _t8 = _t6.getLocalizedLogger(name, rb, _t7)?;
        Ok(_t8)
    }

    #[cfg_attr(any(), java_method(name = "exit", descriptor = "(I)V", access = "public static"))]
    pub fn exit(status: i32) -> Result<()> {
        let _t0: JvmObject = Runtime::getRuntime()?;
        _t0.exit(status)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "gc", descriptor = "()V", access = "public static"))]
    pub fn gc() -> Result<()> {
        let _t0: JvmObject = Runtime::getRuntime()?;
        _t0.gc()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "runFinalization", descriptor = "()V", access = "public static"))]
    pub fn runFinalization() -> Result<()> {
        let _t0: JvmObject = Runtime::getRuntime()?;
        _t0.runFinalization()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "load", descriptor = "(Ljava/lang/String;)V", access = "public static"))]
    pub fn load(filename: String) -> Result<()> {
        let _t0: JvmObject = Runtime::getRuntime()?;
        let _t1: JvmObject = Reflection::getCallerClass()?;
        _t0.load0(_t1, filename)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "loadLibrary", descriptor = "(Ljava/lang/String;)V", access = "public static"))]
    pub fn loadLibrary(libname: String) -> Result<()> {
        let _t0: JvmObject = Runtime::getRuntime()?;
        let _t1: JvmObject = Reflection::getCallerClass()?;
        _t0.loadLibrary0(_t1, libname)?;
        Ok(())
    }

    #[cfg_attr(any(), java_native(name = "mapLibraryName", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public static native"))]
    pub fn mapLibraryName(arg0: String) -> Result<String> {
        todo!("native java/lang/System.mapLibraryName")
    }

    #[cfg_attr(any(), java_method(name = "newPrintStream", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;)Ljava/io/PrintStream;", access = "private static"))]
    pub fn newPrintStream(out: JvmObject, enc: String) -> Result<JvmObject> {
        let _t0: JvmObject = Charset::forName(enc, UTF_8::INSTANCE())?;
        return Ok(PrintStream::new(BufferedOutputStream::new(out, 128i32)?, 1i32, _t0)?);
        Ok(PrintStream::new(BufferedOutputStream::new(out, 128i32)?, 1i32)?)
    }

    #[cfg_attr(any(), java_method(name = "logInitException", descriptor = "(ZZLjava/lang/String;Ljava/lang/Throwable;)V", access = "private static"))]
    pub fn logInitException(printToStderr: bool, printStackTrace: bool, msg: String, e: JvmObject) -> Result<()> {
        let _t0: i32 = VM::initLevel()?;
        panic!("{}", /* InternalError::new(String::from("system classes not initialized"))? */);
        let mut log: JvmObject = System::out();
        log.println(msg)?;
        e.printStackTrace(log)?;
        log.println(e)?;
        let _t1 = e.getSuppressed()?;
        let mut cause: JvmObject = _t1;
        let mut local_6: i32 = (cause.len() as i32);
        let mut local_7: i32 = 0i32;
        loop {
            if local_7 >= local_6 { break; }
            let mut suppressed: JvmObject = cause[local_7 as usize].clone();
            String::new().append(&String::from("Suppressed:"))?;
            String::new().append(&suppressed)?;
            log.println(String::new())?;
            local_7 = local_7.wrapping_add(1i32);
        }
        let _t2 = e.getCause()?;
        cause = _t2;
        String::new().append(&String::from("Caused by:"))?;
        String::new().append(&cause)?;
        log.println(String::new())?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "createProperties", descriptor = "(Ljava/util/Map;)Ljava/util/Properties;", access = "private static"))]
    pub fn createProperties(initialProps: JvmObject) -> Result<JvmObject> {
        let _t0 = initialProps.size()?;
        let mut properties: Properties = Properties::new(_t0)?;
        let _t1 = initialProps.entrySet()?;
        let _t2 = _t1.iterator()?;
        let mut local_2: JvmObject = _t2;
        loop {
            let _t0 = local_2.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_2.next()?;
            let mut entry: JvmObject = _t0;
            let _t1 = entry.getKey()?;
            let mut prop: JvmObject = _t1;
            let mut local_5: JvmObject = prop;
            let mut local_6: i32 = -1i32;
            let _t2 = local_5.hashCode()?;
            /* TODO: lookupswitch default:202 -1594982994:188 -903405997:120 -137219825:171 82382212:137 2006761672:154 */
            let _t3 = local_5.equals(String::from("sun.nio.MaxDirectMemorySize"))?;
            local_6 = 0i32;
            let _t4 = local_5.equals(String::from("sun.nio.PageAlignDirectMemory"))?;
            local_6 = 1i32;
            let _t5 = local_5.equals(String::from("java.lang.Integer.IntegerCache.high"))?;
            local_6 = 2i32;
            let _t6 = local_5.equals(String::from("sun.java.launcher.diag"))?;
            local_6 = 3i32;
            let _t7 = local_5.equals(String::from("jdk.boot.class.path.append"))?;
            local_6 = 4i32;
            /* TODO: tableswitch default:243 low:0 high:4 */
            let _t8 = entry.getValue()?;
            let _t9 = properties.put(prop, _t8)?;
        }
        Ok(properties)
    }

    #[cfg_attr(any(), java_method(name = "initPhase1", descriptor = "()V", access = "private static"))]
    pub fn initPhase1() -> Result<()> {
        System::setJavaLangAccess()?;
        let _t0: JvmObject = SystemProps::initProperties()?;
        let mut tempProps: JvmObject = _t0;
        VersionProps::init(tempProps)?;
        VM::saveProperties(tempProps)?;
        let _t1: JvmObject = System::createProperties(tempProps)?;
        System::props(_t1);
        let _t2 = System::props().getProperty(String::from("sun.jnu.encoding"))?;
        let mut jnuEncoding: String = _t2;
        let _t3: bool = Charset::isSupported(jnuEncoding)?;
        System::notSupportedJnuEncoding(jnuEncoding);
        let _t4 = System::props().setProperty(String::from("sun.jnu.encoding"), String::from("UTF-8"))?;
        let _t5: String = StaticProperty::javaHome()?;
        let _t6 = System::props().getProperty(String::from("line.separator"))?;
        System::lineSeparator(_t6);
        let mut fdIn: FileInputStream = FileInputStream::new(FileDescriptor::in())?;
        let mut fdOut: FileOutputStream = FileOutputStream::new(FileDescriptor::out())?;
        let mut fdErr: FileOutputStream = FileOutputStream::new(FileDescriptor::err())?;
        System::initialIn(BufferedInputStream::new(fdIn)?);
        System::setIn0(System::initialIn())?;
        let _t7 = System::props().getProperty(String::from("stdout.encoding"))?;
        let _t8: JvmObject = System::newPrintStream(fdOut, _t7)?;
        System::setOut0(_t8)?;
        let _t9 = System::props().getProperty(String::from("stderr.encoding"))?;
        let _t10: JvmObject = System::newPrintStream(fdErr, _t9)?;
        System::setErr0(_t10)?;
        Terminator::setup()?;
        VM::initializeOSEnvironment()?;
        let _t11: JvmObject = SharedSecrets::getJavaLangRefAccess()?;
        _t11.startThreads()?;
        VM::initLevel(1i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "initPhase2", descriptor = "(ZZ)I", access = "private static"))]
    pub fn initPhase2(printToStderr: bool, printStackTrace: bool) -> Result<i32> {
        let _t0: JvmObject = ModuleBootstrap::boot()?;
        System::bootLayer(_t0);
        let mut e: i32 = /* UNDERFLOW */;
        System::logInitException(printToStderr, printStackTrace, String::from("Error occurred during initialization of boot layer"), e)?;
        return Ok(-1i32);
        VM::initLevel(2i32)?;
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "initPhase3", descriptor = "()V", access = "private static"))]
    pub fn initPhase3() -> Result<()> {
        let _t0: JvmObject = Unsafe::getUnsafe()?;
        _t0.ensureClassInitialized(578i32)?;
        let _t1: bool = SystemProps::isBadIoTmpdir()?;
        System::err().println(String::from("WARNING: java.io.tmpdir directory does not exist"))?;
        let _t2: String = System::getProperty(String::from("java.security.manager"))?;
        let mut smProp: String = _t2;
        let mut needWarning: i32 = 0i32;
        let mut scl: String = smProp;
        let mut local_3: i32 = -1i32;
        let _t3 = scl.hashCode()?;
        /* TODO: lookupswitch default:145 0:118 92906313:103 271239035:88 1544803905:133 */
        let _t4 = scl.equals(String::from("disallow"))?;
        local_3 = 0i32;
        let _t5 = scl.equals(String::from("allow"))?;
        local_3 = 1i32;
        let _t6 = scl.equals(String::from(""))?;
        local_3 = 2i32;
        let _t7 = scl.equals(String::from("default"))?;
        local_3 = 3i32;
        /* TODO: tableswitch default:209 low:0 high:3 */
        System::allowSecurityManager(1i32);
        System::allowSecurityManager(2i32);
        System::implSetSecurityManager(SecurityManager::new()?)?;
        System::allowSecurityManager(2i32);
        needWarning = 1i32;
        let _t8: JvmObject = ClassLoader::getBuiltinAppClassLoader()?;
        let mut cl: JvmObject = _t8;
        let _t9: JvmObject = Class::forName(smProp, 0i32, cl)?;
        let mut c: JvmObject = _t9;
        let mut _arr10: Vec<JvmObject> = Vec::with_capacity(0i32 as usize);
        let _t11 = c.getConstructor(_arr10)?;
        let mut ctor: JvmObject = _t11;
        let _t12 = 65i32.isAssignableFrom(c)?;
        let _t13 = c.getModifiers()?;
        let _t14: bool = Modifier::isPublic(_t13)?;
        let _t15 = ctor.getModifiers()?;
        let _t16: bool = Modifier::isPublic(_t15)?;
        String::new().append(&String::from("Could not create SecurityManager:"))?;
        let _t17 = ctor.toString()?;
        String::new().append(&_t17)?;
        panic!("{}", /* Error::new(String::new())? */);
        ctor.setAccessible(1i32)?;
        let mut _arr18: Vec<JvmObject> = Vec::with_capacity(0i32 as usize);
        let _t19 = ctor.newInstance(_arr18)?;
        let mut sm: JvmObject = _t19;
        System::implSetSecurityManager(sm)?;
        needWarning = 1i32;
        cl = _t16;
        panic!("{}", /* InternalError::new(String::from("Could not create SecurityManager"), cl)? */);
        System::allowSecurityManager(2i32);
        System::allowSecurityManager(1i32);
        System::err().println(String::from("WARNING: A command line option has enabled the Security Manager
    WARNING: The Security Manager is deprecated and will be removed in a future release"))?;
        String::new().append(&String::from("WARNING: The encoding of the underlying platform's file system is not supported:"))?;
        String::new().append(&System::notSupportedJnuEncoding())?;
        System::err().println(String::new())?;
        System::initialErrStream(System::err());
        VM::initLevel(3i32)?;
        let _t20: JvmObject = ClassLoader::initSystemClassLoader()?;
        scl = _t20;
        let _t21: JvmObject = Thread::currentThread()?;
        _t21.setContextClassLoader(scl)?;
        VM::initLevel(4i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "setJavaLangAccess", descriptor = "()V", access = "private static"))]
    pub fn setJavaLangAccess() -> Result<()> {
        SharedSecrets::setJavaLangAccess(System$2::new()?)?;
        Ok(())
    }
}
