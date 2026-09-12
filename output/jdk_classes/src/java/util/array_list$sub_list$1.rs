#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ArrayList$SubList$1",
    super_class = "java/lang/Object",
    interfaces  = "java/util/ListIterator",
    access      = "",
    source      = "ArrayList.java",
))]
pub struct ArrayList_SubList_1 {
    #[cfg_attr(any(), java_field(name = "cursor", descriptor = "I"))]
    pub cursor: Field<i32>,
    #[cfg_attr(any(), java_field(name = "lastRet", descriptor = "I"))]
    pub lastRet: Field<i32>,
    #[cfg_attr(any(), java_field(name = "expectedModCount", descriptor = "I"))]
    pub expectedModCount: Field<i32>,
    #[cfg_attr(any(), java_field(name = "val$index", descriptor = "I", access = "final"))]
    pub val$index: Field<i32>,
    #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/util/ArrayList$SubList;", access = "final"))]
    pub this$0: Field<Object>,
}

impl ArrayList_SubList_1 {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/ArrayList$SubList;I)V"))]
    pub fn new(this_0: Object, arg_1: i32) -> Result<Self> {
        let this = Self { cursor: Field::new(0), lastRet: Field::new(0), expectedModCount: Field::new(0), val$index: Field::new(0), this$0: Field::new(Default::default()) };
        this.this$0.set(this_0);
        this.val$index.set(arg_1);
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.cursor.set(this.val$index.get());
        this.lastRet.set(-1i32);
        this.expectedModCount.set(this.this$0.get().modCount.get());
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "hasNext", descriptor = "()Z", access = "public"))]
    pub fn hasNext(&self) -> Result<bool> {
        let this = self;
        Ok(this.cursor.get() != this.this$0.get().size.get())
    }

    #[cfg_attr(any(), java_method(name = "next", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn next(&self) -> Result<Object> {
        let this = self;
        this.checkForComodification()?;
        let mut i: i32 = this.cursor.get();
        return Err(JvmError::Custom(String::from("athrow")));
        let mut elementData: Vec<Object> = this.this$0.get().root.get().elementData.get();
        return Err(JvmError::Custom(String::from("athrow")));
        this.cursor.set((i).wrapping_add(1i32));
        this.lastRet.set(i);
        Ok(elementData[(this.this$0.get().offset.get()).wrapping_add(i) as usize].clone())
    }

    #[cfg_attr(any(), java_method(name = "hasPrevious", descriptor = "()Z", access = "public"))]
    pub fn hasPrevious(&self) -> Result<bool> {
        let this = self;
        Ok(this.cursor.get()!=0i32)
    }

    #[cfg_attr(any(), java_method(name = "previous", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn previous(&self) -> Result<Object> {
        let this = self;
        this.checkForComodification()?;
        let mut i: i32 = (this.cursor.get()).wrapping_sub(1i32);
        return Err(JvmError::Custom(String::from("athrow")));
        let mut elementData: Vec<Object> = this.this$0.get().root.get().elementData.get();
        return Err(JvmError::Custom(String::from("athrow")));
        this.cursor.set(i);
        this.lastRet.set(i);
        Ok(elementData[(this.this$0.get().offset.get()).wrapping_add(i) as usize].clone())
    }

    #[cfg_attr(any(), java_method(name = "forEachRemaining", descriptor = "(Ljava/util/function/Consumer;)V", access = "public"))]
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(action)?;
        let mut size: i32 = this.this$0.get().size.get();
        let mut i: i32 = this.cursor.get();
        let mut es: Vec<Object> = this.this$0.get().root.get().elementData.get();
        return Err(JvmError::Custom(String::from("athrow")));
        loop {
            if i >= size { break; }
            let _t0: Object = ArrayList::elementAt(&es, (this.this$0.get().offset.get()).wrapping_add(i))?;
            action.accept(_t0)?;
            i = i.wrapping_add(1i32);
        }
        this.cursor.set(i);
        this.lastRet.set((i).wrapping_sub(1i32));
        this.checkForComodification()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "nextIndex", descriptor = "()I", access = "public"))]
    pub fn nextIndex(&self) -> Result<i32> {
        let this = self;
        Ok(this.cursor.get())
    }

    #[cfg_attr(any(), java_method(name = "previousIndex", descriptor = "()I", access = "public"))]
    pub fn previousIndex(&self) -> Result<i32> {
        let this = self;
        Ok((this.cursor.get()).wrapping_sub(1i32))
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "()V", access = "public"))]
    pub fn remove(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        this.checkForComodification()?;
        let _t0 = this.this$0.get().remove(this.lastRet.get())?;
        this.cursor.set(this.lastRet.get());
        this.lastRet.set(-1i32);
        this.expectedModCount.set(this.this$0.get().modCount.get());
        let mut ex: i32 = this.lastRet.get();
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "set", descriptor = "(Ljava/lang/Object;)V", access = "public"))]
    pub fn set(&self, e: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        this.checkForComodification()?;
        let _t0 = this.this$0.get().root.get().set((this.this$0.get().offset.get()).wrapping_add(this.lastRet.get()), e)?;
        let mut ex: i32 = this.lastRet.get();
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(Ljava/lang/Object;)V", access = "public"))]
    pub fn add(&self, e: Object) -> Result<()> {
        let this = self;
        this.checkForComodification()?;
        let mut i: i32 = this.cursor.get();
        this.this$0.get().add(i, e)?;
        this.cursor.set((i).wrapping_add(1i32));
        this.lastRet.set(-1i32);
        this.expectedModCount.set(this.this$0.get().modCount.get());
        i = todo!("stack underflow");
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
