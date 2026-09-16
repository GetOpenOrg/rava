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

impl<E: Clone + Default + 'static> From<ArrayDeque<E>> for AbstractCollection<E> {
    fn from(v: ArrayDeque<E>) -> AbstractCollection<E> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/ArrayDeque"]
    #[super_class       = "java/util/AbstractCollection"]
    #[interfaces        = "java/util/Deque,java/lang/Cloneable,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "<E:Ljava/lang/Object;>Ljava/util/AbstractCollection<TE;>;Ljava/util/Deque<TE;>;Ljava/lang/Cloneable;Ljava/io/Serializable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ArrayDeque.java"]
    #[inner_classes     = "java/util/ArrayDeque$DeqIterator:java/util/ArrayDeque:DeqIterator:2;java/util/ArrayDeque$DescendingIterator:java/util/ArrayDeque:DescendingIterator:2;java/util/ArrayDeque$DeqSpliterator:java/util/ArrayDeque:DeqSpliterator:16;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractCollection<E>"]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Iterable;java/lang/Object;java/util/AbstractCollection;java/util/ArrayDeque;java/util/Collection;java/util/Deque;java/util/Queue;java/util/SequencedCollection"]

    pub struct ArrayDeque<E: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "elements", descriptor = "[Ljava/lang/Object;", access = "package", modifiers = "transient", is_static = false))]
        pub elements: Rc<RefCell<Vec<Object>>>,
        #[cfg_attr(any(), java_field(name = "head", descriptor = "I", access = "package", modifiers = "transient", is_static = false))]
        pub head: i32,
        #[cfg_attr(any(), java_field(name = "tail", descriptor = "I", access = "package", modifiers = "transient", is_static = false))]
        pub tail: i32,
    }

    impl<E> ArrayDeque<E> {
        #[cfg_attr(any(), java_field(name = "MAX_ARRAY_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2147483639"))]
        // static field: MAX_ARRAY_SIZE:I
        pub fn MAX_ARRAY_SIZE() -> i32 {
            2147483639
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "2340985798034038923"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            2340985798034038923i64
        }

        #[java_method(name = "grow", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn grow(&self, mut needed: i32) -> Result<()> {
            let this = self;
            let mut oldCapacity = (this.__get_elements().borrow().len() as i32);
            let mut jump = (if oldCapacity < 64i32 { (oldCapacity).wrapping_add(2i32) } else { (oldCapacity>>((1i32&0x1f))) });
            let mut newCapacity = (oldCapacity).wrapping_add(jump);
            if (((oldCapacity).wrapping_add(jump)).wrapping_sub(2147483639i32)>0) {
                let _t0 = this.newCapacity(needed, jump)?;
                newCapacity = _t0;
            }
            let _t0: Rc<RefCell<Vec<Object>>> = Arrays::copyOf_arr_obj_i(Clone::clone(&this.__get_elements()), newCapacity)?;
            this.__set_elements(Clone::clone(&_t0));
            let mut es: Rc<RefCell<Vec<Object>>> = _t0;
            let mut newSpace = (newCapacity).wrapping_sub(oldCapacity);
            System::arraycopy(Object::from_any(es.clone()), this.__get_head(), Object::from_any(es.clone()), (this.__get_head()).wrapping_add(newSpace), (oldCapacity).wrapping_sub(this.__get_head()))?;
            let mut i = this.__get_head();
            this.__set_head((this.__get_head()).wrapping_add(newSpace));
            let mut to = (this.__get_head()).wrapping_add(newSpace);
            loop {
                if i >= to { break; }
                es.borrow_mut()[i as usize] = Clone::clone(&Object::default());
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "newCapacity", descriptor = "(II)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newCapacity(&self, mut needed: i32, mut jump: i32) -> Result<i32> {
            let this = self;
            let mut oldCapacity = (this.__get_elements().borrow().len() as i32);
            let mut minCapacity = (oldCapacity).wrapping_add(needed);
            if (minCapacity<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            return Ok(2147483647i32);
            if needed > jump {
                return Ok(minCapacity);
            }
            Ok((if (((oldCapacity).wrapping_add(jump)).wrapping_sub(2147483639i32)<0) { (oldCapacity).wrapping_add(jump) } else { 2147483639i32 }))
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractCollection::new()?);
            let mut _arr0: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 17i32 as usize]));
            this.__set_elements(Clone::clone(&_arr0));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i(numElements: i32) -> Result<Self> {
            panic!("stub: java/util/ArrayDeque.<init>:(I)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Collection;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<+TE;>;)V")]
        pub fn new_coll(c: Object) -> Result<Self> {
            panic!("stub: java/util/ArrayDeque.<init>:(Ljava/util/Collection;)V")
        }

        #[java_method(name = "inc", descriptor = "(II)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: inc(II)I
        pub fn inc_i_i(mut i: i32, mut modulus: i32) -> Result<i32> {
            i = i.wrapping_add(1i32);
            if i >= modulus {
                i = 0i32;
            }
            Ok(i)
        }

        #[java_method(name = "dec", descriptor = "(II)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn dec(mut i: i32, mut modulus: i32) -> Result<i32> {
            i = i.wrapping_sub(1i32);
            if (i<0) {
                i = (modulus).wrapping_sub(1i32);
            }
            Ok(i)
        }

        #[java_method(name = "inc", descriptor = "(III)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn inc_i_i_i(i: i32, distance: i32, modulus: i32) -> Result<i32> {
            panic!("stub: java/util/ArrayDeque.inc:(III)I")
        }

        #[java_method(name = "sub", descriptor = "(III)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sub(mut i: i32, mut j: i32, mut modulus: i32) -> Result<i32> {
            i = (i).wrapping_sub(j);
            if ((i).wrapping_sub(j)<0) {
                i = (i).wrapping_add(modulus);
            }
            Ok(i)
        }

        #[java_method(name = "elementAt", descriptor = "([Ljava/lang/Object;I)Ljava/lang/Object;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>([Ljava/lang/Object;I)TE;")]
        pub fn elementAt(mut es: Rc<RefCell<Vec<Object>>>, mut i: i32) -> Result<E> {
            Ok(panic!("null"))
        }

        #[java_method(name = "nonNullElementAt", descriptor = "([Ljava/lang/Object;I)Ljava/lang/Object;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>([Ljava/lang/Object;I)TE;")]
        pub fn nonNullElementAt(es: Rc<RefCell<Vec<Object>>>, i: i32) -> Result<Object> {
            panic!("stub: java/util/ArrayDeque.nonNullElementAt:([Ljava/lang/Object;I)Ljava/lang/Object;")
        }

        #[java_method(name = "addFirst", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V")]
        pub fn addFirst(&self, mut e: E) -> Result<()> {
            let this = self;
            if _is_jnull(&e) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut es = this.__get_elements();
            let _t0: i32 = ArrayDeque::<Object>::dec(this.__get_head(), (es.borrow().len() as i32))?;
            this.__set_head(_t0);
            es.borrow_mut()[_t0 as usize] = Object::from_any(e.clone());
            if this.__get_head() == this.__get_tail() {
                this.grow(1i32)?;
            }
            Ok(())
        }

        #[java_method(name = "addLast", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V")]
        pub fn addLast(&self, mut e: E) -> Result<()> {
            let this = self;
            if _is_jnull(&e) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut es = this.__get_elements();
            es.borrow_mut()[this.__get_tail() as usize] = Object::from_any(e.clone());
            let _t0: i32 = ArrayDeque::<Object>::inc_i_i(this.__get_tail(), (es.borrow().len() as i32))?;
            this.__set_tail(_t0);
            if this.__get_head() == _t0 {
                this.grow(1i32)?;
            }
            Ok(())
        }

        #[java_method(name = "addAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<+TE;>;)Z")]
        pub fn addAll(&self, c: Object) -> Result<bool> {
            panic!("stub: java/util/ArrayDeque.addAll:(Ljava/util/Collection;)Z")
        }

        #[java_method(name = "copyElements", descriptor = "(Ljava/util/Collection;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<+TE;>;)V")]
        pub fn copyElements(&self, c: Object) -> Result<()> {
            panic!("stub: java/util/ArrayDeque.copyElements:(Ljava/util/Collection;)V")
        }

        #[java_method(name = "offerFirst", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)Z")]
        pub fn offerFirst(&self, e: E) -> Result<bool> {
            panic!("stub: java/util/ArrayDeque.offerFirst:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "offerLast", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)Z")]
        pub fn offerLast(&self, mut e: E) -> Result<bool> {
            let this = self;
            this.addLast(Clone::clone(&e))?;
            Ok((1i32 != 0i32))
        }

        #[java_method(name = "removeFirst", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn removeFirst(&self) -> Result<E> {
            let this = self;
            let _t0 = this.pollFirst()?;
            let mut e: E = _t0;
            if _is_jnull(&e) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(e)
        }

        #[java_method(name = "removeLast", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn removeLast(&self) -> Result<Object> {
            panic!("stub: java/util/ArrayDeque.removeLast:()Ljava/lang/Object;")
        }

        #[java_method(name = "pollFirst", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn pollFirst(&self) -> Result<E> {
            let this = self;
            let mut es = this.__get_elements();
            let mut h = this.__get_head();
            let _t0: Object = ArrayDeque::<Object>::elementAt(Clone::clone(&this.__get_elements()), this.__get_head())?;
            let mut e: E = _t0;
            if !_is_jnull(&e) {
                es.borrow_mut()[h as usize] = Clone::clone(&Object::default());
                let _t1: i32 = ArrayDeque::<Object>::inc_i_i(h, (es.borrow().len() as i32))?;
                this.__set_head(_t1);
            }
            Ok(e)
        }

        #[java_method(name = "pollLast", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn pollLast(&self) -> Result<Object> {
            panic!("stub: java/util/ArrayDeque.pollLast:()Ljava/lang/Object;")
        }

        #[java_method(name = "getFirst", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn getFirst(&self) -> Result<Object> {
            panic!("stub: java/util/ArrayDeque.getFirst:()Ljava/lang/Object;")
        }

        #[java_method(name = "getLast", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn getLast(&self) -> Result<Object> {
            panic!("stub: java/util/ArrayDeque.getLast:()Ljava/lang/Object;")
        }

        #[java_method(name = "peekFirst", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn peekFirst(&self) -> Result<E> {
            let this = self;
            let _t0: Object = ArrayDeque::<Object>::elementAt(Clone::clone(&this.__get_elements()), this.__get_head())?;
            Ok(panic!("null"))
        }

        #[java_method(name = "peekLast", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn peekLast(&self) -> Result<Object> {
            panic!("stub: java/util/ArrayDeque.peekLast:()Ljava/lang/Object;")
        }

        #[java_method(name = "removeFirstOccurrence", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn removeFirstOccurrence(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/ArrayDeque.removeFirstOccurrence:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "removeLastOccurrence", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn removeLastOccurrence(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/ArrayDeque.removeLastOccurrence:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)Z")]
        pub fn add(&self, e: E) -> Result<bool> {
            panic!("stub: java/util/ArrayDeque.add:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "offer", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)Z")]
        pub fn offer(&self, mut e: E) -> Result<bool> {
            let this = self;
            let _t0 = this.offerLast(Clone::clone(&e))?;
            Ok(_t0)
        }

        #[java_method(name = "remove", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn remove(&self) -> Result<Object> {
            panic!("stub: java/util/ArrayDeque.remove:()Ljava/lang/Object;")
        }

        #[java_method(name = "poll", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn poll(&self) -> Result<E> {
            let this = self;
            let _t0 = this.pollFirst()?;
            Ok(panic!("null"))
        }

        #[java_method(name = "element", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn element(&self) -> Result<Object> {
            panic!("stub: java/util/ArrayDeque.element:()Ljava/lang/Object;")
        }

        #[java_method(name = "peek", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn peek(&self) -> Result<E> {
            let this = self;
            let _t0 = this.peekFirst()?;
            Ok(panic!("null"))
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

        #[java_method(name = "delete", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn delete(&self, i: i32) -> Result<bool> {
            panic!("stub: java/util/ArrayDeque.delete:(I)Z")
        }

        #[java_method(name = "size", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn size(&self) -> Result<i32> {
            let this = self;
            let _t0: i32 = ArrayDeque::<Object>::sub(this.__get_tail(), this.__get_head(), (this.__get_elements().borrow().len() as i32))?;
            Ok(_t0)
        }

        #[java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty(&self) -> Result<bool> {
            panic!("stub: java/util/ArrayDeque.isEmpty:()Z")
        }

        #[java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TE;>;")]
        pub fn iterator(&self) -> Result<Object> {
            panic!("stub: java/util/ArrayDeque.iterator:()Ljava/util/Iterator;")
        }

        #[java_method(name = "descendingIterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TE;>;")]
        pub fn descendingIterator(&self) -> Result<Object> {
            panic!("stub: java/util/ArrayDeque.descendingIterator:()Ljava/util/Iterator;")
        }

        #[java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TE;>;")]
        pub fn spliterator(&self) -> Result<Object> {
            panic!("stub: java/util/ArrayDeque.spliterator:()Ljava/util/Spliterator;")
        }

        #[java_method(name = "forEach", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TE;>;)V")]
        pub fn forEach(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/ArrayDeque.forEach:(Ljava/util/function/Consumer;)V")
        }

        #[java_method(name = "removeIf", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TE;>;)Z")]
        pub fn removeIf(&self, filter: Object) -> Result<bool> {
            panic!("stub: java/util/ArrayDeque.removeIf:(Ljava/util/function/Predicate;)Z")
        }

        #[java_method(name = "removeAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z")]
        pub fn removeAll(&self, c: Object) -> Result<bool> {
            panic!("stub: java/util/ArrayDeque.removeAll:(Ljava/util/Collection;)Z")
        }

        #[java_method(name = "retainAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z")]
        pub fn retainAll(&self, c: Object) -> Result<bool> {
            panic!("stub: java/util/ArrayDeque.retainAll:(Ljava/util/Collection;)Z")
        }

        #[java_method(name = "bulkRemove", descriptor = "(Ljava/util/function/Predicate;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TE;>;)Z")]
        pub fn bulkRemove(&self, filter: Object) -> Result<bool> {
            panic!("stub: java/util/ArrayDeque.bulkRemove:(Ljava/util/function/Predicate;)Z")
        }

        #[java_method(name = "nBits", descriptor = "(I)[J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nBits(n: i32) -> Result<Rc<RefCell<Vec<i64>>>> {
            panic!("stub: java/util/ArrayDeque.nBits:(I)[J")
        }

        #[java_method(name = "setBit", descriptor = "([JI)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setBit(bits: Rc<RefCell<Vec<i64>>>, i: i32) -> Result<()> {
            panic!("stub: java/util/ArrayDeque.setBit:([JI)V")
        }

        #[java_method(name = "isClear", descriptor = "([JI)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isClear(bits: Rc<RefCell<Vec<i64>>>, i: i32) -> Result<bool> {
            panic!("stub: java/util/ArrayDeque.isClear:([JI)Z")
        }

        #[java_method(name = "bulkRemoveModified", descriptor = "(Ljava/util/function/Predicate;I)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TE;>;I)Z")]
        pub fn bulkRemoveModified(&self, filter: Object, beg: i32) -> Result<bool> {
            panic!("stub: java/util/ArrayDeque.bulkRemoveModified:(Ljava/util/function/Predicate;I)Z")
        }

        #[java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contains(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/ArrayDeque.contains:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remove_obj(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/ArrayDeque.remove:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<()> {
            panic!("stub: java/util/ArrayDeque.clear:()V")
        }

        #[java_method(name = "circularClear", descriptor = "([Ljava/lang/Object;II)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn circularClear(es: Rc<RefCell<Vec<Object>>>, i: i32, end: i32) -> Result<()> {
            panic!("stub: java/util/ArrayDeque.circularClear:([Ljava/lang/Object;II)V")
        }

        #[java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toArray(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/ArrayDeque.toArray:()[Ljava/lang/Object;")
        }

        #[java_method(name = "toArray", descriptor = "(Ljava/lang/Class;)[Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/lang/Class<[TT;>;)[TT;")]
        pub fn toArray_class(&self, klazz: Class<Rc<RefCell<Vec<Object>>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/ArrayDeque.toArray:(Ljava/lang/Class;)[Ljava/lang/Object;")
        }

        #[java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;")]
        pub fn toArray_arr_obj(&self, a: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/ArrayDeque.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/util/ArrayDeque;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/ArrayDeque<TE;>;")]
        pub fn clone(&self) -> Result<ArrayDeque<Object>> {
            panic!("stub: java/util/ArrayDeque.clone:()Ljava/util/ArrayDeque;")
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/ArrayDeque.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/ArrayDeque.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "checkInvariants", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkInvariants(&self) -> Result<()> {
            panic!("stub: java/util/ArrayDeque.checkInvariants:()V")
        }

        #[java_method(name = "reversed", descriptor = "()Ljava/util/Deque;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Deque<TE;>;")]
        pub fn reversed(&self) -> Result<Object> {
            panic!("stub: java/util/ArrayDeque.reversed:()Ljava/util/Deque;")
        }
    }
}
