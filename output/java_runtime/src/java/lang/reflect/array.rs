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
    #[binary_name       = "java/lang/reflect/Array"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Array.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/lang/reflect/Array"]

    pub struct Array;

    impl Array {
        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/reflect/Array.<init>:()V")
        }

        #[java_method(name = "newInstance", descriptor = "(Ljava/lang/Class;I)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NegativeArraySizeException", generic_signature = "(Ljava/lang/Class<*>;I)Ljava/lang/Object;")]
        // java: newInstance(Ljava/lang/Class;I)Ljava/lang/Object;
        pub fn newInstance_class_i(mut componentType: Object, mut length: i32) -> Result<Object> {
            let _t0: Object = Array::newArray(Clone::clone(&componentType), length)?;
            Ok(_t0)
        }

        #[java_method(name = "newInstance", descriptor = "(Ljava/lang/Class;[I)Ljava/lang/Object;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/NegativeArraySizeException", generic_signature = "(Ljava/lang/Class<*>;[I)Ljava/lang/Object;")]
        pub fn newInstance_class_arr_i(componentType: Object, dimensions: Rc<RefCell<Vec<i32>>>) -> Result<Object> {
            panic!("stub: java/lang/reflect/Array.newInstance:(Ljava/lang/Class;[I)Ljava/lang/Object;")
        }

        #[native]
        #[java_native(name = "getLength", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException")]
        pub fn getLength(arg0: Object) -> Result<i32> {
            panic!("native: java/lang/reflect/Array.getLength:(Ljava/lang/Object;)I")
        }

        #[native]
        #[java_native(name = "get", descriptor = "(Ljava/lang/Object;I)Ljava/lang/Object;", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn get(arg0: Object, arg1: i32) -> Result<Object> {
            panic!("native: java/lang/reflect/Array.get:(Ljava/lang/Object;I)Ljava/lang/Object;")
        }

        #[native]
        #[java_native(name = "getBoolean", descriptor = "(Ljava/lang/Object;I)Z", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn getBoolean(arg0: Object, arg1: i32) -> Result<bool> {
            panic!("native: java/lang/reflect/Array.getBoolean:(Ljava/lang/Object;I)Z")
        }

        #[native]
        #[java_native(name = "getByte", descriptor = "(Ljava/lang/Object;I)B", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn getByte(arg0: Object, arg1: i32) -> Result<i8> {
            panic!("native: java/lang/reflect/Array.getByte:(Ljava/lang/Object;I)B")
        }

        #[native]
        #[java_native(name = "getChar", descriptor = "(Ljava/lang/Object;I)C", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn getChar(arg0: Object, arg1: i32) -> Result<u16> {
            panic!("native: java/lang/reflect/Array.getChar:(Ljava/lang/Object;I)C")
        }

        #[native]
        #[java_native(name = "getShort", descriptor = "(Ljava/lang/Object;I)S", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn getShort(arg0: Object, arg1: i32) -> Result<i16> {
            panic!("native: java/lang/reflect/Array.getShort:(Ljava/lang/Object;I)S")
        }

        #[native]
        #[java_native(name = "getInt", descriptor = "(Ljava/lang/Object;I)I", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn getInt(arg0: Object, arg1: i32) -> Result<i32> {
            panic!("native: java/lang/reflect/Array.getInt:(Ljava/lang/Object;I)I")
        }

        #[native]
        #[java_native(name = "getLong", descriptor = "(Ljava/lang/Object;I)J", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn getLong(arg0: Object, arg1: i32) -> Result<i64> {
            panic!("native: java/lang/reflect/Array.getLong:(Ljava/lang/Object;I)J")
        }

        #[native]
        #[java_native(name = "getFloat", descriptor = "(Ljava/lang/Object;I)F", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn getFloat(arg0: Object, arg1: i32) -> Result<f32> {
            panic!("native: java/lang/reflect/Array.getFloat:(Ljava/lang/Object;I)F")
        }

        #[native]
        #[java_native(name = "getDouble", descriptor = "(Ljava/lang/Object;I)D", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn getDouble(arg0: Object, arg1: i32) -> Result<f64> {
            panic!("native: java/lang/reflect/Array.getDouble:(Ljava/lang/Object;I)D")
        }

        #[native]
        #[java_native(name = "set", descriptor = "(Ljava/lang/Object;ILjava/lang/Object;)V", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn set(arg0: Object, arg1: i32, arg2: Object) -> Result<()> {
            panic!("native: java/lang/reflect/Array.set:(Ljava/lang/Object;ILjava/lang/Object;)V")
        }

        #[native]
        #[java_native(name = "setBoolean", descriptor = "(Ljava/lang/Object;IZ)V", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn setBoolean(arg0: Object, arg1: i32, arg2: bool) -> Result<()> {
            panic!("native: java/lang/reflect/Array.setBoolean:(Ljava/lang/Object;IZ)V")
        }

        #[native]
        #[java_native(name = "setByte", descriptor = "(Ljava/lang/Object;IB)V", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn setByte(arg0: Object, arg1: i32, arg2: i8) -> Result<()> {
            panic!("native: java/lang/reflect/Array.setByte:(Ljava/lang/Object;IB)V")
        }

        #[native]
        #[java_native(name = "setChar", descriptor = "(Ljava/lang/Object;IC)V", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn setChar(arg0: Object, arg1: i32, arg2: u16) -> Result<()> {
            panic!("native: java/lang/reflect/Array.setChar:(Ljava/lang/Object;IC)V")
        }

        #[native]
        #[java_native(name = "setShort", descriptor = "(Ljava/lang/Object;IS)V", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn setShort(arg0: Object, arg1: i32, arg2: i16) -> Result<()> {
            panic!("native: java/lang/reflect/Array.setShort:(Ljava/lang/Object;IS)V")
        }

        #[native]
        #[java_native(name = "setInt", descriptor = "(Ljava/lang/Object;II)V", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn setInt(arg0: Object, arg1: i32, arg2: i32) -> Result<()> {
            panic!("native: java/lang/reflect/Array.setInt:(Ljava/lang/Object;II)V")
        }

        #[native]
        #[java_native(name = "setLong", descriptor = "(Ljava/lang/Object;IJ)V", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn setLong(arg0: Object, arg1: i32, arg2: i64) -> Result<()> {
            panic!("native: java/lang/reflect/Array.setLong:(Ljava/lang/Object;IJ)V")
        }

        #[native]
        #[java_native(name = "setFloat", descriptor = "(Ljava/lang/Object;IF)V", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn setFloat(arg0: Object, arg1: i32, arg2: f32) -> Result<()> {
            panic!("native: java/lang/reflect/Array.setFloat:(Ljava/lang/Object;IF)V")
        }

        #[native]
        #[java_native(name = "setDouble", descriptor = "(Ljava/lang/Object;ID)V", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/ArrayIndexOutOfBoundsException")]
        pub fn setDouble(arg0: Object, arg1: i32, arg2: f64) -> Result<()> {
            panic!("native: java/lang/reflect/Array.setDouble:(Ljava/lang/Object;ID)V")
        }

        #[native]
        #[java_native(name = "newArray", descriptor = "(Ljava/lang/Class;I)Ljava/lang/Object;", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NegativeArraySizeException", generic_signature = "(Ljava/lang/Class<*>;I)Ljava/lang/Object;")]
        pub fn newArray(arg0: Object, arg1: i32) -> Result<Object> {
            panic!("native: java/lang/reflect/Array.newArray:(Ljava/lang/Class;I)Ljava/lang/Object;")
        }

        #[native]
        #[java_native(name = "multiNewArray", descriptor = "(Ljava/lang/Class;[I)Ljava/lang/Object;", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException,java/lang/NegativeArraySizeException", generic_signature = "(Ljava/lang/Class<*>;[I)Ljava/lang/Object;")]
        pub fn multiNewArray(arg0: Object, arg1: Rc<RefCell<Vec<i32>>>) -> Result<Object> {
            panic!("native: java/lang/reflect/Array.multiNewArray:(Ljava/lang/Class;[I)Ljava/lang/Object;")
        }
    }
}
