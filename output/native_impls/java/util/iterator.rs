// Iterator 注入字段：迭代状态（Vec 共享引用 + 当前位置）
/// @field _data: JField<Rc<RefCell<Vec<Object>>>>
/// @field _pos:  JField<i32>

/// java/util/Iterator.hasNext:()Z
pub fn hasNext<E: Clone + 'static>(_this: &Iterator<E>) -> Result<bool> {
    Ok(_this._pos.get() < _this._data.get().borrow().len() as i32)
}

/// java/util/Iterator.next:()Ljava/lang/Object;
pub fn next<E: Clone + 'static>(_this: &Iterator<E>) -> Result<Object> {
    let pos = _this._pos.get() as usize;
    let data = _this._data.get();
    let v = data.borrow().get(pos).cloned()
        .ok_or(JvmError::Custom("NoSuchElementException".into()))?;
    _this._pos.set(_this._pos.get() + 1);
    Ok(v)
}
