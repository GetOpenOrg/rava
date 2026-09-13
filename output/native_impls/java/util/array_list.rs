/// @synthetic
pub fn new() -> Result<ArrayList<Object>> {
    Ok(ArrayList::default())
}

/// java/util/ArrayList.add:(Ljava/lang/Object;)Z
pub fn add__obj<E: Clone + 'static>(_this: &ArrayList<E>, e: Object) -> Result<bool> {
    let rc = _this.elementData.get();
    rc.borrow_mut().push(e);
    _this.size.set(_this.size.get() + 1);
    Ok(true)
}

/// java/util/ArrayList.size:()I
pub fn size<E: Clone + 'static>(_this: &ArrayList<E>) -> Result<i32> {
    Ok(_this.size.get())
}

/// java/util/ArrayList.get:(I)Ljava/lang/Object;
pub fn get__i<E: Clone + 'static>(_this: &ArrayList<E>, index: i32) -> Result<Object> {
    let rc = _this.elementData.get();
    let vec = rc.borrow();
    vec.get(index as usize).cloned()
        .ok_or(JvmError::ArrayIndexOutOfBoundsException(index))
}

/// java/util/ArrayList.isEmpty:()Z
pub fn isEmpty<E: Clone + 'static>(_this: &ArrayList<E>) -> Result<bool> {
    Ok(_this.size.get() == 0)
}

/// java/util/ArrayList.clear:()V
pub fn clear<E: Clone + 'static>(_this: &ArrayList<E>) -> Result<()> {
    let rc = _this.elementData.get();
    rc.borrow_mut().clear();
    _this.size.set(0);
    Ok(())
}

/// java/util/ArrayList.iterator:()Ljava/util/Iterator;
pub fn iterator<E: Clone + 'static>(_this: &ArrayList<E>) -> Result<Object> {
    let iter = Iterator::<Object> {
        _data: JField::new(_this.elementData.get()),
        _pos: JField::new(0i32),
        _phantom: std::marker::PhantomData,
    };
    Ok(Object::from_any(iter))
}

/// java/util/ArrayList.contains:(Ljava/lang/Object;)Z
pub fn contains__obj<E: Clone + 'static>(_this: &ArrayList<E>, o: Object) -> Result<bool> {
    let data = _this.elementData.get();
    let vec = data.borrow();
    let found = vec.iter().any(|x| std::rc::Rc::ptr_eq(&x.0, &o.0));
    Ok(found)
}
