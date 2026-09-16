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

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/Spliterators$ArraySpliterator"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/Spliterator"]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/Spliterator<TT;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Spliterators.java"]
    #[inner_classes     = "java/util/Spliterators$ArraySpliterator:java/util/Spliterators:ArraySpliterator:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Spliterator;java/util/Spliterators$ArraySpliterator"]

    pub struct Spliterators_ArraySpliterator<T: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "array", descriptor = "[Ljava/lang/Object;", access = "private", modifiers = "final", is_static = false))]
        pub array: Rc<RefCell<Vec<Object>>>,
        #[cfg_attr(any(), java_field(name = "index", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub index: i32,
        #[cfg_attr(any(), java_field(name = "fence", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub fence: i32,
        #[cfg_attr(any(), java_field(name = "characteristics", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub characteristics: i32,
        #[cfg_attr(any(), java_field(name = "estimatedSize", descriptor = "J", access = "private", modifiers = "", is_static = false))]
        pub estimatedSize: i64,
    }

    impl<T> Spliterators_ArraySpliterator<T> {
        #[java_method(name = "<init>", descriptor = "([Ljava/lang/Object;I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_obj_i(array: Rc<RefCell<Vec<Object>>>, additionalCharacteristics: i32) -> Result<Self> {
            panic!("stub: java/util/Spliterators$ArraySpliterator.<init>:([Ljava/lang/Object;I)V")
        }

        #[java_method(name = "<init>", descriptor = "([Ljava/lang/Object;III)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>([Ljava/lang/Object;III)V
        pub fn new_arr_obj_i_i_i(mut array: Rc<RefCell<Vec<Object>>>, mut origin: i32, mut fence: i32, mut additionalCharacteristics: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_array(Clone::clone(&array));
            this.__set_index(origin);
            this.__set_fence(fence);
            this.__set_characteristics(((additionalCharacteristics|64i32)|16384i32));
            this.__set_estimatedSize(-1i64);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "([Ljava/lang/Object;IIIJ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_obj_i_i_i_l(array: Rc<RefCell<Vec<Object>>>, origin: i32, fence: i32, characteristics: i32, estimatedSize: i64) -> Result<Self> {
            panic!("stub: java/util/Spliterators$ArraySpliterator.<init>:([Ljava/lang/Object;IIIJ)V")
        }

        #[java_method(name = "trySplit", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TT;>;")]
        pub fn trySplit(&self) -> Result<Object> {
            panic!("stub: java/util/Spliterators$ArraySpliterator.trySplit:()Ljava/util/Spliterator;")
        }

        #[java_method(name = "forEachRemaining", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;)V")]
        pub fn forEachRemaining(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/Spliterators$ArraySpliterator.forEachRemaining:(Ljava/util/function/Consumer;)V")
        }

        #[java_method(name = "tryAdvance", descriptor = "(Ljava/util/function/Consumer;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;)Z")]
        pub fn tryAdvance(&self, action: Object) -> Result<bool> {
            panic!("stub: java/util/Spliterators$ArraySpliterator.tryAdvance:(Ljava/util/function/Consumer;)Z")
        }

        #[java_method(name = "estimateSize", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn estimateSize(&self) -> Result<i64> {
            panic!("stub: java/util/Spliterators$ArraySpliterator.estimateSize:()J")
        }

        #[java_method(name = "characteristics", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn characteristics(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_characteristics())
        }

        #[java_method(name = "getComparator", descriptor = "()Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Comparator<-TT;>;")]
        pub fn getComparator(&self) -> Result<Object> {
            let this = self;
            let _t0 = this.hasCharacteristics(4i32)?;
            if _t0 {
                return Ok(Object::default());
            }
            return Err(JvmError::Custom("athrow".to_owned()));
        }

        #[java_method(name = "getExactSizeIfKnown", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getExactSizeIfKnown(&self) -> Result<i64> {
            panic!("stub: java/util/Spliterators$ArraySpliterator.getExactSizeIfKnown:()J")
        }

        #[java_method(name = "hasCharacteristics", descriptor = "(I)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasCharacteristics(&self, mut characteristics: i32) -> Result<bool> {
            let this = self;
            let _t0 = this.characteristics()?;
            Ok((_t0&characteristics) == characteristics)
        }
    }
}
