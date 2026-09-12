#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Runtime",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public",
    source      = "Runtime.java",
))]
pub struct Runtime;

impl Runtime {
    #[cfg_attr(any(), java_method(name = "getRuntime", descriptor = "()Ljava/lang/Runtime;", access = "public static"))]
    pub fn getRuntime() -> Result<Object> {
        Ok(Runtime::currentRuntime())
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "exit", descriptor = "(I)V", access = "public"))]
    pub fn exit(&self, status: i32) -> Result<()> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkExit(status)?;
        Shutdown::exit(status)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "addShutdownHook", descriptor = "(Ljava/lang/Thread;)V", access = "public"))]
    pub fn addShutdownHook(&self, hook: Object) -> Result<()> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(RuntimePermission::new(String::from("shutdownHooks"))?)?;
        ApplicationShutdownHooks::add(hook)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "removeShutdownHook", descriptor = "(Ljava/lang/Thread;)Z", access = "public"))]
    pub fn removeShutdownHook(&self, hook: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(RuntimePermission::new(String::from("shutdownHooks"))?)?;
        let _t1: bool = ApplicationShutdownHooks::remove(hook)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "halt", descriptor = "(I)V", access = "public"))]
    pub fn halt(&self, status: i32) -> Result<()> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkExit(status)?;
        Shutdown::beforeHalt()?;
        Shutdown::halt(status)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "exec", descriptor = "(Ljava/lang/String;)Ljava/lang/Process;", access = "public"))]
    // java: exec(Ljava/lang/String;)Ljava/lang/Process;
    pub fn exec__str(&self, command: String) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        let _t0 = todo!("stack underflow").exec(todo!("stack underflow"), this, command)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "exec", descriptor = "(Ljava/lang/String;[Ljava/lang/String;)Ljava/lang/Process;", access = "public"))]
    // java: exec(Ljava/lang/String;[Ljava/lang/String;)Ljava/lang/Process;
    pub fn exec__str_arr_str(&self, command: String, envp: Vec<String>) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        let _t0 = todo!("stack underflow").exec(this, command, envp)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "exec", descriptor = "(Ljava/lang/String;[Ljava/lang/String;Ljava/io/File;)Ljava/lang/Process;", access = "public"))]
    // java: exec(Ljava/lang/String;[Ljava/lang/String;Ljava/io/File;)Ljava/lang/Process;
    pub fn exec__str_arr_str_file(&self, command: String, envp: Vec<String>, dir: Object) -> Result<Object> {
        let this = self;
        let _t0 = command.isEmpty()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut st: StringTokenizer = StringTokenizer::new(command)?;
        let _t1 = st.countTokens()?;
        let mut _arr2: Vec<Object> = Vec::with_capacity(_t1 as usize);
        let mut cmdarray: Vec<Object> = _arr2;
        let mut i: i32 = 0i32;
        loop {
            let _t0 = st.hasMoreTokens()?;
            if _t0==0i32 { break; }
            let _t0 = st.nextToken()?;
            cmdarray[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        let _t3 = this.exec(cmdarray, envp, dir)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "exec", descriptor = "([Ljava/lang/String;)Ljava/lang/Process;", access = "public"))]
    // java: exec([Ljava/lang/String;)Ljava/lang/Process;
    pub fn exec__arr_str(&self, cmdarray: Vec<String>) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        let _t0 = todo!("stack underflow").exec(todo!("stack underflow"), this, cmdarray)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "exec", descriptor = "([Ljava/lang/String;[Ljava/lang/String;)Ljava/lang/Process;", access = "public"))]
    // java: exec([Ljava/lang/String;[Ljava/lang/String;)Ljava/lang/Process;
    pub fn exec__arr_str_arr_str(&self, cmdarray: Vec<String>, envp: Vec<String>) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        let _t0 = todo!("stack underflow").exec(this, cmdarray, envp)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "exec", descriptor = "([Ljava/lang/String;[Ljava/lang/String;Ljava/io/File;)Ljava/lang/Process;", access = "public"))]
    // java: exec([Ljava/lang/String;[Ljava/lang/String;Ljava/io/File;)Ljava/lang/Process;
    pub fn exec__arr_str_arr_str_file(&self, cmdarray: Vec<String>, envp: Vec<String>, dir: Object) -> Result<Object> {
        let this = self;
        let _t0 = ProcessBuilder::new(cmdarray)?.environment(envp)?;
        let _t1 = _t0.directory(dir)?;
        let _t2 = _t1.start()?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_native(name = "availableProcessors", descriptor = "()I", access = "public native"))]
    pub fn availableProcessors(&self) -> Result<i32> {
        todo!("native java/lang/Runtime.availableProcessors")
    }

    #[cfg_attr(any(), java_native(name = "freeMemory", descriptor = "()J", access = "public native"))]
    pub fn freeMemory(&self) -> Result<i64> {
        todo!("native java/lang/Runtime.freeMemory")
    }

    #[cfg_attr(any(), java_native(name = "totalMemory", descriptor = "()J", access = "public native"))]
    pub fn totalMemory(&self) -> Result<i64> {
        todo!("native java/lang/Runtime.totalMemory")
    }

    #[cfg_attr(any(), java_native(name = "maxMemory", descriptor = "()J", access = "public native"))]
    pub fn maxMemory(&self) -> Result<i64> {
        todo!("native java/lang/Runtime.maxMemory")
    }

    #[cfg_attr(any(), java_native(name = "gc", descriptor = "()V", access = "public native"))]
    pub fn gc(&self) -> Result<()> {
        todo!("native java/lang/Runtime.gc")
    }

    #[cfg_attr(any(), java_method(name = "runFinalization", descriptor = "()V", access = "public"))]
    pub fn runFinalization(&self) -> Result<()> {
        let this = self;
        let _t0: Object = SharedSecrets::getJavaLangRefAccess()?;
        _t0.runFinalization()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "load", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn load(&self, filename: String) -> Result<()> {
        let this = self;
        let _t0: Object = Reflection::getCallerClass()?;
        this.load0(_t0, filename)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "load0", descriptor = "(Ljava/lang/Class;Ljava/lang/String;)V"))]
    pub fn load0(&self, fromClass: Object, filename: String) -> Result<()> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkLink(filename)?;
        let mut file: File = File::new(filename)?;
        let _t1 = file.isAbsolute()?;
        String::new().append(&String::from("Expecting an absolute path of the library:"))?;
        String::new().append(&filename)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t2: Object = ClassLoader::loadLibrary(fromClass, file)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "loadLibrary", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn loadLibrary(&self, libname: String) -> Result<()> {
        let this = self;
        let _t0: Object = Reflection::getCallerClass()?;
        this.loadLibrary0(_t0, libname)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "loadLibrary0", descriptor = "(Ljava/lang/Class;Ljava/lang/String;)V"))]
    pub fn loadLibrary0(&self, fromClass: Object, libname: String) -> Result<()> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        security.checkLink(libname)?;
        let _t1 = libname.indexOf(File::separatorChar())?;
        String::new().append(&String::from("Directory separator should not appear in library name:"))?;
        String::new().append(&libname)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t2: Object = ClassLoader::loadLibrary(fromClass, libname)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "version", descriptor = "()Ljava/lang/Runtime$Version;", access = "public static"))]
    pub fn version() -> Result<Object> {
        let mut v: Object = Runtime::version();
        let _t0: Object = VersionProps::versionNumbers()?;
        let _t1: Object = VersionProps::pre()?;
        let _t2: Object = VersionProps::build()?;
        let _t3: Object = VersionProps::optional()?;
        v = Runtime_Version::new(_t0, _t1, _t2, _t3)?;
        Runtime::version(v);
        Ok(v)
    }
}
