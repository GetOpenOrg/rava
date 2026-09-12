#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/System$2",
    super_class = "java/lang/Object",
    interfaces  = "jdk/internal/access/JavaLangAccess",
    access      = "",
    source      = "System.java",
))]
pub struct System_2;

impl System_2 {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: getDeclaredPublicMethods(Ljava/lang/Class;Ljava/lang/String;[Ljava/lang/Class;)Ljava/util/List;
    pub fn getDeclaredPublicMethods(&self, klass: Object, name: String, parameterTypes: Vec<Object>) -> Result<Object> {
        let this = self;
        let _t0 = klass.getDeclaredPublicMethods(name, parameterTypes)?;
        Ok(_t0)
    }

    // java: getConstantPool(Ljava/lang/Class;)Ljdk/internal/reflect/ConstantPool;
    pub fn getConstantPool(&self, klass: Object) -> Result<Object> {
        let this = self;
        let _t0 = klass.getConstantPool()?;
        Ok(_t0)
    }

    // java: casAnnotationType(Ljava/lang/Class;Lsun/reflect/annotation/AnnotationType;Lsun/reflect/annotation/AnnotationType;)Z
    pub fn casAnnotationType(&self, klass: Object, oldType: Object, newType: Object) -> Result<bool> {
        let this = self;
        let _t0 = klass.casAnnotationType(oldType, newType)?;
        Ok(_t0)
    }

    // java: getAnnotationType(Ljava/lang/Class;)Lsun/reflect/annotation/AnnotationType;
    pub fn getAnnotationType(&self, klass: Object) -> Result<Object> {
        let this = self;
        let _t0 = klass.getAnnotationType()?;
        Ok(_t0)
    }

    // java: getDeclaredAnnotationMap(Ljava/lang/Class;)Ljava/util/Map;
    pub fn getDeclaredAnnotationMap(&self, klass: Object) -> Result<Object> {
        let this = self;
        let _t0 = klass.getDeclaredAnnotationMap()?;
        Ok(_t0)
    }

    // java: getRawClassAnnotations(Ljava/lang/Class;)[B
    pub fn getRawClassAnnotations(&self, klass: Object) -> Result<Vec<i8>> {
        let this = self;
        let _t0 = klass.getRawAnnotations()?;
        Ok(_t0)
    }

    // java: getRawClassTypeAnnotations(Ljava/lang/Class;)[B
    pub fn getRawClassTypeAnnotations(&self, klass: Object) -> Result<Vec<i8>> {
        let this = self;
        let _t0 = klass.getRawTypeAnnotations()?;
        Ok(_t0)
    }

    // java: getRawExecutableTypeAnnotations(Ljava/lang/reflect/Executable;)[B
    pub fn getRawExecutableTypeAnnotations(&self, executable: Object) -> Result<Vec<i8>> {
        let this = self;
        let _t0: Vec<i8> = Class::getExecutableTypeAnnotationBytes(executable)?;
        Ok(_t0)
    }

    // java: getEnumConstantsShared(Ljava/lang/Class;)[Ljava/lang/Enum;
    pub fn getEnumConstantsShared(&self, klass: Object) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = klass.getEnumConstantsShared()?;
        Ok(_t0)
    }

    // java: blockedOn(Lsun/nio/ch/Interruptible;)V
    pub fn blockedOn(&self, b: Object) -> Result<()> {
        let this = self;
        Thread::blockedOn(b)?;
        Ok(())
    }

    // java: registerShutdownHook(IZLjava/lang/Runnable;)V
    pub fn registerShutdownHook(&self, slot: i32, registerShutdownInProgress: bool, hook: Object) -> Result<()> {
        let this = self;
        Shutdown::add(slot, registerShutdownInProgress, hook)?;
        Ok(())
    }

    // java: newThreadWithAcc(Ljava/lang/Runnable;Ljava/security/AccessControlContext;)Ljava/lang/Thread;
    pub fn newThreadWithAcc(&self, target: Object, acc: Object) -> Result<Object> {
        let this = self;
        Ok(Thread::new(target, acc)?)
    }

    // java: invokeFinalize(Ljava/lang/Object;)V
    pub fn invokeFinalize(&self, o: Object) -> Result<()> {
        let this = self;
        o.finalize()?;
        Ok(())
    }

