#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/TimSort",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "",
    source      = "TimSort.java",
))]
pub struct TimSort<T> {
    #[cfg_attr(any(), java_field(name = "a", descriptor = "[Ljava/lang/Object;", access = "private final"))]
    pub a: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "c", descriptor = "Ljava/util/Comparator;", access = "private final"))]
    pub c: Field<Object>,
    #[cfg_attr(any(), java_field(name = "minGallop", descriptor = "I", access = "private"))]
    pub minGallop: Field<i32>,
    #[cfg_attr(any(), java_field(name = "tmp", descriptor = "[Ljava/lang/Object;", access = "private"))]
    pub tmp: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "tmpBase", descriptor = "I", access = "private"))]
    pub tmpBase: Field<i32>,
    #[cfg_attr(any(), java_field(name = "tmpLen", descriptor = "I", access = "private"))]
    pub tmpLen: Field<i32>,
    #[cfg_attr(any(), java_field(name = "stackSize", descriptor = "I", access = "private"))]
    pub stackSize: Field<i32>,
    #[cfg_attr(any(), java_field(name = "runBase", descriptor = "[I", access = "private final"))]
    pub runBase: Field<Vec<i32>>,
    #[cfg_attr(any(), java_field(name = "runLen", descriptor = "[I", access = "private final"))]
    pub runLen: Field<Vec<i32>>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> TimSort<T> {
    // java: <init>([Ljava/lang/Object;Ljava/util/Comparator;[Ljava/lang/Object;II)V
    pub fn new(a: Vec<Object>, c: Object, work: Vec<Object>, workBase: i32, workLen: i32) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), c: Field::new(Default::default()), minGallop: Field::new(0), tmp: Field::new(Default::default()), tmpBase: Field::new(0), tmpLen: Field::new(0), stackSize: Field::new(0), runBase: Field::new(Default::default()), runLen: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.minGallop.set(7i32);
        this.stackSize.set(0i32);
        this.a.set(a);
        this.c.set(c);
        let mut len: i32 = (a.len() as i32);
        let mut tlen: i32 = 256i32;
        let _t0 = a.getClass()?;
        let _t1 = _t0.getComponentType()?;
        let _t2: Object = Array::newInstance(_t1, tlen)?;
        let mut newArray: Object = _t2;
        this.tmp.set(newArray);
        this.tmpBase.set(0i32);
        this.tmpLen.set(tlen);
        this.tmp.set(work);
        this.tmpBase.set(workBase);
        this.tmpLen.set(workLen);
        newArray = 49i32;
        let mut _arr3: Vec<i32> = vec![0i32; newArray as usize];
        this.runBase.set(_arr3);
        let mut _arr4: Vec<i32> = vec![0i32; newArray as usize];
        this.runLen.set(_arr4);
        Ok(this)
    }

    // java: sort([Ljava/lang/Object;IILjava/util/Comparator;[Ljava/lang/Object;II)V
    pub fn sort(a: &[Object], lo: i32, hi: i32, c: Object, work: &[Object], workBase: i32, workLen: i32) -> Result<()> {
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut nRemaining: i32 = (hi).wrapping_sub(lo);
        return Ok(());
        let _t0: i32 = TimSort::countRunAndMakeAscending(&a, lo, hi, c)?;
        let mut initRunLen: i32 = _t0;
        TimSort::binarySort(&a, lo, hi, (lo).wrapping_add(initRunLen), c)?;
        return Ok(());
        initRunLen = TimSort::new(a, c, work, workBase, workLen)?;
        let _t1: i32 = TimSort::minRunLength(nRemaining)?;
        let mut minRun: i32 = _t1;
        let _t2: i32 = TimSort::countRunAndMakeAscending(&a, lo, hi, c)?;
        let mut runLen: i32 = _t2;
        let mut force: i32 = minRun;
        TimSort::binarySort(&a, lo, (lo).wrapping_add(force), (lo).wrapping_add(runLen), c)?;
        runLen = force;
        initRunLen.pushRun(lo, runLen)?;
        initRunLen.mergeCollapse()?;
        lo = (lo).wrapping_add(runLen);
        nRemaining = (nRemaining).wrapping_sub(runLen);
        return Err(JvmError::Custom("athrow".to_owned()));
        initRunLen.mergeForceCollapse()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: binarySort([Ljava/lang/Object;IIILjava/util/Comparator;)V
    pub fn binarySort(a: &[Object], lo: i32, hi: i32, start: i32, c: Object) -> Result<()> {
        return Err(JvmError::Custom("athrow".to_owned()));
        start = start.wrapping_add(1i32);
        loop {
            if start >= hi { break; }
            let mut pivot: Object = a[start as usize].clone();
            let mut left: i32 = lo;
            let mut right: i32 = start;
            return Err(JvmError::Custom("athrow".to_owned()));
            let mut mid: i32 = (((left).wrapping_add(right) as u32>>(1i32&0x1f)) as i32);
            let _t0 = c.compare(pivot, a[mid as usize].clone())?;
            right = mid;
            left = (mid).wrapping_add(1i32);
            return Err(JvmError::Custom("athrow".to_owned()));
            mid = (start).wrapping_sub(left);
            /* TODO: lookupswitch default:197 1:184 2:172 */
            a[(left).wrapping_add(2i32) as usize] = a[(left).wrapping_add(1i32) as usize].clone();
            a[(left).wrapping_add(1i32) as usize] = a[left as usize].clone();
            System::arraycopy(&a, left, &a, (left).wrapping_add(1i32), mid)?;
            a[left as usize] = pivot;
            start = start.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: countRunAndMakeAscending([Ljava/lang/Object;IILjava/util/Comparator;)I
    pub fn countRunAndMakeAscending(a: &[Object], lo: i32, hi: i32, c: Object) -> Result<i32> {
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut runHi: i32 = (lo).wrapping_add(1i32);
        return Ok(1i32);
        runHi = runHi.wrapping_add(1i32);
        let _t0 = c.compare(a[runHi as usize].clone(), a[lo as usize].clone())?;
        loop {
            if runHi >= hi { break; }
            let _t0 = c.compare(a[runHi as usize].clone(), a[(runHi).wrapping_sub(1i32) as usize].clone())?;
            runHi = runHi.wrapping_add(1i32);
        }
        TimSort::reverseRange(&a, lo, runHi)?;
        loop {
            if runHi >= hi { break; }
            let _t0 = c.compare(a[runHi as usize].clone(), a[(runHi).wrapping_sub(1i32) as usize].clone())?;
            runHi = runHi.wrapping_add(1i32);
        }
        Ok((runHi).wrapping_sub(lo))
    }

    // java: reverseRange([Ljava/lang/Object;II)V
    pub fn reverseRange(a: &[Object], lo: i32, hi: i32) -> Result<()> {
        hi = hi.wrapping_sub(1i32);
        loop {
            if lo >= hi { break; }
            let mut t: Object = a[lo as usize].clone();
            lo = lo.wrapping_add(1i32);
            a[lo as usize] = a[hi as usize].clone();
            hi = hi.wrapping_sub(1i32);
            a[hi as usize] = t;
        }
        Ok(())
    }

    // java: minRunLength(I)I
    pub fn minRunLength(n: i32) -> Result<i32> {
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut r: i32 = 0i32;
        loop {
            if n < 32i32 { break; }
            r = (r|(n&1i32));
            n = (n>>((1i32&0x1f)));
        }
        Ok((n).wrapping_add(r))
    }

    // java: pushRun(II)V
    pub fn pushRun(&self, runBase: i32, runLen: i32) -> Result<()> {
        let this = self;
        this.runBase.get()[this.stackSize.get() as usize] = runBase;
        this.runLen.get()[this.stackSize.get() as usize] = runLen;
        this.stackSize.set((this.stackSize.get()).wrapping_add(1i32));
        Ok(())
    }

    // java: mergeCollapse()V
    pub fn mergeCollapse(&self) -> Result<()> {
        let this = self;
        loop {
            if this.stackSize.get() <= 1i32 { break; }
            let mut n: i32 = (this.stackSize.get()).wrapping_sub(2i32);
            n = n.wrapping_sub(1i32);
            this.mergeAt(n)?;
        }
        Ok(())
    }

    // java: mergeForceCollapse()V
    pub fn mergeForceCollapse(&self) -> Result<()> {
        let this = self;
        loop {
            if this.stackSize.get() <= 1i32 { break; }
            let mut n: i32 = (this.stackSize.get()).wrapping_sub(2i32);
            n = n.wrapping_sub(1i32);
            this.mergeAt(n)?;
        }
        Ok(())
    }

    // java: mergeAt(I)V
    pub fn mergeAt(&self, i: i32) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut base1: i32 = this.runBase.get()[i as usize];
        let mut len1: i32 = this.runLen.get()[i as usize];
        let mut base2: i32 = this.runBase.get()[(i).wrapping_add(1i32) as usize];
        let mut len2: i32 = this.runLen.get()[(i).wrapping_add(1i32) as usize];
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        this.runLen.get()[i as usize] = (len1).wrapping_add(len2);
        this.runBase.get()[(i).wrapping_add(1i32) as usize] = this.runBase.get()[(i).wrapping_add(2i32) as usize];
        this.runLen.get()[(i).wrapping_add(1i32) as usize] = this.runLen.get()[(i).wrapping_add(2i32) as usize];
        this.stackSize.set((this.stackSize.get()).wrapping_sub(1i32));
        let _t0: i32 = TimSort::gallopRight(this.a.get()[base2 as usize].clone(), &this.a.get(), base1, len1, 0i32, this.c.get())?;
        let mut k: i32 = _t0;
        return Err(JvmError::Custom("athrow".to_owned()));
        base1 = (base1).wrapping_add(k);
        len1 = (len1).wrapping_sub(k);
        return Ok(());
        let _t1: i32 = TimSort::gallopLeft(this.a.get()[((base1).wrapping_add(len1)).wrapping_sub(1i32) as usize].clone(), &this.a.get(), base2, len2, (len2).wrapping_sub(1i32), this.c.get())?;
        len2 = _t1;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Ok(());
        this.mergeLo(base1, len1, base2, len2)?;
        this.mergeHi(base1, len1, base2, len2)?;
        Ok(())
    }

    // java: gallopLeft(Ljava/lang/Object;[Ljava/lang/Object;IIILjava/util/Comparator;)I
    pub fn gallopLeft(key: T, a: &[Object], base: i32, len: i32, hint: i32, c: Object) -> Result<i32> {
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut lastOfs: i32 = 0i32;
        let mut ofs: i32 = 1i32;
        let _t0 = c.compare(key, a[(base).wrapping_add(hint) as usize].clone())?;
        let mut maxOfs: i32 = (len).wrapping_sub(hint);
        loop {
            if ofs >= maxOfs { break; }
            let _t0 = c.compare(key, a[((base).wrapping_add(hint)).wrapping_add(ofs) as usize].clone())?;
            lastOfs = ofs;
            ofs = ((ofs<<(1i32&0x1f))).wrapping_add(1i32);
            ofs = maxOfs;
        }
        ofs = maxOfs;
        lastOfs = (lastOfs).wrapping_add(hint);
        ofs = (ofs).wrapping_add(hint);
        maxOfs = (hint).wrapping_add(1i32);
        loop {
            if ofs >= maxOfs { break; }
            let _t0 = c.compare(key, a[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize].clone())?;
            lastOfs = ofs;
            ofs = ((ofs<<(1i32&0x1f))).wrapping_add(1i32);
            ofs = maxOfs;
        }
        ofs = maxOfs;
        let mut tmp: i32 = lastOfs;
        lastOfs = (hint).wrapping_sub(ofs);
        ofs = (hint).wrapping_sub(tmp);
        return Err(JvmError::Custom("athrow".to_owned()));
        lastOfs = lastOfs.wrapping_add(1i32);
        loop {
            if lastOfs >= ofs { break; }
            maxOfs = (lastOfs).wrapping_add((((ofs).wrapping_sub(lastOfs) as u32>>(1i32&0x1f)) as i32));
            let _t0 = c.compare(key, a[(base).wrapping_add(maxOfs) as usize].clone())?;
            lastOfs = (maxOfs).wrapping_add(1i32);
            ofs = maxOfs;
        }
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(ofs)
    }

    // java: gallopRight(Ljava/lang/Object;[Ljava/lang/Object;IIILjava/util/Comparator;)I
    pub fn gallopRight(key: T, a: &[Object], base: i32, len: i32, hint: i32, c: Object) -> Result<i32> {
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut ofs: i32 = 1i32;
        let mut lastOfs: i32 = 0i32;
        let _t0 = c.compare(key, a[(base).wrapping_add(hint) as usize].clone())?;
        let mut maxOfs: i32 = (hint).wrapping_add(1i32);
        loop {
            if ofs >= maxOfs { break; }
            let _t0 = c.compare(key, a[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize].clone())?;
            lastOfs = ofs;
            ofs = ((ofs<<(1i32&0x1f))).wrapping_add(1i32);
            ofs = maxOfs;
        }
        ofs = maxOfs;
        let mut tmp: i32 = lastOfs;
        lastOfs = (hint).wrapping_sub(ofs);
        ofs = (hint).wrapping_sub(tmp);
        maxOfs = (len).wrapping_sub(hint);
        loop {
            if ofs >= maxOfs { break; }
            let _t0 = c.compare(key, a[((base).wrapping_add(hint)).wrapping_add(ofs) as usize].clone())?;
            lastOfs = ofs;
            ofs = ((ofs<<(1i32&0x1f))).wrapping_add(1i32);
            ofs = maxOfs;
        }
        ofs = maxOfs;
        lastOfs = (lastOfs).wrapping_add(hint);
        ofs = (ofs).wrapping_add(hint);
        return Err(JvmError::Custom("athrow".to_owned()));
        lastOfs = lastOfs.wrapping_add(1i32);
        loop {
            if lastOfs >= ofs { break; }
            maxOfs = (lastOfs).wrapping_add((((ofs).wrapping_sub(lastOfs) as u32>>(1i32&0x1f)) as i32));
            let _t0 = c.compare(key, a[(base).wrapping_add(maxOfs) as usize].clone())?;
            ofs = maxOfs;
            lastOfs = (maxOfs).wrapping_add(1i32);
        }
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(ofs)
    }

    // java: mergeLo(IIII)V
    pub fn mergeLo(&self, base1: i32, len1: i32, base2: i32, len2: i32) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut a: Vec<Object> = this.a.get();
        let _t0 = this.ensureCapacity(len1)?;
        let mut tmp: Vec<Object> = _t0;
        let mut cursor1: i32 = this.tmpBase.get();
        let mut cursor2: i32 = base2;
        let mut dest: i32 = base1;
        System::arraycopy(&a, base1, &tmp, cursor1, len1)?;
        dest = dest.wrapping_add(1i32);
        cursor2 = cursor2.wrapping_add(1i32);
        a[dest as usize] = a[cursor2 as usize].clone();
        len2 = len2.wrapping_sub(1i32);
        System::arraycopy(&tmp, cursor1, &a, dest, len1)?;
        return Ok(());
        System::arraycopy(&a, cursor2, &a, dest, len2)?;
        a[(dest).wrapping_add(len2) as usize] = tmp[cursor1 as usize].clone();
        return Ok(());
        let mut c: Object = this.c.get();
        let mut minGallop: i32 = this.minGallop.get();
        let mut count1: i32 = 0i32;
        let mut count2: i32 = 0i32;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = c.compare(a[cursor2 as usize].clone(), tmp[cursor1 as usize].clone())?;
        dest = dest.wrapping_add(1i32);
        cursor2 = cursor2.wrapping_add(1i32);
        a[dest as usize] = a[cursor2 as usize].clone();
        count2 = count2.wrapping_add(1i32);
        count1 = 0i32;
        len2 = len2.wrapping_sub(1i32);
        dest = dest.wrapping_add(1i32);
        cursor1 = cursor1.wrapping_add(1i32);
        a[dest as usize] = tmp[cursor1 as usize].clone();
        count1 = count1.wrapping_add(1i32);
        count2 = 0i32;
        len1 = len1.wrapping_sub(1i32);
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t2: i32 = TimSort::gallopRight(a[cursor2 as usize].clone(), &tmp, cursor1, len1, 0i32, c)?;
        count1 = _t2;
        System::arraycopy(&tmp, cursor1, &a, dest, count1)?;
        dest = (dest).wrapping_add(count1);
        cursor1 = (cursor1).wrapping_add(count1);
        len1 = (len1).wrapping_sub(count1);
        dest = dest.wrapping_add(1i32);
        cursor2 = cursor2.wrapping_add(1i32);
        a[dest as usize] = a[cursor2 as usize].clone();
        len2 = len2.wrapping_sub(1i32);
        let _t3: i32 = TimSort::gallopLeft(tmp[cursor1 as usize].clone(), &a, cursor2, len2, 0i32, c)?;
        count2 = _t3;
        System::arraycopy(&a, cursor2, &a, dest, count2)?;
        dest = (dest).wrapping_add(count2);
        cursor2 = (cursor2).wrapping_add(count2);
        len2 = (len2).wrapping_sub(count2);
        dest = dest.wrapping_add(1i32);
        cursor1 = cursor1.wrapping_add(1i32);
        a[dest as usize] = tmp[cursor1 as usize].clone();
        len1 = len1.wrapping_sub(1i32);
        minGallop = minGallop.wrapping_sub(1i32);
        minGallop = 0i32;
        minGallop = minGallop.wrapping_add(2i32);
        1i32.minGallop.set(minGallop);
        return Err(JvmError::Custom("athrow".to_owned()));
        System::arraycopy(&a, cursor2, &a, dest, len2)?;
        a[(dest).wrapping_add(len2) as usize] = tmp[cursor1 as usize].clone();
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        System::arraycopy(&tmp, cursor1, &a, dest, len1)?;
        Ok(())
    }

    // java: mergeHi(IIII)V
    pub fn mergeHi(&self, base1: i32, len1: i32, base2: i32, len2: i32) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut a: Vec<Object> = this.a.get();
        let _t0 = this.ensureCapacity(len2)?;
        let mut tmp: Vec<Object> = _t0;
        let mut tmpBase: i32 = this.tmpBase.get();
        System::arraycopy(&a, base2, &tmp, tmpBase, len2)?;
        let mut cursor1: i32 = ((base1).wrapping_add(len1)).wrapping_sub(1i32);
        let mut cursor2: i32 = ((tmpBase).wrapping_add(len2)).wrapping_sub(1i32);
        let mut dest: i32 = ((base2).wrapping_add(len2)).wrapping_sub(1i32);
        dest = dest.wrapping_sub(1i32);
        cursor1 = cursor1.wrapping_sub(1i32);
        a[dest as usize] = a[cursor1 as usize].clone();
        len1 = len1.wrapping_sub(1i32);
        System::arraycopy(&tmp, tmpBase, &a, (dest).wrapping_sub((len2).wrapping_sub(1i32)), len2)?;
        return Ok(());
        dest = (dest).wrapping_sub(len1);
        cursor1 = (cursor1).wrapping_sub(len1);
        System::arraycopy(&a, (cursor1).wrapping_add(1i32), &a, (dest).wrapping_add(1i32), len1)?;
        a[dest as usize] = tmp[cursor2 as usize].clone();
        return Ok(());
        let mut c: Object = this.c.get();
        let mut minGallop: i32 = this.minGallop.get();
        let mut count1: i32 = 0i32;
        let mut count2: i32 = 0i32;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = c.compare(tmp[cursor2 as usize].clone(), a[cursor1 as usize].clone())?;
        dest = dest.wrapping_sub(1i32);
        cursor1 = cursor1.wrapping_sub(1i32);
        a[dest as usize] = a[cursor1 as usize].clone();
        count1 = count1.wrapping_add(1i32);
        count2 = 0i32;
        len1 = len1.wrapping_sub(1i32);
        dest = dest.wrapping_sub(1i32);
        cursor2 = cursor2.wrapping_sub(1i32);
        a[dest as usize] = tmp[cursor2 as usize].clone();
        count2 = count2.wrapping_add(1i32);
        count1 = 0i32;
        len2 = len2.wrapping_sub(1i32);
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t2: i32 = TimSort::gallopRight(tmp[cursor2 as usize].clone(), &a, base1, len1, (len1).wrapping_sub(1i32), c)?;
        count1 = (len1).wrapping_sub(_t2);
        dest = (dest).wrapping_sub(count1);
        cursor1 = (cursor1).wrapping_sub(count1);
        len1 = (len1).wrapping_sub(count1);
        System::arraycopy(&a, (cursor1).wrapping_add(1i32), &a, (dest).wrapping_add(1i32), count1)?;
        dest = dest.wrapping_sub(1i32);
        cursor2 = cursor2.wrapping_sub(1i32);
        a[dest as usize] = tmp[cursor2 as usize].clone();
        len2 = len2.wrapping_sub(1i32);
        let _t3: i32 = TimSort::gallopLeft(a[cursor1 as usize].clone(), &tmp, tmpBase, len2, (len2).wrapping_sub(1i32), c)?;
        count2 = (len2).wrapping_sub(_t3);
        dest = (dest).wrapping_sub(count2);
        cursor2 = (cursor2).wrapping_sub(count2);
        len2 = (len2).wrapping_sub(count2);
        System::arraycopy(&tmp, (cursor2).wrapping_add(1i32), &a, (dest).wrapping_add(1i32), count2)?;
        dest = dest.wrapping_sub(1i32);
        cursor1 = cursor1.wrapping_sub(1i32);
        a[dest as usize] = a[cursor1 as usize].clone();
        len1 = len1.wrapping_sub(1i32);
        minGallop = minGallop.wrapping_sub(1i32);
        minGallop = 0i32;
        minGallop = minGallop.wrapping_add(2i32);
        1i32.minGallop.set(minGallop);
        return Err(JvmError::Custom("athrow".to_owned()));
        dest = (dest).wrapping_sub(len1);
        cursor1 = (cursor1).wrapping_sub(len1);
        System::arraycopy(&a, (cursor1).wrapping_add(1i32), &a, (dest).wrapping_add(1i32), len1)?;
        a[dest as usize] = tmp[cursor2 as usize].clone();
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        System::arraycopy(&tmp, tmpBase, &a, (dest).wrapping_sub((len2).wrapping_sub(1i32)), len2)?;
        Ok(())
    }

    // java: ensureCapacity(I)[Ljava/lang/Object;
    pub fn ensureCapacity(&self, minCapacity: i32) -> Result<Vec<Object>> {
        let this = self;
        let _t0: i32 = Integer::numberOfLeadingZeros(minCapacity)?;
        let mut newSize: i32 = ((-1i32 as u32>>(_t0&0x1f)) as i32);
        newSize = newSize.wrapping_add(1i32);
        newSize = minCapacity;
        let _t1: i32 = (newSize).min((((this.a.get().len() as i32) as u32>>(1i32&0x1f)) as i32));
        newSize = _t1;
        let _t2 = this.a.get().getClass()?;
        let _t3 = _t2.getComponentType()?;
        let _t4: Object = Array::newInstance(_t3, newSize)?;
        let mut newArray: Object = _t4;
        this.tmp.set(newArray);
        this.tmpLen.set(newSize);
        this.tmpBase.set(0i32);
        Ok(this.tmp.get())
    }
}
