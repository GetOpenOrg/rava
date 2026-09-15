// @jvm_class: java/util/HashSet
// @jvm_rename: add -> add_obj, contains -> contains_obj
use java_runtime::prelude::*;
use super::*;

fn _get_store<E: Clone + Default + 'static>(
    this: &super::HashSet<E>,
) -> Rc<RefCell<Vec<Object>>> {
    this.map.get().downcast::<Rc<RefCell<Vec<Object>>>>()
}

fn _obj_eq(a: &Object, b: &Object) -> bool {
    if a == b { return true; }
    if let (Some(sa), Some(sb)) = (a.0.downcast_ref::<super::super::super::lang::string::String>(),
                                    b.0.downcast_ref::<super::super::super::lang::string::String>()) {
        return sa.to_string() == sb.to_string();
    }
    false
}

impl<E: Clone + Default + 'static> super::HashSet<E> {
    pub fn new() -> Result<HashSet<Object>> {
        let set = HashSet::default();
        let store: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![]));
        set.map.set(Object::from_any(store));
        Ok(set)
    }

    pub fn add_obj(&self, e: Object) -> Result<bool> {
        let store = _get_store(self);
        let mut vec = store.borrow_mut();
        if vec.iter().any(|x| _obj_eq(x, &e)) {
            return Ok(false);
        }
        vec.push(e);
        Ok(true)
    }

    pub fn contains_obj(&self, o: Object) -> Result<bool> {
        let store = _get_store(self);
        let vec = store.borrow();
        Ok(vec.iter().any(|x| _obj_eq(x, &o)))
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

impl<E: Clone + Default + Into<Object> + From<Object> + 'static> super::HashSet<E> {
    /// Java: new HashSet<E>() — 类型安全构造器
    pub fn new_typed() -> Result<HashSet<E>> {
        let base = HashSet::<Object>::new()?;
        Ok(HashSet {
            _super: base._super,
            map: base.map,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Java: set.add(e)  — 无需 .into()
    pub fn add(&self, e: E) -> Result<bool> {
        self.add_obj(e.into())
    }

    /// Java: set.contains(e)  — 类型安全查找，无需 .into()
    pub fn contains(&self, e: E) -> Result<bool> {
        self.contains_obj(e.into())
    }
}
