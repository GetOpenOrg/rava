#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/System$LoggerFinder",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "System.java",
))]
pub struct System_LoggerFinder;

impl System_LoggerFinder {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "protected"))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        let _t0: Object = System$LoggerFinder::checkPermission()?;
        /* invokespecial Method java/lang/System$LoggerFinder.<init>:(Ljava/lang/Void;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Void;)V", access = "private"))]
    // java: <init>(Ljava/lang/Void;)V
    pub fn new__void(unused: Object) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "checkPermission", descriptor = "()Ljava/lang/Void;", access = "private static"))]
    pub fn checkPermission() -> Result<Object> {
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(System$LoggerFinder::LOGGERFINDER_PERMISSION())?;
        /* TODO: aconst_null  */
        Ok(sm)
    }

    #[cfg_attr(any(), java_native(name = "getLogger", descriptor = "(Ljava/lang/String;Ljava/lang/Module;)Ljava/lang/System$Logger;", access = "public abstract"))]
    pub fn getLogger(&self, arg0: String, arg1: Object) -> Result<Object> {
        todo!("abstract java/lang/System$LoggerFinder.getLogger")
    }

    #[cfg_attr(any(), java_method(name = "getLocalizedLogger", descriptor = "(Ljava/lang/String;Ljava/util/ResourceBundle;Ljava/lang/Module;)Ljava/lang/System$Logger;", access = "public"))]
    pub fn getLocalizedLogger(&self, name: String, bundle: Object, module: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.getLogger(name, module)?;
        Ok(LocalizedLoggerWrapper::new(_t0, bundle)?)
    }

    #[cfg_attr(any(), java_method(name = "getLoggerFinder", descriptor = "()Ljava/lang/System$LoggerFinder;", access = "public static"))]
    pub fn getLoggerFinder() -> Result<Object> {
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(System$LoggerFinder::LOGGERFINDER_PERMISSION())?;
        let _t1: Object = System$LoggerFinder::accessProvider()?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "accessProvider", descriptor = "()Ljava/lang/System$LoggerFinder;", access = "static"))]
    pub fn accessProvider() -> Result<Object> {
        let mut finder: Object = System$LoggerFinder::service();
        /* TODO: invokedynamic 48 */
        let mut pa: Object = finder;
        /* TODO: aconst_null  */
        let mut _arr0: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr0[0i32 as usize] = System$LoggerFinder::LOGGERFINDER_PERMISSION();
        let _t1: Object = AccessController::doPrivileged(todo!("stack underflow"), pa, &_arr0)?;
        finder = _t1;
        return Ok(finder);
        System$LoggerFinder::service(finder);
        Ok(finder)
    }
}
