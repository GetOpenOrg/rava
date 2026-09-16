#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::math::*;
use crate::java::nio::*;
use crate::java::nio::charset::*;
use crate::java::security::*;
use crate::java::text::*;
use crate::java::text::spi::*;
use crate::java::time::*;
use crate::java::time::chrono::*;
use crate::java::time::temporal::*;
use crate::java::time::zone::*;
use crate::java::util::*;
use crate::java::util::concurrent::*;
use crate::java::util::concurrent::atomic::*;
use crate::java::util::concurrent::locks::*;
use crate::java::util::function::*;
use crate::java::util::regex::*;
use crate::java::util::spi::*;
use crate::java::util::stream::*;
use crate::java::util::zip::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::reflect::misc::*;
use crate::sun::security::action::*;
use crate::sun::security::util::*;
use crate::sun::text::*;
use crate::sun::util::*;
use crate::sun::util::calendar::*;
use crate::sun::util::locale::*;
use crate::sun::util::locale::provider::*;
use crate::sun::util::spi::*;
use crate::java::text::Normalizer;

impl<E: Clone + Default + 'static> From<AbstractList<E>> for AbstractCollection<E> {
    fn from(v: AbstractList<E>) -> AbstractCollection<E> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/AbstractList"]
    #[super_class       = "java/util/AbstractCollection"]
    #[interfaces        = "java/util/List"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<E:Ljava/lang/Object;>Ljava/util/AbstractCollection<TE;>;Ljava/util/List<TE;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AbstractList.java"]
    #[inner_classes     = "java/util/AbstractList$Itr:java/util/AbstractList:Itr:2;java/util/AbstractList$ListItr:java/util/AbstractList:ListItr:2;java/util/AbstractList$RandomAccessSubList:java/util/AbstractList:RandomAccessSubList:10;java/util/AbstractList$SubList:java/util/AbstractList:SubList:10;java/util/AbstractList$RandomAccessSpliterator:java/util/AbstractList:RandomAccessSpliterator:24;java/util/AbstractList$SubList$1:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractCollection<E>"]
    #[all_supertypes    = "java/lang/Iterable;java/lang/Object;java/util/AbstractCollection;java/util/AbstractList;java/util/Collection;java/util/List;java/util/SequencedCollection"]
    #[has_hash_code_method = true]

    pub struct AbstractList<E: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "modCount", descriptor = "I", access = "protected", modifiers = "transient", is_static = false))]
        pub modCount: i32,
    }

    impl<E> AbstractList<E> {
        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractCollection::new()?);
            this.__set_modCount(0i32);
            Ok(this)
        }

        #[java_method(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)Z")]
        // java: add(Ljava/lang/Object;)Z
        pub fn add_obj(&self, mut e: E) -> Result<bool> {
            let this = self;
            let _t0 = this.__super().size()?;
            this.add_i_obj(_t0, Clone::clone(&e))?;
            Ok((1i32 != 0i32))
        }

        #[java_method(name = "get", descriptor = "(I)Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(I)TE;")]
        pub fn get(&self, arg0: i32) -> Result<Object> {
            panic!("stub: java/util/AbstractList.get:(I)Ljava/lang/Object;")
        }

        #[java_method(name = "set", descriptor = "(ILjava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITE;)TE;")]
        pub fn set(&self, index: i32, element: E) -> Result<Object> {
            panic!("stub: java/util/AbstractList.set:(ILjava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "add", descriptor = "(ILjava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITE;)V")]
        // java: add(ILjava/lang/Object;)V
        pub fn add_i_obj(&self, mut index: i32, mut element: E) -> Result<()> {
            let this = self;
            return Err(JvmError::Custom("athrow".to_owned()));
            Ok(())
        }

        #[java_method(name = "remove", descriptor = "(I)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)TE;")]
        pub fn remove(&self, index: i32) -> Result<Object> {
            panic!("stub: java/util/AbstractList.remove:(I)Ljava/lang/Object;")
        }

        #[java_method(name = "indexOf", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOf(&self, o: Object) -> Result<i32> {
            panic!("stub: java/util/AbstractList.indexOf:(Ljava/lang/Object;)I")
        }

        #[java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
            panic!("stub: java/util/AbstractList.lastIndexOf:(Ljava/lang/Object;)I")
        }

        #[java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<()> {
            let this = self;
            let _t0 = this.__super().size()?;
            this.removeRange(0i32, _t0)?;
            Ok(())
        }

        #[java_method(name = "addAll", descriptor = "(ILjava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/util/Collection<+TE;>;)Z")]
        pub fn addAll(&self, index: i32, c: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractList.addAll:(ILjava/util/Collection;)Z")
        }

        #[java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TE;>;")]
        pub fn iterator(&self) -> Result<Object> {
            let this = self;
            Ok(Object::from_any(AbstractList_Itr::new(Clone::clone(this))?.clone()))
        }

        #[java_method(name = "listIterator", descriptor = "()Ljava/util/ListIterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/ListIterator<TE;>;")]
        // java: listIterator()Ljava/util/ListIterator;
        pub fn listIterator(&self) -> Result<Object> {
            let this = self;
            let _t0 = this.listIterator_i(0i32)?;
            Ok(_t0)
        }

        #[java_method(name = "listIterator", descriptor = "(I)Ljava/util/ListIterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)Ljava/util/ListIterator<TE;>;")]
        // java: listIterator(I)Ljava/util/ListIterator;
        pub fn listIterator_i(&self, mut index: i32) -> Result<Object> {
            let this = self;
            this.rangeCheckForAdd(index)?;
            Ok(Object::from_any(AbstractList_ListItr::new(Clone::clone(this), index)?.clone()))
        }

        #[java_method(name = "subList", descriptor = "(II)Ljava/util/List;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(II)Ljava/util/List<TE;>;")]
        pub fn subList(&self, mut fromIndex: i32, mut toIndex: i32) -> Result<Object> {
            let this = self;
            let _t0 = this.__super().size()?;
            AbstractList::<Object>::subListRangeCheck(fromIndex, toIndex, _t0)?;
            Ok(Object::from_any((if false { AbstractList_RandomAccessSubList::<Object>::new_abstra_i_i(Clone::clone(this), fromIndex, toIndex)? } else { AbstractList_SubList::<Object>::new_abstra_i_i(Clone::clone(this), fromIndex, toIndex)? }).clone()))
        }

        #[java_method(name = "subListRangeCheck", descriptor = "(III)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subListRangeCheck(mut fromIndex: i32, mut toIndex: i32, mut size: i32) -> Result<()> {
            if (fromIndex<0) {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("fromIndex = ")))?;
                let _t1 = _t0.append_i(fromIndex)?;
                let _t2 = _t1.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if toIndex > size {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("toIndex = ")))?;
                let _t1 = _t0.append_i(toIndex)?;
                let _t2 = _t1.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if fromIndex > toIndex {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("fromIndex(")))?;
                let _t1 = _t0.append_i(fromIndex)?;
                let _t2 = _t1.append_str(Clone::clone(&String::from(") > toIndex(")))?;
                let _t3 = _t2.append_i(toIndex)?;
                let _t4 = _t3.append_str(Clone::clone(&String::from(")")))?;
                let _t5 = _t4.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractList.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "removeRange", descriptor = "(II)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn removeRange(&self, mut fromIndex: i32, mut toIndex: i32) -> Result<()> {
            let this = self;
            let _t0 = this.listIterator_i(fromIndex)?;
            let mut it: Object = _t0;
            let mut i: i32 = 0i32;
            let mut n = (toIndex).wrapping_sub(fromIndex);
            loop {
                if i >= n { break; }
                let _vdispatch1: Object = if let Some(_d) = it.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = it.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = it.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = it.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                if let Some(_d) = it.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.remove()?; } else if let Some(_d) = it.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.remove()?; } else if let Some(_d) = it.0.as_any().downcast_ref::<Object>() { _d.remove()?; } else if let Some(__f) = it.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<()>>>() { (__f)()?; }
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "rangeCheckForAdd", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rangeCheckForAdd(&self, mut index: i32) -> Result<()> {
            let this = self;
            let _t0 = this.__super().size()?;
            if index > _t0 {
                let _t1 = this.outOfBoundsMsg(index)?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "outOfBoundsMsg", descriptor = "(I)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn outOfBoundsMsg(&self, mut index: i32) -> Result<String> {
            let this = self;
            let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Index: ")))?;
            let _t1 = _t0.append_i(index)?;
            let _t2 = _t1.append_str(Clone::clone(&String::from(", Size: ")))?;
            let _t3 = this.__super().size()?;
            let _t4 = _t2.append_i(_t3)?;
            let _t5 = _t4.toString()?;
            Ok(_t5)
        }

        #[java_method(name = "replaceAll", descriptor = "(Ljava/util/function/UnaryOperator;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/UnaryOperator<TE;>;)V")]
        pub fn replaceAll(&self, operator: Object) -> Result<()> {
            panic!("stub: java/util/AbstractList.replaceAll:(Ljava/util/function/UnaryOperator;)V")
        }

        #[java_method(name = "sort", descriptor = "(Ljava/util/Comparator;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Comparator<-TE;>;)V")]
        pub fn sort(&self, mut c: Object) -> Result<()> {
            let this = self;
            let _t0 = this.__super().toArray()?;
            let mut a: Rc<RefCell<Vec<Object>>> = _t0;
            Arrays::sort_arr_obj_compar(Clone::clone(&a), Clone::clone(&c))?;
            let _t1 = this.listIterator()?;
            let mut i: Object = _t1;
            let mut local_4: Rc<RefCell<Vec<Object>>> = a;
            let mut local_5 = (local_4.borrow().len() as i32);
            let mut local_6: i32 = 0i32;
            loop {
                if local_6 >= local_5 { break; }
                let mut e = Clone::clone(&local_4.borrow()[local_6 as usize]);
                let _vdispatch2: Object = if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = i.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                if let Some(_d) = i.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.set(Clone::clone(&e))?; } else if let Some(_d) = i.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.set(Clone::clone(&e))?; } else if let Some(_d) = i.0.as_any().downcast_ref::<Object>() { _d.set(Clone::clone(&e))?; } else if let Some(__f) = i.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<()>>>() { (__f)(Clone::clone(&e))?; }
                local_6 = local_6.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TE;>;")]
        pub fn spliterator(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractList.spliterator:()Ljava/util/Spliterator;")
        }

        #[java_method(name = "addFirst", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V")]
        pub fn addFirst(&self, e: E) -> Result<()> {
            panic!("stub: java/util/AbstractList.addFirst:(Ljava/lang/Object;)V")
        }

        #[java_method(name = "addLast", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V")]
        pub fn addLast(&self, e: E) -> Result<()> {
            panic!("stub: java/util/AbstractList.addLast:(Ljava/lang/Object;)V")
        }

        #[java_method(name = "getFirst", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn getFirst(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractList.getFirst:()Ljava/lang/Object;")
        }

        #[java_method(name = "getLast", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn getLast(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractList.getLast:()Ljava/lang/Object;")
        }

        #[java_method(name = "removeFirst", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn removeFirst(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractList.removeFirst:()Ljava/lang/Object;")
        }

        #[java_method(name = "removeLast", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn removeLast(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractList.removeLast:()Ljava/lang/Object;")
        }

        #[java_method(name = "reversed", descriptor = "()Ljava/util/List;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/List<TE;>;")]
        pub fn reversed(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractList.reversed:()Ljava/util/List;")
        }
    }
}
