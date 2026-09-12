#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Set",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Collection",
    access      = "public abstract",
    source      = "Set.java",
))]
pub struct Set<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Set<E> {
    // java: size()I
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/Set.size")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/util/Set.isEmpty")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Set.contains")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        todo!("abstract java/util/Set.iterator")
    }

    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        todo!("abstract java/util/Set.toArray")
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, arg0: Vec<Object>) -> Result<Vec<Object>> {
        todo!("abstract java/util/Set.toArray")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Set.add")
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Set.remove")
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Set.containsAll")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Set.addAll")
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Set.retainAll")
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Set.removeAll")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        todo!("abstract java/util/Set.clear")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Set.equals")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/Set.hashCode")
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Spliterators::spliterator__coll_i(this, 1i32)?;
        Ok(_t0)
    }

    // java: of()Ljava/util/Set;
    // java: of()Ljava/util/Set;
    pub fn of() -> Result<Object> {
        Ok(ImmutableCollections::EMPTY_SET())
    }

    // java: of(Ljava/lang/Object;)Ljava/util/Set;
    // java: of(Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj(e1: E) -> Result<Object> {
        Ok(ImmutableCollections_Set12::new(e1)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj(e1: E, e2: E) -> Result<Object> {
        Ok(ImmutableCollections_Set12::new(e1, e2)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj(e1: E, e2: E, e3: E) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(3i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        Ok(ImmutableCollections_SetN::new(_arr0)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(4i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        Ok(ImmutableCollections_SetN::new(_arr0)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(5i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        _arr0[4i32 as usize] = e5;
        Ok(ImmutableCollections_SetN::new(_arr0)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(6i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        _arr0[4i32 as usize] = e5;
        _arr0[5i32 as usize] = e6;
        Ok(ImmutableCollections_SetN::new(_arr0)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(7i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        _arr0[4i32 as usize] = e5;
        _arr0[5i32 as usize] = e6;
        _arr0[6i32 as usize] = e7;
        Ok(ImmutableCollections_SetN::new(_arr0)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E, e8: E) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(8i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        _arr0[4i32 as usize] = e5;
        _arr0[5i32 as usize] = e6;
        _arr0[6i32 as usize] = e7;
        _arr0[7i32 as usize] = e8;
        Ok(ImmutableCollections_SetN::new(_arr0)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E, e8: E, e9: E) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(9i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        _arr0[4i32 as usize] = e5;
        _arr0[5i32 as usize] = e6;
        _arr0[6i32 as usize] = e7;
        _arr0[7i32 as usize] = e8;
        _arr0[8i32 as usize] = e9;
        Ok(ImmutableCollections_SetN::new(_arr0)?)
    }

    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    // java: of(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E, e8: E, e9: E, e10: E) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(10i32 as usize);
        _arr0[0i32 as usize] = e1;
        _arr0[1i32 as usize] = e2;
        _arr0[2i32 as usize] = e3;
        _arr0[3i32 as usize] = e4;
        _arr0[4i32 as usize] = e5;
        _arr0[5i32 as usize] = e6;
        _arr0[6i32 as usize] = e7;
        _arr0[7i32 as usize] = e8;
        _arr0[8i32 as usize] = e9;
        _arr0[9i32 as usize] = e10;
        Ok(ImmutableCollections_SetN::new(_arr0)?)
    }

    // java: of([Ljava/lang/Object;)Ljava/util/Set;
    // java: of([Ljava/lang/Object;)Ljava/util/Set;
    pub fn of__arr_obj(elements: &[Object]) -> Result<Object> {
        /* TODO: tableswitch default:59 low:0 high:2 */
        let mut set: Object = ImmutableCollections::EMPTY_SET();
        return Ok(set);
        return Ok(ImmutableCollections_Set12::new(elements[0i32 as usize].clone())?);
        return Ok(ImmutableCollections_Set12::new(elements[0i32 as usize].clone(), elements[1i32 as usize].clone())?);
        Ok(ImmutableCollections_SetN::new(elements)?)
    }

    // java: copyOf(Ljava/util/Collection;)Ljava/util/Set;
    pub fn copyOf(coll: Object) -> Result<Object> {
        return Ok(coll);
        let _t0 = coll.isEmpty()?;
        let _t1: Object = Set::of()?;
        return Ok(_t1);
        let _t2 = HashSet::<_>::new()?.toArray()?;
        let _t3: Object = Set::of__arr_obj(&_t2)?;
        Ok(_t3)
    }
}