    // java: createOrGetClassLoaderValueMap(Ljava/lang/ClassLoader;)Ljava/util/concurrent/ConcurrentHashMap;
    pub fn createOrGetClassLoaderValueMap(&self, cl: Object) -> Result<Object> {
        let this = self;
        let _t0 = cl.createOrGetClassLoaderValueMap()?;
        Ok(_t0)
    }

    // java: defineClass(Ljava/lang/ClassLoader;Ljava/lang/String;[BLjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;
    // java: defineClass(Ljava/lang/ClassLoader;Ljava/lang/String;[BLjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;
    pub fn defineClass__classl_str_arr_b_protec_str(&self, loader: Object, name: String, b: Vec<i8>, pd: Object, source: String) -> Result<Object> {
        let this = self;
        let _t0: Object = ClassLoader::defineClass1(loader, name, &b, 0i32, (b.len() as i32), pd, source)?;
        Ok(_t0)
    }

    // java: defineClass(Ljava/lang/ClassLoader;Ljava/lang/Class;Ljava/lang/String;[BLjava/security/ProtectionDomain;ZILjava/lang/Object;)Ljava/lang/Class;
    // java: defineClass(Ljava/lang/ClassLoader;Ljava/lang/Class;Ljava/lang/String;[BLjava/security/ProtectionDomain;ZILjava/lang/Object;)Ljava/lang/Class;
    pub fn defineClass__classl_class_str_arr_b_protec_z_i_obj(&self, loader: Object, lookup: Object, name: String, b: Vec<i8>, pd: Object, initialize: bool, flags: i32, classData: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = ClassLoader::defineClass0(loader, lookup, name, &b, 0i32, (b.len() as i32), pd, initialize, flags, classData)?;
        Ok(_t0)
    }

    // java: findBootstrapClassOrNull(Ljava/lang/String;)Ljava/lang/Class;
    pub fn findBootstrapClassOrNull(&self, name: String) -> Result<Object> {
        let this = self;
        let _t0: Object = ClassLoader::findBootstrapClassOrNull(name)?;
        Ok(_t0)
    }

    // java: definePackage(Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/lang/Module;)Ljava/lang/Package;
    pub fn definePackage(&self, cl: Object, name: String, module: Object) -> Result<Object> {
        let this = self;
        let _t0 = cl.definePackage(name, module)?;
        Ok(_t0)
    }

    // java: fastUUID(JJ)Ljava/lang/String;
    pub fn fastUUID(&self, lsb: i64, arg_1: i64) -> Result<String> {
        let this = self;
        let _t0: String = Long::fastUUID(lsb, local_3)?;
        Ok(_t0)
    }

    // java: addNonExportedPackages(Ljava/lang/ModuleLayer;)V
    pub fn addNonExportedPackages(&self, layer: Object) -> Result<()> {
        let this = self;
        SecurityManager::addNonExportedPackages(layer)?;
        Ok(())
    }

    // java: invalidatePackageAccessCache()V
    pub fn invalidatePackageAccessCache(&self) -> Result<()> {
        let this = self;
        SecurityManager::invalidatePackageAccessCache()?;
        Ok(())
    }

