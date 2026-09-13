/// @synthetic
pub fn new_default() -> Result<ArrayList<Object>> {
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
