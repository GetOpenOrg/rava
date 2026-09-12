#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/ObjectInputStream",
    super_class = "java/io/InputStream",
    interfaces  = "java/io/ObjectInput,java/io/ObjectStreamConstants",
    access      = "public",
    source      = "ObjectInputStream.java",
))]
pub struct ObjectInputStream {
    #[cfg_attr(any(), java_field(name = "bin", descriptor = "Ljava/io/ObjectInputStream$BlockDataInputStream;", access = "private final"))]
    pub bin: Field<Object>,
    #[cfg_attr(any(), java_field(name = "vlist", descriptor = "Ljava/io/ObjectInputStream$ValidationList;", access = "private final"))]
    pub vlist: Field<Object>,
    #[cfg_attr(any(), java_field(name = "depth", descriptor = "J", access = "private"))]
    pub depth: Field<i64>,
    #[cfg_attr(any(), java_field(name = "totalObjectRefs", descriptor = "J", access = "private"))]
    pub totalObjectRefs: Field<i64>,
    #[cfg_attr(any(), java_field(name = "closed", descriptor = "Z", access = "private"))]
    pub closed: Field<bool>,
    #[cfg_attr(any(), java_field(name = "handles", descriptor = "Ljava/io/ObjectInputStream$HandleTable;", access = "private final"))]
    pub handles: Field<Object>,
    #[cfg_attr(any(), java_field(name = "passHandle", descriptor = "I", access = "private"))]
    pub passHandle: Field<i32>,
    #[cfg_attr(any(), java_field(name = "defaultDataEnd", descriptor = "Z", access = "private"))]
    pub defaultDataEnd: Field<bool>,
    #[cfg_attr(any(), java_field(name = "enableOverride", descriptor = "Z", access = "private final"))]
    pub enableOverride: Field<bool>,
    #[cfg_attr(any(), java_field(name = "enableResolve", descriptor = "Z", access = "private"))]
    pub enableResolve: Field<bool>,
    #[cfg_attr(any(), java_field(name = "curContext", descriptor = "Ljava/io/SerialCallbackContext;", access = "private"))]
    pub curContext: Field<Object>,
    #[cfg_attr(any(), java_field(name = "serialFilter", descriptor = "Ljava/io/ObjectInputFilter;", access = "private"))]
    pub serialFilter: Field<Object>,
    #[cfg_attr(any(), java_field(name = "streamFilterSet", descriptor = "Z", access = "private"))]
    pub streamFilterSet: Field<bool>,
}

