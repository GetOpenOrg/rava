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

impl From<Pattern_BnM> for Pattern_Node {
    fn from(v: Pattern_BnM) -> Pattern_Node { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/regex/Pattern$BnM"]
    #[super_class       = "java/util/regex/Pattern$Node"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Pattern.java"]
    #[inner_classes     = "java/util/regex/Pattern$Slice:java/util/regex/Pattern:Slice:8;java/util/regex/Pattern$SliceS:java/util/regex/Pattern:SliceS:24;java/util/regex/Pattern$BnMS:java/util/regex/Pattern:BnMS:24;java/util/regex/Pattern$Node:java/util/regex/Pattern:Node:8;java/util/regex/Pattern$BnM:java/util/regex/Pattern:BnM:8;java/util/regex/Pattern$TreeInfo:java/util/regex/Pattern:TreeInfo:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Pattern_Node"]
    #[superclass_fields(next: Pattern_Node)]
    #[all_supertypes    = "java/lang/Object;java/util/regex/Pattern$BnM;java/util/regex/Pattern$Node"]

    pub struct Pattern_BnM {
        #[cfg_attr(any(), java_field(name = "buffer", descriptor = "[I", is_static = false))]
        pub buffer: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "lastOcc", descriptor = "[I", is_static = false))]
        pub lastOcc: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "optoSft", descriptor = "[I", is_static = false))]
        pub optoSft: Rc<RefCell<Vec<i32>>>,
    }

    impl Pattern_BnM {
        #[java_method(name = "optimize", descriptor = "(Ljava/util/regex/Pattern$Node;)Ljava/util/regex/Pattern$Node;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn optimize(mut node: Pattern_Node) -> Result<Pattern_Node> {
            if !(false) {
                return Ok(node);
            }
            let mut src = Default::default().__get_buffer();
            let mut patternLength = (src.borrow().len() as i32);
            if patternLength < 4i32 {
                return Ok(node);
            }
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 128i32 as usize]));
            let mut lastOcc: Rc<RefCell<Vec<i32>>> = _arr0;
            let mut _arr1: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; patternLength as usize]));
            let mut optoSft: Rc<RefCell<Vec<i32>>> = _arr1;
            let mut i: i32 = 0i32;
            loop {
                if i >= patternLength { break; }
                lastOcc.borrow_mut()[(src.borrow()[i as usize]&127i32) as usize] = (i).wrapping_add(1i32);
                i = i.wrapping_add(1i32);
            }
            i = patternLength;
            loop {
                if (i<=0) { break; }
                let mut j = (patternLength).wrapping_sub(1i32);
                loop {
                    if j < i { break; }
                    optoSft.borrow_mut()[(j).wrapping_sub(1i32) as usize] = i;
                    j = j.wrapping_sub(1i32);
                }
                loop {
                    if (j<=0) { break; }
                    j = j.wrapping_sub(1i32);
                    optoSft.borrow_mut()[j as usize] = i;
                }
                i = i.wrapping_sub(1i32);
            }
            optoSft.borrow_mut()[(patternLength).wrapping_sub(1i32) as usize] = 1i32;
            if false {
                return Ok(<_ as Into<Pattern_Node>>::into(Pattern_BnMS::new(Clone::clone(&src), Clone::clone(&lastOcc), Clone::clone(&optoSft), Clone::clone(&node.__get_next()))?));
            }
            Ok(<_ as Into<Pattern_Node>>::into(Pattern_BnM::new(Clone::clone(&src), Clone::clone(&lastOcc), Clone::clone(&optoSft), Clone::clone(&node.__get_next()))?))
        }

        #[java_method(name = "<init>", descriptor = "([I[I[ILjava/util/regex/Pattern$Node;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut src: Rc<RefCell<Vec<i32>>>, mut lastOcc: Rc<RefCell<Vec<i32>>>, mut optoSft: Rc<RefCell<Vec<i32>>>, mut next: Pattern_Node) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Pattern_Node::new()?);
            this.__set_buffer(Clone::clone(&src));
            this.__set_lastOcc(Clone::clone(&lastOcc));
            this.__set_optoSft(Clone::clone(&optoSft));
            this.__set_next(Clone::clone(&next));
            Ok(this)
        }

        #[java_method(name = "match", descriptor = "(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn match_(&self, matcher: Matcher, i: i32, seq: Object) -> Result<bool> {
            panic!("stub: java/util/regex/Pattern$BnM.match:(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z")
        }

        #[java_method(name = "study", descriptor = "(Ljava/util/regex/Pattern$TreeInfo;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn study(&self, info: Pattern_TreeInfo) -> Result<bool> {
            panic!("stub: java/util/regex/Pattern$BnM.study:(Ljava/util/regex/Pattern$TreeInfo;)Z")
        }
    }
}
