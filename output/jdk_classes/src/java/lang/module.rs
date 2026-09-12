#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Module",
    super_class = "java/lang/Object",
    interfaces  = "java/lang/reflect/AnnotatedElement",
    access      = "public final",
    source      = "Module.java",
))]
pub struct Module {
    #[cfg_attr(any(), java_field(name = "layer", descriptor = "Ljava/lang/ModuleLayer;", access = "private final"))]
    pub layer: Field<Object>,
    #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub name: Field<String>,
    #[cfg_attr(any(), java_field(name = "loader", descriptor = "Ljava/lang/ClassLoader;", access = "private final"))]
    pub loader: Field<Object>,
    #[cfg_attr(any(), java_field(name = "descriptor", descriptor = "Ljava/lang/module/ModuleDescriptor;", access = "private final"))]
    pub descriptor: Field<Object>,
    #[cfg_attr(any(), java_field(name = "enableNativeAccess", descriptor = "Z", access = "private"))]
    pub enableNativeAccess: Field<bool>,
    #[cfg_attr(any(), java_field(name = "reads", descriptor = "Ljava/util/Set;", access = "private"))]
    pub reads: Field<Object>,
    #[cfg_attr(any(), java_field(name = "openPackages", descriptor = "Ljava/util/Map;", access = "private"))]
    pub openPackages: Field<Object>,
    #[cfg_attr(any(), java_field(name = "exportedPackages", descriptor = "Ljava/util/Map;", access = "private"))]
    pub exportedPackages: Field<Object>,
    #[cfg_attr(any(), java_field(name = "moduleInfoClass", descriptor = "Ljava/lang/Class;", access = "private"))]
    pub moduleInfoClass: Field<Object>,
}

