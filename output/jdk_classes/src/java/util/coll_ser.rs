#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/CollSer",
    super_class = "java/lang/Object",
    interfaces  = "java/io/Serializable",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct CollSer {
    #[cfg_attr(any(), java_field(name = "tag", descriptor = "I", access = "private final"))]
    pub tag: Field<i32>,
    #[cfg_attr(any(), java_field(name = "array", descriptor = "[Ljava/lang/Object;", access = "private"))]
    pub array: Field<Vec<Object>>,
}

impl CollSer {
    // java: <init>(I[Ljava/lang/Object;)V
    pub fn new(t: i32, a: Vec<Object>) -> Result<Self> {
        let this = Self { tag: Field::new(0), array: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.tag.set(t);
        this.array.set(a);
        Ok(this)
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, ois: Object) -> Result<()> {
        let this = self;
        ois.defaultReadObject()?;
        let _t0 = ois.readInt()?;
        let mut len: i32 = _t0;
        String::new().append(&String::from("negative length"))?;
        String::new().append(&len)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1: Object = SharedSecrets::getJavaObjectInputStreamAccess()?;
        _t1.checkArray(ois, 53i32, len)?;
        let mut _arr2: Vec<Object> = Vec::with_capacity(len as usize);
        let mut a: Vec<Object> = _arr2;
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            let _t0 = ois.readObject()?;
            a[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        this.array.set(a);
        Ok(())
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, oos: Object) -> Result<()> {
        let this = self;
        oos.defaultWriteObject()?;
        oos.writeInt((this.array.get().len() as i32))?;
        let mut i: i32 = 0i32;
        loop {
            if i >= (this.array.get().len() as i32) { break; }
            oos.writeObject(this.array.get()[i as usize].clone())?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: readResolve()Ljava/lang/Object;
    pub fn readResolve(&self) -> Result<Object> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        /* TODO: tableswitch default:143 low:1 high:4 */
        let _t0: Object = List::of__arr_obj(&this.array.get())?;
        return Ok(_t0);
        let _t1: Vec<Object> = Arrays::copyOf__arr_obj_i_class(&this.array.get(), (this.array.get().len() as i32), 53i32)?;
        let _t2: Object = ImmutableCollections::listFromTrustedArrayNullsAllowed(&_t1)?;
        return Ok(_t2);
        let _t3: Object = Set::of__arr_obj(&this.array.get())?;
        return Ok(_t3);
        return Ok(ImmutableCollections::EMPTY_MAP());
        return Ok(ImmutableCollections_Map1::new(this.array.get()[0i32 as usize].clone(), this.array.get()[1i32 as usize].clone())?);
        return Ok(ImmutableCollections_MapN::new(this.array.get())?);
        let mut _arr4: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr4[0i32 as usize] = this.tag.get();
        let _t5: String = String::format(String::from("invalid flags 0x%x"), &_arr4)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut ex: i32 = 2i32;
        let mut ioe: InvalidObjectException = InvalidObjectException::new(String::from("invalid object"))?;
        let _t6 = ioe.initCause(ex)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }
}
