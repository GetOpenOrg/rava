/// @synthetic
pub fn new_default() -> Result<HashSet<Object>> {
    let set = HashSet::default();
    let store: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![]));
    set.map.set(Object::from_any(store));
    Ok(set)
}

fn _get_store<E: Clone + 'static>(this: &HashSet<E>) -> Rc<RefCell<Vec<Object>>> {
    this.map.get().downcast::<Rc<RefCell<Vec<Object>>>>()
}

fn _obj_eq(a: &Object, b: &Object) -> bool {
    if a == b { return true; }
    if let (Some(sa), Some(sb)) = (a.0.downcast_ref::<String>(), b.0.downcast_ref::<String>()) {
        return sa.to_string() == sb.to_string();
    }
    false
}

/// java/util/HashSet.add:(Ljava/lang/Object;)Z
pub fn add<E: Clone + 'static>(_this: &HashSet<E>, e: Object) -> Result<bool> {
    let store = _get_store(_this);
    let mut vec = store.borrow_mut();
    if vec.iter().any(|x| _obj_eq(x, &e)) {
        return Ok(false);
    }
    vec.push(e);
    Ok(true)
}

/// java/util/HashSet.contains:(Ljava/lang/Object;)Z
pub fn contains<E: Clone + 'static>(_this: &HashSet<E>, o: Object) -> Result<bool> {
    let store = _get_store(_this);
    let vec = store.borrow();
    Ok(vec.iter().any(|x| _obj_eq(x, &o)))
}

/// java/util/HashSet.size:()I
pub fn size<E: Clone + 'static>(_this: &HashSet<E>) -> Result<i32> {
    let store = _get_store(_this);
    let n = store.borrow().len() as i32;
    Ok(n)
}

/// java/util/HashSet.isEmpty:()Z
pub fn isEmpty<E: Clone + 'static>(_this: &HashSet<E>) -> Result<bool> {
    let store = _get_store(_this);
    let empty = store.borrow().is_empty();
    Ok(empty)
}
