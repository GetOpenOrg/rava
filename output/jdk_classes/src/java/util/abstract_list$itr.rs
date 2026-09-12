#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/AbstractList$Itr",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Iterator",
    access      = "",
    source      = "AbstractList.java",
))]
pub struct AbstractList_Itr {
    #[cfg_attr(any(), java_field(name = "cursor", descriptor = "I"))]
    pub cursor: Field<i32>,
    #[cfg_attr(any(), java_field(name = "lastRet", descriptor = "I"))]
    pub lastRet: Field<i32>,
    #[cfg_attr(any(), java_field(name = "expectedModCount", descriptor = "I"))]
    pub expectedModCount: Field<i32>,
    #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/util/AbstractList;", access = "final"))]
    pub this$0: Field<Object>,
}

impl AbstractList_Itr {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/AbstractList;)V", access = "private"))]
    pub fn new(arg_0: Object) -> Result<Self> {
        let this = Self { cursor: Field::new(0), lastRet: Field::new(0), expectedModCount: Field::new(0), this$0: Field::new(Default::default()) };
        this.this$0.set(arg_0);
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.cursor.set(0i32);
        this.lastRet.set(-1i32);
        this.expectedModCount.set(this.this$0.get().modCount.get());
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "hasNext", descriptor = "()Z", access = "public"))]
    pub fn hasNext(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.this$0.get().size()?;
        Ok(this.cursor.get() != _t0)
    }

    #[cfg_attr(any(), java_method(name = "next", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn next(&self) -> Result<Object> {
        let this = self;
        this.checkForComodification()?;
        let mut i: i32 = this.cursor.get();
        let _t0 = this.this$0.get().get(i)?;
        let mut next: Object = _t0;
        this.lastRet.set(i);
        this.cursor.set((i).wrapping_add(1i32));
        return Ok(next);
        i = todo!("stack underflow");
        this.checkForComodification()?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "()V", access = "public"))]
    pub fn remove(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        this.checkForComodification()?;
        let _t0 = this.this$0.get().remove(this.lastRet.get())?;
        this.cursor.set((this.cursor.get()).wrapping_sub(1i32));
        this.lastRet.set(-1i32);
        this.expectedModCount.set(this.this$0.get().modCount.get());
        let mut e: i32 = this.cursor.get();
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkForComodification", descriptor = "()V", access = "final"))]
    pub fn checkForComodification(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }
}
