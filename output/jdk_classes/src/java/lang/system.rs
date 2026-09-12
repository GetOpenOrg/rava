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
    // java: registerNatives()V
    pub fn registerNatives() -> Result<()> {
        todo!("native java/lang/System.registerNatives")
    }

    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: allowSecurityManager()Z
    pub fn allowSecurityManager() -> Result<bool> {
        Ok(System::allowSecurityManager() != 1i32)
    }

    // java: setIn(Ljava/io/InputStream;)V
    pub fn setIn(in_: Object) -> Result<()> {
        System::checkIO()?;
        System::setIn0(in_)?;
        Ok(())
    }

    // java: setOut(Ljava/io/PrintStream;)V
    pub fn setOut(out: Object) -> Result<()> {
        System::checkIO()?;
        System::setOut0(out)?;
        Ok(())
    }

    // java: setErr(Ljava/io/PrintStream;)V
    pub fn setErr(err: Object) -> Result<()> {
        System::checkIO()?;
        System::setErr0(err)?;
        Ok(())
    }

    // java: console()Ljava/io/Console;
    pub fn console() -> Result<Object> {
        let mut c: Object = System::cons();
        let mut local_1: i32 = 8i32;
        /* TODO: monitorenter  */
        c = System::cons();
        let _t0: Object = SharedSecrets::getJavaIOAccess()?;
        let _t1 = _t0.console()?;
        c = _t1;
        System::cons(_t1);
        /* TODO: monitorexit  */
        let mut local_2: i32 = local_1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(c)
    }

    // java: inheritedChannel()Ljava/nio/channels/Channel;
    pub fn inheritedChannel() -> Result<Object> {
        let _t0: Object = SelectorProvider::provider()?;
        let _t1 = _t0.inheritedChannel()?;
        Ok(_t1)
    }

    // java: checkIO()V
    pub fn checkIO() -> Result<()> {
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(RuntimePermission::new(String::from("setIO"))?)?;
        Ok(())
    }

    // java: setIn0(Ljava/io/InputStream;)V
    pub fn setIn0(arg0: Object) -> Result<()> {
        todo!("native java/lang/System.setIn0")
    }

    // java: setOut0(Ljava/io/PrintStream;)V
    pub fn setOut0(arg0: Object) -> Result<()> {
        todo!("native java/lang/System.setOut0")
    }

    // java: setErr0(Ljava/io/PrintStream;)V
    pub fn setErr0(arg0: Object) -> Result<()> {
        todo!("native java/lang/System.setErr0")
    }

    // java: codeSource(Ljava/lang/Class;)Ljava/net/URL;
    pub fn codeSource(clazz: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(clazz)?;
        /* TODO: invokedynamic 76 */
        let mut pa: Object = clazz;
        let _t1: Object = AccessController::doPrivileged(pa)?;
        let _t2 = _t1.getCodeSource()?;
        let mut cs: Object = _t2;
        let _t3 = cs.getLocation()?;
        /* TODO: aconst_null  */
        Ok(_t3)
    }

    // java: setSecurityManager(Ljava/lang/SecurityManager;)V
    pub fn setSecurityManager(sm: Object) -> Result<()> {
        let _t0: bool = System::allowSecurityManager()?;
        let _t1: Object = Reflection::getCallerClass()?;
        let mut callerClass: Object = _t1;
        let _t2 = System_CallersHolder::callers().putIfAbsent(callerClass, 1i32)?;
        let _t3: Object = System::codeSource(callerClass)?;
        let mut url: Object = _t3;
        let _t4 = callerClass.getName()?;
        let mut source: String = _t4;
        let _t5 = callerClass.getName()?;
        String::new().append(&_t5)?;
        String::new().append(&String::from("("))?;
        String::new().append(&url)?;
        String::new().append(&String::from(")"))?;
        source = String::new();
        let mut _arr6: Vec<Object> = Vec::with_capacity(2i32 as usize);
        _arr6[0i32 as usize] = source;
        let _t7 = callerClass.getName()?;
        _arr6[1i32 as usize] = _t7;
        let _t8 = System::initialErrStream().printf(String::from("WARNING: A terminally deprecated method in java.lang.System has been called\nWARNING: System::setSecurityManager has been called by %s\nWARNING: Please consider reporting this to the maintainers of %s\nWARNING: System::setSecurityManager will be removed in a future release"), _arr6)?;
        System::implSetSecurityManager(sm)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: implSetSecurityManager(Ljava/lang/SecurityManager;)V
    pub fn implSetSecurityManager(sm: Object) -> Result<()> {
        let _t0 = 2i32.getResource(String::from("java/lang/ANY"))?;
        let _t1: Object = DefaultFileSystemProvider::theFileSystem()?;
        sm.checkPackageAccess(String::from("java.lang"))?;
        let mut local_1: Object = sm;
        System::setSecurityManager0(sm)?;
        Ok(())
    }

    // java: setSecurityManager0(Ljava/lang/SecurityManager;)V
    pub fn setSecurityManager0(s: Object) -> Result<()> {
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(RuntimePermission::new(String::from("setSecurityManager"))?)?;
        let _t1 = s.getClass()?;
        let _t2 = _t1.getClassLoader()?;
        let _t3: Object = AccessController::doPrivileged(System_1::new(s)?)?;
        System::security(s);
        Ok(())
    }

    // java: getSecurityManager()Ljava/lang/SecurityManager;
    pub fn getSecurityManager() -> Result<Object> {
        let _t0: bool = System::allowSecurityManager()?;
        return Ok(System::security());
        /* TODO: aconst_null  */
        Ok(_t0)
    }

    // java: currentTimeMillis()J
    pub fn currentTimeMillis() -> Result<i64> {
        todo!("native java/lang/System.currentTimeMillis")
    }

    // java: nanoTime()J
    pub fn nanoTime() -> Result<i64> {
        todo!("native java/lang/System.nanoTime")
    }

    // java: arraycopy(Ljava/lang/Object;ILjava/lang/Object;II)V
    pub fn arraycopy(arg0: Object, arg1: i32, arg2: Object, arg3: i32, arg4: i32) -> Result<()> {
        todo!("native java/lang/System.arraycopy")
    }

    // java: identityHashCode(Ljava/lang/Object;)I
    pub fn identityHashCode(arg0: Object) -> Result<i32> {
        todo!("native java/lang/System.identityHashCode")
    }

    // java: getProperties()Ljava/util/Properties;
    pub fn getProperties() -> Result<Object> {
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPropertiesAccess()?;
        Ok(System::props())
    }

    // java: lineSeparator()Ljava/lang/String;
    pub fn lineSeparator() -> Result<String> {
        Ok(System::lineSeparator())
    }

    // java: setProperties(Ljava/util/Properties;)V
    pub fn setProperties(props: Object) -> Result<()> {
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPropertiesAccess()?;
        let _t1: Object = SystemProps::initProperties()?;
        let mut tempProps: Object = _t1;
        VersionProps::init(tempProps)?;
        let _t2: Object = System::createProperties(tempProps)?;
        props = _t2;
        System::props(props);
        Ok(())
    }

    // java: getProperty(Ljava/lang/String;)Ljava/lang/String;
    // java: getProperty(Ljava/lang/String;)Ljava/lang/String;
    pub fn getProperty__str(key: String) -> Result<String> {
        System::checkKey(key)?;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPropertyAccess(key)?;
        let _t1 = System::props().getProperty(key)?;
        Ok(_t1)
    }

    // java: getProperty(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
    // java: getProperty(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
    pub fn getProperty__str_str(key: String, def: String) -> Result<String> {
        System::checkKey(key)?;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPropertyAccess(key)?;
        let _t1 = System::props().getProperty(key, def)?;
        Ok(_t1)
    }

    // java: setProperty(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
    pub fn setProperty(key: String, value: String) -> Result<String> {
        System::checkKey(key)?;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(PropertyPermission::new(key, String::from("write"))?)?;
        let _t1 = System::props().setProperty(key, value)?;
        Ok(_t1)
    }

    // java: clearProperty(Ljava/lang/String;)Ljava/lang/String;
    pub fn clearProperty(key: String) -> Result<String> {
        System::checkKey(key)?;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(PropertyPermission::new(key, String::from("write"))?)?;
        let _t1 = System::props().remove(key)?;
        Ok(_t1)
    }

    // java: checkKey(Ljava/lang/String;)V
    pub fn checkKey(key: String) -> Result<()> {
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = key.isEmpty()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: getenv(Ljava/lang/String;)Ljava/lang/String;
    // java: getenv(Ljava/lang/String;)Ljava/lang/String;
    pub fn getenv__str(name: String) -> Result<String> {
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        String::new().append(&String::from("getenv."))?;
        String::new().append(&name)?;
        sm.checkPermission(RuntimePermission::new(String::new())?)?;
        let _t1: String = ProcessEnvironment::getenv(name)?;
        Ok(_t1)
    }

    // java: getenv()Ljava/util/Map;
    // java: getenv()Ljava/util/Map;
    pub fn getenv() -> Result<Object> {
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(RuntimePermission::new(String::from("getenv.*"))?)?;
        let _t1: Object = ProcessEnvironment::getenv()?;
        Ok(_t1)
    }

    // java: getLogger(Ljava/lang/String;)Ljava/lang/System$Logger;
    // java: getLogger(Ljava/lang/String;)Ljava/lang/System$Logger;
    pub fn getLogger__str(name: String) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(name)?;
        let _t1: Object = Reflection::getCallerClass()?;
        let mut caller: Object = _t1;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t2 = caller.getModule()?;
        let _t3: Object = LazyLoggers::getLogger(name, _t2)?;
        Ok(_t3)
    }

    // java: getLogger(Ljava/lang/String;Ljava/util/ResourceBundle;)Ljava/lang/System$Logger;
    // java: getLogger(Ljava/lang/String;Ljava/util/ResourceBundle;)Ljava/lang/System$Logger;
    pub fn getLogger__str_resour(name: String, bundle: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(bundle)?;
        let mut rb: Object = _t0;
        let _t1: Object = Objects::requireNonNull(name)?;
        let _t2: Object = Reflection::getCallerClass()?;
        let mut caller: Object = _t2;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t3: Object = System::getSecurityManager()?;
        let mut sm: Object = _t3;
        /* TODO: invokedynamic 313 */
        let mut pa: Object = caller;
        /* TODO: aconst_null  */
        let mut _arr4: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr4[0i32 as usize] = System_LoggerFinder::LOGGERFINDER_PERMISSION();
        let _t5: Object = AccessController::doPrivileged(rb, pa, &_arr4)?;
        return Ok(_t5);
        let _t6: Object = System_LoggerFinder::accessProvider()?;
        let _t7 = caller.getModule()?;
        let _t8 = _t6.getLocalizedLogger(name, rb, _t7)?;
        Ok(_t8)
    }

    // java: exit(I)V
    pub fn exit(status: i32) -> Result<()> {
        let _t0: Object = Runtime::getRuntime()?;
        _t0.exit(status)?;
        Ok(())
    }

    // java: gc()V
    pub fn gc() -> Result<()> {
        let _t0: Object = Runtime::getRuntime()?;
        _t0.gc()?;
        Ok(())
    }

    // java: runFinalization()V
    pub fn runFinalization() -> Result<()> {
        let _t0: Object = Runtime::getRuntime()?;
        _t0.runFinalization()?;
        Ok(())
    }

    // java: load(Ljava/lang/String;)V
    pub fn load(filename: String) -> Result<()> {
        let _t0: Object = Runtime::getRuntime()?;
        let _t1: Object = Reflection::getCallerClass()?;
        _t0.load0(_t1, filename)?;
        Ok(())
    }

    // java: loadLibrary(Ljava/lang/String;)V
    pub fn loadLibrary(libname: String) -> Result<()> {
        let _t0: Object = Runtime::getRuntime()?;
        let _t1: Object = Reflection::getCallerClass()?;
        _t0.loadLibrary0(_t1, libname)?;
        Ok(())
    }

    // java: mapLibraryName(Ljava/lang/String;)Ljava/lang/String;
    pub fn mapLibraryName(arg0: String) -> Result<String> {
        todo!("native java/lang/System.mapLibraryName")
    }

    // java: newPrintStream(Ljava/io/OutputStream;Ljava/lang/String;)Ljava/io/PrintStream;
    pub fn newPrintStream(out: Object, enc: String) -> Result<Object> {
        let _t0: Object = Charset::forName(enc, UTF_8::INSTANCE())?;
        return Ok(PrintStream::new(BufferedOutputStream::new(out, 128i32)?, 1i32, _t0)?);
        Ok(PrintStream::new(BufferedOutputStream::new(out, 128i32)?, 1i32)?)
    }

    // java: logInitException(ZZLjava/lang/String;Ljava/lang/Throwable;)V
    pub fn logInitException(printToStderr: bool, printStackTrace: bool, msg: String, e: Object) -> Result<()> {
        let _t0: i32 = VM::initLevel()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut log: Object = System::out();
        log.println(msg)?;
        e.printStackTrace(log)?;
        log.println(e)?;
        let _t1 = e.getSuppressed()?;
        let mut cause: Vec<Object> = _t1;
        let mut local_6: i32 = (cause.len() as i32);
        let mut local_7: i32 = 0i32;
        loop {
            if local_7 >= local_6 { break; }
            let mut suppressed: Object = cause[local_7 as usize].clone();
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

    // java: createProperties(Ljava/util/Map;)Ljava/util/Properties;
    pub fn createProperties(initialProps: Object) -> Result<Object> {
        let _t0 = initialProps.size()?;
        let mut properties: Properties = Properties::new(_t0)?;
        let _t1 = initialProps.entrySet()?;
        let _t2 = _t1.iterator()?;
        let mut local_2: Object = _t2;
        loop {
            let _t0 = local_2.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_2.next()?;
            let mut entry: Object = _t0;
            let _t1 = entry.getKey()?;
            let mut prop: Object = _t1;
            let mut local_5: Object = prop;
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

    // java: initPhase1()V
    pub fn initPhase1() -> Result<()> {
        System::setJavaLangAccess()?;
        let _t0: Object = SystemProps::initProperties()?;
        let mut tempProps: Object = _t0;
        VersionProps::init(tempProps)?;
        VM::saveProperties(tempProps)?;
        let _t1: Object = System::createProperties(tempProps)?;
        System::props(_t1);
        let _t2 = System::props().getProperty(String::from("sun.jnu.encoding"))?;
        let mut jnuEncoding: String = _t2;
        let _t3: bool = Charset::isSupported(jnuEncoding)?;
        System::notSupportedJnuEncoding(jnuEncoding);
        let _t4 = System::props().setProperty(String::from("sun.jnu.encoding"), String::from("UTF-8"))?;
        let _t5: String = StaticProperty::javaHome()?;
        let _t6 = System::props().getProperty(String::from("line.separator"))?;
        System::lineSeparator(_t6);
        let mut fdIn: FileInputStream = FileInputStream::new(FileDescriptor::in_())?;
        let mut fdOut: FileOutputStream = FileOutputStream::new(FileDescriptor::out())?;
        let mut fdErr: FileOutputStream = FileOutputStream::new(FileDescriptor::err())?;
        System::initialIn(BufferedInputStream::new(fdIn)?);
        System::setIn0(System::initialIn())?;
        let _t7 = System::props().getProperty(String::from("stdout.encoding"))?;
        let _t8: Object = System::newPrintStream(fdOut, _t7)?;
        System::setOut0(_t8)?;
        let _t9 = System::props().getProperty(String::from("stderr.encoding"))?;
        let _t10: Object = System::newPrintStream(fdErr, _t9)?;
        System::setErr0(_t10)?;
        Terminator::setup()?;
        VM::initializeOSEnvironment()?;
        let _t11: Object = SharedSecrets::getJavaLangRefAccess()?;
        _t11.startThreads()?;
        VM::initLevel(1i32)?;
        Ok(())
    }

    // java: initPhase2(ZZ)I
    pub fn initPhase2(printToStderr: bool, printStackTrace: bool) -> Result<i32> {
        let _t0: Object = ModuleBootstrap::boot()?;
        System::bootLayer(_t0);
        let mut e: i32 = todo!("stack underflow");
        System::logInitException(printToStderr, printStackTrace, String::from("Error occurred during initialization of boot layer"), e)?;
        return Ok(-1i32);
        VM::initLevel(2i32)?;
        Ok(0i32)
    }

    // java: initPhase3()V
    pub fn initPhase3() -> Result<()> {
        let _t0: Object = Unsafe::getUnsafe()?;
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
        let _t8: Object = ClassLoader::getBuiltinAppClassLoader()?;
        let mut cl: Object = _t8;
        let _t9: Object = Class::forName(smProp, 0i32, cl)?;
        let mut c: Object = _t9;
        let mut _arr10: Vec<Object> = Vec::with_capacity(0i32 as usize);
        let _t11 = c.getConstructor(_arr10)?;
        let mut ctor: Object = _t11;
        let _t12 = 65i32.isAssignableFrom(c)?;
        let _t13 = c.getModifiers()?;
        let _t14: bool = Modifier::isPublic(_t13)?;
        let _t15 = ctor.getModifiers()?;
        let _t16: bool = Modifier::isPublic(_t15)?;
        String::new().append(&String::from("Could not create SecurityManager:"))?;
        let _t17 = ctor.toString()?;
        String::new().append(&_t17)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        ctor.setAccessible(1i32)?;
        let mut _arr18: Vec<Object> = Vec::with_capacity(0i32 as usize);
        let _t19 = ctor.newInstance(_arr18)?;
        let mut sm: Object = _t19;
        System::implSetSecurityManager(sm)?;
        needWarning = 1i32;
        cl = _t16;
        return Err(JvmError::Custom("athrow".to_owned()));
        System::allowSecurityManager(2i32);
        System::allowSecurityManager(1i32);
        System::err().println(String::from("WARNING: A command line option has enabled the Security Manager\nWARNING: The Security Manager is deprecated and will be removed in a future release"))?;
        String::new().append(&String::from("WARNING: The encoding of the underlying platform's file system is not supported:"))?;
        String::new().append(&System::notSupportedJnuEncoding())?;
        System::err().println(String::new())?;
        System::initialErrStream(System::err());
        VM::initLevel(3i32)?;
        let _t20: Object = ClassLoader::initSystemClassLoader()?;
        scl = _t20;
        let _t21: Object = Thread::currentThread()?;
        _t21.setContextClassLoader(scl)?;
        VM::initLevel(4i32)?;
        Ok(())
    }

    // java: setJavaLangAccess()V
    pub fn setJavaLangAccess() -> Result<()> {
        SharedSecrets::setJavaLangAccess(System_2::new()?)?;
        Ok(())
    }
}
