#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public",
    source      = "Collections.java",
))]
pub struct Collections;

impl Collections {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: sort(Ljava/util/List;)V
    // java: sort(Ljava/util/List;)V
    pub fn sort__list(list: Object) -> Result<()> {
        /* TODO: aconst_null  */
        todo!("stack underflow").sort(list)?;
        Ok(())
    }

    // java: sort(Ljava/util/List;Ljava/util/Comparator;)V
    // java: sort(Ljava/util/List;Ljava/util/Comparator;)V
    pub fn sort__list_compar(list: Object, c: Object) -> Result<()> {
        list.sort(c)?;
        Ok(())
    }

    // java: binarySearch(Ljava/util/List;Ljava/lang/Object;)I
    // java: binarySearch(Ljava/util/List;Ljava/lang/Object;)I
    pub fn binarySearch__list_obj(list: Object, key: Object) -> Result<i32> {
        let _t0 = list.size()?;
        let _t1: i32 = Collections::indexedBinarySearch__list_obj(list, key)?;
        return Ok(_t1);
        let _t2: i32 = Collections::iteratorBinarySearch__list_obj(list, key)?;
        Ok(_t2)
    }

    // java: indexedBinarySearch(Ljava/util/List;Ljava/lang/Object;)I
    // java: indexedBinarySearch(Ljava/util/List;Ljava/lang/Object;)I
    pub fn indexedBinarySearch__list_obj(list: Object, key: Object) -> Result<i32> {
        let mut low: i32 = 0i32;
        let _t0 = list.size()?;
        let mut high: i32 = (_t0).wrapping_sub(1i32);
        loop {
            if low > high { break; }
            let mut mid: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            let _t0 = list.get(mid)?;
            let mut midVal: Object = _t0;
            let _t1 = midVal.compareTo(key)?;
            let mut cmp: i32 = _t1;
            low = (mid).wrapping_add(1i32);
            high = (mid).wrapping_sub(1i32);
            return Ok(mid);
        }
        Ok(((low).wrapping_add(1i32)).wrapping_neg())
    }

    // java: iteratorBinarySearch(Ljava/util/List;Ljava/lang/Object;)I
    // java: iteratorBinarySearch(Ljava/util/List;Ljava/lang/Object;)I
    pub fn iteratorBinarySearch__list_obj(list: Object, key: Object) -> Result<i32> {
        let mut low: i32 = 0i32;
        let _t0 = list.size()?;
        let mut high: i32 = (_t0).wrapping_sub(1i32);
        let _t1 = list.listIterator()?;
        let mut i: Object = _t1;
        loop {
            if low > high { break; }
            let mut mid: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            let _t0: Object = Collections::get(i, mid)?;
            let mut midVal: Object = _t0;
            let _t1 = midVal.compareTo(key)?;
            let mut cmp: i32 = _t1;
            low = (mid).wrapping_add(1i32);
            high = (mid).wrapping_sub(1i32);
            return Ok(mid);
        }
        Ok(((low).wrapping_add(1i32)).wrapping_neg())
    }

    // java: get(Ljava/util/ListIterator;I)Ljava/lang/Object;
    pub fn get(i: Object, index: i32) -> Result<Object> {
        let _t0 = i.nextIndex()?;
        let mut pos: i32 = _t0;
        let _t1 = i.next()?;
        let mut obj: Object = _t1;
        pos = pos.wrapping_add(1i32);
        let _t2 = i.previous()?;
        obj = _t2;
        pos = pos.wrapping_sub(1i32);
        Ok(obj)
    }

