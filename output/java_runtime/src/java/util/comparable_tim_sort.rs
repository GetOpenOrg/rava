#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::function::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/util/ComparableTimSort",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "package",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "ComparableTimSort.java",
    all_supertypes    = "java/lang/Object;java/util/ComparableTimSort",
)]
#[derive(Clone, Default, PartialEq)]
pub struct ComparableTimSort {
    #[cfg_attr(any(), java_field(name = "a", descriptor = "[Ljava/lang/Object;", access = "private", modifiers = "final", is_static = false))]
    pub a: JField<Rc<RefCell<Vec<Object>>>>,
    #[cfg_attr(any(), java_field(name = "minGallop", descriptor = "I", access = "private", modifiers = "", is_static = false))]
    pub minGallop: JField<i32>,
    #[cfg_attr(any(), java_field(name = "tmp", descriptor = "[Ljava/lang/Object;", access = "private", modifiers = "", is_static = false))]
    pub tmp: JField<Rc<RefCell<Vec<Object>>>>,
    #[cfg_attr(any(), java_field(name = "tmpBase", descriptor = "I", access = "private", modifiers = "", is_static = false))]
    pub tmpBase: JField<i32>,
    #[cfg_attr(any(), java_field(name = "tmpLen", descriptor = "I", access = "private", modifiers = "", is_static = false))]
    pub tmpLen: JField<i32>,
    #[cfg_attr(any(), java_field(name = "stackSize", descriptor = "I", access = "private", modifiers = "", is_static = false))]
    pub stackSize: JField<i32>,
    #[cfg_attr(any(), java_field(name = "runBase", descriptor = "[I", access = "private", modifiers = "final", is_static = false))]
    pub runBase: JField<Rc<RefCell<Vec<i32>>>>,
    #[cfg_attr(any(), java_field(name = "runLen", descriptor = "[I", access = "private", modifiers = "final", is_static = false))]
    pub runLen: JField<Rc<RefCell<Vec<i32>>>>,
}

impl ComparableTimSort {
    #[cfg_attr(any(), java_field(name = "MIN_MERGE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "32"))]
    // static field: MIN_MERGE:I
    pub fn MIN_MERGE() -> i32 {
        32
    }

    #[cfg_attr(any(), java_field(name = "MIN_GALLOP", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "7"))]
    // static field: MIN_GALLOP:I
    pub fn MIN_GALLOP() -> i32 {
        7
    }

    #[cfg_attr(any(), java_field(name = "INITIAL_TMP_STORAGE_LENGTH", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "256"))]
    // static field: INITIAL_TMP_STORAGE_LENGTH:I
    pub fn INITIAL_TMP_STORAGE_LENGTH() -> i32 {
        256
    }

