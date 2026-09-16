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
use crate::jdk::internal::icu::text::*;
use crate::java::text::Normalizer;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/icu/text/NormalizerBase"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/lang/Cloneable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "NormalizerBase.java"]
    #[inner_classes     = "jdk/internal/icu/text/NormalizerBase$1:::4104;java/text/Normalizer$Form:java/text/Normalizer:Form:16409;jdk/internal/icu/text/NormalizerBase$Mode:jdk/internal/icu/text/NormalizerBase:Mode:1033;jdk/internal/icu/text/NormalizerBase$NONEMode:jdk/internal/icu/text/NormalizerBase:NONEMode:26;jdk/internal/icu/text/NormalizerBase$NFDMode:jdk/internal/icu/text/NormalizerBase:NFDMode:26;jdk/internal/icu/text/NormalizerBase$NFKDMode:jdk/internal/icu/text/NormalizerBase:NFKDMode:26;jdk/internal/icu/text/NormalizerBase$NFCMode:jdk/internal/icu/text/NormalizerBase:NFCMode:26;jdk/internal/icu/text/NormalizerBase$NFKCMode:jdk/internal/icu/text/NormalizerBase:NFKCMode:26;jdk/internal/icu/text/NormalizerBase$NFKC32ModeImpl:jdk/internal/icu/text/NormalizerBase:NFKC32ModeImpl:26;jdk/internal/icu/text/NormalizerBase$NFC32ModeImpl:jdk/internal/icu/text/NormalizerBase:NFC32ModeImpl:26;jdk/internal/icu/text/NormalizerBase$NFKD32ModeImpl:jdk/internal/icu/text/NormalizerBase:NFKD32ModeImpl:26;jdk/internal/icu/text/NormalizerBase$NFD32ModeImpl:jdk/internal/icu/text/NormalizerBase:NFD32ModeImpl:26;jdk/internal/icu/text/NormalizerBase$Unicode32:jdk/internal/icu/text/NormalizerBase:Unicode32:26;jdk/internal/icu/text/NormalizerBase$NFKCModeImpl:jdk/internal/icu/text/NormalizerBase:NFKCModeImpl:26;jdk/internal/icu/text/NormalizerBase$NFCModeImpl:jdk/internal/icu/text/NormalizerBase:NFCModeImpl:26;jdk/internal/icu/text/NormalizerBase$NFKDModeImpl:jdk/internal/icu/text/NormalizerBase:NFKDModeImpl:26;jdk/internal/icu/text/NormalizerBase$NFDModeImpl:jdk/internal/icu/text/NormalizerBase:NFDModeImpl:26;jdk/internal/icu/text/NormalizerBase$ModeImpl:jdk/internal/icu/text/NormalizerBase:ModeImpl:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Cloneable;java/lang/Object;jdk/internal/icu/text/NormalizerBase"]

    pub struct NormalizerBase {
        #[cfg_attr(any(), java_field(name = "text", descriptor = "Ljdk/internal/icu/text/UCharacterIterator;", access = "private", modifiers = "", is_static = false))]
        pub text: Object,
        #[cfg_attr(any(), java_field(name = "norm2", descriptor = "Ljdk/internal/icu/text/Normalizer2;", access = "private", modifiers = "", is_static = false))]
        pub norm2: Object,
        #[cfg_attr(any(), java_field(name = "mode", descriptor = "Ljdk/internal/icu/text/NormalizerBase$Mode;", access = "private", modifiers = "", is_static = false))]
        pub mode: Object,
        #[cfg_attr(any(), java_field(name = "options", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub options: i32,
        #[cfg_attr(any(), java_field(name = "currentIndex", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub currentIndex: i32,
        #[cfg_attr(any(), java_field(name = "nextIndex", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub nextIndex: i32,
        #[cfg_attr(any(), java_field(name = "buffer", descriptor = "Ljava/lang/StringBuilder;", access = "private", modifiers = "", is_static = false))]
        pub buffer: StringBuilder,
        #[cfg_attr(any(), java_field(name = "bufferPos", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub bufferPos: i32,
    }

    impl NormalizerBase {
        #[cfg_attr(any(), java_field(name = "UNICODE_3_2", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "32"))]
        // static field: UNICODE_3_2:I
        pub fn UNICODE_3_2() -> i32 {
            32
        }

        #[cfg_attr(any(), java_field(name = "UNICODE_3_2_0_ORIGINAL", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "32"))]
        // static field: UNICODE_3_2_0_ORIGINAL:I
        pub fn UNICODE_3_2_0_ORIGINAL() -> i32 {
            32
        }

        #[cfg_attr(any(), java_field(name = "UNICODE_LATEST", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: UNICODE_LATEST:I
        pub fn UNICODE_LATEST() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "DONE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "-1"))]
        // static field: DONE:I
        pub fn DONE() -> i32 {
            -1
        }

        #[cfg_attr(any(), java_field(name = "NONE", descriptor = "Ljdk/internal/icu/text/NormalizerBase$Mode;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NONE:Ljdk/internal/icu/text/NormalizerBase$Mode;
        pub fn NONE() -> Object {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.NONE:Ljdk/internal/icu/text/NormalizerBase$Mode;")
        }

        #[cfg_attr(any(), java_field(name = "NFD", descriptor = "Ljdk/internal/icu/text/NormalizerBase$Mode;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NFD:Ljdk/internal/icu/text/NormalizerBase$Mode;
        pub fn NFD() -> Object {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.NFD:Ljdk/internal/icu/text/NormalizerBase$Mode;")
        }

        #[cfg_attr(any(), java_field(name = "NFKD", descriptor = "Ljdk/internal/icu/text/NormalizerBase$Mode;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NFKD:Ljdk/internal/icu/text/NormalizerBase$Mode;
        pub fn NFKD() -> Object {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.NFKD:Ljdk/internal/icu/text/NormalizerBase$Mode;")
        }

        #[cfg_attr(any(), java_field(name = "NFC", descriptor = "Ljdk/internal/icu/text/NormalizerBase$Mode;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NFC:Ljdk/internal/icu/text/NormalizerBase$Mode;
        pub fn NFC() -> Object {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.NFC:Ljdk/internal/icu/text/NormalizerBase$Mode;")
        }

        #[cfg_attr(any(), java_field(name = "NFKC", descriptor = "Ljdk/internal/icu/text/NormalizerBase$Mode;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NFKC:Ljdk/internal/icu/text/NormalizerBase$Mode;
        pub fn NFKC() -> Object {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.NFKC:Ljdk/internal/icu/text/NormalizerBase$Mode;")
        }

        #[java_method(name = "toMode", descriptor = "(Ljava/text/Normalizer$Form;)Ljdk/internal/icu/text/NormalizerBase$Mode;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toMode(form: Normalizer_Form) -> Result<Object> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.toMode:(Ljava/text/Normalizer$Form;)Ljdk/internal/icu/text/NormalizerBase$Mode;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljdk/internal/icu/text/NormalizerBase$Mode;I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str_normal_i(str: String, mode: Object, opt: i32) -> Result<Self> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.<init>:(Ljava/lang/String;Ljdk/internal/icu/text/NormalizerBase$Mode;I)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljdk/internal/icu/text/NormalizerBase$Mode;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str_normal(str: String, mode: Object) -> Result<Self> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.<init>:(Ljava/lang/String;Ljdk/internal/icu/text/NormalizerBase$Mode;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/text/CharacterIterator;Ljdk/internal/icu/text/NormalizerBase$Mode;I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_charac_normal_i(iter: Object, mode: Object, opt: i32) -> Result<Self> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.<init>:(Ljava/text/CharacterIterator;Ljdk/internal/icu/text/NormalizerBase$Mode;I)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/text/CharacterIterator;Ljdk/internal/icu/text/NormalizerBase$Mode;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_charac_normal(iter: Object, mode: Object) -> Result<Self> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.<init>:(Ljava/text/CharacterIterator;Ljdk/internal/icu/text/NormalizerBase$Mode;)V")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "normalize", descriptor = "(Ljava/lang/String;Ljdk/internal/icu/text/NormalizerBase$Mode;I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalize_str_normal_i(str: String, mode: Object, options: i32) -> Result<String> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.normalize:(Ljava/lang/String;Ljdk/internal/icu/text/NormalizerBase$Mode;I)Ljava/lang/String;")
        }

        #[java_method(name = "normalize", descriptor = "(Ljava/lang/String;Ljava/text/Normalizer$Form;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalize_str_normal(str: String, form: Normalizer_Form) -> Result<String> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.normalize:(Ljava/lang/String;Ljava/text/Normalizer$Form;)Ljava/lang/String;")
        }

        #[java_method(name = "normalize", descriptor = "(Ljava/lang/String;Ljava/text/Normalizer$Form;I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalize_str_normal_i_1(str: String, form: Normalizer_Form, options: i32) -> Result<String> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.normalize:(Ljava/lang/String;Ljava/text/Normalizer$Form;I)Ljava/lang/String;")
        }

        #[java_method(name = "isNormalized", descriptor = "(Ljava/lang/String;Ljdk/internal/icu/text/NormalizerBase$Mode;I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isNormalized_str_normal_i(str: String, mode: Object, options: i32) -> Result<bool> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.isNormalized:(Ljava/lang/String;Ljdk/internal/icu/text/NormalizerBase$Mode;I)Z")
        }

        #[java_method(name = "isNormalized", descriptor = "(Ljava/lang/String;Ljava/text/Normalizer$Form;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isNormalized_str_normal(str: String, form: Normalizer_Form) -> Result<bool> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.isNormalized:(Ljava/lang/String;Ljava/text/Normalizer$Form;)Z")
        }

        #[java_method(name = "isNormalized", descriptor = "(Ljava/lang/String;Ljava/text/Normalizer$Form;I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isNormalized_str_normal_i_1(str: String, form: Normalizer_Form, options: i32) -> Result<bool> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.isNormalized:(Ljava/lang/String;Ljava/text/Normalizer$Form;I)Z")
        }

        #[java_method(name = "current", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn current(&self) -> Result<i32> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.current:()I")
        }

        #[java_method(name = "next", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn next(&self) -> Result<i32> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.next:()I")
        }

        #[java_method(name = "previous", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn previous(&self) -> Result<i32> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.previous:()I")
        }

        #[java_method(name = "reset", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reset(&self) -> Result<()> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.reset:()V")
        }

        #[java_method(name = "setIndexOnly", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setIndexOnly(&self, index: i32) -> Result<()> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.setIndexOnly:(I)V")
        }

        #[java_method(name = "setIndex", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setIndex(&self, index: i32) -> Result<i32> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.setIndex:(I)I")
        }

        #[java_method(name = "getBeginIndex", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getBeginIndex(&self) -> Result<i32> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.getBeginIndex:()I")
        }

        #[java_method(name = "getEndIndex", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getEndIndex(&self) -> Result<i32> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.getEndIndex:()I")
        }

        #[java_method(name = "getIndex", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIndex(&self) -> Result<i32> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.getIndex:()I")
        }

        #[java_method(name = "endIndex", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn endIndex(&self) -> Result<i32> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.endIndex:()I")
        }

        #[java_method(name = "setMode", descriptor = "(Ljdk/internal/icu/text/NormalizerBase$Mode;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMode(&self, newMode: Object) -> Result<()> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.setMode:(Ljdk/internal/icu/text/NormalizerBase$Mode;)V")
        }

        #[java_method(name = "getMode", descriptor = "()Ljdk/internal/icu/text/NormalizerBase$Mode;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMode(&self) -> Result<Object> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.getMode:()Ljdk/internal/icu/text/NormalizerBase$Mode;")
        }

        #[java_method(name = "setText", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setText_str(&self, newText: String) -> Result<()> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.setText:(Ljava/lang/String;)V")
        }

        #[java_method(name = "setText", descriptor = "(Ljava/text/CharacterIterator;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setText_charac(&self, newText: Object) -> Result<()> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.setText:(Ljava/text/CharacterIterator;)V")
        }

        #[java_method(name = "clearBuffer", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clearBuffer(&self) -> Result<()> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.clearBuffer:()V")
        }

        #[java_method(name = "nextNormalize", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextNormalize(&self) -> Result<bool> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.nextNormalize:()Z")
        }

        #[java_method(name = "previousNormalize", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn previousNormalize(&self) -> Result<bool> {
            panic!("stub: jdk/internal/icu/text/NormalizerBase.previousNormalize:()Z")
        }
    }
}
