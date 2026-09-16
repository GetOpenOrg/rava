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
use crate::jdk::internal::util::*;
use crate::java::text::Normalizer;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/util/ByteArray"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ByteArray.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;jdk/internal/util/ByteArray"]

    pub struct ByteArray;

    impl ByteArray {
        #[cfg_attr(any(), java_field(name = "SHORT", descriptor = "Ljava/lang/invoke/VarHandle;", access = "private", modifiers = "static final", is_static = true))]
        // static field: SHORT:Ljava/lang/invoke/VarHandle;
        pub fn SHORT() -> Object {
            panic!("stub: jdk/internal/util/ByteArray.SHORT:Ljava/lang/invoke/VarHandle;")
        }

        #[cfg_attr(any(), java_field(name = "CHAR", descriptor = "Ljava/lang/invoke/VarHandle;", access = "private", modifiers = "static final", is_static = true))]
        // static field: CHAR:Ljava/lang/invoke/VarHandle;
        pub fn CHAR() -> Object {
            panic!("stub: jdk/internal/util/ByteArray.CHAR:Ljava/lang/invoke/VarHandle;")
        }

        #[cfg_attr(any(), java_field(name = "INT", descriptor = "Ljava/lang/invoke/VarHandle;", access = "private", modifiers = "static final", is_static = true))]
        // static field: INT:Ljava/lang/invoke/VarHandle;
        pub fn INT() -> Object {
            panic!("stub: jdk/internal/util/ByteArray.INT:Ljava/lang/invoke/VarHandle;")
        }

        #[cfg_attr(any(), java_field(name = "FLOAT", descriptor = "Ljava/lang/invoke/VarHandle;", access = "private", modifiers = "static final", is_static = true))]
        // static field: FLOAT:Ljava/lang/invoke/VarHandle;
        pub fn FLOAT() -> Object {
            panic!("stub: jdk/internal/util/ByteArray.FLOAT:Ljava/lang/invoke/VarHandle;")
        }

        #[cfg_attr(any(), java_field(name = "LONG", descriptor = "Ljava/lang/invoke/VarHandle;", access = "private", modifiers = "static final", is_static = true))]
        // static field: LONG:Ljava/lang/invoke/VarHandle;
        pub fn LONG() -> Object {
            panic!("stub: jdk/internal/util/ByteArray.LONG:Ljava/lang/invoke/VarHandle;")
        }

        #[cfg_attr(any(), java_field(name = "DOUBLE", descriptor = "Ljava/lang/invoke/VarHandle;", access = "private", modifiers = "static final", is_static = true))]
        // static field: DOUBLE:Ljava/lang/invoke/VarHandle;
        pub fn DOUBLE() -> Object {
            panic!("stub: jdk/internal/util/ByteArray.DOUBLE:Ljava/lang/invoke/VarHandle;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: jdk/internal/util/ByteArray.<init>:()V")
        }

        #[java_method(name = "getBoolean", descriptor = "([BI)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBoolean(array: Rc<RefCell<Vec<i8>>>, offset: i32) -> Result<bool> {
            panic!("stub: jdk/internal/util/ByteArray.getBoolean:([BI)Z")
        }

        #[java_method(name = "getChar", descriptor = "([BI)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getChar(array: Rc<RefCell<Vec<i8>>>, offset: i32) -> Result<u16> {
            panic!("stub: jdk/internal/util/ByteArray.getChar:([BI)C")
        }

        #[java_method(name = "getShort", descriptor = "([BI)S", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShort(array: Rc<RefCell<Vec<i8>>>, offset: i32) -> Result<i16> {
            panic!("stub: jdk/internal/util/ByteArray.getShort:([BI)S")
        }

        #[java_method(name = "getUnsignedShort", descriptor = "([BI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getUnsignedShort(array: Rc<RefCell<Vec<i8>>>, offset: i32) -> Result<i32> {
            panic!("stub: jdk/internal/util/ByteArray.getUnsignedShort:([BI)I")
        }

        #[java_method(name = "getInt", descriptor = "([BI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInt(array: Rc<RefCell<Vec<i8>>>, offset: i32) -> Result<i32> {
            panic!("stub: jdk/internal/util/ByteArray.getInt:([BI)I")
        }

        #[java_method(name = "getFloat", descriptor = "([BI)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFloat(array: Rc<RefCell<Vec<i8>>>, offset: i32) -> Result<f32> {
            panic!("stub: jdk/internal/util/ByteArray.getFloat:([BI)F")
        }

        #[java_method(name = "getFloatRaw", descriptor = "([BI)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFloatRaw(array: Rc<RefCell<Vec<i8>>>, offset: i32) -> Result<f32> {
            panic!("stub: jdk/internal/util/ByteArray.getFloatRaw:([BI)F")
        }

        #[java_method(name = "getLong", descriptor = "([BI)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLong(array: Rc<RefCell<Vec<i8>>>, offset: i32) -> Result<i64> {
            panic!("stub: jdk/internal/util/ByteArray.getLong:([BI)J")
        }

        #[java_method(name = "getDouble", descriptor = "([BI)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDouble(array: Rc<RefCell<Vec<i8>>>, offset: i32) -> Result<f64> {
            panic!("stub: jdk/internal/util/ByteArray.getDouble:([BI)D")
        }

        #[java_method(name = "getDoubleRaw", descriptor = "([BI)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDoubleRaw(array: Rc<RefCell<Vec<i8>>>, offset: i32) -> Result<f64> {
            panic!("stub: jdk/internal/util/ByteArray.getDoubleRaw:([BI)D")
        }

        #[java_method(name = "setBoolean", descriptor = "([BIZ)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setBoolean(array: Rc<RefCell<Vec<i8>>>, offset: i32, value: bool) -> Result<()> {
            panic!("stub: jdk/internal/util/ByteArray.setBoolean:([BIZ)V")
        }

        #[java_method(name = "setChar", descriptor = "([BIC)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setChar(array: Rc<RefCell<Vec<i8>>>, offset: i32, value: u16) -> Result<()> {
            panic!("stub: jdk/internal/util/ByteArray.setChar:([BIC)V")
        }

        #[java_method(name = "setShort", descriptor = "([BIS)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setShort(array: Rc<RefCell<Vec<i8>>>, offset: i32, value: i16) -> Result<()> {
            panic!("stub: jdk/internal/util/ByteArray.setShort:([BIS)V")
        }

        #[java_method(name = "setUnsignedShort", descriptor = "([BII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setUnsignedShort(array: Rc<RefCell<Vec<i8>>>, offset: i32, value: i32) -> Result<()> {
            panic!("stub: jdk/internal/util/ByteArray.setUnsignedShort:([BII)V")
        }

        #[java_method(name = "setInt", descriptor = "([BII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setInt(array: Rc<RefCell<Vec<i8>>>, offset: i32, value: i32) -> Result<()> {
            panic!("stub: jdk/internal/util/ByteArray.setInt:([BII)V")
        }

        #[java_method(name = "setFloat", descriptor = "([BIF)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setFloat(array: Rc<RefCell<Vec<i8>>>, offset: i32, value: f32) -> Result<()> {
            panic!("stub: jdk/internal/util/ByteArray.setFloat:([BIF)V")
        }

        #[java_method(name = "setFloatRaw", descriptor = "([BIF)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setFloatRaw(array: Rc<RefCell<Vec<i8>>>, offset: i32, value: f32) -> Result<()> {
            panic!("stub: jdk/internal/util/ByteArray.setFloatRaw:([BIF)V")
        }

        #[java_method(name = "setLong", descriptor = "([BIJ)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setLong(array: Rc<RefCell<Vec<i8>>>, offset: i32, value: i64) -> Result<()> {
            panic!("stub: jdk/internal/util/ByteArray.setLong:([BIJ)V")
        }

        #[java_method(name = "setDouble", descriptor = "([BID)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDouble(array: Rc<RefCell<Vec<i8>>>, offset: i32, value: f64) -> Result<()> {
            panic!("stub: jdk/internal/util/ByteArray.setDouble:([BID)V")
        }

        #[java_method(name = "setDoubleRaw", descriptor = "([BID)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDoubleRaw(array: Rc<RefCell<Vec<i8>>>, offset: i32, value: f64) -> Result<()> {
            panic!("stub: jdk/internal/util/ByteArray.setDoubleRaw:([BID)V")
        }

        #[java_method(name = "create", descriptor = "(Ljava/lang/Class;)Ljava/lang/invoke/VarHandle;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Ljava/lang/invoke/VarHandle;")]
        pub fn create(viewArrayClass: Object) -> Result<Object> {
            panic!("stub: jdk/internal/util/ByteArray.create:(Ljava/lang/Class;)Ljava/lang/invoke/VarHandle;")
        }
    }
}
