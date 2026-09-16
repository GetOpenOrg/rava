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
use crate::jdk::internal::misc::Unsafe;
use crate::jdk::internal::util::StaticProperty;

impl From<Properties> for Hashtable<Object, Object> {
    fn from(v: Properties) -> Hashtable<Object, Object> { v.__into_super() }
}

impl From<Properties> for Dictionary<Object, Object> {
    fn from(v: Properties) -> Dictionary<Object, Object> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/Properties"]
    #[super_class       = "java/util/Hashtable"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/util/Hashtable<Ljava/lang/Object;Ljava/lang/Object;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Properties.java"]
    #[inner_classes     = "java/util/Properties$LineReader:java/util/Properties:LineReader:10;java/util/Collections$SynchronizedSet:java/util/Collections:SynchronizedSet:8;java/util/Properties$EntrySet:java/util/Properties:EntrySet:10;java/util/Map$Entry:java/util/Map:Entry:1545;java/util/concurrent/ConcurrentHashMap$KeySetView:java/util/concurrent/ConcurrentHashMap:KeySetView:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Hashtable<Object, Object>"]
    #[superclass_fields(table: Rc<RefCell<Vec<Object>>>, count: i32, threshold: i32, loadFactor: f32, modCount: i32, keySet: Object, entrySet: Object, values: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Object;java/util/Dictionary;java/util/Hashtable;java/util/Map;java/util/Properties"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Properties {
        #[cfg_attr(any(), java_field(name = "defaults", descriptor = "Ljava/util/Properties;", access = "protected", modifiers = "volatile", is_static = false))]
        pub defaults: Properties,
        #[cfg_attr(any(), java_field(name = "map", descriptor = "Ljava/util/concurrent/ConcurrentHashMap;", access = "private", modifiers = "volatile transient", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap<Ljava/lang/Object;Ljava/lang/Object;>;"))]
        pub map: ConcurrentHashMap<Object, Object>,
    }

    impl Properties {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "4112578634029874840"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            4112578634029874840i64
        }

        #[cfg_attr(any(), java_field(name = "UNSAFE", descriptor = "Ljdk/internal/misc/Unsafe;", access = "private", modifiers = "static final", is_static = true))]
        // static field: UNSAFE:Ljdk/internal/misc/Unsafe;
        pub fn UNSAFE() -> Unsafe {
            panic!("stub: java/util/Properties.UNSAFE:Ljdk/internal/misc/Unsafe;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/Properties.<init>:()V")
        }

        #[java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i(initialCapacity: i32) -> Result<Self> {
            panic!("stub: java/util/Properties.<init>:(I)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Properties;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_proper(defaults: Properties) -> Result<Self> {
            panic!("stub: java/util/Properties.<init>:(Ljava/util/Properties;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Properties;I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_proper_i(defaults: Properties, initialCapacity: i32) -> Result<Self> {
            panic!("stub: java/util/Properties.<init>:(Ljava/util/Properties;I)V")
        }

        #[java_method(name = "setProperty", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setProperty(&self, mut key: String, mut value: String) -> Result<Object> {
            let this = self;
            let _t0 = this.put(Object::from_any(key.clone()), Object::from_any(value.clone()))?;
            Ok(_t0)
        }

        #[java_method(name = "load", descriptor = "(Ljava/io/Reader;)V", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn load_reader(&self, reader: Object) -> Result<()> {
            panic!("stub: java/util/Properties.load:(Ljava/io/Reader;)V")
        }

        #[java_method(name = "load", descriptor = "(Ljava/io/InputStream;)V", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn load_inputs(&self, inStream: InputStream) -> Result<()> {
            panic!("stub: java/util/Properties.load:(Ljava/io/InputStream;)V")
        }

        #[java_method(name = "load0", descriptor = "(Ljava/util/Properties$LineReader;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn load0(&self, lr: Object) -> Result<()> {
            panic!("stub: java/util/Properties.load0:(Ljava/util/Properties$LineReader;)V")
        }

        #[java_method(name = "loadConvert", descriptor = "([CIILjava/lang/StringBuilder;)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn loadConvert(&self, in_: Rc<RefCell<Vec<u16>>>, off: i32, len: i32, out: StringBuilder) -> Result<String> {
            panic!("stub: java/util/Properties.loadConvert:([CIILjava/lang/StringBuilder;)Ljava/lang/String;")
        }

        #[java_method(name = "saveConvert", descriptor = "(Ljava/lang/String;ZZ)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn saveConvert(&self, theString: String, escapeSpace: bool, escapeUnicode: bool) -> Result<String> {
            panic!("stub: java/util/Properties.saveConvert:(Ljava/lang/String;ZZ)Ljava/lang/String;")
        }

        #[java_method(name = "writeComments", descriptor = "(Ljava/io/BufferedWriter;Ljava/lang/String;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeComments(bw: BufferedWriter, comments: String) -> Result<()> {
            panic!("stub: java/util/Properties.writeComments:(Ljava/io/BufferedWriter;Ljava/lang/String;)V")
        }

        #[java_method(name = "save", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn save(&self, out: OutputStream, comments: String) -> Result<()> {
            panic!("stub: java/util/Properties.save:(Ljava/io/OutputStream;Ljava/lang/String;)V")
        }

        #[java_method(name = "store", descriptor = "(Ljava/io/Writer;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn store_writer_str(&self, writer: Writer, comments: String) -> Result<()> {
            panic!("stub: java/util/Properties.store:(Ljava/io/Writer;Ljava/lang/String;)V")
        }

        #[java_method(name = "store", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn store_output_str(&self, out: OutputStream, comments: String) -> Result<()> {
            panic!("stub: java/util/Properties.store:(Ljava/io/OutputStream;Ljava/lang/String;)V")
        }

        #[java_method(name = "store0", descriptor = "(Ljava/io/BufferedWriter;Ljava/lang/String;Z)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn store0(&self, bw: BufferedWriter, comments: String, escUnicode: bool) -> Result<()> {
            panic!("stub: java/util/Properties.store0:(Ljava/io/BufferedWriter;Ljava/lang/String;Z)V")
        }

        #[java_method(name = "writeDateComment", descriptor = "(Ljava/io/BufferedWriter;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeDateComment(bw: BufferedWriter) -> Result<()> {
            panic!("stub: java/util/Properties.writeDateComment:(Ljava/io/BufferedWriter;)V")
        }

        #[java_method(name = "loadFromXML", descriptor = "(Ljava/io/InputStream;)V", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/util/InvalidPropertiesFormatException")]
        pub fn loadFromXML(&self, in_: InputStream) -> Result<()> {
            panic!("stub: java/util/Properties.loadFromXML:(Ljava/io/InputStream;)V")
        }

        #[java_method(name = "storeToXML", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn storeToXML_output_str(&self, os: OutputStream, comment: String) -> Result<()> {
            panic!("stub: java/util/Properties.storeToXML:(Ljava/io/OutputStream;Ljava/lang/String;)V")
        }

        #[java_method(name = "storeToXML", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn storeToXML_output_str_str(&self, os: OutputStream, comment: String, encoding: String) -> Result<()> {
            panic!("stub: java/util/Properties.storeToXML:(Ljava/io/OutputStream;Ljava/lang/String;Ljava/lang/String;)V")
        }

        #[java_method(name = "storeToXML", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;Ljava/nio/charset/Charset;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn storeToXML_output_str_charse(&self, os: OutputStream, comment: String, charset: Charset) -> Result<()> {
            panic!("stub: java/util/Properties.storeToXML:(Ljava/io/OutputStream;Ljava/lang/String;Ljava/nio/charset/Charset;)V")
        }

        #[java_method(name = "getProperty", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getProperty(Ljava/lang/String;)Ljava/lang/String;
        pub fn getProperty_str(&self, mut key: String) -> Result<String> {
            let this = self;
            let _t0 = this.__get_map().get(Object::from_any(key.clone()))?;
            let mut oval: Object = _t0;
            let mut sval = (if (oval.is_instance_of("java/lang/String")) { (oval).downcast::<String>() } else { Default::default() });
            let mut _merged3: String;
            if _is_jnull(&sval) {
                let mut defaults = this.__get_defaults();
                let mut _merged2: String;
                if !_is_jnull(&this.__get_defaults()) {
                    let _t1 = defaults.getProperty_str(Clone::clone(&key))?;
                    _merged2 = _t1;
                } else {
                    _merged2 = sval;
                }
                _merged3 = _merged2;
            } else {
                _merged3 = sval;
            }
            Ok(_merged3)
        }

        #[java_method(name = "getProperty", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getProperty_str_str(&self, key: String, defaultValue: String) -> Result<String> {
            panic!("stub: java/util/Properties.getProperty:(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "propertyNames", descriptor = "()Ljava/util/Enumeration;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Enumeration<*>;")]
        pub fn propertyNames(&self) -> Result<Object> {
            panic!("stub: java/util/Properties.propertyNames:()Ljava/util/Enumeration;")
        }

        #[java_method(name = "stringPropertyNames", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/lang/String;>;")]
        pub fn stringPropertyNames(&self) -> Result<Object> {
            panic!("stub: java/util/Properties.stringPropertyNames:()Ljava/util/Set;")
        }

        #[java_method(name = "list", descriptor = "(Ljava/io/PrintStream;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn list_prints(&self, out: PrintStream) -> Result<()> {
            panic!("stub: java/util/Properties.list:(Ljava/io/PrintStream;)V")
        }

        #[java_method(name = "list", descriptor = "(Ljava/io/PrintWriter;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn list_printw(&self, out: Object) -> Result<()> {
            panic!("stub: java/util/Properties.list:(Ljava/io/PrintWriter;)V")
        }

        #[java_method(name = "enumerate", descriptor = "(Ljava/util/Map;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/lang/String;Ljava/lang/Object;>;)V")]
        pub fn enumerate(&self, h: Object) -> Result<()> {
            panic!("stub: java/util/Properties.enumerate:(Ljava/util/Map;)V")
        }

        #[java_method(name = "enumerateStringProperties", descriptor = "(Ljava/util/Map;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/lang/String;Ljava/lang/String;>;)V")]
        pub fn enumerateStringProperties(&self, h: Object) -> Result<()> {
            panic!("stub: java/util/Properties.enumerateStringProperties:(Ljava/util/Map;)V")
        }

        #[java_method(name = "size", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn size(&self) -> Result<i32> {
            panic!("stub: java/util/Properties.size:()I")
        }

        #[java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty(&self) -> Result<bool> {
            panic!("stub: java/util/Properties.isEmpty:()Z")
        }

        #[java_method(name = "keys", descriptor = "()Ljava/util/Enumeration;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Enumeration<Ljava/lang/Object;>;")]
        pub fn keys(&self) -> Result<Object> {
            panic!("stub: java/util/Properties.keys:()Ljava/util/Enumeration;")
        }

        #[java_method(name = "elements", descriptor = "()Ljava/util/Enumeration;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Enumeration<Ljava/lang/Object;>;")]
        pub fn elements(&self) -> Result<Object> {
            panic!("stub: java/util/Properties.elements:()Ljava/util/Enumeration;")
        }

        #[java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contains(&self, value: Object) -> Result<bool> {
            panic!("stub: java/util/Properties.contains:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "containsValue", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsValue(&self, value: Object) -> Result<bool> {
            panic!("stub: java/util/Properties.containsValue:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "containsKey", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsKey(&self, key: Object) -> Result<bool> {
            panic!("stub: java/util/Properties.containsKey:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "get", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get(&self, key: Object) -> Result<Object> {
            panic!("stub: java/util/Properties.get:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "put", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put(&self, mut key: Object, mut value: Object) -> Result<Object> {
            let this = self;
            let _t0 = this.__get_map().put(Clone::clone(&key), Clone::clone(&value))?;
            Ok(_t0)
        }

        #[java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remove_obj(&self, key: Object) -> Result<Object> {
            panic!("stub: java/util/Properties.remove:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "putAll", descriptor = "(Ljava/util/Map;)V", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<**>;)V")]
        pub fn putAll(&self, t: Object) -> Result<()> {
            panic!("stub: java/util/Properties.putAll:(Ljava/util/Map;)V")
        }

        #[java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<()> {
            panic!("stub: java/util/Properties.clear:()V")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "keySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/lang/Object;>;")]
        pub fn keySet(&self) -> Result<Object> {
            panic!("stub: java/util/Properties.keySet:()Ljava/util/Set;")
        }

        #[java_method(name = "values", descriptor = "()Ljava/util/Collection;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Collection<Ljava/lang/Object;>;")]
        pub fn values(&self) -> Result<Object> {
            panic!("stub: java/util/Properties.values:()Ljava/util/Collection;")
        }

        #[java_method(name = "entrySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/util/Map$Entry<Ljava/lang/Object;Ljava/lang/Object;>;>;")]
        pub fn entrySet(&self) -> Result<Object> {
            panic!("stub: java/util/Properties.entrySet:()Ljava/util/Set;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/Properties.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "getOrDefault", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOrDefault(&self, key: Object, defaultValue: Object) -> Result<Object> {
            panic!("stub: java/util/Properties.getOrDefault:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "forEach", descriptor = "(Ljava/util/function/BiConsumer;)V", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiConsumer<-Ljava/lang/Object;-Ljava/lang/Object;>;)V")]
        pub fn forEach(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/Properties.forEach:(Ljava/util/function/BiConsumer;)V")
        }

        #[java_method(name = "replaceAll", descriptor = "(Ljava/util/function/BiFunction;)V", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiFunction<-Ljava/lang/Object;-Ljava/lang/Object;*>;)V")]
        pub fn replaceAll(&self, function: Object) -> Result<()> {
            panic!("stub: java/util/Properties.replaceAll:(Ljava/util/function/BiFunction;)V")
        }

        #[java_method(name = "putIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putIfAbsent(&self, key: Object, value: Object) -> Result<Object> {
            panic!("stub: java/util/Properties.putIfAbsent:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "remove", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remove_obj_obj(&self, key: Object, value: Object) -> Result<bool> {
            panic!("stub: java/util/Properties.remove:(Ljava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn replace_obj_obj_obj(&self, key: Object, oldValue: Object, newValue: Object) -> Result<bool> {
            panic!("stub: java/util/Properties.replace:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn replace_obj_obj(&self, key: Object, value: Object) -> Result<Object> {
            panic!("stub: java/util/Properties.replace:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "computeIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;Ljava/util/function/Function<-Ljava/lang/Object;*>;)Ljava/lang/Object;")]
        pub fn computeIfAbsent(&self, key: Object, mappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/Properties.computeIfAbsent:(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;")
        }

        #[java_method(name = "computeIfPresent", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;Ljava/util/function/BiFunction<-Ljava/lang/Object;-Ljava/lang/Object;*>;)Ljava/lang/Object;")]
        pub fn computeIfPresent(&self, key: Object, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/Properties.computeIfPresent:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "compute", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;Ljava/util/function/BiFunction<-Ljava/lang/Object;-Ljava/lang/Object;*>;)Ljava/lang/Object;")]
        pub fn compute(&self, key: Object, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/Properties.compute:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "merge", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction<-Ljava/lang/Object;-Ljava/lang/Object;*>;)Ljava/lang/Object;")]
        pub fn merge(&self, key: Object, value: Object, remappingFunction: Object) -> Result<Object> {
            panic!("stub: java/util/Properties.merge:(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "rehash", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rehash(&self) -> Result<()> {
            panic!("stub: java/util/Properties.rehash:()V")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/util/Properties.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "writeHashtable", descriptor = "(Ljava/io/ObjectOutputStream;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeHashtable(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/Properties.writeHashtable:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "readHashtable", descriptor = "(Ljava/io/ObjectInputStream;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readHashtable(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/Properties.readHashtable:(Ljava/io/ObjectInputStream;)V")
        }
    }
}
