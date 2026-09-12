#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/ObjectOutputStream",
    super_class = "java/io/OutputStream",
    interfaces  = "java/io/ObjectOutput,java/io/ObjectStreamConstants",
    access      = "public",
    source      = "ObjectOutputStream.java",
))]
pub struct ObjectOutputStream {
    #[cfg_attr(any(), java_field(name = "bout", descriptor = "Ljava/io/ObjectOutputStream$BlockDataOutputStream;", access = "private final"))]
    pub bout: Field<Object>,
    #[cfg_attr(any(), java_field(name = "handles", descriptor = "Ljava/io/ObjectOutputStream$HandleTable;", access = "private final"))]
    pub handles: Field<Object>,
    #[cfg_attr(any(), java_field(name = "subs", descriptor = "Ljava/io/ObjectOutputStream$ReplaceTable;", access = "private final"))]
    pub subs: Field<Object>,
    #[cfg_attr(any(), java_field(name = "protocol", descriptor = "I", access = "private"))]
    pub protocol: Field<i32>,
    #[cfg_attr(any(), java_field(name = "depth", descriptor = "I", access = "private"))]
    pub depth: Field<i32>,
    #[cfg_attr(any(), java_field(name = "primVals", descriptor = "[B", access = "private"))]
    pub primVals: Field<Vec<i8>>,
    #[cfg_attr(any(), java_field(name = "enableOverride", descriptor = "Z", access = "private final"))]
    pub enableOverride: Field<bool>,
    #[cfg_attr(any(), java_field(name = "enableReplace", descriptor = "Z", access = "private"))]
    pub enableReplace: Field<bool>,
    #[cfg_attr(any(), java_field(name = "curContext", descriptor = "Ljava/io/SerialCallbackContext;", access = "private"))]
    pub curContext: Field<Object>,
    #[cfg_attr(any(), java_field(name = "curPut", descriptor = "Ljava/io/ObjectOutputStream$PutFieldImpl;", access = "private"))]
    pub curPut: Field<Object>,
    #[cfg_attr(any(), java_field(name = "debugInfoStack", descriptor = "Ljava/io/ObjectOutputStream$DebugTraceInfoStack;", access = "private final"))]
    pub debugInfoStack: Field<Object>,
}

