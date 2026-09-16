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
    #[binary_name       = "java/text/BreakIterator"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/lang/Cloneable"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "BreakIterator.java"]
    #[inner_classes     = "java/text/BreakIterator$BreakIteratorCache:java/text/BreakIterator:BreakIteratorCache:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Cloneable;java/lang/Object;java/text/BreakIterator"]

    pub struct BreakIterator;

    impl BreakIterator {
        #[cfg_attr(any(), java_field(name = "DONE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "-1"))]
        // static field: DONE:I
        pub fn DONE() -> i32 {
            -1
        }

        #[cfg_attr(any(), java_field(name = "CHARACTER_INDEX", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: CHARACTER_INDEX:I
        pub fn CHARACTER_INDEX() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "WORD_INDEX", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: WORD_INDEX:I
        pub fn WORD_INDEX() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "LINE_INDEX", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: LINE_INDEX:I
        pub fn LINE_INDEX() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "SENTENCE_INDEX", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: SENTENCE_INDEX:I
        pub fn SENTENCE_INDEX() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "iterCache", descriptor = "[Ljava/lang/ref/SoftReference;", access = "private", modifiers = "static final", is_static = true, generic_signature = "[Ljava/lang/ref/SoftReference<Ljava/text/BreakIterator$BreakIteratorCache;>;"))]
        // static field: iterCache:[Ljava/lang/ref/SoftReference;
        pub fn iterCache() -> Rc<RefCell<Vec<SoftReference<BreakIterator_BreakIteratorCache>>>> {
            panic!("stub: java/text/BreakIterator.iterCache:[Ljava/lang/ref/SoftReference;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/text/BreakIterator.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "first", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn first(&self) -> Result<i32> {
            panic!("stub: java/text/BreakIterator.first:()I")
        }

        #[java_method(name = "last", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn last(&self) -> Result<i32> {
            panic!("stub: java/text/BreakIterator.last:()I")
        }

        #[java_method(name = "next", descriptor = "(I)I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn next_i(&self, arg0: i32) -> Result<i32> {
            panic!("stub: java/text/BreakIterator.next:(I)I")
        }

        #[java_method(name = "next", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn next(&self) -> Result<i32> {
            panic!("stub: java/text/BreakIterator.next:()I")
        }

        #[java_method(name = "previous", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn previous(&self) -> Result<i32> {
            panic!("stub: java/text/BreakIterator.previous:()I")
        }

        #[java_method(name = "following", descriptor = "(I)I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn following(&self, arg0: i32) -> Result<i32> {
            panic!("stub: java/text/BreakIterator.following:(I)I")
        }

        #[java_method(name = "preceding", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn preceding(&self, offset: i32) -> Result<i32> {
            panic!("stub: java/text/BreakIterator.preceding:(I)I")
        }

        #[java_method(name = "isBoundary", descriptor = "(I)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isBoundary(&self, mut offset: i32) -> Result<bool> {
            let this = self;
            if (offset==0) {
                return Ok((1i32 != 0i32));
            }
            let _t0 = this.following((offset).wrapping_sub(1i32))?;
            let mut boundary: i32 = _t0;
            if boundary == -1i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(boundary == offset)
        }

        #[java_method(name = "current", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn current(&self) -> Result<i32> {
            panic!("stub: java/text/BreakIterator.current:()I")
        }

        #[java_method(name = "getText", descriptor = "()Ljava/text/CharacterIterator;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getText(&self) -> Result<Object> {
            panic!("stub: java/text/BreakIterator.getText:()Ljava/text/CharacterIterator;")
        }

        #[java_method(name = "setText", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: setText(Ljava/lang/String;)V
        pub fn setText_str(&self, mut newText: String) -> Result<()> {
            let this = self;
            this.setText_charac(Object::from_any(StringCharacterIterator::new_str(Clone::clone(&newText))?.clone()))?;
            Ok(())
        }

        #[java_method(name = "setText", descriptor = "(Ljava/text/CharacterIterator;)V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn setText_charac(&self, arg0: Object) -> Result<()> {
            panic!("stub: java/text/BreakIterator.setText:(Ljava/text/CharacterIterator;)V")
        }

        #[java_method(name = "getWordInstance", descriptor = "()Ljava/text/BreakIterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getWordInstance() -> Result<BreakIterator> {
            panic!("stub: java/text/BreakIterator.getWordInstance:()Ljava/text/BreakIterator;")
        }

        #[java_method(name = "getWordInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/BreakIterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getWordInstance(Ljava/util/Locale;)Ljava/text/BreakIterator;
        pub fn getWordInstance_locale(mut locale: Locale) -> Result<BreakIterator> {
            let _t0: BreakIterator = BreakIterator::getBreakInstance(Clone::clone(&locale), 1i32)?;
            Ok(_t0)
        }

        #[java_method(name = "getLineInstance", descriptor = "()Ljava/text/BreakIterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLineInstance() -> Result<BreakIterator> {
            panic!("stub: java/text/BreakIterator.getLineInstance:()Ljava/text/BreakIterator;")
        }

        #[java_method(name = "getLineInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/BreakIterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLineInstance_locale(locale: Locale) -> Result<BreakIterator> {
            panic!("stub: java/text/BreakIterator.getLineInstance:(Ljava/util/Locale;)Ljava/text/BreakIterator;")
        }

        #[java_method(name = "getCharacterInstance", descriptor = "()Ljava/text/BreakIterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCharacterInstance() -> Result<BreakIterator> {
            panic!("stub: java/text/BreakIterator.getCharacterInstance:()Ljava/text/BreakIterator;")
        }

        #[java_method(name = "getCharacterInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/BreakIterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCharacterInstance_locale(locale: Locale) -> Result<BreakIterator> {
            panic!("stub: java/text/BreakIterator.getCharacterInstance:(Ljava/util/Locale;)Ljava/text/BreakIterator;")
        }

        #[java_method(name = "getSentenceInstance", descriptor = "()Ljava/text/BreakIterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSentenceInstance() -> Result<BreakIterator> {
            panic!("stub: java/text/BreakIterator.getSentenceInstance:()Ljava/text/BreakIterator;")
        }

        #[java_method(name = "getSentenceInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/BreakIterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSentenceInstance_locale(locale: Locale) -> Result<BreakIterator> {
            panic!("stub: java/text/BreakIterator.getSentenceInstance:(Ljava/util/Locale;)Ljava/text/BreakIterator;")
        }

        #[java_method(name = "getBreakInstance", descriptor = "(Ljava/util/Locale;I)Ljava/text/BreakIterator;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBreakInstance(mut locale: Locale, mut type_: i32) -> Result<BreakIterator> {
            let _t0 = Clone::clone(&BreakIterator::iterCache().borrow()[type_ as usize]).get()?;
            let mut cache = (_t0).downcast::<BreakIterator_BreakIteratorCache>();
            let _t1 = cache.getLocale()?;
            let _t2 = _t1.equals(Object::from_any(locale.clone()))?;
            if _t2 {
                let _t3 = cache.createBreakInstance()?;
                return Ok(_t3);
            }
            let _t3: BreakIterator = BreakIterator::createBreakInstance_locale_i(Clone::clone(&locale), type_)?;
            let mut cache: BreakIterator = _t3;
            let mut cache = BreakIterator_BreakIteratorCache::new(Clone::clone(&locale), Clone::clone(&cache))?;
            BreakIterator::iterCache().borrow_mut()[type_ as usize] = Clone::clone(&SoftReference::<Object>::new_obj(Object::from_any(cache.clone()))?);
            Ok(cache)
        }

        #[java_method(name = "createBreakInstance", descriptor = "(Ljava/util/Locale;I)Ljava/text/BreakIterator;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: createBreakInstance(Ljava/util/Locale;I)Ljava/text/BreakIterator;
        pub fn createBreakInstance_locale_i(mut locale: Locale, mut type_: i32) -> Result<BreakIterator> {
            let _t0: LocaleProviderAdapter = LocaleProviderAdapter::getAdapter(Default::default(), Clone::clone(&locale))?;
            let mut adapter: LocaleProviderAdapter = _t0;
            let _t1: BreakIterator = BreakIterator::createBreakInstance_locale_locale_i(Clone::clone(&adapter), Clone::clone(&locale), type_)?;
            let mut iterator: BreakIterator = _t1;
            if _is_jnull(&iterator) {
                let _t2: LocaleProviderAdapter = LocaleProviderAdapter::forJRE()?;
                let _t3: BreakIterator = BreakIterator::createBreakInstance_locale_locale_i(Clone::clone(&_t2), Clone::clone(&locale), type_)?;
                iterator = _t3;
            }
            Ok(iterator)
        }

        #[java_method(name = "createBreakInstance", descriptor = "(Lsun/util/locale/provider/LocaleProviderAdapter;Ljava/util/Locale;I)Ljava/text/BreakIterator;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createBreakInstance_locale_locale_i(adapter: LocaleProviderAdapter, locale: Locale, type_: i32) -> Result<BreakIterator> {
            panic!("stub: java/text/BreakIterator.createBreakInstance:(Lsun/util/locale/provider/LocaleProviderAdapter;Ljava/util/Locale;I)Ljava/text/BreakIterator;")
        }

        #[java_method(name = "getAvailableLocales", descriptor = "()[Ljava/util/Locale;", access = "public", modifiers = "static synchronized", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAvailableLocales() -> Result<Rc<RefCell<Vec<Locale>>>> {
            panic!("stub: java/text/BreakIterator.getAvailableLocales:()[Ljava/util/Locale;")
        }
    }
}