impl Module {
    // java: <init>(Ljava/lang/ModuleLayer;Ljava/lang/ClassLoader;Ljava/lang/module/ModuleDescriptor;Ljava/net/URI;)V
    // java: <init>(Ljava/lang/ModuleLayer;Ljava/lang/ClassLoader;Ljava/lang/module/ModuleDescriptor;Ljava/net/URI;)V
    pub fn new__module_classl_module_uri(layer: Object, loader: Object, descriptor: Object, uri: Object) -> Result<Self> {
        let this = Self { layer: Field::new(Default::default()), name: Field::new(String::new()), loader: Field::new(Default::default()), descriptor: Field::new(Default::default()), enableNativeAccess: Field::new(false), reads: Field::new(Default::default()), openPackages: Field::new(Default::default()), exportedPackages: Field::new(Default::default()), moduleInfoClass: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.layer.set(layer);
        let _t0 = descriptor.name()?;
        this.name.set(_t0);
        this.loader.set(loader);
        this.descriptor.set(descriptor);
        let _t1 = descriptor.isOpen()?;
        let _t2 = descriptor.isAutomatic()?;
        let mut isOpen: i32 = _t2!=0i32;
        let _t3 = descriptor.version()?;
        /* TODO: aconst_null  */
        let _t4 = _t1.orElse(_t3)?;
        let mut version: Object = _t4;
        /* TODO: aconst_null  */
        let _t5: String = Objects::toString(todo!("stack underflow"), version)?;
        let mut vs: String = _t5;
        /* TODO: aconst_null  */
        let _t6: String = Objects::toString(todo!("stack underflow"), uri)?;
        let mut loc: String = _t6;
        let _t7 = descriptor.packages()?;
        let _t8 = _t7.toArray()?;
        let mut packages: Vec<Object> = _t8;
        Module::defineModule0(this, isOpen, vs, loc, &packages)?;
        let _t9: Object = ClassLoaders::platformClassLoader()?;
        let _t10 = this.implAddEnableNativeAccess()?;
        Ok(this)
    }

    // java: <init>(Ljava/lang/ClassLoader;)V
    // java: <init>(Ljava/lang/ClassLoader;)V
    pub fn new__classl(loader: Object) -> Result<Self> {
        let this = Self { layer: Field::new(Default::default()), name: Field::new(String::new()), loader: Field::new(Default::default()), descriptor: Field::new(Default::default()), enableNativeAccess: Field::new(false), reads: Field::new(Default::default()), openPackages: Field::new(Default::default()), exportedPackages: Field::new(Default::default()), moduleInfoClass: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        /* TODO: aconst_null  */
        todo!("stack underflow").layer.set(this);
        /* TODO: aconst_null  */
        todo!("stack underflow").name.set(this);
        this.loader.set(loader);
        /* TODO: aconst_null  */
        todo!("stack underflow").descriptor.set(this);
        Ok(this)
    }

    // java: <init>(Ljava/lang/ClassLoader;Ljava/lang/module/ModuleDescriptor;)V
    // java: <init>(Ljava/lang/ClassLoader;Ljava/lang/module/ModuleDescriptor;)V
    pub fn new__classl_module(loader: Object, descriptor: Object) -> Result<Self> {
        let this = Self { layer: Field::new(Default::default()), name: Field::new(String::new()), loader: Field::new(Default::default()), descriptor: Field::new(Default::default()), enableNativeAccess: Field::new(false), reads: Field::new(Default::default()), openPackages: Field::new(Default::default()), exportedPackages: Field::new(Default::default()), moduleInfoClass: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        /* TODO: aconst_null  */
        todo!("stack underflow").layer.set(this);
        let _t0 = descriptor.name()?;
        this.name.set(_t0);
        this.loader.set(loader);
        this.descriptor.set(descriptor);
        Ok(this)
    }

    // java: isNamed()Z
    pub fn isNamed(&self) -> Result<bool> {
        let this = self;
        Ok(!this.name.get().is_none())
    }

    // java: getName()Ljava/lang/String;
    pub fn getName(&self) -> Result<String> {
        let this = self;
        Ok(this.name.get())
    }

    // java: getClassLoader()Ljava/lang/ClassLoader;
    pub fn getClassLoader(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(SecurityConstants::GET_CLASSLOADER_PERMISSION())?;
        Ok(this.loader.get())
    }

    // java: getDescriptor()Ljava/lang/module/ModuleDescriptor;
    pub fn getDescriptor(&self) -> Result<Object> {
        let this = self;
        Ok(this.descriptor.get())
    }

    // java: getLayer()Ljava/lang/ModuleLayer;
    pub fn getLayer(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.isNamed()?;
        let mut layer: Object = this.layer.get();
        return Ok(layer);
        let _t1 = this.name.get().equals(String::from("java.base"))?;
        let _t2: Object = ModuleLayer::boot()?;
        return Ok(_t2);
        /* TODO: aconst_null  */
        Ok(_t1)
    }

    // java: implAddEnableNativeAccess()Ljava/lang/Module;
    pub fn implAddEnableNativeAccess(&self) -> Result<Object> {
        let this = self;
        let _t0: bool = Module_EnableNativeAccess::trySetEnableNativeAccess(this)?;
        Ok(this)
    }

    // java: isNativeAccessEnabled()Z
    pub fn isNativeAccessEnabled(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.moduleForNativeAccess()?;
        let mut target: Object = _t0;
        let _t1: bool = Module_EnableNativeAccess::isNativeAccessEnabled(target)?;
        Ok(_t1)
    }

    // java: moduleForNativeAccess()Ljava/lang/Module;
    pub fn moduleForNativeAccess(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.isNamed()?;
        Ok(Module::ALL_UNNAMED_MODULE())
    }

    // java: ensureNativeAccess(Ljava/lang/Class;Ljava/lang/String;)V
    pub fn ensureNativeAccess(&self, owner: Object, methodName: String) -> Result<()> {
        let this = self;
        let _t0 = this.moduleForNativeAccess()?;
        let mut target: Object = _t0;
        let _t1: bool = Module_EnableNativeAccess::isNativeAccessEnabled(target)?;
        let _t2: bool = ModuleBootstrap::hasEnableNativeAccessFlag()?;
        String::new().append(&String::from("Illegal native access from:"))?;
        String::new().append(&this)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t3: bool = Module_EnableNativeAccess::trySetEnableNativeAccess(target)?;
        let _t4 = owner.getName()?;
        let mut cls: String = _t4;
        String::new().append(&cls)?;
        String::new().append(&String::from("::"))?;
        String::new().append(&methodName)?;
        let mut mtd: String = String::new();
        let _t5 = this.isNamed()?;
        String::new().append(&String::from("module"))?;
        let _t6 = this.getName()?;
        String::new().append(&_t6)?;
        let mut mod_: String = String::from("the unnamed module");
        let _t7 = this.isNamed()?;
        let _t8 = this.getName()?;
        let mut modflag: String = String::from("ALL-UNNAMED");
        let mut _arr9: Vec<Object> = Vec::with_capacity(4i32 as usize);
        _arr9[0i32 as usize] = cls;
        _arr9[1i32 as usize] = mtd;
        _arr9[2i32 as usize] = mod_;
        _arr9[3i32 as usize] = modflag;
        let _t10 = System::err().printf(String::from("WARNING: A restricted method in %s has been called\nWARNING: %s has been called by %s\nWARNING: Use --enable-native-access=%s to avoid a warning for this module\n%n"), _arr9)?;
        Ok(())
    }

    // java: implAddEnableNativeAccessToAllUnnamed()V
    pub fn implAddEnableNativeAccessToAllUnnamed() -> Result<()> {
        let _t0: bool = Module_EnableNativeAccess::trySetEnableNativeAccess(Module::ALL_UNNAMED_MODULE())?;
        Ok(())
    }

    // java: canRead(Ljava/lang/Module;)Z
    pub fn canRead(&self, other: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(other)?;
        let _t1 = this.isNamed()?;
        return Ok(1i32);
        return Ok(1i32);
        let _t2 = other.isNamed()?;
        let mut reads: Object = this.reads.get();
        let _t3 = reads.contains(other)?;
        return Ok(1i32);
        let _t4 = Module_ReflectionData::reads().containsKeyPair(this, other)?;
        return Ok(1i32);
        let _t5 = other.isNamed()?;
        let _t6 = Module_ReflectionData::reads().containsKeyPair(this, Module::ALL_UNNAMED_MODULE())?;
        return Ok(1i32);
        Ok(0i32)
    }

    // java: addReads(Ljava/lang/Module;)Ljava/lang/Module;
    pub fn addReads(&self, other: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(other)?;
        let _t1 = this.isNamed()?;
        let _t2: Object = Reflection::getCallerClass()?;
        let _t3 = this.getCallerModule(_t2)?;
        let mut caller: Object = _t3;
        String::new().append(&caller)?;
        String::new().append(&String::from("!="))?;
        String::new().append(&this)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.implAddReads(other, 1i32)?;
        Ok(this)
    }

    // java: implAddReads(Ljava/lang/Module;)V
    // java: implAddReads(Ljava/lang/Module;)V
    pub fn implAddReads__module(&self, other: Object) -> Result<()> {
        let this = self;
        this.implAddReads(other, 1i32)?;
        Ok(())
    }

    // java: implAddReadsAllUnnamed()V
    pub fn implAddReadsAllUnnamed(&self) -> Result<()> {
        let this = self;
        this.implAddReads(Module::ALL_UNNAMED_MODULE(), 1i32)?;
        Ok(())
    }

    // java: implAddReadsNoSync(Ljava/lang/Module;)V
    pub fn implAddReadsNoSync(&self, other: Object) -> Result<()> {
        let this = self;
        this.implAddReads(other, 0i32)?;
        Ok(())
    }

    // java: implAddReads(Ljava/lang/Module;Z)V
    // java: implAddReads(Ljava/lang/Module;Z)V
    pub fn implAddReads__module_z(&self, other: Object, syncVM: bool) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(other)?;
        let _t1 = this.canRead(other)?;
        /* TODO: aconst_null  */
        Module::addReads0(Module::ALL_UNNAMED_MODULE(), this)?;
        Module::addReads0(this, other)?;
        let _t2 = Module_ReflectionData::reads().putIfAbsent(this, other, Boolean::TRUE())?;
        Ok(())
    }

    // java: isExported(Ljava/lang/String;Ljava/lang/Module;)Z
    // java: isExported(Ljava/lang/String;Ljava/lang/Module;)Z
    pub fn isExported__str_module(&self, pn: String, other: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(pn)?;
        let _t1: Object = Objects::requireNonNull(other)?;
        let _t2 = this.implIsExportedOrOpen(pn, other, 0i32)?;
        Ok(_t2)
    }

    // java: isOpen(Ljava/lang/String;Ljava/lang/Module;)Z
    // java: isOpen(Ljava/lang/String;Ljava/lang/Module;)Z
    pub fn isOpen__str_module(&self, pn: String, other: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(pn)?;
        let _t1: Object = Objects::requireNonNull(other)?;
        let _t2 = this.implIsExportedOrOpen(pn, other, 1i32)?;
        Ok(_t2)
    }

    // java: isExported(Ljava/lang/String;)Z
    // java: isExported(Ljava/lang/String;)Z
    pub fn isExported__str(&self, pn: String) -> Result<bool> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(pn)?;
        let _t1 = this.implIsExportedOrOpen(pn, Module::EVERYONE_MODULE(), 0i32)?;
        Ok(_t1)
    }

    // java: isOpen(Ljava/lang/String;)Z
    // java: isOpen(Ljava/lang/String;)Z
    pub fn isOpen__str(&self, pn: String) -> Result<bool> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(pn)?;
        let _t1 = this.implIsExportedOrOpen(pn, Module::EVERYONE_MODULE(), 1i32)?;
        Ok(_t1)
    }

