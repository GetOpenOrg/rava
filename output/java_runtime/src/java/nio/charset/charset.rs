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
use crate::jdk::internal::misc::VM;
use crate::jdk::internal::util::StaticProperty;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/nio/charset/Charset"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/lang/Comparable"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "Ljava/lang/Object;Ljava/lang/Comparable<Ljava/nio/charset/Charset;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Charset.java"]
    #[inner_classes     = "java/nio/charset/Charset$1:::0;java/nio/charset/Charset$ThreadTrackHolder:java/nio/charset/Charset:ThreadTrackHolder:10;java/nio/charset/Charset$2:::0;java/nio/charset/Charset$ExtendedProviderHolder:java/nio/charset/Charset:ExtendedProviderHolder:10;java/nio/charset/Charset$3:::0;java/nio/charset/Charset$ExtendedProviderHolder$1:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Comparable;java/lang/Object;java/nio/charset/Charset"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Charset {
        #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub name: String,
        #[cfg_attr(any(), java_field(name = "aliases", descriptor = "[Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub aliases: Rc<RefCell<Vec<String>>>,
        #[cfg_attr(any(), java_field(name = "aliasSet", descriptor = "Ljava/util/Set;", access = "private", modifiers = "", is_static = false, generic_signature = "Ljava/util/Set<Ljava/lang/String;>;"))]
        pub aliasSet: Object,
    }

    impl Charset {
        #[cfg_attr(any(), java_field(name = "standardProvider", descriptor = "Ljava/nio/charset/spi/CharsetProvider;", access = "private", modifiers = "static final", is_static = true))]
        // static field: standardProvider:Ljava/nio/charset/spi/CharsetProvider;
        pub fn standardProvider() -> Object {
            panic!("stub: java/nio/charset/Charset.standardProvider:Ljava/nio/charset/spi/CharsetProvider;")
        }

        #[cfg_attr(any(), java_field(name = "zeroAliases", descriptor = "[Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: zeroAliases:[Ljava/lang/String;
        pub fn zeroAliases() -> Rc<RefCell<Vec<String>>> {
            Rc::new(RefCell::new(Vec::new()))
        }

        #[cfg_attr(any(), java_field(name = "cache1", descriptor = "[Ljava/lang/Object;", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: cache1:[Ljava/lang/Object;
        pub fn cache1() -> Rc<RefCell<Vec<Object>>> {
            panic!("stub: java/nio/charset/Charset.cache1:[Ljava/lang/Object;")
        }

        #[cfg_attr(any(), java_field(name = "cache2", descriptor = "[Ljava/lang/Object;", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: cache2:[Ljava/lang/Object;
        pub fn cache2() -> Rc<RefCell<Vec<Object>>> {
            panic!("stub: java/nio/charset/Charset.cache2:[Ljava/lang/Object;")
        }

        #[cfg_attr(any(), java_field(name = "defaultCharset", descriptor = "Ljava/nio/charset/Charset;", access = "private", modifiers = "static", is_static = true))]
        // static field: defaultCharset:Ljava/nio/charset/Charset;
        pub fn defaultCharset_field() -> Charset {
            panic!("stub: java/nio/charset/Charset.defaultCharset:Ljava/nio/charset/Charset;")
        }

        #[java_method(name = "checkName", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkName(s: String) -> Result<()> {
            panic!("stub: java/nio/charset/Charset.checkName:(Ljava/lang/String;)V")
        }

        #[java_method(name = "cache", descriptor = "(Ljava/lang/String;Ljava/nio/charset/Charset;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn cache(charsetName: String, cs: Charset) -> Result<()> {
            panic!("stub: java/nio/charset/Charset.cache:(Ljava/lang/String;Ljava/nio/charset/Charset;)V")
        }

        #[java_method(name = "providers", descriptor = "()Ljava/util/Iterator;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<Ljava/nio/charset/spi/CharsetProvider;>;")]
        pub fn providers() -> Result<Object> {
            panic!("stub: java/nio/charset/Charset.providers:()Ljava/util/Iterator;")
        }

        #[java_method(name = "tryBeginLookup", descriptor = "()Ljava/lang/Object;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tryBeginLookup() -> Result<Object> {
            panic!("stub: java/nio/charset/Charset.tryBeginLookup:()Ljava/lang/Object;")
        }

        #[java_method(name = "endLookup", descriptor = "(Ljava/lang/Object;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn endLookup(key: Object) -> Result<()> {
            panic!("stub: java/nio/charset/Charset.endLookup:(Ljava/lang/Object;)V")
        }

        #[java_method(name = "lookupViaProviders", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lookupViaProviders(charsetName: String) -> Result<Charset> {
            panic!("stub: java/nio/charset/Charset.lookupViaProviders:(Ljava/lang/String;)Ljava/nio/charset/Charset;")
        }

        #[java_method(name = "lookupExtendedCharset", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lookupExtendedCharset(charsetName: String) -> Result<Charset> {
            panic!("stub: java/nio/charset/Charset.lookupExtendedCharset:(Ljava/lang/String;)Ljava/nio/charset/Charset;")
        }

        #[java_method(name = "lookup", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lookup(charsetName: String) -> Result<Charset> {
            panic!("stub: java/nio/charset/Charset.lookup:(Ljava/lang/String;)Ljava/nio/charset/Charset;")
        }

        #[java_method(name = "lookup2", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lookup2(charsetName: String) -> Result<Charset> {
            panic!("stub: java/nio/charset/Charset.lookup2:(Ljava/lang/String;)Ljava/nio/charset/Charset;")
        }

        #[java_method(name = "isSupported", descriptor = "(Ljava/lang/String;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSupported(charsetName: String) -> Result<bool> {
            panic!("stub: java/nio/charset/Charset.isSupported:(Ljava/lang/String;)Z")
        }

        #[java_method(name = "forName", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forName_str(charsetName: String) -> Result<Charset> {
            panic!("stub: java/nio/charset/Charset.forName:(Ljava/lang/String;)Ljava/nio/charset/Charset;")
        }

        #[java_method(name = "forName", descriptor = "(Ljava/lang/String;Ljava/nio/charset/Charset;)Ljava/nio/charset/Charset;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forName_str_charse(charsetName: String, fallback: Charset) -> Result<Charset> {
            panic!("stub: java/nio/charset/Charset.forName:(Ljava/lang/String;Ljava/nio/charset/Charset;)Ljava/nio/charset/Charset;")
        }

        #[java_method(name = "put", descriptor = "(Ljava/util/Iterator;Ljava/util/Map;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Iterator<Ljava/nio/charset/Charset;>;Ljava/util/Map<Ljava/lang/String;Ljava/nio/charset/Charset;>;)V")]
        pub fn put(i: Object, m: Object) -> Result<()> {
            panic!("stub: java/nio/charset/Charset.put:(Ljava/util/Iterator;Ljava/util/Map;)V")
        }

        #[java_method(name = "availableCharsets", descriptor = "()Ljava/util/SortedMap;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/SortedMap<Ljava/lang/String;Ljava/nio/charset/Charset;>;")]
        pub fn availableCharsets() -> Result<Object> {
            panic!("stub: java/nio/charset/Charset.availableCharsets:()Ljava/util/SortedMap;")
        }

        #[java_method(name = "defaultCharset", descriptor = "()Ljava/nio/charset/Charset;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn defaultCharset() -> Result<Charset> {
            panic!("stub: java/nio/charset/Charset.defaultCharset:()Ljava/nio/charset/Charset;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;[Ljava/lang/String;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(canonicalName: String, aliases: Rc<RefCell<Vec<String>>>) -> Result<Self> {
            panic!("stub: java/nio/charset/Charset.<init>:(Ljava/lang/String;[Ljava/lang/String;)V")
        }

        #[java_method(name = "name", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn name(&self) -> Result<String> {
            let this = self;
            Ok(this.__get_name())
        }

        #[java_method(name = "aliases", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/lang/String;>;")]
        pub fn aliases(&self) -> Result<Object> {
            panic!("stub: java/nio/charset/Charset.aliases:()Ljava/util/Set;")
        }

        #[java_method(name = "displayName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn displayName(&self) -> Result<String> {
            panic!("stub: java/nio/charset/Charset.displayName:()Ljava/lang/String;")
        }

        #[java_method(name = "isRegistered", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isRegistered(&self) -> Result<bool> {
            panic!("stub: java/nio/charset/Charset.isRegistered:()Z")
        }

        #[java_method(name = "displayName", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn displayName_locale(&self, locale: Locale) -> Result<String> {
            panic!("stub: java/nio/charset/Charset.displayName:(Ljava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "contains", descriptor = "(Ljava/nio/charset/Charset;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn contains(&self, arg0: Charset) -> Result<bool> {
            panic!("stub: java/nio/charset/Charset.contains:(Ljava/nio/charset/Charset;)Z")
        }

        #[java_method(name = "newDecoder", descriptor = "()Ljava/nio/charset/CharsetDecoder;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn newDecoder(&self) -> Result<Object> {
            panic!("stub: java/nio/charset/Charset.newDecoder:()Ljava/nio/charset/CharsetDecoder;")
        }

        #[java_method(name = "newEncoder", descriptor = "()Ljava/nio/charset/CharsetEncoder;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn newEncoder(&self) -> Result<CharsetEncoder> {
            panic!("stub: java/nio/charset/Charset.newEncoder:()Ljava/nio/charset/CharsetEncoder;")
        }

        #[java_method(name = "canEncode", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn canEncode(&self) -> Result<bool> {
            panic!("stub: java/nio/charset/Charset.canEncode:()Z")
        }

        #[java_method(name = "decode", descriptor = "(Ljava/nio/ByteBuffer;)Ljava/nio/CharBuffer;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn decode(&self, bb: ByteBuffer) -> Result<CharBuffer> {
            panic!("stub: java/nio/charset/Charset.decode:(Ljava/nio/ByteBuffer;)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "encode", descriptor = "(Ljava/nio/CharBuffer;)Ljava/nio/ByteBuffer;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn encode_charbu(&self, cb: CharBuffer) -> Result<ByteBuffer> {
            panic!("stub: java/nio/charset/Charset.encode:(Ljava/nio/CharBuffer;)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "encode", descriptor = "(Ljava/lang/String;)Ljava/nio/ByteBuffer;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn encode_str(&self, str: String) -> Result<ByteBuffer> {
            panic!("stub: java/nio/charset/Charset.encode:(Ljava/lang/String;)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/nio/charset/Charset;)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, mut that: Charset) -> Result<i32> {
            let this = self;
            let _t0 = this.name()?;
            let _t1 = that.name()?;
            let _t2 = _t0.compareToIgnoreCase(Clone::clone(&_t1))?;
            Ok(_t2)
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, ob: Object) -> Result<bool> {
            panic!("stub: java/nio/charset/Charset.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }
    }
}