impl ObjectInputStream {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/InputStream;)V", access = "public"))]
    // java: <init>(Ljava/io/InputStream;)V
    pub fn new__inputs(in_: Object) -> Result<Self> {
        let this = Self { bin: Field::new(Default::default()), vlist: Field::new(Default::default()), depth: Field::new(0), totalObjectRefs: Field::new(0), closed: Field::new(false), handles: Field::new(Default::default()), passHandle: Field::new(0), defaultDataEnd: Field::new(false), enableOverride: Field::new(false), enableResolve: Field::new(false), curContext: Field::new(Default::default()), serialFilter: Field::new(Default::default()), streamFilterSet: Field::new(false) };
        /* invokespecial Method java/io/InputStream.<init>:()V */
        this.passHandle.set(-1i32);
        this.defaultDataEnd.set(0i32);
        this.verifySubclass()?;
        this.bin.set(ObjectInputStream_BlockDataInputStream::new(this, in_)?);
        this.handles.set(ObjectInputStream_HandleTable::new(10i32)?);
        this.vlist.set(ObjectInputStream_ValidationList::new()?);
        this.streamFilterSet.set(0i32);
        let _t0: Object = ObjectInputFilter$Config::getSerialFilterFactorySingleton()?;
        /* TODO: aconst_null  */
        let _t1: Object = ObjectInputFilter$Config::getSerialFilter()?;
        let _t2 = this.apply(_t0, _t1)?;
        todo!("stack underflow").serialFilter.set(_t2);
        this.enableOverride.set(0i32);
        this.readStreamHeader()?;
        let _t3 = this.bin.get().setBlockDataMode(1i32)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "protected"))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { bin: Field::new(Default::default()), vlist: Field::new(Default::default()), depth: Field::new(0), totalObjectRefs: Field::new(0), closed: Field::new(false), handles: Field::new(Default::default()), passHandle: Field::new(0), defaultDataEnd: Field::new(false), enableOverride: Field::new(false), enableResolve: Field::new(false), curContext: Field::new(Default::default()), serialFilter: Field::new(Default::default()), streamFilterSet: Field::new(false) };
        /* invokespecial Method java/io/InputStream.<init>:()V */
        this.passHandle.set(-1i32);
        this.defaultDataEnd.set(0i32);
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(ObjectInputStream::SUBCLASS_IMPLEMENTATION_PERMISSION())?;
        /* TODO: aconst_null  */
        sm.bin.set(this);
        /* TODO: aconst_null  */
        todo!("stack underflow").handles.set(this);
        /* TODO: aconst_null  */
        todo!("stack underflow").vlist.set(this);
        this.streamFilterSet.set(0i32);
        let _t1: Object = ObjectInputFilter$Config::getSerialFilterFactorySingleton()?;
        /* TODO: aconst_null  */
        let _t2: Object = ObjectInputFilter$Config::getSerialFilter()?;
        let _t3 = this.apply(_t1, _t2)?;
        todo!("stack underflow").serialFilter.set(_t3);
        this.enableOverride.set(1i32);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "readObject", descriptor = "()Ljava/lang/Object;", access = "public final"))]
    // java: readObject()Ljava/lang/Object;
    pub fn readObject(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.readObject(96i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "readString", descriptor = "()Ljava/lang/String;", access = "private"))]
    // java: readString()Ljava/lang/String;
    pub fn readString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.readObject(102i32)?;
        return Ok(_t0);
        let mut cnf: i32 = todo!("stack underflow");
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "readObject", descriptor = "(Ljava/lang/Class;)Ljava/lang/Object;", access = "private final"))]
    // java: readObject(Ljava/lang/Class;)Ljava/lang/Object;
    pub fn readObject__class(&self, type_: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.readObjectOverride()?;
        return Ok(_t0);
        return Err(JvmError::Custom(String::from("athrow")));
        let mut outerHandle: i32 = this.passHandle.get();
        let _t1 = this.readObject0(type_, 0i32)?;
        let mut obj: Object = _t1;
        this.handles.get().markDependency(outerHandle, this.passHandle.get())?;
        let _t2 = this.handles.get().lookupException(this.passHandle.get())?;
        let mut ex: Object = _t2;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        this.vlist.get().doCallbacks()?;
        this.freeze()?;
        let mut local_5: Object = obj;
        this.passHandle.set(outerHandle);
        /* TODO: lcmp  */
        this.clear()?;
        return Ok(local_5);
        let mut local_6: i64 = 0i64;
        this.passHandle.set(outerHandle);
        /* TODO: lcmp  */
        this.clear()?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "readObjectOverride", descriptor = "()Ljava/lang/Object;", access = "protected"))]
    pub fn readObjectOverride(&self) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "readUnshared", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn readUnshared(&self) -> Result<Object> {
        let this = self;
        let mut outerHandle: i32 = this.passHandle.get();
        let _t0 = this.readObject0(96i32, 1i32)?;
        let mut obj: Object = _t0;
        this.handles.get().markDependency(outerHandle, this.passHandle.get())?;
        let _t1 = this.handles.get().lookupException(this.passHandle.get())?;
        let mut ex: Object = _t1;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        this.vlist.get().doCallbacks()?;
        this.freeze()?;
        let mut local_4: Object = obj;
        this.passHandle.set(outerHandle);
        /* TODO: lcmp  */
        this.clear()?;
        return Ok(local_4);
        let mut local_5: i64 = 0i64;
        this.passHandle.set(outerHandle);
        /* TODO: lcmp  */
        this.clear()?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "defaultReadObject", descriptor = "()V", access = "public"))]
    pub fn defaultReadObject(&self) -> Result<()> {
        let this = self;
        let mut ctx: Object = this.curContext.get();
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0 = ctx.getObj()?;
        let mut curObj: Object = _t0;
        let _t1 = ctx.getDesc()?;
        let mut curDesc: Object = _t1;
        let _t2 = this.bin.get().setBlockDataMode(0i32)?;
        let mut values: ObjectInputStream_FieldValues = ObjectInputStream_FieldValues::new(this, curDesc, 1i32)?;
        values.defaultCheckFieldValues(curObj)?;
        values.defaultSetFieldValues(curObj)?;
        let _t3 = this.bin.get().setBlockDataMode(1i32)?;
        let _t4 = curDesc.hasWriteObjectData()?;
        this.defaultDataEnd.set(1i32);
        let _t5 = this.handles.get().lookupException(this.passHandle.get())?;
        let mut ex: Object = _t5;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readFields", descriptor = "()Ljava/io/ObjectInputStream$GetField;", access = "public"))]
    pub fn readFields(&self) -> Result<Object> {
        let this = self;
        let mut ctx: Object = this.curContext.get();
        return Err(JvmError::Custom(String::from("athrow")));
        ctx.checkAndSetUsed()?;
        let _t0 = ctx.getDesc()?;
        let mut curDesc: Object = _t0;
        let _t1 = this.bin.get().setBlockDataMode(0i32)?;
        let mut values: ObjectInputStream_FieldValues = ObjectInputStream_FieldValues::new(this, curDesc, 0i32)?;
        let _t2 = this.bin.get().setBlockDataMode(1i32)?;
        let _t3 = curDesc.hasWriteObjectData()?;
        this.defaultDataEnd.set(1i32);
        Ok(values)
    }

    #[cfg_attr(any(), java_method(name = "registerValidation", descriptor = "(Ljava/io/ObjectInputValidation;I)V", access = "public"))]
    pub fn registerValidation(&self, obj: Object, prio: i32) -> Result<()> {
        let this = self;
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        this.vlist.get().register(obj, prio)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "resolveClass", descriptor = "(Ljava/io/ObjectStreamClass;)Ljava/lang/Class;", access = "protected"))]
    pub fn resolveClass(&self, desc: Object) -> Result<Object> {
        let this = self;
        let _t0 = desc.getName()?;
        let mut name: String = _t0;
        let _t1: Object = ObjectInputStream::latestUserDefinedLoader()?;
        let _t2: Object = Class::forName(name, 0i32, _t1)?;
        return Ok(_t2);
        let mut ex: i32 = todo!("stack underflow");
        let _t3 = ObjectInputStream::primClasses().get(name)?;
        let mut cl: Object = _t3;
        return Ok(cl);
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "resolveProxyClass", descriptor = "([Ljava/lang/String;)Ljava/lang/Class;", access = "protected"))]
    pub fn resolveProxyClass(&self, interfaces: Vec<String>) -> Result<Object> {
        let this = self;
        let _t0: Object = ObjectInputStream::latestUserDefinedLoader()?;
        let mut latestLoader: Object = _t0;
        /* TODO: aconst_null  */
        let mut nonPublicLoader: i32 = todo!("stack underflow");
        let mut hasNonPublicInterface: i32 = 0i32;
        let mut _arr1: Vec<Object> = Vec::with_capacity((interfaces.len() as i32) as usize);
        let mut classObjs: Vec<Object> = _arr1;
        let mut i: i32 = 0i32;
        loop {
            if i >= (interfaces.len() as i32) { break; }
            let _t0: Object = Class::forName(interfaces[i as usize].clone(), 0i32, latestLoader)?;
            let mut cl: Object = _t0;
            let _t1 = cl.getModifiers()?;
            let _t2 = cl.getClassLoader()?;
            return Err(JvmError::Custom(String::from("athrow")));
            let _t3 = cl.getClassLoader()?;
            nonPublicLoader = _t3;
            hasNonPublicInterface = 1i32;
            classObjs[i as usize] = cl;
            i = i.wrapping_add(1i32);
        }
        let _t2: Object = Proxy::getProxyClass(latestLoader, &classObjs)?;
        i = _t2;
        return Ok(i);
        i = nonPublicLoader;
        /* TODO: aconst_null  */
        let mut _obj3: ClassNotFoundException = ClassNotFoundException::new(ClassNotFoundException::new(), i)?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "resolveObject", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "protected"))]
    pub fn resolveObject(&self, obj: Object) -> Result<Object> {
        let this = self;
        Ok(obj)
    }

    #[cfg_attr(any(), java_method(name = "enableResolveObject", descriptor = "(Z)Z", access = "protected"))]
    pub fn enableResolveObject(&self, enable: bool) -> Result<bool> {
        let this = self;
        return Ok(enable);
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(ObjectInputStream::SUBSTITUTION_PERMISSION())?;
        this.enableResolve.set(enable);
        Ok(this.enableResolve.get()==0i32)
    }

    #[cfg_attr(any(), java_method(name = "readStreamHeader", descriptor = "()V", access = "protected"))]
    pub fn readStreamHeader(&self) -> Result<()> {
        let this = self;
        let _t0 = this.bin.get().readShort()?;
        let mut s0: i32 = _t0;
        let _t1 = this.bin.get().readShort()?;
        let mut s1: i32 = _t1;
        let mut _arr2: Vec<Object> = Vec::with_capacity(2i32 as usize);
        let _t3: Object = Short::valueOf(s0)?;
        _arr2[0i32 as usize] = _t3;
        let _t4: Object = Short::valueOf(s1)?;
        _arr2[1i32 as usize] = _t4;
        let _t5: String = String::format(String::from("invalid stream header: %04X%04X"), &_arr2)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readClassDescriptor", descriptor = "()Ljava/io/ObjectStreamClass;", access = "protected"))]
    pub fn readClassDescriptor(&self) -> Result<Object> {
        let this = self;
        let mut desc: ObjectStreamClass = ObjectStreamClass::new()?;
        desc.readNonProxy(this)?;
        Ok(desc)
    }

    #[cfg_attr(any(), java_method(name = "read", descriptor = "()I", access = "public"))]
    // java: read()I
    pub fn read(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.bin.get().read()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "read", descriptor = "([BII)I", access = "public"))]
    // java: read([BII)I
    pub fn read__arr_b_i_i(&self, buf: Vec<i8>, off: i32, len: i32) -> Result<i32> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: i32 = Objects::checkFromIndexSize(off, len, (buf.len() as i32))?;
        let _t1 = this.bin.get().read(buf, off, len, 0i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "available", descriptor = "()I", access = "public"))]
    pub fn available(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.bin.get().available()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "close", descriptor = "()V", access = "public"))]
    pub fn close(&self) -> Result<()> {
        let this = self;
        this.closed.set(1i32);
        /* TODO: lcmp  */
        this.clear()?;
        this.bin.get().close()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readBoolean", descriptor = "()Z", access = "public"))]
    pub fn readBoolean(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.bin.get().readBoolean()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "readByte", descriptor = "()B", access = "public"))]
    pub fn readByte(&self) -> Result<i8> {
        let this = self;
        let _t0 = this.bin.get().readByte()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "readUnsignedByte", descriptor = "()I", access = "public"))]
    pub fn readUnsignedByte(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.bin.get().readUnsignedByte()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "readChar", descriptor = "()C", access = "public"))]
    pub fn readChar(&self) -> Result<u16> {
        let this = self;
        let _t0 = this.bin.get().readChar()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "readShort", descriptor = "()S", access = "public"))]
    pub fn readShort(&self) -> Result<i16> {
        let this = self;
        let _t0 = this.bin.get().readShort()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "readUnsignedShort", descriptor = "()I", access = "public"))]
    pub fn readUnsignedShort(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.bin.get().readUnsignedShort()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "readInt", descriptor = "()I", access = "public"))]
    pub fn readInt(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.bin.get().readInt()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "readLong", descriptor = "()J", access = "public"))]
    pub fn readLong(&self) -> Result<i64> {
        let this = self;
        let _t0 = this.bin.get().readLong()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "readFloat", descriptor = "()F", access = "public"))]
    pub fn readFloat(&self) -> Result<f32> {
        let this = self;
        let _t0 = this.bin.get().readFloat()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "readDouble", descriptor = "()D", access = "public"))]
    pub fn readDouble(&self) -> Result<f64> {
        let this = self;
        let _t0 = this.bin.get().readDouble()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "readFully", descriptor = "([B)V", access = "public"))]
    // java: readFully([B)V
    pub fn readFully__arr_b(&self, buf: Vec<i8>) -> Result<()> {
        let this = self;
        this.bin.get().readFully(buf, 0i32, (buf.len() as i32), 0i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readFully", descriptor = "([BII)V", access = "public"))]
    // java: readFully([BII)V
    pub fn readFully__arr_b_i_i(&self, buf: Vec<i8>, off: i32, len: i32) -> Result<()> {
        let this = self;
        let _t0: i32 = Objects::checkFromIndexSize(off, len, (buf.len() as i32))?;
        this.bin.get().readFully(buf, off, len, 0i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "skipBytes", descriptor = "(I)I", access = "public"))]
    pub fn skipBytes(&self, len: i32) -> Result<i32> {
        let this = self;
        let _t0 = this.bin.get().skipBytes(len)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "readLine", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn readLine(&self) -> Result<String> {
        let this = self;
        let _t0 = this.bin.get().readLine()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "readUTF", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn readUTF(&self) -> Result<String> {
        let this = self;
        let _t0 = this.bin.get().readUTF()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getObjectInputFilter", descriptor = "()Ljava/io/ObjectInputFilter;", access = "public final"))]
    pub fn getObjectInputFilter(&self) -> Result<Object> {
        let this = self;
        Ok(this.serialFilter.get())
    }

    #[cfg_attr(any(), java_method(name = "setObjectInputFilter", descriptor = "(Ljava/io/ObjectInputFilter;)V", access = "public final"))]
    pub fn setObjectInputFilter(&self, filter: Object) -> Result<()> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(ObjectStreamConstants::SERIAL_FILTER_PERMISSION())?;
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        this.streamFilterSet.set(1i32);
        let _t1: Object = ObjectInputFilter$Config::getSerialFilterFactory()?;
        let _t2 = _t1.apply(this.serialFilter.get(), filter)?;
        let mut next: Object = _t2;
        return Err(JvmError::Custom(String::from("athrow")));
        this.serialFilter.set(next);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "filterCheck", descriptor = "(Ljava/lang/Class;I)V", access = "private"))]
    pub fn filterCheck(&self, clazz: Object, arrayLength: i32) -> Result<()> {
        let this = self;
        let _t0 = this.bin.get().getBytesRead()?;
        let mut bytesRead: i64 = _t0;
        /* TODO: aconst_null  */
        let mut ex: i64 = 0i64;
        /* TODO: aconst_null  */
        let mut status: Object = this.bin.get();
        let _t1 = this.serialFilter.get().checkInput(ObjectInputStream_FilterValues::new(clazz, (arrayLength as i64), this.totalObjectRefs.get(), this.depth.get(), bytesRead)?)?;
        status = _t1;
        let mut e: Object = this.serialFilter.get();
        status = ObjectInputFilter$Status::REJECTED();
        ex = e;
        let mut _arr2: Vec<Object> = Vec::with_capacity(7i32 as usize);
        _arr2[0i32 as usize] = status;
        _arr2[1i32 as usize] = clazz;
        _arr2[2i32 as usize] = arrayLength;
        _arr2[3i32 as usize] = this.totalObjectRefs.get();
        _arr2[4i32 as usize] = this.depth.get();
        _arr2[5i32 as usize] = bytesRead;
        let _t3: String = Objects::toString(ex, String::from("n/a"))?;
        _arr2[6i32 as usize] = _t3;
        System$Logger$Level::DEBUG().log(System$Logger$Level::TRACE(), String::from("ObjectInputFilter {0}: {1}, array length: {2}, nRefs: {3}, depth: {4}, bytes: {5}, ex: {6}"), _arr2)?;
        e = DeserializationEvent::new()?;
        let _t4 = e.shouldCommit()?;
        e.filterConfigured.set(!this.serialFilter.get().is_none());
        let _t5 = status.name()?;
        /* TODO: aconst_null  */
        status.filterStatus.set(_t5);
        e.type.set(clazz);
        e.arrayLength.set(arrayLength);
        e.objectReferences.set(this.totalObjectRefs.get());
        e.depth.set(this.depth.get());
        e.bytesRead.set(bytesRead);
        let _t6 = ex.getClass()?;
        /* TODO: aconst_null  */
        ex.exceptionType.set(_t6);
        let _t7 = ex.getMessage()?;
        /* TODO: aconst_null  */
        ex.exceptionMessage.set(_t7);
        e.commit()?;
        String::new().append(&String::from("filter status:"))?;
        String::new().append(&status)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkArray", descriptor = "(Ljava/lang/Class;I)V", access = "private"))]
    pub fn checkArray(&self, arrayType: Object, arrayLength: i32) -> Result<()> {
        let this = self;
        let _t0 = arrayType.isArray()?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        this.filterCheck(arrayType, arrayLength)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "verifySubclass", descriptor = "()V", access = "private"))]
    pub fn verifySubclass(&self) -> Result<()> {
        let this = self;
        let _t0 = this.getClass()?;
        let mut cl: Object = _t0;
        return Ok(());
        let _t1: Object = System::getSecurityManager()?;
        let mut sm: Object = _t1;
        return Ok(());
        let _t2 = ObjectInputStream$Caches::subclassAudits().get(cl)?;
        let mut result: i32 = _t2;
        sm.checkPermission(ObjectInputStream::SUBCLASS_IMPLEMENTATION_PERMISSION())?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "auditSubclass", descriptor = "(Ljava/lang/Class;)Ljava/lang/Boolean;", access = "private static"))]
    pub fn auditSubclass(subcl: Object) -> Result<bool> {
        let _t0: Object = AccessController::doPrivileged(ObjectInputStream_1::new(subcl)?)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()V", access = "private"))]
    pub fn clear(&self) -> Result<()> {
        let this = self;
        this.handles.get().clear()?;
        this.vlist.get().clear()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readObject0", descriptor = "(Ljava/lang/Class;Z)Ljava/lang/Object;", access = "private"))]
    pub fn readObject0(&self, type_: Object, unshared: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.bin.get().getBlockDataMode()?;
        let mut oldMode: i32 = _t0;
        let _t1 = this.bin.get().currentBlockRemaining()?;
        let mut remain: i32 = _t1;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let _t2 = this.bin.get().setBlockDataMode(0i32)?;
        loop {
            let _t0 = this.bin.get().peekByte()?;
            remain = _t0;
            if _t0 != 121i32 { break; }
            let _t0 = this.bin.get().readByte()?;
            this.handleReset()?;
        }
        this.depth.set((this.depth.get()).wrapping_add(1i64));
        this.totalObjectRefs.set((this.totalObjectRefs.get()).wrapping_add(1i64));
        /* TODO: tableswitch default:631 low:112 high:126 */
        let _t3 = this.readNull()?;
        let mut ex: Object = _t3;
        this.depth.set((this.depth.get()).wrapping_sub(1i64));
        let _t4 = this.bin.get().setBlockDataMode(oldMode)?;
        return Ok(ex);
        let _t5 = this.readHandle(unshared)?;
        let _t6 = type_.cast(_t5)?;
        ex = _t6;
        this.depth.set((this.depth.get()).wrapping_sub(1i64));
        let _t7 = this.bin.get().setBlockDataMode(oldMode)?;
        return Ok(ex);
        return Err(JvmError::Custom(String::from("athrow")));
        let _t8 = this.readClass(unshared)?;
        ex = _t8;
        this.depth.set((this.depth.get()).wrapping_sub(1i64));
        let _t9 = this.bin.get().setBlockDataMode(oldMode)?;
        return Ok(ex);
        return Err(JvmError::Custom(String::from("athrow")));
        let _t10 = this.readClassDesc(unshared)?;
        ex = _t10;
        this.depth.set((this.depth.get()).wrapping_sub(1i64));
        let _t11 = this.bin.get().setBlockDataMode(oldMode)?;
        return Ok(ex);
        let _t12 = this.readString(unshared)?;
        let _t13 = this.checkResolve(_t12)?;
        ex = _t13;
        this.depth.set((this.depth.get()).wrapping_sub(1i64));
        let _t14 = this.bin.get().setBlockDataMode(oldMode)?;
        return Ok(ex);
        return Err(JvmError::Custom(String::from("athrow")));
        let _t15 = this.readArray(unshared)?;
        let _t16 = this.checkResolve(_t15)?;
        ex = _t16;
        this.depth.set((this.depth.get()).wrapping_sub(1i64));
        let _t17 = this.bin.get().setBlockDataMode(oldMode)?;
        return Ok(ex);
        return Err(JvmError::Custom(String::from("athrow")));
        let _t18 = this.readEnum(unshared)?;
        let _t19 = this.checkResolve(_t18)?;
        ex = _t19;
        this.depth.set((this.depth.get()).wrapping_sub(1i64));
        let _t20 = this.bin.get().setBlockDataMode(oldMode)?;
        return Ok(ex);
        return Err(JvmError::Custom(String::from("athrow")));
        let _t21 = this.readOrdinaryObject(unshared)?;
        let _t22 = this.checkResolve(_t21)?;
        ex = _t22;
        this.depth.set((this.depth.get()).wrapping_sub(1i64));
        let _t23 = this.bin.get().setBlockDataMode(oldMode)?;
        return Ok(ex);
        return Err(JvmError::Custom(String::from("athrow")));
        let _t24 = this.readFatalException()?;
        ex = _t24;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t25 = this.bin.get().setBlockDataMode(1i32)?;
        let _t26 = this.bin.get().peek()?;
        let _t27 = this.bin.get().currentBlockRemaining()?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let mut _arr28: Vec<Object> = Vec::with_capacity(1i32 as usize);
        let _t29: Object = Byte::valueOf(remain)?;
        _arr28[0i32 as usize] = _t29;
        let _t30: String = String::format(String::from("invalid type code: %02X"), &_arr28)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_6: i32 = oldMode;
        this.depth.set((this.depth.get()).wrapping_sub(1i64));
        let _t31 = this.bin.get().setBlockDataMode(oldMode)?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "checkResolve", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "private"))]
    pub fn checkResolve(&self, obj: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.handles.get().lookupException(this.passHandle.get())?;
        return Ok(obj);
        let _t1 = this.resolveObject(obj)?;
        let mut rep: Object = _t1;
        let _t2 = rep.getClass()?;
        let _t3 = _t2.isArray()?;
        let _t4 = rep.getClass()?;
        let _t5: i32 = Array::getLength(rep)?;
        this.filterCheck(_t4, _t5)?;
        let _t6 = rep.getClass()?;
        this.filterCheck(_t6, -1i32)?;
        this.handles.get().setObject(this.passHandle.get(), rep)?;
        Ok(rep)
    }

    #[cfg_attr(any(), java_method(name = "readTypeString", descriptor = "()Ljava/lang/String;"))]
    pub fn readTypeString(&self) -> Result<String> {
        let this = self;
        let mut oldHandle: i32 = this.passHandle.get();
        let _t0 = this.bin.get().peekByte()?;
        let mut tc: i32 = _t0;
        /* TODO: lookupswitch default:85 112:56 113:66 116:77 124:77 */
        let _t1 = this.readNull()?;
        let _t2 = this.readHandle(0i32)?;
        let _t3 = this.readString(0i32)?;
        let mut _arr4: Vec<Object> = Vec::with_capacity(1i32 as usize);
        let _t5: Object = Byte::valueOf(tc)?;
        _arr4[0i32 as usize] = _t5;
        let _t6: String = String::format(String::from("invalid type code: %02X"), &_arr4)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_3: String = _t3;
        this.passHandle.set(oldHandle);
        return Ok(local_3);
        let mut local_4: Object = _t2;
        this.passHandle.set(oldHandle);
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "readNull", descriptor = "()Ljava/lang/Object;", access = "private"))]
    pub fn readNull(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.bin.get().readByte()?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.passHandle.set(-1i32);
        /* TODO: aconst_null  */
        Ok(112i32)
    }

    #[cfg_attr(any(), java_method(name = "readHandle", descriptor = "(Z)Ljava/lang/Object;", access = "private"))]
    pub fn readHandle(&self, unshared: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.bin.get().readByte()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1 = this.bin.get().readInt()?;
        this.passHandle.set((_t1).wrapping_sub(632i32));
        let _t2 = this.handles.get().size()?;
        let mut _arr3: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr3[0i32 as usize] = (this.passHandle.get()).wrapping_add(632i32);
        let _t4: String = String::format(String::from("invalid handle value: %08X"), &_arr3)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let _t5 = this.handles.get().lookupObject(this.passHandle.get())?;
        let mut obj: Object = _t5;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: aconst_null  */
        ObjectInputStream::unsharedMarker().filterCheck(this, -1i32)?;
        Ok(obj)
    }

    #[cfg_attr(any(), java_method(name = "readClass", descriptor = "(Z)Ljava/lang/Class;", access = "private"))]
    pub fn readClass(&self, unshared: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.bin.get().readByte()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1 = this.readClassDesc(0i32)?;
        let mut desc: Object = _t1;
        let _t2 = desc.forClass()?;
        let mut cl: Object = _t2;
        let _t3 = ObjectInputStream::unsharedMarker().assign(cl)?;
        unshared.passHandle.set(_t3);
        let _t4 = desc.getResolveException()?;
        let mut resolveEx: Object = _t4;
        this.handles.get().markException(this.passHandle.get(), resolveEx)?;
        this.handles.get().finish(this.passHandle.get())?;
        Ok(cl)
    }

    #[cfg_attr(any(), java_method(name = "readClassDesc", descriptor = "(Z)Ljava/io/ObjectStreamClass;", access = "private"))]
    pub fn readClassDesc(&self, unshared: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.bin.get().peekByte()?;
        let mut tc: i32 = _t0;
        /* TODO: lookupswitch default:95 112:52 113:78 114:70 125:62 */
        let _t1 = this.readNull()?;
        let _t2 = this.readProxyDesc(unshared)?;
        let _t3 = this.readNonProxyDesc(unshared)?;
        let _t4 = this.readHandle(unshared)?;
        let mut d: Object = _t4;
        d.checkInitialized()?;
        let mut _arr5: Vec<Object> = Vec::with_capacity(1i32 as usize);
        let _t6: Object = Byte::valueOf(tc)?;
        _arr5[0i32 as usize] = _t6;
        let _t7: String = String::format(String::from("invalid type code: %02X"), &_arr5)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(d)
    }

    #[cfg_attr(any(), java_method(name = "isCustomSubclass", descriptor = "()Z", access = "private"))]
    pub fn isCustomSubclass(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.getClass()?;
        let _t1 = _t0.getClassLoader()?;
        let _t2 = 7i32.getClassLoader()?;
        Ok(/* if_acmpeq */ true)
    }

    #[cfg_attr(any(), java_method(name = "readProxyDesc", descriptor = "(Z)Ljava/io/ObjectStreamClass;", access = "private"))]
    pub fn readProxyDesc(&self, unshared: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.bin.get().readByte()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut desc: ObjectStreamClass = ObjectStreamClass::new()?;
        let _t1 = ObjectInputStream::unsharedMarker().assign(desc)?;
        let mut descHandle: i32 = _t1;
        this.passHandle.set(-1i32);
        let _t2 = this.bin.get().readInt()?;
        let mut numIfaces: i32 = _t2;
        String::new().append(&String::from("interface limit exceeded:"))?;
        String::new().append(&numIfaces)?;
        String::new().append(&String::from(", limit:"))?;
        String::new().append(&ObjectInputStream$Caches::PROXY_INTERFACE_LIMIT())?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut _arr3: Vec<Object> = Vec::with_capacity(numIfaces as usize);
        let mut ifaces: Vec<Object> = _arr3;
        let mut i: i32 = 0i32;
        loop {
            if i >= numIfaces { break; }
            let _t0 = this.bin.get().readUTF()?;
            ifaces[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        String::new().append(&String::from("interface limit exceeded:"))?;
        String::new().append(&numIfaces)?;
        String::new().append(&String::from(", limit:"))?;
        String::new().append(&ObjectInputStream$Caches::PROXY_INTERFACE_LIMIT())?;
        String::new().append(&String::from(";"))?;
        let _t4: String = Arrays::toString(&ifaces)?;
        String::new().append(&_t4)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: aconst_null  */
        i = ObjectInputStream$Caches::PROXY_INTERFACE_LIMIT();
        /* TODO: aconst_null  */
        let mut resolveEx: i32 = numIfaces;
        let _t5 = this.bin.get().setBlockDataMode(1i32)?;
        let _t6 = this.resolveProxyClass(ifaces)?;
        i = _t6;
        resolveEx = ClassNotFoundException::new(String::from("null class"))?;
        let _t7: bool = Proxy::isProxyClass(i)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t8 = this.getClass()?;
        let _t9 = _t8.getClassLoader()?;
        let _t10 = i.getInterfaces()?;
        ReflectUtil::checkProxyPackageAccess(_t9, &_t10)?;
        let _t11 = i.getInterfaces()?;
        let mut ex: Vec<Object> = _t11;
        let mut local_9: i32 = (ex.len() as i32);
        let mut local_10: i32 = 0i32;
        loop {
            if local_10 >= local_9 { break; }
            let mut clazz: Object = ex[local_10 as usize].clone();
            this.filterCheck(clazz, -1i32)?;
            local_10 = local_10.wrapping_add(1i32);
        }
        ex = _t7;
        resolveEx = ex;
        ex = _t6;
        let _t12 = ex.getMessage()?;
        return Err(JvmError::Custom(String::from("athrow")));
        ex = 679i32;
        String::new().append(&String::from("Proxy interface limit exceeded:"))?;
        let _t13: String = Arrays::toString(&ifaces)?;
        String::new().append(&_t13)?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.filterCheck(i, -1i32)?;
        this.skipCustomData()?;
        this.totalObjectRefs.set((this.totalObjectRefs.get()).wrapping_add(1i64));
        this.depth.set((this.depth.get()).wrapping_add(1i64));
        let _t14 = this.readClassDesc(0i32)?;
        desc.initProxy(i, resolveEx, _t14)?;
        this.depth.set((this.depth.get()).wrapping_sub(1i64));
        ex = numIfaces;
        String::new().append(&String::from("Proxy interface limit exceeded:"))?;
        let _t15: String = Arrays::toString(&ifaces)?;
        String::new().append(&_t15)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_12: bool = unshared;
        this.depth.set((this.depth.get()).wrapping_sub(1i64));
        return Err(JvmError::Custom(String::from("athrow")));
        this.handles.get().finish(descHandle)?;
        this.passHandle.set(descHandle);
        Ok(desc)
    }

    #[cfg_attr(any(), java_method(name = "readNonProxyDesc", descriptor = "(Z)Ljava/io/ObjectStreamClass;", access = "private"))]
    pub fn readNonProxyDesc(&self, unshared: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.bin.get().readByte()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut desc: ObjectStreamClass = ObjectStreamClass::new()?;
        let _t1 = ObjectInputStream::unsharedMarker().assign(desc)?;
        let mut descHandle: i32 = _t1;
        this.passHandle.set(-1i32);
        let _t2 = this.readClassDescriptor()?;
        let mut readDesc: Object = _t2;
        let mut ex: bool = unshared;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: aconst_null  */
        ex = this.handles.get();
        /* TODO: aconst_null  */
        let mut resolveEx: i32 = 114i32;
        let _t3 = this.bin.get().setBlockDataMode(1i32)?;
        let _t4 = this.isCustomSubclass()?;
        let mut checksRequired: i32 = _t4;
        let _t5 = this.resolveClass(readDesc)?;
        ex = _t5;
        resolveEx = ClassNotFoundException::new(String::from("null class"))?;
        ReflectUtil::checkPackageAccess(ex)?;
        let mut ex: i32 = checksRequired;
        resolveEx = ex;
        this.filterCheck(ex, -1i32)?;
        this.skipCustomData()?;
        this.totalObjectRefs.set((this.totalObjectRefs.get()).wrapping_add(1i64));
        this.depth.set((this.depth.get()).wrapping_add(1i64));
        let _t6 = this.readClassDesc(0i32)?;
        desc.initNonProxy(readDesc, ex, resolveEx, _t6)?;
        /* TODO: aconst_null  */
        ex = ex;
        let _t7 = desc.getSuperDesc()?;
        let mut sDesc: Object = _t7;
        loop {
            if sDesc.is_none() { break; }
            let _t0 = sDesc.getLocalDesc()?;
            ex = _t0;
            let _t1 = sDesc.getSuperDesc()?;
            sDesc = _t1;
        }
        let _t8 = desc.getLocalDesc()?;
        let _t9 = _t8.getSuperDesc()?;
        sDesc = _t9;
        loop {
            if sDesc.is_none() { break; }
            let _t0 = sDesc.forClass()?;
            this.filterCheck(_t0, -1i32)?;
            let _t1 = sDesc.getSuperDesc()?;
            sDesc = _t1;
        }
        this.depth.set((this.depth.get()).wrapping_sub(1i64));
        let mut local_10: Object = _t5;
        this.depth.set((this.depth.get()).wrapping_sub(1i64));
        return Err(JvmError::Custom(String::from("athrow")));
        this.handles.get().finish(descHandle)?;
        this.passHandle.set(descHandle);
        Ok(desc)
    }

    #[cfg_attr(any(), java_method(name = "readString", descriptor = "(Z)Ljava/lang/String;", access = "private"))]
    // java: readString(Z)Ljava/lang/String;
    pub fn readString__z(&self, unshared: bool) -> Result<String> {
        let this = self;
        let _t0 = this.bin.get().readByte()?;
        let mut tc: i32 = _t0;
        /* TODO: lookupswitch default:56 116:36 124:46 */
        let _t1 = this.bin.get().readUTF()?;
        let _t2 = this.bin.get().readLongUTF()?;
        let mut _arr3: Vec<Object> = Vec::with_capacity(1i32 as usize);
        let _t4: Object = Byte::valueOf(tc)?;
        _arr3[0i32 as usize] = _t4;
        let _t5: String = String::format(String::from("invalid type code: %02X"), &_arr3)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut str: String = _t2;
        let _t6 = ObjectInputStream::unsharedMarker().assign(str)?;
        unshared.passHandle.set(_t6);
        this.handles.get().finish(this.passHandle.get())?;
        Ok(str)
    }

    #[cfg_attr(any(), java_method(name = "readArray", descriptor = "(Z)Ljava/lang/Object;", access = "private"))]
    pub fn readArray(&self, unshared: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.bin.get().readByte()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1 = this.readClassDesc(0i32)?;
        let mut desc: Object = _t1;
        let _t2 = this.bin.get().readInt()?;
        let mut len: i32 = _t2;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t3 = desc.forClass()?;
        this.filterCheck(_t3, len)?;
        /* TODO: aconst_null  */
        let mut array: i32 = len;
        /* TODO: aconst_null  */
        let mut ccl: i32 = 117i32;
        let _t4 = desc.forClass()?;
        let mut cl: Object = _t4;
        let _t5 = cl.getComponentType()?;
        ccl = _t5;
        let _t6: Object = Array::newInstance(ccl, len)?;
        array = _t6;
        let _t7 = ObjectInputStream::unsharedMarker().assign(array)?;
        let mut arrayHandle: i32 = _t7;
        let _t8 = desc.getResolveException()?;
        let mut resolveEx: Object = _t8;
        this.handles.get().markException(arrayHandle, resolveEx)?;
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            let _t0 = this.readObject0(96i32, 0i32)?;
            i = i.wrapping_add(1i32);
        }
        let _t9 = ccl.isPrimitive()?;
        this.bin.get().readInts(array, 0i32, len)?;
        this.bin.get().readFully(array, 0i32, len, 1i32)?;
        this.bin.get().readLongs(array, 0i32, len)?;
        this.bin.get().readFloats(array, 0i32, len)?;
        this.bin.get().readDoubles(array, 0i32, len)?;
        this.bin.get().readShorts(array, 0i32, len)?;
        this.bin.get().readChars(array, 0i32, len)?;
        this.bin.get().readBooleans(array, 0i32, len)?;
        return Err(JvmError::Custom(String::from("athrow")));
        i = array;
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            let _t0 = this.readObject0(96i32, 0i32)?;
            i[i as usize] = _t0;
            this.handles.get().markDependency(arrayHandle, this.passHandle.get())?;
            i = i.wrapping_add(1i32);
        }
        this.handles.get().finish(arrayHandle)?;
        this.passHandle.set(arrayHandle);
        Ok(array)
    }

    #[cfg_attr(any(), java_method(name = "readEnum", descriptor = "(Z)Ljava/lang/Enum;", access = "private"))]
    pub fn readEnum(&self, unshared: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.bin.get().readByte()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1 = this.readClassDesc(0i32)?;
        let mut desc: Object = _t1;
        let _t2 = desc.isEnum()?;
        String::new().append(&String::from("non-enum class:"))?;
        String::new().append(&desc)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: aconst_null  */
        let _t3 = unshared.assign(ObjectInputStream::unsharedMarker())?;
        let mut enumHandle: i32 = _t3;
        let _t4 = desc.getResolveException()?;
        let mut resolveEx: Object = _t4;
        this.handles.get().markException(enumHandle, resolveEx)?;
        let _t5 = this.readString(0i32)?;
        let mut name: String = _t5;
        /* TODO: aconst_null  */
        let mut result: Object = resolveEx;
        let _t6 = desc.forClass()?;
        let mut cl: Object = _t6;
        let _t7: Object = Enum::valueOf(cl, name)?;
        let mut en: Object = _t7;
        result = en;
        en = cl;
        String::new().append(&String::from("enum constant"))?;
        String::new().append(&name)?;
        String::new().append(&String::from("does not exist in"))?;
        String::new().append(&cl)?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.handles.get().setObject(enumHandle, result)?;
        this.handles.get().finish(enumHandle)?;
        this.passHandle.set(enumHandle);
        Ok(result)
    }

    #[cfg_attr(any(), java_method(name = "readOrdinaryObject", descriptor = "(Z)Ljava/lang/Object;", access = "private"))]
    pub fn readOrdinaryObject(&self, unshared: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.bin.get().readByte()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1 = this.readClassDesc(0i32)?;
        let mut desc: Object = _t1;
        desc.checkDeserialize()?;
        let _t2 = desc.forClass()?;
        let mut cl: Object = _t2;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t3 = desc.isInstantiable()?;
        let _t4 = desc.newInstance()?;
        /* TODO: aconst_null  */
        let mut obj: Object = _t4;
        let mut ex: bool = _t3;
        let _t5 = desc.forClass()?;
        let _t6 = _t5.getName()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t7 = ObjectInputStream::unsharedMarker().assign(obj)?;
        unshared.passHandle.set(_t7);
        let _t8 = desc.getResolveException()?;
        ex = _t8;
        this.handles.get().markException(this.passHandle.get(), ex)?;
        let _t9 = desc.isRecord()?;
        let mut isRecord: i32 = _t9;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t10 = this.readRecord(desc)?;
        obj = _t10;
        this.handles.get().setObject(this.passHandle.get(), obj)?;
        let _t11 = desc.isExternalizable()?;
        this.readExternalData(obj, desc)?;
        this.readSerialData(obj, desc)?;
        this.handles.get().finish(this.passHandle.get())?;
        let _t12 = this.handles.get().lookupException(this.passHandle.get())?;
        let _t13 = desc.hasReadResolveMethod()?;
        let _t14 = desc.invokeReadResolve(obj)?;
        let mut rep: Object = _t14;
        let _t15 = rep.getClass()?;
        let _t16 = _t15.isArray()?;
        let _t17: Object = ObjectInputStream::cloneArray(rep)?;
        rep = _t17;
        let _t18 = rep.getClass()?;
        let _t19 = _t18.isArray()?;
        let _t20 = rep.getClass()?;
        let _t21: i32 = Array::getLength(rep)?;
        this.filterCheck(_t20, _t21)?;
        let _t22 = rep.getClass()?;
        this.filterCheck(_t22, -1i32)?;
        obj = rep;
        this.handles.get().setObject(this.passHandle.get(), rep)?;
        Ok(obj)
    }

    #[cfg_attr(any(), java_method(name = "readExternalData", descriptor = "(Ljava/io/Externalizable;Ljava/io/ObjectStreamClass;)V", access = "private"))]
    pub fn readExternalData(&self, obj: Object, desc: Object) -> Result<()> {
        let this = self;
        let mut oldContext: Object = this.curContext.get();
        oldContext.check()?;
        /* TODO: aconst_null  */
        oldContext.curContext.set(this);
        let _t0 = desc.hasBlockExternalData()?;
        let mut blocked: i32 = _t0;
        let _t1 = this.bin.get().setBlockDataMode(1i32)?;
        obj.readExternal(this)?;
        let mut ex: Object = obj;
        this.handles.get().markException(this.passHandle.get(), ex)?;
        this.skipCustomData()?;
        oldContext.check()?;
        this.curContext.set(oldContext);
        let mut local_6: Object = oldContext;
        oldContext.check()?;
        this.curContext.set(oldContext);
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readRecord", descriptor = "(Ljava/io/ObjectStreamClass;)Ljava/lang/Object;", access = "private"))]
    pub fn readRecord(&self, desc: Object) -> Result<Object> {
        let this = self;
        let _t0 = desc.getClassDataLayout()?;
        let mut slots: Vec<Object> = _t0;
        let mut i: i32 = 0i32;
        loop {
            if i >= ((slots.len() as i32)).wrapping_sub(1i32) { break; }
            let _ = ObjectInputStream_FieldValues::new(this, slots[i as usize].clone().desc.get(), 1i32)?;
            i = i.wrapping_add(1i32);
        }
        i = ObjectInputStream_FieldValues::new(this, desc, 1i32)?;
        let _t1: Object = ObjectStreamClass$RecordSupport::deserializationCtr(desc)?;
        let mut ctrMH: Object = _t1;
        let _t2 = ctrMH.invokeExact(i.primValues.get(), i.objValues.get())?;
        return Ok(_t2);
        let mut e: i32 = 1i32;
        let _t3 = e.getMessage()?;
        return Err(JvmError::Custom(String::from("athrow")));
        e = (slots.len() as i32);
        return Err(JvmError::Custom(String::from("athrow")));
        e = todo!("stack underflow");
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "readSerialData", descriptor = "(Ljava/lang/Object;Ljava/io/ObjectStreamClass;)V", access = "private"))]
    pub fn readSerialData(&self, obj: Object, desc: Object) -> Result<()> {
        let this = self;
        let _t0 = desc.getClassDataLayout()?;
        let mut slots: Vec<Object> = _t0;
        /* TODO: aconst_null  */
        let mut slotValues: i32 = todo!("stack underflow");
        let mut hasSpecialReadMethod: i32 = 0i32;
        let mut i: i32 = 1i32;
        loop {
            if i >= (slots.len() as i32) { break; }
            let mut slotDesc: Object = slots[i as usize].clone().desc.get();
            let _t0 = slotDesc.hasReadObjectMethod()?;
            let _t1 = slotDesc.hasReadObjectNoDataMethod()?;
            hasSpecialReadMethod = 1i32;
            i = i.wrapping_add(1i32);
        }
        let mut _arr1: Vec<Object> = Vec::with_capacity((slots.len() as i32) as usize);
        slotValues = _arr1;
        i = 0i32;
        loop {
            if i >= (slots.len() as i32) { break; }
            slotDesc = slots[i as usize].clone().desc.get();
            let _t0 = this.handles.get().lookupException(this.passHandle.get())?;
            let _ = ObjectInputStream_FieldValues::new(this, slotDesc, 1i32)?;
            let _t1 = slotDesc.hasReadObjectMethod()?;
            let mut oldContext: Object = this.curContext.get();
            oldContext.check()?;
            this.curContext.set(SerialCallbackContext::new(obj, slotDesc)?);
            let _t2 = this.bin.get().setBlockDataMode(1i32)?;
            slotDesc.invokeReadObject(obj, this)?;
            this.curContext.get().setUsed()?;
            oldContext.check()?;
            this.curContext.set(oldContext);
            let mut ex: Object = oldContext;
            this.handles.get().markException(this.passHandle.get(), ex)?;
            this.curContext.get().setUsed()?;
            oldContext.check()?;
            this.curContext.set(oldContext);
            let mut local_10: Object = oldContext;
            this.curContext.get().setUsed()?;
            oldContext.check()?;
            this.curContext.set(oldContext);
            return Err(JvmError::Custom(String::from("athrow")));
            this.defaultDataEnd.set(0i32);
            oldContext = ObjectInputStream_FieldValues::new(this, slotDesc, 1i32)?;
            slotValues[i as usize] = oldContext;
            oldContext.defaultCheckFieldValues(obj)?;
            oldContext.defaultSetFieldValues(obj)?;
            let _t3 = slotDesc.hasWriteObjectData()?;
            this.skipCustomData()?;
            let _t4 = this.bin.get().setBlockDataMode(0i32)?;
            let _t5 = slotDesc.hasReadObjectNoDataMethod()?;
            let _t6 = this.handles.get().lookupException(this.passHandle.get())?;
            slotDesc.invokeReadObjectNoData(obj)?;
            i = i.wrapping_add(1i32);
        }
        i = 0i32;
        loop {
            if i >= (slots.len() as i32) { break; }
            slotValues[i as usize].clone().defaultCheckFieldValues(obj)?;
            i = i.wrapping_add(1i32);
        }
        i = 0i32;
        loop {
            if i >= (slots.len() as i32) { break; }
            slotValues[i as usize].clone().defaultSetFieldValues(obj)?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "skipCustomData", descriptor = "()V", access = "private"))]
    pub fn skipCustomData(&self) -> Result<()> {
        let this = self;
        let mut oldHandle: i32 = this.passHandle.get();
        let _t0 = this.bin.get().getBlockDataMode()?;
        this.bin.get().skipBlockData()?;
        let _t1 = this.bin.get().setBlockDataMode(0i32)?;
        let _t2 = this.bin.get().peekByte()?;
        /* TODO: tableswitch default:94 low:119 high:122 */
        let _t3 = this.bin.get().setBlockDataMode(1i32)?;
        let _t4 = this.bin.get().readByte()?;
        this.passHandle.set(oldHandle);
        return Ok(());
        let _t5 = this.readObject0(96i32, 0i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readFatalException", descriptor = "()Ljava/io/IOException;", access = "private"))]
    pub fn readFatalException(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.bin.get().readByte()?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.clear()?;
        let _t1 = this.bin.get().peekByte()?;
        let mut tc: i32 = _t1;
        let mut _arr2: Vec<Object> = Vec::with_capacity(1i32 as usize);
        let _t3: Object = Byte::valueOf(tc)?;
        _arr2[0i32 as usize] = _t3;
        let _t4: String = String::format(String::from("invalid type code: %02X"), &_arr2)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t5 = this.readObject0(96i32, 0i32)?;
        Ok(_t5)
    }

    #[cfg_attr(any(), java_method(name = "handleReset", descriptor = "()V", access = "private"))]
    pub fn handleReset(&self) -> Result<()> {
        let this = self;
        /* TODO: lcmp  */
        String::new().append(&String::from("unexpected reset; recursion depth:"))?;
        String::new().append(&this.depth.get())?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.clear()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "latestUserDefinedLoader", descriptor = "()Ljava/lang/ClassLoader;", access = "private static"))]
    pub fn latestUserDefinedLoader() -> Result<Object> {
        let _t0: Object = VM::latestUserDefinedLoader()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "freeze", descriptor = "()V", access = "private"))]
    pub fn freeze(&self) -> Result<()> {
        let this = self;
        ObjectInputStream::UNSAFE().storeFence()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "cloneArray", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "private static"))]
    pub fn cloneArray(array: Object) -> Result<Object> {
        let _t0 = array.clone()?;
        return Ok(_t0);
        let _t1 = array.clone()?;
        return Ok(_t1);
        let _t2 = array.clone()?;
        return Ok(_t2);
        let _t3 = array.clone()?;
        return Ok(_t3);
        let _t4 = array.clone()?;
        return Ok(_t4);
        let _t5 = array.clone()?;
        return Ok(_t5);
        let _t6 = array.clone()?;
        return Ok(_t6);
        let _t7 = array.clone()?;
        return Ok(_t7);
        let _t8 = array.clone()?;
        return Ok(_t8);
        return Err(JvmError::Custom(String::from("athrow")));
    }
}
