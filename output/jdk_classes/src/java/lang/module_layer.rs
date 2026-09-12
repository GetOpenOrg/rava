#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/ModuleLayer",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "ModuleLayer.java",
))]
pub struct ModuleLayer {
    #[cfg_attr(any(), java_field(name = "cf", descriptor = "Ljava/lang/module/Configuration;", access = "private final"))]
    pub cf: Field<Object>,
    #[cfg_attr(any(), java_field(name = "parents", descriptor = "Ljava/util/List;", access = "private final"))]
    pub parents: Field<Object>,
    #[cfg_attr(any(), java_field(name = "nameToModule", descriptor = "Ljava/util/Map;", access = "private final"))]
    pub nameToModule: Field<Object>,
    #[cfg_attr(any(), java_field(name = "allLayers", descriptor = "Ljava/util/List;", access = "private"))]
    pub allLayers: Field<Object>,
    #[cfg_attr(any(), java_field(name = "modules", descriptor = "Ljava/util/Set;", access = "private"))]
    pub modules: Field<Object>,
    #[cfg_attr(any(), java_field(name = "servicesCatalog", descriptor = "Ljdk/internal/module/ServicesCatalog;", access = "private"))]
    pub servicesCatalog: Field<Object>,
}

impl ModuleLayer {
    // java: <init>(Ljava/lang/module/Configuration;Ljava/util/List;Ljava/util/function/Function;)V
    pub fn new(cf: Object, parents: Object, clf: Object) -> Result<Self> {
        let this = Self { cf: Field::new(Default::default()), parents: Field::new(Default::default()), nameToModule: Field::new(Default::default()), allLayers: Field::new(Default::default()), modules: Field::new(Default::default()), servicesCatalog: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.cf.set(cf);
        this.parents.set(parents);
        let _t0 = parents.isEmpty()?;
        let _t1: Object = Map::of()?;
        let mut map: Object = _t1;
        let _t2: Object = Module::defineModules(cf, clf, this)?;
        map = _t2;
        this.nameToModule.set(map);
        Ok(this)
    }

    // java: defineModulesWithOneLoader(Ljava/lang/module/Configuration;Ljava/lang/ClassLoader;)Ljava/lang/ModuleLayer;
    // java: defineModulesWithOneLoader(Ljava/lang/module/Configuration;Ljava/lang/ClassLoader;)Ljava/lang/ModuleLayer;
    pub fn defineModulesWithOneLoader__config_classl(&self, cf: Object, parentLoader: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = List::of(this)?;
        let _t1: Object = ModuleLayer::defineModulesWithOneLoader(cf, _t0, parentLoader)?;
        let _t2 = _t1.layer()?;
        Ok(_t2)
    }

    // java: defineModulesWithManyLoaders(Ljava/lang/module/Configuration;Ljava/lang/ClassLoader;)Ljava/lang/ModuleLayer;
    // java: defineModulesWithManyLoaders(Ljava/lang/module/Configuration;Ljava/lang/ClassLoader;)Ljava/lang/ModuleLayer;
    pub fn defineModulesWithManyLoaders__config_classl(&self, cf: Object, parentLoader: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = List::of(this)?;
        let _t1: Object = ModuleLayer::defineModulesWithManyLoaders(cf, _t0, parentLoader)?;
        let _t2 = _t1.layer()?;
        Ok(_t2)
    }

    // java: defineModules(Ljava/lang/module/Configuration;Ljava/util/function/Function;)Ljava/lang/ModuleLayer;
    // java: defineModules(Ljava/lang/module/Configuration;Ljava/util/function/Function;)Ljava/lang/ModuleLayer;
    pub fn defineModules__config_functi(&self, cf: Object, clf: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = List::of(this)?;
        let _t1: Object = ModuleLayer::defineModules(cf, _t0, clf)?;
        let _t2 = _t1.layer()?;
        Ok(_t2)
    }

    // java: defineModulesWithOneLoader(Ljava/lang/module/Configuration;Ljava/util/List;Ljava/lang/ClassLoader;)Ljava/lang/ModuleLayer$Controller;
    // java: defineModulesWithOneLoader(Ljava/lang/module/Configuration;Ljava/util/List;Ljava/lang/ClassLoader;)Ljava/lang/ModuleLayer$Controller;
    pub fn defineModulesWithOneLoader__config_list_classl(cf: Object, parentLayers: Object, parentLoader: Object) -> Result<Object> {
        let _t0: Object = List::copyOf(parentLayers)?;
        let mut parents: Object = _t0;
        ModuleLayer::checkConfiguration(cf, parents)?;
        ModuleLayer::checkCreateClassLoaderPermission()?;
        ModuleLayer::checkGetClassLoaderPermission()?;
        let _t1 = cf.modules()?;
        let mut loader: Loader = Loader::new(_t1, parentLoader)?;
        let _t2 = loader.initRemotePackageMap(cf, parents)?;
        /* TODO: invokedynamic 87 */
        let mut layer: ModuleLayer = ModuleLayer::new(cf, parents, loader)?;
        return Ok(ModuleLayer_Controller::new(layer)?);
        loader = todo!("stack underflow");
        let _t3 = loader.getMessage()?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: defineModulesWithManyLoaders(Ljava/lang/module/Configuration;Ljava/util/List;Ljava/lang/ClassLoader;)Ljava/lang/ModuleLayer$Controller;
    // java: defineModulesWithManyLoaders(Ljava/lang/module/Configuration;Ljava/util/List;Ljava/lang/ClassLoader;)Ljava/lang/ModuleLayer$Controller;
    pub fn defineModulesWithManyLoaders__config_list_classl(cf: Object, parentLayers: Object, parentLoader: Object) -> Result<Object> {
        let _t0: Object = List::copyOf(parentLayers)?;
        let mut parents: Object = _t0;
        ModuleLayer::checkConfiguration(cf, parents)?;
        ModuleLayer::checkCreateClassLoaderPermission()?;
        ModuleLayer::checkGetClassLoaderPermission()?;
        let mut pool: LoaderPool = LoaderPool::new(cf, parents, parentLoader)?;
        let _t1: Object = Objects::requireNonNull(pool)?;
        /* TODO: invokedynamic 123 */
        let mut layer: ModuleLayer = ModuleLayer::new(cf, parents, pool)?;
        return Ok(ModuleLayer_Controller::new(layer)?);
        layer = todo!("stack underflow");
        let _t2 = layer.getMessage()?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: defineModules(Ljava/lang/module/Configuration;Ljava/util/List;Ljava/util/function/Function;)Ljava/lang/ModuleLayer$Controller;
    // java: defineModules(Ljava/lang/module/Configuration;Ljava/util/List;Ljava/util/function/Function;)Ljava/lang/ModuleLayer$Controller;
    pub fn defineModules__config_list_functi(cf: Object, parentLayers: Object, clf: Object) -> Result<Object> {
        let _t0: Object = List::copyOf(parentLayers)?;
        let mut parents: Object = _t0;
        ModuleLayer::checkConfiguration(cf, parents)?;
        let _t1: Object = Objects::requireNonNull(clf)?;
        ModuleLayer::checkGetClassLoaderPermission()?;
        let _t2: Object = ModuleLayer::boot()?;
        ModuleLayer::checkForDuplicatePkgs(cf, clf)?;
        let mut layer: ModuleLayer = ModuleLayer::new(cf, parents, clf)?;
        return Ok(ModuleLayer_Controller::new(layer)?);
        layer = _t2;
        let _t3 = layer.getMessage()?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: checkConfiguration(Ljava/lang/module/Configuration;Ljava/util/List;)V
    pub fn checkConfiguration(cf: Object, parentLayers: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull(cf)?;
        let _t1 = cf.parents()?;
        let mut parentConfigurations: Object = _t1;
        let _t2 = parentLayers.size()?;
        let _t3 = parentConfigurations.size()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut index: i32 = 0i32;
        let _t4 = parentLayers.iterator()?;
        let mut local_4: Object = _t4;
        loop {
            let _t0 = local_4.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_4.next()?;
            let mut parent: Object = _t0;
            let _t1 = parent.configuration()?;
            let _t2 = parentConfigurations.get(index)?;
            return Err(JvmError::Custom("athrow".to_owned()));
            index = index.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: checkCreateClassLoaderPermission()V
    pub fn checkCreateClassLoaderPermission() -> Result<()> {
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(SecurityConstants::CREATE_CLASSLOADER_PERMISSION())?;
        Ok(())
    }

    // java: checkGetClassLoaderPermission()V
    pub fn checkGetClassLoaderPermission() -> Result<()> {
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(SecurityConstants::GET_CLASSLOADER_PERMISSION())?;
        Ok(())
    }

    // java: checkForDuplicatePkgs(Ljava/lang/module/Configuration;Ljava/util/function/Function;)V
    pub fn checkForDuplicatePkgs(cf: Object, clf: Object) -> Result<()> {
        let mut loaderToPackages: HashMap<_, _> = HashMap::<_, _>::new()?;
        let _t0 = cf.modules()?;
        let _t1 = _t0.iterator()?;
        let mut local_3: Object = _t1;
        loop {
            let _t0 = local_3.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_3.next()?;
            let mut resolvedModule: Object = _t0;
            let _t1 = resolvedModule.reference()?;
            let _t2 = _t1.descriptor()?;
            let mut descriptor: Object = _t2;
            let _t3 = descriptor.name()?;
            let _t4 = clf.apply(_t3)?;
            let mut loader: Object = _t4;
            /* TODO: invokedynamic 216 */
            let _t5 = todo!("stack underflow").computeIfAbsent(loaderToPackages, loader)?;
            let mut loaderPackages: Object = _t5;
            let _t6 = descriptor.packages()?;
            let _t7 = _t6.iterator()?;
            let mut local_8: Object = _t7;
            let _t8 = local_8.hasNext()?;
            let _t9 = local_8.next()?;
            let mut pkg: Object = _t9;
            let _t10 = loaderPackages.add(pkg)?;
            let mut added: i32 = _t10;
            let mut _arr11: Vec<Object> = Vec::with_capacity(1i32 as usize);
            _arr11[0i32 as usize] = pkg;
            let _t12: Object = ModuleLayer::fail(String::from("More than one module with package %s mapped to the same class loader"), &_arr11)?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        Ok(())
    }

    // java: fail(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/LayerInstantiationException;
    pub fn fail(fmt: String, args: &[Object]) -> Result<Object> {
        let _t0: String = String::format(fmt, &args)?;
        let mut msg: String = _t0;
        Ok(LayerInstantiationException::new(msg)?)
    }

    // java: configuration()Ljava/lang/module/Configuration;
    pub fn configuration(&self) -> Result<Object> {
        let this = self;
        Ok(this.cf.get())
    }

    // java: parents()Ljava/util/List;
    pub fn parents(&self) -> Result<Object> {
        let this = self;
        Ok(this.parents.get())
    }

    // java: layers()Ljava/util/stream/Stream;
    // java: layers()Ljava/util/stream/Stream;
    pub fn layers(&self) -> Result<Object> {
        let this = self;
        let mut allLayers: Object = this.allLayers.get();
        let _t0 = allLayers.stream()?;
        return Ok(_t0);
        allLayers = ArrayList::<_>::new()?;
        let mut visited: HashSet<_> = HashSet::<_>::new()?;
        let mut stack: ArrayDeque = ArrayDeque::new()?;
        let _t1 = visited.add(this)?;
        stack.push(this)?;
        loop {
            let _t0 = stack.isEmpty()?;
            if _t0!=0i32 { break; }
            let _t0 = stack.pop()?;
            let mut layer: Object = _t0;
            let _t1 = allLayers.add(layer)?;
            let _t2 = layer.parents.get().size()?;
            let mut i: i32 = (_t2).wrapping_sub(1i32);
            let _t3 = layer.parents.get().get(i)?;
            let mut parent: Object = _t3;
            let _t4 = visited.add(parent)?;
            stack.push(parent)?;
            i = i.wrapping_sub(1i32);
        }
        let _t2: Object = Collections::unmodifiableList(allLayers)?;
        allLayers = _t2;
        this.allLayers.set(_t2);
        let _t3 = allLayers.stream()?;
        Ok(_t3)
    }

    // java: modules()Ljava/util/Set;
    pub fn modules(&self) -> Result<Object> {
        let this = self;
        let mut modules: Object = this.modules.get();
        let _t0 = this.nameToModule.get().values()?;
        let _t1: Object = Set::copyOf(_t0)?;
        modules = _t1;
        this.modules.set(_t1);
        Ok(modules)
    }

    // java: findModule(Ljava/lang/String;)Ljava/util/Optional;
    pub fn findModule(&self, name: String) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(name)?;
        let _t1: Object = Optional::empty()?;
        return Ok(_t1);
        let _t2 = this.nameToModule.get().get(name)?;
        let mut m: Object = _t2;
        let _t3: Object = Optional::of(m)?;
        return Ok(_t3);
        let _t4 = this.layers()?;
        let _t5 = _t4.skip(1i64)?;
        /* TODO: invokedynamic 309 */
        let _t6 = _t5.map(name)?;
        /* TODO: invokedynamic 316 */
        let _t7 = m.filter(_t6)?;
        let _t8 = _t7.findAny()?;
        Ok(_t8)
    }

    // java: findLoader(Ljava/lang/String;)Ljava/lang/ClassLoader;
    pub fn findLoader(&self, name: String) -> Result<Object> {
        let this = self;
        let _t0 = this.findModule(name)?;
        let mut om: Object = _t0;
        let _t1 = om.isPresent()?;
        let _t2 = om.get()?;
        let _t3 = _t2.getClassLoader()?;
        return Ok(_t3);
        String::new().append(&String::from("Module"))?;
        String::new().append(&name)?;
        String::new().append(&String::from("not known to this layer"))?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.modules()?;
        let _t1 = _t0.stream()?;
        /* TODO: invokedynamic 356 */
        let _t2 = todo!("stack underflow").map(_t1)?;
        let _t3: Object = Collectors::joining(String::from(","))?;
        let _t4 = _t2.collect(_t3)?;
        Ok(_t4)
    }

    // java: empty()Ljava/lang/ModuleLayer;
    pub fn empty() -> Result<Object> {
        Ok(ModuleLayer::EMPTY_LAYER())
    }

    // java: boot()Ljava/lang/ModuleLayer;
    pub fn boot() -> Result<Object> {
        Ok(System::bootLayer())
    }

    // java: getServicesCatalog()Ljdk/internal/module/ServicesCatalog;
    pub fn getServicesCatalog(&self) -> Result<Object> {
        let this = self;
        let mut servicesCatalog: Object = this.servicesCatalog.get();
        return Ok(servicesCatalog);
        let mut local_2: ModuleLayer = this;
        /* TODO: monitorenter  */
        servicesCatalog = this.servicesCatalog.get();
        let _t0: Object = ServicesCatalog::create()?;
        servicesCatalog = _t0;
        let _t1 = this.nameToModule.get().values()?;
        let _t2 = _t1.iterator()?;
        let mut local_3: Object = _t2;
        loop {
            let _t0 = local_3.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_3.next()?;
            let mut m: Object = _t0;
            servicesCatalog.register(m)?;
        }
        this.servicesCatalog.set(servicesCatalog);
        /* TODO: monitorexit  */
        let mut local_5: ModuleLayer = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(servicesCatalog)
    }

    // java: bindToLoader(Ljava/lang/ClassLoader;)V
    pub fn bindToLoader(&self, loader: Object) -> Result<()> {
        let this = self;
        let _t0 = ModuleLayer::CLV().get(loader)?;
        let mut list: Object = _t0;
        list = CopyOnWriteArrayList::new()?;
        let _t1 = ModuleLayer::CLV().putIfAbsent(loader, list)?;
        let mut previous: Object = _t1;
        list = previous;
        let _t2 = list.add(this)?;
        Ok(())
    }

    // java: layers(Ljava/lang/ClassLoader;)Ljava/util/stream/Stream;
    // java: layers(Ljava/lang/ClassLoader;)Ljava/util/stream/Stream;
    pub fn layers__classl(loader: Object) -> Result<Object> {
        let _t0 = ModuleLayer::CLV().get(loader)?;
        let mut list: Object = _t0;
        let _t1 = list.stream()?;
        return Ok(_t1);
        let _t2: Object = Stream::empty()?;
        Ok(_t2)
    }
}