impl ObjectOutputStream {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;)V", access = "public"))]
    // java: <init>(Ljava/io/OutputStream;)V
    pub fn new__output(out: Object) -> Result<Self> {
        let this = Self { bout: Field::new(Default::default()), handles: Field::new(Default::default()), subs: Field::new(Default::default()), protocol: Field::new(0), depth: Field::new(0), primVals: Field::new(Default::default()), enableOverride: Field::new(false), enableReplace: Field::new(false), curContext: Field::new(Default::default()), curPut: Field::new(Default::default()), debugInfoStack: Field::new(Default::default()) };
        /* invokespecial Method java/io/OutputStream.<init>:()V */
        this.protocol.set(2i32);
        this.verifySubclass()?;
        this.bout.set(ObjectOutputStream_BlockDataOutputStream::new(out)?);
        this.handles.set(ObjectOutputStream_HandleTable::new(10i32, 3.0f32)?);
        this.subs.set(ObjectOutputStream_ReplaceTable::new(10i32, 3.0f32)?);
        this.enableOverride.set(0i32);
        this.writeStreamHeader()?;
        let _t0 = this.bout.get().setBlockDataMode(1i32)?;
        this.debugInfoStack.set(ObjectOutputStream_DebugTraceInfoStack::new()?);
        /* TODO: aconst_null  */
        ObjectOutputStream::extendedDebugInfo().debugInfoStack.set(this);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "protected"))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { bout: Field::new(Default::default()), handles: Field::new(Default::default()), subs: Field::new(Default::default()), protocol: Field::new(0), depth: Field::new(0), primVals: Field::new(Default::default()), enableOverride: Field::new(false), enableReplace: Field::new(false), curContext: Field::new(Default::default()), curPut: Field::new(Default::default()), debugInfoStack: Field::new(Default::default()) };
        /* invokespecial Method java/io/OutputStream.<init>:()V */
        this.protocol.set(2i32);
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(ObjectOutputStream::SUBCLASS_IMPLEMENTATION_PERMISSION())?;
        /* TODO: aconst_null  */
        sm.bout.set(this);
        /* TODO: aconst_null  */
        todo!("stack underflow").handles.set(this);
        /* TODO: aconst_null  */
        todo!("stack underflow").subs.set(this);
        this.enableOverride.set(1i32);
        /* TODO: aconst_null  */
        todo!("stack underflow").debugInfoStack.set(this);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "useProtocolVersion", descriptor = "(I)V", access = "public"))]
    pub fn useProtocolVersion(&self, version: i32) -> Result<()> {
        let this = self;
        let _t0 = this.handles.get().size()?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lookupswitch default:56 1:48 2:48 */
        this.protocol.set(version);
        String::new().append(&String::from("unknown version:"))?;
        String::new().append(&version)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeObject", descriptor = "(Ljava/lang/Object;)V", access = "public final"))]
    pub fn writeObject(&self, obj: Object) -> Result<()> {
        let this = self;
        this.writeObjectOverride(obj)?;
        return Ok(());
        this.writeObject0(obj, 0i32)?;
        let mut ex: bool = this.enableOverride.get();
        this.writeFatalException(ex)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeObjectOverride", descriptor = "(Ljava/lang/Object;)V", access = "protected"))]
    pub fn writeObjectOverride(&self, obj: Object) -> Result<()> {
        let this = self;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeUnshared", descriptor = "(Ljava/lang/Object;)V", access = "public"))]
    pub fn writeUnshared(&self, obj: Object) -> Result<()> {
        let this = self;
        this.writeObject0(obj, 1i32)?;
        let mut ex: i32 = todo!("stack underflow");
        this.writeFatalException(ex)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "defaultWriteObject", descriptor = "()V", access = "public"))]
    pub fn defaultWriteObject(&self) -> Result<()> {
        let this = self;
        let mut ctx: Object = this.curContext.get();
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0 = ctx.getObj()?;
        let mut curObj: Object = _t0;
        let _t1 = ctx.getDesc()?;
        let mut curDesc: Object = _t1;
        let _t2 = this.bout.get().setBlockDataMode(0i32)?;
        this.defaultWriteFields(curObj, curDesc)?;
        let _t3 = this.bout.get().setBlockDataMode(1i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "putFields", descriptor = "()Ljava/io/ObjectOutputStream$PutField;", access = "public"))]
    pub fn putFields(&self) -> Result<Object> {
        let this = self;
        let mut ctx: Object = this.curContext.get();
        return Err(JvmError::Custom(String::from("athrow")));
        ctx.checkAndSetUsed()?;
        let _t0 = ctx.getDesc()?;
        let mut curDesc: Object = _t0;
        this.curPut.set(ObjectOutputStream_PutFieldImpl::new(this, curDesc)?);
        Ok(this.curPut.get())
    }

    #[cfg_attr(any(), java_method(name = "writeFields", descriptor = "()V", access = "public"))]
    pub fn writeFields(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0 = this.bout.get().setBlockDataMode(0i32)?;
        this.curPut.get().writeFields()?;
        let _t1 = this.bout.get().setBlockDataMode(1i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "reset", descriptor = "()V", access = "public"))]
    pub fn reset(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0 = this.bout.get().setBlockDataMode(0i32)?;
        this.bout.get().writeByte(121i32)?;
        this.clear()?;
        let _t1 = this.bout.get().setBlockDataMode(1i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "annotateClass", descriptor = "(Ljava/lang/Class;)V", access = "protected"))]
    pub fn annotateClass(&self, cl: Object) -> Result<()> {
        let this = self;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "annotateProxyClass", descriptor = "(Ljava/lang/Class;)V", access = "protected"))]
    pub fn annotateProxyClass(&self, cl: Object) -> Result<()> {
        let this = self;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "replaceObject", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "protected"))]
    pub fn replaceObject(&self, obj: Object) -> Result<Object> {
        let this = self;
        Ok(obj)
    }

    #[cfg_attr(any(), java_method(name = "enableReplaceObject", descriptor = "(Z)Z", access = "protected"))]
    pub fn enableReplaceObject(&self, enable: bool) -> Result<bool> {
        let this = self;
        return Ok(enable);
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(ObjectOutputStream::SUBSTITUTION_PERMISSION())?;
        this.enableReplace.set(enable);
        Ok(this.enableReplace.get()==0i32)
    }

    #[cfg_attr(any(), java_method(name = "writeStreamHeader", descriptor = "()V", access = "protected"))]
    pub fn writeStreamHeader(&self) -> Result<()> {
        let this = self;
        this.bout.get().writeShort(-21267i32)?;
        this.bout.get().writeShort(5i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeClassDescriptor", descriptor = "(Ljava/io/ObjectStreamClass;)V", access = "protected"))]
    pub fn writeClassDescriptor(&self, desc: Object) -> Result<()> {
        let this = self;
        desc.writeNonProxy(this)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(I)V", access = "public"))]
    // java: write(I)V
    pub fn write__i(&self, val: i32) -> Result<()> {
        let this = self;
        this.bout.get().write(val)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([B)V", access = "public"))]
    // java: write([B)V
    pub fn write__arr_b(&self, buf: Vec<i8>) -> Result<()> {
        let this = self;
        this.bout.get().write(buf, 0i32, (buf.len() as i32), 0i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([BII)V", access = "public"))]
    // java: write([BII)V
    pub fn write__arr_b_i_i(&self, buf: Vec<i8>, off: i32, len: i32) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: i32 = Objects::checkFromIndexSize(off, len, (buf.len() as i32))?;
        this.bout.get().write(buf, off, len, 0i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "flush", descriptor = "()V", access = "public"))]
    pub fn flush(&self) -> Result<()> {
        let this = self;
        this.bout.get().flush()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "drain", descriptor = "()V", access = "protected"))]
    pub fn drain(&self) -> Result<()> {
        let this = self;
        this.bout.get().drain()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "close", descriptor = "()V", access = "public"))]
    pub fn close(&self) -> Result<()> {
        let this = self;
        this.flush()?;
        this.clear()?;
        this.bout.get().close()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeBoolean", descriptor = "(Z)V", access = "public"))]
    pub fn writeBoolean(&self, val: bool) -> Result<()> {
        let this = self;
        this.bout.get().writeBoolean(val)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeByte", descriptor = "(I)V", access = "public"))]
    pub fn writeByte(&self, val: i32) -> Result<()> {
        let this = self;
        this.bout.get().writeByte(val)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeShort", descriptor = "(I)V", access = "public"))]
    pub fn writeShort(&self, val: i32) -> Result<()> {
        let this = self;
        this.bout.get().writeShort(val)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeChar", descriptor = "(I)V", access = "public"))]
    pub fn writeChar(&self, val: i32) -> Result<()> {
        let this = self;
        this.bout.get().writeChar(val)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeInt", descriptor = "(I)V", access = "public"))]
    pub fn writeInt(&self, val: i32) -> Result<()> {
        let this = self;
        this.bout.get().writeInt(val)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeLong", descriptor = "(J)V", access = "public"))]
    pub fn writeLong(&self, val: i64) -> Result<()> {
        let this = self;
        this.bout.get().writeLong(val)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeFloat", descriptor = "(F)V", access = "public"))]
    pub fn writeFloat(&self, val: f32) -> Result<()> {
        let this = self;
        this.bout.get().writeFloat(val)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeDouble", descriptor = "(D)V", access = "public"))]
    pub fn writeDouble(&self, val: f64) -> Result<()> {
        let this = self;
        this.bout.get().writeDouble(val)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeBytes", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn writeBytes(&self, str: String) -> Result<()> {
        let this = self;
        this.bout.get().writeBytes(str)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeChars", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn writeChars(&self, str: String) -> Result<()> {
        let this = self;
        this.bout.get().writeChars(str)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeUTF", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn writeUTF(&self, str: String) -> Result<()> {
        let this = self;
        this.bout.get().writeUTF(str)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getProtocolVersion", descriptor = "()I"))]
    pub fn getProtocolVersion(&self) -> Result<i32> {
        let this = self;
        Ok(this.protocol.get())
    }

    #[cfg_attr(any(), java_method(name = "writeTypeString", descriptor = "(Ljava/lang/String;)V"))]
    pub fn writeTypeString(&self, str: String) -> Result<()> {
        let this = self;
        this.writeNull()?;
        let _t0 = this.handles.get().lookup(str)?;
        let mut handle: i32 = _t0;
        this.writeHandle(handle)?;
        this.writeString(str, 0i32)?;
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
        let _t2 = ObjectOutputStream$Caches::subclassAudits().get(cl)?;
        let mut result: i32 = _t2;
        sm.checkPermission(ObjectOutputStream::SUBCLASS_IMPLEMENTATION_PERMISSION())?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "auditSubclass", descriptor = "(Ljava/lang/Class;)Ljava/lang/Boolean;", access = "private static"))]
    pub fn auditSubclass(subcl: Object) -> Result<bool> {
        let _t0: Object = AccessController::doPrivileged(ObjectOutputStream_1::new(subcl)?)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()V", access = "private"))]
    pub fn clear(&self) -> Result<()> {
        let this = self;
        this.subs.get().clear()?;
        this.handles.get().clear()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeObject0", descriptor = "(Ljava/lang/Object;Z)V", access = "private"))]
    pub fn writeObject0(&self, obj: Object, unshared: bool) -> Result<()> {
        let this = self;
        let _t0 = this.bout.get().setBlockDataMode(0i32)?;
        let mut oldMode: i32 = _t0;
        this.depth.set((this.depth.get()).wrapping_add(1i32));
        let _t1 = this.subs.get().lookup(obj)?;
        obj = _t1;
        this.writeNull()?;
        this.depth.set((this.depth.get()).wrapping_sub(1i32));
        let _t2 = this.bout.get().setBlockDataMode(oldMode)?;
        return Ok(());
        let _t3 = this.handles.get().lookup(obj)?;
        let mut h: i32 = _t3;
        this.writeHandle(h)?;
        this.depth.set((this.depth.get()).wrapping_sub(1i32));
        let _t4 = this.bout.get().setBlockDataMode(oldMode)?;
        return Ok(());
        this.writeClass(obj, unshared)?;
        this.depth.set((this.depth.get()).wrapping_sub(1i32));
        let _t5 = this.bout.get().setBlockDataMode(oldMode)?;
        return Ok(());
        this.writeClassDesc(obj, unshared)?;
        this.depth.set((this.depth.get()).wrapping_sub(1i32));
        let _t6 = this.bout.get().setBlockDataMode(oldMode)?;
        return Ok(());
        let mut orig: Object = obj;
        let _t7 = obj.getClass()?;
        let mut cl: Object = _t7;
        loop {
            let _t0: Object = ObjectStreamClass::lookup(cl, 1i32)?;
            let mut desc: Object = _t0;
            let _t1 = desc.hasWriteReplaceMethod()?;
            if _t1==0i32 { break; }
            let _t0 = desc.invokeWriteReplace(obj)?;
            obj = _t0;
            let _t1 = obj.getClass()?;
            let mut repCl: Object = _t1;
            cl = repCl;
        }
        let _t8 = this.replaceObject(obj)?;
        repCl = _t8;
        let _t9 = repCl.getClass()?;
        cl = _t9;
        let _t10: Object = ObjectStreamClass::lookup(cl, 1i32)?;
        desc = _t10;
        obj = repCl;
        this.subs.get().assign(orig, obj)?;
        this.writeNull()?;
        this.depth.set((this.depth.get()).wrapping_sub(1i32));
        let _t11 = this.bout.get().setBlockDataMode(oldMode)?;
        return Ok(());
        let _t12 = this.handles.get().lookup(obj)?;
        h = _t12;
        this.writeHandle(h)?;
        this.depth.set((this.depth.get()).wrapping_sub(1i32));
        let _t13 = this.bout.get().setBlockDataMode(oldMode)?;
        return Ok(());
        this.writeClass(obj, unshared)?;
        this.depth.set((this.depth.get()).wrapping_sub(1i32));
        let _t14 = this.bout.get().setBlockDataMode(oldMode)?;
        return Ok(());
        this.writeClassDesc(obj, unshared)?;
        this.depth.set((this.depth.get()).wrapping_sub(1i32));
        let _t15 = this.bout.get().setBlockDataMode(oldMode)?;
        return Ok(());
        this.writeString(obj, unshared)?;
        let _t16 = cl.isArray()?;
        this.writeArray(obj, desc, unshared)?;
        this.writeEnum(obj, desc, unshared)?;
        this.writeOrdinaryObject(obj, desc, unshared)?;
        let _t17 = cl.getName()?;
        String::new().append(&_t17)?;
        String::new().append(&String::from(""))?;
        let _t18 = this.debugInfoStack.get().toString()?;
        String::new().append(&_t18)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t19 = cl.getName()?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.depth.set((this.depth.get()).wrapping_sub(1i32));
        let _t20 = this.bout.get().setBlockDataMode(oldMode)?;
        let mut local_9: bool = ObjectOutputStream::extendedDebugInfo();
        this.depth.set((this.depth.get()).wrapping_sub(1i32));
        let _t21 = this.bout.get().setBlockDataMode(oldMode)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeNull", descriptor = "()V", access = "private"))]
    pub fn writeNull(&self) -> Result<()> {
        let this = self;
        this.bout.get().writeByte(112i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeHandle", descriptor = "(I)V", access = "private"))]
    pub fn writeHandle(&self, handle: i32) -> Result<()> {
        let this = self;
        this.bout.get().writeByte(113i32)?;
        this.bout.get().writeInt((358i32).wrapping_add(handle))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeClass", descriptor = "(Ljava/lang/Class;Z)V", access = "private"))]
    pub fn writeClass(&self, cl: Object, unshared: bool) -> Result<()> {
        let this = self;
        this.bout.get().writeByte(118i32)?;
        let _t0: Object = ObjectStreamClass::lookup(cl, 1i32)?;
        this.writeClassDesc(_t0, 0i32)?;
        /* TODO: aconst_null  */
        let _t1 = unshared.assign(cl)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeClassDesc", descriptor = "(Ljava/io/ObjectStreamClass;Z)V", access = "private"))]
    pub fn writeClassDesc(&self, desc: Object, unshared: bool) -> Result<()> {
        let this = self;
        this.writeNull()?;
        let _t0 = this.handles.get().lookup(desc)?;
        let mut handle: i32 = _t0;
        this.writeHandle(handle)?;
        let _t1 = desc.isProxy()?;
        this.writeProxyDesc(desc, unshared)?;
        this.writeNonProxyDesc(desc, unshared)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "isCustomSubclass", descriptor = "()Z", access = "private"))]
    pub fn isCustomSubclass(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.getClass()?;
        let _t1 = _t0.getClassLoader()?;
        let _t2 = 10i32.getClassLoader()?;
        Ok(/* if_acmpeq */ true)
    }

    #[cfg_attr(any(), java_method(name = "writeProxyDesc", descriptor = "(Ljava/io/ObjectStreamClass;Z)V", access = "private"))]
    pub fn writeProxyDesc(&self, desc: Object, unshared: bool) -> Result<()> {
        let this = self;
        this.bout.get().writeByte(125i32)?;
        /* TODO: aconst_null  */
        let _t0 = unshared.assign(desc)?;
        let _t1 = desc.forClass()?;
        let mut cl: Object = _t1;
        let _t2 = cl.getInterfaces()?;
        let mut ifaces: Vec<Object> = _t2;
        this.bout.get().writeInt((ifaces.len() as i32))?;
        let mut i: i32 = 0i32;
        loop {
            if i >= (ifaces.len() as i32) { break; }
            let _t0 = ifaces[i as usize].clone().getName()?;
            this.bout.get().writeUTF(_t0)?;
            i = i.wrapping_add(1i32);
        }
        let _t3 = this.bout.get().setBlockDataMode(1i32)?;
        let _t4 = this.isCustomSubclass()?;
        ReflectUtil::checkPackageAccess(cl)?;
        this.annotateProxyClass(cl)?;
        let _t5 = this.bout.get().setBlockDataMode(0i32)?;
        this.bout.get().writeByte(120i32)?;
        let _t6 = desc.getSuperDesc()?;
        this.writeClassDesc(_t6, 0i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeNonProxyDesc", descriptor = "(Ljava/io/ObjectStreamClass;Z)V", access = "private"))]
    pub fn writeNonProxyDesc(&self, desc: Object, unshared: bool) -> Result<()> {
        let this = self;
        this.bout.get().writeByte(114i32)?;
        /* TODO: aconst_null  */
        let _t0 = unshared.assign(desc)?;
        desc.writeNonProxy(this)?;
        this.writeClassDescriptor(desc)?;
        let _t1 = desc.forClass()?;
        let mut cl: Object = _t1;
        let _t2 = this.bout.get().setBlockDataMode(1i32)?;
        let _t3 = this.isCustomSubclass()?;
        ReflectUtil::checkPackageAccess(cl)?;
        this.annotateClass(cl)?;
        let _t4 = this.bout.get().setBlockDataMode(0i32)?;
        this.bout.get().writeByte(120i32)?;
        let _t5 = desc.getSuperDesc()?;
        this.writeClassDesc(_t5, 0i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeString", descriptor = "(Ljava/lang/String;Z)V", access = "private"))]
    pub fn writeString(&self, str: String, unshared: bool) -> Result<()> {
        let this = self;
        /* TODO: aconst_null  */
        let _t0 = unshared.assign(str)?;
        let _t1 = this.bout.get().getUTFLength(str)?;
        let mut utflen: i64 = _t1;
        /* TODO: lcmp  */
        this.bout.get().writeByte(116i32)?;
        this.bout.get().writeUTF(str, utflen)?;
        this.bout.get().writeByte(124i32)?;
        this.bout.get().writeLongUTF(str, utflen)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeArray", descriptor = "(Ljava/lang/Object;Ljava/io/ObjectStreamClass;Z)V", access = "private"))]
    pub fn writeArray(&self, array: Object, desc: Object, unshared: bool) -> Result<()> {
        let this = self;
        this.bout.get().writeByte(117i32)?;
        this.writeClassDesc(desc, 0i32)?;
        /* TODO: aconst_null  */
        let _t0 = unshared.assign(array)?;
        let _t1 = desc.forClass()?;
        let _t2 = _t1.getComponentType()?;
        let mut ccl: Object = _t2;
        let _t3 = ccl.isPrimitive()?;
        let mut ia: Object = array;
        this.bout.get().writeInt((ia.len() as i32))?;
        this.bout.get().writeInts(ia, 0i32, (ia.len() as i32))?;
        ia = array;
        this.bout.get().writeInt((ia.len() as i32))?;
        this.bout.get().write(ia, 0i32, (ia.len() as i32), 1i32)?;
        ia = array;
        this.bout.get().writeInt((ia.len() as i32))?;
        this.bout.get().writeLongs(ia, 0i32, (ia.len() as i32))?;
        ia = array;
        this.bout.get().writeInt((ia.len() as i32))?;
        this.bout.get().writeFloats(ia, 0i32, (ia.len() as i32))?;
        ia = array;
        this.bout.get().writeInt((ia.len() as i32))?;
        this.bout.get().writeDoubles(ia, 0i32, (ia.len() as i32))?;
        ia = array;
        this.bout.get().writeInt((ia.len() as i32))?;
        this.bout.get().writeShorts(ia, 0i32, (ia.len() as i32))?;
        ia = array;
        this.bout.get().writeInt((ia.len() as i32))?;
        this.bout.get().writeChars(ia, 0i32, (ia.len() as i32))?;
        ia = array;
        this.bout.get().writeInt((ia.len() as i32))?;
        this.bout.get().writeBooleans(ia, 0i32, (ia.len() as i32))?;
        return Err(JvmError::Custom(String::from("athrow")));
        ia = array;
        let mut len: i32 = (ia.len() as i32);
        this.bout.get().writeInt(len)?;
        String::new().append(&String::from("array (class ""))?;
        let _t4 = array.getClass()?;
        let _t5 = _t4.getName()?;
        String::new().append(&_t5)?;
        String::new().append(&String::from("", size:"))?;
        String::new().append(&len)?;
        String::new().append(&String::from(")"))?;
        this.debugInfoStack.get().push(String::new())?;
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            String::new().append(&String::from("element of array (index:"))?;
            String::new().append(&i)?;
            String::new().append(&String::from(")"))?;
            this.debugInfoStack.get().push(String::new())?;
            this.writeObject0(ia[i as usize].clone(), 0i32)?;
            this.debugInfoStack.get().pop()?;
            let mut local_8: bool = ObjectOutputStream::extendedDebugInfo();
            this.debugInfoStack.get().pop()?;
            return Err(JvmError::Custom(String::from("athrow")));
            i = i.wrapping_add(1i32);
        }
        this.debugInfoStack.get().pop()?;
        let mut local_9: bool = ObjectOutputStream::extendedDebugInfo();
        this.debugInfoStack.get().pop()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeEnum", descriptor = "(Ljava/lang/Enum;Ljava/io/ObjectStreamClass;Z)V", access = "private"))]
    pub fn writeEnum(&self, en: Object, desc: Object, unshared: bool) -> Result<()> {
        let this = self;
        this.bout.get().writeByte(126i32)?;
        let _t0 = desc.getSuperDesc()?;
        let mut sdesc: Object = _t0;
        let _t1 = sdesc.forClass()?;
        desc.writeClassDesc(sdesc, 0i32)?;
        /* TODO: aconst_null  */
        let _t2 = unshared.assign(en)?;
        let _t3 = en.name()?;
        this.writeString(_t3, 0i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeOrdinaryObject", descriptor = "(Ljava/lang/Object;Ljava/io/ObjectStreamClass;Z)V", access = "private"))]
    pub fn writeOrdinaryObject(&self, obj: Object, desc: Object, unshared: bool) -> Result<()> {
        let this = self;
        String::from("root").append(&String::from(""))?;
        String::from("root").append(&String::from("object (class ""))?;
        let _t0 = obj.getClass()?;
        let _t1 = _t0.getName()?;
        String::from("root").append(&_t1)?;
        String::from("root").append(&String::from("","))?;
        let _t2 = obj.toString()?;
        String::from("root").append(&_t2)?;
        String::from("root").append(&String::from(")"))?;
        1i32.push(String::from("root"))?;
        desc.checkSerialize()?;
        this.bout.get().writeByte(115i32)?;
        this.writeClassDesc(desc, 0i32)?;
        /* TODO: aconst_null  */
        let _t3 = unshared.assign(obj)?;
        let _t4 = desc.isRecord()?;
        this.writeRecordData(obj, desc)?;
        let _t5 = desc.isExternalizable()?;
        let _t6 = desc.isProxy()?;
        this.writeExternalData(obj)?;
        this.writeSerialData(obj, desc)?;
        this.debugInfoStack.get().pop()?;
        let mut local_4: bool = ObjectOutputStream::extendedDebugInfo();
        this.debugInfoStack.get().pop()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeExternalData", descriptor = "(Ljava/io/Externalizable;)V", access = "private"))]
    pub fn writeExternalData(&self, obj: Object) -> Result<()> {
        let this = self;
        let mut oldPut: Object = this.curPut.get();
        /* TODO: aconst_null  */
        todo!("stack underflow").curPut.set(this);
        this.debugInfoStack.get().push(String::from("writeExternal data"))?;
        let mut oldContext: Object = this.curContext.get();
        /* TODO: aconst_null  */
        ObjectOutputStream::extendedDebugInfo().curContext.set(this);
        obj.writeExternal(this)?;
        let _t0 = this.bout.get().setBlockDataMode(1i32)?;
        obj.writeExternal(this)?;
        let _t1 = this.bout.get().setBlockDataMode(0i32)?;
        this.bout.get().writeByte(120i32)?;
        this.curContext.set(oldContext);
        this.debugInfoStack.get().pop()?;
        let mut local_4: bool = ObjectOutputStream::extendedDebugInfo();
        this.curContext.set(oldContext);
        this.debugInfoStack.get().pop()?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.curPut.set(oldPut);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeRecordData", descriptor = "(Ljava/lang/Object;Ljava/io/ObjectStreamClass;)V", access = "private"))]
    pub fn writeRecordData(&self, obj: Object, desc: Object) -> Result<()> {
        let this = self;
        let _t0 = obj.getClass()?;
        let _t1 = _t0.isRecord()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t2 = desc.getClassDataLayout()?;
        let mut slots: Vec<Object> = _t2;
        String::new().append(&String::from("expected a single record slot length, but found:"))?;
        String::new().append(&(slots.len() as i32))?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.defaultWriteFields(obj, desc)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeSerialData", descriptor = "(Ljava/lang/Object;Ljava/io/ObjectStreamClass;)V", access = "private"))]
    pub fn writeSerialData(&self, obj: Object, desc: Object) -> Result<()> {
        let this = self;
        let _t0 = desc.getClassDataLayout()?;
        let mut slots: Vec<Object> = _t0;
        let mut i: i32 = 0i32;
        loop {
            if i >= (slots.len() as i32) { break; }
            let mut slotDesc: Object = slots[i as usize].clone().desc.get();
            let _t0 = slotDesc.hasWriteObjectMethod()?;
            let mut oldPut: Object = this.curPut.get();
            /* TODO: aconst_null  */
            _t0.curPut.set(this);
            let mut oldContext: Object = this.curContext.get();
            String::new().append(&String::from("custom writeObject data (class ""))?;
            let _t1 = slotDesc.getName()?;
            String::new().append(&_t1)?;
            String::new().append(&String::from("")"))?;
            this.debugInfoStack.get().push(String::new())?;
            this.curContext.set(SerialCallbackContext::new(obj, slotDesc)?);
            let _t2 = this.bout.get().setBlockDataMode(1i32)?;
            slotDesc.invokeWriteObject(obj, this)?;
            let _t3 = this.bout.get().setBlockDataMode(0i32)?;
            this.bout.get().writeByte(120i32)?;
            this.curContext.get().setUsed()?;
            this.curContext.set(oldContext);
            this.debugInfoStack.get().pop()?;
            let mut local_8: bool = ObjectOutputStream::extendedDebugInfo();
            this.curContext.get().setUsed()?;
            this.curContext.set(oldContext);
            this.debugInfoStack.get().pop()?;
            return Err(JvmError::Custom(String::from("athrow")));
            this.curPut.set(oldPut);
            this.defaultWriteFields(obj, slotDesc)?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "defaultWriteFields", descriptor = "(Ljava/lang/Object;Ljava/io/ObjectStreamClass;)V", access = "private"))]
    pub fn defaultWriteFields(&self, obj: Object, desc: Object) -> Result<()> {
        let this = self;
        let _t0 = desc.forClass()?;
        let mut cl: Object = _t0;
        let _t1 = cl.isInstance(obj)?;
        return Err(JvmError::Custom(String::from("athrow")));
        desc.checkDefaultSerialize()?;
        let _t2 = desc.getPrimDataSize()?;
        let mut primDataSize: i32 = _t2;
        let mut _arr3: Vec<i8> = vec![0i8; primDataSize as usize];
        this.primVals.set(_arr3);
        desc.getPrimFieldValues(obj, this.primVals.get())?;
        this.bout.get().write(this.primVals.get(), 0i32, primDataSize, 0i32)?;
        let _t4 = desc.getNumObjFields()?;
        let mut numObjFields: i32 = _t4;
        let _t5 = desc.getFields(0i32)?;
        let mut fields: Vec<Object> = _t5;
        let mut _arr6: Vec<Object> = Vec::with_capacity(numObjFields as usize);
        let mut objVals: Vec<Object> = _arr6;
        let mut numPrimFields: i32 = ((fields.len() as i32)).wrapping_sub((objVals.len() as i32));
        desc.getObjFieldValues(obj, objVals)?;
        let mut i: i32 = 0i32;
        loop {
            if i >= (objVals.len() as i32) { break; }
            String::new().append(&String::from("field (class ""))?;
            let _t0 = desc.getName()?;
            String::new().append(&_t0)?;
            String::new().append(&String::from("", name: ""))?;
            let _t1 = fields[(numPrimFields).wrapping_add(i) as usize].clone().getName()?;
            String::new().append(&_t1)?;
            String::new().append(&String::from("", type: ""))?;
            let _t2 = fields[(numPrimFields).wrapping_add(i) as usize].clone().getType()?;
            String::new().append(&_t2)?;
            String::new().append(&String::from("")"))?;
            this.debugInfoStack.get().push(String::new())?;
            let _t3 = fields[(numPrimFields).wrapping_add(i) as usize].clone().isUnshared()?;
            this.writeObject0(objVals[i as usize].clone(), _t3)?;
            this.debugInfoStack.get().pop()?;
            let mut local_10: bool = ObjectOutputStream::extendedDebugInfo();
            this.debugInfoStack.get().pop()?;
            return Err(JvmError::Custom(String::from("athrow")));
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeFatalException", descriptor = "(Ljava/io/IOException;)V", access = "private"))]
    pub fn writeFatalException(&self, ex: Object) -> Result<()> {
        let this = self;
        this.clear()?;
        let _t0 = this.bout.get().setBlockDataMode(0i32)?;
        let mut oldMode: i32 = _t0;
        this.bout.get().writeByte(123i32)?;
        this.writeObject0(ex, 0i32)?;
        this.clear()?;
        let _t1 = this.bout.get().setBlockDataMode(oldMode)?;
        let mut local_3: i32 = todo!("stack underflow");
        let _t2 = this.bout.get().setBlockDataMode(oldMode)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }
}