    // java: binarySearch(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I
    // java: binarySearch(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn binarySearch__list_obj_compar(list: Object, key: Object, c: Object) -> Result<i32> {
        let _t0: i32 = Collections::binarySearch__list_obj(list, key)?;
        return Ok(_t0);
        let _t1 = list.size()?;
        let _t2: i32 = Collections::indexedBinarySearch__list_obj_compar(list, key, c)?;
        return Ok(_t2);
        let _t3: i32 = Collections::iteratorBinarySearch__list_obj_compar(list, key, c)?;
        Ok(_t3)
    }

    // java: indexedBinarySearch(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I
    // java: indexedBinarySearch(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn indexedBinarySearch__list_obj_compar(l: Object, key: Object, c: Object) -> Result<i32> {
        let mut low: i32 = 0i32;
        let _t0 = l.size()?;
        let mut high: i32 = (_t0).wrapping_sub(1i32);
        loop {
            if low > high { break; }
            let mut mid: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            let _t0 = l.get(mid)?;
            let mut midVal: Object = _t0;
            let _t1 = c.compare(midVal, key)?;
            let mut cmp: i32 = _t1;
            low = (mid).wrapping_add(1i32);
            high = (mid).wrapping_sub(1i32);
            return Ok(mid);
        }
        Ok(((low).wrapping_add(1i32)).wrapping_neg())
    }

    // java: iteratorBinarySearch(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I
    // java: iteratorBinarySearch(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn iteratorBinarySearch__list_obj_compar(l: Object, key: Object, c: Object) -> Result<i32> {
        let mut low: i32 = 0i32;
        let _t0 = l.size()?;
        let mut high: i32 = (_t0).wrapping_sub(1i32);
        let _t1 = l.listIterator()?;
        let mut i: Object = _t1;
        loop {
            if low > high { break; }
            let mut mid: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            let _t0: Object = Collections::get(i, mid)?;
            let mut midVal: Object = _t0;
            let _t1 = c.compare(midVal, key)?;
            let mut cmp: i32 = _t1;
            low = (mid).wrapping_add(1i32);
            high = (mid).wrapping_sub(1i32);
            return Ok(mid);
        }
        Ok(((low).wrapping_add(1i32)).wrapping_neg())
    }

    // java: reverse(Ljava/util/List;)V
    pub fn reverse(list: Object) -> Result<()> {
        let _t0 = list.size()?;
        let mut size: i32 = _t0;
        let mut i: i32 = 0i32;
        let mut mid: i32 = (size>>((1i32&0x1f)));
        let mut j: i32 = (size).wrapping_sub(1i32);
        loop {
            if i >= mid { break; }
            Collections::swap__list_i_i(list, i, j)?;
            i = i.wrapping_add(1i32);
            j = j.wrapping_sub(1i32);
        }
        let _t1 = list.listIterator()?;
        i = _t1;
        let _t2 = list.listIterator(size)?;
        mid = _t2;
        j = 0i32;
        let _t3 = list.size()?;
        let mut mid: i32 = (_t3>>((1i32&0x1f)));
        loop {
            if j >= mid { break; }
            let _t0 = i.next()?;
            let mut tmp: Object = _t0;
            let _t1 = mid.previous()?;
            i.set(_t1)?;
            mid.set(tmp)?;
            j = j.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: shuffle(Ljava/util/List;)V
    // java: shuffle(Ljava/util/List;)V
    pub fn shuffle__list(list: Object) -> Result<()> {
        let mut rnd: Object = Collections::r();
        rnd = Random::new()?;
        Collections::r(Random::new()?);
        Collections::shuffle__list_random(list, rnd)?;
        Ok(())
    }

    // java: shuffle(Ljava/util/List;Ljava/util/Random;)V
    // java: shuffle(Ljava/util/List;Ljava/util/Random;)V
    pub fn shuffle__list_random(list: Object, rnd: Object) -> Result<()> {
        Collections::shuffle__list_random(list, rnd)?;
        Ok(())
    }

    // java: shuffle(Ljava/util/List;Ljava/util/random/RandomGenerator;)V
    // java: shuffle(Ljava/util/List;Ljava/util/random/RandomGenerator;)V
    pub fn shuffle__list_random(list: Object, rnd: Object) -> Result<()> {
        let _t0 = list.size()?;
        let mut size: i32 = _t0;
        let mut i: i32 = size;
        loop {
            if i <= 1i32 { break; }
            let _t0 = rnd.nextInt(i)?;
            Collections::swap__list_i_i(list, (i).wrapping_sub(1i32), _t0)?;
            i = i.wrapping_sub(1i32);
        }
        let _t1 = list.toArray()?;
        i = _t1;
        let mut i: i32 = size;
        loop {
            if i <= 1i32 { break; }
            let _t0 = rnd.nextInt(i)?;
            Collections::swap__arr_obj_i_i(i, (i).wrapping_sub(1i32), _t0)?;
            i = i.wrapping_sub(1i32);
        }
        let _t2 = list.listIterator()?;
        i = _t2;
        let mut local_5: i32 = i;
        let mut local_6: i32 = (local_5.len() as i32);
        let mut local_7: i32 = 0i32;
        loop {
            if local_7 >= local_6 { break; }
            let mut e: Object = local_5[local_7 as usize].clone();
            let _t0 = i.next()?;
            i.set(e)?;
            local_7 = local_7.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: swap(Ljava/util/List;II)V
    // java: swap(Ljava/util/List;II)V
    pub fn swap__list_i_i(list: Object, i: i32, j: i32) -> Result<()> {
        let mut l: Object = list;
        let _t0 = l.get(i)?;
        let _t1 = l.set(j, _t0)?;
        let _t2 = l.set(i, _t1)?;
        Ok(())
    }

    // java: swap([Ljava/lang/Object;II)V
    // java: swap([Ljava/lang/Object;II)V
    pub fn swap__arr_obj_i_i(arr: &[Object], i: i32, j: i32) -> Result<()> {
        let mut tmp: Object = arr[i as usize].clone();
        arr[i as usize] = arr[j as usize].clone();
        arr[j as usize] = tmp;
        Ok(())
    }

    // java: fill(Ljava/util/List;Ljava/lang/Object;)V
    pub fn fill(list: Object, obj: Object) -> Result<()> {
        let _t0 = list.size()?;
        let mut size: i32 = _t0;
        let mut i: i32 = 0i32;
        loop {
            if i >= size { break; }
            let _t0 = list.set(i, obj)?;
            i = i.wrapping_add(1i32);
        }
        let _t1 = list.listIterator()?;
        i = _t1;
        let mut i: i32 = 0i32;
        loop {
            if i >= size { break; }
            let _t0 = i.next()?;
            i.set(obj)?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: copy(Ljava/util/List;Ljava/util/List;)V
    pub fn copy(dest: Object, src: Object) -> Result<()> {
        let _t0 = src.size()?;
        let mut srcSize: i32 = _t0;
        let _t1 = dest.size()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut i: i32 = 0i32;
        loop {
            if i >= srcSize { break; }
            let _t0 = src.get(i)?;
            let _t1 = dest.set(i, _t0)?;
            i = i.wrapping_add(1i32);
        }
        let _t2 = dest.listIterator()?;
        i = _t2;
        let _t3 = src.listIterator()?;
        let mut si: Object = _t3;
        let mut i: i32 = 0i32;
        loop {
            if i >= srcSize { break; }
            let _t0 = i.next()?;
            let _t1 = si.next()?;
            i.set(_t1)?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: min(Ljava/util/Collection;)Ljava/lang/Object;
    // java: min(Ljava/util/Collection;)Ljava/lang/Object;
    pub fn min__coll(coll: Object) -> Result<Object> {
        let _t0 = coll.iterator()?;
        let mut i: Object = _t0;
        let _t1 = i.next()?;
        let mut candidate: Object = _t1;
        loop {
            let _t0 = i.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = i.next()?;
            let mut next: Object = _t0;
            let _t1 = next.compareTo(candidate)?;
            candidate = next;
        }
        Ok(candidate)
    }

    // java: min(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;
    // java: min(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;
    pub fn min__coll_compar(coll: Object, comp: Object) -> Result<Object> {
        let _t0: Object = Collections::min__coll(coll)?;
        return Ok(_t0);
        let _t1 = coll.iterator()?;
        let mut i: Object = _t1;
        let _t2 = i.next()?;
        let mut candidate: Object = _t2;
        loop {
            let _t0 = i.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = i.next()?;
            let mut next: Object = _t0;
            let _t1 = comp.compare(next, candidate)?;
            candidate = next;
        }
        Ok(candidate)
    }

    // java: max(Ljava/util/Collection;)Ljava/lang/Object;
    // java: max(Ljava/util/Collection;)Ljava/lang/Object;
    pub fn max__coll(coll: Object) -> Result<Object> {
        let _t0 = coll.iterator()?;
        let mut i: Object = _t0;
        let _t1 = i.next()?;
        let mut candidate: Object = _t1;
        loop {
            let _t0 = i.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = i.next()?;
            let mut next: Object = _t0;
            let _t1 = next.compareTo(candidate)?;
            candidate = next;
        }
        Ok(candidate)
    }

    // java: max(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;
    // java: max(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;
    pub fn max__coll_compar(coll: Object, comp: Object) -> Result<Object> {
        let _t0: Object = Collections::max__coll(coll)?;
        return Ok(_t0);
        let _t1 = coll.iterator()?;
        let mut i: Object = _t1;
        let _t2 = i.next()?;
        let mut candidate: Object = _t2;
        loop {
            let _t0 = i.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = i.next()?;
            let mut next: Object = _t0;
            let _t1 = comp.compare(next, candidate)?;
            candidate = next;
        }
        Ok(candidate)
    }

    // java: rotate(Ljava/util/List;I)V
    pub fn rotate(list: Object, distance: i32) -> Result<()> {
        let _t0 = list.size()?;
        Collections::rotate1(list, distance)?;
        Collections::rotate2(list, distance)?;
        Ok(())
    }

    // java: rotate1(Ljava/util/List;I)V
    pub fn rotate1(list: Object, distance: i32) -> Result<()> {
        let _t0 = list.size()?;
        let mut size: i32 = _t0;
        return Ok(());
        distance = (distance%size);
        distance = (distance).wrapping_add(size);
        return Ok(());
        let mut bound: i32 = (size).wrapping_sub(distance);
        let mut cycleStart: i32 = 0i32;
        let mut nMoved: i32 = 0i32;
        loop {
            if nMoved >= size { break; }
            let _t0 = list.get(cycleStart)?;
            let mut displaced: Object = _t0;
            let mut i: i32 = cycleStart;
            i = (i).wrapping_sub(size);
            i = (i).wrapping_add(distance);
            let _t1 = list.set(i, displaced)?;
            displaced = _t1;
            nMoved = nMoved.wrapping_add(1i32);
            cycleStart = cycleStart.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: rotate2(Ljava/util/List;I)V
    pub fn rotate2(list: Object, distance: i32) -> Result<()> {
        let _t0 = list.size()?;
        let mut size: i32 = _t0;
        return Ok(());
        let mut mid: i32 = ((distance).wrapping_neg()%size);
        mid = (mid).wrapping_add(size);
        return Ok(());
        let _t1 = list.subList(0i32, mid)?;
        Collections::reverse(_t1)?;
        let _t2 = list.subList(mid, size)?;
        Collections::reverse(_t2)?;
        Collections::reverse(list)?;
        Ok(())
    }

    // java: replaceAll(Ljava/util/List;Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn replaceAll(list: Object, oldVal: Object, newVal: Object) -> Result<bool> {
        let mut result: i32 = 0i32;
        let _t0 = list.size()?;
        let mut size: i32 = _t0;
        let mut i: i32 = 0i32;
        loop {
            if i >= size { break; }
            let _t0 = list.get(i)?;
            let _t1 = list.set(i, newVal)?;
            result = 1i32;
            i = i.wrapping_add(1i32);
        }
        i = 0i32;
        loop {
            if i >= size { break; }
            let _t0 = list.get(i)?;
            let _t1 = oldVal.equals(_t0)?;
            let _t2 = list.set(i, newVal)?;
            result = 1i32;
            i = i.wrapping_add(1i32);
        }
        let _t1 = list.listIterator()?;
        i = _t1;
        let mut i: i32 = 0i32;
        loop {
            if i >= size { break; }
            let _t0 = i.next()?;
            i.set(newVal)?;
            result = 1i32;
            i = i.wrapping_add(1i32);
        }
        i = 0i32;
        loop {
            if i >= size { break; }
            let _t0 = i.next()?;
            let _t1 = oldVal.equals(_t0)?;
            i.set(newVal)?;
            result = 1i32;
            i = i.wrapping_add(1i32);
        }
        Ok(result)
    }

    // java: indexOfSubList(Ljava/util/List;Ljava/util/List;)I
    pub fn indexOfSubList(source: Object, target: Object) -> Result<i32> {
        let _t0 = source.size()?;
        let mut sourceSize: i32 = _t0;
        let _t1 = target.size()?;
        let mut targetSize: i32 = _t1;
        let mut maxCandidate: i32 = (sourceSize).wrapping_sub(targetSize);
        let mut candidate: i32 = 0i32;
        loop {
            if candidate > maxCandidate { break; }
            let mut i: i32 = 0i32;
            let mut j: i32 = candidate;
            let _t0 = target.get(i)?;
            let _t1 = source.get(j)?;
            let _t2: bool = Collections::eq(_t0, _t1)?;
            i = i.wrapping_add(1i32);
            j = j.wrapping_add(1i32);
            return Ok(candidate);
            candidate = candidate.wrapping_add(1i32);
        }
        let _t2 = source.listIterator()?;
        candidate = _t2;
        i = 0i32;
        loop {
            if i > maxCandidate { break; }
            let _t0 = target.listIterator()?;
            j = _t0;
            let mut i: i32 = 0i32;
            let _t1 = j.next()?;
            let _t2 = candidate.next()?;
            let _t3: bool = Collections::eq(_t1, _t2)?;
            let mut j: i32 = 0i32;
            let _t4 = candidate.previous()?;
            j = j.wrapping_add(1i32);
            i = i.wrapping_add(1i32);
            return Ok(i);
            i = i.wrapping_add(1i32);
        }
        Ok(-1i32)
    }

    // java: lastIndexOfSubList(Ljava/util/List;Ljava/util/List;)I
    pub fn lastIndexOfSubList(source: Object, target: Object) -> Result<i32> {
        let _t0 = source.size()?;
        let mut sourceSize: i32 = _t0;
        let _t1 = target.size()?;
        let mut targetSize: i32 = _t1;
        let mut maxCandidate: i32 = (sourceSize).wrapping_sub(targetSize);
        let mut candidate: i32 = maxCandidate;
        loop {
            if candidate<0i32 { break; }
            let mut i: i32 = 0i32;
            let mut j: i32 = candidate;
            let _t0 = target.get(i)?;
            let _t1 = source.get(j)?;
            let _t2: bool = Collections::eq(_t0, _t1)?;
            i = i.wrapping_add(1i32);
            j = j.wrapping_add(1i32);
            return Ok(candidate);
            candidate = candidate.wrapping_sub(1i32);
        }
        return Ok(-1i32);
        let _t2 = source.listIterator(maxCandidate)?;
        candidate = _t2;
        i = maxCandidate;
        loop {
            if i<0i32 { break; }
            let _t0 = target.listIterator()?;
            j = _t0;
            let mut i: i32 = 0i32;
            let _t1 = j.next()?;
            let _t2 = candidate.next()?;
            let _t3: bool = Collections::eq(_t1, _t2)?;
            let mut j: i32 = 0i32;
            let _t4 = candidate.previous()?;
            j = j.wrapping_add(1i32);
            i = i.wrapping_add(1i32);
            return Ok(i);
            i = i.wrapping_sub(1i32);
        }
        Ok(-1i32)
    }

    // java: unmodifiableCollection(Ljava/util/Collection;)Ljava/util/Collection;
    pub fn unmodifiableCollection(c: Object) -> Result<Object> {
        let _t0 = c.getClass()?;
        return Ok(c);
        Ok(Collections_UnmodifiableCollection::new(c)?)
    }

    // java: unmodifiableSequencedCollection(Ljava/util/SequencedCollection;)Ljava/util/SequencedCollection;
    pub fn unmodifiableSequencedCollection(c: Object) -> Result<Object> {
        let _t0 = c.getClass()?;
        return Ok(c);
        Ok(Collections_UnmodifiableSequencedCollection::new(c)?)
    }

    // java: unmodifiableSet(Ljava/util/Set;)Ljava/util/Set;
    pub fn unmodifiableSet(s: Object) -> Result<Object> {
        let _t0 = s.getClass()?;
        return Ok(s);
        Ok(Collections_UnmodifiableSet::new(s)?)
    }

    // java: unmodifiableSequencedSet(Ljava/util/SequencedSet;)Ljava/util/SequencedSet;
    pub fn unmodifiableSequencedSet(s: Object) -> Result<Object> {
        let _t0 = s.getClass()?;
        return Ok(s);
        Ok(Collections_UnmodifiableSequencedSet::new(s)?)
    }

    // java: unmodifiableSortedSet(Ljava/util/SortedSet;)Ljava/util/SortedSet;
    pub fn unmodifiableSortedSet(s: Object) -> Result<Object> {
        let _t0 = s.getClass()?;
        return Ok(s);
        Ok(Collections_UnmodifiableSortedSet::new(s)?)
    }

    // java: unmodifiableNavigableSet(Ljava/util/NavigableSet;)Ljava/util/NavigableSet;
    pub fn unmodifiableNavigableSet(s: Object) -> Result<Object> {
        let _t0 = s.getClass()?;
        return Ok(s);
        Ok(Collections_UnmodifiableNavigableSet::new(s)?)
    }

    // java: unmodifiableList(Ljava/util/List;)Ljava/util/List;
    pub fn unmodifiableList(list: Object) -> Result<Object> {
        let _t0 = list.getClass()?;
        let _t1 = list.getClass()?;
        return Ok(list);
        Ok(Collections_UnmodifiableList::new(list)?)
    }

    // java: unmodifiableMap(Ljava/util/Map;)Ljava/util/Map;
    pub fn unmodifiableMap(m: Object) -> Result<Object> {
        let _t0 = m.getClass()?;
        return Ok(m);
        Ok(Collections_UnmodifiableMap::new(m)?)
    }

    // java: unmodifiableSequencedMap(Ljava/util/SequencedMap;)Ljava/util/SequencedMap;
    pub fn unmodifiableSequencedMap(m: Object) -> Result<Object> {
        let _t0 = m.getClass()?;
        return Ok(m);
        Ok(Collections_UnmodifiableSequencedMap::new(m)?)
    }

    // java: unmodifiableSortedMap(Ljava/util/SortedMap;)Ljava/util/SortedMap;
    pub fn unmodifiableSortedMap(m: Object) -> Result<Object> {
        let _t0 = m.getClass()?;
        return Ok(m);
        Ok(Collections_UnmodifiableSortedMap::new(m)?)
    }

    // java: unmodifiableNavigableMap(Ljava/util/NavigableMap;)Ljava/util/NavigableMap;
    pub fn unmodifiableNavigableMap(m: Object) -> Result<Object> {
        let _t0 = m.getClass()?;
        return Ok(m);
        Ok(Collections_UnmodifiableNavigableMap::new(m)?)
    }

    // java: synchronizedCollection(Ljava/util/Collection;)Ljava/util/Collection;
    // java: synchronizedCollection(Ljava/util/Collection;)Ljava/util/Collection;
    pub fn synchronizedCollection__coll(c: Object) -> Result<Object> {
        Ok(Collections_SynchronizedCollection::new(c)?)
    }

    // java: synchronizedCollection(Ljava/util/Collection;Ljava/lang/Object;)Ljava/util/Collection;
    // java: synchronizedCollection(Ljava/util/Collection;Ljava/lang/Object;)Ljava/util/Collection;
    pub fn synchronizedCollection__coll_obj(c: Object, mutex: Object) -> Result<Object> {
        Ok(Collections_SynchronizedCollection::new(c, mutex)?)
    }

    // java: synchronizedSet(Ljava/util/Set;)Ljava/util/Set;
    // java: synchronizedSet(Ljava/util/Set;)Ljava/util/Set;
    pub fn synchronizedSet__set(s: Object) -> Result<Object> {
        Ok(Collections_SynchronizedSet::new(s)?)
    }

    // java: synchronizedSet(Ljava/util/Set;Ljava/lang/Object;)Ljava/util/Set;
    // java: synchronizedSet(Ljava/util/Set;Ljava/lang/Object;)Ljava/util/Set;
    pub fn synchronizedSet__set_obj(s: Object, mutex: Object) -> Result<Object> {
        Ok(Collections_SynchronizedSet::new(s, mutex)?)
    }

    // java: synchronizedSortedSet(Ljava/util/SortedSet;)Ljava/util/SortedSet;
    pub fn synchronizedSortedSet(s: Object) -> Result<Object> {
        Ok(Collections_SynchronizedSortedSet::new(s)?)
    }

    // java: synchronizedNavigableSet(Ljava/util/NavigableSet;)Ljava/util/NavigableSet;
    pub fn synchronizedNavigableSet(s: Object) -> Result<Object> {
        Ok(Collections_SynchronizedNavigableSet::new(s)?)
    }

    // java: synchronizedList(Ljava/util/List;)Ljava/util/List;
    // java: synchronizedList(Ljava/util/List;)Ljava/util/List;
    pub fn synchronizedList__list(list: Object) -> Result<Object> {
        Ok(Collections_SynchronizedList::new(list)?)
    }

    // java: synchronizedList(Ljava/util/List;Ljava/lang/Object;)Ljava/util/List;
    // java: synchronizedList(Ljava/util/List;Ljava/lang/Object;)Ljava/util/List;
    pub fn synchronizedList__list_obj(list: Object, mutex: Object) -> Result<Object> {
        Ok(Collections_SynchronizedList::new(list, mutex)?)
    }

    // java: synchronizedMap(Ljava/util/Map;)Ljava/util/Map;
    pub fn synchronizedMap(m: Object) -> Result<Object> {
        Ok(Collections_SynchronizedMap::new(m)?)
    }

    // java: synchronizedSortedMap(Ljava/util/SortedMap;)Ljava/util/SortedMap;
    pub fn synchronizedSortedMap(m: Object) -> Result<Object> {
        Ok(Collections_SynchronizedSortedMap::new(m)?)
    }

    // java: synchronizedNavigableMap(Ljava/util/NavigableMap;)Ljava/util/NavigableMap;
    pub fn synchronizedNavigableMap(m: Object) -> Result<Object> {
        Ok(Collections_SynchronizedNavigableMap::new(m)?)
    }

    // java: checkedCollection(Ljava/util/Collection;Ljava/lang/Class;)Ljava/util/Collection;
    pub fn checkedCollection(c: Object, type_: Object) -> Result<Object> {
        Ok(Collections_CheckedCollection::new(c, type_)?)
    }

    // java: zeroLengthArray(Ljava/lang/Class;)[Ljava/lang/Object;
    pub fn zeroLengthArray(type_: Object) -> Result<Vec<Object>> {
        let _t0: Object = Array::newInstance(type_, 0i32)?;
        Ok(_t0)
    }

    // java: checkedQueue(Ljava/util/Queue;Ljava/lang/Class;)Ljava/util/Queue;
    pub fn checkedQueue(queue: Object, type_: Object) -> Result<Object> {
        Ok(Collections_CheckedQueue::new(queue, type_)?)
    }

    // java: checkedSet(Ljava/util/Set;Ljava/lang/Class;)Ljava/util/Set;
    pub fn checkedSet(s: Object, type_: Object) -> Result<Object> {
        Ok(Collections_CheckedSet::new(s, type_)?)
    }

    // java: checkedSortedSet(Ljava/util/SortedSet;Ljava/lang/Class;)Ljava/util/SortedSet;
    pub fn checkedSortedSet(s: Object, type_: Object) -> Result<Object> {
        Ok(Collections_CheckedSortedSet::new(s, type_)?)
    }

    // java: checkedNavigableSet(Ljava/util/NavigableSet;Ljava/lang/Class;)Ljava/util/NavigableSet;
    pub fn checkedNavigableSet(s: Object, type_: Object) -> Result<Object> {
        Ok(Collections_CheckedNavigableSet::new(s, type_)?)
    }

    // java: checkedList(Ljava/util/List;Ljava/lang/Class;)Ljava/util/List;
    pub fn checkedList(list: Object, type_: Object) -> Result<Object> {
        Ok(Collections_CheckedList::new(list, type_)?)
    }

    // java: checkedMap(Ljava/util/Map;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/Map;
    pub fn checkedMap(m: Object, keyType: Object, valueType: Object) -> Result<Object> {
        Ok(Collections_CheckedMap::new(m, keyType, valueType)?)
    }

    // java: checkedSortedMap(Ljava/util/SortedMap;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/SortedMap;
    pub fn checkedSortedMap(m: Object, keyType: Object, valueType: Object) -> Result<Object> {
        Ok(Collections_CheckedSortedMap::new(m, keyType, valueType)?)
    }

    // java: checkedNavigableMap(Ljava/util/NavigableMap;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/NavigableMap;
    pub fn checkedNavigableMap(m: Object, keyType: Object, valueType: Object) -> Result<Object> {
        Ok(Collections_CheckedNavigableMap::new(m, keyType, valueType)?)
    }

    // java: emptyIterator()Ljava/util/Iterator;
    pub fn emptyIterator() -> Result<Object> {
        Ok(Collections_EmptyIterator::EMPTY_ITERATOR())
    }

    // java: emptyListIterator()Ljava/util/ListIterator;
    pub fn emptyListIterator() -> Result<Object> {
        Ok(Collections_EmptyListIterator::EMPTY_ITERATOR())
    }

    // java: emptyEnumeration()Ljava/util/Enumeration;
    pub fn emptyEnumeration() -> Result<Object> {
        Ok(Collections_EmptyEnumeration::EMPTY_ENUMERATION())
    }

    // java: emptySet()Ljava/util/Set;
    pub fn emptySet() -> Result<Object> {
        Ok(Collections::EMPTY_SET())
    }

    // java: emptySortedSet()Ljava/util/SortedSet;
    pub fn emptySortedSet() -> Result<Object> {
        Ok(Collections_UnmodifiableNavigableSet::EMPTY_NAVIGABLE_SET())
    }

    // java: emptyNavigableSet()Ljava/util/NavigableSet;
    pub fn emptyNavigableSet() -> Result<Object> {
        Ok(Collections_UnmodifiableNavigableSet::EMPTY_NAVIGABLE_SET())
    }

    // java: emptyList()Ljava/util/List;
    pub fn emptyList() -> Result<Object> {
        Ok(Collections::EMPTY_LIST())
    }

    // java: emptyMap()Ljava/util/Map;
    pub fn emptyMap() -> Result<Object> {
        Ok(Collections::EMPTY_MAP())
    }

    // java: emptySortedMap()Ljava/util/SortedMap;
    pub fn emptySortedMap() -> Result<Object> {
        Ok(Collections_UnmodifiableNavigableMap::EMPTY_NAVIGABLE_MAP())
    }

    // java: emptyNavigableMap()Ljava/util/NavigableMap;
    pub fn emptyNavigableMap() -> Result<Object> {
        Ok(Collections_UnmodifiableNavigableMap::EMPTY_NAVIGABLE_MAP())
    }

    // java: singleton(Ljava/lang/Object;)Ljava/util/Set;
    pub fn singleton(o: Object) -> Result<Object> {
        Ok(Collections_SingletonSet::new(o)?)
    }

    // java: singletonIterator(Ljava/lang/Object;)Ljava/util/Iterator;
    pub fn singletonIterator(e: Object) -> Result<Object> {
        Ok(Collections_1::new(e)?)
    }

    // java: singletonSpliterator(Ljava/lang/Object;)Ljava/util/Spliterator;
    pub fn singletonSpliterator(element: Object) -> Result<Object> {
        Ok(Collections_2::new(element)?)
    }

    // java: singletonList(Ljava/lang/Object;)Ljava/util/List;
    pub fn singletonList(o: Object) -> Result<Object> {
        Ok(Collections_SingletonList::new(o)?)
    }

    // java: singletonMap(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    pub fn singletonMap(key: Object, value: Object) -> Result<Object> {
        Ok(Collections_SingletonMap::new(key, value)?)
    }

    // java: nCopies(ILjava/lang/Object;)Ljava/util/List;
    pub fn nCopies(n: i32, o: Object) -> Result<Object> {
        String::new().append(&String::from("List length ="))?;
        String::new().append(&n)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(Collections_CopiesList::new(n, o)?)
    }

    // java: reverseOrder()Ljava/util/Comparator;
    // java: reverseOrder()Ljava/util/Comparator;
    pub fn reverseOrder() -> Result<Object> {
        Ok(Collections_ReverseComparator::REVERSE_ORDER())
    }

    // java: reverseOrder(Ljava/util/Comparator;)Ljava/util/Comparator;
    // java: reverseOrder(Ljava/util/Comparator;)Ljava/util/Comparator;
    pub fn reverseOrder__compar(cmp: Object) -> Result<Object> {
        return Ok(Collections_ReverseComparator::REVERSE_ORDER());
        return Ok(Comparators_NaturalOrderComparator::INSTANCE());
        return Ok(Collections_ReverseComparator::REVERSE_ORDER());
        return Ok(cmp.cmp.get());
        Ok(Collections_ReverseComparator2::new(cmp)?)
    }

    // java: enumeration(Ljava/util/Collection;)Ljava/util/Enumeration;
    pub fn enumeration(c: Object) -> Result<Object> {
        Ok(Collections_3::new(c)?)
    }

    // java: list(Ljava/util/Enumeration;)Ljava/util/ArrayList;
    pub fn list(e: Object) -> Result<Object> {
        let mut l: ArrayList<_> = ArrayList::<_>::new()?;
        loop {
            let _t0 = e.hasMoreElements()?;
            if _t0==0i32 { break; }
            let _t0 = e.nextElement()?;
            let _t1 = l.add(_t0)?;
        }
        Ok(l)
    }

    // java: eq(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn eq(o1: Object, o2: Object) -> Result<bool> {
        Ok(o2.is_none())
    }

    // java: frequency(Ljava/util/Collection;Ljava/lang/Object;)I
    pub fn frequency(c: Object, o: Object) -> Result<i32> {
        let mut result: i32 = 0i32;
        let _t0 = c.iterator()?;
        let mut local_3: Object = _t0;
        loop {
            let _t0 = local_3.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_3.next()?;
            let mut e: Object = _t0;
            result = result.wrapping_add(1i32);
        }
        let _t1 = c.iterator()?;
        local_3 = _t1;
        loop {
            let _t0 = local_3.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_3.next()?;
            e = _t0;
            let _t1 = o.equals(e)?;
            result = result.wrapping_add(1i32);
        }
        Ok(result)
    }

    // java: disjoint(Ljava/util/Collection;Ljava/util/Collection;)Z
    pub fn disjoint(c1: Object, c2: Object) -> Result<bool> {
        let mut contains: Object = c2;
        let mut iterate: Object = c1;
        iterate = c2;
        contains = c1;
        let _t0 = c1.size()?;
        let mut c1size: i32 = _t0;
        let _t1 = c2.size()?;
        let mut c2size: i32 = _t1;
        return Ok(1i32);
        iterate = c2;
        contains = c1;
        let _t2 = iterate.iterator()?;
        c1size = _t2;
        loop {
            let _t0 = c1size.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = c1size.next()?;
            c2size = _t0;
            let _t1 = contains.contains(c2size)?;
            return Ok(0i32);
        }
        Ok(1i32)
    }

    // java: addAll(Ljava/util/Collection;[Ljava/lang/Object;)Z
    pub fn addAll(c: Object, elements: &[Object]) -> Result<bool> {
        let mut result: i32 = 0i32;
        let mut local_3: Vec<Object> = elements;
        let mut local_4: i32 = (local_3.len() as i32);
        let mut local_5: i32 = 0i32;
        loop {
            if local_5 >= local_4 { break; }
            let mut element: Object = local_3[local_5 as usize].clone();
            let _t0 = c.add(element)?;
            result = (result|_t0);
            local_5 = local_5.wrapping_add(1i32);
        }
        Ok(result)
    }

    // java: newSetFromMap(Ljava/util/Map;)Ljava/util/Set;
    pub fn newSetFromMap(map: Object) -> Result<Object> {
        let _t0 = map.isEmpty()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(Collections_SetFromMap::new(map)?)
    }

    // java: newSequencedSetFromMap(Ljava/util/SequencedMap;)Ljava/util/SequencedSet;
    pub fn newSequencedSetFromMap(map: Object) -> Result<Object> {
        let _t0 = map.isEmpty()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(Collections_SequencedSetFromMap::new(map)?)
    }

    // java: asLifoQueue(Ljava/util/Deque;)Ljava/util/Queue;
    pub fn asLifoQueue(deque: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(deque)?;
        Ok(Collections_AsLIFOQueue::new(_t0)?)
    }
}