    // java: implIsExportedOrOpen(Ljava/lang/String;Ljava/lang/Module;Z)Z
    pub fn implIsExportedOrOpen(&self, pn: String, other: Object, open: bool) -> Result<bool> {
        let this = self;
        let _t0 = this.isNamed()?;
        return Ok(1i32);
        let _t1 = this.descriptor.get().packages()?;
        let _t2 = _t1.contains(pn)?;
        return Ok(1i32);
        let _t3 = this.descriptor.get().isOpen()?;
        let _t4 = this.descriptor.get().isAutomatic()?;
        let _t5 = this.descriptor.get().packages()?;
        let _t6 = _t5.contains(pn)?;
        return Ok(_t6);
        let _t7 = this.isStaticallyExportedOrOpen(pn, other, open)?;
        return Ok(1i32);
        let _t8 = this.isReflectivelyExportedOrOpen(pn, other, open)?;
        return Ok(1i32);
        Ok(0i32)
    }

    // java: isStaticallyExportedOrOpen(Ljava/lang/String;Ljava/lang/Module;Z)Z
    pub fn isStaticallyExportedOrOpen(&self, pn: String, other: Object, open: bool) -> Result<bool> {
        let this = self;
        let mut openPackages: Object = this.openPackages.get();
        let _t0 = openPackages.get(pn)?;
        let _t1 = this.allows(_t0, other)?;
        return Ok(1i32);
        let mut exportedPackages: Object = this.exportedPackages.get();
        let _t2 = exportedPackages.get(pn)?;
        let _t3 = this.allows(_t2, other)?;
        return Ok(1i32);
        Ok(0i32)
    }

    // java: allows(Ljava/util/Set;Ljava/lang/Module;)Z
    pub fn allows(&self, targets: Object, module: Object) -> Result<bool> {
        let this = self;
        let _t0 = targets.contains(Module::EVERYONE_MODULE())?;
        return Ok(1i32);
        let _t1 = targets.contains(module)?;
        return Ok(1i32);
        let _t2 = module.isNamed()?;
        let _t3 = targets.contains(Module::ALL_UNNAMED_MODULE())?;
        return Ok(1i32);
        Ok(0i32)
    }

    // java: isReflectivelyExportedOrOpen(Ljava/lang/String;Ljava/lang/Module;Z)Z
    pub fn isReflectivelyExportedOrOpen(&self, pn: String, other: Object, open: bool) -> Result<bool> {
        let this = self;
        let _t0 = Module_ReflectionData::exports().get(this, Module::EVERYONE_MODULE())?;
        let mut exports: Object = _t0;
        let _t1 = exports.get(pn)?;
        let mut b: Object = _t1;
        let mut isOpen: i32 = b;
        return Ok(1i32);
        let _t2 = Module_ReflectionData::exports().get(this, other)?;
        exports = _t2;
        let _t3 = exports.get(pn)?;
        b = _t3;
        isOpen = b;
        return Ok(1i32);
        let _t4 = other.isNamed()?;
        let _t5 = Module_ReflectionData::exports().get(this, Module::ALL_UNNAMED_MODULE())?;
        exports = _t5;
        let _t6 = exports.get(pn)?;
        b = _t6;
        isOpen = b;
        return Ok(1i32);
        Ok(0i32)
    }

    // java: isReflectivelyExported(Ljava/lang/String;Ljava/lang/Module;)Z
    pub fn isReflectivelyExported(&self, pn: String, other: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.isReflectivelyExportedOrOpen(pn, other, 0i32)?;
        Ok(_t0)
    }

    // java: isReflectivelyOpened(Ljava/lang/String;Ljava/lang/Module;)Z
    pub fn isReflectivelyOpened(&self, pn: String, other: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.isReflectivelyExportedOrOpen(pn, other, 1i32)?;
        Ok(_t0)
    }

