#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ArrayList$ListItr",
    super_class = "java/util/ArrayList$Itr",
    interfaces  = "java/util/ListIterator",
    access      = "",
    source      = "ArrayList.java",
))]
pub struct ArrayList_ListItr {
    #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/util/ArrayList;", access = "final"))]
    pub this_0: Field<Object>,
}

impl ArrayList_ListItr {
    // java: <init>(Ljava/util/ArrayList;I)V
    pub fn new(arg_0: Object, index: i32) -> Result<Self> {
        let this = Self { this_0: Field::new(Default::default()) };
        this.this_0.set(arg_0);
        /* invokespecial Method java/util/ArrayList$Itr.<init>:(Ljava/util/ArrayList;)V */
        this.cursor.set(index);
        Ok(this)
    }

    // java: hasPrevious()Z
    pub fn hasPrevious(&self) -> Result<bool> {
        let this = self;
        Ok(this.cursor.get()!=0i32)
    }

    // java: nextIndex()I
    pub fn nextIndex(&self) -> Result<i32> {
        let this = self;
        Ok(this.cursor.get())
    }

    // java: previousIndex()I
    pub fn previousIndex(&self) -> Result<i32> {
        let this = self;
        Ok((this.cursor.get()).wrapping_sub(1i32))
    }

    // java: previous()Ljava/lang/Object;
    pub fn previous(&self) -> Result<Object> {
        let this = self;
        this.checkForComodification()?;
        let mut i: i32 = (this.cursor.get()).wrapping_sub(1i32);
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut elementData: Vec<Object> = this.this_0.get().elementData.get();
        return Err(JvmError::Custom("athrow".to_owned()));
        this.cursor.set(i);
        this.lastRet.set(i);
        Ok(elementData[i as usize].clone())
    }

    // java: set(Ljava/lang/Object;)V
    pub fn set(&self, e: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.checkForComodification()?;
        let _t0 = this.this_0.get().set(this.lastRet.get(), e)?;
        let mut ex: i32 = this.lastRet.get();
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: add(Ljava/lang/Object;)V
    pub fn add(&self, e: Object) -> Result<()> {
        let this = self;
        this.checkForComodification()?;
        let mut i: i32 = this.cursor.get();
        this.this_0.get().add(i, e)?;
        this.cursor.set((i).wrapping_add(1i32));
        this.lastRet.set(-1i32);
        this.expectedModCount.set(this.this_0.get().modCount.get());
        i = todo!("stack underflow");
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }
}
