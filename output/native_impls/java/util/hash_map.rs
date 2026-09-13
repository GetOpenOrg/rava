/// @synthetic
pub fn new() -> Result<HashMap<Object, Object>> {
    let map = HashMap::default();
    let store: Rc<RefCell<Vec<(Object, Object)>>> = Rc::new(RefCell::new(vec![]));
    map.table.set(Object::from_any(store));
    Ok(map)
}

fn _get_store<K: Clone + 'static, V: Clone + 'static>(this: &HashMap<K, V>) -> Rc<RefCell<Vec<(Object, Object)>>> {
    this.table.get().downcast::<Rc<RefCell<Vec<(Object, Object)>>>>()
}

// 在 jdk_classes 上下文中可访问 String，支持 String key 内容比较
fn _obj_eq(a: &Object, b: &Object) -> bool {
    if a == b { return true; }
    if let (Some(sa), Some(sb)) = (a.0.downcast_ref::<String>(), b.0.downcast_ref::<String>()) {
        return sa.to_string() == sb.to_string();
    }
    false
}

/// java/util/HashMap.put:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
pub fn put<K: Clone + 'static, V: Clone + 'static>(_this: &HashMap<K, V>, key: Object, value: Object) -> Result<Object> {
    let store = _get_store(_this);
    let mut vec = store.borrow_mut();
    for (k, v) in vec.iter_mut() {
        if _obj_eq(k, &key) {
            let old = v.clone();
            *v = value;
            return Ok(old);
        }
    }
    vec.push((key, value));
    _this.size.set(vec.len() as i32);
    Ok(Object::default())
}

/// java/util/HashMap.get:(Ljava/lang/Object;)Ljava/lang/Object;
pub fn get<K: Clone + 'static, V: Clone + 'static>(_this: &HashMap<K, V>, key: Object) -> Result<Object> {
    let store = _get_store(_this);
    let vec = store.borrow();
    for (k, v) in vec.iter() {
        if _obj_eq(k, &key) {
            return Ok(v.clone());
        }
    }
    Ok(Object::default())
}

/// java/util/HashMap.containsKey:(Ljava/lang/Object;)Z
pub fn containsKey<K: Clone + 'static, V: Clone + 'static>(_this: &HashMap<K, V>, key: Object) -> Result<bool> {
    let store = _get_store(_this);
    let vec = store.borrow();
    Ok(vec.iter().any(|(k, _)| _obj_eq(k, &key)))
}

/// java/util/HashMap.size:()I
pub fn size<K: Clone + 'static, V: Clone + 'static>(_this: &HashMap<K, V>) -> Result<i32> {
    let store = _get_store(_this);
    let n = store.borrow().len() as i32;
    Ok(n)
}

/// java/util/HashMap.isEmpty:()Z
pub fn isEmpty<K: Clone + 'static, V: Clone + 'static>(_this: &HashMap<K, V>) -> Result<bool> {
    let store = _get_store(_this);
    let empty = store.borrow().is_empty();
    Ok(empty)
}
