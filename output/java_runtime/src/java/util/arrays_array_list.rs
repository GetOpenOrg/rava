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

impl<E: Clone + Default + 'static> From<Arrays_ArrayList<E>> for AbstractList<E> {
    fn from(v: Arrays_ArrayList<E>) -> AbstractList<E> { v.__into_super() }
}

impl<E: Clone + Default + 'static> From<Arrays_ArrayList<E>> for AbstractCollection<E> {
    fn from(v: Arrays_ArrayList<E>) -> AbstractCollection<E> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/Arrays$ArrayList"]
    #[super_class       = "java/util/AbstractList"]
    #[interfaces        = "java/util/RandomAccess,java/io/Serializable"]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "<E:Ljava/lang/Object;>Ljava/util/AbstractList<TE;>;Ljava/util/RandomAccess;Ljava/io/Serializable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Arrays.java"]
    #[inner_classes     = "java/util/Arrays$ArrayList:java/util/Arrays:ArrayList:10;java/util/Arrays$ArrayItr:java/util/Arrays:ArrayItr:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractList<E>"]
    #[superclass_fields(modCount: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Iterable;java/lang/Object;java/util/AbstractCollection;java/util/AbstractList;java/util/Arrays$ArrayList;java/util/Collection;java/util/List;java/util/RandomAccess;java/util/SequencedCollection"]

    pub struct Arrays_ArrayList<E: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "a", descriptor = "[Ljava/lang/Object;", access = "private", modifiers = "final", is_static = false, generic_signature = "[TE;"))]
        pub a: Rc<RefCell<Vec<E>>>,
    }

    impl<E> Arrays_ArrayList<E> {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-2764017481108945198"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -2764017481108945198i64
        }

        #[java_method(name = "<init>", descriptor = "([Ljava/lang/Object;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([TE;)V")]
        pub fn new(mut array: Rc<RefCell<Vec<E>>>) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractList::new()?);
            let _t0: Object = Objects::requireNonNull_obj(Object::from_any(array.clone()))?;
            this.__set_a(Clone::clone(&(_t0).downcast::<Rc<RefCell<Vec<E>>>>()));
            Ok(this)
        }

        #[java_method(name = "size", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn size(&self) -> Result<i32> {
            panic!("stub: java/util/Arrays$ArrayList.size:()I")
        }

        #[java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toArray(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/Arrays$ArrayList.toArray:()[Ljava/lang/Object;")
        }

        #[java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;")]
        pub fn toArray_arr_obj(&self, a: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/Arrays$ArrayList.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
        }

        #[java_method(name = "get", descriptor = "(I)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)TE;")]
        pub fn get(&self, index: i32) -> Result<Object> {
            panic!("stub: java/util/Arrays$ArrayList.get:(I)Ljava/lang/Object;")
        }

        #[java_method(name = "set", descriptor = "(ILjava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITE;)TE;")]
        pub fn set(&self, index: i32, element: E) -> Result<Object> {
            panic!("stub: java/util/Arrays$ArrayList.set:(ILjava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "indexOf", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOf(&self, o: Object) -> Result<i32> {
            panic!("stub: java/util/Arrays$ArrayList.indexOf:(Ljava/lang/Object;)I")
        }

        #[java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contains(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/Arrays$ArrayList.contains:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TE;>;")]
        pub fn spliterator(&self) -> Result<Object> {
            panic!("stub: java/util/Arrays$ArrayList.spliterator:()Ljava/util/Spliterator;")
        }

        #[java_method(name = "forEach", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TE;>;)V")]
        pub fn forEach(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/Arrays$ArrayList.forEach:(Ljava/util/function/Consumer;)V")
        }

        #[java_method(name = "replaceAll", descriptor = "(Ljava/util/function/UnaryOperator;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/UnaryOperator<TE;>;)V")]
        pub fn replaceAll(&self, operator: Object) -> Result<()> {
            panic!("stub: java/util/Arrays$ArrayList.replaceAll:(Ljava/util/function/UnaryOperator;)V")
        }

        #[java_method(name = "sort", descriptor = "(Ljava/util/Comparator;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Comparator<-TE;>;)V")]
        pub fn sort(&self, c: Object) -> Result<()> {
            panic!("stub: java/util/Arrays$ArrayList.sort:(Ljava/util/Comparator;)V")
        }

        #[java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TE;>;")]
        pub fn iterator(&self) -> Result<Object> {
            panic!("stub: java/util/Arrays$ArrayList.iterator:()Ljava/util/Iterator;")
        }
    }
}
