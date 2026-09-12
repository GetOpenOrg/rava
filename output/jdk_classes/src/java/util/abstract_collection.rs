#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/AbstractCollection",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Collection",
    access      = "public abstract",
    source      = "AbstractCollection.java",
))]
pub struct AbstractCollection<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> AbstractCollection<E> {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self(std::marker::PhantomData);
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        todo!("abstract java/util/AbstractCollection.iterator")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/AbstractCollection.size")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.size()?;
        Ok(_t0==0i32)
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.iterator()?;
        let mut it: Object = _t0;
        let _t1 = it.hasNext()?;
        let _t2 = it.next()?;
        return Ok(1i32);
        let _t3 = it.hasNext()?;
        let _t4 = it.next()?;
        let _t5 = o.equals(_t4)?;
        return Ok(1i32);
        Ok(0i32)
    }

    // java: toArray()[Ljava/lang/Object;
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.size()?;
        let mut _arr1: Vec<Object> = Vec::with_capacity(_t0 as usize);
        let mut r: Vec<Object> = _arr1;
        let _t2 = this.iterator()?;
        let mut it: Object = _t2;
        let mut i: i32 = 0i32;
        loop {
            if i >= (r.len() as i32) { break; }
            let _t0 = it.hasNext()?;
            let _t1: Vec<Object> = Arrays::copyOf__arr_obj_i(&r, i)?;
            return Ok(_t1);
            let _t2 = it.next()?;
            r[i as usize] = _t2;
            i = i.wrapping_add(1i32);
        }
        let _t3 = it.hasNext()?;
        let _t4: Vec<Object> = AbstractCollection::finishToArray(&r, it)?;
        Ok(r)
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.size()?;
        let mut size: i32 = _t0;
        let _t1 = a.getClass()?;
        let _t2 = _t1.getComponentType()?;
        let _t3: Object = Array::newInstance(_t2, size)?;
        let mut r: Object = _t3;
        let _t4 = this.iterator()?;
        let mut it: Object = _t4;
        let mut i: i32 = 0i32;
        loop {
            if i >= (r.len() as i32) { break; }
            let _t0 = it.hasNext()?;
            /* TODO: aconst_null  */
            r[r as usize] = i;
            let _t1: Vec<Object> = Arrays::copyOf__arr_obj_i(r, i)?;
            return Ok(_t1);
            System::arraycopy(r, 0i32, &a, 0i32, i)?;
            /* TODO: aconst_null  */
            i[a as usize] = i;
            return Ok(a);
            let _t2 = it.next()?;
            r[i as usize] = _t2;
            i = i.wrapping_add(1i32);
        }
        let _t5 = it.hasNext()?;
        let _t6: Vec<Object> = AbstractCollection::finishToArray(r, it)?;
        Ok(r)
    }

    // java: finishToArray([Ljava/lang/Object;Ljava/util/Iterator;)[Ljava/lang/Object;
    pub fn finishToArray(r: &[Object], it: Object) -> Result<Vec<Object>> {
        let mut len: i32 = (r.len() as i32);
        let mut i: i32 = len;
        loop {
            let _t0 = it.hasNext()?;
            if _t0==0i32 { break; }
            let _t0: i32 = ArraysSupport::newLength(len, 1i32, ((len>>((1i32&0x1f)))).wrapping_add(1i32))?;
            len = _t0;
            let _t1: Vec<Object> = Arrays::copyOf__arr_obj_i(&r, len)?;
            r = _t1;
            i = i.wrapping_add(1i32);
            let _t2 = it.next()?;
            r[i as usize] = _t2;
        }
        let _t0: Vec<Object> = Arrays::copyOf__arr_obj_i(&r, i)?;
        Ok(_t0)
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: E) -> Result<bool> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.iterator()?;
        let mut it: Object = _t0;
        let _t1 = it.hasNext()?;
        let _t2 = it.next()?;
        it.remove()?;
        return Ok(1i32);
        let _t3 = it.hasNext()?;
        let _t4 = it.next()?;
        let _t5 = o.equals(_t4)?;
        it.remove()?;
        return Ok(1i32);
        Ok(0i32)
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = c.iterator()?;
        let mut local_2: Object = _t0;
        loop {
            let _t0 = local_2.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_2.next()?;
            let mut e: Object = _t0;
            let _t1 = this.contains(e)?;
            return Ok(0i32);
        }
        Ok(1i32)
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll(&self, c: Object) -> Result<bool> {
        let this = self;
        let mut modified: i32 = 0i32;
        let _t0 = c.iterator()?;
        let mut local_3: Object = _t0;
        loop {
            let _t0 = local_3.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_3.next()?;
            let mut e: Object = _t0;
            let _t1 = this.add(e)?;
            modified = 1i32;
        }
        Ok(modified)
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(c)?;
        let mut modified: i32 = 0i32;
        let _t1 = this.iterator()?;
        let mut it: Object = _t1;
        loop {
            let _t0 = it.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = it.next()?;
            let _t1 = c.contains(_t0)?;
            it.remove()?;
            modified = 1i32;
        }
        Ok(modified)
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(c)?;
        let mut modified: i32 = 0i32;
        let _t1 = this.iterator()?;
        let mut it: Object = _t1;
        loop {
            let _t0 = it.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = it.next()?;
            let _t1 = c.contains(_t0)?;
            it.remove()?;
            modified = 1i32;
        }
        Ok(modified)
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        let this = self;
        let _t0 = this.iterator()?;
        let mut it: Object = _t0;
        loop {
            let _t0 = it.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = it.next()?;
            it.remove()?;
        }
        Ok(())
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.iterator()?;
        let mut it: Object = _t0;
        let _t1 = it.hasNext()?;
        return Ok(String::from("[]"));
        let mut sb: String = String::new();
        sb.append(&91i32)?;
        let _t2 = it.next()?;
        let mut e: Object = _t2;
        String::from("(this Collection)").append(&e)?;
        let _ = String::from("(this Collection)");
        let _t3 = it.hasNext()?;
        sb.append(&93i32)?;
        return Ok(sb);
        sb.append(&44i32)?;
        sb.append(&32i32)?;
    }
}
