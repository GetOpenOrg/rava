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

impl<E: Clone + Default + 'static> From<LinkedList<E>> for AbstractSequentialList<E> {
    fn from(v: LinkedList<E>) -> AbstractSequentialList<E> { v.__into_super() }
}

impl<E: Clone + Default + 'static> From<LinkedList<E>> for AbstractList<E> {
    fn from(v: LinkedList<E>) -> AbstractList<E> { v.__into_super().__into_super() }
}

impl<E: Clone + Default + 'static> From<LinkedList<E>> for AbstractCollection<E> {
    fn from(v: LinkedList<E>) -> AbstractCollection<E> { v.__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/LinkedList"]
    #[super_class       = "java/util/AbstractSequentialList"]
    #[interfaces        = "java/util/List,java/util/Deque,java/lang/Cloneable,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "<E:Ljava/lang/Object;>Ljava/util/AbstractSequentialList<TE;>;Ljava/util/List<TE;>;Ljava/util/Deque<TE;>;Ljava/lang/Cloneable;Ljava/io/Serializable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "LinkedList.java"]
    #[inner_classes     = "java/util/LinkedList$Node:java/util/LinkedList:Node:10;java/util/LinkedList$ListItr:java/util/LinkedList:ListItr:2;java/util/LinkedList$DescendingIterator:java/util/LinkedList:DescendingIterator:2;java/util/LinkedList$LLSpliterator:java/util/LinkedList:LLSpliterator:24;java/util/LinkedList$ReverseOrderLinkedListView:java/util/LinkedList:ReverseOrderLinkedListView:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractSequentialList<E>"]
    #[superclass_fields(modCount: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Iterable;java/lang/Object;java/util/AbstractCollection;java/util/AbstractList;java/util/AbstractSequentialList;java/util/Collection;java/util/Deque;java/util/LinkedList;java/util/List;java/util/Queue;java/util/SequencedCollection"]

    pub struct LinkedList<E: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "package", modifiers = "transient", is_static = false))]
        pub size: i32,
        #[cfg_attr(any(), java_field(name = "first", descriptor = "Ljava/util/LinkedList$Node;", access = "package", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/LinkedList$Node<TE;>;"))]
        pub first: LinkedList_Node<E>,
        #[cfg_attr(any(), java_field(name = "last", descriptor = "Ljava/util/LinkedList$Node;", access = "package", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/LinkedList$Node<TE;>;"))]
        pub last: LinkedList_Node<E>,
    }

    impl<E> LinkedList<E> {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "876323262645176354"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            876323262645176354i64
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractSequentialList::new()?);
            this.__set_size(0i32);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Collection;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<+TE;>;)V")]
        pub fn new_coll(c: Object) -> Result<Self> {
            panic!("stub: java/util/LinkedList.<init>:(Ljava/util/Collection;)V")
        }

        #[java_method(name = "linkFirst", descriptor = "(Ljava/lang/Object;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V")]
        pub fn linkFirst(&self, mut e: E) -> Result<()> {
            let this = self;
            let mut f = this.__get_first();
            let mut newNode = LinkedList_Node::<Object>::new(Default::default(), Clone::clone(&e), Clone::clone(&f))?;
            this.__set_first(Clone::clone(&newNode));
            if _is_jnull(&f) {
                this.__set_last(Clone::clone(&newNode));
            } else {
                f.__set_prev(Clone::clone(&newNode));
            }
            this.__set_size((this.__get_size()).wrapping_add(1i32));
            this.__set_modCount((this.__get_modCount()).wrapping_add(1i32));
            Ok(())
        }

        #[java_method(name = "linkLast", descriptor = "(Ljava/lang/Object;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V")]
        pub fn linkLast(&self, mut e: E) -> Result<()> {
            let this = self;
            let mut l = this.__get_last();
            let mut newNode = LinkedList_Node::<Object>::new(Clone::clone(&l), Clone::clone(&e), Default::default())?;
            this.__set_last(Clone::clone(&newNode));
            if _is_jnull(&l) {
                this.__set_first(Clone::clone(&newNode));
            } else {
                l.__set_next(Clone::clone(&newNode));
            }
            this.__set_size((this.__get_size()).wrapping_add(1i32));
            this.__set_modCount((this.__get_modCount()).wrapping_add(1i32));
            Ok(())
        }

        #[java_method(name = "linkBefore", descriptor = "(Ljava/lang/Object;Ljava/util/LinkedList$Node;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;Ljava/util/LinkedList$Node<TE;>;)V")]
        pub fn linkBefore(&self, e: E, succ: LinkedList_Node<E>) -> Result<()> {
            panic!("stub: java/util/LinkedList.linkBefore:(Ljava/lang/Object;Ljava/util/LinkedList$Node;)V")
        }

        #[java_method(name = "unlinkFirst", descriptor = "(Ljava/util/LinkedList$Node;)Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/LinkedList$Node<TE;>;)TE;")]
        pub fn unlinkFirst(&self, mut f: LinkedList_Node<E>) -> Result<E> {
            let this = self;
            let mut element = f.__get_item();
            let mut next = f.__get_next();
            f.__set_item(Clone::clone(&Object::default()));
            f.__set_next(Default::default());
            this.__set_first(Clone::clone(&next));
            if _is_jnull(&next) {
                this.__set_last(Default::default());
            } else {
                next.__set_prev(Default::default());
            }
            this.__set_size((this.__get_size()).wrapping_sub(1i32));
            this.__set_modCount((this.__get_modCount()).wrapping_add(1i32));
            Ok(element)
        }

        #[java_method(name = "unlinkLast", descriptor = "(Ljava/util/LinkedList$Node;)Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/LinkedList$Node<TE;>;)TE;")]
        pub fn unlinkLast(&self, mut l: LinkedList_Node<E>) -> Result<E> {
            let this = self;
            let mut element = l.__get_item();
            let mut prev = l.__get_prev();
            l.__set_item(Clone::clone(&Object::default()));
            l.__set_prev(Default::default());
            this.__set_last(Clone::clone(&prev));
            if _is_jnull(&prev) {
                this.__set_first(Default::default());
            } else {
                prev.__set_next(Default::default());
            }
            this.__set_size((this.__get_size()).wrapping_sub(1i32));
            this.__set_modCount((this.__get_modCount()).wrapping_add(1i32));
            Ok(element)
        }

        #[java_method(name = "unlink", descriptor = "(Ljava/util/LinkedList$Node;)Ljava/lang/Object;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/LinkedList$Node<TE;>;)TE;")]
        pub fn unlink(&self, x: LinkedList_Node<E>) -> Result<Object> {
            panic!("stub: java/util/LinkedList.unlink:(Ljava/util/LinkedList$Node;)Ljava/lang/Object;")
        }

        #[java_method(name = "getFirst", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn getFirst(&self) -> Result<E> {
            let this = self;
            let mut f = this.__get_first();
            if _is_jnull(&f) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(panic!("null"))
        }

        #[java_method(name = "getLast", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn getLast(&self) -> Result<E> {
            let this = self;
            let mut l = this.__get_last();
            if _is_jnull(&l) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(panic!("null"))
        }

        #[java_method(name = "removeFirst", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn removeFirst(&self) -> Result<E> {
            let this = self;
            let mut f = this.__get_first();
            if _is_jnull(&f) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0 = this.unlinkFirst(Clone::clone(&f))?;
            Ok(panic!("null"))
        }

        #[java_method(name = "removeLast", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn removeLast(&self) -> Result<E> {
            let this = self;
            let mut l = this.__get_last();
            if _is_jnull(&l) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0 = this.unlinkLast(Clone::clone(&l))?;
            Ok(panic!("null"))
        }

        #[java_method(name = "addFirst", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V")]
        pub fn addFirst(&self, mut e: E) -> Result<()> {
            let this = self;
            this.linkFirst(Clone::clone(&e))?;
            Ok(())
        }

        #[java_method(name = "addLast", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V")]
        pub fn addLast(&self, mut e: E) -> Result<()> {
            let this = self;
            this.linkLast(Clone::clone(&e))?;
            Ok(())
        }

        #[java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contains(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/LinkedList.contains:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "size", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn size(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_size())
        }

        #[java_method(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)Z")]
        // java: add(Ljava/lang/Object;)Z
        pub fn add_obj(&self, mut e: E) -> Result<bool> {
            let this = self;
            this.linkLast(Clone::clone(&e))?;
            Ok((1i32 != 0i32))
        }

        #[java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remove_obj(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/LinkedList.remove:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "addAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<+TE;>;)Z")]
        pub fn addAll_coll(&self, c: Object) -> Result<bool> {
            panic!("stub: java/util/LinkedList.addAll:(Ljava/util/Collection;)Z")
        }

        #[java_method(name = "addAll", descriptor = "(ILjava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/util/Collection<+TE;>;)Z")]
        pub fn addAll_i_coll(&self, index: i32, c: Object) -> Result<bool> {
            panic!("stub: java/util/LinkedList.addAll:(ILjava/util/Collection;)Z")
        }

        #[java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<()> {
            let this = self;
            let mut x = this.__get_first();
            loop {
                if _is_jnull(&x) { break; }
                let mut next = x.__get_next();
                x.__set_item(Clone::clone(&Object::default()));
                x.__set_next(Default::default());
                x.__set_prev(Default::default());
                x = next;
            }
            this.__set_last(Default::default());
            this.__set_first(Default::default());
            this.__set_size(0i32);
            this.__set_modCount((this.__get_modCount()).wrapping_add(1i32));
            Ok(())
        }

        #[java_method(name = "get", descriptor = "(I)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)TE;")]
        pub fn get(&self, mut index: i32) -> Result<E> {
            let this = self;
            this.checkElementIndex(index)?;
            let _t0 = this.node(index)?;
            Ok(panic!("null"))
        }

        #[java_method(name = "set", descriptor = "(ILjava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITE;)TE;")]
        pub fn set(&self, index: i32, element: E) -> Result<Object> {
            panic!("stub: java/util/LinkedList.set:(ILjava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "add", descriptor = "(ILjava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITE;)V")]
        pub fn add_i_obj(&self, index: i32, element: E) -> Result<()> {
            panic!("stub: java/util/LinkedList.add:(ILjava/lang/Object;)V")
        }

        #[java_method(name = "remove", descriptor = "(I)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)TE;")]
        pub fn remove_i(&self, index: i32) -> Result<Object> {
            panic!("stub: java/util/LinkedList.remove:(I)Ljava/lang/Object;")
        }

        #[java_method(name = "isElementIndex", descriptor = "(I)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isElementIndex(&self, mut index: i32) -> Result<bool> {
            let this = self;
            Ok((if (index>=0) { index < this.__get_size() } else { (0i32 != 0) }))
        }

        #[java_method(name = "isPositionIndex", descriptor = "(I)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isPositionIndex(&self, index: i32) -> Result<bool> {
            panic!("stub: java/util/LinkedList.isPositionIndex:(I)Z")
        }

        #[java_method(name = "outOfBoundsMsg", descriptor = "(I)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn outOfBoundsMsg(&self, mut index: i32) -> Result<String> {
            let this = self;
            let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Index: ")))?;
            let _t1 = _t0.append_i(index)?;
            let _t2 = _t1.append_str(Clone::clone(&String::from(", Size: ")))?;
            let _t3 = _t2.append_i(this.__get_size())?;
            let _t4 = _t3.toString()?;
            Ok(_t4)
        }

        #[java_method(name = "checkElementIndex", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkElementIndex(&self, mut index: i32) -> Result<()> {
            let this = self;
            let _t0 = this.isElementIndex(index)?;
            if !(_t0) {
                let _t1 = this.outOfBoundsMsg(index)?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "checkPositionIndex", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkPositionIndex(&self, index: i32) -> Result<()> {
            panic!("stub: java/util/LinkedList.checkPositionIndex:(I)V")
        }

        #[java_method(name = "node", descriptor = "(I)Ljava/util/LinkedList$Node;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)Ljava/util/LinkedList$Node<TE;>;")]
        pub fn node(&self, mut index: i32) -> Result<LinkedList_Node<E>> {
            let this = self;
            let mut x = this.__get_first();
            let mut i: i32 = 0i32;
            loop {
                if i >= index { break; }
                x = x.__get_next();
                i = i.wrapping_add(1i32);
            }
            return Ok(Default::default());
            x = this.__get_last();
            i = (this.__get_size()).wrapping_sub(1i32);
            loop {
                if i <= index { break; }
                x = x.__get_prev();
                i = i.wrapping_sub(1i32);
            }
            Ok(Default::default())
        }

        #[java_method(name = "indexOf", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOf(&self, o: Object) -> Result<i32> {
            panic!("stub: java/util/LinkedList.indexOf:(Ljava/lang/Object;)I")
        }

        #[java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
            panic!("stub: java/util/LinkedList.lastIndexOf:(Ljava/lang/Object;)I")
        }

        #[java_method(name = "peek", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn peek(&self) -> Result<E> {
            let this = self;
            let mut f = this.__get_first();
            Ok(panic!("null"))
        }

        #[java_method(name = "element", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn element(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedList.element:()Ljava/lang/Object;")
        }

        #[java_method(name = "poll", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn poll(&self) -> Result<E> {
            let this = self;
            let mut f = this.__get_first();
            let mut _merged1: Object;
            if _is_jnull(&f) {
                _merged1 = Object::default();
            } else {
                let _t0 = this.unlinkFirst(Clone::clone(&f))?;
                _merged1 = _t0;
            }
            Ok(panic!("null"))
        }

        #[java_method(name = "remove", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn remove(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedList.remove:()Ljava/lang/Object;")
        }

        #[java_method(name = "offer", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)Z")]
        pub fn offer(&self, mut e: E) -> Result<bool> {
            let this = self;
            let _t0 = this.add_obj(Clone::clone(&e))?;
            Ok(_t0)
        }

        #[java_method(name = "offerFirst", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)Z")]
        pub fn offerFirst(&self, e: E) -> Result<bool> {
            panic!("stub: java/util/LinkedList.offerFirst:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "offerLast", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)Z")]
        pub fn offerLast(&self, e: E) -> Result<bool> {
            panic!("stub: java/util/LinkedList.offerLast:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "peekFirst", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn peekFirst(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedList.peekFirst:()Ljava/lang/Object;")
        }

        #[java_method(name = "peekLast", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn peekLast(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedList.peekLast:()Ljava/lang/Object;")
        }

        #[java_method(name = "pollFirst", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn pollFirst(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedList.pollFirst:()Ljava/lang/Object;")
        }

        #[java_method(name = "pollLast", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn pollLast(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedList.pollLast:()Ljava/lang/Object;")
        }

        #[java_method(name = "push", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V")]
        pub fn push(&self, mut e: E) -> Result<()> {
            let this = self;
            this.addFirst(Clone::clone(&e))?;
            Ok(())
        }

        #[java_method(name = "pop", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn pop(&self) -> Result<E> {
            let this = self;
            let _t0 = this.removeFirst()?;
            Ok(panic!("null"))
        }

        #[java_method(name = "removeFirstOccurrence", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn removeFirstOccurrence(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/LinkedList.removeFirstOccurrence:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "removeLastOccurrence", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn removeLastOccurrence(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/LinkedList.removeLastOccurrence:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "listIterator", descriptor = "(I)Ljava/util/ListIterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)Ljava/util/ListIterator<TE;>;")]
        pub fn listIterator(&self, index: i32) -> Result<Object> {
            panic!("stub: java/util/LinkedList.listIterator:(I)Ljava/util/ListIterator;")
        }

        #[java_method(name = "descendingIterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TE;>;")]
        pub fn descendingIterator(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedList.descendingIterator:()Ljava/util/Iterator;")
        }

        #[java_method(name = "superClone", descriptor = "()Ljava/util/LinkedList;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/LinkedList<TE;>;")]
        pub fn superClone(&self) -> Result<LinkedList<Object>> {
            panic!("stub: java/util/LinkedList.superClone:()Ljava/util/LinkedList;")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedList.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toArray()[Ljava/lang/Object;
        pub fn toArray(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            let this = self;
            let mut _arr0: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); this.__get_size() as usize]));
            let mut result: Rc<RefCell<Vec<Object>>> = _arr0;
            let mut i: i32 = 0i32;
            let mut x = this.__get_first();
            loop {
                if _is_jnull(&x) { break; }
                i = i.wrapping_add(1i32);
                result.borrow_mut()[i as usize] = Clone::clone(&x.__get_item());
                x = x.__get_next();
            }
            Ok(result)
        }

        #[java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;")]
        // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
        pub fn toArray_arr_obj(&self, mut a: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
            let this = self;
            if (a.borrow().len() as i32) < this.__get_size() {
                let _t0: Object = Object::default();
                let _vdispatch1: Object = if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let _t2: Object = Array::newInstance_class_i(Clone::clone(&_vdispatch1), this.__get_size())?;
                a = (_t2).downcast::<Rc<RefCell<Vec<Object>>>>();
            }
            let mut i: i32 = 0i32;
            let mut result: Rc<RefCell<Vec<Object>>> = a;
            let mut x = this.__get_first();
            loop {
                if _is_jnull(&x) { break; }
                i = i.wrapping_add(1i32);
                result.borrow_mut()[i as usize] = Clone::clone(&x.__get_item());
                x = x.__get_next();
            }
            if (a.borrow().len() as i32) > this.__get_size() {
                a.borrow_mut()[this.__get_size() as usize] = Clone::clone(&Object::default());
            }
            Ok(a)
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/LinkedList.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/LinkedList.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TE;>;")]
        pub fn spliterator(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedList.spliterator:()Ljava/util/Spliterator;")
        }

        #[java_method(name = "reversed", descriptor = "()Ljava/util/LinkedList;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/LinkedList<TE;>;")]
        pub fn reversed(&self) -> Result<LinkedList<Object>> {
            panic!("stub: java/util/LinkedList.reversed:()Ljava/util/LinkedList;")
        }

        #[java_method(name = "replaceAll", descriptor = "(Ljava/util/function/UnaryOperator;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/UnaryOperator<TE;>;)V")]
        pub fn replaceAll(&self, operator: Object) -> Result<()> {
            panic!("stub: java/util/LinkedList.replaceAll:(Ljava/util/function/UnaryOperator;)V")
        }

        #[java_method(name = "sort", descriptor = "(Ljava/util/Comparator;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Comparator<-TE;>;)V")]
        pub fn sort(&self, mut c: Object) -> Result<()> {
            let this = self;
            let _t0 = this.toArray()?;
            let mut a: Rc<RefCell<Vec<Object>>> = _t0;
            Arrays::sort_arr_obj_compar(Clone::clone(&a), Clone::clone(&c))?;
            let _t1 = this.__super().__super().listIterator()?;
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
    }
}
