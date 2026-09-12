#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ResourceBundle",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "ResourceBundle.java",
))]
pub struct ResourceBundle {
    #[cfg_attr(any(), java_field(name = "parent", descriptor = "Ljava/util/ResourceBundle;", access = "protected"))]
    pub parent: Field<Object>,
    #[cfg_attr(any(), java_field(name = "locale", descriptor = "Ljava/util/Locale;", access = "private"))]
    pub locale: Field<Object>,
    #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private"))]
    pub name: Field<String>,
    #[cfg_attr(any(), java_field(name = "expired", descriptor = "Z", access = "private"))]
    pub expired: Field<bool>,
    #[cfg_attr(any(), java_field(name = "cacheKey", descriptor = "Ljava/util/ResourceBundle$CacheKey;", access = "private"))]
    pub cacheKey: Field<Object>,
    #[cfg_attr(any(), java_field(name = "keySet", descriptor = "Ljava/util/Set;", access = "private"))]
    pub keySet: Field<Object>,
}

impl ResourceBundle {
    #[cfg_attr(any(), java_method(name = "getBaseBundleName", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getBaseBundleName(&self) -> Result<String> {
        let this = self;
        Ok(this.name.get())
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    pub fn new() -> Result<Self> {
        let this = Self { parent: Field::new(Default::default()), locale: Field::new(Default::default()), name: Field::new(String::new()), expired: Field::new(false), cacheKey: Field::new(Default::default()), keySet: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        /* TODO: aconst_null  */
        todo!("stack underflow").parent.set(this);
        /* TODO: aconst_null  */
        todo!("stack underflow").locale.set(this);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "getString", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public final"))]
    pub fn getString(&self, key: String) -> Result<String> {
        let this = self;
        let _t0 = this.getObject(key)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getStringArray", descriptor = "(Ljava/lang/String;)[Ljava/lang/String;", access = "public final"))]
    pub fn getStringArray(&self, key: String) -> Result<Vec<String>> {
        let this = self;
        let _t0 = this.getObject(key)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getObject", descriptor = "(Ljava/lang/String;)Ljava/lang/Object;", access = "public final"))]
    pub fn getObject(&self, key: String) -> Result<Object> {
        let this = self;
        let _t0 = this.handleGetObject(key)?;
        let mut obj: Object = _t0;
        let _t1 = this.parent.get().getObject(key)?;
        obj = _t1;
        String::new().append(&String::from("Can't find resource for bundle"))?;
        let _t2 = this.getClass()?;
        let _t3 = _t2.getName()?;
        String::new().append(&_t3)?;
        String::new().append(&String::from(", key"))?;
        String::new().append(&key)?;
        let _t4 = this.getClass()?;
        let _t5 = _t4.getName()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(obj)
    }

    #[cfg_attr(any(), java_method(name = "getLocale", descriptor = "()Ljava/util/Locale;", access = "public"))]
    pub fn getLocale(&self) -> Result<Object> {
        let this = self;
        Ok(this.locale.get())
    }

    #[cfg_attr(any(), java_method(name = "getLoader", descriptor = "(Ljava/lang/Module;)Ljava/lang/ClassLoader;", access = "private static"))]
    pub fn getLoader(module: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(module)?;
        /* TODO: invokedynamic 67 */
        let mut pa: Object = module;
        let _t1: Object = AccessController::doPrivileged(pa)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getLoaderForControl", descriptor = "(Ljava/lang/Module;)Ljava/lang/ClassLoader;", access = "private static"))]
    pub fn getLoaderForControl(module: Object) -> Result<Object> {
        let _t0: Object = ResourceBundle::getLoader(module)?;
        let mut loader: Object = _t0;
        let _t1: Object = ClassLoader::getPlatformClassLoader()?;
        Ok(loader)
    }

    #[cfg_attr(any(), java_method(name = "setParent", descriptor = "(Ljava/util/ResourceBundle;)V", access = "protected"))]
    pub fn setParent(&self, parent: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        this.parent.set(parent);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getBundle", descriptor = "(Ljava/lang/String;)Ljava/util/ResourceBundle;", access = "public static final"))]
    // java: getBundle(Ljava/lang/String;)Ljava/util/ResourceBundle;
    pub fn getBundle__str(baseName: String) -> Result<Object> {
        let _t0: Object = Reflection::getCallerClass()?;
        let mut caller: Object = _t0;
        let _t1: Object = Locale::getDefault()?;
        let _t2: Object = ResourceBundle::getDefaultControl(caller, baseName)?;
        let _t3: Object = ResourceBundle::getBundleImpl(baseName, _t1, caller, _t2)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "getBundle", descriptor = "(Ljava/lang/String;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;", access = "public static final"))]
    // java: getBundle(Ljava/lang/String;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;
    pub fn getBundle__str_resour(baseName: String, control: Object) -> Result<Object> {
        let _t0: Object = Reflection::getCallerClass()?;
        let mut caller: Object = _t0;
        let _t1: Object = Locale::getDefault()?;
        let mut targetLocale: Object = _t1;
        ResourceBundle::checkNamedModule(caller)?;
        let _t2: Object = ResourceBundle::getBundleImpl(baseName, targetLocale, caller, control)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "getBundle", descriptor = "(Ljava/lang/String;Ljava/util/Locale;)Ljava/util/ResourceBundle;", access = "public static final"))]
    // java: getBundle(Ljava/lang/String;Ljava/util/Locale;)Ljava/util/ResourceBundle;
    pub fn getBundle__str_locale(baseName: String, locale: Object) -> Result<Object> {
        let _t0: Object = Reflection::getCallerClass()?;
        let mut caller: Object = _t0;
        let _t1: Object = ResourceBundle::getDefaultControl(caller, baseName)?;
        let _t2: Object = ResourceBundle::getBundleImpl(baseName, locale, caller, _t1)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "getBundle", descriptor = "(Ljava/lang/String;Ljava/lang/Module;)Ljava/util/ResourceBundle;", access = "public static"))]
    // java: getBundle(Ljava/lang/String;Ljava/lang/Module;)Ljava/util/ResourceBundle;
    pub fn getBundle__str_module(baseName: String, module: Object) -> Result<Object> {
        let _t0: Object = Reflection::getCallerClass()?;
        let _t1: Object = Locale::getDefault()?;
        let _t2: Object = ResourceBundle::getDefaultControl(module, baseName)?;
        let _t3: Object = ResourceBundle::getBundleFromModule(_t0, module, baseName, _t1, _t2)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "getBundle", descriptor = "(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/Module;)Ljava/util/ResourceBundle;", access = "public static"))]
    // java: getBundle(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/Module;)Ljava/util/ResourceBundle;
    pub fn getBundle__str_locale_module(baseName: String, targetLocale: Object, module: Object) -> Result<Object> {
        let _t0: Object = Reflection::getCallerClass()?;
        let _t1: Object = ResourceBundle::getDefaultControl(module, baseName)?;
        let _t2: Object = ResourceBundle::getBundleFromModule(_t0, module, baseName, targetLocale, _t1)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "getBundle", descriptor = "(Ljava/lang/String;Ljava/util/Locale;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;", access = "public static final"))]
    // java: getBundle(Ljava/lang/String;Ljava/util/Locale;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;
    pub fn getBundle__str_locale_resour(baseName: String, targetLocale: Object, control: Object) -> Result<Object> {
        let _t0: Object = Reflection::getCallerClass()?;
        let mut caller: Object = _t0;
        ResourceBundle::checkNamedModule(caller)?;
        let _t1: Object = ResourceBundle::getBundleImpl(baseName, targetLocale, caller, control)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getBundle", descriptor = "(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/ClassLoader;)Ljava/util/ResourceBundle;", access = "public static"))]
    // java: getBundle(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/ClassLoader;)Ljava/util/ResourceBundle;
    pub fn getBundle__str_locale_classl(baseName: String, locale: Object, loader: Object) -> Result<Object> {
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: Object = Reflection::getCallerClass()?;
        let mut caller: Object = _t0;
        let _t1: Object = ResourceBundle::getDefaultControl(caller, baseName)?;
        let _t2: Object = ResourceBundle::getBundleImpl(baseName, locale, caller, loader, _t1)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "getBundle", descriptor = "(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/ClassLoader;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;", access = "public static"))]
    // java: getBundle(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/ClassLoader;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;
    pub fn getBundle__str_locale_classl_resour(baseName: String, targetLocale: Object, loader: Object, control: Object) -> Result<Object> {
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: Object = Reflection::getCallerClass()?;
        let mut caller: Object = _t0;
        ResourceBundle::checkNamedModule(caller)?;
        let _t1: Object = ResourceBundle::getBundleImpl(baseName, targetLocale, caller, loader, control)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getDefaultControl", descriptor = "(Ljava/lang/Class;Ljava/lang/String;)Ljava/util/ResourceBundle$Control;", access = "private static"))]
    // java: getDefaultControl(Ljava/lang/Class;Ljava/lang/String;)Ljava/util/ResourceBundle$Control;
    pub fn getDefaultControl__class_str(caller: Object, baseName: String) -> Result<Object> {
        let _t0: Object = ResourceBundle::getCallerModule(caller)?;
        let mut callerModule: Object = _t0;
        let _t1: Object = ResourceBundle::getDefaultControl(callerModule, baseName)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getDefaultControl", descriptor = "(Ljava/lang/Module;Ljava/lang/String;)Ljava/util/ResourceBundle$Control;", access = "private static"))]
    // java: getDefaultControl(Ljava/lang/Module;Ljava/lang/String;)Ljava/util/ResourceBundle$Control;
    pub fn getDefaultControl__module_str(targetModule: Object, baseName: String) -> Result<Object> {
        let _t0 = targetModule.isNamed()?;
        let _t1: Object = ResourceBundle$ResourceBundleControlProviderHolder::getControl(baseName)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "checkNamedModule", descriptor = "(Ljava/lang/Class;)V", access = "private static"))]
    pub fn checkNamedModule(caller: Object) -> Result<()> {
        let _t0: Object = ResourceBundle::getCallerModule(caller)?;
        let mut callerModule: Object = _t0;
        let _t1 = callerModule.isNamed()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getBundleImpl", descriptor = "(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/Class;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;", access = "private static"))]
    // java: getBundleImpl(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/Class;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;
    pub fn getBundleImpl__str_locale_class_resour(baseName: String, locale: Object, caller: Object, control: Object) -> Result<Object> {
        let _t0: Object = ResourceBundle::getCallerModule(caller)?;
        let _t1: Object = ResourceBundle::getLoader(_t0)?;
        let mut loader: Object = _t1;
        let _t2: Object = ResourceBundle::getBundleImpl(baseName, locale, caller, loader, control)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "getCallerModule", descriptor = "(Ljava/lang/Class;)Ljava/lang/Module;", access = "private static"))]
    pub fn getCallerModule(caller: Object) -> Result<Object> {
        let _t0 = caller.getModule()?;
        let _t1: Object = ClassLoader::getSystemClassLoader()?;
        let _t2 = _t1.getUnnamedModule()?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "getBundleImpl", descriptor = "(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/Class;Ljava/lang/ClassLoader;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;", access = "private static"))]
    // java: getBundleImpl(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/Class;Ljava/lang/ClassLoader;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;
    pub fn getBundleImpl__str_locale_class_classl_resour(baseName: String, locale: Object, caller: Object, loader: Object, control: Object) -> Result<Object> {
        let _t0: Object = ResourceBundle::getCallerModule(caller)?;
        let mut callerModule: Object = _t0;
        let _t1 = callerModule.isNamed()?;
        let _t2: Object = ResourceBundle::getLoader(callerModule)?;
        let _t3: Object = ResourceBundle::getBundleImpl(callerModule, callerModule, baseName, locale, control)?;
        return Ok(_t3);
        let _t4 = loader.getUnnamedModule()?;
        let _t5: Object = BootLoader::getUnnamedModule()?;
        let mut unnamedModule: Object = _t5;
        let _t6: Object = ResourceBundle::getBundleImpl(callerModule, unnamedModule, baseName, locale, control)?;
        Ok(_t6)
    }

    #[cfg_attr(any(), java_method(name = "getBundleFromModule", descriptor = "(Ljava/lang/Class;Ljava/lang/Module;Ljava/lang/String;Ljava/util/Locale;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;", access = "private static"))]
    pub fn getBundleFromModule(caller: Object, module: Object, baseName: String, locale: Object, control: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(module)?;
        let _t1: Object = ResourceBundle::getCallerModule(caller)?;
        let mut callerModule: Object = _t1;
        let _t2: Object = System::getSecurityManager()?;
        let mut sm: Object = _t2;
        sm.checkPermission(SecurityConstants::GET_CLASSLOADER_PERMISSION())?;
        let _t3: Object = ResourceBundle::getBundleImpl(callerModule, module, baseName, locale, control)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "getBundleImpl", descriptor = "(Ljava/lang/Module;Ljava/lang/Module;Ljava/lang/String;Ljava/util/Locale;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;", access = "private static"))]
    // java: getBundleImpl(Ljava/lang/Module;Ljava/lang/Module;Ljava/lang/String;Ljava/util/Locale;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;
    pub fn getBundleImpl__module_module_str_locale_resour(callerModule: Object, module: Object, baseName: String, locale: Object, control: Object) -> Result<Object> {
        return Err(JvmError::Custom(String::from("athrow")));
        let mut cacheKey: ResourceBundle_CacheKey = ResourceBundle_CacheKey::new(baseName, locale, module, callerModule)?;
        /* TODO: aconst_null  */
        let mut bundle: Object = control;
        let _t0 = ResourceBundle::cacheList().get(cacheKey)?;
        let mut bundleRef: Object = _t0;
        let _t1 = bundleRef.get()?;
        bundle = _t1;
        /* TODO: aconst_null  */
        bundleRef = bundleRef;
        let _t2: bool = ResourceBundle::isValidBundle(bundle)?;
        let _t3: bool = ResourceBundle::hasValidParentChain(bundle)?;
        return Ok(bundle);
        let mut isKnownControl: i32 = true!=0i32;
        let _t4 = control.getFormats(baseName)?;
        let mut formats: Object = _t4;
        let _t5: bool = ResourceBundle::checkList(formats)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: aconst_null  */
        let mut baseBundle: bool = _t5;
        let mut targetLocale: Object = locale;
        loop {
            if targetLocale.is_none() { break; }
            let _t0 = control.getCandidateLocales(baseName, targetLocale)?;
            let mut candidateLocales: Object = _t0;
            let _t1: bool = ResourceBundle::checkList(candidateLocales)?;
            return Err(JvmError::Custom(String::from("athrow")));
            let _t2: Object = ResourceBundle::findBundle(callerModule, module, cacheKey, candidateLocales, formats, 0i32, control, baseBundle)?;
            bundle = _t2;
            let _t3: bool = ResourceBundle::isValidBundle(bundle)?;
            let _t4 = Locale::ROOT().equals(bundle.locale.get())?;
            let mut isBaseBundle: i32 = _t4;
            let _t5 = bundle.locale.get().equals(locale)?;
            let _t6 = candidateLocales.size()?;
            let _t7 = candidateLocales.get(0i32)?;
            let _t8 = bundle.locale.get().equals(_t7)?;
            baseBundle = bundle;
            let _t9 = control.getFallbackLocale(baseName, targetLocale)?;
            targetLocale = _t9;
        }
        let _t6 = cacheKey.getCause()?;
        ResourceBundle::throwMissingResourceException(baseName, locale, _t6)?;
        bundle = baseBundle;
        Reference::reachabilityFence(callerModule)?;
        Reference::reachabilityFence(module)?;
        Ok(bundle)
    }

    #[cfg_attr(any(), java_method(name = "checkList", descriptor = "(Ljava/util/List;)Z", access = "private static"))]
    pub fn checkList(a: Object) -> Result<bool> {
        let _t0 = a.isEmpty()?;
        let mut valid: i32 = _t0==0i32;
        let _t1 = a.size()?;
        let mut size: i32 = _t1;
        let mut i: i32 = 0i32;
        loop {
            if valid==0i32 { break; }
            let _t0 = a.get(i)?;
            valid = 0i32;
            i = i.wrapping_add(1i32);
        }
        Ok(valid)
    }

    #[cfg_attr(any(), java_method(name = "findBundle", descriptor = "(Ljava/lang/Module;Ljava/lang/Module;Ljava/util/ResourceBundle$CacheKey;Ljava/util/List;Ljava/util/List;ILjava/util/ResourceBundle$Control;Ljava/util/ResourceBundle;)Ljava/util/ResourceBundle;", access = "private static"))]
    pub fn findBundle(callerModule: Object, module: Object, cacheKey: Object, candidateLocales: Object, formats: Object, index: i32, control: Object, baseBundle: Object) -> Result<Object> {
        let _t0 = candidateLocales.get(index)?;
        let mut targetLocale: Object = _t0;
        /* TODO: aconst_null  */
        let mut parent: i32 = todo!("stack underflow");
        let _t1 = candidateLocales.size()?;
        let _t2: Object = ResourceBundle::findBundle(callerModule, module, cacheKey, candidateLocales, formats, (index).wrapping_add(1i32), control, baseBundle)?;
        parent = _t2;
        let _t3 = Locale::ROOT().equals(targetLocale)?;
        return Ok(baseBundle);
        loop {
            let _t0 = ResourceBundle::referenceQueue().poll()?;
            let mut ref_: Object = _t0;
            if _t0.is_none() { break; }
            let _t0 = ref_.getCacheKey()?;
            let _t1 = ResourceBundle::cacheList().remove(_t0)?;
        }
        let mut expiredBundle: i32 = 0i32;
        let _t4 = cacheKey.setLocale(targetLocale)?;
        let _t5: Object = ResourceBundle::findBundleInCache(cacheKey, control)?;
        let mut bundle: Object = _t5;
        let _t6: bool = ResourceBundle::isValidBundle(bundle)?;
        expiredBundle = bundle.expired.get();
        return Ok(bundle);
        let _t7 = ResourceBundle::cacheList().get(cacheKey)?;
        let mut bundleRef: Object = _t7;
        let _t8 = bundleRef.refersTo(bundle)?;
        let _t9 = ResourceBundle::cacheList().remove(cacheKey, bundleRef)?;
        let mut _arr10: Vec<Object> = Vec::with_capacity(4i32 as usize);
        _arr10[0i32 as usize] = index;
        _arr10[1i32 as usize] = candidateLocales;
        _arr10[2i32 as usize] = cacheKey;
        _arr10[3i32 as usize] = formats;
        ResourceBundle::trace(String::from("findBundle: %d %s %s formats: %s%n"), &_arr10)?;
        let _t11 = module.isNamed()?;
        let _t12: Object = ResourceBundle::loadBundle(cacheKey, formats, control, module, callerModule)?;
        bundle = _t12;
        let _t13: Object = ResourceBundle::loadBundle(cacheKey, formats, control, expiredBundle)?;
        bundle = _t13;
        bundle.setParent(parent)?;
        bundle.locale.set(targetLocale);
        let _t14: Object = ResourceBundle::putBundleInCache(cacheKey, bundle, control)?;
        bundle = _t14;
        return Ok(bundle);
        let _t15: Object = ResourceBundle::putBundleInCache(cacheKey, ResourceBundle::NONEXISTENT_BUNDLE(), control)?;
        Ok(parent)
    }

    #[cfg_attr(any(), java_method(name = "loadBundle", descriptor = "(Ljava/util/ResourceBundle$CacheKey;Ljava/util/List;Ljava/util/ResourceBundle$Control;Ljava/lang/Module;Ljava/lang/Module;)Ljava/util/ResourceBundle;", access = "private static"))]
    // java: loadBundle(Ljava/util/ResourceBundle$CacheKey;Ljava/util/List;Ljava/util/ResourceBundle$Control;Ljava/lang/Module;Ljava/lang/Module;)Ljava/util/ResourceBundle;
    pub fn loadBundle__resour_list_resour_module_module(cacheKey: Object, formats: Object, control: Object, module: Object, callerModule: Object) -> Result<Object> {
        let _t0 = cacheKey.getName()?;
        let mut baseName: String = _t0;
        let _t1 = cacheKey.getLocale()?;
        let mut targetLocale: Object = _t1;
        /* TODO: aconst_null  */
        let mut bundle: i32 = todo!("stack underflow");
        let _t2 = cacheKey.hasProviders()?;
        let _t3 = cacheKey.getProviders()?;
        let _t4: Object = ResourceBundle::loadBundleFromProviders(baseName, targetLocale, _t3, cacheKey)?;
        bundle = _t4;
        let _t5: Object = ResourceBundle::getLoader(module)?;
        let mut loader: Object = _t5;
        let _t6: Object = ResourceBundle::getResourceBundleProviderType(baseName, loader)?;
        let mut svc: Object = _t6;
        let _t7: bool = Reflection::verifyModuleAccess(callerModule, svc)?;
        let _t8 = callerModule.canUse(svc)?;
        let _t9 = cacheKey.getProviders()?;
        let _t10: Object = ResourceBundle::loadBundleFromProviders(baseName, targetLocale, _t9, cacheKey)?;
        bundle = _t10;
        cacheKey.setFormat(String::from(""))?;
        let _t11 = cacheKey.callerHasProvider()?;
        let _t12 = formats.iterator()?;
        loader = _t12;
        loop {
            let _t0 = loader.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = loader.next()?;
            svc = _t0;
            let mut e: Object = svc;
            let mut local_11: i32 = -1i32;
            let _t1 = e.hashCode()?;
            /* TODO: lookupswitch default:223 -1786601812:192 779991167:209 */
            let _t2 = e.equals(String::from("java.class"))?;
            local_11 = 0i32;
            let _t3 = e.equals(String::from("java.properties"))?;
            local_11 = 1i32;
            /* TODO: lookupswitch default:278 0:252 1:265 */
            let _t4: Object = ResourceBundle$ResourceBundleProviderHelper::loadResourceBundle(callerModule, module, baseName, targetLocale)?;
            let _t5: Object = ResourceBundle$ResourceBundleProviderHelper::loadPropertyResourceBundle(callerModule, module, baseName, targetLocale)?;
            String::new().append(&String::from("unexpected format:"))?;
            String::new().append(&svc)?;
            return Err(JvmError::Custom(String::from("athrow")));
            bundle = _t5;
            cacheKey.setFormat(svc)?;
            e = bundle;
            cacheKey.setCause(e)?;
        }
        Ok(bundle)
    }

    #[cfg_attr(any(), java_method(name = "getServiceLoader", descriptor = "(Ljava/lang/Module;Ljava/lang/String;)Ljava/util/ServiceLoader;", access = "private static"))]
    pub fn getServiceLoader(module: Object, baseName: String) -> Result<Object> {
        let _t0 = module.isNamed()?;
        /* TODO: aconst_null  */
        return Ok(_t0);
        let _t1: Object = ResourceBundle::getLoader(module)?;
        let mut loader: Object = _t1;
        let _t2: Object = ResourceBundle::getResourceBundleProviderType(baseName, loader)?;
        let mut service: Object = _t2;
        let _t3: bool = Reflection::verifyModuleAccess(module, service)?;
        let _t4: Object = ServiceLoader::load(service, loader, module)?;
        return Ok(_t4);
        let mut e: bool = _t3;
        /* TODO: aconst_null  */
        return Ok(service);
        /* TODO: aconst_null  */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "getResourceBundleProviderType", descriptor = "(Ljava/lang/String;Ljava/lang/ClassLoader;)Ljava/lang/Class;", access = "private static"))]
    pub fn getResourceBundleProviderType(baseName: String, loader: Object) -> Result<Object> {
        let _t0 = baseName.lastIndexOf(46i32)?;
        let mut i: i32 = _t0;
        /* TODO: aconst_null  */
        return Ok(i);
        let _t1 = baseName.length()?;
        let _t2 = baseName.substring((i).wrapping_add(1i32), _t1)?;
        String::new().append(&_t2)?;
        String::new().append(&String::from("Provider"))?;
        let mut name: String = String::new();
        let _t3 = baseName.substring(0i32, i)?;
        String::new().append(&_t3)?;
        String::new().append(&String::from(".spi."))?;
        String::new().append(&name)?;
        let mut providerName: String = String::new();
        let _t4: Object = AccessController::doPrivileged(ResourceBundle_3::new(providerName, loader)?)?;
        Ok(_t4)
    }

    #[cfg_attr(any(), java_method(name = "loadBundleFromProviders", descriptor = "(Ljava/lang/String;Ljava/util/Locale;Ljava/util/ServiceLoader;Ljava/util/ResourceBundle$CacheKey;)Ljava/util/ResourceBundle;", access = "private static"))]
    pub fn loadBundleFromProviders(baseName: String, locale: Object, providers: Object, cacheKey: Object) -> Result<Object> {
        /* TODO: aconst_null  */
        return Ok(providers);
        let _t0: Object = AccessController::doPrivileged(ResourceBundle_4::new(providers, cacheKey, baseName, locale)?)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "loadBundle", descriptor = "(Ljava/util/ResourceBundle$CacheKey;Ljava/util/List;Ljava/util/ResourceBundle$Control;Z)Ljava/util/ResourceBundle;", access = "private static"))]
    // java: loadBundle(Ljava/util/ResourceBundle$CacheKey;Ljava/util/List;Ljava/util/ResourceBundle$Control;Z)Ljava/util/ResourceBundle;
    pub fn loadBundle__resour_list_resour_z(cacheKey: Object, formats: Object, control: Object, reload: bool) -> Result<Object> {
        let _t0 = cacheKey.getLocale()?;
        let mut targetLocale: Object = _t0;
        let _t1 = cacheKey.getModule()?;
        let mut module: Object = _t1;
        String::new().append(&String::from("Module for cache key:"))?;
        String::new().append(&cacheKey)?;
        String::new().append(&String::from("has been GCed."))?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t2: Object = ResourceBundle::getLoaderForControl(module)?;
        let mut loader: Object = _t2;
        /* TODO: aconst_null  */
        let mut bundle: Object = module;
        let _t3 = formats.iterator()?;
        let mut local_8: Object = _t3;
        loop {
            let _t0 = local_8.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_8.next()?;
            let mut format: Object = _t0;
            let _t1 = cacheKey.getName()?;
            let _t2 = control.newBundle(_t1, targetLocale, format, loader, reload)?;
            bundle = _t2;
            let mut error: i32 = todo!("stack underflow");
            cacheKey.setCause(error)?;
            cacheKey.setFormat(format)?;
            let _t3 = cacheKey.getName()?;
            bundle.name.set(_t3);
            bundle.locale.set(targetLocale);
            bundle.expired.set(0i32);
        }
        Ok(bundle)
    }

    #[cfg_attr(any(), java_method(name = "isValidBundle", descriptor = "(Ljava/util/ResourceBundle;)Z", access = "private static"))]
    pub fn isValidBundle(bundle: Object) -> Result<bool> {
        Ok(/* if_acmpeq */ true)
    }

    #[cfg_attr(any(), java_method(name = "hasValidParentChain", descriptor = "(Ljava/util/ResourceBundle;)Z", access = "private static"))]
    pub fn hasValidParentChain(bundle: Object) -> Result<bool> {
        let _t0: i64 = System::currentTimeMillis()?;
        let mut now: i64 = _t0;
        loop {
            if bundle.is_none() { break; }
            return Ok(0i32);
            let mut key: Object = bundle.cacheKey.get();
            let mut expirationTime: i64 = key.expirationTime.get();
            /* TODO: lcmp  */
            /* TODO: lcmp  */
            return Ok(0i32);
            bundle = bundle.parent.get();
        }
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "throwMissingResourceException", descriptor = "(Ljava/lang/String;Ljava/util/Locale;Ljava/lang/Throwable;)V", access = "private static"))]
    pub fn throwMissingResourceException(baseName: String, locale: Object, cause: Object) -> Result<()> {
        /* TODO: aconst_null  */
        cause = true;
        String::new().append(&String::from("Can't find bundle for base name"))?;
        String::new().append(&baseName)?;
        String::new().append(&String::from(", locale"))?;
        String::new().append(&locale)?;
        String::new().append(&baseName)?;
        String::new().append(&String::from("_"))?;
        String::new().append(&locale)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "findBundleInCache", descriptor = "(Ljava/util/ResourceBundle$CacheKey;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;", access = "private static"))]
    pub fn findBundleInCache(cacheKey: Object, control: Object) -> Result<Object> {
        let _t0 = ResourceBundle::cacheList().get(cacheKey)?;
        let mut bundleRef: Object = _t0;
        /* TODO: aconst_null  */
        return Ok(bundleRef);
        let _t1 = bundleRef.get()?;
        let mut bundle: Object = _t1;
        /* TODO: aconst_null  */
        return Ok(bundle);
        let mut p: Object = bundle.parent.get();
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        bundle.expired.set(1i32);
        /* TODO: aconst_null  */
        ResourceBundle::NONEXISTENT_BUNDLE().cacheKey.set(bundle);
        let _t2 = ResourceBundle::cacheList().remove(cacheKey, bundleRef)?;
        /* TODO: aconst_null  */
        bundle = bundle;
        let _t3 = bundleRef.getCacheKey()?;
        let mut key: Object = _t3;
        let mut expirationTime: i64 = key.expirationTime.get();
        /* TODO: lcmp  */
        let _t4: i64 = System::currentTimeMillis()?;
        /* TODO: lcmp  */
        let mut local_8: Object = bundle;
        /* TODO: monitorenter  */
        expirationTime = key.expirationTime.get();
        /* TODO: lcmp  */
        let _t5: i64 = System::currentTimeMillis()?;
        /* TODO: lcmp  */
        let _t6 = cacheKey.getModule()?;
        let mut module: Object = _t6;
        let _t7 = key.getName()?;
        let _t8 = key.getLocale()?;
        let _t9 = key.getFormat()?;
        let _t10: Object = ResourceBundle::getLoaderForControl(module)?;
        let _t11 = control.needsReload(_t7, _t8, _t9, _t10, bundle, key.loadTime.get())?;
        module.expired.set(_t11!=0i32);
        module = bundle;
        cacheKey.setCause(module)?;
        /* TODO: aconst_null  */
        bundle.expired.get().cacheKey.set(bundle);
        let _t12 = ResourceBundle::cacheList().remove(cacheKey, bundleRef)?;
        ResourceBundle::setExpirationTime(key, control)?;
        /* TODO: monitorexit  */
        let mut local_10: Object = local_8;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t13 = ResourceBundle::cacheList().remove(cacheKey, bundleRef)?;
        /* TODO: aconst_null  */
        bundle = local_8;
        Ok(bundle)
    }

    #[cfg_attr(any(), java_method(name = "putBundleInCache", descriptor = "(Ljava/util/ResourceBundle$CacheKey;Ljava/util/ResourceBundle;Ljava/util/ResourceBundle$Control;)Ljava/util/ResourceBundle;", access = "private static"))]
    pub fn putBundleInCache(cacheKey: Object, bundle: Object, control: Object) -> Result<Object> {
        ResourceBundle::setExpirationTime(cacheKey, control)?;
        /* TODO: lcmp  */
        let mut key: ResourceBundle_CacheKey = ResourceBundle_CacheKey::new(cacheKey)?;
        let mut bundleRef: ResourceBundle_BundleReference = ResourceBundle_BundleReference::new(bundle, ResourceBundle::referenceQueue(), key)?;
        bundle.cacheKey.set(key);
        let _t0 = ResourceBundle::cacheList().putIfAbsent(key, bundleRef)?;
        let mut result: Object = _t0;
        let _t1 = result.get()?;
        let mut rb: Object = _t1;
        /* TODO: aconst_null  */
        rb.expired.get().cacheKey.set(bundle);
        bundle = rb;
        bundleRef.clear()?;
        let _t2 = ResourceBundle::cacheList().put(key, bundleRef)?;
        Ok(bundle)
    }

    #[cfg_attr(any(), java_method(name = "setExpirationTime", descriptor = "(Ljava/util/ResourceBundle$CacheKey;Ljava/util/ResourceBundle$Control;)V", access = "private static"))]
    pub fn setExpirationTime(cacheKey: Object, control: Object) -> Result<()> {
        let _t0 = cacheKey.getName()?;
        let _t1 = cacheKey.getLocale()?;
        let _t2 = control.getTimeToLive(_t0, _t1)?;
        let mut ttl: i64 = _t2;
        /* TODO: lcmp  */
        let _t3: i64 = System::currentTimeMillis()?;
        let mut now: i64 = _t3;
        cacheKey.loadTime.set(now);
        cacheKey.expirationTime.set((now).wrapping_add(ttl));
        /* TODO: lcmp  */
        cacheKey.expirationTime.set(ttl);
        String::new().append(&String::from("Invalid Control: TTL="))?;
        String::new().append(&ttl)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "clearCache", descriptor = "()V", access = "public static final"))]
    // java: clearCache()V
    pub fn clearCache() -> Result<()> {
        let _t0: Object = Reflection::getCallerClass()?;
        let _t1: Object = ResourceBundle::getCallerModule(_t0)?;
        let mut callerModule: Object = _t1;
        let _t2 = ResourceBundle::cacheList().keySet()?;
        /* TODO: invokedynamic 541 */
        let _t3 = _t2.removeIf(callerModule)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "clearCache", descriptor = "(Ljava/lang/ClassLoader;)V", access = "public static final"))]
    // java: clearCache(Ljava/lang/ClassLoader;)V
    pub fn clearCache__classl(loader: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull(loader)?;
        let _t1 = ResourceBundle::cacheList().keySet()?;
        /* TODO: invokedynamic 551 */
        let _t2 = _t1.removeIf(loader)?;
        Ok(())
    }

    #[cfg_attr(any(), java_native(name = "handleGetObject", descriptor = "(Ljava/lang/String;)Ljava/lang/Object;", access = "protected abstract"))]
    pub fn handleGetObject(&self, arg0: String) -> Result<Object> {
        todo!("abstract java/util/ResourceBundle.handleGetObject")
    }

    #[cfg_attr(any(), java_native(name = "getKeys", descriptor = "()Ljava/util/Enumeration;", access = "public abstract"))]
    pub fn getKeys(&self) -> Result<Object> {
        todo!("abstract java/util/ResourceBundle.getKeys")
    }

    #[cfg_attr(any(), java_method(name = "containsKey", descriptor = "(Ljava/lang/String;)Z", access = "public"))]
    pub fn containsKey(&self, key: String) -> Result<bool> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut rb: java/util/ResourceBundle = this;
        loop {
            if rb.is_none() { break; }
            let _t0 = rb.handleKeySet()?;
            let _t1 = _t0.contains(key)?;
            return Ok(1i32);
            rb = rb.parent.get();
        }
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "keySet", descriptor = "()Ljava/util/Set;", access = "public"))]
    pub fn keySet(&self) -> Result<Object> {
        let this = self;
        let mut keys: HashSet<_> = HashSet::<_>::new()?;
        let mut rb: java/util/ResourceBundle = this;
        loop {
            if rb.is_none() { break; }
            let _t0 = rb.handleKeySet()?;
            let _t1 = keys.addAll(_t0)?;
            rb = rb.parent.get();
        }
        Ok(keys)
    }

    #[cfg_attr(any(), java_method(name = "handleKeySet", descriptor = "()Ljava/util/Set;", access = "protected"))]
    pub fn handleKeySet(&self) -> Result<Object> {
        let this = self;
        let mut local_1: java/util/ResourceBundle = this;
        /* TODO: monitorenter  */
        let mut keys: HashSet<_> = HashSet::<_>::new()?;
        let _t0 = this.getKeys()?;
        let mut enumKeys: Object = _t0;
        loop {
            let _t0 = enumKeys.hasMoreElements()?;
            if _t0==0i32 { break; }
            let _t0 = enumKeys.nextElement()?;
            let mut key: Object = _t0;
            let _t1 = this.handleGetObject(key)?;
            let _t2 = keys.add(key)?;
        }
        this.keySet.set(keys);
        /* TODO: monitorexit  */
        let mut local_5: java/util/ResourceBundle = local_1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(this.keySet.get())
    }

    #[cfg_attr(any(), java_method(name = "uncheckedThrow", descriptor = "(Ljava/lang/Throwable;)V", access = "private static"))]
    pub fn uncheckedThrow(t: Object) -> Result<()> {
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "trace", descriptor = "(Ljava/lang/String;[Ljava/lang/Object;)V", access = "private static"))]
    pub fn trace(format: String, params: &[Object]) -> Result<()> {
        let _t0 = System::out().format(format, params)?;
        Ok(())
    }
}
