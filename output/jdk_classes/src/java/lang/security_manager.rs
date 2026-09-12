#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/SecurityManager",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public",
    source      = "SecurityManager.java",
))]
pub struct SecurityManager {
    #[cfg_attr(any(), java_field(name = "initialized", descriptor = "Z", access = "private"))]
    pub initialized: Field<bool>,
}

impl SecurityManager {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    pub fn new() -> Result<Self> {
        let this = Self { initialized: Field::new(false) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.initialized.set(0i32);
        let mut local_1: i32 = 8i32;
        /* TODO: monitorenter  */
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(RuntimePermission::new(String::from("createSecurityManager"))?)?;
        this.initialized.set(1i32);
        /* TODO: monitorexit  */
        let mut local_3: i32 = local_1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(this)
    }

    #[cfg_attr(any(), java_native(name = "getClassContext", descriptor = "()[Ljava/lang/Class;", access = "protected native"))]
    pub fn getClassContext(&self) -> Result<Vec<Object>> {
        todo!("native java/lang/SecurityManager.getClassContext")
    }

    #[cfg_attr(any(), java_method(name = "getSecurityContext", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn getSecurityContext(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = AccessController::getContext()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "checkPermission", descriptor = "(Ljava/security/Permission;)V", access = "public"))]
    // java: checkPermission(Ljava/security/Permission;)V
    pub fn checkPermission__permis(&self, perm: Object) -> Result<()> {
        let this = self;
        AccessController::checkPermission(perm)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkPermission", descriptor = "(Ljava/security/Permission;Ljava/lang/Object;)V", access = "public"))]
    // java: checkPermission(Ljava/security/Permission;Ljava/lang/Object;)V
    pub fn checkPermission__permis_obj(&self, perm: Object, context: Object) -> Result<()> {
        let this = self;
        context.checkPermission(perm)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkCreateClassLoader", descriptor = "()V", access = "public"))]
    pub fn checkCreateClassLoader(&self) -> Result<()> {
        let this = self;
        this.checkPermission(SecurityConstants::CREATE_CLASSLOADER_PERMISSION())?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getRootGroup", descriptor = "()Ljava/lang/ThreadGroup;", access = "private static"))]
    pub fn getRootGroup() -> Result<Object> {
        let _t0: Object = Thread::currentThread()?;
        let _t1 = _t0.getThreadGroup()?;
        let mut root: Object = _t1;
        loop {
            let _t0 = root.getParent()?;
            if _t0.is_none() { break; }
            let _t0 = root.getParent()?;
            root = _t0;
        }
        Ok(root)
    }

    #[cfg_attr(any(), java_method(name = "checkAccess", descriptor = "(Ljava/lang/Thread;)V", access = "public"))]
    // java: checkAccess(Ljava/lang/Thread;)V
    pub fn checkAccess__thread(&self, t: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0 = t.getThreadGroup()?;
        this.checkPermission(SecurityConstants::MODIFY_THREAD_PERMISSION())?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkAccess", descriptor = "(Ljava/lang/ThreadGroup;)V", access = "public"))]
    // java: checkAccess(Ljava/lang/ThreadGroup;)V
    pub fn checkAccess__thread(&self, g: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        this.checkPermission(SecurityConstants::MODIFY_THREADGROUP_PERMISSION())?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkExit", descriptor = "(I)V", access = "public"))]
    pub fn checkExit(&self, status: i32) -> Result<()> {
        let this = self;
        String::new().append(&String::from("exitVM."))?;
        String::new().append(&status)?;
        this.checkPermission(RuntimePermission::new(String::new())?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkExec", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn checkExec(&self, cmd: String) -> Result<()> {
        let this = self;
        let mut f: File = File::new(cmd)?;
        let _t0 = f.isAbsolute()?;
        this.checkPermission(FilePermission::new(cmd, String::from("execute"))?)?;
        this.checkPermission(FilePermission::new(String::from("<<ALL FILES>>"), String::from("execute"))?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkLink", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn checkLink(&self, lib: String) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        String::new().append(&String::from("loadLibrary."))?;
        String::new().append(&lib)?;
        this.checkPermission(RuntimePermission::new(String::new())?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkRead", descriptor = "(Ljava/io/FileDescriptor;)V", access = "public"))]
    // java: checkRead(Ljava/io/FileDescriptor;)V
    pub fn checkRead__filede(&self, fd: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        this.checkPermission(RuntimePermission::new(String::from("readFileDescriptor"))?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkRead", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    // java: checkRead(Ljava/lang/String;)V
    pub fn checkRead__str(&self, file: String) -> Result<()> {
        let this = self;
        this.checkPermission(FilePermission::new(file, String::from("read"))?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkRead", descriptor = "(Ljava/lang/String;Ljava/lang/Object;)V", access = "public"))]
    // java: checkRead(Ljava/lang/String;Ljava/lang/Object;)V
    pub fn checkRead__str_obj(&self, file: String, context: Object) -> Result<()> {
        let this = self;
        this.checkPermission(FilePermission::new(file, String::from("read"))?, context)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkWrite", descriptor = "(Ljava/io/FileDescriptor;)V", access = "public"))]
    // java: checkWrite(Ljava/io/FileDescriptor;)V
    pub fn checkWrite__filede(&self, fd: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        this.checkPermission(RuntimePermission::new(String::from("writeFileDescriptor"))?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkWrite", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    // java: checkWrite(Ljava/lang/String;)V
    pub fn checkWrite__str(&self, file: String) -> Result<()> {
        let this = self;
        this.checkPermission(FilePermission::new(file, String::from("write"))?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkDelete", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn checkDelete(&self, file: String) -> Result<()> {
        let this = self;
        this.checkPermission(FilePermission::new(file, String::from("delete"))?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkConnect", descriptor = "(Ljava/lang/String;I)V", access = "public"))]
    // java: checkConnect(Ljava/lang/String;I)V
    pub fn checkConnect__str_i(&self, host: String, port: i32) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0 = host.startsWith(String::from("["))?;
        let _t1 = host.indexOf(58i32)?;
        String::new().append(&String::from("["))?;
        String::new().append(&host)?;
        String::new().append(&String::from("]"))?;
        host = String::new();
        this.checkPermission(SocketPermission::new(host, String::from("resolve"))?)?;
        String::new().append(&host)?;
        String::new().append(&String::from(":"))?;
        String::new().append(&port)?;
        this.checkPermission(SocketPermission::new(String::new(), String::from("connect"))?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkConnect", descriptor = "(Ljava/lang/String;ILjava/lang/Object;)V", access = "public"))]
    // java: checkConnect(Ljava/lang/String;ILjava/lang/Object;)V
    pub fn checkConnect__str_i_obj(&self, host: String, port: i32, context: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0 = host.startsWith(String::from("["))?;
        let _t1 = host.indexOf(58i32)?;
        String::new().append(&String::from("["))?;
        String::new().append(&host)?;
        String::new().append(&String::from("]"))?;
        host = String::new();
        this.checkPermission(SocketPermission::new(host, String::from("resolve"))?, context)?;
        String::new().append(&host)?;
        String::new().append(&String::from(":"))?;
        String::new().append(&port)?;
        this.checkPermission(SocketPermission::new(String::new(), String::from("connect"))?, context)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkListen", descriptor = "(I)V", access = "public"))]
    pub fn checkListen(&self, port: i32) -> Result<()> {
        let this = self;
        String::new().append(&String::from("localhost:"))?;
        String::new().append(&port)?;
        this.checkPermission(SocketPermission::new(String::new(), String::from("listen"))?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkAccept", descriptor = "(Ljava/lang/String;I)V", access = "public"))]
    pub fn checkAccept(&self, host: String, port: i32) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0 = host.startsWith(String::from("["))?;
        let _t1 = host.indexOf(58i32)?;
        String::new().append(&String::from("["))?;
        String::new().append(&host)?;
        String::new().append(&String::from("]"))?;
        host = String::new();
        String::new().append(&host)?;
        String::new().append(&String::from(":"))?;
        String::new().append(&port)?;
        this.checkPermission(SocketPermission::new(String::new(), String::from("accept"))?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkMulticast", descriptor = "(Ljava/net/InetAddress;)V", access = "public"))]
    // java: checkMulticast(Ljava/net/InetAddress;)V
    pub fn checkMulticast__inetad(&self, maddr: Object) -> Result<()> {
        let this = self;
        let _t0 = maddr.getHostAddress()?;
        let mut host: String = _t0;
        let _t1 = host.startsWith(String::from("["))?;
        let _t2 = host.indexOf(58i32)?;
        String::new().append(&String::from("["))?;
        String::new().append(&host)?;
        String::new().append(&String::from("]"))?;
        host = String::new();
        this.checkPermission(SocketPermission::new(host, String::from("connect,accept"))?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkMulticast", descriptor = "(Ljava/net/InetAddress;B)V", access = "public"))]
    // java: checkMulticast(Ljava/net/InetAddress;B)V
    pub fn checkMulticast__inetad_b(&self, maddr: Object, ttl: i8) -> Result<()> {
        let this = self;
        let _t0 = maddr.getHostAddress()?;
        let mut host: String = _t0;
        let _t1 = host.startsWith(String::from("["))?;
        let _t2 = host.indexOf(58i32)?;
        String::new().append(&String::from("["))?;
        String::new().append(&host)?;
        String::new().append(&String::from("]"))?;
        host = String::new();
        this.checkPermission(SocketPermission::new(host, String::from("connect,accept"))?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkPropertiesAccess", descriptor = "()V", access = "public"))]
    pub fn checkPropertiesAccess(&self) -> Result<()> {
        let this = self;
        this.checkPermission(PropertyPermission::new(String::from("*"), String::from("read,write"))?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkPropertyAccess", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn checkPropertyAccess(&self, key: String) -> Result<()> {
        let this = self;
        this.checkPermission(PropertyPermission::new(key, String::from("read"))?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkPrintJobAccess", descriptor = "()V", access = "public"))]
    pub fn checkPrintJobAccess(&self) -> Result<()> {
        let this = self;
        this.checkPermission(RuntimePermission::new(String::from("queuePrintJob"))?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getPackages", descriptor = "(Ljava/lang/String;)[Ljava/lang/String;", access = "private static"))]
    pub fn getPackages(p: String) -> Result<Vec<String>> {
        /* TODO: aconst_null  */
        let mut packages: i32 = todo!("stack underflow");
        let _t0 = p.isEmpty()?;
        let mut tok: StringTokenizer = StringTokenizer::new(p, String::from(","))?;
        let _t1 = tok.countTokens()?;
        let mut n: i32 = _t1;
        let mut _arr2: Vec<Object> = Vec::with_capacity(n as usize);
        packages = _arr2;
        let mut i: i32 = 0i32;
        loop {
            let _t0 = tok.hasMoreElements()?;
            if _t0==0i32 { break; }
            let _t0 = tok.nextToken()?;
            let _t1 = _t0.trim()?;
            let mut s: String = _t1;
            i = i.wrapping_add(1i32);
            packages[i as usize] = s;
        }
        let mut _arr3: Vec<Object> = Vec::with_capacity(0i32 as usize);
        packages = _arr3;
        Ok(packages)
    }

    #[cfg_attr(any(), java_method(name = "addNonExportedPackages", descriptor = "(Ljava/lang/ModuleLayer;)V", access = "static"))]
    pub fn addNonExportedPackages(layer: Object) -> Result<()> {
        let _t0: Object = ModuleLoaderMap::bootModules()?;
        let mut bootModules: Object = _t0;
        let _t1: Object = ModuleLoaderMap::platformModules()?;
        let mut platformModules: Object = _t1;
        let _t2 = layer.modules()?;
        let _t3 = _t2.stream()?;
        /* TODO: invokedynamic 220 */
        let _t4 = todo!("stack underflow").map(_t3)?;
        /* TODO: invokedynamic 230 */
        let _t5 = bootModules.filter(platformModules)?;
        /* TODO: invokedynamic 238 */
        let _t6 = _t4.map(_t5)?;
        /* TODO: invokedynamic 239 */
        let _t7 = todo!("stack underflow").flatMap(_t6)?;
        /* TODO: invokedynamic 243 */
        todo!("stack underflow").forEach(_t7)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "invalidatePackageAccessCache", descriptor = "()V", access = "static"))]
    pub fn invalidatePackageAccessCache() -> Result<()> {
        let mut local_0: Object = SecurityManager::packageAccessLock();
        /* TODO: monitorenter  */
        SecurityManager::packageAccessValid(0i32);
        /* TODO: monitorexit  */
        let mut local_1: Object = local_0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        local_0 = SecurityManager::packageDefinitionLock();
        /* TODO: monitorenter  */
        SecurityManager::packageDefinitionValid(0i32);
        /* TODO: monitorexit  */
        let mut local_2: Object = local_0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "nonExportedPkgs", descriptor = "(Ljava/lang/module/ModuleDescriptor;)Ljava/util/Set;", access = "private static"))]
    pub fn nonExportedPkgs(md: Object) -> Result<Object> {
        let _t0 = md.packages()?;
        let mut pkgs: HashSet<_> = HashSet::<_>::new()?;
        let _t1 = md.exports()?;
        let _t2 = _t1.stream()?;
        /* TODO: invokedynamic 276 */
        let _t3 = todo!("stack underflow").filter(_t2)?;
        /* TODO: invokedynamic 279 */
        let _t4 = todo!("stack underflow").map(_t3)?;
        let _t5: Object = Objects::requireNonNull(pkgs)?;
        /* TODO: invokedynamic 286 */
        _t4.forEach(pkgs)?;
        let _t6 = md.opens()?;
        let _t7 = _t6.stream()?;
        /* TODO: invokedynamic 292 */
        let _t8 = todo!("stack underflow").filter(_t7)?;
        /* TODO: invokedynamic 293 */
        let _t9 = todo!("stack underflow").map(_t8)?;
        let _t10: Object = Objects::requireNonNull(pkgs)?;
        /* TODO: invokedynamic 286 */
        _t9.forEach(pkgs)?;
        Ok(pkgs)
    }

    #[cfg_attr(any(), java_method(name = "checkPackageAccess", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn checkPackageAccess(&self, pkg: String) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(pkg, String::from("package name can't be null"))?;
        let _t1 = SecurityManager::nonExportedPkgs().containsKey(pkg)?;
        String::new().append(&String::from("accessClassInPackage."))?;
        String::new().append(&pkg)?;
        this.checkPermission(RuntimePermission::new(String::new())?)?;
        return Ok(());
        let mut plen: Object = SecurityManager::packageAccessLock();
        /* TODO: monitorenter  */
        let _t2: Object = AccessController::doPrivileged(SecurityManager_1::new(this)?)?;
        let mut tmpPropertyStr: Object = _t2;
        let _t3: Vec<String> = SecurityManager::getPackages(tmpPropertyStr)?;
        SecurityManager::packageAccess(_t3);
        SecurityManager::packageAccessValid(1i32);
        let mut restrictedPkgs: Vec<String> = SecurityManager::packageAccess();
        /* TODO: monitorexit  */
        let mut local_5: Object = plen;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t4 = pkg.length()?;
        plen = _t4;
        tmpPropertyStr = restrictedPkgs;
        local_5 = (tmpPropertyStr.len() as i32);
        let mut local_6: i32 = 0i32;
        loop {
            if local_6 >= local_5 { break; }
            let mut restrictedPkg: Object = tmpPropertyStr[local_6 as usize].clone();
            let _t0 = restrictedPkg.length()?;
            let mut rlast: i32 = (_t0).wrapping_sub(1i32);
            let _t1 = pkg.startsWith(restrictedPkg)?;
            let _t2 = restrictedPkg.startsWith(pkg)?;
            let _t3 = restrictedPkg.charAt(rlast)?;
            String::new().append(&String::from("accessClassInPackage."))?;
            String::new().append(&pkg)?;
            this.checkPermission(RuntimePermission::new(String::new())?)?;
            local_6 = local_6.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkPackageDefinition", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn checkPackageDefinition(&self, pkg: String) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(pkg, String::from("package name can't be null"))?;
        let _t1 = SecurityManager::nonExportedPkgs().containsKey(pkg)?;
        String::new().append(&String::from("defineClassInPackage."))?;
        String::new().append(&pkg)?;
        this.checkPermission(RuntimePermission::new(String::new())?)?;
        return Ok(());
        let mut local_3: Object = SecurityManager::packageDefinitionLock();
        /* TODO: monitorenter  */
        let _t2: Object = AccessController::doPrivileged(SecurityManager_2::new(this)?)?;
        let mut tmpPropertyStr: Object = _t2;
        let _t3: Vec<String> = SecurityManager::getPackages(tmpPropertyStr)?;
        SecurityManager::packageDefinition(_t3);
        SecurityManager::packageDefinitionValid(1i32);
        let mut pkgs: Vec<String> = SecurityManager::packageDefinition();
        /* TODO: monitorexit  */
        let mut local_5: Object = local_3;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        local_3 = pkgs;
        tmpPropertyStr = (local_3.len() as i32);
        local_5 = 0i32;
        loop {
            if local_5 >= tmpPropertyStr { break; }
            let mut restrictedPkg: Object = local_3[local_5 as usize].clone();
            let _t0 = pkg.startsWith(restrictedPkg)?;
            String::new().append(&pkg)?;
            String::new().append(&String::from("."))?;
            let _t1 = restrictedPkg.equals(String::new())?;
            String::new().append(&String::from("defineClassInPackage."))?;
            String::new().append(&pkg)?;
            this.checkPermission(RuntimePermission::new(String::new())?)?;
            local_5 = local_5.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkSetFactory", descriptor = "()V", access = "public"))]
    pub fn checkSetFactory(&self) -> Result<()> {
        let this = self;
        this.checkPermission(RuntimePermission::new(String::from("setFactory"))?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkSecurityAccess", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn checkSecurityAccess(&self, target: String) -> Result<()> {
        let this = self;
        this.checkPermission(SecurityPermission::new(target)?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getThreadGroup", descriptor = "()Ljava/lang/ThreadGroup;", access = "public"))]
    pub fn getThreadGroup(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Thread::currentThread()?;
        let _t1 = _t0.getThreadGroup()?;
        Ok(_t1)
    }
}
