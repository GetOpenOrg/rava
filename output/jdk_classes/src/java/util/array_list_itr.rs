#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ArrayList$Itr",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Iterator",
    access      = "",
    source      = "ArrayList.java",
))]
pub struct ArrayList_Itr {
    #[cfg_attr(any(), java_field(name = "cursor", descriptor = "I"))]
    pub cursor: Field<i32>,
    #[cfg_attr(any(), java_field(name = "lastRet", descriptor = "I"))]
    pub lastRet: Field<i32>,
    #[cfg_attr(any(), java_field(name = "expectedModCount", descriptor = "I"))]
    pub expectedModCount: Field<i32>,
    #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/util/ArrayList;", access = "final"))]
    pub this_0: Field<Object>,
}

impl ArrayList_Itr {
    // java: <init>(Ljava/util/ArrayList;)V
    pub fn new(arg_0: Object) -> Result<Self> {
        let this = Self { cursor: Field::new(0), lastRet: Field::new(0), expectedModCount: Field::new(0), this_0: Field::new(Default::default()) };
        this.this_0.set(arg_0);
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.lastRet.set(-1i32);
        this.expectedModCount.set(this.this_0.get().modCount.get());
        Ok(this)
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        let this = self;
        Ok(this.cursor.get() != this.this_0.get().size.get())
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<Object> {
        let this = self;
        this.checkForComodification()?;
        let mut i: i32 = this.cursor.get();
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut elementData: Vec<Object> = this.this_0.get().elementData.get();
        return Err(JvmError::Custom("athrow".to_owned()));
        this.cursor.set((i).wrapping_add(1i32));
        this.lastRet.set(i);
        Ok(elementData[i as usize].clone())
    }

    // java: remove()V
    pub fn remove(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.checkForComodification()?;
        let _t0 = this.this_0.get().remove(this.lastRet.get())?;
        this.cursor.set(this.lastRet.get());
        this.lastRet.set(-1i32);
        this.expectedModCount.set(this.this_0.get().modCount.get());
        let mut ex: i32 = this.lastRet.get();
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(action)?;
        let mut size: i32 = this.this_0.get().size.get();
        let mut i: i32 = this.cursor.get();
        let mut es: Vec<Object> = this.this_0.get().elementData.get();
        return Err(JvmError::Custom("athrow".to_owned()));
        loop {
            if i >= size { break; }
            let _t0: Object = ArrayList::elementAt(&es, i)?;
            action.accept(_t0)?;
            i = i.wrapping_add(1i32);
        }
        this.cursor.set(i);
        this.lastRet.set((i).wrapping_sub(1i32));
        this.checkForComodification()?;
        Ok(())
    }

    // java: checkForComodification()V
    pub fn checkForComodification(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }
}