    // java: addExports(Ljava/lang/String;Ljava/lang/Module;)Ljava/lang/Module;
    pub fn addExports(&self, pn: String, other: Object) -> Result<Object> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: Object = Objects::requireNonNull(other)?;
        let _t1 = this.isNamed()?;
        let _t2: Object = Reflection::getCallerClass()?;
        let _t3 = this.getCallerModule(_t2)?;
        let mut caller: Object = _t3;
        String::new().append(&caller)?;
        String::new().append(&String::from("!="))?;
        String::new().append(&this)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.implAddExportsOrOpens(pn, other, 0i32, 1i32)?;
        Ok(this)
    }

    // java: addOpens(Ljava/lang/String;Ljava/lang/Module;)Ljava/lang/Module;
    pub fn addOpens(&self, pn: String, other: Object) -> Result<Object> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: Object = Objects::requireNonNull(other)?;
        let _t1 = this.isNamed()?;
        let _t2: Object = Reflection::getCallerClass()?;
        let _t3 = this.getCallerModule(_t2)?;
        let mut caller: Object = _t3;
        let _t4 = this.isOpen(pn, caller)?;
        String::new().append(&pn)?;
        String::new().append(&String::from("is not open to"))?;
        String::new().append(&caller)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.implAddExportsOrOpens(pn, other, 1i32, 1i32)?;
        Ok(this)
    }

    // java: implAddExports(Ljava/lang/String;)V
    // java: implAddExports(Ljava/lang/String;)V
    pub fn implAddExports__str(&self, pn: String) -> Result<()> {
        let this = self;
        this.implAddExportsOrOpens(pn, Module::EVERYONE_MODULE(), 0i32, 1i32)?;
        Ok(())
    }

    // java: implAddExports(Ljava/lang/String;Ljava/lang/Module;)V
    // java: implAddExports(Ljava/lang/String;Ljava/lang/Module;)V
    pub fn implAddExports__str_module(&self, pn: String, other: Object) -> Result<()> {
        let this = self;
        this.implAddExportsOrOpens(pn, other, 0i32, 1i32)?;
        Ok(())
    }

    // java: implAddExportsToAllUnnamed(Ljava/lang/String;)V
    pub fn implAddExportsToAllUnnamed(&self, pn: String) -> Result<()> {
        let this = self;
        this.implAddExportsOrOpens(pn, Module::ALL_UNNAMED_MODULE(), 0i32, 1i32)?;
        Ok(())
    }

    // java: implAddExportsNoSync(Ljava/lang/String;)V
    // java: implAddExportsNoSync(Ljava/lang/String;)V
    pub fn implAddExportsNoSync__str(&self, pn: String) -> Result<()> {
        let this = self;
        let _t0 = pn.replace(47i32, 46i32)?;
        this.implAddExportsOrOpens(_t0, Module::EVERYONE_MODULE(), 0i32, 0i32)?;
        Ok(())
    }

    // java: implAddExportsNoSync(Ljava/lang/String;Ljava/lang/Module;)V
    // java: implAddExportsNoSync(Ljava/lang/String;Ljava/lang/Module;)V
    pub fn implAddExportsNoSync__str_module(&self, pn: String, other: Object) -> Result<()> {
        let this = self;
        let _t0 = pn.replace(47i32, 46i32)?;
        this.implAddExportsOrOpens(_t0, other, 0i32, 0i32)?;
        Ok(())
    }

    // java: implAddOpens(Ljava/lang/String;)V
    // java: implAddOpens(Ljava/lang/String;)V
    pub fn implAddOpens__str(&self, pn: String) -> Result<()> {
        let this = self;
        this.implAddExportsOrOpens(pn, Module::EVERYONE_MODULE(), 1i32, 1i32)?;
        Ok(())
    }

    // java: implAddOpens(Ljava/lang/String;Ljava/lang/Module;)V
    // java: implAddOpens(Ljava/lang/String;Ljava/lang/Module;)V
    pub fn implAddOpens__str_module(&self, pn: String, other: Object) -> Result<()> {
        let this = self;
        this.implAddExportsOrOpens(pn, other, 1i32, 1i32)?;
        Ok(())
    }

    // java: implAddOpensToAllUnnamed(Ljava/lang/String;)V
    // java: implAddOpensToAllUnnamed(Ljava/lang/String;)V
    pub fn implAddOpensToAllUnnamed__str(&self, pn: String) -> Result<()> {
        let this = self;
        this.implAddExportsOrOpens(pn, Module::ALL_UNNAMED_MODULE(), 1i32, 1i32)?;
        Ok(())
    }

    // java: implAddExportsOrOpens(Ljava/lang/String;Ljava/lang/Module;ZZ)V
    pub fn implAddExportsOrOpens(&self, pn: String, other: Object, open: bool, syncVM: bool) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(other)?;
        let _t1: Object = Objects::requireNonNull(pn)?;
        let _t2 = this.isNamed()?;
        let _t3 = this.descriptor.get().isOpen()?;
        let _t4 = this.descriptor.get().isAutomatic()?;
        return Ok(());
        let _t5 = this.implIsExportedOrOpen(pn, other, open)?;
        return Ok(());
        let _t6 = this.descriptor.get().packages()?;
        let _t7 = _t6.contains(pn)?;
        String::new().append(&String::from("package"))?;
        String::new().append(&pn)?;
        String::new().append(&String::from("not in contents"))?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Module::addExportsToAll0(this, pn)?;
        Module::addExportsToAllUnnamed0(this, pn)?;
        Module::addExports0(this, pn, other)?;
        /* TODO: invokedynamic 305 */
        let _t8 = Module::ALL_UNNAMED_MODULE().computeIfAbsent(Module_ReflectionData::exports(), this, other)?;
        let mut map: Object = _t8;
        let _t9 = map.put(pn, Boolean::TRUE())?;
        let _t10 = map.putIfAbsent(pn, Boolean::FALSE())?;
        Ok(())
    }

    // java: implAddOpensToAllUnnamed(Ljava/util/Set;Ljava/util/Set;)V
    // java: implAddOpensToAllUnnamed(Ljava/util/Set;Ljava/util/Set;)V
    pub fn implAddOpensToAllUnnamed__set_set(&self, concealedPkgs: Object, exportedPkgs: Object) -> Result<()> {
        let this = self;
        let _t0: bool = VM::isModuleSystemInited()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut openPackages: Object = this.openPackages.get();
        let _t1 = concealedPkgs.size()?;
        let _t2 = exportedPkgs.size()?;
        let _t3: Object = HashMap::newHashMap((_t1).wrapping_add(_t2))?;
        openPackages = _t3;
        openPackages = HashMap::<_, _>::new()?;
        this.implAddOpensToAllUnnamed(concealedPkgs, openPackages)?;
        this.implAddOpensToAllUnnamed(exportedPkgs, openPackages)?;
        this.openPackages.set(openPackages);
        Ok(())
    }

    // java: implAddOpensToAllUnnamed(Ljava/util/Set;Ljava/util/Map;)V
    // java: implAddOpensToAllUnnamed(Ljava/util/Set;Ljava/util/Map;)V
    pub fn implAddOpensToAllUnnamed__set_map(&self, pkgs: Object, openPackages: Object) -> Result<()> {
        let this = self;
        let _t0 = pkgs.iterator()?;
        let mut local_3: Object = _t0;
        loop {
            let _t0 = local_3.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_3.next()?;
            let mut pn: Object = _t0;
            let _t1 = openPackages.putIfAbsent(pn, Module::ALL_UNNAMED_MODULE_SET())?;
            let mut prev: Object = _t1;
            let _t2 = prev.add(Module::ALL_UNNAMED_MODULE())?;
            Module::addExportsToAllUnnamed0(this, pn)?;
        }
        Ok(())
    }

    // java: addUses(Ljava/lang/Class;)Ljava/lang/Module;
    pub fn addUses(&self, service: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(service)?;
        let _t1 = this.isNamed()?;
        let _t2 = this.descriptor.get().isAutomatic()?;
        let _t3: Object = Reflection::getCallerClass()?;
        let _t4 = this.getCallerModule(_t3)?;
        let mut caller: Object = _t4;
        String::new().append(&caller)?;
        String::new().append(&String::from("!="))?;
        String::new().append(&this)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.implAddUses(service)?;
        Ok(this)
    }

    // java: implAddUses(Ljava/lang/Class;)V
    pub fn implAddUses(&self, service: Object) -> Result<()> {
        let this = self;
        let _t0 = this.canUse(service)?;
        let _t1 = Module_ReflectionData::uses().putIfAbsent(this, service, Boolean::TRUE())?;
        Ok(())
    }

    // java: canUse(Ljava/lang/Class;)Z
    pub fn canUse(&self, service: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(service)?;
        let _t1 = this.isNamed()?;
        return Ok(1i32);
        let _t2 = this.descriptor.get().isAutomatic()?;
        return Ok(1i32);
        let _t3 = this.descriptor.get().uses()?;
        let _t4 = service.getName()?;
        let _t5 = _t3.contains(_t4)?;
        return Ok(1i32);
        let _t6 = Module_ReflectionData::uses().containsKeyPair(this, service)?;
        Ok(_t6)
    }

    // java: getPackages()Ljava/util/Set;
    pub fn getPackages(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.isNamed()?;
        let _t1 = this.descriptor.get().packages()?;
        return Ok(_t1);
        let _t2: Object = BootLoader::packages()?;
        let mut packages: Object = _t2;
        let _t3 = this.loader.get().packages()?;
        packages = _t3;
        /* TODO: invokedynamic 388 */
        let _t4 = packages.filter(this)?;
        /* TODO: invokedynamic 398 */
        let _t5 = this.loader.get().map(_t4)?;
        let _t6: Object = Collectors::toSet()?;
        let _t7 = _t5.collect(_t6)?;
        Ok(_t7)
    }

    // java: defineModules(Ljava/lang/module/Configuration;Ljava/util/function/Function;Ljava/lang/ModuleLayer;)Ljava/util/Map;
    pub fn defineModules(cf: Object, clf: Object, layer: Object) -> Result<Object> {
        let _t0: Object = ModuleLayer::boot()?;
        let mut isBootLayer: i32 = _t0.is_none();
        let _t1 = cf.modules()?;
        let _t2 = _t1.size()?;
        let mut numModules: i32 = _t2;
        let _t3: Object = HashMap::newHashMap(numModules)?;
        let mut nameToModule: Object = _t3;
        let mut _arr4: Vec<Object> = Vec::with_capacity(numModules as usize);
        let mut resolvedModules: Vec<Object> = _arr4;
        let mut _arr5: Vec<Object> = Vec::with_capacity(numModules as usize);
        let mut modules: Vec<Object> = _arr5;
        let mut _arr6: Vec<Object> = Vec::with_capacity(numModules as usize);
        let mut classLoaders: Vec<Object> = _arr6;
        let _t7 = cf.modules()?;
        let _t8 = _t7.toArray(resolvedModules)?;
        resolvedModules = _t8;
        let mut toBindLoaders: HashSet<_> = HashSet::<_>::new()?;
        let mut hasPlatformModules: i32 = 0i32;
        let _t9: Object = ClassLoaders::platformClassLoader()?;
        let mut pcl: Object = _t9;
        let _t10: bool = ModuleLoaderMap::isBuiltinMapper(clf)?;
        let mut isModuleLoaderMapper: i32 = _t10;
        let mut index: i32 = 0i32;
        loop {
            if index >= numModules { break; }
            let _t0 = resolvedModules[index as usize].clone().name()?;
            let mut name: String = _t0;
            let _t1 = clf.apply(name)?;
            let mut loader: Object = _t1;
            return Err(JvmError::Custom("athrow".to_owned()));
            hasPlatformModules = 1i32;
            let _t2 = toBindLoaders.add(loader)?;
            classLoaders[index as usize] = loader;
            index = index.wrapping_add(1i32);
        }
        index = 0i32;
        loop {
            if index >= numModules { break; }
            let _t0 = resolvedModules[index as usize].clone().reference()?;
            name = _t0;
            let _t1 = name.descriptor()?;
            loader = _t1;
            let _t2 = loader.name()?;
            let mut name: String = _t2;
            let mut loader: Object = classLoaders[index as usize].clone();
            let _t3 = name.equals(String::from("java.base"))?;
            let _t4 = 2i32.getModule()?;
            let mut m: Object = _t4;
            let _t5 = name.location()?;
            /* TODO: aconst_null  */
            let _t6 = _t3.orElse(_t5)?;
            let mut uri: Object = _t6;
            m = Module::new(layer, loader, loader, uri)?;
            let _t7 = nameToModule.put(name, m)?;
            modules[index as usize] = m;
            index = index.wrapping_add(1i32);
        }
        index = 0i32;
        loop {
            if index >= numModules { break; }
            name = resolvedModules[index as usize].clone();
            let _t0 = name.reference()?;
            loader = _t0;
            let _t1 = loader.descriptor()?;
            name = _t1;
            loader = modules[index as usize].clone();
            m = HashSet::<_>::new()?;
            let _t2: Object = Map::of()?;
            uri = _t2;
            let _t3 = name.reads()?;
            let _t4 = _t3.iterator()?;
            let mut loader: Object = _t4;
            let _t5 = loader.hasNext()?;
            let _t6 = loader.next()?;
            let mut other: Object = _t6;
            /* TODO: aconst_null  */
            let mut m2: bool = _t5;
            let _t7 = other.configuration()?;
            let _t8 = other.name()?;
            let _t9 = nameToModule.get(_t8)?;
            m2 = _t9;
            return Err(JvmError::Custom("athrow".to_owned()));
            let _t10 = layer.parents()?;
            let _t11 = _t10.iterator()?;
            let mut local_23: Object = _t11;
            let _t12 = local_23.hasNext()?;
            let _t13 = local_23.next()?;
            let mut parent: Object = _t13;
            let _t14: Object = Module::findModule(parent, other)?;
            m2 = _t14;
            return Err(JvmError::Custom("athrow".to_owned()));
            let _t15 = uri.isEmpty()?;
            uri = HashMap::<_, _>::new()?;
            let _t16 = other.name()?;
            let _t17 = uri.put(_t16, m2)?;
            let _t18 = m.add(m2)?;
            Module::addReads0(loader, m2)?;
            loader.reads.set(m);
            let _t19 = name.isAutomatic()?;
            loader.implAddReads(Module::ALL_UNNAMED_MODULE(), 1i32)?;
            let _t20 = name.isOpen()?;
            let _t21 = name.isAutomatic()?;
            let _t22 = name.opens()?;
            let _t23 = _t22.isEmpty()?;
            Module::initExports(loader, nameToModule)?;
            let _t24 = layer.parents()?;
            Module::initExportsAndOpens(loader, uri, nameToModule, _t24)?;
            index = index.wrapping_add(1i32);
        }
        let _t11: Object = BootLoader::getServicesCatalog()?;
        index = _t11;
        let _t12: Object = ServicesCatalog::getServicesCatalog(pcl)?;
        name = _t12;
        loader = 0i32;
        loop {
            if loader >= numModules { break; }
            name = resolvedModules[loader as usize].clone();
            let _t0 = name.reference()?;
            loader = _t0;
            let _t1 = loader.descriptor()?;
            m = _t1;
            let _t2 = m.provides()?;
            let _t3 = _t2.isEmpty()?;
            uri = modules[loader as usize].clone();
            loader = classLoaders[loader as usize].clone();
            index.register(uri)?;
            name.register(uri)?;
            loader = loader.wrapping_add(1i32);
        }
        let _t13 = toBindLoaders.iterator()?;
        index = _t13;
        loop {
            let _t0 = index.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = index.next()?;
            name = _t0;
            layer.bindToLoader(name)?;
        }
        Ok(nameToModule)
    }

    // java: findModule(Ljava/lang/ModuleLayer;Ljava/lang/module/ResolvedModule;)Ljava/lang/Module;
    // java: findModule(Ljava/lang/ModuleLayer;Ljava/lang/module/ResolvedModule;)Ljava/lang/Module;
    pub fn findModule__module_resolv(parent: Object, resolvedModule: Object) -> Result<Object> {
        let _t0 = resolvedModule.configuration()?;
        let mut cf: Object = _t0;
        let _t1 = resolvedModule.name()?;
        let mut dn: String = _t1;
        let _t2 = parent.layers()?;
        /* TODO: invokedynamic 535 */
        let _t3 = _t2.filter(cf)?;
        let _t4 = _t3.findAny()?;
        /* TODO: invokedynamic 541 */
        let _t5 = _t4.map(dn)?;
        /* TODO: aconst_null  */
        let _t6 = todo!("stack underflow").orElse(_t5)?;
        Ok(_t6)
    }

    // java: initExports(Ljava/lang/Module;Ljava/util/Map;)V
    pub fn initExports(m: Object, nameToModule: Object) -> Result<()> {
        let mut exportedPackages: HashMap<_, _> = HashMap::<_, _>::new()?;
        let _t0 = m.getDescriptor()?;
        let _t1 = _t0.exports()?;
        let _t2 = _t1.iterator()?;
        let mut local_3: Object = _t2;
        loop {
            let _t0 = local_3.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_3.next()?;
            let mut exports: Object = _t0;
            let _t1 = exports.source()?;
            let mut source: String = _t1;
            let _t2 = exports.isQualified()?;
            let mut targets: HashSet<_> = HashSet::<_>::new()?;
            let _t3 = exports.targets()?;
            let _t4 = _t3.iterator()?;
            let mut local_7: Object = _t4;
            let _t5 = local_7.hasNext()?;
            let _t6 = local_7.next()?;
            let mut target: Object = _t6;
            let _t7 = nameToModule.get(target)?;
            let mut m2: Object = _t7;
            Module::addExports0(m, source, m2)?;
            let _t8 = targets.add(m2)?;
            let _t9 = targets.isEmpty()?;
            let _t10 = exportedPackages.put(source, targets)?;
            Module::addExportsToAll0(m, source)?;
            let _t11 = exportedPackages.put(source, Module::EVERYONE_SET())?;
        }
        let _t3 = exportedPackages.isEmpty()?;
        m.exportedPackages.set(exportedPackages);
        Ok(())
    }

    // java: initExportsAndOpens(Ljava/lang/Module;Ljava/util/Map;Ljava/util/Map;Ljava/util/List;)V
    pub fn initExportsAndOpens(m: Object, nameToSource: Object, nameToModule: Object, parents: Object) -> Result<()> {
        let _t0 = m.getDescriptor()?;
        let mut descriptor: Object = _t0;
        let mut openPackages: HashMap<_, _> = HashMap::<_, _>::new()?;
        let mut exportedPackages: HashMap<_, _> = HashMap::<_, _>::new()?;
        let _t1 = descriptor.opens()?;
        let _t2 = _t1.iterator()?;
        let mut local_7: Object = _t2;
        loop {
            let _t0 = local_7.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_7.next()?;
            let mut opens: Object = _t0;
            let _t1 = opens.source()?;
            let mut source: String = _t1;
            let _t2 = opens.isQualified()?;
            let mut targets: HashSet<_> = HashSet::<_>::new()?;
            let _t3 = opens.targets()?;
            let _t4 = _t3.iterator()?;
            let mut targets: Object = _t4;
            let _t5 = targets.hasNext()?;
            let _t6 = targets.next()?;
            let mut target: Object = _t6;
            let _t7: Object = Module::findModule(target, nameToSource, nameToModule, parents)?;
            let mut m2: Object = _t7;
            Module::addExports0(m, source, m2)?;
            let _t8 = targets.add(m2)?;
            let _t9 = targets.isEmpty()?;
            let _t10 = openPackages.put(source, targets)?;
            Module::addExportsToAll0(m, source)?;
            let _t11 = openPackages.put(source, Module::EVERYONE_SET())?;
        }
        let _t3 = descriptor.exports()?;
        let _t4 = _t3.iterator()?;
        local_7 = _t4;
        loop {
            let _t0 = local_7.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_7.next()?;
            opens = _t0;
            let _t1 = opens.source()?;
            source = _t1;
            let _t2 = openPackages.get(source)?;
            targets = _t2;
            let _t3 = targets.contains(Module::EVERYONE_MODULE())?;
            let _t4 = opens.isQualified()?;
            targets = HashSet::<_>::new()?;
            let _t5 = opens.targets()?;
            let _t6 = _t5.iterator()?;
            target = _t6;
            let _t7 = target.hasNext()?;
            let _t8 = target.next()?;
            m2 = _t8;
            let _t9: Object = Module::findModule(m2, nameToSource, nameToModule, parents)?;
            let mut m2: Object = _t9;
            let _t10 = targets.contains(m2)?;
            Module::addExports0(m, source, m2)?;
            let _t11 = targets.add(m2)?;
            let _t12 = targets.isEmpty()?;
            let _t13 = exportedPackages.put(source, targets)?;
            Module::addExportsToAll0(m, source)?;
            let _t14 = exportedPackages.put(source, Module::EVERYONE_SET())?;
        }
        let _t5 = openPackages.isEmpty()?;
        m.openPackages.set(openPackages);
        let _t6 = exportedPackages.isEmpty()?;
        m.exportedPackages.set(exportedPackages);
        Ok(())
    }

    // java: findModule(Ljava/lang/String;Ljava/util/Map;Ljava/util/Map;Ljava/util/List;)Ljava/lang/Module;
    // java: findModule(Ljava/lang/String;Ljava/util/Map;Ljava/util/Map;Ljava/util/List;)Ljava/lang/Module;
    pub fn findModule__str_map_map_list(target: String, nameToSource: Object, nameToModule: Object, parents: Object) -> Result<Object> {
        let _t0 = nameToSource.get(target)?;
        let mut m: Object = _t0;
        let _t1 = nameToModule.get(target)?;
        m = _t1;
        let _t2 = parents.iterator()?;
        let mut local_5: Object = _t2;
        loop {
            let _t0 = local_5.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_5.next()?;
            let mut parent: Object = _t0;
            let _t1 = parent.findModule(target)?;
            /* TODO: aconst_null  */
            let _t2 = todo!("stack underflow").orElse(_t1)?;
            m = _t2;
        }
        Ok(m)
    }

    // java: getAnnotation(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;
    pub fn getAnnotation(&self, annotationClass: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.moduleInfoClass()?;
        let _t1 = _t0.getDeclaredAnnotation(annotationClass)?;
        Ok(_t1)
    }

    // java: getAnnotations()[Ljava/lang/annotation/Annotation;
    pub fn getAnnotations(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.moduleInfoClass()?;
        let _t1 = _t0.getAnnotations()?;
        Ok(_t1)
    }

    // java: getDeclaredAnnotations()[Ljava/lang/annotation/Annotation;
    pub fn getDeclaredAnnotations(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.moduleInfoClass()?;
        let _t1 = _t0.getDeclaredAnnotations()?;
        Ok(_t1)
    }

    // java: moduleInfoClass()Ljava/lang/Class;
    pub fn moduleInfoClass(&self) -> Result<Object> {
        let this = self;
        let mut clazz: Object = this.moduleInfoClass.get();
        return Ok(clazz);
        let mut local_2: Module = this;
        /* TODO: monitorenter  */
        clazz = this.moduleInfoClass.get();
        let _t0 = this.isNamed()?;
        /* TODO: invokedynamic 594 */
        let mut pa: Module = this;
        let _t1: Object = AccessController::doPrivileged(pa)?;
        clazz = _t1;
        clazz = 604i32;
        this.moduleInfoClass.set(clazz);
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_4: Object = clazz;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: loadModuleInfoClass()Ljava/lang/Class;
    // java: loadModuleInfoClass()Ljava/lang/Class;
    pub fn loadModuleInfoClass(&self) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        let mut clazz: i32 = todo!("stack underflow");
        let _t0 = this.getResourceAsStream(String::from("module-info.class"))?;
        let mut in_: Object = _t0;
        let _t1 = this.loadModuleInfoClass(in_)?;
        clazz = _t1;
        in_.close()?;
        let mut local_3: Object = in_;
        in_.close()?;
        let mut local_4: Object = in_;
        local_3.addSuppressed(local_4)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        in_ = in_;
        Ok(clazz)
    }

    // java: loadModuleInfoClass(Ljava/io/InputStream;)Ljava/lang/Class;
    // java: loadModuleInfoClass(Ljava/io/InputStream;)Ljava/lang/Class;
    pub fn loadModuleInfoClass__inputs(&self, in_: Object) -> Result<Object> {
        let this = self;
        let mut MODULE_INFO: String = String::from("module-info");
        let _t0 = in_.readAllBytes()?;
        let mut _arr1: Vec<Object> = Vec::with_capacity(1i32 as usize);
        let _t2: Object = Classfile_Option::constantPoolSharing(0i32)?;
        _arr1[0i32 as usize] = _t2;
        let _t3: Object = Classfile::parse(&_t0, &_arr1)?;
        /* TODO: invokedynamic 647 */
        let _t4 = todo!("stack underflow").transform(_t3)?;
        let mut bytes: Vec<i8> = _t4;
        let mut cl: Module_1 = Module_1::new(this, this.loader.get(), bytes)?;
        let _t5 = cl.loadClass(String::from("module-info"))?;
        return Ok(_t5);
        let mut e: i32 = todo!("stack underflow");
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: getResourceAsStream(Ljava/lang/String;)Ljava/io/InputStream;
    pub fn getResourceAsStream(&self, name: String) -> Result<Object> {
        let this = self;
        let _t0 = name.startsWith(String::from("/"))?;
        let _t1 = name.substring(1i32)?;
        name = _t1;
        let _t2 = this.isNamed()?;
        let _t3: bool = Resources::canEncapsulate(name)?;
        let _t4: Object = Reflection::getCallerClass()?;
        let _t5 = this.getCallerModule(_t4)?;
        let mut caller: Object = _t5;
        let _t6 = 2i32.getModule()?;
        let _t7: String = Resources::toPackageName(name)?;
        let mut pn: String = _t7;
        let _t8 = this.getPackages()?;
        let _t9 = _t8.contains(pn)?;
        let _t10 = this.isOpen(pn)?;
        /* TODO: aconst_null  */
        return Ok(_t10);
        let _t11 = this.isOpen(pn, caller)?;
        /* TODO: aconst_null  */
        return Ok(_t11);
        caller = this.name.get();
        let _t12: Object = BootLoader::findResourceAsStream(caller, name)?;
        return Ok(_t12);
        let _t13 = this.loader.get().findResourceAsStream(caller, name)?;
        return Ok(_t13);
        let _t14 = this.loader.get().findResource(caller, name)?;
        pn = _t14;
        let _t15 = pn.openStream()?;
        return Ok(_t15);
        let mut local_4: String = pn;
        /* TODO: aconst_null  */
        Ok(true)
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.isNamed()?;
        String::new().append(&String::from("module"))?;
        String::new().append(&this.name.get())?;
        return Ok(String::new());
        let _t1: i32 = System::identityHashCode(this)?;
        let _t2: String = Integer::toHexString(_t1)?;
        let mut id: String = _t2;
        String::new().append(&String::from("unnamed module @"))?;
        String::new().append(&id)?;
        Ok(String::new())
    }

    // java: getCallerModule(Ljava/lang/Class;)Ljava/lang/Module;
    pub fn getCallerModule(&self, caller: Object) -> Result<Object> {
        let this = self;
        let _t0 = caller.getModule()?;
        /* TODO: aconst_null  */
        Ok(_t0)
    }

    // java: defineModule0(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V
    pub fn defineModule0(arg0: Object, arg1: bool, arg2: String, arg3: String, arg4: Vec<Object>) -> Result<()> {
        todo!("native java/lang/Module.defineModule0")
    }

    // java: addReads0(Ljava/lang/Module;Ljava/lang/Module;)V
    pub fn addReads0(arg0: Object, arg1: Object) -> Result<()> {
        todo!("native java/lang/Module.addReads0")
    }

    // java: addExports0(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V
    pub fn addExports0(arg0: Object, arg1: String, arg2: Object) -> Result<()> {
        todo!("native java/lang/Module.addExports0")
    }

    // java: addExportsToAll0(Ljava/lang/Module;Ljava/lang/String;)V
    pub fn addExportsToAll0(arg0: Object, arg1: String) -> Result<()> {
        todo!("native java/lang/Module.addExportsToAll0")
    }

    // java: addExportsToAllUnnamed0(Ljava/lang/Module;Ljava/lang/String;)V
    pub fn addExportsToAllUnnamed0(arg0: Object, arg1: String) -> Result<()> {
        todo!("native java/lang/Module.addExportsToAllUnnamed0")
    }
}