    #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
    // static field: $assertionsDisabled:Z
    pub fn _assertionsDisabled() -> bool {
        false
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([Ljava/lang/Object;[Ljava/lang/Object;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new(mut a: Rc<RefCell<Vec<Object>>>, mut work: Rc<RefCell<Vec<Object>>>, mut workBase: i32, mut workLen: i32) -> Result<Self> {
        let mut this = Self { a: JField::new(Default::default()), minGallop: JField::new(0), tmp: JField::new(Default::default()), tmpBase: JField::new(0), tmpLen: JField::new(0), stackSize: JField::new(0), runBase: JField::new(Default::default()), runLen: JField::new(Default::default()), ..Default::default() };
        /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
        this.minGallop.set(7i32);
        this.stackSize.set(0i32);
        this.a.set(Clone::clone(&a));
        let mut len = (a.borrow().len() as i32);
        let mut tlen = (if len < 512i32 { ((len as u32>>(1i32&0x1f)) as i32) } else { 256i32 });
        if (workBase).wrapping_add(tlen) > (work.borrow().len() as i32) {
            let mut _arr0: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); tlen as usize]));
            this.tmp.set(Clone::clone(&_arr0));
            this.tmpBase.set(0i32);
            this.tmpLen.set(tlen);
        } else {
            this.tmp.set(Clone::clone(&work));
            this.tmpBase.set(workBase);
            this.tmpLen.set(workLen);
        }
        let mut stackLen = (if len < 120i32 { 5i32 } else { (if len < 1542i32 { 10i32 } else { (if len < 119151i32 { 24i32 } else { 49i32 }) }) });
        let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; stackLen as usize]));
        this.runBase.set(Clone::clone(&_arr0));
        let mut _arr1: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; stackLen as usize]));
        this.runLen.set(Clone::clone(&_arr1));
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "sort", descriptor = "([Ljava/lang/Object;II[Ljava/lang/Object;II)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn sort(mut a: Rc<RefCell<Vec<Object>>>, mut lo: i32, mut hi: i32, mut work: Rc<RefCell<Vec<Object>>>, mut workBase: i32, mut workLen: i32) -> Result<()> {
        if hi > (a.borrow().len() as i32) {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        let mut nRemaining = (hi).wrapping_sub(lo);
        if nRemaining < 2i32 {
            return Ok(());
        }
        if nRemaining < 32i32 {
            let _t0: i32 = ComparableTimSort::countRunAndMakeAscending(Clone::clone(&a), lo, hi)?;
            let mut initRunLen: i32 = _t0;
            ComparableTimSort::binarySort(Clone::clone(&a), lo, hi, (lo).wrapping_add(initRunLen))?;
            return Ok(());
        }
        let mut initRunLen = ComparableTimSort::new(Clone::clone(&a), Clone::clone(&work), workBase, workLen)?;
        let _t0: i32 = ComparableTimSort::minRunLength(nRemaining)?;
        let mut minRun: i32 = _t0;
        loop {
            let _t1: i32 = ComparableTimSort::countRunAndMakeAscending(Clone::clone(&a), lo, hi)?;
            let mut runLen: i32 = _t1;
            let mut force = (if nRemaining <= minRun { nRemaining } else { minRun });
            ComparableTimSort::binarySort(Clone::clone(&a), lo, (lo).wrapping_add(force), (lo).wrapping_add(runLen))?;
            runLen = force;
            initRunLen.pushRun(lo, runLen)?;
            initRunLen.mergeCollapse()?;
            lo = (lo).wrapping_add(runLen);
            nRemaining = (nRemaining).wrapping_sub(runLen);
            if (nRemaining==0) { break; }
        }
        if lo != hi {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        initRunLen.mergeForceCollapse()?;
        if initRunLen.stackSize.get() != 1i32 {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "binarySort", descriptor = "([Ljava/lang/Object;III)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn binarySort(mut a: Rc<RefCell<Vec<Object>>>, mut lo: i32, mut hi: i32, mut start: i32) -> Result<()> {
        if start > hi {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        if start == lo {
            start = start.wrapping_add(1i32);
        }
        loop {
            if start >= hi { break; }
            let mut pivot = Clone::clone(&a.borrow()[start as usize]);
            let mut left: i32 = lo;
            let mut right: i32 = start;
            if left > right {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut mid = Default::default();
            loop {
                if left >= right { break; }
                mid = (((left).wrapping_add(right) as u32>>(1i32&0x1f)) as i32);
                let _vdispatch0: i32 = if let Some(_d) = pivot.0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[mid as usize])))? } else if let Some(_d) = pivot.0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[mid as usize])))? } else if let Some(_d) = pivot.0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[mid as usize])))? } else if let Some(_d) = pivot.0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[mid as usize])))? } else if let Some(_d) = pivot.0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[mid as usize])))? } else if let Some(_d) = pivot.0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[mid as usize])))? } else if let Some(_d) = pivot.0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[mid as usize])))? } else if let Some(__f) = pivot.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&a.borrow()[mid as usize])))? } else { Default::default() };
                if (_vdispatch0<0) {
                    right = mid;
                } else {
                    left = (mid).wrapping_add(1i32);
                }
            }
            if left != right {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            mid = (start).wrapping_sub(left);
            match mid {
                1 => {
                    let _aastore_tmp0 = Clone::clone(&Clone::clone(&a.borrow()[left as usize]));
                    a.borrow_mut()[(left).wrapping_add(1i32) as usize] = _aastore_tmp0;
                }
                2 => {
                    let _aastore_tmp0 = Clone::clone(&Clone::clone(&a.borrow()[(left).wrapping_add(1i32) as usize]));
                    a.borrow_mut()[(left).wrapping_add(2i32) as usize] = _aastore_tmp0;
                    let _aastore_tmp1 = Clone::clone(&Clone::clone(&a.borrow()[left as usize]));
                    a.borrow_mut()[(left).wrapping_add(1i32) as usize] = _aastore_tmp1;
                }
                _ => {
                    System::arraycopy(Object::from_any(a.clone()), left, Object::from_any(a.clone()), (left).wrapping_add(1i32), mid)?;
                }
            }
            a.borrow_mut()[left as usize] = Clone::clone(&pivot);
            start = start.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "countRunAndMakeAscending", descriptor = "([Ljava/lang/Object;II)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn countRunAndMakeAscending(mut a: Rc<RefCell<Vec<Object>>>, mut lo: i32, mut hi: i32) -> Result<i32> {
        if lo >= hi {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        let mut runHi = (lo).wrapping_add(1i32);
        if runHi == hi {
            return Ok(1i32);
        }
        runHi = runHi.wrapping_add(1i32);
        let _vdispatch0: i32 = if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[lo as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[lo as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[lo as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[lo as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[lo as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[lo as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[lo as usize])))? } else if let Some(__f) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&a.borrow()[lo as usize])))? } else { Default::default() };
        if (_vdispatch0<0) {
            loop {
                if runHi >= hi { break; }
                let _vdispatch1: i32 = if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(runHi).wrapping_sub(1i32) as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(runHi).wrapping_sub(1i32) as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(runHi).wrapping_sub(1i32) as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(runHi).wrapping_sub(1i32) as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(runHi).wrapping_sub(1i32) as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(runHi).wrapping_sub(1i32) as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(runHi).wrapping_sub(1i32) as usize])))? } else if let Some(__f) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&a.borrow()[(runHi).wrapping_sub(1i32) as usize])))? } else { Default::default() };
                if (_vdispatch1<0) {
                    runHi = runHi.wrapping_add(1i32);
                    continue;
                }
                break;
            }
            ComparableTimSort::reverseRange(Clone::clone(&a), lo, runHi)?;
        } else {
            loop {
                if runHi >= hi { break; }
                let _vdispatch1: i32 = if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(runHi).wrapping_sub(1i32) as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(runHi).wrapping_sub(1i32) as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(runHi).wrapping_sub(1i32) as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(runHi).wrapping_sub(1i32) as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(runHi).wrapping_sub(1i32) as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(runHi).wrapping_sub(1i32) as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(runHi).wrapping_sub(1i32) as usize])))? } else if let Some(__f) = Clone::clone(&a.borrow()[runHi as usize]).0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&a.borrow()[(runHi).wrapping_sub(1i32) as usize])))? } else { Default::default() };
                if (_vdispatch1>=0) {
                    runHi = runHi.wrapping_add(1i32);
                    continue;
                }
                break;
            }
        }
        Ok((runHi).wrapping_sub(lo))
    }

    #[cfg_attr(any(), java_method(name = "reverseRange", descriptor = "([Ljava/lang/Object;II)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn reverseRange(mut a: Rc<RefCell<Vec<Object>>>, mut lo: i32, mut hi: i32) -> Result<()> {
        hi = hi.wrapping_sub(1i32);
        loop {
            if lo >= hi { break; }
            let mut t = Clone::clone(&a.borrow()[lo as usize]);
            lo = lo.wrapping_add(1i32);
            let _aastore_tmp0 = Clone::clone(&Clone::clone(&a.borrow()[hi as usize]));
            a.borrow_mut()[lo as usize] = _aastore_tmp0;
            hi = hi.wrapping_sub(1i32);
            a.borrow_mut()[hi as usize] = Clone::clone(&t);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "minRunLength", descriptor = "(I)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn minRunLength(mut n: i32) -> Result<i32> {
        if (n<0) {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        let mut r: i32 = 0i32;
        loop {
            if n < 32i32 { break; }
            r = (r|(n&1i32));
            n = (n>>((1i32&0x1f)));
        }
        Ok((n).wrapping_add(r))
    }

    #[cfg_attr(any(), java_method(name = "pushRun", descriptor = "(II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn pushRun(&self, mut runBase: i32, mut runLen: i32) -> Result<()> {
        let this = self;
        this.runBase.get().borrow_mut()[this.stackSize.get() as usize] = runBase;
        this.runLen.get().borrow_mut()[this.stackSize.get() as usize] = runLen;
        this.stackSize.set((this.stackSize.get()).wrapping_add(1i32));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "mergeCollapse", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mergeCollapse(&self) -> Result<()> {
        let this = self;
        loop {
            if this.stackSize.get() <= 1i32 { break; }
            let mut n = (this.stackSize.get()).wrapping_sub(2i32);
            if n > 1i32 {
                if this.runLen.get().borrow()[(n).wrapping_sub(2i32) as usize] <= (this.runLen.get().borrow()[n as usize]).wrapping_add(this.runLen.get().borrow()[(n).wrapping_sub(1i32) as usize]) {
                    if this.runLen.get().borrow()[(n).wrapping_sub(1i32) as usize] < this.runLen.get().borrow()[(n).wrapping_add(1i32) as usize] {
                        n = n.wrapping_sub(1i32);
                        if this.runLen.get().borrow()[n as usize] > this.runLen.get().borrow()[(n).wrapping_add(1i32) as usize] {
                            break;
                        }
                    } else {
                        this.mergeAt(n)?;
                        continue;
                    }
                } else {
                    if this.runLen.get().borrow()[n as usize] > this.runLen.get().borrow()[(n).wrapping_add(1i32) as usize] {
                        break;
                    }
                }
            } else {
                if this.runLen.get().borrow()[n as usize] > this.runLen.get().borrow()[(n).wrapping_add(1i32) as usize] {
                    break;
                }
            }
            this.mergeAt(n)?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "mergeForceCollapse", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mergeForceCollapse(&self) -> Result<()> {
        let this = self;
        loop {
            if this.stackSize.get() <= 1i32 { break; }
            let mut n = (this.stackSize.get()).wrapping_sub(2i32);
            if this.runLen.get().borrow()[(n).wrapping_sub(1i32) as usize] < this.runLen.get().borrow()[(n).wrapping_add(1i32) as usize] {
                n = n.wrapping_sub(1i32);
            }
            this.mergeAt(n)?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "mergeAt", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mergeAt(&self, mut i: i32) -> Result<()> {
        let this = self;
        if this.stackSize.get() < 2i32 {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        if (i<0) {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        if i != (this.stackSize.get()).wrapping_sub(3i32) {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        let mut base1 = this.runBase.get().borrow()[i as usize];
        let mut len1 = this.runLen.get().borrow()[i as usize];
        let mut base2 = this.runBase.get().borrow()[(i).wrapping_add(1i32) as usize];
        let mut len2 = this.runLen.get().borrow()[(i).wrapping_add(1i32) as usize];
        if (len2<=0) {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        if (base1).wrapping_add(len1) != base2 {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        this.runLen.get().borrow_mut()[i as usize] = (len1).wrapping_add(len2);
        if i == (this.stackSize.get()).wrapping_sub(3i32) {
            let _tmp_val0 = this.runBase.get().borrow()[(i).wrapping_add(2i32) as usize];
            this.runBase.get().borrow_mut()[(i).wrapping_add(1i32) as usize] = _tmp_val0;
            let _tmp_val1 = this.runLen.get().borrow()[(i).wrapping_add(2i32) as usize];
            this.runLen.get().borrow_mut()[(i).wrapping_add(1i32) as usize] = _tmp_val1;
        }
        this.stackSize.set((this.stackSize.get()).wrapping_sub(1i32));
        let _t0: i32 = ComparableTimSort::gallopRight(Clone::clone(&Clone::clone(&this.a.get().borrow()[base2 as usize])), Clone::clone(&this.a.get()), base1, len1, 0i32)?;
        let mut k: i32 = _t0;
        if (k<0) {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        base1 = (base1).wrapping_add(k);
        len1 = (len1).wrapping_sub(k);
        if (len1==0) {
            return Ok(());
        }
        let _t1: i32 = ComparableTimSort::gallopLeft(Clone::clone(&Clone::clone(&this.a.get().borrow()[((base1).wrapping_add(len1)).wrapping_sub(1i32) as usize])), Clone::clone(&this.a.get()), base2, len2, (len2).wrapping_sub(1i32))?;
        len2 = _t1;
        if (len2<0) {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        if (len2==0) {
            return Ok(());
        }
        if len1 <= len2 {
            this.mergeLo(base1, len1, base2, len2)?;
        } else {
            this.mergeHi(base1, len1, base2, len2)?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "gallopLeft", descriptor = "(Ljava/lang/Comparable;[Ljava/lang/Object;III)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Comparable<Ljava/lang/Object;>;[Ljava/lang/Object;III)I"))]
    pub fn gallopLeft(mut key: Object, mut a: Rc<RefCell<Vec<Object>>>, mut base: i32, mut len: i32, mut hint: i32) -> Result<i32> {
        if hint >= len {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        let mut lastOfs: i32 = 0i32;
        let mut ofs: i32 = 1i32;
        let _vdispatch0: i32 = if let Some(_d) = key.0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(hint) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(hint) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(hint) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(hint) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(hint) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(hint) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(hint) as usize])))? } else if let Some(__f) = key.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(hint) as usize])))? } else { Default::default() };
    let mut maxOfs = Default::default();
        if (_vdispatch0>0) {
            maxOfs = (len).wrapping_sub(hint);
            loop {
                if ofs >= maxOfs { break; }
                let _vdispatch1: i32 = if let Some(_d) = key.0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_add(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_add(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_add(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_add(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_add(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_add(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_add(ofs) as usize])))? } else if let Some(__f) = key.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_add(ofs) as usize])))? } else { Default::default() };
                lastOfs = ofs;
                ofs = ((ofs<<(1i32&0x1f))).wrapping_add(1i32);
                ofs = maxOfs;
            }
            if ofs > maxOfs {
                ofs = maxOfs;
            }
            lastOfs = (lastOfs).wrapping_add(hint);
            ofs = (ofs).wrapping_add(hint);
        } else {
            maxOfs = (hint).wrapping_add(1i32);
            loop {
                if ofs >= maxOfs { break; }
                let _vdispatch1: i32 = if let Some(_d) = key.0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize])))? } else if let Some(__f) = key.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize])))? } else { Default::default() };
                lastOfs = ofs;
                ofs = ((ofs<<(1i32&0x1f))).wrapping_add(1i32);
                ofs = maxOfs;
            }
            if ofs > maxOfs {
                ofs = maxOfs;
            }
            let mut tmp: i32 = lastOfs;
            lastOfs = (hint).wrapping_sub(ofs);
            ofs = (hint).wrapping_sub(tmp);
        }
        if ofs > len {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        lastOfs = lastOfs.wrapping_add(1i32);
        loop {
            if lastOfs >= ofs { break; }
            maxOfs = (lastOfs).wrapping_add((((ofs).wrapping_sub(lastOfs) as u32>>(1i32&0x1f)) as i32));
            let _vdispatch1: i32 = if let Some(_d) = key.0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(maxOfs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(maxOfs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(maxOfs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(maxOfs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(maxOfs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(maxOfs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(maxOfs) as usize])))? } else if let Some(__f) = key.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(maxOfs) as usize])))? } else { Default::default() };
            if (_vdispatch1>0) {
                lastOfs = (maxOfs).wrapping_add(1i32);
            } else {
                ofs = maxOfs;
            }
        }
        if lastOfs != ofs {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        Ok(ofs)
    }

    #[cfg_attr(any(), java_method(name = "gallopRight", descriptor = "(Ljava/lang/Comparable;[Ljava/lang/Object;III)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Comparable<Ljava/lang/Object;>;[Ljava/lang/Object;III)I"))]
    pub fn gallopRight(mut key: Object, mut a: Rc<RefCell<Vec<Object>>>, mut base: i32, mut len: i32, mut hint: i32) -> Result<i32> {
        if hint >= len {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        let mut ofs: i32 = 1i32;
        let mut lastOfs: i32 = 0i32;
        let _vdispatch0: i32 = if let Some(_d) = key.0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(hint) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(hint) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(hint) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(hint) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(hint) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(hint) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(hint) as usize])))? } else if let Some(__f) = key.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(hint) as usize])))? } else { Default::default() };
    let mut maxOfs = Default::default();
        if (_vdispatch0<0) {
            maxOfs = (hint).wrapping_add(1i32);
            loop {
                if ofs >= maxOfs { break; }
                let _vdispatch1: i32 = if let Some(_d) = key.0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize])))? } else if let Some(__f) = key.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_sub(ofs) as usize])))? } else { Default::default() };
                lastOfs = ofs;
                ofs = ((ofs<<(1i32&0x1f))).wrapping_add(1i32);
                ofs = maxOfs;
            }
            if ofs > maxOfs {
                ofs = maxOfs;
            }
            let mut tmp: i32 = lastOfs;
            lastOfs = (hint).wrapping_sub(ofs);
            ofs = (hint).wrapping_sub(tmp);
        } else {
            maxOfs = (len).wrapping_sub(hint);
            loop {
                if ofs >= maxOfs { break; }
                let _vdispatch1: i32 = if let Some(_d) = key.0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_add(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_add(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_add(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_add(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_add(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_add(ofs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_add(ofs) as usize])))? } else if let Some(__f) = key.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&a.borrow()[((base).wrapping_add(hint)).wrapping_add(ofs) as usize])))? } else { Default::default() };
                lastOfs = ofs;
                ofs = ((ofs<<(1i32&0x1f))).wrapping_add(1i32);
                ofs = maxOfs;
            }
            if ofs > maxOfs {
                ofs = maxOfs;
            }
            lastOfs = (lastOfs).wrapping_add(hint);
            ofs = (ofs).wrapping_add(hint);
        }
        if ofs > len {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        lastOfs = lastOfs.wrapping_add(1i32);
        loop {
            if lastOfs >= ofs { break; }
            maxOfs = (lastOfs).wrapping_add((((ofs).wrapping_sub(lastOfs) as u32>>(1i32&0x1f)) as i32));
            let _vdispatch1: i32 = if let Some(_d) = key.0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(maxOfs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(maxOfs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(maxOfs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(maxOfs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(maxOfs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(maxOfs) as usize])))? } else if let Some(_d) = key.0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(maxOfs) as usize])))? } else if let Some(__f) = key.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&a.borrow()[(base).wrapping_add(maxOfs) as usize])))? } else { Default::default() };
            if (_vdispatch1<0) {
                ofs = maxOfs;
            } else {
                lastOfs = (maxOfs).wrapping_add(1i32);
            }
        }
        if lastOfs != ofs {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        Ok(ofs)
    }

    #[cfg_attr(any(), java_method(name = "mergeLo", descriptor = "(IIII)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mergeLo(&self, mut base1: i32, mut len1: i32, mut base2: i32, mut len2: i32) -> Result<()> {
        let this = self;
        if (base1).wrapping_add(len1) != base2 {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        let mut a = this.a.get();
        let _t0 = this.ensureCapacity(len1)?;
        let mut tmp: Rc<RefCell<Vec<Object>>> = _t0;
        let mut cursor1 = this.tmpBase.get();
        let mut cursor2: i32 = base2;
        let mut dest: i32 = base1;
        System::arraycopy(Object::from_any(a.clone()), base1, Object::from_any(tmp.clone()), cursor1, len1)?;
        dest = dest.wrapping_add(1i32);
        cursor2 = cursor2.wrapping_add(1i32);
        let _aastore_tmp1 = Clone::clone(&Clone::clone(&a.borrow()[cursor2 as usize]));
        a.borrow_mut()[dest as usize] = _aastore_tmp1;
        len2 = len2.wrapping_sub(1i32);
        if (len2==0) {
            System::arraycopy(Object::from_any(tmp.clone()), cursor1, Object::from_any(a.clone()), dest, len1)?;
            return Ok(());
        }
        if len1 == 1i32 {
            System::arraycopy(Object::from_any(a.clone()), cursor2, Object::from_any(a.clone()), dest, len2)?;
            let _aastore_tmp2 = Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize]));
            a.borrow_mut()[(dest).wrapping_add(len2) as usize] = _aastore_tmp2;
            return Ok(());
        }
        let mut minGallop = this.minGallop.get();
        loop {
            let mut count1: i32 = 0i32;
            let mut count2: i32 = 0i32;
            loop {
                if (len2<=0) {
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                let _vdispatch2: i32 = if let Some(_d) = Clone::clone(&a.borrow()[cursor2 as usize]).0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[cursor2 as usize]).0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[cursor2 as usize]).0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[cursor2 as usize]).0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[cursor2 as usize]).0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[cursor2 as usize]).0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize])))? } else if let Some(_d) = Clone::clone(&a.borrow()[cursor2 as usize]).0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize])))? } else if let Some(__f) = Clone::clone(&a.borrow()[cursor2 as usize]).0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize])))? } else { Default::default() };
                if (_vdispatch2<0) {
                    dest = dest.wrapping_add(1i32);
                    cursor2 = cursor2.wrapping_add(1i32);
                    let _aastore_tmp3 = Clone::clone(&Clone::clone(&a.borrow()[cursor2 as usize]));
                    a.borrow_mut()[dest as usize] = _aastore_tmp3;
                    count2 = count2.wrapping_add(1i32);
                    count1 = 0i32;
                    len2 = len2.wrapping_sub(1i32);
                    if (len2==0) {
                        break;
                        dest = dest.wrapping_add(1i32);
                        cursor1 = cursor1.wrapping_add(1i32);
                        let _aastore_tmp4 = Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize]));
                        a.borrow_mut()[dest as usize] = _aastore_tmp4;
                        count1 = count1.wrapping_add(1i32);
                        count2 = 0i32;
                        len1 = len1.wrapping_sub(1i32);
                        if len1 == 1i32 {
                            break;
                        }
                    } else {
                        loop {
                            if (len2<=0) {
                                return Err(JvmError::Custom("athrow".to_owned()));
                            }
                            let _t4: i32 = ComparableTimSort::gallopRight(Clone::clone(&Clone::clone(&a.borrow()[cursor2 as usize])), Clone::clone(&tmp), cursor1, len1, 0i32)?;
                            count1 = _t4;
                            if (count1!=0) {
                                System::arraycopy(Object::from_any(tmp.clone()), cursor1, Object::from_any(a.clone()), dest, count1)?;
                                dest = (dest).wrapping_add(count1);
                                cursor1 = (cursor1).wrapping_add(count1);
                                len1 = (len1).wrapping_sub(count1);
                                if len1 <= 1i32 {
                                    break;
                                }
                            } else {
                                dest = dest.wrapping_add(1i32);
                                cursor2 = cursor2.wrapping_add(1i32);
                                let _aastore_tmp5 = Clone::clone(&Clone::clone(&a.borrow()[cursor2 as usize]));
                                a.borrow_mut()[dest as usize] = _aastore_tmp5;
                                len2 = len2.wrapping_sub(1i32);
                                if (len2==0) {
                                    break;
                                }
                                let _t6: i32 = ComparableTimSort::gallopLeft(Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize])), Clone::clone(&a), cursor2, len2, 0i32)?;
                                count2 = _t6;
                                if (count2!=0) {
                                    System::arraycopy(Object::from_any(a.clone()), cursor2, Object::from_any(a.clone()), dest, count2)?;
                                    dest = (dest).wrapping_add(count2);
                                    cursor2 = (cursor2).wrapping_add(count2);
                                    len2 = (len2).wrapping_sub(count2);
                                    if (len2==0) {
                                        break;
                                    }
                                } else {
                                    dest = dest.wrapping_add(1i32);
                                    cursor1 = cursor1.wrapping_add(1i32);
                                    let _aastore_tmp7 = Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize]));
                                    a.borrow_mut()[dest as usize] = _aastore_tmp7;
                                    len1 = len1.wrapping_sub(1i32);
                                    if len1 == 1i32 {
                                        break;
                                    }
                                    minGallop = minGallop.wrapping_sub(1i32);
                                    if (minGallop<0) {
                                        minGallop = 0i32;
                                    }
                                    minGallop = minGallop.wrapping_add(2i32);
                                    continue;
                                }
                            }
                            if ((panic!("stack underflow") as i32)==0) { break; }
                        }
                        if (minGallop<0) {
                            minGallop = 0i32;
                        }
                        minGallop = minGallop.wrapping_add(2i32);
                        continue;
                    }
                } else {
                    dest = dest.wrapping_add(1i32);
                    cursor1 = cursor1.wrapping_add(1i32);
                    let _aastore_tmp3 = Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize]));
                    a.borrow_mut()[dest as usize] = _aastore_tmp3;
                    count1 = count1.wrapping_add(1i32);
                    count2 = 0i32;
                    len1 = len1.wrapping_sub(1i32);
                    if len1 == 1i32 {
                        break;
                    }
                    loop {
                        if (len2<=0) {
                            return Err(JvmError::Custom("athrow".to_owned()));
                        }
                        let _t4: i32 = ComparableTimSort::gallopRight(Clone::clone(&Clone::clone(&a.borrow()[cursor2 as usize])), Clone::clone(&tmp), cursor1, len1, 0i32)?;
                        count1 = _t4;
                        if (count1!=0) {
                            System::arraycopy(Object::from_any(tmp.clone()), cursor1, Object::from_any(a.clone()), dest, count1)?;
                            dest = (dest).wrapping_add(count1);
                            cursor1 = (cursor1).wrapping_add(count1);
                            len1 = (len1).wrapping_sub(count1);
                            if len1 <= 1i32 {
                                break;
                            }
                        } else {
                            dest = dest.wrapping_add(1i32);
                            cursor2 = cursor2.wrapping_add(1i32);
                            let _aastore_tmp5 = Clone::clone(&Clone::clone(&a.borrow()[cursor2 as usize]));
                            a.borrow_mut()[dest as usize] = _aastore_tmp5;
                            len2 = len2.wrapping_sub(1i32);
                            if (len2==0) {
                                break;
                            }
                            let _t6: i32 = ComparableTimSort::gallopLeft(Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize])), Clone::clone(&a), cursor2, len2, 0i32)?;
                            count2 = _t6;
                            if (count2!=0) {
                                System::arraycopy(Object::from_any(a.clone()), cursor2, Object::from_any(a.clone()), dest, count2)?;
                                dest = (dest).wrapping_add(count2);
                                cursor2 = (cursor2).wrapping_add(count2);
                                len2 = (len2).wrapping_sub(count2);
                                if (len2==0) {
                                    break;
                                }
                            } else {
                                dest = dest.wrapping_add(1i32);
                                cursor1 = cursor1.wrapping_add(1i32);
                                let _aastore_tmp7 = Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize]));
                                a.borrow_mut()[dest as usize] = _aastore_tmp7;
                                len1 = len1.wrapping_sub(1i32);
                                if len1 == 1i32 {
                                    break;
                                }
                                minGallop = minGallop.wrapping_sub(1i32);
                                if (minGallop<0) {
                                    minGallop = 0i32;
                                }
                                minGallop = minGallop.wrapping_add(2i32);
                                continue;
                            }
                        }
                        if ((panic!("stack underflow") as i32)==0) { break; }
                    }
                    if (minGallop<0) {
                        minGallop = 0i32;
                    }
                    minGallop = minGallop.wrapping_add(2i32);
                    continue;
                }
                if (panic!("stack underflow") as i32) >= (panic!("stack underflow") as i32) { break; }
            }
            loop {
                if (len2<=0) {
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                let _t2: i32 = ComparableTimSort::gallopRight(Clone::clone(&Clone::clone(&a.borrow()[cursor2 as usize])), Clone::clone(&tmp), cursor1, len1, 0i32)?;
                count1 = _t2;
                if (count1!=0) {
                    System::arraycopy(Object::from_any(tmp.clone()), cursor1, Object::from_any(a.clone()), dest, count1)?;
                    dest = (dest).wrapping_add(count1);
                    cursor1 = (cursor1).wrapping_add(count1);
                    len1 = (len1).wrapping_sub(count1);
                    if len1 <= 1i32 {
                        break;
                    }
                } else {
                    dest = dest.wrapping_add(1i32);
                    cursor2 = cursor2.wrapping_add(1i32);
                    let _aastore_tmp3 = Clone::clone(&Clone::clone(&a.borrow()[cursor2 as usize]));
                    a.borrow_mut()[dest as usize] = _aastore_tmp3;
                    len2 = len2.wrapping_sub(1i32);
                    if (len2==0) {
                        break;
                    }
                    let _t4: i32 = ComparableTimSort::gallopLeft(Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize])), Clone::clone(&a), cursor2, len2, 0i32)?;
                    count2 = _t4;
                    if (count2!=0) {
                        System::arraycopy(Object::from_any(a.clone()), cursor2, Object::from_any(a.clone()), dest, count2)?;
                        dest = (dest).wrapping_add(count2);
                        cursor2 = (cursor2).wrapping_add(count2);
                        len2 = (len2).wrapping_sub(count2);
                        if (len2==0) {
                            break;
                        }
                    } else {
                        dest = dest.wrapping_add(1i32);
                        cursor1 = cursor1.wrapping_add(1i32);
                        let _aastore_tmp5 = Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize]));
                        a.borrow_mut()[dest as usize] = _aastore_tmp5;
                        len1 = len1.wrapping_sub(1i32);
                        if len1 == 1i32 {
                            break;
                        }
                        minGallop = minGallop.wrapping_sub(1i32);
                        if (minGallop<0) {
                            minGallop = 0i32;
                        }
                        minGallop = minGallop.wrapping_add(2i32);
                        continue;
                    }
                }
                if ((panic!("stack underflow") as i32)==0) { break; }
            }
            if (minGallop<0) {
                minGallop = 0i32;
            }
            minGallop = minGallop.wrapping_add(2i32);
        }
        this.minGallop.set((if minGallop < 1i32 { 1i32 } else { minGallop }));
        if len1 == 1i32 {
            if (len2<=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            System::arraycopy(Object::from_any(a.clone()), cursor2, Object::from_any(a.clone()), dest, len2)?;
            let _aastore_tmp2 = Clone::clone(&Clone::clone(&tmp.borrow()[cursor1 as usize]));
            a.borrow_mut()[(dest).wrapping_add(len2) as usize] = _aastore_tmp2;
        } else {
            if (len1==0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (len2!=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if len1 <= 1i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            System::arraycopy(Object::from_any(tmp.clone()), cursor1, Object::from_any(a.clone()), dest, len1)?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "mergeHi", descriptor = "(IIII)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mergeHi(&self, mut base1: i32, mut len1: i32, mut base2: i32, mut len2: i32) -> Result<()> {
        let this = self;
        if (base1).wrapping_add(len1) != base2 {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        let mut a = this.a.get();
        let _t0 = this.ensureCapacity(len2)?;
        let mut tmp: Rc<RefCell<Vec<Object>>> = _t0;
        let mut tmpBase = this.tmpBase.get();
        System::arraycopy(Object::from_any(a.clone()), base2, Object::from_any(tmp.clone()), tmpBase, len2)?;
        let mut cursor1 = ((base1).wrapping_add(len1)).wrapping_sub(1i32);
        let mut cursor2 = ((tmpBase).wrapping_add(len2)).wrapping_sub(1i32);
        let mut dest = ((base2).wrapping_add(len2)).wrapping_sub(1i32);
        dest = dest.wrapping_sub(1i32);
        cursor1 = cursor1.wrapping_sub(1i32);
        let _aastore_tmp1 = Clone::clone(&Clone::clone(&a.borrow()[cursor1 as usize]));
        a.borrow_mut()[dest as usize] = _aastore_tmp1;
        len1 = len1.wrapping_sub(1i32);
        if (len1==0) {
            System::arraycopy(Object::from_any(tmp.clone()), tmpBase, Object::from_any(a.clone()), (dest).wrapping_sub((len2).wrapping_sub(1i32)), len2)?;
            return Ok(());
        }
        if len2 == 1i32 {
            dest = (dest).wrapping_sub(len1);
            cursor1 = (cursor1).wrapping_sub(len1);
            System::arraycopy(Object::from_any(a.clone()), (cursor1).wrapping_add(1i32), Object::from_any(a.clone()), (dest).wrapping_add(1i32), len1)?;
            let _aastore_tmp2 = Clone::clone(&Clone::clone(&tmp.borrow()[cursor2 as usize]));
            a.borrow_mut()[dest as usize] = _aastore_tmp2;
            return Ok(());
        }
        let mut minGallop = this.minGallop.get();
        loop {
            let mut count1: i32 = 0i32;
            let mut count2: i32 = 0i32;
            loop {
                if len2 <= 1i32 {
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                let _vdispatch2: i32 = if let Some(_d) = Clone::clone(&tmp.borrow()[cursor2 as usize]).0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[cursor1 as usize])))? } else if let Some(_d) = Clone::clone(&tmp.borrow()[cursor2 as usize]).0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[cursor1 as usize])))? } else if let Some(_d) = Clone::clone(&tmp.borrow()[cursor2 as usize]).0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[cursor1 as usize])))? } else if let Some(_d) = Clone::clone(&tmp.borrow()[cursor2 as usize]).0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[cursor1 as usize])))? } else if let Some(_d) = Clone::clone(&tmp.borrow()[cursor2 as usize]).0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[cursor1 as usize])))? } else if let Some(_d) = Clone::clone(&tmp.borrow()[cursor2 as usize]).0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[cursor1 as usize])))? } else if let Some(_d) = Clone::clone(&tmp.borrow()[cursor2 as usize]).0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&a.borrow()[cursor1 as usize])))? } else if let Some(__f) = Clone::clone(&tmp.borrow()[cursor2 as usize]).0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&a.borrow()[cursor1 as usize])))? } else { Default::default() };
                if (_vdispatch2<0) {
                    dest = dest.wrapping_sub(1i32);
                    cursor1 = cursor1.wrapping_sub(1i32);
                    let _aastore_tmp3 = Clone::clone(&Clone::clone(&a.borrow()[cursor1 as usize]));
                    a.borrow_mut()[dest as usize] = _aastore_tmp3;
                    count1 = count1.wrapping_add(1i32);
                    count2 = 0i32;
                    len1 = len1.wrapping_sub(1i32);
                    if (len1==0) {
                        break;
                        dest = dest.wrapping_sub(1i32);
                        cursor2 = cursor2.wrapping_sub(1i32);
                        let _aastore_tmp4 = Clone::clone(&Clone::clone(&tmp.borrow()[cursor2 as usize]));
                        a.borrow_mut()[dest as usize] = _aastore_tmp4;
                        count2 = count2.wrapping_add(1i32);
                        count1 = 0i32;
                        len2 = len2.wrapping_sub(1i32);
                        if len2 == 1i32 {
                            break;
                        }
                    } else {
                        loop {
                            if len2 <= 1i32 {
                                return Err(JvmError::Custom("athrow".to_owned()));
                            }
                            let _t4: i32 = ComparableTimSort::gallopRight(Clone::clone(&Clone::clone(&tmp.borrow()[cursor2 as usize])), Clone::clone(&a), base1, len1, (len1).wrapping_sub(1i32))?;
                            count1 = (len1).wrapping_sub(_t4);
                            if (count1!=0) {
                                dest = (dest).wrapping_sub(count1);
                                cursor1 = (cursor1).wrapping_sub(count1);
                                len1 = (len1).wrapping_sub(count1);
                                System::arraycopy(Object::from_any(a.clone()), (cursor1).wrapping_add(1i32), Object::from_any(a.clone()), (dest).wrapping_add(1i32), count1)?;
                                if (len1==0) {
                                    break;
                                }
                            } else {
                                dest = dest.wrapping_sub(1i32);
                                cursor2 = cursor2.wrapping_sub(1i32);
                                let _aastore_tmp5 = Clone::clone(&Clone::clone(&tmp.borrow()[cursor2 as usize]));
                                a.borrow_mut()[dest as usize] = _aastore_tmp5;
                                len2 = len2.wrapping_sub(1i32);
                                if len2 == 1i32 {
                                    break;
                                }
                                let _t6: i32 = ComparableTimSort::gallopLeft(Clone::clone(&Clone::clone(&a.borrow()[cursor1 as usize])), Clone::clone(&tmp), tmpBase, len2, (len2).wrapping_sub(1i32))?;
                                count2 = (len2).wrapping_sub(_t6);
                                if (count2!=0) {
                                    dest = (dest).wrapping_sub(count2);
                                    cursor2 = (cursor2).wrapping_sub(count2);
                                    len2 = (len2).wrapping_sub(count2);
                                    System::arraycopy(Object::from_any(tmp.clone()), (cursor2).wrapping_add(1i32), Object::from_any(a.clone()), (dest).wrapping_add(1i32), count2)?;
                                    if len2 <= 1i32 {
                                        break;
                                    }
                                } else {
                                    dest = dest.wrapping_sub(1i32);
                                    cursor1 = cursor1.wrapping_sub(1i32);
                                    let _aastore_tmp7 = Clone::clone(&Clone::clone(&a.borrow()[cursor1 as usize]));
                                    a.borrow_mut()[dest as usize] = _aastore_tmp7;
                                    len1 = len1.wrapping_sub(1i32);
                                    if (len1==0) {
                                        break;
                                    }
                                    minGallop = minGallop.wrapping_sub(1i32);
                                    if (minGallop<0) {
                                        minGallop = 0i32;
                                    }
                                    minGallop = minGallop.wrapping_add(2i32);
                                    continue;
                                }
                            }
                            if ((panic!("stack underflow") as i32)==0) { break; }
                        }
                        if (minGallop<0) {
                            minGallop = 0i32;
                        }
                        minGallop = minGallop.wrapping_add(2i32);
                        continue;
                    }
                } else {
                    dest = dest.wrapping_sub(1i32);
                    cursor2 = cursor2.wrapping_sub(1i32);
                    let _aastore_tmp3 = Clone::clone(&Clone::clone(&tmp.borrow()[cursor2 as usize]));
                    a.borrow_mut()[dest as usize] = _aastore_tmp3;
                    count2 = count2.wrapping_add(1i32);
                    count1 = 0i32;
                    len2 = len2.wrapping_sub(1i32);
                    if len2 == 1i32 {
                        break;
                    }
                    loop {
                        if len2 <= 1i32 {
                            return Err(JvmError::Custom("athrow".to_owned()));
                        }
                        let _t4: i32 = ComparableTimSort::gallopRight(Clone::clone(&Clone::clone(&tmp.borrow()[cursor2 as usize])), Clone::clone(&a), base1, len1, (len1).wrapping_sub(1i32))?;
                        count1 = (len1).wrapping_sub(_t4);
                        if (count1!=0) {
                            dest = (dest).wrapping_sub(count1);
                            cursor1 = (cursor1).wrapping_sub(count1);
                            len1 = (len1).wrapping_sub(count1);
                            System::arraycopy(Object::from_any(a.clone()), (cursor1).wrapping_add(1i32), Object::from_any(a.clone()), (dest).wrapping_add(1i32), count1)?;
                            if (len1==0) {
                                break;
                            }
                        } else {
                            dest = dest.wrapping_sub(1i32);
                            cursor2 = cursor2.wrapping_sub(1i32);
                            let _aastore_tmp5 = Clone::clone(&Clone::clone(&tmp.borrow()[cursor2 as usize]));
                            a.borrow_mut()[dest as usize] = _aastore_tmp5;
                            len2 = len2.wrapping_sub(1i32);
                            if len2 == 1i32 {
                                break;
                            }
                            let _t6: i32 = ComparableTimSort::gallopLeft(Clone::clone(&Clone::clone(&a.borrow()[cursor1 as usize])), Clone::clone(&tmp), tmpBase, len2, (len2).wrapping_sub(1i32))?;
                            count2 = (len2).wrapping_sub(_t6);
                            if (count2!=0) {
                                dest = (dest).wrapping_sub(count2);
                                cursor2 = (cursor2).wrapping_sub(count2);
                                len2 = (len2).wrapping_sub(count2);
                                System::arraycopy(Object::from_any(tmp.clone()), (cursor2).wrapping_add(1i32), Object::from_any(a.clone()), (dest).wrapping_add(1i32), count2)?;
                                if len2 <= 1i32 {
                                    break;
                                }
                            } else {
                                dest = dest.wrapping_sub(1i32);
                                cursor1 = cursor1.wrapping_sub(1i32);
                                let _aastore_tmp7 = Clone::clone(&Clone::clone(&a.borrow()[cursor1 as usize]));
                                a.borrow_mut()[dest as usize] = _aastore_tmp7;
                                len1 = len1.wrapping_sub(1i32);
                                if (len1==0) {
                                    break;
                                }
                                minGallop = minGallop.wrapping_sub(1i32);
                                if (minGallop<0) {
                                    minGallop = 0i32;
                                }
                                minGallop = minGallop.wrapping_add(2i32);
                                continue;
                            }
                        }
                        if ((panic!("stack underflow") as i32)==0) { break; }
                    }
                    if (minGallop<0) {
                        minGallop = 0i32;
                    }
                    minGallop = minGallop.wrapping_add(2i32);
                    continue;
                }
                if (panic!("stack underflow") as i32) >= (panic!("stack underflow") as i32) { break; }
            }
            loop {
                if len2 <= 1i32 {
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                let _t2: i32 = ComparableTimSort::gallopRight(Clone::clone(&Clone::clone(&tmp.borrow()[cursor2 as usize])), Clone::clone(&a), base1, len1, (len1).wrapping_sub(1i32))?;
                count1 = (len1).wrapping_sub(_t2);
                if (count1!=0) {
                    dest = (dest).wrapping_sub(count1);
                    cursor1 = (cursor1).wrapping_sub(count1);
                    len1 = (len1).wrapping_sub(count1);
                    System::arraycopy(Object::from_any(a.clone()), (cursor1).wrapping_add(1i32), Object::from_any(a.clone()), (dest).wrapping_add(1i32), count1)?;
                    if (len1==0) {
                        break;
                    }
                } else {
                    dest = dest.wrapping_sub(1i32);
                    cursor2 = cursor2.wrapping_sub(1i32);
                    let _aastore_tmp3 = Clone::clone(&Clone::clone(&tmp.borrow()[cursor2 as usize]));
                    a.borrow_mut()[dest as usize] = _aastore_tmp3;
                    len2 = len2.wrapping_sub(1i32);
                    if len2 == 1i32 {
                        break;
                    }
                    let _t4: i32 = ComparableTimSort::gallopLeft(Clone::clone(&Clone::clone(&a.borrow()[cursor1 as usize])), Clone::clone(&tmp), tmpBase, len2, (len2).wrapping_sub(1i32))?;
                    count2 = (len2).wrapping_sub(_t4);
                    if (count2!=0) {
                        dest = (dest).wrapping_sub(count2);
                        cursor2 = (cursor2).wrapping_sub(count2);
                        len2 = (len2).wrapping_sub(count2);
                        System::arraycopy(Object::from_any(tmp.clone()), (cursor2).wrapping_add(1i32), Object::from_any(a.clone()), (dest).wrapping_add(1i32), count2)?;
                        if len2 <= 1i32 {
                            break;
                        }
                    } else {
                        dest = dest.wrapping_sub(1i32);
                        cursor1 = cursor1.wrapping_sub(1i32);
                        let _aastore_tmp5 = Clone::clone(&Clone::clone(&a.borrow()[cursor1 as usize]));
                        a.borrow_mut()[dest as usize] = _aastore_tmp5;
                        len1 = len1.wrapping_sub(1i32);
                        if (len1==0) {
                            break;
                        }
                        minGallop = minGallop.wrapping_sub(1i32);
                        if (minGallop<0) {
                            minGallop = 0i32;
                        }
                        minGallop = minGallop.wrapping_add(2i32);
                        continue;
                    }
                }
                if ((panic!("stack underflow") as i32)==0) { break; }
            }
            if (minGallop<0) {
                minGallop = 0i32;
            }
            minGallop = minGallop.wrapping_add(2i32);
        }
        this.minGallop.set((if minGallop < 1i32 { 1i32 } else { minGallop }));
        if len2 == 1i32 {
            if (len1<=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            dest = (dest).wrapping_sub(len1);
            cursor1 = (cursor1).wrapping_sub(len1);
            System::arraycopy(Object::from_any(a.clone()), (cursor1).wrapping_add(1i32), Object::from_any(a.clone()), (dest).wrapping_add(1i32), len1)?;
            let _aastore_tmp2 = Clone::clone(&Clone::clone(&tmp.borrow()[cursor2 as usize]));
            a.borrow_mut()[dest as usize] = _aastore_tmp2;
        } else {
            if (len2==0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (len1!=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (len2<=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            System::arraycopy(Object::from_any(tmp.clone()), tmpBase, Object::from_any(a.clone()), (dest).wrapping_sub((len2).wrapping_sub(1i32)), len2)?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "ensureCapacity", descriptor = "(I)[Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn ensureCapacity(&self, mut minCapacity: i32) -> Result<Rc<RefCell<Vec<Object>>>> {
        let this = self;
        let _t0: i32 = Integer::numberOfLeadingZeros(minCapacity)?;
        let mut newSize = ((-1i32 as u32>>(_t0&0x1f)) as i32);
        newSize = newSize.wrapping_add(1i32);
        if (newSize<0) {
            newSize = minCapacity;
        } else {
            let _t1: i32 = Math::min_i_i(newSize, (((this.a.get().borrow().len() as i32) as u32>>(1i32&0x1f)) as i32))?;
            newSize = _t1;
        }
        let mut _arr1: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); newSize as usize]));
        let mut newArray: Rc<RefCell<Vec<Object>>> = _arr1;
        this.tmp.set(Clone::clone(&newArray));
        this.tmpLen.set(newSize);
        this.tmpBase.set(0i32);
        Ok(this.tmp.get())
    }
}
