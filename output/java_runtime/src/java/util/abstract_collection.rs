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
use crate::jdk::internal::util::ArraysSupport;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/AbstractCollection"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/Collection"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<E:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/Collection<TE;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AbstractCollection.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Iterable;java/lang/Object;java/util/AbstractCollection;java/util/Collection"]
    #[has_to_string_method = true]

    pub struct AbstractCollection<E: Clone + Default + 'static>;

    impl<E> AbstractCollection<E> {
        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TE;>;")]
        pub fn iterator(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractCollection.iterator:()Ljava/util/Iterator;")
        }

        #[java_method(name = "size", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn size(&self) -> Result<i32> {
            panic!("stub: java/util/AbstractCollection.size:()I")
        }

        #[java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty(&self) -> Result<bool> {
            panic!("stub: java/util/AbstractCollection.isEmpty:()Z")
        }

        #[java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contains(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractCollection.contains:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toArray()[Ljava/lang/Object;
        pub fn toArray(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            let this = self;
            let _t0 = this.size()?;
            let mut _arr1: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); _t0 as usize]));
            let mut r: Rc<RefCell<Vec<Object>>> = _arr1;
            let _t2 = this.iterator()?;
            let mut it: Object = _t2;
            let mut i: i32 = 0i32;
            loop {
                if i >= (r.borrow().len() as i32) { break; }
                let _vdispatch3: bool = if let Some(_d) = it.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = it.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
                if !(_vdispatch3) {
                    let _t4: Rc<RefCell<Vec<Object>>> = Arrays::copyOf_arr_obj_i(Clone::clone(&r), i)?;
                    return Ok(_t4);
                }
                let _vdispatch4: Object = if let Some(_d) = it.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = it.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = it.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.next()? } else if let Some(_d) = it.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(_d) = it.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.next()? } else if let Some(_d) = it.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = it.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                r.borrow_mut()[i as usize] = Clone::clone(&_vdispatch4);
                i = i.wrapping_add(1i32);
            }
            let _vdispatch3: bool = if let Some(_d) = it.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = it.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
            let mut _merged5: Rc<RefCell<Vec<Object>>>;
            if _vdispatch3 {
                let _t4: Rc<RefCell<Vec<Object>>> = AbstractCollection::<Object>::finishToArray(Clone::clone(&r), Clone::clone(&it))?;
                _merged5 = _t4;
            } else {
                _merged5 = r;
            }
            Ok(_merged5)
        }

        #[java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;")]
        pub fn toArray_arr_obj(&self, a: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/AbstractCollection.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
        }

        #[java_method(name = "finishToArray", descriptor = "([Ljava/lang/Object;Ljava/util/Iterator;)[Ljava/lang/Object;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;Ljava/util/Iterator<*>;)[TT;")]
        pub fn finishToArray(mut r: Rc<RefCell<Vec<Object>>>, mut it: Object) -> Result<Rc<RefCell<Vec<Object>>>> {
            let mut len = (r.borrow().len() as i32);
            let mut i: i32 = len;
            loop {
                let _vdispatch0: bool = if let Some(_d) = it.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = it.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = it.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
                if !(_vdispatch0) { break; }
                if i == len {
                    let _t0: i32 = ArraysSupport::newLength(len, 1i32, ((len>>((1i32&0x1f)))).wrapping_add(1i32))?;
                    len = _t0;
                    let _t1: Rc<RefCell<Vec<Object>>> = Arrays::copyOf_arr_obj_i(Clone::clone(&r), len)?;
                    r = _t1;
                }
                i = i.wrapping_add(1i32);
                let _vdispatch0: Object = if let Some(_d) = it.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = it.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = it.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.next()? } else if let Some(_d) = it.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(_d) = it.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.next()? } else if let Some(_d) = it.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = it.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                r.borrow_mut()[i as usize] = Clone::clone(&_vdispatch0);
            }
            let mut _merged1: Rc<RefCell<Vec<Object>>>;
            if i == len {
                _merged1 = r;
            } else {
                let _t0: Rc<RefCell<Vec<Object>>> = Arrays::copyOf_arr_obj_i(Clone::clone(&r), i)?;
                _merged1 = _t0;
            }
            Ok(_merged1)
        }

        #[java_method(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)Z")]
        pub fn add(&self, e: E) -> Result<bool> {
            panic!("stub: java/util/AbstractCollection.add:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remove(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractCollection.remove:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "containsAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z")]
        pub fn containsAll(&self, c: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractCollection.containsAll:(Ljava/util/Collection;)Z")
        }

        #[java_method(name = "addAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<+TE;>;)Z")]
        pub fn addAll(&self, c: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractCollection.addAll:(Ljava/util/Collection;)Z")
        }

        #[java_method(name = "removeAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z")]
        pub fn removeAll(&self, c: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractCollection.removeAll:(Ljava/util/Collection;)Z")
        }

        #[java_method(name = "retainAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z")]
        pub fn retainAll(&self, c: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractCollection.retainAll:(Ljava/util/Collection;)Z")
        }

        #[java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<()> {
            panic!("stub: java/util/AbstractCollection.clear:()V")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "toArray", descriptor = "(Ljava/util/function/IntFunction;)[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/IntFunction<[TT;>;)[TT;")]
        pub fn toArray_intfun(&self, generator: Object) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/AbstractCollection.toArray:(Ljava/util/function/IntFunction;)[Ljava/lang/Object;")
        }

        #[java_method(name = "removeIf", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TE;>;)Z")]
        pub fn removeIf(&self, filter: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractCollection.removeIf:(Ljava/util/function/Predicate;)Z")
        }

        #[java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TE;>;")]
        pub fn spliterator(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractCollection.spliterator:()Ljava/util/Spliterator;")
        }

        #[java_method(name = "stream", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<TE;>;")]
        pub fn stream(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractCollection.stream:()Ljava/util/stream/Stream;")
        }

        #[java_method(name = "parallelStream", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<TE;>;")]
        pub fn parallelStream(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractCollection.parallelStream:()Ljava/util/stream/Stream;")
        }

        #[java_method(name = "forEach", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;)V")]
        pub fn forEach(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/AbstractCollection.forEach:(Ljava/util/function/Consumer;)V")
        }
    }
}