    // java: defineModule(Ljava/lang/ClassLoader;Ljava/lang/module/ModuleDescriptor;Ljava/net/URI;)Ljava/lang/Module;
    pub fn defineModule(&self, loader: Object, descriptor: Object, uri: Object) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        let mut _obj0: Module = Module::new(Module::new(), loader, descriptor, uri)?;
        Ok(_obj0)
    }

    // java: defineUnnamedModule(Ljava/lang/ClassLoader;)Ljava/lang/Module;
    pub fn defineUnnamedModule(&self, loader: Object) -> Result<Object> {
        let this = self;
        Ok(Module::new(loader)?)
    }

    // java: addReads(Ljava/lang/Module;Ljava/lang/Module;)V
    pub fn addReads(&self, m1: Object, m2: Object) -> Result<()> {
        let this = self;
        m1.implAddReads(m2)?;
        Ok(())
    }

    // java: addReadsAllUnnamed(Ljava/lang/Module;)V
    pub fn addReadsAllUnnamed(&self, m: Object) -> Result<()> {
        let this = self;
        m.implAddReadsAllUnnamed()?;
        Ok(())
    }

    // java: addExports(Ljava/lang/Module;Ljava/lang/String;)V
    // java: addExports(Ljava/lang/Module;Ljava/lang/String;)V
    pub fn addExports__module_str(&self, m: Object, pn: String) -> Result<()> {
        let this = self;
        m.implAddExports(pn)?;
        Ok(())
    }

    // java: addExports(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V
    // java: addExports(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V
    pub fn addExports__module_str_module(&self, m: Object, pn: String, other: Object) -> Result<()> {
        let this = self;
        m.implAddExports(pn, other)?;
        Ok(())
    }

    // java: addExportsToAllUnnamed(Ljava/lang/Module;Ljava/lang/String;)V
    pub fn addExportsToAllUnnamed(&self, m: Object, pn: String) -> Result<()> {
        let this = self;
        m.implAddExportsToAllUnnamed(pn)?;
        Ok(())
    }

    // java: addOpens(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V
    pub fn addOpens(&self, m: Object, pn: String, other: Object) -> Result<()> {
        let this = self;
        m.implAddOpens(pn, other)?;
        Ok(())
    }

    // java: addOpensToAllUnnamed(Ljava/lang/Module;Ljava/lang/String;)V
    // java: addOpensToAllUnnamed(Ljava/lang/Module;Ljava/lang/String;)V
    pub fn addOpensToAllUnnamed__module_str(&self, m: Object, pn: String) -> Result<()> {
        let this = self;
        m.implAddOpensToAllUnnamed(pn)?;
        Ok(())
    }

    // java: addOpensToAllUnnamed(Ljava/lang/Module;Ljava/util/Set;Ljava/util/Set;)V
    // java: addOpensToAllUnnamed(Ljava/lang/Module;Ljava/util/Set;Ljava/util/Set;)V
    pub fn addOpensToAllUnnamed__module_set_set(&self, m: Object, concealedPackages: Object, exportedPackages: Object) -> Result<()> {
        let this = self;
        m.implAddOpensToAllUnnamed(concealedPackages, exportedPackages)?;
        Ok(())
    }

    // java: addUses(Ljava/lang/Module;Ljava/lang/Class;)V
    pub fn addUses(&self, m: Object, service: Object) -> Result<()> {
        let this = self;
        m.implAddUses(service)?;
        Ok(())
    }

    // java: isReflectivelyExported(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)Z
    pub fn isReflectivelyExported(&self, m: Object, pn: String, other: Object) -> Result<bool> {
        let this = self;
        let _t0 = m.isReflectivelyExported(pn, other)?;
        Ok(_t0)
    }

    // java: isReflectivelyOpened(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)Z
    pub fn isReflectivelyOpened(&self, m: Object, pn: String, other: Object) -> Result<bool> {
        let this = self;
        let _t0 = m.isReflectivelyOpened(pn, other)?;
        Ok(_t0)
    }

    // java: addEnableNativeAccess(Ljava/lang/Module;)Ljava/lang/Module;
    pub fn addEnableNativeAccess(&self, m: Object) -> Result<Object> {
        let this = self;
        let _t0 = m.implAddEnableNativeAccess()?;
        Ok(_t0)
    }

    // java: addEnableNativeAccessToAllUnnamed()V
    pub fn addEnableNativeAccessToAllUnnamed(&self) -> Result<()> {
        let this = self;
        Module::implAddEnableNativeAccessToAllUnnamed()?;
        Ok(())
    }

    // java: ensureNativeAccess(Ljava/lang/Module;Ljava/lang/Class;Ljava/lang/String;)V
    pub fn ensureNativeAccess(&self, m: Object, owner: Object, methodName: String) -> Result<()> {
        let this = self;
        m.ensureNativeAccess(owner, methodName)?;
        Ok(())
    }

    // java: getServicesCatalog(Ljava/lang/ModuleLayer;)Ljdk/internal/module/ServicesCatalog;
    pub fn getServicesCatalog(&self, layer: Object) -> Result<Object> {
        let this = self;
        let _t0 = layer.getServicesCatalog()?;
        Ok(_t0)
    }

    // java: bindToLoader(Ljava/lang/ModuleLayer;Ljava/lang/ClassLoader;)V
    pub fn bindToLoader(&self, layer: Object, loader: Object) -> Result<()> {
        let this = self;
        layer.bindToLoader(loader)?;
        Ok(())
    }

    // java: layers(Ljava/lang/ModuleLayer;)Ljava/util/stream/Stream;
    // java: layers(Ljava/lang/ModuleLayer;)Ljava/util/stream/Stream;
    pub fn layers__module(&self, layer: Object) -> Result<Object> {
        let this = self;
        let _t0 = layer.layers()?;
        Ok(_t0)
    }

    // java: layers(Ljava/lang/ClassLoader;)Ljava/util/stream/Stream;
    // java: layers(Ljava/lang/ClassLoader;)Ljava/util/stream/Stream;
    pub fn layers__classl(&self, loader: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = ModuleLayer::layers(loader)?;
        Ok(_t0)
    }

    // java: countPositives([BII)I
    pub fn countPositives(&self, bytes: Vec<i8>, offset: i32, length: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = StringCoding::countPositives(&bytes, offset, length)?;
        Ok(_t0)
    }

    // java: newStringNoRepl([BLjava/nio/charset/Charset;)Ljava/lang/String;
    pub fn newStringNoRepl(&self, bytes: Vec<i8>, cs: Object) -> Result<String> {
        let this = self;
        let _t0: String = String::newStringNoRepl(&bytes, cs)?;
        Ok(_t0)
    }

    // java: getUTF16Char([BI)C
    pub fn getUTF16Char(&self, bytes: Vec<i8>, index: i32) -> Result<u16> {
        let this = self;
        let _t0: u16 = StringUTF16::getChar(&bytes, index)?;
        Ok(_t0)
    }

    // java: getBytesNoRepl(Ljava/lang/String;Ljava/nio/charset/Charset;)[B
    pub fn getBytesNoRepl(&self, s: String, cs: Object) -> Result<Vec<i8>> {
        let this = self;
        let _t0: Vec<i8> = String::getBytesNoRepl(s, cs)?;
        Ok(_t0)
    }

    // java: newStringUTF8NoRepl([BII)Ljava/lang/String;
    pub fn newStringUTF8NoRepl(&self, bytes: Vec<i8>, off: i32, len: i32) -> Result<String> {
        let this = self;
        let _t0: String = String::newStringUTF8NoRepl(&bytes, off, len, 1i32)?;
        Ok(_t0)
    }

    // java: getBytesUTF8NoRepl(Ljava/lang/String;)[B
    pub fn getBytesUTF8NoRepl(&self, s: String) -> Result<Vec<i8>> {
        let this = self;
        let _t0: Vec<i8> = String::getBytesUTF8NoRepl(s)?;
        Ok(_t0)
    }

    // java: inflateBytesToChars([BI[CII)V
    pub fn inflateBytesToChars(&self, src: Vec<i8>, srcOff: i32, dst: Vec<u16>, dstOff: i32, len: i32) -> Result<()> {
        let this = self;
        StringLatin1::inflate(&src, srcOff, &dst, dstOff, len)?;
        Ok(())
    }

    // java: decodeASCII([BI[CII)I
    pub fn decodeASCII(&self, src: Vec<i8>, srcOff: i32, dst: Vec<u16>, dstOff: i32, len: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = String::decodeASCII(&src, srcOff, &dst, dstOff, len)?;
        Ok(_t0)
    }

    // java: encodeASCII([CI[BII)I
    pub fn encodeASCII(&self, src: Vec<u16>, srcOff: i32, dst: Vec<i8>, dstOff: i32, len: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = StringCoding::implEncodeAsciiArray(&src, srcOff, &dst, dstOff, len)?;
        Ok(_t0)
    }

    // java: initialSystemIn()Ljava/io/InputStream;
    pub fn initialSystemIn(&self) -> Result<Object> {
        let this = self;
        Ok(System::initialIn())
    }

    // java: setCause(Ljava/lang/Throwable;Ljava/lang/Throwable;)V
    pub fn setCause(&self, t: Object, cause: Object) -> Result<()> {
        let this = self;
        t.setCause(cause)?;
        Ok(())
    }

    // java: protectionDomain(Ljava/lang/Class;)Ljava/security/ProtectionDomain;
    pub fn protectionDomain(&self, c: Object) -> Result<Object> {
        let this = self;
        let _t0 = c.protectionDomain()?;
        Ok(_t0)
    }

    // java: stringConcatHelper(Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;
    pub fn stringConcatHelper(&self, name: String, methodType: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = StringConcatHelper::lookupStatic(name, methodType)?;
        Ok(_t0)
    }

    // java: stringConcatInitialCoder()J
    pub fn stringConcatInitialCoder(&self) -> Result<i64> {
        let this = self;
        let _t0: i64 = StringConcatHelper::initialCoder()?;
        Ok(_t0)
    }

    // java: stringConcatMix(JLjava/lang/String;)J
    pub fn stringConcatMix(&self, lengthCoder: i64, arg_1: String) -> Result<i64> {
        let this = self;
        let _t0: i64 = StringConcatHelper::mix(lengthCoder, local_3)?;
        Ok(_t0)
    }

    // java: stringConcatCoder(C)J
    pub fn stringConcatCoder(&self, value: u16) -> Result<i64> {
        let this = self;
        let _t0: i64 = StringConcatHelper::coder(value)?;
        Ok(_t0)
    }

    // java: stringBuilderConcatMix(JLjava/lang/StringBuilder;)J
    pub fn stringBuilderConcatMix(&self, lengthCoder: i64, arg_1: Object) -> Result<i64> {
        let this = self;
        let _t0 = local_3.mix(lengthCoder)?;
        Ok(_t0)
    }

    // java: stringBuilderConcatPrepend(J[BLjava/lang/StringBuilder;)J
    pub fn stringBuilderConcatPrepend(&self, lengthCoder: i64, arg_1: Vec<i8>, buf: Object) -> Result<i64> {
        let this = self;
        let _t0 = local_4.prepend(lengthCoder, buf)?;
        Ok(_t0)
    }

    // java: join(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;I)Ljava/lang/String;
    pub fn join(&self, prefix: String, suffix: String, delimiter: String, elements: Vec<String>, size: i32) -> Result<String> {
        let this = self;
        let _t0: String = String::join(prefix, suffix, delimiter, &elements, size)?;
        Ok(_t0)
    }

    // java: classData(Ljava/lang/Class;)Ljava/lang/Object;
    pub fn classData(&self, c: Object) -> Result<Object> {
        let this = self;
        let _t0 = c.getClassData()?;
        Ok(_t0)
    }

    // java: findNative(Ljava/lang/ClassLoader;Ljava/lang/String;)J
    pub fn findNative(&self, loader: Object, entry: String) -> Result<i64> {
        let this = self;
        let _t0: i64 = ClassLoader::findNative(loader, entry)?;
        Ok(_t0)
    }

    // java: exit(I)V
    pub fn exit(&self, statusCode: i32) -> Result<()> {
        let this = self;
        Shutdown::exit(statusCode)?;
        Ok(())
    }

    // java: getAllThreads()[Ljava/lang/Thread;
    pub fn getAllThreads(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Vec<Object> = Thread::getAllThreads()?;
        Ok(_t0)
    }

    // java: threadContainer(Ljava/lang/Thread;)Ljdk/internal/vm/ThreadContainer;
    pub fn threadContainer(&self, thread: Object) -> Result<Object> {
        let this = self;
        let _t0 = thread.threadContainer()?;
        Ok(_t0)
    }

    // java: start(Ljava/lang/Thread;Ljdk/internal/vm/ThreadContainer;)V
    pub fn start(&self, thread: Object, container: Object) -> Result<()> {
        let this = self;
        thread.start(container)?;
        Ok(())
    }

    // java: headStackableScope(Ljava/lang/Thread;)Ljdk/internal/vm/StackableScope;
    pub fn headStackableScope(&self, thread: Object) -> Result<Object> {
        let this = self;
        let _t0 = thread.headStackableScopes()?;
        Ok(_t0)
    }

    // java: setHeadStackableScope(Ljdk/internal/vm/StackableScope;)V
    pub fn setHeadStackableScope(&self, scope: Object) -> Result<()> {
        let this = self;
        Thread::setHeadStackableScope(scope)?;
        Ok(())
    }

    // java: currentCarrierThread()Ljava/lang/Thread;
    pub fn currentCarrierThread(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Thread::currentCarrierThread()?;
        Ok(_t0)
    }

    // java: executeOnCarrierThread(Ljava/util/concurrent/Callable;)Ljava/lang/Object;
    pub fn executeOnCarrierThread(&self, task: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Thread::currentThread()?;
        let mut local_3: Object = _t0;
        let mut vthread: Object = local_3;
        let _t1 = vthread.executeOnCarrierThread(task)?;
        return Ok(_t1);
        let _t2 = task.call()?;
        Ok(_t2)
    }

    // java: getCarrierThreadLocal(Ljdk/internal/misc/CarrierThreadLocal;)Ljava/lang/Object;
    pub fn getCarrierThreadLocal(&self, local: Object) -> Result<Object> {
        let this = self;
        let _t0 = local.getCarrierThreadLocal()?;
        Ok(_t0)
    }

    // java: setCarrierThreadLocal(Ljdk/internal/misc/CarrierThreadLocal;Ljava/lang/Object;)V
    pub fn setCarrierThreadLocal(&self, local: Object, value: Object) -> Result<()> {
        let this = self;
        local.setCarrierThreadLocal(value)?;
        Ok(())
    }

    // java: removeCarrierThreadLocal(Ljdk/internal/misc/CarrierThreadLocal;)V
    pub fn removeCarrierThreadLocal(&self, local: Object) -> Result<()> {
        let this = self;
        local.removeCarrierThreadLocal()?;
        Ok(())
    }

    // java: isCarrierThreadLocalPresent(Ljdk/internal/misc/CarrierThreadLocal;)Z
    pub fn isCarrierThreadLocalPresent(&self, local: Object) -> Result<bool> {
        let this = self;
        let _t0 = local.isCarrierThreadLocalPresent()?;
        Ok(_t0)
    }

    // java: scopedValueCache()[Ljava/lang/Object;
    pub fn scopedValueCache(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Vec<Object> = Thread::scopedValueCache()?;
        Ok(_t0)
    }

    // java: setScopedValueCache([Ljava/lang/Object;)V
    pub fn setScopedValueCache(&self, cache: Vec<Object>) -> Result<()> {
        let this = self;
        Thread::setScopedValueCache(&cache)?;
        Ok(())
    }

    // java: scopedValueBindings()Ljava/lang/Object;
    pub fn scopedValueBindings(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Thread::scopedValueBindings()?;
        Ok(_t0)
    }

    // java: getContinuation(Ljava/lang/Thread;)Ljdk/internal/vm/Continuation;
    pub fn getContinuation(&self, thread: Object) -> Result<Object> {
        let this = self;
        let _t0 = thread.getContinuation()?;
        Ok(_t0)
    }

    // java: setContinuation(Ljava/lang/Thread;Ljdk/internal/vm/Continuation;)V
    pub fn setContinuation(&self, thread: Object, continuation: Object) -> Result<()> {
        let this = self;
        thread.setContinuation(continuation)?;
        Ok(())
    }

    // java: virtualThreadContinuationScope()Ljdk/internal/vm/ContinuationScope;
    pub fn virtualThreadContinuationScope(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = VirtualThread::continuationScope()?;
        Ok(_t0)
    }

    // java: parkVirtualThread()V
    // java: parkVirtualThread()V
    pub fn parkVirtualThread(&self) -> Result<()> {
        let this = self;
        let _t0: Object = Thread::currentThread()?;
        let mut thread: Object = _t0;
        let mut vthread: Object = thread;
        vthread.park()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: parkVirtualThread(J)V
    // java: parkVirtualThread(J)V
    pub fn parkVirtualThread__l(&self, nanos: i64) -> Result<()> {
        let this = self;
        let _t0: Object = Thread::currentThread()?;
        let mut thread: Object = _t0;
        let mut vthread: Object = thread;
        vthread.parkNanos(nanos)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: unparkVirtualThread(Ljava/lang/Thread;)V
    pub fn unparkVirtualThread(&self, thread: Object) -> Result<()> {
        let this = self;
        let mut vthread: Object = thread;
        vthread.unpark()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: newStackWalkerInstance(Ljava/util/Set;Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)Ljava/lang/StackWalker;
    pub fn newStackWalkerInstance(&self, options: Object, contScope: Object, continuation: Object) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        let _t0: Object = StackWalker::newInstance(todo!("stack underflow"), options, contScope, continuation)?;
        Ok(_t0)
    }

    // java: getLoaderNameID(Ljava/lang/ClassLoader;)Ljava/lang/String;
    pub fn getLoaderNameID(&self, loader: Object) -> Result<String> {
        let this = self;
        let _t0 = loader.nameAndId()?;
        Ok(String::from("null"))
    }

    // java: allowSecurityManager()Z
    pub fn allowSecurityManager(&self) -> Result<bool> {
        let this = self;
        let _t0: bool = System::allowSecurityManager()?;
        Ok(_t0)
    }
}
