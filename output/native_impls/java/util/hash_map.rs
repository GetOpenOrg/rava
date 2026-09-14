// @jvm_class: java/util/HashMap
// @jvm_rename: put -> put_obj, get -> get_obj
use java_runtime::prelude::*;
use super::*;

fn _get_store<K: Clone + Default + 'static, V: Clone + Default + 'static>(
    this: &super::HashMap<K, V>,
) -> Rc<RefCell<Vec<(Object, Object)>>> {
    this.table.get().downcast::<Rc<RefCell<Vec<(Object, Object)>>>>()
}

// 支持 String key 内容比较
fn _obj_eq(a: &Object, b: &Object) -> bool {
    if a == b { return true; }
    if let (Some(sa), Some(sb)) = (a.0.downcast_ref::<super::super::string::String>(),
                                    b.0.downcast_ref::<super::super::string::String>()) {
        return sa.to_string() == sb.to_string();
    }
    false
}

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> super::HashMap<K, V> {
    pub fn new() -> Result<HashMap<Object, Object>> {
        let map = HashMap::default();
        let store: Rc<RefCell<Vec<(Object, Object)>>> = Rc::new(RefCell::new(vec![]));
        map.table.set(Object::from_any(store));
        Ok(map)
    }

    pub fn put_obj(&self, key: Object, value: Object) -> Result<Object> {
        let store = _get_store(self);
        let mut vec = store.borrow_mut();
        for (k, v) in vec.iter_mut() {
            if _obj_eq(k, &key) {
                let old = v.clone();
                *v = value;
                return Ok(old);
            }
        }
        vec.push((key, value));
        self.size.set(vec.len() as i32);
        Ok(Object::default())
    }

    pub fn get_obj(&self, key: Object) -> Result<Object> {
        let store = _get_store(self);
        let vec = store.borrow();
        for (k, v) in vec.iter() {
            if _obj_eq(k, &key) {
                return Ok(v.clone());
            }
        }
        Ok(Object::default())
    }

    pub fn containsKey(&self, key: Object) -> Result<bool> {
        let store = _get_store(self);
        let vec = store.borrow();
        Ok(vec.iter().any(|(k, _)| _obj_eq(k, &key)))
    }

    pub fn size(&self) -> Result<i32> {
        let store = _get_store(self);
        let n = store.borrow().len() as i32;
        Ok(n)
    }

    pub fn isEmpty(&self) -> Result<bool> {
        let store = _get_store(self);
        let empty = store.borrow().is_empty();
        Ok(empty)
    }
}

impl<K: Clone + Default + Into<Object> + From<Object> + 'static,
     V: Clone + Default + Into<Object> + From<Object> + 'static> super::HashMap<K, V>
{
    /// Java: new HashMap<K,V>() — 类型安全构造器
    pub fn new_typed() -> Result<HashMap<K, V>> {
        let base = HashMap::<Object, Object>::new()?;
        Ok(HashMap {
            _super: base._super,
            table: base.table,
            entrySet: base.entrySet,
            size: base.size,
            modCount: base.modCount,
            threshold: base.threshold,
            loadFactor: base.loadFactor,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Java: map.put(key, value)  — 无需 .into()
    pub fn put(&self, key: K, value: V) -> Result<Object> {
        self.put_obj(key.into(), value.into())
    }

    /// Java: V v = map.get(key)  — 返回类型安全的 V，无需 downcast
    pub fn get(&self, key: K) -> Result<V> {
        Ok(V::from(self.get_obj(key.into())?))
    }

    /// Java: map.containsKey(key)  — 类型安全 key 查找（snake_case）
    pub fn contains_key(&self, key: K) -> Result<bool> {
        self.containsKey(key.into())
    }
}
