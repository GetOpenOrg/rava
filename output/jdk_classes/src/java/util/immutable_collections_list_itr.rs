#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$ListItr",
    super_class = "java/lang/Object",
    interfaces  = "java/util/ListIterator",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_ListItr<E> {
    #[cfg_attr(any(), java_field(name = "list", descriptor = "Ljava/util/List;", access = "private final"))]
    pub list: Field<Object>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "private final"))]
    pub size: Field<i32>,
    #[cfg_attr(any(), java_field(name = "isListIterator", descriptor = "Z", access = "private final"))]
    pub isListIterator: Field<bool>,
    #[cfg_attr(any(), java_field(name = "cursor", descriptor = "I", access = "private"))]
    pub cursor: Field<i32>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> ImmutableCollections_ListItr<E> {
    // java: <init>(Ljava/util/List;I)V
    // java: <init>(Ljava/util/List;I)V
    pub fn new__list_i(list: Object, size: i32) -> Result<Self> {
        let this = Self { list: Field::new(Default::default()), size: Field::new(0), isListIterator: Field::new(false), cursor: Field::new(0), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.list.set(list);
        this.size.set(size);
        this.cursor.set(0i32);
        this.isListIterator.set(0i32);
        Ok(this)
    }

    // java: <init>(Ljava/util/List;II)V
    // java: <init>(Ljava/util/List;II)V
    pub fn new__list_i_i(list: Object, size: i32, index: i32) -> Result<Self> {
        let this = Self { list: Field::new(Default::default()), size: Field::new(0), isListIterator: Field::new(false), cursor: Field::new(0), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.list.set(list);
        this.size.set(size);
        this.cursor.set(index);
        this.isListIterator.set(1i32);
        Ok(this)
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        let this = self;
        Ok(this.cursor.get() != this.size.get())
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<E> {
        let this = self;
        let mut i: i32 = this.cursor.get();
        let _t0 = this.list.get().get(i)?;
        let mut next: Object = _t0;
        this.cursor.set((i).wrapping_add(1i32));
        return Ok(next);
        i = todo!("stack underflow");
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: remove()V
    pub fn remove(&self) -> Result<()> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: hasPrevious()Z
    pub fn hasPrevious(&self) -> Result<bool> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(this.cursor.get()!=0i32)
    }

    // java: previous()Ljava/lang/Object;
    pub fn previous(&self) -> Result<E> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut i: i32 = (this.cursor.get()).wrapping_sub(1i32);
        let _t1 = this.list.get().get(i)?;
        let mut previous: Object = _t1;
        this.cursor.set(i);
        return Ok(previous);
        i = this.isListIterator.get();
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: nextIndex()I
    pub fn nextIndex(&self) -> Result<i32> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(this.cursor.get())
    }

    // java: previousIndex()I
    pub fn previousIndex(&self) -> Result<i32> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok((this.cursor.get()).wrapping_sub(1i32))
    }

    // java: set(Ljava/lang/Object;)V
    pub fn set(&self, e: E) -> Result<()> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: add(Ljava/lang/Object;)V
    pub fn add(&self, e: E) -> Result<()> {
        let this = self;
        let _t0: Object = ImmutableCollections::uoe()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }
}
