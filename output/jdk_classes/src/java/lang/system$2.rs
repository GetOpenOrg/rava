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
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "getDeclaredPublicMethods", descriptor = "(Ljava/lang/Class;Ljava/lang/String;[Ljava/lang/Class;)Ljava/util/List;", access = "public"))]
    pub fn getDeclaredPublicMethods(&self, klass: Object, name: String, parameterTypes: Vec<Object>) -> Result<Object> {
        let this = self;
        let _t0 = klass.getDeclaredPublicMethods(name, parameterTypes)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getConstantPool", descriptor = "(Ljava/lang/Class;)Ljdk/internal/reflect/ConstantPool;", access = "public"))]
    pub fn getConstantPool(&self, klass: Object) -> Result<Object> {
        let this = self;
        let _t0 = klass.getConstantPool()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "casAnnotationType", descriptor = "(Ljava/lang/Class;Lsun/reflect/annotation/AnnotationType;Lsun/reflect/annotation/AnnotationType;)Z", access = "public"))]
    pub fn casAnnotationType(&self, klass: Object, oldType: Object, newType: Object) -> Result<bool> {
        let this = self;
        let _t0 = klass.casAnnotationType(oldType, newType)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getAnnotationType", descriptor = "(Ljava/lang/Class;)Lsun/reflect/annotation/AnnotationType;", access = "public"))]
    pub fn getAnnotationType(&self, klass: Object) -> Result<Object> {
        let this = self;
        let _t0 = klass.getAnnotationType()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getDeclaredAnnotationMap", descriptor = "(Ljava/lang/Class;)Ljava/util/Map;", access = "public"))]
    pub fn getDeclaredAnnotationMap(&self, klass: Object) -> Result<Object> {
        let this = self;
        let _t0 = klass.getDeclaredAnnotationMap()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getRawClassAnnotations", descriptor = "(Ljava/lang/Class;)[B", access = "public"))]
    pub fn getRawClassAnnotations(&self, klass: Object) -> Result<Vec<i8>> {
        let this = self;
        let _t0 = klass.getRawAnnotations()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getRawClassTypeAnnotations", descriptor = "(Ljava/lang/Class;)[B", access = "public"))]
    pub fn getRawClassTypeAnnotations(&self, klass: Object) -> Result<Vec<i8>> {
        let this = self;
        let _t0 = klass.getRawTypeAnnotations()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getRawExecutableTypeAnnotations", descriptor = "(Ljava/lang/reflect/Executable;)[B", access = "public"))]
    pub fn getRawExecutableTypeAnnotations(&self, executable: Object) -> Result<Vec<i8>> {
        let this = self;
        let _t0: Vec<i8> = Class::getExecutableTypeAnnotationBytes(executable)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getEnumConstantsShared", descriptor = "(Ljava/lang/Class;)[Ljava/lang/Enum;", access = "public"))]
    pub fn getEnumConstantsShared(&self, klass: Object) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = klass.getEnumConstantsShared()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "blockedOn", descriptor = "(Lsun/nio/ch/Interruptible;)V", access = "public"))]
    pub fn blockedOn(&self, b: Object) -> Result<()> {
        let this = self;
        Thread::blockedOn(b)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "registerShutdownHook", descriptor = "(IZLjava/lang/Runnable;)V", access = "public"))]
    pub fn registerShutdownHook(&self, slot: i32, registerShutdownInProgress: bool, hook: Object) -> Result<()> {
        let this = self;
        Shutdown::add(slot, registerShutdownInProgress, hook)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "newThreadWithAcc", descriptor = "(Ljava/lang/Runnable;Ljava/security/AccessControlContext;)Ljava/lang/Thread;", access = "public"))]
    pub fn newThreadWithAcc(&self, target: Object, acc: Object) -> Result<Object> {
        let this = self;
        Ok(Thread::new(target, acc)?)
    }

    #[cfg_attr(any(), java_method(name = "invokeFinalize", descriptor = "(Ljava/lang/Object;)V", access = "public"))]
    pub fn invokeFinalize(&self, o: Object) -> Result<()> {
        let this = self;
        o.finalize()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "createOrGetClassLoaderValueMap", descriptor = "(Ljava/lang/ClassLoader;)Ljava/util/concurrent/ConcurrentHashMap;", access = "public"))]
    pub fn createOrGetClassLoaderValueMap(&self, cl: Object) -> Result<Object> {
        let this = self;
        let _t0 = cl.createOrGetClassLoaderValueMap()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "defineClass", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/String;[BLjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;", access = "public"))]
    // java: defineClass(Ljava/lang/ClassLoader;Ljava/lang/String;[BLjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;
    pub fn defineClass__classl_str_arr_b_protec_str(&self, loader: Object, name: String, b: Vec<i8>, pd: Object, source: String) -> Result<Object> {
        let this = self;
        let _t0: Object = ClassLoader::defineClass1(loader, name, &b, 0i32, (b.len() as i32), pd, source)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "defineClass", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/Class;Ljava/lang/String;[BLjava/security/ProtectionDomain;ZILjava/lang/Object;)Ljava/lang/Class;", access = "public"))]
    // java: defineClass(Ljava/lang/ClassLoader;Ljava/lang/Class;Ljava/lang/String;[BLjava/security/ProtectionDomain;ZILjava/lang/Object;)Ljava/lang/Class;
    pub fn defineClass__classl_class_str_arr_b_protec_z_i_obj(&self, loader: Object, lookup: Object, name: String, b: Vec<i8>, pd: Object, initialize: bool, flags: i32, classData: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = ClassLoader::defineClass0(loader, lookup, name, &b, 0i32, (b.len() as i32), pd, initialize, flags, classData)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "findBootstrapClassOrNull", descriptor = "(Ljava/lang/String;)Ljava/lang/Class;", access = "public"))]
    pub fn findBootstrapClassOrNull(&self, name: String) -> Result<Object> {
        let this = self;
        let _t0: Object = ClassLoader::findBootstrapClassOrNull(name)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "definePackage", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/lang/Module;)Ljava/lang/Package;", access = "public"))]
    pub fn definePackage(&self, cl: Object, name: String, module: Object) -> Result<Object> {
        let this = self;
        let _t0 = cl.definePackage(name, module)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "fastUUID", descriptor = "(JJ)Ljava/lang/String;", access = "public"))]
    pub fn fastUUID(&self, lsb: i64, arg_1: i64) -> Result<String> {
        let this = self;
        let _t0: String = Long::fastUUID(lsb, local_3)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "addNonExportedPackages", descriptor = "(Ljava/lang/ModuleLayer;)V", access = "public"))]
    pub fn addNonExportedPackages(&self, layer: Object) -> Result<()> {
        let this = self;
        SecurityManager::addNonExportedPackages(layer)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "invalidatePackageAccessCache", descriptor = "()V", access = "public"))]
    pub fn invalidatePackageAccessCache(&self) -> Result<()> {
        let this = self;
        SecurityManager::invalidatePackageAccessCache()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "defineModule", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/module/ModuleDescriptor;Ljava/net/URI;)Ljava/lang/Module;", access = "public"))]
    pub fn defineModule(&self, loader: Object, descriptor: Object, uri: Object) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        let mut _obj0: Module = Module::new(Module::new(), loader, descriptor, uri)?;
        Ok(_obj0)
    }

    #[cfg_attr(any(), java_method(name = "defineUnnamedModule", descriptor = "(Ljava/lang/ClassLoader;)Ljava/lang/Module;", access = "public"))]
    pub fn defineUnnamedModule(&self, loader: Object) -> Result<Object> {
        let this = self;
        Ok(Module::new(loader)?)
    }

    #[cfg_attr(any(), java_method(name = "addReads", descriptor = "(Ljava/lang/Module;Ljava/lang/Module;)V", access = "public"))]
    pub fn addReads(&self, m1: Object, m2: Object) -> Result<()> {
        let this = self;
        m1.implAddReads(m2)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "addReadsAllUnnamed", descriptor = "(Ljava/lang/Module;)V", access = "public"))]
    pub fn addReadsAllUnnamed(&self, m: Object) -> Result<()> {
        let this = self;
        m.implAddReadsAllUnnamed()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "addExports", descriptor = "(Ljava/lang/Module;Ljava/lang/String;)V", access = "public"))]
    // java: addExports(Ljava/lang/Module;Ljava/lang/String;)V
    pub fn addExports__module_str(&self, m: Object, pn: String) -> Result<()> {
        let this = self;
        m.implAddExports(pn)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "addExports", descriptor = "(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V", access = "public"))]
    // java: addExports(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V
    pub fn addExports__module_str_module(&self, m: Object, pn: String, other: Object) -> Result<()> {
        let this = self;
        m.implAddExports(pn, other)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "addExportsToAllUnnamed", descriptor = "(Ljava/lang/Module;Ljava/lang/String;)V", access = "public"))]
    pub fn addExportsToAllUnnamed(&self, m: Object, pn: String) -> Result<()> {
        let this = self;
        m.implAddExportsToAllUnnamed(pn)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "addOpens", descriptor = "(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V", access = "public"))]
    pub fn addOpens(&self, m: Object, pn: String, other: Object) -> Result<()> {
        let this = self;
        m.implAddOpens(pn, other)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "addOpensToAllUnnamed", descriptor = "(Ljava/lang/Module;Ljava/lang/String;)V", access = "public"))]
    // java: addOpensToAllUnnamed(Ljava/lang/Module;Ljava/lang/String;)V
    pub fn addOpensToAllUnnamed__module_str(&self, m: Object, pn: String) -> Result<()> {
        let this = self;
        m.implAddOpensToAllUnnamed(pn)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "addOpensToAllUnnamed", descriptor = "(Ljava/lang/Module;Ljava/util/Set;Ljava/util/Set;)V", access = "public"))]
    // java: addOpensToAllUnnamed(Ljava/lang/Module;Ljava/util/Set;Ljava/util/Set;)V
    pub fn addOpensToAllUnnamed__module_set_set(&self, m: Object, concealedPackages: Object, exportedPackages: Object) -> Result<()> {
        let this = self;
        m.implAddOpensToAllUnnamed(concealedPackages, exportedPackages)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "addUses", descriptor = "(Ljava/lang/Module;Ljava/lang/Class;)V", access = "public"))]
    pub fn addUses(&self, m: Object, service: Object) -> Result<()> {
        let this = self;
        m.implAddUses(service)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "isReflectivelyExported", descriptor = "(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)Z", access = "public"))]
    pub fn isReflectivelyExported(&self, m: Object, pn: String, other: Object) -> Result<bool> {
        let this = self;
        let _t0 = m.isReflectivelyExported(pn, other)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "isReflectivelyOpened", descriptor = "(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)Z", access = "public"))]
    pub fn isReflectivelyOpened(&self, m: Object, pn: String, other: Object) -> Result<bool> {
        let this = self;
        let _t0 = m.isReflectivelyOpened(pn, other)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "addEnableNativeAccess", descriptor = "(Ljava/lang/Module;)Ljava/lang/Module;", access = "public"))]
    pub fn addEnableNativeAccess(&self, m: Object) -> Result<Object> {
        let this = self;
        let _t0 = m.implAddEnableNativeAccess()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "addEnableNativeAccessToAllUnnamed", descriptor = "()V", access = "public"))]
    pub fn addEnableNativeAccessToAllUnnamed(&self) -> Result<()> {
        let this = self;
        Module::implAddEnableNativeAccessToAllUnnamed()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "ensureNativeAccess", descriptor = "(Ljava/lang/Module;Ljava/lang/Class;Ljava/lang/String;)V", access = "public"))]
    pub fn ensureNativeAccess(&self, m: Object, owner: Object, methodName: String) -> Result<()> {
        let this = self;
        m.ensureNativeAccess(owner, methodName)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getServicesCatalog", descriptor = "(Ljava/lang/ModuleLayer;)Ljdk/internal/module/ServicesCatalog;", access = "public"))]
    pub fn getServicesCatalog(&self, layer: Object) -> Result<Object> {
        let this = self;
        let _t0 = layer.getServicesCatalog()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "bindToLoader", descriptor = "(Ljava/lang/ModuleLayer;Ljava/lang/ClassLoader;)V", access = "public"))]
    pub fn bindToLoader(&self, layer: Object, loader: Object) -> Result<()> {
        let this = self;
        layer.bindToLoader(loader)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "layers", descriptor = "(Ljava/lang/ModuleLayer;)Ljava/util/stream/Stream;", access = "public"))]
    // java: layers(Ljava/lang/ModuleLayer;)Ljava/util/stream/Stream;
    pub fn layers__module(&self, layer: Object) -> Result<Object> {
        let this = self;
        let _t0 = layer.layers()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "layers", descriptor = "(Ljava/lang/ClassLoader;)Ljava/util/stream/Stream;", access = "public"))]
    // java: layers(Ljava/lang/ClassLoader;)Ljava/util/stream/Stream;
    pub fn layers__classl(&self, loader: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = ModuleLayer::layers(loader)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "countPositives", descriptor = "([BII)I", access = "public"))]
    pub fn countPositives(&self, bytes: Vec<i8>, offset: i32, length: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = StringCoding::countPositives(&bytes, offset, length)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "newStringNoRepl", descriptor = "([BLjava/nio/charset/Charset;)Ljava/lang/String;", access = "public"))]
    pub fn newStringNoRepl(&self, bytes: Vec<i8>, cs: Object) -> Result<String> {
        let this = self;
        let _t0: String = String::newStringNoRepl(&bytes, cs)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getUTF16Char", descriptor = "([BI)C", access = "public"))]
    pub fn getUTF16Char(&self, bytes: Vec<i8>, index: i32) -> Result<u16> {
        let this = self;
        let _t0: u16 = StringUTF16::getChar(&bytes, index)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getBytesNoRepl", descriptor = "(Ljava/lang/String;Ljava/nio/charset/Charset;)[B", access = "public"))]
    pub fn getBytesNoRepl(&self, s: String, cs: Object) -> Result<Vec<i8>> {
        let this = self;
        let _t0: Vec<i8> = String::getBytesNoRepl(s, cs)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "newStringUTF8NoRepl", descriptor = "([BII)Ljava/lang/String;", access = "public"))]
    pub fn newStringUTF8NoRepl(&self, bytes: Vec<i8>, off: i32, len: i32) -> Result<String> {
        let this = self;
        let _t0: String = String::newStringUTF8NoRepl(&bytes, off, len, 1i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getBytesUTF8NoRepl", descriptor = "(Ljava/lang/String;)[B", access = "public"))]
    pub fn getBytesUTF8NoRepl(&self, s: String) -> Result<Vec<i8>> {
        let this = self;
        let _t0: Vec<i8> = String::getBytesUTF8NoRepl(s)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "inflateBytesToChars", descriptor = "([BI[CII)V", access = "public"))]
    pub fn inflateBytesToChars(&self, src: Vec<i8>, srcOff: i32, dst: Vec<u16>, dstOff: i32, len: i32) -> Result<()> {
        let this = self;
        StringLatin1::inflate(&src, srcOff, &dst, dstOff, len)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "decodeASCII", descriptor = "([BI[CII)I", access = "public"))]
    pub fn decodeASCII(&self, src: Vec<i8>, srcOff: i32, dst: Vec<u16>, dstOff: i32, len: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = String::decodeASCII(&src, srcOff, &dst, dstOff, len)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "encodeASCII", descriptor = "([CI[BII)I", access = "public"))]
    pub fn encodeASCII(&self, src: Vec<u16>, srcOff: i32, dst: Vec<i8>, dstOff: i32, len: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = StringCoding::implEncodeAsciiArray(&src, srcOff, &dst, dstOff, len)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "initialSystemIn", descriptor = "()Ljava/io/InputStream;", access = "public"))]
    pub fn initialSystemIn(&self) -> Result<Object> {
        let this = self;
        Ok(System::initialIn())
    }

    #[cfg_attr(any(), java_method(name = "setCause", descriptor = "(Ljava/lang/Throwable;Ljava/lang/Throwable;)V", access = "public"))]
    pub fn setCause(&self, t: Object, cause: Object) -> Result<()> {
        let this = self;
        t.setCause(cause)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "protectionDomain", descriptor = "(Ljava/lang/Class;)Ljava/security/ProtectionDomain;", access = "public"))]
    pub fn protectionDomain(&self, c: Object) -> Result<Object> {
        let this = self;
        let _t0 = c.protectionDomain()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "stringConcatHelper", descriptor = "(Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;", access = "public"))]
    pub fn stringConcatHelper(&self, name: String, methodType: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = StringConcatHelper::lookupStatic(name, methodType)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "stringConcatInitialCoder", descriptor = "()J", access = "public"))]
    pub fn stringConcatInitialCoder(&self) -> Result<i64> {
        let this = self;
        let _t0: i64 = StringConcatHelper::initialCoder()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "stringConcatMix", descriptor = "(JLjava/lang/String;)J", access = "public"))]
    pub fn stringConcatMix(&self, lengthCoder: i64, arg_1: String) -> Result<i64> {
        let this = self;
        let _t0: i64 = StringConcatHelper::mix(lengthCoder, local_3)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "stringConcatCoder", descriptor = "(C)J", access = "public"))]
    pub fn stringConcatCoder(&self, value: u16) -> Result<i64> {
        let this = self;
        let _t0: i64 = StringConcatHelper::coder(value)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "stringBuilderConcatMix", descriptor = "(JLjava/lang/StringBuilder;)J", access = "public"))]
    pub fn stringBuilderConcatMix(&self, lengthCoder: i64, arg_1: Object) -> Result<i64> {
        let this = self;
        let _t0 = local_3.mix(lengthCoder)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "stringBuilderConcatPrepend", descriptor = "(J[BLjava/lang/StringBuilder;)J", access = "public"))]
    pub fn stringBuilderConcatPrepend(&self, lengthCoder: i64, arg_1: Vec<i8>, buf: Object) -> Result<i64> {
        let this = self;
        let _t0 = local_4.prepend(lengthCoder, buf)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "join", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;I)Ljava/lang/String;", access = "public"))]
    pub fn join(&self, prefix: String, suffix: String, delimiter: String, elements: Vec<String>, size: i32) -> Result<String> {
        let this = self;
        let _t0: String = String::join(prefix, suffix, delimiter, &elements, size)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "classData", descriptor = "(Ljava/lang/Class;)Ljava/lang/Object;", access = "public"))]
    pub fn classData(&self, c: Object) -> Result<Object> {
        let this = self;
        let _t0 = c.getClassData()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "findNative", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/String;)J", access = "public"))]
    pub fn findNative(&self, loader: Object, entry: String) -> Result<i64> {
        let this = self;
        let _t0: i64 = ClassLoader::findNative(loader, entry)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "exit", descriptor = "(I)V", access = "public"))]
    pub fn exit(&self, statusCode: i32) -> Result<()> {
        let this = self;
        Shutdown::exit(statusCode)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getAllThreads", descriptor = "()[Ljava/lang/Thread;", access = "public"))]
    pub fn getAllThreads(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Vec<Object> = Thread::getAllThreads()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "threadContainer", descriptor = "(Ljava/lang/Thread;)Ljdk/internal/vm/ThreadContainer;", access = "public"))]
    pub fn threadContainer(&self, thread: Object) -> Result<Object> {
        let this = self;
        let _t0 = thread.threadContainer()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "start", descriptor = "(Ljava/lang/Thread;Ljdk/internal/vm/ThreadContainer;)V", access = "public"))]
    pub fn start(&self, thread: Object, container: Object) -> Result<()> {
        let this = self;
        thread.start(container)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "headStackableScope", descriptor = "(Ljava/lang/Thread;)Ljdk/internal/vm/StackableScope;", access = "public"))]
    pub fn headStackableScope(&self, thread: Object) -> Result<Object> {
        let this = self;
        let _t0 = thread.headStackableScopes()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "setHeadStackableScope", descriptor = "(Ljdk/internal/vm/StackableScope;)V", access = "public"))]
    pub fn setHeadStackableScope(&self, scope: Object) -> Result<()> {
        let this = self;
        Thread::setHeadStackableScope(scope)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "currentCarrierThread", descriptor = "()Ljava/lang/Thread;", access = "public"))]
    pub fn currentCarrierThread(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Thread::currentCarrierThread()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "executeOnCarrierThread", descriptor = "(Ljava/util/concurrent/Callable;)Ljava/lang/Object;", access = "public"))]
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

    #[cfg_attr(any(), java_method(name = "getCarrierThreadLocal", descriptor = "(Ljdk/internal/misc/CarrierThreadLocal;)Ljava/lang/Object;", access = "public"))]
    pub fn getCarrierThreadLocal(&self, local: Object) -> Result<Object> {
        let this = self;
        let _t0 = local.getCarrierThreadLocal()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "setCarrierThreadLocal", descriptor = "(Ljdk/internal/misc/CarrierThreadLocal;Ljava/lang/Object;)V", access = "public"))]
    pub fn setCarrierThreadLocal(&self, local: Object, value: Object) -> Result<()> {
        let this = self;
        local.setCarrierThreadLocal(value)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "removeCarrierThreadLocal", descriptor = "(Ljdk/internal/misc/CarrierThreadLocal;)V", access = "public"))]
    pub fn removeCarrierThreadLocal(&self, local: Object) -> Result<()> {
        let this = self;
        local.removeCarrierThreadLocal()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "isCarrierThreadLocalPresent", descriptor = "(Ljdk/internal/misc/CarrierThreadLocal;)Z", access = "public"))]
    pub fn isCarrierThreadLocalPresent(&self, local: Object) -> Result<bool> {
        let this = self;
        let _t0 = local.isCarrierThreadLocalPresent()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "scopedValueCache", descriptor = "()[Ljava/lang/Object;", access = "public"))]
    pub fn scopedValueCache(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Vec<Object> = Thread::scopedValueCache()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "setScopedValueCache", descriptor = "([Ljava/lang/Object;)V", access = "public"))]
    pub fn setScopedValueCache(&self, cache: Vec<Object>) -> Result<()> {
        let this = self;
        Thread::setScopedValueCache(&cache)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "scopedValueBindings", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn scopedValueBindings(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Thread::scopedValueBindings()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getContinuation", descriptor = "(Ljava/lang/Thread;)Ljdk/internal/vm/Continuation;", access = "public"))]
    pub fn getContinuation(&self, thread: Object) -> Result<Object> {
        let this = self;
        let _t0 = thread.getContinuation()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "setContinuation", descriptor = "(Ljava/lang/Thread;Ljdk/internal/vm/Continuation;)V", access = "public"))]
    pub fn setContinuation(&self, thread: Object, continuation: Object) -> Result<()> {
        let this = self;
        thread.setContinuation(continuation)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "virtualThreadContinuationScope", descriptor = "()Ljdk/internal/vm/ContinuationScope;", access = "public"))]
    pub fn virtualThreadContinuationScope(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = VirtualThread::continuationScope()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "parkVirtualThread", descriptor = "()V", access = "public"))]
    // java: parkVirtualThread()V
    pub fn parkVirtualThread(&self) -> Result<()> {
        let this = self;
        let _t0: Object = Thread::currentThread()?;
        let mut thread: Object = _t0;
        let mut vthread: Object = thread;
        vthread.park()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "parkVirtualThread", descriptor = "(J)V", access = "public"))]
    // java: parkVirtualThread(J)V
    pub fn parkVirtualThread__l(&self, nanos: i64) -> Result<()> {
        let this = self;
        let _t0: Object = Thread::currentThread()?;
        let mut thread: Object = _t0;
        let mut vthread: Object = thread;
        vthread.parkNanos(nanos)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "unparkVirtualThread", descriptor = "(Ljava/lang/Thread;)V", access = "public"))]
    pub fn unparkVirtualThread(&self, thread: Object) -> Result<()> {
        let this = self;
        let mut vthread: Object = thread;
        vthread.unpark()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "newStackWalkerInstance", descriptor = "(Ljava/util/Set;Ljdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;)Ljava/lang/StackWalker;", access = "public"))]
    pub fn newStackWalkerInstance(&self, options: Object, contScope: Object, continuation: Object) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        let _t0: Object = StackWalker::newInstance(todo!("stack underflow"), options, contScope, continuation)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getLoaderNameID", descriptor = "(Ljava/lang/ClassLoader;)Ljava/lang/String;", access = "public"))]
    pub fn getLoaderNameID(&self, loader: Object) -> Result<String> {
        let this = self;
        let _t0 = loader.nameAndId()?;
        Ok(String::from("null"))
    }

    #[cfg_attr(any(), java_method(name = "allowSecurityManager", descriptor = "()Z", access = "public"))]
    pub fn allowSecurityManager(&self) -> Result<bool> {
        let this = self;
        let _t0: bool = System::allowSecurityManager()?;
        Ok(_t0)
    }
}
