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
use crate::sun::text::Normalizer;
use crate::jdk::internal::util::ArraysSupport;
use crate::jdk::internal::util::regex::Grapheme;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/regex/Pattern"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Pattern.java"]
    #[inner_classes     = "java/util/regex/Pattern$Start:java/util/regex/Pattern:Start:8;java/util/regex/Pattern$Node:java/util/regex/Pattern:Node:8;java/text/Normalizer$Form:java/text/Normalizer:Form:16409;java/util/regex/Pattern$GroupHead:java/util/regex/Pattern:GroupHead:24;java/util/regex/Pattern$Slice:java/util/regex/Pattern:Slice:8;java/util/regex/Pattern$BnM:java/util/regex/Pattern:BnM:8;java/util/regex/Pattern$StartS:java/util/regex/Pattern:StartS:24;java/util/regex/Pattern$Begin:java/util/regex/Pattern:Begin:24;java/util/regex/Pattern$First:java/util/regex/Pattern:First:24;java/util/regex/Pattern$Loop:java/util/regex/Pattern:Loop:8;java/util/regex/Pattern$BranchConn:java/util/regex/Pattern:BranchConn:24;java/util/regex/Pattern$Branch:java/util/regex/Pattern:Branch:24;java/util/regex/Pattern$NFCCharProperty:java/util/regex/Pattern:NFCCharProperty:10;java/util/regex/Pattern$CharPredicate:java/util/regex/Pattern:CharPredicate:1544;java/util/regex/Pattern$CharProperty:java/util/regex/Pattern:CharProperty:8;java/util/regex/Pattern$UnixCaret:java/util/regex/Pattern:UnixCaret:24;java/util/regex/Pattern$Caret:java/util/regex/Pattern:Caret:24;java/util/regex/Pattern$UnixDollar:java/util/regex/Pattern:UnixDollar:24;java/util/regex/Pattern$Dollar:java/util/regex/Pattern:Dollar:24;java/util/regex/Pattern$CIBackRef:java/util/regex/Pattern:CIBackRef:8;java/util/regex/Pattern$BackRef:java/util/regex/Pattern:BackRef:8;java/util/regex/Pattern$Bound:java/util/regex/Pattern:Bound:24;java/util/regex/Pattern$BmpCharPredicate:java/util/regex/Pattern:BmpCharPredicate:1544;java/util/regex/Pattern$LastMatch:java/util/regex/Pattern:LastMatch:24;java/util/regex/Pattern$LineEnding:java/util/regex/Pattern:LineEnding:24;java/util/regex/Pattern$XGrapheme:java/util/regex/Pattern:XGrapheme:8;java/util/regex/Pattern$GraphemeBound:java/util/regex/Pattern:GraphemeBound:8;java/util/regex/Pattern$End:java/util/regex/Pattern:End:24;java/util/regex/Pattern$BitClass:java/util/regex/Pattern:BitClass:24;java/util/regex/Pattern$BmpCharProperty:java/util/regex/Pattern:BmpCharProperty:10;java/util/regex/Pattern$Pos:java/util/regex/Pattern:Pos:24;java/util/regex/Pattern$Neg:java/util/regex/Pattern:Neg:24;java/util/regex/Pattern$Ques:java/util/regex/Pattern:Ques:24;java/util/regex/Pattern$Qtype:java/util/regex/Pattern:Qtype:16408;java/util/regex/Pattern$LookBehindEndNode:java/util/regex/Pattern:LookBehindEndNode:8;java/util/regex/Pattern$TreeInfo:java/util/regex/Pattern:TreeInfo:24;java/util/regex/Pattern$BehindS:java/util/regex/Pattern:BehindS:24;java/util/regex/Pattern$Behind:java/util/regex/Pattern:Behind:8;java/util/regex/Pattern$NotBehindS:java/util/regex/Pattern:NotBehindS:24;java/util/regex/Pattern$NotBehind:java/util/regex/Pattern:NotBehind:8;java/util/regex/Pattern$Curly:java/util/regex/Pattern:Curly:24;java/util/regex/Pattern$GroupTail:java/util/regex/Pattern:GroupTail:24;java/util/regex/Pattern$GroupCurly:java/util/regex/Pattern:GroupCurly:24;java/util/regex/Pattern$LazyLoop:java/util/regex/Pattern:LazyLoop:24;java/util/regex/Pattern$Prolog:java/util/regex/Pattern:Prolog:24;java/util/regex/Pattern$BmpCharPropertyGreedy:java/util/regex/Pattern:BmpCharPropertyGreedy:24;java/util/regex/Pattern$CharPropertyGreedy:java/util/regex/Pattern:CharPropertyGreedy:8;java/util/regex/Pattern$SliceUS:java/util/regex/Pattern:SliceUS:24;java/util/regex/Pattern$SliceU:java/util/regex/Pattern:SliceU:24;java/util/regex/Pattern$SliceIS:java/util/regex/Pattern:SliceIS:8;java/util/regex/Pattern$SliceI:java/util/regex/Pattern:SliceI:8;java/util/regex/Pattern$SliceS:java/util/regex/Pattern:SliceS:24;java/util/regex/Pattern$1MatcherIterator::MatcherIterator:0;java/util/regex/Pattern$LastNode:java/util/regex/Pattern:LastNode:8;java/util/regex/Pattern$BnMS:java/util/regex/Pattern:BnMS:24;java/util/regex/Pattern$SliceNode:java/util/regex/Pattern:SliceNode:8;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/util/regex/Pattern"]
    #[has_to_string_method = true]

    pub struct Pattern {
        #[cfg_attr(any(), java_field(name = "pattern", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub pattern: String,
        #[cfg_attr(any(), java_field(name = "flags", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub flags: i32,
        #[cfg_attr(any(), java_field(name = "flags0", descriptor = "I", access = "private", modifiers = "transient", is_static = false))]
        pub flags0: i32,
        #[cfg_attr(any(), java_field(name = "compiled", descriptor = "Z", access = "private", modifiers = "volatile transient", is_static = false))]
        pub compiled: bool,
        #[cfg_attr(any(), java_field(name = "normalizedPattern", descriptor = "Ljava/lang/String;", access = "private", modifiers = "transient", is_static = false))]
        pub normalizedPattern: String,
        #[cfg_attr(any(), java_field(name = "root", descriptor = "Ljava/util/regex/Pattern$Node;", access = "package", modifiers = "transient", is_static = false))]
        pub root: Pattern_Node,
        #[cfg_attr(any(), java_field(name = "matchRoot", descriptor = "Ljava/util/regex/Pattern$Node;", access = "package", modifiers = "transient", is_static = false))]
        pub matchRoot: Pattern_Node,
        #[cfg_attr(any(), java_field(name = "buffer", descriptor = "[I", access = "package", modifiers = "transient", is_static = false))]
        pub buffer: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "predicate", descriptor = "Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "transient", is_static = false))]
        pub predicate: Object,
        #[cfg_attr(any(), java_field(name = "namedGroups", descriptor = "Ljava/util/Map;", access = "package", modifiers = "volatile transient", is_static = false, generic_signature = "Ljava/util/Map<Ljava/lang/String;Ljava/lang/Integer;>;"))]
        pub namedGroups: Object,
        #[cfg_attr(any(), java_field(name = "groupNodes", descriptor = "[Ljava/util/regex/Pattern$GroupHead;", access = "package", modifiers = "transient", is_static = false))]
        pub groupNodes: Rc<RefCell<Vec<Pattern_GroupHead>>>,
        #[cfg_attr(any(), java_field(name = "topClosureNodes", descriptor = "Ljava/util/List;", access = "package", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/List<Ljava/util/regex/Pattern$Node;>;"))]
        pub topClosureNodes: Object,
        #[cfg_attr(any(), java_field(name = "localTCNCount", descriptor = "I", access = "package", modifiers = "transient", is_static = false))]
        pub localTCNCount: i32,
        #[cfg_attr(any(), java_field(name = "hasGroupRef", descriptor = "Z", access = "package", modifiers = "transient", is_static = false))]
        pub hasGroupRef: bool,
        #[cfg_attr(any(), java_field(name = "temp", descriptor = "[I", access = "private", modifiers = "transient", is_static = false))]
        pub temp: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "capturingGroupCount", descriptor = "I", access = "package", modifiers = "transient", is_static = false))]
        pub capturingGroupCount: i32,
        #[cfg_attr(any(), java_field(name = "localCount", descriptor = "I", access = "package", modifiers = "transient", is_static = false))]
        pub localCount: i32,
        #[cfg_attr(any(), java_field(name = "cursor", descriptor = "I", access = "private", modifiers = "transient", is_static = false))]
        pub cursor: i32,
        #[cfg_attr(any(), java_field(name = "patternLength", descriptor = "I", access = "private", modifiers = "transient", is_static = false))]
        pub patternLength: i32,
        #[cfg_attr(any(), java_field(name = "hasSupplementary", descriptor = "Z", access = "private", modifiers = "transient", is_static = false))]
        pub hasSupplementary: bool,
    }

    impl Pattern {
        #[cfg_attr(any(), java_field(name = "UNIX_LINES", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: UNIX_LINES:I
        pub fn UNIX_LINES() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "CASE_INSENSITIVE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: CASE_INSENSITIVE:I
        pub fn CASE_INSENSITIVE() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "COMMENTS", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: COMMENTS:I
        pub fn COMMENTS() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "MULTILINE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: MULTILINE:I
        pub fn MULTILINE() -> i32 {
            8
        }

        #[cfg_attr(any(), java_field(name = "LITERAL", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: LITERAL:I
        pub fn LITERAL() -> i32 {
            16
        }

        #[cfg_attr(any(), java_field(name = "DOTALL", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "32"))]
        // static field: DOTALL:I
        pub fn DOTALL() -> i32 {
            32
        }

        #[cfg_attr(any(), java_field(name = "UNICODE_CASE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "64"))]
        // static field: UNICODE_CASE:I
        pub fn UNICODE_CASE() -> i32 {
            64
        }

        #[cfg_attr(any(), java_field(name = "CANON_EQ", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "128"))]
        // static field: CANON_EQ:I
        pub fn CANON_EQ() -> i32 {
            128
        }

        #[cfg_attr(any(), java_field(name = "UNICODE_CHARACTER_CLASS", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "256"))]
        // static field: UNICODE_CHARACTER_CLASS:I
        pub fn UNICODE_CHARACTER_CLASS() -> i32 {
            256
        }

        #[cfg_attr(any(), java_field(name = "ALL_FLAGS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "511"))]
        // static field: ALL_FLAGS:I
        pub fn ALL_FLAGS() -> i32 {
            511
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "5073258162644648461"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            5073258162644648461i64
        }

        #[cfg_attr(any(), java_field(name = "MAX_REPS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "2147483647"))]
        // static field: MAX_REPS:I
        pub fn MAX_REPS() -> i32 {
            2147483647
        }

        #[cfg_attr(any(), java_field(name = "accept", descriptor = "Ljava/util/regex/Pattern$Node;", access = "package", modifiers = "static final", is_static = true))]
        // static field: accept:Ljava/util/regex/Pattern$Node;
        pub fn accept_field() -> Pattern_Node {
            panic!("stub: java/util/regex/Pattern.accept:Ljava/util/regex/Pattern$Node;")
        }

        #[cfg_attr(any(), java_field(name = "lastAccept", descriptor = "Ljava/util/regex/Pattern$Node;", access = "package", modifiers = "static final", is_static = true))]
        // static field: lastAccept:Ljava/util/regex/Pattern$Node;
        pub fn lastAccept() -> Pattern_Node {
            panic!("stub: java/util/regex/Pattern.lastAccept:Ljava/util/regex/Pattern$Node;")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "compile", descriptor = "(Ljava/lang/String;)Ljava/util/regex/Pattern;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: compile(Ljava/lang/String;)Ljava/util/regex/Pattern;
        pub fn compile_str(mut regex: String) -> Result<Pattern> {
            Ok(Pattern::new(Clone::clone(&regex), 0i32)?)
        }

        #[java_method(name = "compile", descriptor = "(Ljava/lang/String;I)Ljava/util/regex/Pattern;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compile_str_i(regex: String, flags: i32) -> Result<Pattern> {
            panic!("stub: java/util/regex/Pattern.compile:(Ljava/lang/String;I)Ljava/util/regex/Pattern;")
        }

        #[java_method(name = "pattern", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn pattern(&self) -> Result<String> {
            panic!("stub: java/util/regex/Pattern.pattern:()Ljava/lang/String;")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "matcher", descriptor = "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn matcher(&self, mut input: Object) -> Result<Matcher> {
            let this = self;
            let mut m = this.clone();
            if !(this.__get_compiled()) {
                this.compile()?;
            }
            let mut m = Matcher::new_patter_seq(Clone::clone(this), Clone::clone(&input))?;
            Ok(m)
        }

        #[java_method(name = "flags", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn flags(&self) -> Result<i32> {
            panic!("stub: java/util/regex/Pattern.flags:()I")
        }

        #[java_method(name = "matches", descriptor = "(Ljava/lang/String;Ljava/lang/CharSequence;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn matches(mut regex: String, mut input: Object) -> Result<bool> {
            let _t0: Pattern = Pattern::compile_str(Clone::clone(&regex))?;
            let mut p: Pattern = _t0;
            let _t1 = p.matcher(Clone::clone(&input))?;
            let mut m: Matcher = _t1;
            let _t2 = m.matches()?;
            Ok(_t2)
        }

        #[java_method(name = "split", descriptor = "(Ljava/lang/CharSequence;I)[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: split(Ljava/lang/CharSequence;I)[Ljava/lang/String;
        pub fn split_seq_i(&self, mut input: Object, mut limit: i32) -> Result<Rc<RefCell<Vec<String>>>> {
            let this = self;
            let _t0 = this.split_seq_i_z(Clone::clone(&input), limit, (0i32 != 0i32))?;
            Ok(_t0)
        }

        #[java_method(name = "splitWithDelimiters", descriptor = "(Ljava/lang/CharSequence;I)[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn splitWithDelimiters(&self, mut input: Object, mut limit: i32) -> Result<Rc<RefCell<Vec<String>>>> {
            let this = self;
            let _t0 = this.split_seq_i_z(Clone::clone(&input), limit, (1i32 != 0i32))?;
            Ok(_t0)
        }

        #[java_method(name = "split", descriptor = "(Ljava/lang/CharSequence;IZ)[Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: split(Ljava/lang/CharSequence;IZ)[Ljava/lang/String;
        pub fn split_seq_i_z(&self, mut input: Object, mut limit: i32, mut withDelimiters: bool) -> Result<Rc<RefCell<Vec<String>>>> {
            let this = self;
            let mut matchCount: i32 = 0i32;
            let mut index: i32 = 0i32;
            let mut matchLimited = ((limit>0)) as i32;
            let mut matchList = ArrayList::<Object>::new()?;
            let _t0 = this.matcher(Clone::clone(&input))?;
            let mut m: Matcher = _t0;
            loop {
                let _t1 = m.find()?;
                if !(_t1) { break; }
                let _t1 = m.start()?;
                let _t2 = m.start()?;
                let _t3 = m.end()?;
                if _t2 == _t3 {
                    continue;
                }
                let _t4 = m.start()?;
                let _vdispatch5: Object = if let Some(_d) = input.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.subSequence(index, _t4)? } else if let Some(_d) = input.0.as_any().downcast_ref::<CharBuffer>() { _d.subSequence(index, _t4)? } else if let Some(_d) = input.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.subSequence(index, _t4)? } else if let Some(_d) = input.0.as_any().downcast_ref::<String>() { _d.subSequence(index, _t4)? } else if let Some(_d) = input.0.as_any().downcast_ref::<StringBuilder>() { _d.subSequence(index, _t4)? } else if let Some(_d) = input.0.as_any().downcast_ref::<Object>() { _d.subSequence(index, _t4)? } else if let Some(__f) = input.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32, i32) -> crate::error::Result<Object>>>() { (__f)(index, _t4)? } else { Default::default() };
                let _vdispatch6: String = if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.toString()? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<CharBuffer>() { _d.toString()? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.toString()? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<String>() { _d.toString()? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<StringBuilder>() { _d.toString()? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = _vdispatch5.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
                let mut match_: String = _vdispatch6;
                let _t7 = matchList.add_obj(Object::from_any(match_.clone()))?;
                let _t8 = m.end()?;
                index = _t8;
                if withDelimiters {
                    let _t9 = m.start()?;
                    let _vdispatch10: Object = if let Some(_d) = input.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.subSequence(_t9, index)? } else if let Some(_d) = input.0.as_any().downcast_ref::<CharBuffer>() { _d.subSequence(_t9, index)? } else if let Some(_d) = input.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.subSequence(_t9, index)? } else if let Some(_d) = input.0.as_any().downcast_ref::<String>() { _d.subSequence(_t9, index)? } else if let Some(_d) = input.0.as_any().downcast_ref::<StringBuilder>() { _d.subSequence(_t9, index)? } else if let Some(_d) = input.0.as_any().downcast_ref::<Object>() { _d.subSequence(_t9, index)? } else if let Some(__f) = input.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32, i32) -> crate::error::Result<Object>>>() { (__f)(_t9, index)? } else { Default::default() };
                    let _vdispatch11: String = if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.toString()? } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<CharBuffer>() { _d.toString()? } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.toString()? } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<String>() { _d.toString()? } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<StringBuilder>() { _d.toString()? } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = _vdispatch10.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
                    let _t12 = matchList.add_obj(Object::from_any(_vdispatch11.clone()))?;
                }
                matchCount = matchCount.wrapping_add(1i32);
                continue;
                let _vdispatch9: i32 = if let Some(_d) = input.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.length()? } else if let Some(_d) = input.0.as_any().downcast_ref::<CharBuffer>() { _d.length()? } else if let Some(_d) = input.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.length()? } else if let Some(_d) = input.0.as_any().downcast_ref::<String>() { _d.length()? } else if let Some(_d) = input.0.as_any().downcast_ref::<StringBuilder>() { _d.length()? } else if let Some(_d) = input.0.as_any().downcast_ref::<Object>() { _d.length()? } else if let Some(__f) = input.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
                let _vdispatch10: Object = if let Some(_d) = input.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.subSequence(index, _vdispatch9)? } else if let Some(_d) = input.0.as_any().downcast_ref::<CharBuffer>() { _d.subSequence(index, _vdispatch9)? } else if let Some(_d) = input.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.subSequence(index, _vdispatch9)? } else if let Some(_d) = input.0.as_any().downcast_ref::<String>() { _d.subSequence(index, _vdispatch9)? } else if let Some(_d) = input.0.as_any().downcast_ref::<StringBuilder>() { _d.subSequence(index, _vdispatch9)? } else if let Some(_d) = input.0.as_any().downcast_ref::<Object>() { _d.subSequence(index, _vdispatch9)? } else if let Some(__f) = input.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32, i32) -> crate::error::Result<Object>>>() { (__f)(index, _vdispatch9)? } else { Default::default() };
                let _vdispatch11: String = if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.toString()? } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<CharBuffer>() { _d.toString()? } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.toString()? } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<String>() { _d.toString()? } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<StringBuilder>() { _d.toString()? } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = _vdispatch10.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
                match_ = _vdispatch11;
                let _t12 = matchList.add_obj(Object::from_any(match_.clone()))?;
                let _t13 = m.end()?;
                index = _t13;
                matchCount = matchCount.wrapping_add(1i32);
            }
            if (index==0) {
                let mut _arr1: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
                let _vdispatch2: String = if let Some(_d) = input.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.toString()? } else if let Some(_d) = input.0.as_any().downcast_ref::<CharBuffer>() { _d.toString()? } else if let Some(_d) = input.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.toString()? } else if let Some(_d) = input.0.as_any().downcast_ref::<String>() { _d.toString()? } else if let Some(_d) = input.0.as_any().downcast_ref::<StringBuilder>() { _d.toString()? } else if let Some(_d) = input.0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = input.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
                _arr1.borrow_mut()[0i32 as usize] = Clone::clone(&_vdispatch2);
                return Ok(_arr1);
            }
            if matchCount < limit {
                let _vdispatch1: i32 = if let Some(_d) = input.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.length()? } else if let Some(_d) = input.0.as_any().downcast_ref::<CharBuffer>() { _d.length()? } else if let Some(_d) = input.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.length()? } else if let Some(_d) = input.0.as_any().downcast_ref::<String>() { _d.length()? } else if let Some(_d) = input.0.as_any().downcast_ref::<StringBuilder>() { _d.length()? } else if let Some(_d) = input.0.as_any().downcast_ref::<Object>() { _d.length()? } else if let Some(__f) = input.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
                let _vdispatch2: Object = if let Some(_d) = input.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.subSequence(index, _vdispatch1)? } else if let Some(_d) = input.0.as_any().downcast_ref::<CharBuffer>() { _d.subSequence(index, _vdispatch1)? } else if let Some(_d) = input.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.subSequence(index, _vdispatch1)? } else if let Some(_d) = input.0.as_any().downcast_ref::<String>() { _d.subSequence(index, _vdispatch1)? } else if let Some(_d) = input.0.as_any().downcast_ref::<StringBuilder>() { _d.subSequence(index, _vdispatch1)? } else if let Some(_d) = input.0.as_any().downcast_ref::<Object>() { _d.subSequence(index, _vdispatch1)? } else if let Some(__f) = input.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32, i32) -> crate::error::Result<Object>>>() { (__f)(index, _vdispatch1)? } else { Default::default() };
                let _vdispatch3: String = if let Some(_d) = _vdispatch2.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.toString()? } else if let Some(_d) = _vdispatch2.0.as_any().downcast_ref::<CharBuffer>() { _d.toString()? } else if let Some(_d) = _vdispatch2.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.toString()? } else if let Some(_d) = _vdispatch2.0.as_any().downcast_ref::<String>() { _d.toString()? } else if let Some(_d) = _vdispatch2.0.as_any().downcast_ref::<StringBuilder>() { _d.toString()? } else if let Some(_d) = _vdispatch2.0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = _vdispatch2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
                let _t4 = matchList.add_obj(Object::from_any(_vdispatch3.clone()))?;
            }
            let _t1 = matchList.size()?;
            let mut match_: i32 = _t1;
            loop {
                if (match_<=0) { break; }
                let _t2 = matchList.get((match_).wrapping_sub(1i32))?;
                let _t3 = (_t2).downcast::<String>().isEmpty()?;
                if _t3 {
                    match_ = match_.wrapping_sub(1i32);
                    continue;
                }
                break;
            }
            let mut _arr2: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); match_ as usize]));
            let mut result: Rc<RefCell<Vec<String>>> = _arr2;
            let _t3 = matchList.subList(0i32, match_)?;
            let _vdispatch4: Rc<RefCell<Vec<Object>>> = if let Some(_d) = _t3.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.toArray(Default::default())? } else if let Some(_d) = _t3.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.toArray(Default::default())? } else if let Some(_d) = _t3.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.toArray(Default::default())? } else if let Some(_d) = _t3.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.toArray(Default::default())? } else if let Some(_d) = _t3.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.toArray(Default::default())? } else if let Some(_d) = _t3.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.toArray(Default::default())? } else if let Some(_d) = _t3.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.toArray(Default::default())? } else if let Some(_d) = _t3.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.toArray(Default::default())? } else if let Some(_d) = _t3.0.as_any().downcast_ref::<Object>() { _d.toArray(Default::default())? } else if let Some(__f) = _t3.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Rc<RefCell<Vec<Object>>>) -> crate::error::Result<Rc<RefCell<Vec<Object>>>>>>() { (__f)(Default::default())? } else { Default::default() };
            Ok(Default::default())
        }

        #[java_method(name = "split", descriptor = "(Ljava/lang/CharSequence;)[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn split_seq(&self, input: Object) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: java/util/regex/Pattern.split:(Ljava/lang/CharSequence;)[Ljava/lang/String;")
        }

        #[java_method(name = "quote", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn quote(s: String) -> Result<String> {
            panic!("stub: java/util/regex/Pattern.quote:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/regex/Pattern.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut p: String, mut f: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            if ((f&-512i32)!=0) {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Unknown flag 0x")))?;
                let _t1: String = Integer::toHexString(f)?;
                let _t2 = _t0.append_str(Clone::clone(&_t1))?;
                let _t3 = _t2.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.__set_pattern(Clone::clone(&p));
            this.__set_flags(f);
            if ((this.__get_flags()&256i32)!=0) {
                this.__set_flags((this.__get_flags()|64i32));
            }
            this.__set_flags0(this.__get_flags());
            this.__set_capturingGroupCount(1i32);
            this.__set_localCount(0i32);
            this.__set_localTCNCount(0i32);
            let _t0 = this.__get_pattern().isEmpty()?;
            if !(_t0) {
                this.compile()?;
                let mut soe = (panic!("stack underflow") as i32);
                let _t1 = this.error(Clone::clone(&String::from("Stack overflow during pattern compilation")))?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.__set_root(<_ as Into<Pattern_Node>>::into(Pattern_Start::new(Clone::clone(&Pattern::lastAccept()))?));
            this.__set_matchRoot(Clone::clone(&Pattern::lastAccept()));
            Ok(this)
        }

        #[java_method(name = "normalize", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalize(mut pattern: String) -> Result<String> {
            let _t0 = pattern.length()?;
            let mut plen: i32 = _t0;
            let mut pbuf = StringBuilder::new_i(plen)?;
            let mut last: i32 = 0i32;
            let mut lastStart: i32 = 0i32;
            let mut cc: i32 = 0i32;
            let mut i: i32 = 0i32;
            loop {
                if i >= plen { break; }
                let _t1 = pattern.charAt(i)?;
                let mut c: u16 = _t1;
                let _t2 = pattern.charAt((i).wrapping_add(1i32))?;
                if (_t2 as i32) == 92i32 {
                    i = i.wrapping_add(2i32);
                    last = 0i32;
                    continue;
                }
                if (c as i32) == 91i32 {
                    if last != 92i32 {
                        if lastStart < i {
                            Pattern::normalizeSlice(Clone::clone(&pattern), lastStart, i, Clone::clone(&pbuf))?;
                        }
                        lastStart = i;
                        cc = (((cc).wrapping_add(1i32)) as u16 as i32);
                    } else {
                        cc = (((cc).wrapping_sub(1i32)) as u16 as i32);
                        if (cc==0) {
                            Pattern::normalizeClazz(Clone::clone(&pattern), lastStart, (i).wrapping_add(1i32), Clone::clone(&pbuf))?;
                            lastStart = (i).wrapping_add(1i32);
                        }
                    }
                } else {
                    cc = (((cc).wrapping_sub(1i32)) as u16 as i32);
                    if (cc==0) {
                        Pattern::normalizeClazz(Clone::clone(&pattern), lastStart, (i).wrapping_add(1i32), Clone::clone(&pbuf))?;
                        lastStart = (i).wrapping_add(1i32);
                    }
                }
                let mut last: u16 = c;
                i = i.wrapping_add(1i32);
            }
            if (cc!=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if lastStart < plen {
                Pattern::normalizeSlice(Clone::clone(&pattern), lastStart, plen, Clone::clone(&pbuf))?;
            }
            let _t1 = pbuf.toString()?;
            Ok(_t1)
        }

        #[java_method(name = "normalizeSlice", descriptor = "(Ljava/lang/String;IILjava/lang/StringBuilder;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalizeSlice(mut src: String, mut off: i32, mut limit: i32, mut dst: StringBuilder) -> Result<()> {
            let _t0 = src.length()?;
            let mut len: i32 = _t0;
            let mut off0: i32 = off;
            loop {
                if off >= limit { break; }
                let _t1 = src.charAt(off)?;
                let _t2: bool = ASCII::isAscii((_t1 as i32))?;
                if _t2 {
                    off = off.wrapping_add(1i32);
                    continue;
                }
                break;
            }
            if off == limit {
                let _t1 = dst.append_seq_i_i(Object::from_any(src.clone()), off0, limit)?;
                return Ok(());
            }
            off = off.wrapping_sub(1i32);
            if off < off0 {
                off = off0;
            } else {
                let _t1 = dst.append_seq_i_i(Object::from_any(src.clone()), off0, off)?;
            }
            loop {
                if off >= limit { break; }
                let _t1 = src.codePointAt(off)?;
                let mut ch0: i32 = _t1;
                let _t2 = String::from(".$|()[]{}^?*+\\").indexOf_i(ch0)?;
                if _t2 != -1i32 {
                    let _t3 = dst.append_c(((((ch0) as u16 as i32)) as u16))?;
                    off = off.wrapping_add(1i32);
                    continue;
                }
                let _t3: i32 = Grapheme::nextBoundary(Object::from_any(src.clone()), off, limit)?;
                let mut j: i32 = _t3;
                let _t4 = src.substring_i_i(off, j)?;
                let mut seq: String = _t4;
                let _t5: String = Normalizer::normalize(Object::from_any(seq.clone()), Clone::clone(&Normalizer_Form::NFD()))?;
                let mut nfd: String = _t5;
                off = j;
                let _t6 = nfd.length()?;
                let _t7 = nfd.codePointCount(0i32, _t6)?;
                let _t8 = nfd.codePointAt(0i32)?;
                ch0 = _t8;
                let _t9: i32 = Character::charCount(ch0)?;
                let _t10 = nfd.codePointAt(_t9)?;
                let mut ch1: i32 = _t10;
                let _t11: i32 = Character::getType_i(ch1)?;
                if _t11 == 6i32 {
                    let mut altns = LinkedHashSet::<Object>::new()?;
                    let _t12 = altns.__super().add(Object::from_any(seq.clone()))?;
                    Pattern::produceEquivalentAlternation(Clone::clone(&nfd), Object::from_any(altns.clone()))?;
                    let _t13 = dst.append_str(Clone::clone(&String::from("(?:")))?;
                    let __lam_cap263_0 = dst;
                    let __lam_263: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<()>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<()> { Pattern::lambda_normalizeSlice_0(__lam_cap263_0.clone(), _la0) });
                    altns.forEach(Clone::clone(&Object::from_any(__lam_263)))?;
                    let _t14 = dst.__super().length()?;
                    let _t15 = dst.__super().length()?;
                    let _t16 = dst.delete((_t14).wrapping_sub(1i32), _t15)?;
                    let _t17 = dst.append_str(Clone::clone(&String::from(")")))?;
                    continue;
                }
                let _t12: String = Normalizer::normalize(Object::from_any(seq.clone()), Clone::clone(&Normalizer_Form::NFC()))?;
                let mut altns: String = _t12;
                let _t13 = seq.equals(Object::from_any(altns.clone()))?;
                if !(_t13) {
                    let _t14 = nfd.equals(Object::from_any(altns.clone()))?;
                    if !(_t14) {
                        let _t15 = StringBuilder::new()?.append_str(Clone::clone(&String::from("(?:")))?;
                        let _t16 = _t15.append_str(Clone::clone(&seq))?;
                        let _t17 = _t16.append_str(Clone::clone(&String::from("|")))?;
                        let _t18 = _t17.append_str(Clone::clone(&nfd))?;
                        let _t19 = _t18.append_str(Clone::clone(&String::from("|")))?;
                        let _t20 = _t19.append_str(Clone::clone(&altns))?;
                        let _t21 = _t20.append_str(Clone::clone(&String::from(")")))?;
                        let _t22 = _t21.toString()?;
                        let _t23 = dst.append_str(Clone::clone(&_t22))?;
                    } else {
                        let _t15 = seq.equals(Object::from_any(nfd.clone()))?;
                        if !(_t15) {
                            let _t16 = StringBuilder::new()?.append_str(Clone::clone(&String::from("(?:")))?;
                            let _t17 = _t16.append_str(Clone::clone(&seq))?;
                            let _t18 = _t17.append_str(Clone::clone(&String::from("|")))?;
                            let _t19 = _t18.append_str(Clone::clone(&nfd))?;
                            let _t20 = _t19.append_str(Clone::clone(&String::from(")")))?;
                            let _t21 = _t20.toString()?;
                            let _t22 = dst.append_str(Clone::clone(&_t21))?;
                        } else {
                            let _t16 = dst.append_str(Clone::clone(&seq))?;
                        }
                    }
                } else {
                    let _t14 = seq.equals(Object::from_any(nfd.clone()))?;
                    if !(_t14) {
                        let _t15 = StringBuilder::new()?.append_str(Clone::clone(&String::from("(?:")))?;
                        let _t16 = _t15.append_str(Clone::clone(&seq))?;
                        let _t17 = _t16.append_str(Clone::clone(&String::from("|")))?;
                        let _t18 = _t17.append_str(Clone::clone(&nfd))?;
                        let _t19 = _t18.append_str(Clone::clone(&String::from(")")))?;
                        let _t20 = _t19.toString()?;
                        let _t21 = dst.append_str(Clone::clone(&_t20))?;
                    } else {
                        let _t15 = dst.append_str(Clone::clone(&seq))?;
                    }
                }
            }
            Ok(())
        }

        #[java_method(name = "normalizeClazz", descriptor = "(Ljava/lang/String;IILjava/lang/StringBuilder;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalizeClazz(mut src: String, mut off: i32, mut limit: i32, mut dst: StringBuilder) -> Result<()> {
            let _t0 = src.substring_i_i(off, limit)?;
            let _t1: String = Normalizer::normalize(Object::from_any(_t0.clone()), Clone::clone(&Normalizer_Form::NFC()))?;
            let _t2 = dst.append_str(Clone::clone(&_t1))?;
            Ok(())
        }

        #[java_method(name = "produceEquivalentAlternation", descriptor = "(Ljava/lang/String;Ljava/util/Set;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;Ljava/util/Set<Ljava/lang/String;>;)V")]
        pub fn produceEquivalentAlternation(mut src: String, mut dst: Object) -> Result<()> {
            let _t0: i32 = Pattern::countChars(Object::from_any(src.clone()), 0i32, 1i32)?;
            let mut len: i32 = _t0;
            let _t1 = src.length()?;
            if _t1 == len {
                let _vdispatch2: bool = if let Some(_d) = dst.0.as_any().downcast_ref::<HashMap_EntrySet>() { _d.add(Object::from_any(src.clone()))? } else if let Some(_d) = dst.0.as_any().downcast_ref::<HashMap_KeySet>() { _d.add(Object::from_any(src.clone()))? } else if let Some(_d) = dst.0.as_any().downcast_ref::<TreeMap_EntrySet>() { _d.add(Object::from_any(src.clone()))? } else if let Some(_d) = dst.0.as_any().downcast_ref::<LinkedHashSet<Object>>() { _d.add(Object::from_any(src.clone()))? } else if let Some(_d) = dst.0.as_any().downcast_ref::<AbstractSet<Object>>() { _d.add(Object::from_any(src.clone()))? } else if let Some(_d) = dst.0.as_any().downcast_ref::<HashSet<Object>>() { _d.add(Object::from_any(src.clone()))? } else if let Some(_d) = dst.0.as_any().downcast_ref::<Object>() { _d.add(Object::from_any(src.clone()))? } else if let Some(__f) = dst.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Object::from_any(src.clone()))? } else { Default::default() };
                return Ok(());
            }
            let _t2 = src.substring_i_i(0i32, len)?;
            let mut base: String = _t2;
            let _t3 = src.substring_i(len)?;
            let mut combiningMarks: String = _t3;
            let _t4: Rc<RefCell<Vec<String>>> = Pattern::producePermutations(Clone::clone(&combiningMarks))?;
            let mut perms: Rc<RefCell<Vec<String>>> = _t4;
            let mut x: i32 = 0i32;
            loop {
                if x >= (perms.borrow().len() as i32) { break; }
                let _t5 = StringBuilder::new()?.append_str(Clone::clone(&base))?;
                let _t6 = _t5.append_str(Clone::clone(&Clone::clone(&perms.borrow()[x as usize])))?;
                let _t7 = _t6.toString()?;
                let mut next: String = _t7;
                let _vdispatch8: bool = if let Some(_d) = dst.0.as_any().downcast_ref::<HashMap_EntrySet>() { _d.add(Object::from_any(next.clone()))? } else if let Some(_d) = dst.0.as_any().downcast_ref::<HashMap_KeySet>() { _d.add(Object::from_any(next.clone()))? } else if let Some(_d) = dst.0.as_any().downcast_ref::<TreeMap_EntrySet>() { _d.add(Object::from_any(next.clone()))? } else if let Some(_d) = dst.0.as_any().downcast_ref::<LinkedHashSet<Object>>() { _d.add(Object::from_any(next.clone()))? } else if let Some(_d) = dst.0.as_any().downcast_ref::<AbstractSet<Object>>() { _d.add(Object::from_any(next.clone()))? } else if let Some(_d) = dst.0.as_any().downcast_ref::<HashSet<Object>>() { _d.add(Object::from_any(next.clone()))? } else if let Some(_d) = dst.0.as_any().downcast_ref::<Object>() { _d.add(Object::from_any(next.clone()))? } else if let Some(__f) = dst.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Object::from_any(next.clone()))? } else { Default::default() };
                let _t9: String = Pattern::composeOneStep(Clone::clone(&next))?;
                next = _t9;
                if !_is_jnull(&next) {
                    Pattern::produceEquivalentAlternation(Clone::clone(&next), Clone::clone(&dst))?;
                }
                x = x.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "producePermutations", descriptor = "(Ljava/lang/String;)[Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn producePermutations(mut input: String) -> Result<Rc<RefCell<Vec<String>>>> {
            let _t0 = input.length()?;
            let _t1: i32 = Pattern::countChars(Object::from_any(input.clone()), 0i32, 1i32)?;
            if _t0 == _t1 {
                let mut _arr2: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
                _arr2.borrow_mut()[0i32 as usize] = Clone::clone(&input);
                return Ok(_arr2);
            }
            let _t2 = input.length()?;
            let _t3: i32 = Pattern::countChars(Object::from_any(input.clone()), 0i32, 2i32)?;
            let _t4: i32 = Character::codePointAt_seq_i(Object::from_any(input.clone()), 0i32)?;
            let mut c0: i32 = _t4;
            let _t5: i32 = Character::charCount(c0)?;
            let _t6: i32 = Character::codePointAt_seq_i(Object::from_any(input.clone()), _t5)?;
            let mut c1: i32 = _t6;
            let _t7: i32 = Pattern::getClass(c1)?;
            let _t8: i32 = Pattern::getClass(c0)?;
            if _t7 == _t8 {
                let mut _arr9: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
                _arr9.borrow_mut()[0i32 as usize] = Clone::clone(&input);
                return Ok(_arr9);
            }
            let mut _arr9: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 2i32 as usize]));
            let mut result: Rc<RefCell<Vec<String>>> = _arr9;
            result.borrow_mut()[0i32 as usize] = Clone::clone(&input);
            let mut sb = StringBuilder::new_i(2i32)?;
            let _t10 = sb.appendCodePoint(c1)?;
            let _t11 = sb.appendCodePoint(c0)?;
            let _t12 = sb.toString()?;
            result.borrow_mut()[1i32 as usize] = Clone::clone(&_t12);
            return Ok(result);
            let _t13: i32 = Pattern::countCodePoints(Object::from_any(input.clone()))?;
            c0 = _t13;
            if c0 > 12i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            c1 = 1i32;
            let mut result: i32 = 2i32;
            loop {
                if result > c0 { break; }
                c1 = (c1).wrapping_mul(result);
                result = result.wrapping_add(1i32);
            }
            let mut _arr14: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); c1 as usize]));
            let mut result: Rc<RefCell<Vec<String>>> = _arr14;
            let mut _arr15: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; c0 as usize]));
            let mut sb: Rc<RefCell<Vec<i32>>> = _arr15;
            let mut x: i32 = 0i32;
            let mut i: i32 = 0i32;
            loop {
                if x >= c0 { break; }
                let _t16: i32 = Character::codePointAt_seq_i(Object::from_any(input.clone()), i)?;
                let mut c: i32 = _t16;
                let _t17: i32 = Pattern::getClass(c)?;
                sb.borrow_mut()[x as usize] = _t17;
                let _t18: i32 = Character::charCount(c)?;
                i = (i).wrapping_add(_t18);
                x = x.wrapping_add(1i32);
            }
            x = 0i32;
            let mut c: i32 = 0i32;
            let mut offset: i32 = 0i32;
            loop {
                if c >= c0 { break; }
                let _t16: i32 = Pattern::countChars(Object::from_any(input.clone()), offset, 1i32)?;
                i = _t16;
                let mut y = (c).wrapping_sub(1i32);
                loop {
                    if (y<0) { break; }
                    if sb.borrow()[y as usize] == sb.borrow()[c as usize] {
                        break;
                    }
                    y = y.wrapping_sub(1i32);
                }
                let mut y = StringBuilder::new_str(Clone::clone(&input))?;
                let _t17 = y.delete(offset, (offset).wrapping_add(i))?;
                let _t18 = _t17.toString()?;
                let mut otherChars: String = _t18;
                let _t19: Rc<RefCell<Vec<String>>> = Pattern::producePermutations(Clone::clone(&otherChars))?;
                let mut subResult: Rc<RefCell<Vec<String>>> = _t19;
                let _t20 = input.substring_i_i(offset, (offset).wrapping_add(i))?;
                let mut prefix: String = _t20;
                let mut local_13: Rc<RefCell<Vec<String>>> = subResult;
                let mut local_14 = (local_13.borrow().len() as i32);
                let mut local_15: i32 = 0i32;
                loop {
                    if local_15 >= local_14 { break; }
                    let mut sre = Clone::clone(&local_13.borrow()[local_15 as usize]);
                    x = x.wrapping_add(1i32);
                    let _t21 = StringBuilder::new()?.append_str(Clone::clone(&prefix))?;
                    let _t22 = _t21.append_str(Clone::clone(&sre))?;
                    let _t23 = _t22.toString()?;
                    result.borrow_mut()[x as usize] = Clone::clone(&_t23);
                    local_15 = local_15.wrapping_add(1i32);
                }
                c = c.wrapping_add(1i32);
                offset = (offset).wrapping_add(i);
            }
            let mut _arr16: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); x as usize]));
            let mut c: Rc<RefCell<Vec<String>>> = _arr16;
            System::arraycopy(Object::from_any(result.clone()), 0i32, Object::from_any(c.clone()), 0i32, x)?;
            Ok(c)
        }

        #[java_method(name = "getClass", descriptor = "(I)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getClass(mut c: i32) -> Result<i32> {
            let _t0: i32 = Normalizer::getCombiningClass(c)?;
            Ok(_t0)
        }

        #[java_method(name = "composeOneStep", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn composeOneStep(mut input: String) -> Result<String> {
            let _t0: i32 = Pattern::countChars(Object::from_any(input.clone()), 0i32, 2i32)?;
            let mut len: i32 = _t0;
            let _t1 = input.substring_i_i(0i32, len)?;
            let mut firstTwoCharacters: String = _t1;
            let _t2: String = Normalizer::normalize(Object::from_any(firstTwoCharacters.clone()), Clone::clone(&Normalizer_Form::NFC()))?;
            let mut result: String = _t2;
            let _t3 = result.equals(Object::from_any(firstTwoCharacters.clone()))?;
            if _t3 {
                return Ok(Default::default());
            }
            let _t4 = input.substring_i(len)?;
            let mut remainder: String = _t4;
            let _t5 = StringBuilder::new()?.append_str(Clone::clone(&result))?;
            let _t6 = _t5.append_str(Clone::clone(&remainder))?;
            let _t7 = _t6.toString()?;
            Ok(_t7)
        }

        #[java_method(name = "RemoveQEQuoting", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn RemoveQEQuoting(&self) -> Result<()> {
            let this = self;
            let mut pLen = this.__get_patternLength();
            let mut i: i32 = 0i32;
            loop {
                if i >= (pLen).wrapping_sub(1i32) { break; }
                if this.__get_temp().borrow()[i as usize] != 92i32 {
                    i = i.wrapping_add(1i32);
                    continue;
                }
                if this.__get_temp().borrow()[(i).wrapping_add(1i32) as usize] != 81i32 {
                    i = i.wrapping_add(2i32);
                    continue;
                }
                break;
            }
            if i >= (pLen).wrapping_sub(1i32) {
                return Ok(());
            }
            let mut j: i32 = i;
            i = i.wrapping_add(2i32);
            let _t0: i32 = Math::multiplyExact_i_i(3i32, (pLen).wrapping_sub(i))?;
            let _t1: i32 = Math::addExact_i_i((j).wrapping_add(2i32), _t0)?;
            let mut newTempLen: i32 = _t1;
            let mut _arr2: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; newTempLen as usize]));
            let mut ae: Rc<RefCell<Vec<i32>>> = _arr2;
            System::arraycopy(Object::from_any(this.__get_temp().clone()), 0i32, Object::from_any(ae.clone()), 0i32, j)?;
            let mut inQuote: i32 = 1i32;
            let mut beginQuote: i32 = 1i32;
            loop {
                if i >= pLen { break; }
                i = i.wrapping_add(1i32);
                let mut c = this.__get_temp().borrow()[i as usize];
                let _t3: bool = ASCII::isAscii(c)?;
                let _t4: bool = ASCII::isAlpha(c)?;
                if _t4 {
                    j = j.wrapping_add(1i32);
                    ae.borrow_mut()[j as usize] = c;
                } else {
                    let _t5: bool = ASCII::isDigit(c)?;
                    if _t5 {
                        if (beginQuote!=0) {
                            j = j.wrapping_add(1i32);
                            ae.borrow_mut()[j as usize] = 92i32;
                            j = j.wrapping_add(1i32);
                            ae.borrow_mut()[j as usize] = 120i32;
                            j = j.wrapping_add(1i32);
                            ae.borrow_mut()[j as usize] = 51i32;
                        }
                        j = j.wrapping_add(1i32);
                        ae.borrow_mut()[j as usize] = c;
                    } else {
                        if c != 92i32 {
                            if (inQuote!=0) {
                                j = j.wrapping_add(1i32);
                                ae.borrow_mut()[j as usize] = 92i32;
                            }
                            j = j.wrapping_add(1i32);
                            ae.borrow_mut()[j as usize] = c;
                        } else {
                            if (inQuote!=0) {
                                if this.__get_temp().borrow()[i as usize] == 69i32 {
                                    i = i.wrapping_add(1i32);
                                    inQuote = 0i32;
                                } else {
                                    j = j.wrapping_add(1i32);
                                    ae.borrow_mut()[j as usize] = 92i32;
                                    j = j.wrapping_add(1i32);
                                    ae.borrow_mut()[j as usize] = 92i32;
                                }
                            } else {
                                if this.__get_temp().borrow()[i as usize] == 81i32 {
                                    i = i.wrapping_add(1i32);
                                    inQuote = 1i32;
                                    beginQuote = 1i32;
                                    continue;
                                }
                                j = j.wrapping_add(1i32);
                                ae.borrow_mut()[j as usize] = c;
                                if i != pLen {
                                    j = j.wrapping_add(1i32);
                                    i = i.wrapping_add(1i32);
                                    ae.borrow_mut()[j as usize] = this.__get_temp().borrow()[i as usize];
                                }
                            }
                        }
                    }
                }
                beginQuote = 0i32;
            }
            this.__set_patternLength(j);
            let _t3: Rc<RefCell<Vec<i32>>> = Arrays::copyOf_arr_i_i(Clone::clone(&ae), (j).wrapping_add(2i32))?;
            this.__set_temp(Clone::clone(&_t3));
            Ok(())
        }

        #[java_method(name = "compile", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: compile()V
        pub fn compile(&self) -> Result<()> {
            let this = self;
            let _t0 = this.has(128i32)?;
            if _t0 {
                let _t1 = this.has(16i32)?;
                if !(_t1) {
                    let _t2: String = Pattern::normalize(Clone::clone(&this.__get_pattern()))?;
                    this.__set_normalizedPattern(Clone::clone(&_t2));
                } else {
                    this.__set_normalizedPattern(Clone::clone(&this.__get_pattern()));
                }
            } else {
                this.__set_normalizedPattern(Clone::clone(&this.__get_pattern()));
            }
            let _t1 = this.__get_normalizedPattern().length()?;
            this.__set_patternLength(_t1);
            let mut _arr2: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (this.__get_patternLength()).wrapping_add(2i32) as usize]));
            this.__set_temp(Clone::clone(&_arr2));
            this.__set_hasSupplementary((0i32 != 0i32));
            let mut count: i32 = 0i32;
            let mut x: i32 = 0i32;
            loop {
                if x >= this.__get_patternLength() { break; }
                let _t3 = this.__get_normalizedPattern().codePointAt(x)?;
                let mut c: i32 = _t3;
                let _t4: bool = Pattern::isSupplementary(c)?;
                if _t4 {
                    this.__set_hasSupplementary((1i32 != 0i32));
                }
                count = count.wrapping_add(1i32);
                this.__get_temp().borrow_mut()[count as usize] = c;
                let _t5: i32 = Character::charCount(c)?;
                x = (x).wrapping_add(_t5);
            }
            this.__set_patternLength(count);
            let _t3 = this.has(16i32)?;
            if !(_t3) {
                this.RemoveQEQuoting()?;
            }
            let mut _arr4: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 32i32 as usize]));
            this.__set_buffer(Clone::clone(&_arr4));
            let mut _arr5: Rc<RefCell<Vec<Pattern_GroupHead>>> = Rc::new(RefCell::new(vec![Default::default(); 10i32 as usize]));
            this.__set_groupNodes(Clone::clone(&_arr5));
            this.__set_namedGroups(Clone::clone(&Object::default()));
            this.__set_topClosureNodes(Object::from_any(ArrayList::<Object>::new_i(10i32)?.clone()));
            let _t6 = this.has(16i32)?;
            if _t6 {
                let _t7 = this.newSlice(Clone::clone(&this.__get_temp()), this.__get_patternLength(), this.__get_hasSupplementary())?;
                this.__set_matchRoot(Clone::clone(&_t7));
                this.__get_matchRoot().__set_next(Clone::clone(&Pattern::lastAccept()));
            } else {
                let _t7 = this.expr(Clone::clone(&Pattern::lastAccept()))?;
                this.__set_matchRoot(Clone::clone(&_t7));
                let _t8 = this.peek()?;
                if _t8 == 41i32 {
                    let _t9 = this.error(Clone::clone(&String::from("Unmatched closing ')'")))?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                if this.__get_temp().borrow()[(this.__get_patternLength()).wrapping_sub(1i32) as usize] == 92i32 {
                    let _t9 = this.error(Clone::clone(&String::from("Unescaped trailing backslash")))?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                let _t9 = this.error(Clone::clone(&String::from("Unexpected internal error")))?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if false {
                let _t7: Pattern_Node = Pattern_BnM::optimize(Clone::clone(&this.__get_matchRoot()))?;
                this.__set_root(Clone::clone(&_t7));
                this.__set_root(<_ as Into<Pattern_Node>>::into((if this.__get_hasSupplementary() { Pattern_StartS::new(Clone::clone(&this.__get_matchRoot()))? } else { Pattern_Start::new(Clone::clone(&this.__get_matchRoot()))? })));
            } else {
                if false {
                    this.__set_root(Clone::clone(&this.__get_matchRoot()));
                } else {
                    this.__set_root(<_ as Into<Pattern_Node>>::into((if this.__get_hasSupplementary() { Pattern_StartS::new(Clone::clone(&this.__get_matchRoot()))? } else { Pattern_Start::new(Clone::clone(&this.__get_matchRoot()))? })));
                }
            }
            let _vdispatch7: Object = if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.iterator()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.iterator()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.iterator()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.iterator()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.iterator()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.iterator()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.iterator()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.iterator()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Object>() { _d.iterator()? } else if let Some(__f) = this.__get_topClosureNodes().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut x: Object = _vdispatch7;
            loop {
                let _vdispatch8: bool = if let Some(_d) = x.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = x.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = x.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = x.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = x.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = x.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = x.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
                if !(_vdispatch8) { break; }
                let _vdispatch8: Object = if let Some(_d) = x.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = x.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = x.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.next()? } else if let Some(_d) = x.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(_d) = x.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.next()? } else if let Some(_d) = x.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = x.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let mut node = (_vdispatch8).downcast::<Pattern_Node>();
                if false {
                    this.__set_localTCNCount((this.__get_localTCNCount()).wrapping_add(1i32));
                    Default::default().__set_posIndex(this.__get_localTCNCount());
                }
            }
            this.__set_temp(Default::default());
            this.__set_buffer(Default::default());
            this.__set_groupNodes(Default::default());
            this.__set_patternLength(0i32);
            this.__set_compiled((1i32 != 0i32));
            this.__set_topClosureNodes(Clone::clone(&Object::default()));
            Ok(())
        }

        #[java_method(name = "namedGroupsMap", descriptor = "()Ljava/util/Map;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Map<Ljava/lang/String;Ljava/lang/Integer;>;")]
        pub fn namedGroupsMap(&self) -> Result<Object> {
            let this = self;
            let mut groups = this.__get_namedGroups();
            if _is_jnull(&groups) {
                let mut groups = HashMap::<Object, Object>::new_i(2i32)?;
                this.__set_namedGroups(Object::from_any(HashMap::<Object, Object>::new_i(2i32)?.clone()));
            }
            Ok(Object::from_any(groups.clone()))
        }

        #[java_method(name = "namedGroups", descriptor = "()Ljava/util/Map;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Map<Ljava/lang/String;Ljava/lang/Integer;>;")]
        pub fn namedGroups(&self) -> Result<Object> {
            let this = self;
            let _t0 = this.namedGroupsMap()?;
            let _t1: Object = Map::<Object, Object>::copyOf(Clone::clone(&_t0))?;
            Ok(_t1)
        }

        #[java_method(name = "has", descriptor = "(I)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn has(&self, mut f: i32) -> Result<bool> {
            let this = self;
            Ok(((this.__get_flags0()&f)!=0))
        }

        #[java_method(name = "accept", descriptor = "(ILjava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn accept(&self, mut ch: i32, mut s: String) -> Result<()> {
            let this = self;
            this.__set_cursor((this.__get_cursor()).wrapping_add(1i32));
            let mut testChar = this.__get_temp().borrow()[this.__get_cursor() as usize];
            let _t0 = this.has(4i32)?;
            if _t0 {
                let _t1 = this.parsePastWhitespace(testChar)?;
                testChar = _t1;
            }
            if ch != testChar {
                let _t1 = this.error(Clone::clone(&s))?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "mark", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mark(&self, mut c: i32) -> Result<()> {
            let this = self;
            this.__get_temp().borrow_mut()[this.__get_patternLength() as usize] = c;
            Ok(())
        }

        #[java_method(name = "peek", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn peek(&self) -> Result<i32> {
            let this = self;
            let mut ch = this.__get_temp().borrow()[this.__get_cursor() as usize];
            let _t0 = this.has(4i32)?;
            if _t0 {
                let _t1 = this.peekPastWhitespace(ch)?;
                ch = _t1;
            }
            Ok(ch)
        }

        #[java_method(name = "read", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn read(&self) -> Result<i32> {
            let this = self;
            this.__set_cursor((this.__get_cursor()).wrapping_add(1i32));
            let mut ch = this.__get_temp().borrow()[this.__get_cursor() as usize];
            let _t0 = this.has(4i32)?;
            if _t0 {
                let _t1 = this.parsePastWhitespace(ch)?;
                ch = _t1;
            }
            Ok(ch)
        }

        #[java_method(name = "readEscaped", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn readEscaped(&self) -> Result<i32> {
            panic!("stub: java/util/regex/Pattern.readEscaped:()I")
        }

        #[java_method(name = "next", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn next(&self) -> Result<i32> {
            let this = self;
            this.__set_cursor((this.__get_cursor()).wrapping_add(1i32));
            let mut ch = this.__get_temp().borrow()[(this.__get_cursor()).wrapping_add(1i32) as usize];
            let _t0 = this.has(4i32)?;
            if _t0 {
                let _t1 = this.peekPastWhitespace(ch)?;
                ch = _t1;
            }
            Ok(ch)
        }

        #[java_method(name = "nextEscaped", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextEscaped(&self) -> Result<i32> {
            let this = self;
            this.__set_cursor((this.__get_cursor()).wrapping_add(1i32));
            let mut ch = this.__get_temp().borrow()[(this.__get_cursor()).wrapping_add(1i32) as usize];
            Ok(ch)
        }

        #[java_method(name = "peekPastWhitespace", descriptor = "(I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn peekPastWhitespace(&self, mut ch: i32) -> Result<i32> {
            let this = self;
            loop {
                let _t0: bool = ASCII::isSpace(ch)?;
                if ch != 35i32 { break; }
                loop {
                    let _t0: bool = ASCII::isSpace(ch)?;
                    if !(_t0) { break; }
                    this.__set_cursor((this.__get_cursor()).wrapping_add(1i32));
                    ch = this.__get_temp().borrow()[(this.__get_cursor()).wrapping_add(1i32) as usize];
                }
                let _t0 = this.peekPastLine()?;
                ch = _t0;
            }
            Ok(ch)
        }

        #[java_method(name = "parsePastWhitespace", descriptor = "(I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parsePastWhitespace(&self, mut ch: i32) -> Result<i32> {
            let this = self;
            loop {
                let _t0: bool = ASCII::isSpace(ch)?;
                if ch != 35i32 { break; }
                loop {
                    let _t0: bool = ASCII::isSpace(ch)?;
                    if !(_t0) { break; }
                    this.__set_cursor((this.__get_cursor()).wrapping_add(1i32));
                    ch = this.__get_temp().borrow()[this.__get_cursor() as usize];
                }
                let _t0 = this.parsePastLine()?;
                ch = _t0;
            }
            Ok(ch)
        }

        #[java_method(name = "parsePastLine", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parsePastLine(&self) -> Result<i32> {
            let this = self;
            this.__set_cursor((this.__get_cursor()).wrapping_add(1i32));
            let mut ch = this.__get_temp().borrow()[this.__get_cursor() as usize];
            loop {
                if (ch==0) { break; }
                let _t0 = this.isLineSeparator(ch)?;
                if !(_t0) {
                    this.__set_cursor((this.__get_cursor()).wrapping_add(1i32));
                    ch = this.__get_temp().borrow()[this.__get_cursor() as usize];
                    continue;
                }
                break;
            }
            if this.__get_cursor() > this.__get_patternLength() {
                this.__set_cursor(this.__get_patternLength());
                this.__set_cursor((this.__get_cursor()).wrapping_add(1i32));
                ch = this.__get_temp().borrow()[this.__get_cursor() as usize];
            }
            Ok(ch)
        }

        #[java_method(name = "peekPastLine", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn peekPastLine(&self) -> Result<i32> {
            let this = self;
            this.__set_cursor((this.__get_cursor()).wrapping_add(1i32));
            let mut ch = this.__get_temp().borrow()[(this.__get_cursor()).wrapping_add(1i32) as usize];
            loop {
                if (ch==0) { break; }
                let _t0 = this.isLineSeparator(ch)?;
                if !(_t0) {
                    this.__set_cursor((this.__get_cursor()).wrapping_add(1i32));
                    ch = this.__get_temp().borrow()[(this.__get_cursor()).wrapping_add(1i32) as usize];
                    continue;
                }
                break;
            }
            if this.__get_cursor() > this.__get_patternLength() {
                this.__set_cursor(this.__get_patternLength());
                ch = this.__get_temp().borrow()[this.__get_cursor() as usize];
            }
            Ok(ch)
        }

        #[java_method(name = "isLineSeparator", descriptor = "(I)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLineSeparator(&self, mut ch: i32) -> Result<bool> {
            let this = self;
            let _t0 = this.has(1i32)?;
            return Ok(ch == 10i32);
            Ok(ch == 133i32)
        }

        #[java_method(name = "skip", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn skip(&self) -> Result<i32> {
            let this = self;
            let mut i = this.__get_cursor();
            let mut ch = this.__get_temp().borrow()[(i).wrapping_add(1i32) as usize];
            this.__set_cursor((i).wrapping_add(2i32));
            Ok(ch)
        }

        #[java_method(name = "unread", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unread(&self) -> Result<()> {
            let this = self;
            this.__set_cursor((this.__get_cursor()).wrapping_sub(1i32));
            Ok(())
        }

        #[java_method(name = "error", descriptor = "(Ljava/lang/String;)Ljava/util/regex/PatternSyntaxException;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn error(&self, mut s: String) -> Result<PatternSyntaxException> {
            let this = self;
            Ok(PatternSyntaxException::new(Clone::clone(&s), Clone::clone(&this.__get_normalizedPattern()), (this.__get_cursor()).wrapping_sub(1i32))?)
        }

        #[java_method(name = "findSupplementary", descriptor = "(II)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn findSupplementary(&self, mut start: i32, mut end: i32) -> Result<bool> {
            let this = self;
            let mut i: i32 = start;
            loop {
                if i >= end { break; }
                let _t0: bool = Pattern::isSupplementary(this.__get_temp().borrow()[i as usize])?;
                if _t0 {
                    return Ok((1i32 != 0i32));
                }
                i = i.wrapping_add(1i32);
            }
            Ok((0i32 != 0i32))
        }

        #[java_method(name = "isSupplementary", descriptor = "(I)Z", access = "private", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSupplementary(mut ch: i32) -> Result<bool> {
            let _t0: bool = Character::isSurrogate(((((ch) as u16 as i32)) as u16))?;
            Ok(!(!(_t0)))
        }

        #[java_method(name = "expr", descriptor = "(Ljava/util/regex/Pattern$Node;)Ljava/util/regex/Pattern$Node;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn expr(&self, mut end: Pattern_Node) -> Result<Pattern_Node> {
            let this = self;
            let mut prev: Object = Object::default();
            let mut firstTail: Object = Object::default();
            let mut branch: Object = Object::default();
            let mut branchConn: Object = Object::default();
            let mut prev: Pattern_Node = Default::default();
            let mut branchConn = Default::default();
        let mut node: Pattern_Node = Default::default();
            loop {
                let _t0 = this.sequence(Clone::clone(&end))?;
                node = _t0;
                let mut nodeTail = this.__get_root();
                if _is_jnull(&prev) {
                    prev = node;
                    let mut firstTail: Pattern_Node = nodeTail;
                } else {
                    if _is_jnull(&branchConn) {
                        branchConn = Pattern_BranchConn::new()?;
                        branchConn.__set_next(Clone::clone(&end));
                    }
                    if Object::from_any(node.clone()) == Object::from_any(end.clone()) {
                        node = Object::default();
                    } else {
                        nodeTail.__set_next(Clone::clone(&<_ as Into<Pattern_Node>>::into(branchConn)));
                    }
                    if prev == branch {
                        if let Some(__f) = branch.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Pattern_Node) -> crate::error::Result<()>>>() { (__f)(Clone::clone(&node))?; }
                    } else {
                        if prev == Object::from_any(end.clone()) {
                            prev = Object::default();
                        } else {
                            firstTail.__set_next(Clone::clone(&<_ as Into<Pattern_Node>>::into(branchConn)));
                        }
                        let mut branch = Pattern_Branch::new(Clone::clone(&prev), Clone::clone(&node), Clone::clone(&branchConn).into())?;
                        let mut prev = Pattern_Branch::new(Clone::clone(&prev), Clone::clone(&node), Clone::clone(&branchConn).into())?;
                    }
                }
                let _t1 = this.peek()?;
                if _t1 != 124i32 {
                    return Ok(prev);
                }
                let _t2 = this.next()?;
            }
        }

        #[java_method(name = "sequence", descriptor = "(Ljava/util/regex/Pattern$Node;)Ljava/util/regex/Pattern$Node;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sequence(&self, mut end: Pattern_Node) -> Result<Pattern_Node> {
            let this = self;
            let mut head: Object = Object::default();
            let mut tail: Object = Object::default();
            let mut head: Pattern_Node = Default::default();
            let mut tail: Pattern_Node = Default::default();
            loop {
                let _t0 = this.peek()?;
                let mut ch: i32 = _t0;
                match ch {
                    0 => {
                        if this.__get_cursor() >= this.__get_patternLength() {
                        } else {
                            let _t1 = this.atom()?;
                            let mut node: Pattern_Node = _t1;
                            let _t2 = this.closure(Clone::clone(&node))?;
                            node = _t2;
                            if _is_jnull(&head) {
                                tail = node;
                                head = tail;
                            } else {
                                tail.__set_next(Clone::clone(&node));
                                let mut tail: Pattern_Node = node;
                            }
                            continue;
                        }
                    }
                    36 => {
                        let _t1 = this.next()?;
                        let _t2 = this.has(1i32)?;
                        if _t2 {
                            let _t3 = this.has(8i32)?;
                            let mut node = Pattern_UnixDollar::new(_t3)?;
                        } else {
                            let _t3 = this.has(8i32)?;
                            let mut node = Pattern_Dollar::new(_t3)?;
                        }
                    }
                    40 => {
                        let _t1 = this.group0()?;
                        let mut node: Pattern_Node = _t1;
                        continue;
                        if _is_jnull(&head) {
                            let mut head: Pattern_Node = node;
                        } else {
                            tail.__set_next(Clone::clone(&node));
                        }
                        let mut tail = this.__get_root();
                        continue;
                        let _t2 = this.has(128i32)?;
                        if _t2 {
                            let _t3 = this.has(16i32)?;
                            if !(_t3) {
                                let _t4 = this.clazz((1i32 != 0i32))?;
                                let mut node = Pattern_NFCCharProperty::new(Clone::clone(&_t4))?;
                            } else {
                                let _t4 = this.clazz((1i32 != 0i32))?;
                                let _t5 = this.newCharProperty(Clone::clone(&_t4))?;
                                let mut node: Pattern_CharProperty = _t5;
                            }
                        } else {
                            let _t3 = this.clazz((1i32 != 0i32))?;
                            let _t4 = this.newCharProperty(Clone::clone(&_t3))?;
                            let mut node: Pattern_CharProperty = _t4;
                        }
                    }
                    41 => {
                    }
                    42 => {
                        let _t1 = this.next()?;
                        let _t2 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Dangling meta character '")))?;
                        let _t3 = _t2.append_c(((((ch) as u16 as i32)) as u16))?;
                        let _t4 = _t3.append_str(Clone::clone(&String::from("'")))?;
                        let _t5 = _t4.toString()?;
                        let _t6 = this.error(Clone::clone(&_t5))?;
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                    43 => {
                        let _t1 = this.next()?;
                        let _t2 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Dangling meta character '")))?;
                        let _t3 = _t2.append_c(((((ch) as u16 as i32)) as u16))?;
                        let _t4 = _t3.append_str(Clone::clone(&String::from("'")))?;
                        let _t5 = _t4.toString()?;
                        let _t6 = this.error(Clone::clone(&_t5))?;
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                    46 => {
                        let _t1 = this.next()?;
                        let _t2 = this.has(32i32)?;
                        if _t2 {
                            let _t3: Object = Pattern::ALL()?;
                            let mut node = Pattern_CharProperty::new(Clone::clone(&_t3))?;
                        } else {
                            let _t3 = this.has(1i32)?;
                            if _t3 {
                                let _t4: Object = Pattern::UNIXDOT()?;
                                let mut node = Pattern_CharProperty::new(Clone::clone(&_t4))?;
                            } else {
                                let _t4: Object = Pattern::DOT()?;
                                let mut node = Pattern_CharProperty::new(Clone::clone(&_t4))?;
                            }
                        }
                    }
                    63 => {
                        let _t1 = this.next()?;
                        let _t2 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Dangling meta character '")))?;
                        let _t3 = _t2.append_c(((((ch) as u16 as i32)) as u16))?;
                        let _t4 = _t3.append_str(Clone::clone(&String::from("'")))?;
                        let _t5 = _t4.toString()?;
                        let _t6 = this.error(Clone::clone(&_t5))?;
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                    91 => {
                        let _t1 = this.has(128i32)?;
                        if _t1 {
                            let _t2 = this.has(16i32)?;
                            if !(_t2) {
                                let _t3 = this.clazz((1i32 != 0i32))?;
                                let mut node = Pattern_NFCCharProperty::new(Clone::clone(&_t3))?;
                            } else {
                                let _t3 = this.clazz((1i32 != 0i32))?;
                                let _t4 = this.newCharProperty(Clone::clone(&_t3))?;
                                let mut node: Pattern_CharProperty = _t4;
                            }
                        } else {
                            let _t2 = this.clazz((1i32 != 0i32))?;
                            let _t3 = this.newCharProperty(Clone::clone(&_t2))?;
                            let mut node: Pattern_CharProperty = _t3;
                        }
                    }
                    92 => {
                        let _t1 = this.nextEscaped()?;
                        ch = _t1;
                        if ch == 80i32 {
                            let mut oneLetter: i32 = 1i32;
                            let mut comp = (ch == 80i32) as i32;
                            let _t2 = this.next()?;
                            ch = _t2;
                            if ch != 123i32 {
                                this.unread()?;
                            } else {
                                oneLetter = 0i32;
                            }
                            let _t3 = this.has(128i32)?;
                            if _t3 {
                                let _t4 = this.has(16i32)?;
                                if !(_t4) {
                                    let _t5 = this.family((oneLetter != 0i32), (comp != 0i32))?;
                                    let mut node = Pattern_NFCCharProperty::new(Clone::clone(&_t5))?;
                                } else {
                                    let _t5 = this.family((oneLetter != 0i32), (comp != 0i32))?;
                                    let _t6 = this.newCharProperty(Clone::clone(&_t5))?;
                                    let mut node: Pattern_CharProperty = _t6;
                                }
                            } else {
                                let _t4 = this.family((oneLetter != 0i32), (comp != 0i32))?;
                                let _t5 = this.newCharProperty(Clone::clone(&_t4))?;
                                let mut node: Pattern_CharProperty = _t5;
                            }
                        } else {
                            this.unread()?;
                            let _t2 = this.atom()?;
                            let mut node: Pattern_Node = _t2;
                        }
                    }
                    93 => {
                        let _t1 = this.atom()?;
                        let mut node: Pattern_Node = _t1;
                        let _t2 = this.next()?;
                        let _t3 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Dangling meta character '")))?;
                        let _t4 = _t3.append_c(((((ch) as u16 as i32)) as u16))?;
                        let _t5 = _t4.append_str(Clone::clone(&String::from("'")))?;
                        let _t6 = _t5.toString()?;
                        let _t7 = this.error(Clone::clone(&_t6))?;
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                    94 => {
                        let _t1 = this.next()?;
                        let _t2 = this.has(8i32)?;
                        if _t2 {
                            let _t3 = this.has(1i32)?;
                            if _t3 {
                                let mut node = Pattern_UnixCaret::new()?;
                            } else {
                                let mut node = Pattern_Caret::new()?;
                            }
                        } else {
                            let mut node = Pattern_Begin::new()?;
                        }
                    }
                    124 => {
                    }
                    125 => {
                        let _t1 = this.atom()?;
                        let mut node: Pattern_Node = _t1;
                        let _t2 = this.next()?;
                        let _t3 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Dangling meta character '")))?;
                        let _t4 = _t3.append_c(((((ch) as u16 as i32)) as u16))?;
                        let _t5 = _t4.append_str(Clone::clone(&String::from("'")))?;
                        let _t6 = _t5.toString()?;
                        let _t7 = this.error(Clone::clone(&_t6))?;
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                    _ => {
                        let _t1 = this.atom()?;
                        let mut node: Pattern_Node = _t1;
                        let _t2 = this.closure(Clone::clone(&node))?;
                        node = _t2;
                        if _is_jnull(&head) {
                            let mut tail: Pattern_Node = node;
                            let mut head: Pattern_Node = tail;
                        } else {
                            tail.__set_next(Clone::clone(&node));
                            let mut tail: Pattern_Node = node;
                        }
                        continue;
                    }
                }
            }
            if _is_jnull(&head) {
                return Ok(end);
            }
            tail.__set_next(Clone::clone(&end));
            this.__set_root(Clone::clone(&tail));
            Ok(Default::default())
        }

        #[java_method(name = "atom", descriptor = "()Ljava/util/regex/Pattern$Node;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn atom(&self) -> Result<Pattern_Node> {
            let this = self;
            let mut first: i32 = 0i32;
            let mut prev: i32 = -1i32;
            let mut hasSupplementary: i32 = 0i32;
            let _t0 = this.peek()?;
            let mut ch: i32 = _t0;
            loop {
                match ch {
                    0 => {
                        if this.__get_cursor() >= this.__get_patternLength() {
                            break;
                        }
                    }
                    36 => {
                    }
                    40 => {
                    }
                    41 => {
                    }
                    42 => {
                        this.__set_cursor(prev);
                        first = first.wrapping_sub(1i32);
                    }
                    43 => {
                        this.__set_cursor(prev);
                        first = first.wrapping_sub(1i32);
                    }
                    46 => {
                    }
                    63 => {
                        this.__set_cursor(prev);
                        first = first.wrapping_sub(1i32);
                    }
                    91 => {
                    }
                    92 => {
                        let _t1 = this.nextEscaped()?;
                        ch = _t1;
                        if (first>0) {
                            this.unread()?;
                            break;
                        }
                    }
                    94 => {
                    }
                    123 => {
                        this.__set_cursor(prev);
                        first = first.wrapping_sub(1i32);
                    }
                    124 => {
                    }
                    _ => {
                        prev = this.__get_cursor();
                        this.append(ch, first)?;
                        first = first.wrapping_add(1i32);
                        let _t1: bool = Pattern::isSupplementary(ch)?;
                        if _t1 {
                            hasSupplementary = 1i32;
                        }
                        let _t2 = this.next()?;
                        ch = _t2;
                        continue;
                    }
                }
                if (panic!("stack underflow") as i32) <= (panic!("stack underflow") as i32) { break; }
                this.__set_cursor(prev);
                first = first.wrapping_sub(1i32);
                break;
                break;
                let _t1 = this.nextEscaped()?;
                ch = _t1;
                if (first>0) {
                    this.unread()?;
                    break;
                }
                let mut comp = (ch == 80i32) as i32;
                let mut oneLetter: i32 = 1i32;
                let _t2 = this.next()?;
                ch = _t2;
                if ch != 123i32 {
                    this.unread()?;
                } else {
                    oneLetter = 0i32;
                }
                let _t3 = this.has(128i32)?;
                let _t4 = this.has(16i32)?;
                if !(_t4) {
                    let _t5 = this.family((oneLetter != 0i32), (comp != 0i32))?;
                    return Ok(<_ as Into<Pattern_Node>>::into(Pattern_NFCCharProperty::new(Clone::clone(&_t5))?));
                }
                let _t5 = this.family((oneLetter != 0i32), (comp != 0i32))?;
                let _t6 = this.newCharProperty(Clone::clone(&_t5))?;
                return Ok(<_ as Into<Pattern_Node>>::into(_t6));
                this.unread()?;
                prev = this.__get_cursor();
                let _t7 = this.escape((0i32 != 0i32), (first==0), (0i32 != 0i32))?;
                ch = _t7;
                this.append(ch, first)?;
                first = first.wrapping_add(1i32);
                let _t8: bool = Pattern::isSupplementary(ch)?;
                if _t8 {
                    hasSupplementary = 1i32;
                }
                let _t9 = this.peek()?;
                ch = _t9;
                continue;
                if (first==0) {
                    return Ok(this.__get_root());
                }
                this.__set_cursor(prev);
                break;
                if this.__get_cursor() >= this.__get_patternLength() {
                    break;
                }
                prev = this.__get_cursor();
                this.append(ch, first)?;
                first = first.wrapping_add(1i32);
                let _t10: bool = Pattern::isSupplementary(ch)?;
                if _t10 {
                    hasSupplementary = 1i32;
                }
                let _t11 = this.next()?;
                ch = _t11;
            }
            if first == 1i32 {
                let _t1 = this.single(this.__get_buffer().borrow()[0i32 as usize])?;
                let _t2 = this.newCharProperty(Clone::clone(&_t1))?;
                return Ok(<_ as Into<Pattern_Node>>::into(_t2));
            }
            let _t1 = this.newSlice(Clone::clone(&this.__get_buffer()), first, (hasSupplementary != 0i32))?;
            Ok(_t1)
        }

        #[java_method(name = "append", descriptor = "(II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn append(&self, mut ch: i32, mut index: i32) -> Result<()> {
            let this = self;
            let mut len = (this.__get_buffer().borrow().len() as i32);
            if ((index).wrapping_sub(len)>=0) {
                let _t0: i32 = ArraysSupport::newLength(len, ((1i32).wrapping_add(index)).wrapping_sub(len), len)?;
                len = _t0;
                let _t1: Rc<RefCell<Vec<i32>>> = Arrays::copyOf_arr_i_i(Clone::clone(&this.__get_buffer()), len)?;
                this.__set_buffer(Clone::clone(&_t1));
            }
            this.__get_buffer().borrow_mut()[index as usize] = ch;
            Ok(())
        }

        #[java_method(name = "ref", descriptor = "(I)Ljava/util/regex/Pattern$Node;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ref_(&self, mut refNum: i32) -> Result<Pattern_Node> {
            let this = self;
            let mut done: i32 = 0i32;
            loop {
                if (done!=0) { break; }
                let _t0 = this.peek()?;
                let mut ch: i32 = _t0;
                match ch {
                    48 => {
                        let mut newRefNum = ((refNum).wrapping_mul(10i32)).wrapping_add((ch).wrapping_sub(48i32));
                        if (this.__get_capturingGroupCount()).wrapping_sub(1i32) < newRefNum {
                            done = 1i32;
                        } else {
                            refNum = newRefNum;
                            let _t1 = this.read()?;
                        }
                    }
                    49 => {
                        let mut newRefNum = ((refNum).wrapping_mul(10i32)).wrapping_add((ch).wrapping_sub(48i32));
                        if (this.__get_capturingGroupCount()).wrapping_sub(1i32) < newRefNum {
                            done = 1i32;
                        } else {
                            refNum = newRefNum;
                            let _t1 = this.read()?;
                        }
                    }
                    50 => {
                        let mut newRefNum = ((refNum).wrapping_mul(10i32)).wrapping_add((ch).wrapping_sub(48i32));
                        if (this.__get_capturingGroupCount()).wrapping_sub(1i32) < newRefNum {
                            done = 1i32;
                        } else {
                            refNum = newRefNum;
                            let _t1 = this.read()?;
                        }
                    }
                    51 => {
                        let mut newRefNum = ((refNum).wrapping_mul(10i32)).wrapping_add((ch).wrapping_sub(48i32));
                        if (this.__get_capturingGroupCount()).wrapping_sub(1i32) < newRefNum {
                            done = 1i32;
                        } else {
                            refNum = newRefNum;
                            let _t1 = this.read()?;
                        }
                    }
                    52 => {
                        let mut newRefNum = ((refNum).wrapping_mul(10i32)).wrapping_add((ch).wrapping_sub(48i32));
                        if (this.__get_capturingGroupCount()).wrapping_sub(1i32) < newRefNum {
                            done = 1i32;
                        } else {
                            refNum = newRefNum;
                            let _t1 = this.read()?;
                        }
                    }
                    53 => {
                        let mut newRefNum = ((refNum).wrapping_mul(10i32)).wrapping_add((ch).wrapping_sub(48i32));
                        if (this.__get_capturingGroupCount()).wrapping_sub(1i32) < newRefNum {
                            done = 1i32;
                        } else {
                            refNum = newRefNum;
                            let _t1 = this.read()?;
                        }
                    }
                    54 => {
                        let mut newRefNum = ((refNum).wrapping_mul(10i32)).wrapping_add((ch).wrapping_sub(48i32));
                        if (this.__get_capturingGroupCount()).wrapping_sub(1i32) < newRefNum {
                            done = 1i32;
                        } else {
                            refNum = newRefNum;
                            let _t1 = this.read()?;
                        }
                    }
                    55 => {
                        let mut newRefNum = ((refNum).wrapping_mul(10i32)).wrapping_add((ch).wrapping_sub(48i32));
                        if (this.__get_capturingGroupCount()).wrapping_sub(1i32) < newRefNum {
                            done = 1i32;
                        } else {
                            refNum = newRefNum;
                            let _t1 = this.read()?;
                        }
                    }
                    56 => {
                        let mut newRefNum = ((refNum).wrapping_mul(10i32)).wrapping_add((ch).wrapping_sub(48i32));
                        if (this.__get_capturingGroupCount()).wrapping_sub(1i32) < newRefNum {
                            done = 1i32;
                        } else {
                            refNum = newRefNum;
                            let _t1 = this.read()?;
                        }
                    }
                    57 => {
                        let mut newRefNum = ((refNum).wrapping_mul(10i32)).wrapping_add((ch).wrapping_sub(48i32));
                        if (this.__get_capturingGroupCount()).wrapping_sub(1i32) < newRefNum {
                            done = 1i32;
                        } else {
                            refNum = newRefNum;
                            let _t1 = this.read()?;
                        }
                    }
                    _ => {
                        done = 1i32;
                    }
                }
            }
            this.__set_hasGroupRef((1i32 != 0i32));
            let _t0 = this.has(2i32)?;
            if _t0 {
                let _t1 = this.has(64i32)?;
                return Ok(<_ as Into<Pattern_Node>>::into(Pattern_CIBackRef::new(refNum, _t1)?));
            }
            Ok(<_ as Into<Pattern_Node>>::into(Pattern_BackRef::new(refNum)?))
        }

        #[java_method(name = "escape", descriptor = "(ZZZ)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn escape(&self, mut inclass: bool, mut create: bool, mut isrange: bool) -> Result<i32> {
            let this = self;
            let _t0 = this.skip()?;
            let mut ch: i32 = _t0;
            match ch {
                48 => {
                    let _t1 = this.o()?;
                    return Ok(_t1);
                }
                49 => {
                    if inclass {
                    } else {
                        if create {
                            let _t1 = this.ref_((ch).wrapping_sub(48i32))?;
                            this.__set_root(Clone::clone(&_t1));
                        }
                        return Ok(-1i32);
                        if inclass {
                        } else {
                            if create {
                                this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Begin::new()?)));
                            }
                            return Ok(-1i32);
                            if inclass {
                            } else {
                                if create {
                                    let _t1 = this.has(256i32)?;
                                    this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Bound::new(4i32, _t1)?)));
                                }
                                return Ok(-1i32);
                            }
                        }
                    }
                }
                50 => {
                    if inclass {
                    } else {
                        if create {
                            let _t1 = this.ref_((ch).wrapping_sub(48i32))?;
                            this.__set_root(Clone::clone(&_t1));
                        }
                        return Ok(-1i32);
                        if inclass {
                        } else {
                            if create {
                                this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Begin::new()?)));
                            }
                            return Ok(-1i32);
                            if inclass {
                            } else {
                                if create {
                                    let _t1 = this.has(256i32)?;
                                    this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Bound::new(4i32, _t1)?)));
                                }
                                return Ok(-1i32);
                            }
                        }
                    }
                }
                51 => {
                    if inclass {
                    } else {
                        if create {
                            let _t1 = this.ref_((ch).wrapping_sub(48i32))?;
                            this.__set_root(Clone::clone(&_t1));
                        }
                        return Ok(-1i32);
                        if inclass {
                        } else {
                            if create {
                                this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Begin::new()?)));
                            }
                            return Ok(-1i32);
                            if inclass {
                            } else {
                                if create {
                                    let _t1 = this.has(256i32)?;
                                    this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Bound::new(4i32, _t1)?)));
                                }
                                return Ok(-1i32);
                            }
                        }
                    }
                }
                52 => {
                    if inclass {
                    } else {
                        if create {
                            let _t1 = this.ref_((ch).wrapping_sub(48i32))?;
                            this.__set_root(Clone::clone(&_t1));
                        }
                        return Ok(-1i32);
                        if inclass {
                        } else {
                            if create {
                                this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Begin::new()?)));
                            }
                            return Ok(-1i32);
                            if inclass {
                            } else {
                                if create {
                                    let _t1 = this.has(256i32)?;
                                    this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Bound::new(4i32, _t1)?)));
                                }
                                return Ok(-1i32);
                            }
                        }
                    }
                }
                53 => {
                    if inclass {
                    } else {
                        if create {
                            let _t1 = this.ref_((ch).wrapping_sub(48i32))?;
                            this.__set_root(Clone::clone(&_t1));
                        }
                        return Ok(-1i32);
                        if inclass {
                        } else {
                            if create {
                                this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Begin::new()?)));
                            }
                            return Ok(-1i32);
                            if inclass {
                            } else {
                                if create {
                                    let _t1 = this.has(256i32)?;
                                    this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Bound::new(4i32, _t1)?)));
                                }
                                return Ok(-1i32);
                            }
                        }
                    }
                }
                54 => {
                    if inclass {
                    } else {
                        if create {
                            let _t1 = this.ref_((ch).wrapping_sub(48i32))?;
                            this.__set_root(Clone::clone(&_t1));
                        }
                        return Ok(-1i32);
                        if inclass {
                        } else {
                            if create {
                                this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Begin::new()?)));
                            }
                            return Ok(-1i32);
                            if inclass {
                            } else {
                                if create {
                                    let _t1 = this.has(256i32)?;
                                    this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Bound::new(4i32, _t1)?)));
                                }
                                return Ok(-1i32);
                            }
                        }
                    }
                }
                55 => {
                    if inclass {
                    } else {
                        if create {
                            let _t1 = this.ref_((ch).wrapping_sub(48i32))?;
                            this.__set_root(Clone::clone(&_t1));
                        }
                        return Ok(-1i32);
                        if inclass {
                        } else {
                            if create {
                                this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Begin::new()?)));
                            }
                            return Ok(-1i32);
                            if inclass {
                            } else {
                                if create {
                                    let _t1 = this.has(256i32)?;
                                    this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Bound::new(4i32, _t1)?)));
                                }
                                return Ok(-1i32);
                            }
                        }
                    }
                }
                56 => {
                    if inclass {
                    } else {
                        if create {
                            let _t1 = this.ref_((ch).wrapping_sub(48i32))?;
                            this.__set_root(Clone::clone(&_t1));
                        }
                        return Ok(-1i32);
                        if inclass {
                        } else {
                            if create {
                                this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Begin::new()?)));
                            }
                            return Ok(-1i32);
                            if inclass {
                            } else {
                                if create {
                                    let _t1 = this.has(256i32)?;
                                    this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Bound::new(4i32, _t1)?)));
                                }
                                return Ok(-1i32);
                            }
                        }
                    }
                }
                57 => {
                    if inclass {
                    } else {
                        if create {
                            let _t1 = this.ref_((ch).wrapping_sub(48i32))?;
                            this.__set_root(Clone::clone(&_t1));
                        }
                        return Ok(-1i32);
                        if inclass {
                        } else {
                            if create {
                                this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Begin::new()?)));
                            }
                            return Ok(-1i32);
                            if inclass {
                            } else {
                                if create {
                                    let _t1 = this.has(256i32)?;
                                    this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Bound::new(4i32, _t1)?)));
                                }
                                return Ok(-1i32);
                            }
                        }
                    }
                }
                58 => {
                    return Ok(ch);
                }
                59 => {
                    return Ok(ch);
                }
                60 => {
                    return Ok(ch);
                }
                61 => {
                    return Ok(ch);
                }
                62 => {
                    return Ok(ch);
                }
                63 => {
                    return Ok(ch);
                }
                64 => {
                    return Ok(ch);
                }
                65 => {
                    if inclass {
                    } else {
                        if create {
                            this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Begin::new()?)));
                        }
                        return Ok(-1i32);
                        if inclass {
                        } else {
                            if create {
                                let _t1 = this.has(256i32)?;
                                this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Bound::new(4i32, _t1)?)));
                            }
                            return Ok(-1i32);
                        }
                    }
                }
                66 => {
                    if inclass {
                    } else {
                        if create {
                            let _t1 = this.has(256i32)?;
                            this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Bound::new(4i32, _t1)?)));
                        }
                        return Ok(-1i32);
                    }
                }
                67 => {
                }
                68 => {
                    let _t1 = this.has(256i32)?;
                    let mut _merged3: Object;
                    if _t1 {
                        let _t2: Object = CharPredicates::DIGIT()?;
                        _merged3 = _t2;
                    } else {
                        let _t2: Object = CharPredicates::ASCII_DIGIT()?;
                        _merged3 = _t2;
                    }
                    this.__set_predicate(Clone::clone(&_merged3));
                    let _vdispatch4: Object = if let Some(_d) = this.__get_predicate().0.as_any().downcast_ref::<Pattern_BitClass>() { _d.negate()? } else if let Some(_d) = this.__get_predicate().0.as_any().downcast_ref::<Object>() { _d.negate()? } else if let Some(_d) = this.__get_predicate().0.as_any().downcast_ref::<Object>() { _d.negate()? } else if let Some(__f) = this.__get_predicate().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                    this.__set_predicate(Clone::clone(&_vdispatch4));
                    if !(inclass) {
                        let _t5 = this.newCharProperty(Clone::clone(&this.__get_predicate()))?;
                        this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(_t5)));
                    }
                    return Ok(-1i32);
                }
                69 => {
                }
                70 => {
                }
                71 => {
                    if inclass {
                    } else {
                        if create {
                            this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_LastMatch::new()?)));
                        }
                        return Ok(-1i32);
                        let _t1: Object = Pattern::HorizWS()?;
                        let _vdispatch2: Object = if let Some(_d) = _t1.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.negate()? } else if let Some(_d) = _t1.0.as_any().downcast_ref::<Object>() { _d.negate()? } else if let Some(__f) = _t1.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                        this.__set_predicate(Clone::clone(&_vdispatch2));
                        if !(inclass) {
                            let _t3 = this.newCharProperty(Clone::clone(&this.__get_predicate()))?;
                            this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(_t3)));
                        }
                        return Ok(-1i32);
                    }
                }
                72 => {
                    let _t1: Object = Pattern::HorizWS()?;
                    let _vdispatch2: Object = if let Some(_d) = _t1.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.negate()? } else if let Some(_d) = _t1.0.as_any().downcast_ref::<Object>() { _d.negate()? } else if let Some(__f) = _t1.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                    this.__set_predicate(Clone::clone(&_vdispatch2));
                    if !(inclass) {
                        let _t3 = this.newCharProperty(Clone::clone(&this.__get_predicate()))?;
                        this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(_t3)));
                    }
                    return Ok(-1i32);
                }
                73 => {
                }
                74 => {
                }
                75 => {
                }
                76 => {
                }
                77 => {
                }
                78 => {
                    let _t1 = this.N()?;
                    return Ok(_t1);
                }
                79 => {
                }
                80 => {
                }
                81 => {
                }
                82 => {
                    if inclass {
                    } else {
                        if create {
                            this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_LineEnding::new()?)));
                        }
                        return Ok(-1i32);
                        let _t1 = this.has(256i32)?;
                        let mut _merged3: Object;
                        if _t1 {
                            let _t2: Object = CharPredicates::WHITE_SPACE()?;
                            _merged3 = _t2;
                        } else {
                            let _t2: Object = CharPredicates::ASCII_SPACE()?;
                            _merged3 = _t2;
                        }
                        this.__set_predicate(Clone::clone(&_merged3));
                        let _vdispatch4: Object = if let Some(_d) = this.__get_predicate().0.as_any().downcast_ref::<Pattern_BitClass>() { _d.negate()? } else if let Some(_d) = this.__get_predicate().0.as_any().downcast_ref::<Object>() { _d.negate()? } else if let Some(_d) = this.__get_predicate().0.as_any().downcast_ref::<Object>() { _d.negate()? } else if let Some(__f) = this.__get_predicate().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                        this.__set_predicate(Clone::clone(&_vdispatch4));
                        if !(inclass) {
                            let _t5 = this.newCharProperty(Clone::clone(&this.__get_predicate()))?;
                            this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(_t5)));
                        }
                        return Ok(-1i32);
                    }
                }
                83 => {
                    let _t1 = this.has(256i32)?;
                    let mut _merged3: Object;
                    if _t1 {
                        let _t2: Object = CharPredicates::WHITE_SPACE()?;
                        _merged3 = _t2;
                    } else {
                        let _t2: Object = CharPredicates::ASCII_SPACE()?;
                        _merged3 = _t2;
                    }
                    this.__set_predicate(Clone::clone(&_merged3));
                    let _vdispatch4: Object = if let Some(_d) = this.__get_predicate().0.as_any().downcast_ref::<Pattern_BitClass>() { _d.negate()? } else if let Some(_d) = this.__get_predicate().0.as_any().downcast_ref::<Object>() { _d.negate()? } else if let Some(_d) = this.__get_predicate().0.as_any().downcast_ref::<Object>() { _d.negate()? } else if let Some(__f) = this.__get_predicate().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                    this.__set_predicate(Clone::clone(&_vdispatch4));
                    if !(inclass) {
                        let _t5 = this.newCharProperty(Clone::clone(&this.__get_predicate()))?;
                        this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(_t5)));
                    }
                    return Ok(-1i32);
                }
                84 => {
                }
                85 => {
                }
                86 => {
                    let _t1: Object = Pattern::VertWS()?;
                    let _vdispatch2: Object = if let Some(_d) = _t1.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.negate()? } else if let Some(_d) = _t1.0.as_any().downcast_ref::<Object>() { _d.negate()? } else if let Some(__f) = _t1.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                    this.__set_predicate(Clone::clone(&_vdispatch2));
                    if !(inclass) {
                        let _t3 = this.newCharProperty(Clone::clone(&this.__get_predicate()))?;
                        this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(_t3)));
                    }
                    return Ok(-1i32);
                }
                87 => {
                    let _t1 = this.has(256i32)?;
                    let mut _merged3: Object;
                    if _t1 {
                        let _t2: Object = CharPredicates::WORD()?;
                        _merged3 = _t2;
                    } else {
                        let _t2: Object = CharPredicates::ASCII_WORD()?;
                        _merged3 = _t2;
                    }
                    this.__set_predicate(Clone::clone(&_merged3));
                    let _vdispatch4: Object = if let Some(_d) = this.__get_predicate().0.as_any().downcast_ref::<Pattern_BitClass>() { _d.negate()? } else if let Some(_d) = this.__get_predicate().0.as_any().downcast_ref::<Object>() { _d.negate()? } else if let Some(_d) = this.__get_predicate().0.as_any().downcast_ref::<Object>() { _d.negate()? } else if let Some(__f) = this.__get_predicate().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                    this.__set_predicate(Clone::clone(&_vdispatch4));
                    if !(inclass) {
                        let _t5 = this.newCharProperty(Clone::clone(&this.__get_predicate()))?;
                        this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(_t5)));
                    }
                    return Ok(-1i32);
                }
                88 => {
                    if inclass {
                    } else {
                        if create {
                            this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_XGrapheme::new()?)));
                        }
                        return Ok(-1i32);
                    }
                }
                89 => {
                }
                90 => {
                    if inclass {
                    } else {
                        let _t1 = this.has(1i32)?;
                        if _t1 {
                            this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_UnixDollar::new((0i32 != 0i32))?)));
                        } else {
                            this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Dollar::new((0i32 != 0i32))?)));
                        }
                        return Ok(-1i32);
                        return Ok(7i32);
                        if inclass {
                        } else {
                            let _t2 = this.peek()?;
                            let _t3 = this.skip()?;
                            let _t4 = this.read()?;
                            this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_GraphemeBound::new()?)));
                            return Ok(-1i32);
                            this.unread()?;
                            this.unread()?;
                            let _t5 = this.has(256i32)?;
                            this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Bound::new(3i32, _t5)?)));
                            return Ok(-1i32);
                            let _t6 = this.c()?;
                            return Ok(_t6);
                            let _t7 = this.has(256i32)?;
                            let mut _merged9: Object;
                            if _t7 {
                                let _t8: Object = CharPredicates::DIGIT()?;
                                _merged9 = _t8;
                            } else {
                                let _t8: Object = CharPredicates::ASCII_DIGIT()?;
                                _merged9 = _t8;
                            }
                            this.__set_predicate(Clone::clone(&_merged9));
                            if !(inclass) {
                                let _t10 = this.newCharProperty(Clone::clone(&this.__get_predicate()))?;
                                this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(_t10)));
                            }
                            return Ok(-1i32);
                            return Ok(27i32);
                            return Ok(12i32);
                        }
                    }
                }
                91 => {
                    return Ok(ch);
                }
                92 => {
                    return Ok(ch);
                }
                93 => {
                    return Ok(ch);
                }
                94 => {
                    return Ok(ch);
                }
                95 => {
                    return Ok(ch);
                }
                96 => {
                    return Ok(ch);
                }
                97 => {
                    return Ok(7i32);
                }
                98 => {
                    if inclass {
                    } else {
                        let _t1 = this.peek()?;
                        let _t2 = this.skip()?;
                        let _t3 = this.read()?;
                        this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_GraphemeBound::new()?)));
                        return Ok(-1i32);
                        this.unread()?;
                        this.unread()?;
                        let _t4 = this.has(256i32)?;
                        this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_Bound::new(3i32, _t4)?)));
                        return Ok(-1i32);
                        let _t5 = this.c()?;
                        return Ok(_t5);
                        let _t6 = this.has(256i32)?;
                        let mut _merged8: Object;
                        if _t6 {
                            let _t7: Object = CharPredicates::DIGIT()?;
                            _merged8 = _t7;
                        } else {
                            let _t7: Object = CharPredicates::ASCII_DIGIT()?;
                            _merged8 = _t7;
                        }
                        this.__set_predicate(Clone::clone(&_merged8));
                        if !(inclass) {
                            let _t9 = this.newCharProperty(Clone::clone(&this.__get_predicate()))?;
                            this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(_t9)));
                        }
                        return Ok(-1i32);
                        return Ok(27i32);
                        return Ok(12i32);
                    }
                }
                99 => {
                    let _t1 = this.c()?;
                    return Ok(_t1);
                }
                100 => {
                    let _t1 = this.has(256i32)?;
                    let mut _merged3: Object;
                    if _t1 {
                        let _t2: Object = CharPredicates::DIGIT()?;
                        _merged3 = _t2;
                    } else {
                        let _t2: Object = CharPredicates::ASCII_DIGIT()?;
                        _merged3 = _t2;
                    }
                    this.__set_predicate(Clone::clone(&_merged3));
                    if !(inclass) {
                        let _t4 = this.newCharProperty(Clone::clone(&this.__get_predicate()))?;
                        this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(_t4)));
                    }
                    return Ok(-1i32);
                }
                101 => {
                    return Ok(27i32);
                }
                102 => {
                    return Ok(12i32);
                }
                103 => {
                }
                104 => {
                    let _t1: Object = Pattern::HorizWS()?;
                    this.__set_predicate(Clone::clone(&_t1));
                    if !(inclass) {
                        let _t2 = this.newCharProperty(Clone::clone(&this.__get_predicate()))?;
                        this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(_t2)));
                    }
                    return Ok(-1i32);
                }
                105 => {
                }
                106 => {
                }
                107 => {
                    if inclass {
                    } else {
                        let _t1 = this.read()?;
                        if _t1 != 60i32 {
                            let _t2 = this.error(Clone::clone(&String::from("\\k is not followed by '<' for named capturing group")))?;
                            return Err(JvmError::Custom("athrow".to_owned()));
                        }
                        let _t2 = this.read()?;
                        let _t3 = this.groupname(_t2)?;
                        let mut name: String = _t3;
                        let _t4 = this.namedGroupsMap()?;
                        let _vdispatch5: Object = if let Some(_d) = _t4.0.as_any().downcast_ref::<ImmutableCollections_MapN<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<ImmutableCollections_Map1<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<Properties>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<ConcurrentHashMap<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<ImmutableCollections_AbstractImmutableMap<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<TreeMap<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<LinkedHashMap<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<Hashtable<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<AbstractMap<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<HashMap<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(name.clone()))? } else if let Some(__f) = _t4.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(name.clone()))? } else { Default::default() };
                        let mut number = (_vdispatch5).downcast::<i32>();
                        if _is_jnull(&number) {
                            let _t6 = StringBuilder::new()?.append_str(Clone::clone(&String::from("named capturing group <")))?;
                            let _t7 = _t6.append_str(Clone::clone(&name))?;
                            let _t8 = _t7.append_str(Clone::clone(&String::from("> does not exist")))?;
                            let _t9 = _t8.toString()?;
                            let _t10 = this.error(Clone::clone(&_t9))?;
                            return Err(JvmError::Custom("athrow".to_owned()));
                        }
                        this.__set_hasGroupRef((1i32 != 0i32));
                        let _t6 = this.has(2i32)?;
                        if _t6 {
                            let _t7 = this.has(64i32)?;
                            this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_CIBackRef::new(number, _t7)?)));
                        } else {
                            this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_BackRef::new(number)?)));
                        }
                        return Ok(-1i32);
                    }
                }
                108 => {
                }
                109 => {
                }
                110 => {
                    return Ok(10i32);
                }
                111 => {
                }
                112 => {
                }
                113 => {
                }
                114 => {
                    return Ok(13i32);
                }
                115 => {
                    let _t1 = this.has(256i32)?;
                    let mut _merged3: Object;
                    if _t1 {
                        let _t2: Object = CharPredicates::WHITE_SPACE()?;
                        _merged3 = _t2;
                    } else {
                        let _t2: Object = CharPredicates::ASCII_SPACE()?;
                        _merged3 = _t2;
                    }
                    this.__set_predicate(Clone::clone(&_merged3));
                    if !(inclass) {
                        let _t4 = this.newCharProperty(Clone::clone(&this.__get_predicate()))?;
                        this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(_t4)));
                    }
                    return Ok(-1i32);
                }
                116 => {
                    return Ok(9i32);
                }
                117 => {
                    let _t1 = this.u()?;
                    return Ok(_t1);
                }
                118 => {
                    if isrange {
                        return Ok(11i32);
                    }
                }
                119 => {
                    let _t1 = this.has(256i32)?;
                    let mut _merged3: Object;
                    if _t1 {
                        let _t2: Object = CharPredicates::WORD()?;
                        _merged3 = _t2;
                    } else {
                        let _t2: Object = CharPredicates::ASCII_WORD()?;
                        _merged3 = _t2;
                    }
                    this.__set_predicate(Clone::clone(&_merged3));
                    if !(inclass) {
                        let _t4 = this.newCharProperty(Clone::clone(&this.__get_predicate()))?;
                        this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(_t4)));
                    }
                    return Ok(-1i32);
                }
                120 => {
                    let _t1 = this.x()?;
                    return Ok(_t1);
                }
                121 => {
                }
                122 => {
                    if inclass {
                    } else {
                        if create {
                            this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_End::new()?)));
                        }
                        return Ok(-1i32);
                        return Ok(ch);
                    }
                }
                _ => {
                    return Ok(ch);
                }
            }
            let _t1 = this.error(Clone::clone(&String::from("Illegal/unsupported escape sequence")))?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }

        #[java_method(name = "clazz", descriptor = "(Z)Ljava/util/regex/Pattern$CharPredicate;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clazz(&self, mut consume: bool) -> Result<Object> {
            let this = self;
            let mut prev: Object = Object::default();
            let mut curr: Object = Object::default();
            let mut bits = Pattern_BitClass::new()?;
            let mut isNeg: i32 = 0i32;
            let mut hasBits: i32 = 0i32;
            let _t0 = this.next()?;
            let mut ch: i32 = _t0;
            if this.__get_temp().borrow()[(this.__get_cursor()).wrapping_sub(1i32) as usize] == 91i32 {
                let _t1 = this.next()?;
                ch = _t1;
                isNeg = 1i32;
            }
            let mut prev: Pattern_BitClass = Default::default();
            loop {
        let mut right: Object = Default::default();
                match ch {
                    0 => {
                        let _t1 = this.error(Clone::clone(&String::from("Unclosed character class")))?;
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                    38 => {
                        let _t1 = this.next()?;
                        ch = _t1;
                        let _t2 = this.next()?;
                        ch = _t2;
                        right = Object::default();
                        if ch == 91i32 {
                            if _is_jnull(&right) {
                                let _t3 = this.clazz((1i32 != 0i32))?;
                                right = _t3;
                            } else {
                                let _t3 = this.clazz((1i32 != 0i32))?;
                                let _vdispatch4: Object = if let Some(_d) = right.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.union_patter(Clone::clone(&_t3))? } else if let Some(_d) = right.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&_t3))? } else if let Some(_d) = right.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&_t3))? } else if let Some(__f) = right.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&_t3))? } else { Default::default() };
                                right = _vdispatch4;
                            }
                        } else {
                            this.unread()?;
                            if _is_jnull(&right) {
                                let _t3 = this.clazz((0i32 != 0i32))?;
                                right = _t3;
                            } else {
                                let _t3 = this.clazz((0i32 != 0i32))?;
                                let _vdispatch4: Object = if let Some(_d) = right.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.union_patter(Clone::clone(&_t3))? } else if let Some(_d) = right.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&_t3))? } else if let Some(_d) = right.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&_t3))? } else if let Some(__f) = right.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&_t3))? } else { Default::default() };
                                right = _vdispatch4;
                            }
                        }
                    }
                    91 => {
                        let _t1 = this.clazz((1i32 != 0i32))?;
                        curr = _t1;
                        if _is_jnull(&prev) {
                            prev = curr;
                        } else {
                            let _vdispatch2: Object = if let Some(_d) = prev.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.union_patter(Clone::clone(&curr))? } else if let Some(_d) = prev.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&curr))? } else if let Some(_d) = prev.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&curr))? } else if let Some(__f) = prev.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&curr))? } else { Default::default() };
                            prev = _vdispatch2;
                        }
                        let _t2 = this.peek()?;
                        ch = _t2;
                        continue;
                        let _t3 = this.next()?;
                        ch = _t3;
                        let _t4 = this.next()?;
                        ch = _t4;
                        right = Object::default();
                        if ch == 91i32 {
                            if _is_jnull(&right) {
                                let _t5 = this.clazz((1i32 != 0i32))?;
                                right = _t5;
                            } else {
                                let _t5 = this.clazz((1i32 != 0i32))?;
                                let _vdispatch6: Object = if let Some(_d) = right.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.union_patter(Clone::clone(&_t5))? } else if let Some(_d) = right.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&_t5))? } else if let Some(_d) = right.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&_t5))? } else if let Some(__f) = right.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&_t5))? } else { Default::default() };
                                right = _vdispatch6;
                            }
                        } else {
                            this.unread()?;
                            if _is_jnull(&right) {
                                let _t5 = this.clazz((0i32 != 0i32))?;
                                right = _t5;
                            } else {
                                let _t5 = this.clazz((0i32 != 0i32))?;
                                let _vdispatch6: Object = if let Some(_d) = right.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.union_patter(Clone::clone(&_t5))? } else if let Some(_d) = right.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&_t5))? } else if let Some(_d) = right.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&_t5))? } else if let Some(__f) = right.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&_t5))? } else { Default::default() };
                                right = _vdispatch6;
                            }
                        }
                    }
                    93 => {
                        if consume {
                            let _t1 = this.next()?;
                        }
                        if _is_jnull(&prev) {
                            prev = bits;
                        } else {
                            if (hasBits!=0) {
                                let _vdispatch1: Object = if let Some(_d) = prev.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.union_patter(Object::from_any(bits.clone()))? } else if let Some(_d) = prev.0.as_any().downcast_ref::<Object>() { _d.union_(Object::from_any(bits.clone()))? } else if let Some(_d) = prev.0.as_any().downcast_ref::<Object>() { _d.union_(Object::from_any(bits.clone()))? } else if let Some(__f) = prev.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(bits.clone()))? } else { Default::default() };
                                prev = _vdispatch1;
                            }
                        }
                        if (isNeg!=0) {
                            let _t1 = prev.negate()?;
                            return Ok(_t1);
                        }
                    }
                    _ => {
                        let _t1 = this.range(Clone::clone(&bits))?;
                        curr = _t1;
                        if _is_jnull(&curr) {
                            hasBits = 1i32;
                        } else {
                            if _is_jnull(&prev) {
                                prev = curr;
                            } else {
                                if prev != curr {
                                    let _vdispatch2: Object = if let Some(_d) = prev.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.union_patter(Clone::clone(&curr))? } else if let Some(_d) = prev.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&curr))? } else if let Some(_d) = prev.0.as_any().downcast_ref::<Object>() { _d.union_(Clone::clone(&curr))? } else if let Some(__f) = prev.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&curr))? } else { Default::default() };
                                    prev = _vdispatch2;
                                }
                            }
                        }
                        let _t2 = this.peek()?;
                        ch = _t2;
                        continue;
                    }
                }
                let _t1 = this.peek()?;
                ch = _t1;
                continue;
                if _is_jnull(&prev) {
                    let mut curr: Pattern_BitClass = bits;
                    let mut prev: Pattern_BitClass = curr;
                } else {
                    let _vdispatch2: Object = if let Some(_d) = prev.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.union_patter(Object::from_any(bits.clone()))? } else if let Some(_d) = prev.0.as_any().downcast_ref::<Object>() { _d.union_(Object::from_any(bits.clone()))? } else if let Some(_d) = prev.0.as_any().downcast_ref::<Object>() { _d.union_(Object::from_any(bits.clone()))? } else if let Some(__f) = prev.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(bits.clone()))? } else { Default::default() };
                    prev = _vdispatch2;
                }
                hasBits = 0i32;
                if !_is_jnull(&right) {
                    let mut curr: i32 = right;
                }
                if _is_jnull(&prev) {
                    if _is_jnull(&right) {
                        let _t2 = this.error(Clone::clone(&String::from("Bad class syntax")))?;
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                    let mut prev: i32 = right;
                } else {
                    if _is_jnull(&curr) {
                        let _t2 = this.error(Clone::clone(&String::from("Bad intersection syntax")))?;
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                    let _t2 = prev.and(curr.into())?;
                    let mut prev: Object = _t2;
                }
                continue;
                this.unread()?;
                let _t2 = this.range(Clone::clone(&bits))?;
                let mut curr: Object = _t2;
                if _is_jnull(&curr) {
                    hasBits = 1i32;
                } else {
                    if _is_jnull(&prev) {
                        let mut prev: Object = curr;
                    } else {
                        if prev != curr {
                            let _t3 = prev.union_(Clone::clone(&curr))?;
                            let mut prev: Object = _t3;
                        }
                    }
                }
                let _t3 = this.peek()?;
                ch = _t3;
            }
        }

        #[java_method(name = "bitsOrSingle", descriptor = "(Ljava/util/regex/Pattern$BitClass;I)Ljava/util/regex/Pattern$CharPredicate;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn bitsOrSingle(&self, mut bits: Pattern_BitClass, mut ch: i32) -> Result<Object> {
            let this = self;
            let _t0 = this.has(2i32)?;
            let _t1 = this.has(64i32)?;
            if ch != 229i32 {
                let _t2 = bits.add(ch, this.__get_flags0())?;
                return Ok(Object::default());
            }
            let _t2 = this.single(ch)?;
            Ok(_t2)
        }

        #[java_method(name = "single", descriptor = "(I)Ljava/util/regex/Pattern$CharPredicate;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn single(&self, mut ch: i32) -> Result<Object> {
            let this = self;
            let _t0 = this.has(2i32)?;
            let _t1 = this.has(64i32)?;
            let _t2: i32 = Character::toUpperCase_i(ch)?;
            let mut upper: i32 = _t2;
            let _t3: i32 = Character::toLowerCase_i(upper)?;
            let mut lower: i32 = _t3;
            let _t4: Object = Pattern::SingleU(lower)?;
            return Ok(_t4);
            let _t5: bool = ASCII::isAscii(ch)?;
            let _t6: i32 = ASCII::toLower(ch)?;
            lower = _t6;
            let _t7: i32 = ASCII::toUpper(ch)?;
            upper = _t7;
            if lower != upper {
                let _t8: Object = Pattern::SingleI(lower, upper)?;
                return Ok(_t8);
            }
            let _t8: bool = Pattern::isSupplementary(ch)?;
            if _t8 {
                let _t9: Object = Pattern::SingleS(ch)?;
                return Ok(_t9);
            }
            let _t9: Object = Pattern::Single(ch)?;
            Ok(_t9)
        }

        #[java_method(name = "range", descriptor = "(Ljava/util/regex/Pattern$BitClass;)Ljava/util/regex/Pattern$CharPredicate;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn range(&self, mut bits: Pattern_BitClass) -> Result<Object> {
            let this = self;
            let _t0 = this.peek()?;
            let mut ch: i32 = _t0;
            if ch == 92i32 {
                let _t1 = this.nextEscaped()?;
                ch = _t1;
                let mut comp = (ch == 80i32) as i32;
                let mut oneLetter: i32 = 1i32;
                let _t2 = this.next()?;
                ch = _t2;
                if ch != 123i32 {
                    this.unread()?;
                } else {
                    oneLetter = 0i32;
                }
                let _t3 = this.family((oneLetter != 0i32), (comp != 0i32))?;
                return Ok(_t3);
                comp = (this.__get_temp().borrow()[(this.__get_cursor()).wrapping_add(1i32) as usize] == 45i32) as i32;
                this.unread()?;
                let _t4 = this.escape((1i32 != 0i32), (1i32 != 0i32), (comp != 0i32))?;
                ch = _t4;
                if ch == -1i32 {
                    return Ok(this.__get_predicate());
                }
            } else {
                let _t1 = this.next()?;
            }
            let _t1 = this.peek()?;
            let mut comp = this.__get_temp().borrow()[(this.__get_cursor()).wrapping_add(1i32) as usize];
            if comp == 91i32 {
                let _t2 = this.bitsOrSingle(Clone::clone(&bits), ch)?;
                return Ok(_t2);
            }
            let _t2 = this.next()?;
            let _t3 = this.peek()?;
            let mut oneLetter: i32 = _t3;
            if oneLetter == 92i32 {
                let _t4 = this.escape((1i32 != 0i32), (0i32 != 0i32), (1i32 != 0i32))?;
                oneLetter = _t4;
            } else {
                let _t4 = this.next()?;
            }
            if oneLetter < ch {
                let _t4 = this.error(Clone::clone(&String::from("Illegal character range")))?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t4 = this.has(2i32)?;
            let _t5 = this.has(64i32)?;
            if _t5 {
                let _t6: Object = Pattern::CIRangeU(ch, oneLetter)?;
                return Ok(_t6);
            }
            let _t6: Object = Pattern::CIRange(ch, oneLetter)?;
            return Ok(_t6);
            let _t7: Object = Pattern::Range(ch, oneLetter)?;
            return Ok(_t7);
            let _t8 = this.bitsOrSingle(Clone::clone(&bits), ch)?;
            return Ok(_t8);
            let _t9 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Unexpected character '")))?;
            let _t10 = _t9.append_c(((((ch) as u16 as i32)) as u16))?;
            let _t11 = _t10.append_str(Clone::clone(&String::from("'")))?;
            let _t12 = _t11.toString()?;
            let _t13 = this.error(Clone::clone(&_t12))?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }

        #[java_method(name = "family", descriptor = "(ZZ)Ljava/util/regex/Pattern$CharPredicate;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn family(&self, mut singleLetter: bool, mut isComplement: bool) -> Result<Object> {
            let this = self;
            let _t0 = this.next()?;
            let mut p: Object = Object::default();
        let mut name: String = Default::default();
            if singleLetter {
                let mut c = this.__get_temp().borrow()[this.__get_cursor() as usize];
                let _t1: bool = Character::isSupplementaryCodePoint(c)?;
                if !(_t1) {
                    name = String::from_owned(format!("{}", ((c) as u16 as i32)));
                } else {
                    name = String::new_arr_i_i_i(Clone::clone(&this.__get_temp()), this.__get_cursor(), 1i32)?;
                }
                let _t2 = this.read()?;
            } else {
                let mut c = this.__get_cursor();
                this.mark(125i32)?;
                loop {
                    let _t1 = this.read()?;
                    if _t1 == 125i32 { break; }
                }
                this.mark(0i32)?;
                let mut j = this.__get_cursor();
                if j > this.__get_patternLength() {
                    let _t1 = this.error(Clone::clone(&String::from("Unclosed character family")))?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                if (c).wrapping_add(1i32) >= j {
                    let _t1 = this.error(Clone::clone(&String::from("Empty character family")))?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                name = String::new_arr_i_i_i(Clone::clone(&this.__get_temp()), c, ((j).wrapping_sub(c)).wrapping_sub(1i32))?;
            }
            let _t1 = name.indexOf_i(61i32)?;
            let mut c: i32 = _t1;
            if c != -1i32 {
                let _t2 = name.substring_i((c).wrapping_add(1i32))?;
                let mut j: String = _t2;
                let _t3 = name.substring_i_i(0i32, c)?;
                let _t4 = _t3.toLowerCase_locale(Clone::clone(&Locale::ENGLISH()))?;
                name = _t4;
                let mut local_7: String = name;
                let mut local_8: i32 = -1i32;
                let _t5 = local_7.hashCode()?;
                match _t5 {
                    -907685685 => {
                        let _t6 = local_7.equals(Object::from_any(String::from("script").clone()))?;
                        local_8 = 1i32;
                    }
                    3292 => {
                        let _t6 = local_7.equals(Object::from_any(String::from("gc").clone()))?;
                        local_8 = 4i32;
                    }
                    3664 => {
                        let _t6 = local_7.equals(Object::from_any(String::from("sc").clone()))?;
                        local_8 = 0i32;
                    }
                    97633 => {
                        let _t6 = local_7.equals(Object::from_any(String::from("blk").clone()))?;
                        local_8 = 2i32;
                    }
                    93832333 => {
                        let _t6 = local_7.equals(Object::from_any(String::from("block").clone()))?;
                        local_8 = 3i32;
                    }
                    1265003125 => {
                        let _t6 = local_7.equals(Object::from_any(String::from("general_category").clone()))?;
                        if _t6 {
                            local_8 = 5i32;
                        }
                    }
                    _ => {
                    }
                }
                match local_8 {
                    0 => {
                        let _t6: Object = CharPredicates::forUnicodeScript(Clone::clone(&j))?;
                        p = _t6;
                    }
                    1 => {
                        let _t6: Object = CharPredicates::forUnicodeScript(Clone::clone(&j))?;
                        p = _t6;
                    }
                    2 => {
                        let _t6: Object = CharPredicates::forUnicodeBlock(Clone::clone(&j))?;
                        p = _t6;
                    }
                    3 => {
                        let _t6: Object = CharPredicates::forUnicodeBlock(Clone::clone(&j))?;
                        p = _t6;
                    }
                    4 => {
                        let _t6 = this.has(2i32)?;
                        let _t7: Object = CharPredicates::forProperty(Clone::clone(&j), _t6)?;
                        p = _t7;
                    }
                    5 => {
                        let _t6 = this.has(2i32)?;
                        let _t7: Object = CharPredicates::forProperty(Clone::clone(&j), _t6)?;
                        p = _t7;
                    }
                    _ => {
                    }
                }
                if _is_jnull(&p) {
                    let _t6 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Unknown Unicode property {name=<")))?;
                    let _t7 = _t6.append_str(Clone::clone(&name))?;
                    let _t8 = _t7.append_str(Clone::clone(&String::from(">, value=<")))?;
                    let _t9 = _t8.append_str(Clone::clone(&j))?;
                    let _t10 = _t9.append_str(Clone::clone(&String::from(">}")))?;
                    let _t11 = _t10.toString()?;
                    let _t12 = this.error(Clone::clone(&_t11))?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
            } else {
                let _t2 = name.startsWith_str(Clone::clone(&String::from("In")))?;
                if _t2 {
                    let _t3 = name.substring_i(2i32)?;
                    let _t4: Object = CharPredicates::forUnicodeBlock(Clone::clone(&_t3))?;
                    p = _t4;
                } else {
                    let _t3 = name.startsWith_str(Clone::clone(&String::from("Is")))?;
                    if _t3 {
                        let _t4 = name.substring_i(2i32)?;
                        let mut j: String = _t4;
                        let _t5 = this.has(2i32)?;
                        let _t6: Object = CharPredicates::forUnicodeProperty(Clone::clone(&j), _t5)?;
                        p = _t6;
                        if _is_jnull(&p) {
                            let _t7 = this.has(2i32)?;
                            let _t8: Object = CharPredicates::forProperty(Clone::clone(&j), _t7)?;
                            p = _t8;
                        }
                        if _is_jnull(&p) {
                            let _t7: Object = CharPredicates::forUnicodeScript(Clone::clone(&j))?;
                            p = _t7;
                        }
                    } else {
                        let _t4 = this.has(256i32)?;
                        if _t4 {
                            let _t5 = this.has(2i32)?;
                            let _t6: Object = CharPredicates::forPOSIXName(Clone::clone(&name), _t5)?;
                            p = _t6;
                        }
                        if _is_jnull(&p) {
                            let _t5 = this.has(2i32)?;
                            let _t6: Object = CharPredicates::forProperty(Clone::clone(&name), _t5)?;
                            p = _t6;
                        }
                    }
                }
                if _is_jnull(&p) {
                    let _t3 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Unknown character property name {")))?;
                    let _t4 = _t3.append_str(Clone::clone(&name))?;
                    let _t5 = _t4.append_str(Clone::clone(&String::from("}")))?;
                    let _t6 = _t5.toString()?;
                    let _t7 = this.error(Clone::clone(&_t6))?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
            }
            if isComplement {
                this.__set_hasSupplementary((1i32 != 0i32));
                let _vdispatch2: Object = if let Some(_d) = p.0.as_any().downcast_ref::<Pattern_BitClass>() { _d.negate()? } else if let Some(_d) = p.0.as_any().downcast_ref::<Object>() { _d.negate()? } else if let Some(_d) = p.0.as_any().downcast_ref::<Object>() { _d.negate()? } else if let Some(__f) = p.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                p = _vdispatch2;
            }
            Ok(p)
        }

        #[java_method(name = "newCharProperty", descriptor = "(Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharProperty;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newCharProperty(&self, mut p: Object) -> Result<Pattern_CharProperty> {
            let this = self;
            if _is_jnull(&p) {
                return Ok(Default::default());
            }
            if (p.is_instance_of("java/util/regex/Pattern$BmpCharPredicate")) {
                return Ok(<_ as Into<Pattern_CharProperty>>::into(Pattern_BmpCharProperty::new(Clone::clone(&p))?));
            }
            this.__set_hasSupplementary((1i32 != 0i32));
            Ok(Pattern_CharProperty::new(Clone::clone(&p))?)
        }

        #[java_method(name = "groupname", descriptor = "(I)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn groupname(&self, mut ch: i32) -> Result<String> {
            let this = self;
            let mut sb = StringBuilder::new()?;
            let _t0: bool = ASCII::isAlpha(ch)?;
            if !(_t0) {
                let _t1 = this.error(Clone::clone(&String::from("capturing group name does not start with a Latin letter")))?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            loop {
                let _t1 = sb.append_c(((((ch) as u16 as i32)) as u16))?;
                let _t2 = this.read()?;
                ch = _t2;
                let _t3: bool = ASCII::isAlnum(ch)?;
                if !(_t3) { break; }
            }
            if ch != 62i32 {
                let _t1 = this.error(Clone::clone(&String::from("named capturing group is missing trailing '>'")))?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t1 = sb.toString()?;
            Ok(_t1)
        }

        #[java_method(name = "group0", descriptor = "()Ljava/util/regex/Pattern$Node;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn group0(&self) -> Result<Pattern_Node> {
            let this = self;
            let mut capturingGroup: i32 = 0i32;
            let mut save = this.__get_flags0();
            let _vdispatch0: i32 = if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Object>() { _d.size()? } else if let Some(__f) = this.__get_topClosureNodes().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            let mut saveTCNCount: i32 = _vdispatch0;
            this.__set_root(Default::default());
            let _t1 = this.next()?;
            let mut ch: i32 = _t1;
        let mut head: Pattern_Node = Default::default();
        let mut tail = Default::default();
            if ch == 63i32 {
                let _t2 = this.skip()?;
                ch = _t2;
        tail = Default::default();
        head = Default::default();
                match ch {
                    33 => {
                        let _t3 = this.createGroup((1i32 != 0i32))?;
                        head = _t3;
                        tail = this.__get_root();
                        let _t4 = this.expr(Clone::clone(&tail))?;
                        head.__set_next(Clone::clone(&_t4));
                        if ch == 61i32 {
                            tail = Pattern_Pos::new(Clone::clone(&head))?;
                            head = Pattern_Pos::new(Clone::clone(&head))?;
                        } else {
                            tail = Pattern_Neg::new(Clone::clone(&head))?;
                            head = Pattern_Neg::new(Clone::clone(&head))?;
                        }
                    }
                    36 => {
                        let _t3 = this.error(Clone::clone(&String::from("Unknown group type")))?;
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                    58 => {
                        let _t3 = this.createGroup((1i32 != 0i32))?;
                        head = _t3;
                        tail = this.__get_root();
                        let _t4 = this.expr(Clone::clone(&tail))?;
                        head.__set_next(Clone::clone(&_t4));
                    }
                    60 => {
                        let _t3 = this.read()?;
                        ch = _t3;
                        if ch != 61i32 {
                            if ch != 33i32 {
                                let _t4 = this.groupname(ch)?;
                                let mut name: String = _t4;
                                let _t5 = this.namedGroupsMap()?;
                                let _vdispatch6: bool = if let Some(_d) = _t5.0.as_any().downcast_ref::<ImmutableCollections_MapN<Object, Object>>() { _d.containsKey(Object::from_any(name.clone()))? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<ImmutableCollections_Map1<Object, Object>>() { _d.containsKey(Object::from_any(name.clone()))? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<Properties>() { _d.containsKey(Object::from_any(name.clone()))? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<ConcurrentHashMap<Object, Object>>() { _d.containsKey(Object::from_any(name.clone()))? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<ImmutableCollections_AbstractImmutableMap<Object, Object>>() { _d.containsKey(Object::from_any(name.clone()))? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<TreeMap<Object, Object>>() { _d.containsKey(Object::from_any(name.clone()))? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<LinkedHashMap<Object, Object>>() { _d.containsKey(Object::from_any(name.clone()))? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<Hashtable<Object, Object>>() { _d.containsKey(Object::from_any(name.clone()))? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<Object>() { _d.containsKey(Object::from_any(name.clone()))? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<AbstractMap<Object, Object>>() { _d.containsKey(Object::from_any(name.clone()))? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<HashMap<Object, Object>>() { _d.containsKey(Object::from_any(name.clone()))? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<Object>() { _d.containsKey(Object::from_any(name.clone()))? } else if let Some(__f) = _t5.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Object::from_any(name.clone()))? } else { Default::default() };
                                if _vdispatch6 {
                                    let _t7 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Named capturing group <")))?;
                                    let _t8 = _t7.append_str(Clone::clone(&name))?;
                                    let _t9 = _t8.append_str(Clone::clone(&String::from("> is already defined")))?;
                                    let _t10 = _t9.toString()?;
                                    let _t11 = this.error(Clone::clone(&_t10))?;
                                    return Err(JvmError::Custom("athrow".to_owned()));
                                }
                                capturingGroup = 1i32;
                                let _t7 = this.createGroup((0i32 != 0i32))?;
                                head = _t7;
                                tail = this.__get_root();
                                let _t8 = this.namedGroupsMap()?;
                                let _vdispatch9: Object = if let Some(_d) = _t8.0.as_any().downcast_ref::<ImmutableCollections_MapN<Object, Object>>() { _d.put(Object::from_any(name.clone()), (this.__get_capturingGroupCount()).wrapping_sub(1i32).into())? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<ImmutableCollections_Map1<Object, Object>>() { _d.put(Object::from_any(name.clone()), (this.__get_capturingGroupCount()).wrapping_sub(1i32).into())? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<Properties>() { _d.put(Object::from_any(name.clone()), (this.__get_capturingGroupCount()).wrapping_sub(1i32).into())? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<ConcurrentHashMap<Object, Object>>() { _d.put(Object::from_any(name.clone()), (this.__get_capturingGroupCount()).wrapping_sub(1i32).into())? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<ImmutableCollections_AbstractImmutableMap<Object, Object>>() { _d.put(Object::from_any(name.clone()), (this.__get_capturingGroupCount()).wrapping_sub(1i32).into())? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<TreeMap<Object, Object>>() { _d.put(Object::from_any(name.clone()), (this.__get_capturingGroupCount()).wrapping_sub(1i32).into())? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<LinkedHashMap<Object, Object>>() { _d.put(Object::from_any(name.clone()), (this.__get_capturingGroupCount()).wrapping_sub(1i32).into())? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<Hashtable<Object, Object>>() { _d.put(Object::from_any(name.clone()), (this.__get_capturingGroupCount()).wrapping_sub(1i32).into())? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<Object>() { _d.put(Object::from_any(name.clone()), (this.__get_capturingGroupCount()).wrapping_sub(1i32).into())? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<AbstractMap<Object, Object>>() { _d.put(Object::from_any(name.clone()), (this.__get_capturingGroupCount()).wrapping_sub(1i32).into())? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<HashMap<Object, Object>>() { _d.put(Object::from_any(name.clone()), (this.__get_capturingGroupCount()).wrapping_sub(1i32).into())? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<Object>() { _d.put(Object::from_any(name.clone()), (this.__get_capturingGroupCount()).wrapping_sub(1i32).into())? } else if let Some(__f) = _t8.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(name.clone()), (this.__get_capturingGroupCount()).wrapping_sub(1i32).into())? } else { Default::default() };
                                let _t10 = this.expr(Clone::clone(&tail))?;
                                head.__set_next(Clone::clone(&_t10));
                            } else {
                                let mut name = this.__get_cursor();
                                let _t4 = this.createGroup((1i32 != 0i32))?;
                                head = _t4;
                                tail = this.__get_root();
                                let _t5 = this.expr(Clone::clone(&tail))?;
                                head.__set_next(Clone::clone(&_t5));
                                tail.__set_next(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_LookBehindEndNode::INSTANCE())));
                                let mut info = Pattern_TreeInfo::new()?;
                                let _t6 = head.study(Clone::clone(&info))?;
                                if !(info.__get_maxValid()) {
                                    let _t7 = this.error(Clone::clone(&String::from("Look-behind group does not have an obvious maximum length")))?;
                                    return Err(JvmError::Custom("athrow".to_owned()));
                                }
                                let _t7 = this.findSupplementary(name, this.__get_patternLength())?;
                                let mut hasSupplementary = (_t7) as i32;
                                if ch == 61i32 {
                                    tail = (if (hasSupplementary!=0) { Pattern_BehindS::new(Clone::clone(&head), info.__get_maxLength(), info.__get_minLength())? } else { Pattern_Behind::new(Clone::clone(&head), info.__get_maxLength(), info.__get_minLength())? });
                                    head = (if (hasSupplementary!=0) { Pattern_BehindS::new(Clone::clone(&head), info.__get_maxLength(), info.__get_minLength())? } else { Pattern_Behind::new(Clone::clone(&head), info.__get_maxLength(), info.__get_minLength())? });
                                } else {
                                    tail = (if (hasSupplementary!=0) { Pattern_NotBehindS::new(Clone::clone(&head), info.__get_maxLength(), info.__get_minLength())? } else { Pattern_NotBehind::new(Clone::clone(&head), info.__get_maxLength(), info.__get_minLength())? });
                                    head = (if (hasSupplementary!=0) { Pattern_NotBehindS::new(Clone::clone(&head), info.__get_maxLength(), info.__get_minLength())? } else { Pattern_NotBehind::new(Clone::clone(&head), info.__get_maxLength(), info.__get_minLength())? });
                                }
                                let _vdispatch8: i32 = if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Object>() { _d.size()? } else if let Some(__f) = this.__get_topClosureNodes().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
                                if saveTCNCount < _vdispatch8 {
                                    let _vdispatch9: i32 = if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Object>() { _d.size()? } else if let Some(__f) = this.__get_topClosureNodes().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
                                    let _vdispatch10: Object = if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Object>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(__f) = this.__get_topClosureNodes().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32, i32) -> crate::error::Result<Object>>>() { (__f)(saveTCNCount, _vdispatch9)? } else { Default::default() };
                                    if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<Object>() { _d.clear()?; } else if let Some(__f) = _vdispatch10.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<()>>>() { (__f)()?; }
                                }
                            }
                        } else {
                            let mut name = this.__get_cursor();
                            let _t4 = this.createGroup((1i32 != 0i32))?;
                            head = _t4;
                            tail = this.__get_root();
                            let _t5 = this.expr(Clone::clone(&tail))?;
                            head.__set_next(Clone::clone(&_t5));
                            tail.__set_next(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_LookBehindEndNode::INSTANCE())));
                            let mut info = Pattern_TreeInfo::new()?;
                            let _t6 = head.study(Clone::clone(&info))?;
                            if !(info.__get_maxValid()) {
                                let _t7 = this.error(Clone::clone(&String::from("Look-behind group does not have an obvious maximum length")))?;
                                return Err(JvmError::Custom("athrow".to_owned()));
                            }
                            let _t7 = this.findSupplementary(name, this.__get_patternLength())?;
                            let mut hasSupplementary = (_t7) as i32;
                            if ch == 61i32 {
                                tail = (if (hasSupplementary!=0) { Pattern_BehindS::new(Clone::clone(&head), info.__get_maxLength(), info.__get_minLength())? } else { Pattern_Behind::new(Clone::clone(&head), info.__get_maxLength(), info.__get_minLength())? });
                                head = (if (hasSupplementary!=0) { Pattern_BehindS::new(Clone::clone(&head), info.__get_maxLength(), info.__get_minLength())? } else { Pattern_Behind::new(Clone::clone(&head), info.__get_maxLength(), info.__get_minLength())? });
                            } else {
                                tail = (if (hasSupplementary!=0) { Pattern_NotBehindS::new(Clone::clone(&head), info.__get_maxLength(), info.__get_minLength())? } else { Pattern_NotBehind::new(Clone::clone(&head), info.__get_maxLength(), info.__get_minLength())? });
                                head = (if (hasSupplementary!=0) { Pattern_NotBehindS::new(Clone::clone(&head), info.__get_maxLength(), info.__get_minLength())? } else { Pattern_NotBehind::new(Clone::clone(&head), info.__get_maxLength(), info.__get_minLength())? });
                            }
                            let _vdispatch8: i32 = if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Object>() { _d.size()? } else if let Some(__f) = this.__get_topClosureNodes().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
                            if saveTCNCount < _vdispatch8 {
                                let _vdispatch9: i32 = if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Object>() { _d.size()? } else if let Some(__f) = this.__get_topClosureNodes().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
                                let _vdispatch10: Object = if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Object>() { _d.subList(saveTCNCount, _vdispatch9)? } else if let Some(__f) = this.__get_topClosureNodes().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32, i32) -> crate::error::Result<Object>>>() { (__f)(saveTCNCount, _vdispatch9)? } else { Default::default() };
                                if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch10.0.as_any().downcast_ref::<Object>() { _d.clear()?; } else if let Some(__f) = _vdispatch10.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<()>>>() { (__f)()?; }
                            }
                        }
                    }
                    61 => {
                        let _t3 = this.createGroup((1i32 != 0i32))?;
                        head = _t3;
                        tail = this.__get_root();
                        let _t4 = this.expr(Clone::clone(&tail))?;
                        head.__set_next(Clone::clone(&_t4));
                        if ch == 61i32 {
                            tail = Pattern_Pos::new(Clone::clone(&head))?;
                            head = Pattern_Pos::new(Clone::clone(&head))?;
                        } else {
                            tail = Pattern_Neg::new(Clone::clone(&head))?;
                            head = Pattern_Neg::new(Clone::clone(&head))?;
                        }
                    }
                    62 => {
                        let _t3 = this.createGroup((1i32 != 0i32))?;
                        head = _t3;
                        tail = this.__get_root();
                        let _t4 = this.expr(Clone::clone(&tail))?;
                        head.__set_next(Clone::clone(&_t4));
                        tail = Pattern_Ques::new(Clone::clone(&head), Clone::clone(&Pattern_Qtype::INDEPENDENT()))?;
                        head = Pattern_Ques::new(Clone::clone(&head), Clone::clone(&Pattern_Qtype::INDEPENDENT()))?;
                    }
                    64 => {
                        let _t3 = this.error(Clone::clone(&String::from("Unknown group type")))?;
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                    _ => {
                        this.unread()?;
                        this.addFlag()?;
                        let _t3 = this.read()?;
                        ch = _t3;
                        if ch == 41i32 {
                            return Ok(Default::default());
                        }
                    }
                }
            } else {
                capturingGroup = 1i32;
                let _t2 = this.createGroup((0i32 != 0i32))?;
                head = _t2;
                tail = this.__get_root();
                let _t3 = this.expr(Clone::clone(&tail))?;
                head.__set_next(Clone::clone(&_t3));
            }
            this.accept(41i32, Clone::clone(&String::from("Unclosed group")))?;
            this.__set_flags0(save);
            let _t2 = this.closure(head)?;
            let mut name: Pattern_Node = _t2;
            if Object::from_any(name.clone()) == head {
                this.__set_root(tail);
                return Ok(name);
            }
            if head == tail {
                this.__set_root(Clone::clone(&name));
                return Ok(name);
            }
            let _vdispatch3: i32 = if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Object>() { _d.size()? } else if let Some(__f) = this.__get_topClosureNodes().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            if saveTCNCount < _vdispatch3 {
                let _vdispatch4: i32 = if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.size()? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Object>() { _d.size()? } else if let Some(__f) = this.__get_topClosureNodes().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
                let _vdispatch5: Object = if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.subList(saveTCNCount, _vdispatch4)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.subList(saveTCNCount, _vdispatch4)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.subList(saveTCNCount, _vdispatch4)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.subList(saveTCNCount, _vdispatch4)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.subList(saveTCNCount, _vdispatch4)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.subList(saveTCNCount, _vdispatch4)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.subList(saveTCNCount, _vdispatch4)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.subList(saveTCNCount, _vdispatch4)? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Object>() { _d.subList(saveTCNCount, _vdispatch4)? } else if let Some(__f) = this.__get_topClosureNodes().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32, i32) -> crate::error::Result<Object>>>() { (__f)(saveTCNCount, _vdispatch4)? } else { Default::default() };
                if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.clear()?; } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<Object>() { _d.clear()?; } else if let Some(__f) = _vdispatch5.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<()>>>() { (__f)()?; }
            }
            let mut info: Pattern_Ques = Default::default();
            if Object::from_any(info.__get_type_().clone()) == Object::from_any(Pattern_Qtype::POSSESSIVE().clone()) {
                this.__set_root(Clone::clone(&name));
                return Ok(name);
            }
            tail.__set_next(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_BranchConn::new()?)));
            tail = tail.__get_next();
            if Object::from_any(info.__get_type_().clone()) == Object::from_any(Pattern_Qtype::GREEDY().clone()) {
                head = Pattern_Branch::new(head, Default::default(), Clone::clone(&tail))?;
            } else {
                head = Pattern_Branch::new(Default::default(), head, Clone::clone(&tail))?;
            }
            this.__set_root(Clone::clone(&tail));
            return Ok(<_ as Into<Pattern_Node>>::into(head));
            let mut hasSupplementary: Pattern_Curly = Default::default();
            if Object::from_any(hasSupplementary.__get_type_().clone()) == Object::from_any(Pattern_Qtype::POSSESSIVE().clone()) {
                this.__set_root(Clone::clone(&name));
                return Ok(name);
            }
            let mut info = Pattern_TreeInfo::new()?;
            let _t4 = head.study(Clone::clone(&info))?;
            if _t4 {
                let mut temp: Pattern_GroupTail = Default::default();
                this.__set_root(<_ as Into<Pattern_Node>>::into(Pattern_GroupCurly::new(Clone::clone(&head.__get_next()), hasSupplementary.__get_cmin(), hasSupplementary.__get_cmax(), Clone::clone(&hasSupplementary.__get_type_()), Default::default().__get_localIndex(), Default::default().__get_groupIndex(), (capturingGroup != 0i32))?));
                head = Pattern_GroupCurly::new(Clone::clone(&head.__get_next()), hasSupplementary.__get_cmin(), hasSupplementary.__get_cmax(), Clone::clone(&hasSupplementary.__get_type_()), Default::default().__get_localIndex(), Default::default().__get_groupIndex(), (capturingGroup != 0i32))?;
                return Ok(<_ as Into<Pattern_Node>>::into(head));
            }
            let mut temp = Default::default().__get_localIndex();
        let mut loop_ = Default::default();
            if Object::from_any(hasSupplementary.__get_type_().clone()) == Object::from_any(Pattern_Qtype::GREEDY().clone()) {
                loop_ = Pattern_Loop::new(this.__get_localCount(), temp)?;
                if hasSupplementary.__get_cmax() == 982i32 {
                    let _vdispatch5: bool = if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.add(Object::from_any(loop_.clone()))? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.add(Object::from_any(loop_.clone()))? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.add(Object::from_any(loop_.clone()))? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.add(Object::from_any(loop_.clone()))? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.add(Object::from_any(loop_.clone()))? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.add(Object::from_any(loop_.clone()))? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.add(Object::from_any(loop_.clone()))? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.add(Object::from_any(loop_.clone()))? } else if let Some(_d) = this.__get_topClosureNodes().0.as_any().downcast_ref::<Object>() { _d.add(Object::from_any(loop_.clone()))? } else if let Some(__f) = this.__get_topClosureNodes().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Object::from_any(loop_.clone()))? } else { Default::default() };
                }
            } else {
                loop_ = Pattern_LazyLoop::new(this.__get_localCount(), temp)?;
            }
            let mut prolog = Pattern_Prolog::new(Clone::clone(&loop_))?;
            this.__set_localCount((this.__get_localCount()).wrapping_add(1i32));
            loop_.__set_cmin(hasSupplementary.__get_cmin());
            loop_.__set_cmax(hasSupplementary.__get_cmax());
            loop_.__set_body(Clone::clone(&<_ as Into<Pattern_Node>>::into(head)));
            tail.__set_next(Clone::clone(&<_ as Into<Pattern_Node>>::into(loop_)));
            this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(loop_)));
            return Ok(<_ as Into<Pattern_Node>>::into(prolog));
            let _t5 = this.error(Clone::clone(&String::from("Internal logic error")))?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }

        #[java_method(name = "createGroup", descriptor = "(Z)Ljava/util/regex/Pattern$Node;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createGroup(&self, mut anonymous: bool) -> Result<Pattern_Node> {
            let this = self;
            this.__set_localCount((this.__get_localCount()).wrapping_add(1i32));
            let mut localIndex = this.__get_localCount();
            let mut groupIndex: i32 = 0i32;
            if !(anonymous) {
                this.__set_capturingGroupCount((this.__get_capturingGroupCount()).wrapping_add(1i32));
                groupIndex = this.__get_capturingGroupCount();
            }
            let mut head = Pattern_GroupHead::new(localIndex)?;
            this.__set_root(Clone::clone(&<_ as Into<Pattern_Node>>::into(Pattern_GroupTail::new(localIndex, groupIndex)?)));
            head.__set_tail(Default::default());
            if groupIndex < 10i32 {
                this.__get_groupNodes().borrow_mut()[groupIndex as usize] = Clone::clone(&head);
            }
            Ok(<_ as Into<Pattern_Node>>::into(head))
        }

        #[java_method(name = "addFlag", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addFlag(&self) -> Result<()> {
            let this = self;
            let _t0 = this.peek()?;
            let mut ch: i32 = _t0;
            loop {
                match ch {
                    45 => {
                        let _t1 = this.next()?;
                        ch = _t1;
                        this.subFlag()?;
                        return Ok(());
                    }
                    85 => {
                        this.__set_flags0((this.__get_flags0()|320i32));
                    }
                    99 => {
                        this.__set_flags0((this.__get_flags0()|128i32));
                    }
                    100 => {
                        this.__set_flags0((this.__get_flags0()|1i32));
                    }
                    105 => {
                        this.__set_flags0((this.__get_flags0()|2i32));
                    }
                    109 => {
                        this.__set_flags0((this.__get_flags0()|8i32));
                    }
                    115 => {
                        this.__set_flags0((this.__get_flags0()|32i32));
                    }
                    117 => {
                        this.__set_flags0((this.__get_flags0()|64i32));
                    }
                    120 => {
                        this.__set_flags0((this.__get_flags0()|4i32));
                    }
                    _ => {
                        return Ok(());
                    }
                }
                let _t1 = this.next()?;
                ch = _t1;
            }
            Ok(())
        }

        #[java_method(name = "subFlag", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subFlag(&self) -> Result<()> {
            let this = self;
            let _t0 = this.peek()?;
            let mut ch: i32 = _t0;
            loop {
                match ch {
                    85 => {
                        this.__set_flags0((this.__get_flags0()&-321i32));
                    }
                    99 => {
                        this.__set_flags0((this.__get_flags0()&-129i32));
                    }
                    100 => {
                        this.__set_flags0((this.__get_flags0()&-2i32));
                    }
                    105 => {
                        this.__set_flags0((this.__get_flags0()&-3i32));
                    }
                    109 => {
                        this.__set_flags0((this.__get_flags0()&-9i32));
                    }
                    115 => {
                        this.__set_flags0((this.__get_flags0()&-33i32));
                    }
                    117 => {
                        this.__set_flags0((this.__get_flags0()&-65i32));
                    }
                    120 => {
                        this.__set_flags0((this.__get_flags0()&-5i32));
                    }
                    _ => {
                        return Ok(());
                    }
                }
                let _t1 = this.next()?;
                ch = _t1;
            }
            Ok(())
        }

        #[java_method(name = "qtype", descriptor = "()Ljava/util/regex/Pattern$Qtype;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn qtype(&self) -> Result<Pattern_Qtype> {
            let this = self;
            let _t0 = this.next()?;
            let mut ch: i32 = _t0;
            if ch == 63i32 {
                let _t1 = this.next()?;
                return Ok(Pattern_Qtype::LAZY());
            }
            if ch == 43i32 {
                let _t1 = this.next()?;
                return Ok(Pattern_Qtype::POSSESSIVE());
            }
            Ok(Pattern_Qtype::GREEDY())
        }

        #[java_method(name = "curly", descriptor = "(Ljava/util/regex/Pattern$Node;I)Ljava/util/regex/Pattern$Node;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn curly(&self, mut prev: Pattern_Node, mut cmin: i32) -> Result<Pattern_Node> {
            let this = self;
            let _t0 = this.qtype()?;
            let mut qtype: Pattern_Qtype = _t0;
            if false {
                return Ok(<_ as Into<Pattern_Node>>::into(Pattern_BmpCharPropertyGreedy::new(Clone::clone(&Default::default()), cmin)?));
            }
            if false {
                return Ok(<_ as Into<Pattern_Node>>::into(Pattern_CharPropertyGreedy::new(Clone::clone(&Default::default()), cmin)?));
            }
            Ok(<_ as Into<Pattern_Node>>::into(Pattern_Curly::new(Clone::clone(&prev), cmin, 982i32, Clone::clone(&qtype))?))
        }

        #[java_method(name = "closure", descriptor = "(Ljava/util/regex/Pattern$Node;)Ljava/util/regex/Pattern$Node;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn closure(&self, mut prev: Pattern_Node) -> Result<Pattern_Node> {
            let this = self;
            let _t0 = this.peek()?;
            let mut ch: i32 = _t0;
            let _switch_key = ch;
            let _t1 = this.qtype()?;
            return Ok(<_ as Into<Pattern_Node>>::into(Pattern_Ques::new(Clone::clone(&prev), Clone::clone(&_t1))?));
            let _t2 = this.curly(Clone::clone(&prev), 0i32)?;
            return Ok(_t2);
            let _t3 = this.curly(Clone::clone(&prev), 1i32)?;
            return Ok(_t3);
            let _t4 = this.skip()?;
            ch = _t4;
            let _t5: bool = ASCII::isDigit(ch)?;
            let mut cmin: i32 = 0i32;
            loop {
                let _t6: i32 = Math::multiplyExact_i_i(cmin, 10i32)?;
                let _t7: i32 = Math::addExact_i_i(_t6, (ch).wrapping_sub(48i32))?;
                cmin = _t7;
                let _t8 = this.read()?;
                ch = _t8;
                let _t9: bool = ASCII::isDigit(ch)?;
                if !(_t9) { break; }
            }
            let _t6 = this.read()?;
            ch = _t6;
            if ch == 125i32 {
                this.unread()?;
                let _t7 = this.curly(Clone::clone(&prev), cmin)?;
                return Ok(_t7);
            }
            let mut cmax: i32 = 0i32;
            loop {
                let _t7: bool = ASCII::isDigit(ch)?;
                if !(_t7) { break; }
                let _t7: i32 = Math::multiplyExact_i_i(cmax, 10i32)?;
                let _t8: i32 = Math::addExact_i_i(_t7, (ch).wrapping_sub(48i32))?;
                cmax = _t8;
                let _t9 = this.read()?;
                ch = _t9;
            }
            cmax = cmin;
            if ch != 125i32 {
                let _t7 = this.error(Clone::clone(&String::from("Unclosed counted closure")))?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if cmax < cmin {
                let _t7 = this.error(Clone::clone(&String::from("Illegal repetition range")))?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.unread()?;
            let mut _merged9: Pattern_Ques;
            if (cmin==0) {
                let mut _merged8: Pattern_Ques;
                if cmax == 1i32 {
                    let _t7 = this.qtype()?;
                    _merged8 = Pattern_Ques::new(Clone::clone(&prev), Clone::clone(&_t7))?;
                } else {
                    let _t7 = this.qtype()?;
                    _merged8 = Pattern_Curly::new(Clone::clone(&prev), cmin, cmax, Clone::clone(&_t7))?;
                }
                _merged9 = _merged8;
            } else {
                let _t7 = this.qtype()?;
                _merged9 = Pattern_Curly::new(Clone::clone(&prev), cmin, cmax, Clone::clone(&_t7))?;
            }
            return Ok(<_ as Into<Pattern_Node>>::into(_merged9));
            let _t10 = this.error(Clone::clone(&String::from("Illegal repetition")))?;
            return Err(JvmError::Custom("athrow".to_owned()));
            Ok(prev)
        }

        #[java_method(name = "c", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn c(&self) -> Result<i32> {
            let this = self;
            if this.__get_cursor() < this.__get_patternLength() {
                let _t0 = this.read()?;
                return Ok((_t0^64i32));
            }
            let _t0 = this.error(Clone::clone(&String::from("Illegal control escape sequence")))?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }

        #[java_method(name = "o", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn o(&self) -> Result<i32> {
            let this = self;
            let _t0 = this.read()?;
            let mut n: i32 = _t0;
            let _t1 = this.read()?;
            let mut m: i32 = _t1;
            let _t2 = this.read()?;
            let mut o: i32 = _t2;
            if (((n).wrapping_sub(48i32)|(51i32).wrapping_sub(n))>=0) {
                return Ok(((((n).wrapping_sub(48i32)).wrapping_mul(64i32)).wrapping_add(((m).wrapping_sub(48i32)).wrapping_mul(8i32))).wrapping_add((o).wrapping_sub(48i32)));
            }
            this.unread()?;
            return Ok((((n).wrapping_sub(48i32)).wrapping_mul(8i32)).wrapping_add((m).wrapping_sub(48i32)));
            this.unread()?;
            return Ok((n).wrapping_sub(48i32));
            let _t3 = this.error(Clone::clone(&String::from("Illegal octal escape sequence")))?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }

        #[java_method(name = "x", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn x(&self) -> Result<i32> {
            let this = self;
            let _t0 = this.read()?;
            let mut n: i32 = _t0;
            let _t1: bool = ASCII::isHexDigit(n)?;
            if _t1 {
                let _t2 = this.read()?;
                let mut m: i32 = _t2;
                let _t3: bool = ASCII::isHexDigit(m)?;
                if _t3 {
                    let _t4: i32 = ASCII::toDigit(n)?;
                    let _t5: i32 = ASCII::toDigit(m)?;
                    return Ok(((_t4).wrapping_mul(16i32)).wrapping_add(_t5));
                }
            } else {
                let _t2 = this.peek()?;
                let _t3: bool = ASCII::isHexDigit(_t2)?;
                let mut m: i32 = 0i32;
                loop {
                    let _t4 = this.read()?;
                    n = _t4;
                    let _t5: bool = ASCII::isHexDigit(n)?;
                    let _t6: i32 = ASCII::toDigit(n)?;
                    m = ((m<<(4i32&0x1f))).wrapping_add(_t6);
                    if m > 1048i32 { break; }
                }
                let _t4 = this.error(Clone::clone(&String::from("Hexadecimal codepoint is too big")))?;
                return Err(JvmError::Custom("athrow".to_owned()));
                if n != 125i32 {
                    let _t5 = this.error(Clone::clone(&String::from("Unclosed hexadecimal escape sequence")))?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                return Ok(m);
            }
            let _t2 = this.error(Clone::clone(&String::from("Illegal hexadecimal escape sequence")))?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }

        #[java_method(name = "cursor", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn cursor(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_cursor())
        }

        #[java_method(name = "setcursor", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setcursor(&self, mut pos: i32) -> Result<()> {
            let this = self;
            this.__set_cursor(pos);
            Ok(())
        }

        #[java_method(name = "uxxxx", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn uxxxx(&self) -> Result<i32> {
            let this = self;
            let mut n: i32 = 0i32;
            let mut i: i32 = 0i32;
            loop {
                if i >= 4i32 { break; }
                let _t0 = this.read()?;
                let mut ch: i32 = _t0;
                let _t1: bool = ASCII::isHexDigit(ch)?;
                if !(_t1) {
                    let _t2 = this.error(Clone::clone(&String::from("Illegal Unicode escape sequence")))?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                let _t2: i32 = ASCII::toDigit(ch)?;
                n = ((n).wrapping_mul(16i32)).wrapping_add(_t2);
                i = i.wrapping_add(1i32);
            }
            Ok(n)
        }

        #[java_method(name = "u", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn u(&self) -> Result<i32> {
            let this = self;
            let _t0 = this.uxxxx()?;
            let mut n: i32 = _t0;
            let _t1: bool = Character::isHighSurrogate(((((n) as u16 as i32)) as u16))?;
            let _t2 = this.cursor()?;
            let mut cur: i32 = _t2;
            let _t3 = this.read()?;
            let _t4 = this.read()?;
            let _t5 = this.uxxxx()?;
            let mut n2: i32 = _t5;
            let _t6: bool = Character::isLowSurrogate(((((n2) as u16 as i32)) as u16))?;
            if _t6 {
                let _t7: i32 = Character::toCodePoint(((((n) as u16 as i32)) as u16), ((((n2) as u16 as i32)) as u16))?;
                return Ok(_t7);
            }
            this.setcursor(cur)?;
            Ok(n)
        }

        #[java_method(name = "N", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn N(&self) -> Result<i32> {
            panic!("stub: java/util/regex/Pattern.N:()I")
        }

        #[java_method(name = "countChars", descriptor = "(Ljava/lang/CharSequence;II)I", access = "private", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn countChars(mut seq: Object, mut index: i32, mut lengthInCodePoints: i32) -> Result<i32> {
            let _vdispatch0: i32 = if let Some(_d) = seq.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<CharBuffer>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<String>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<StringBuilder>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<Object>() { _d.length()? } else if let Some(__f) = seq.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            let _vdispatch1: u16 = if let Some(_d) = seq.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.charAt(index)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<CharBuffer>() { _d.charAt(index)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.charAt(index)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<String>() { _d.charAt(index)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<StringBuilder>() { _d.charAt(index)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<Object>() { _d.charAt(index)? } else if let Some(__f) = seq.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<u16>>>() { (__f)(index)? } else { Default::default() };
            let _t2: bool = Character::isHighSurrogate(_vdispatch1)?;
            if !(_t2) {
                return Ok(1i32);
            }
            let _vdispatch3: i32 = if let Some(_d) = seq.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<CharBuffer>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<String>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<StringBuilder>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<Object>() { _d.length()? } else if let Some(__f) = seq.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            let mut length: i32 = _vdispatch3;
            let mut x: i32 = index;
            if index >= length {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut i: i32 = 0i32;
            loop {
                if x >= length { break; }
                x = x.wrapping_add(1i32);
                let _vdispatch4: u16 = if let Some(_d) = seq.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.charAt(x)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<CharBuffer>() { _d.charAt(x)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.charAt(x)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<String>() { _d.charAt(x)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<StringBuilder>() { _d.charAt(x)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<Object>() { _d.charAt(x)? } else if let Some(__f) = seq.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<u16>>>() { (__f)(x)? } else { Default::default() };
                let _t5: bool = Character::isHighSurrogate(_vdispatch4)?;
                let _vdispatch6: u16 = if let Some(_d) = seq.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.charAt(x)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<CharBuffer>() { _d.charAt(x)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.charAt(x)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<String>() { _d.charAt(x)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<StringBuilder>() { _d.charAt(x)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<Object>() { _d.charAt(x)? } else if let Some(__f) = seq.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<u16>>>() { (__f)(x)? } else { Default::default() };
                let _t7: bool = Character::isLowSurrogate(_vdispatch6)?;
                if _t7 {
                    x = x.wrapping_add(1i32);
                }
                i = i.wrapping_add(1i32);
            }
            return Ok((x).wrapping_sub(index));
            if index > length {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (index==0) {
                return Ok(0i32);
            }
            i = (lengthInCodePoints).wrapping_neg();
            let mut i: i32 = 0i32;
            loop {
                if (x<=0) { break; }
                x = x.wrapping_sub(1i32);
                let _vdispatch4: u16 = if let Some(_d) = seq.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.charAt(x)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<CharBuffer>() { _d.charAt(x)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.charAt(x)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<String>() { _d.charAt(x)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<StringBuilder>() { _d.charAt(x)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<Object>() { _d.charAt(x)? } else if let Some(__f) = seq.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<u16>>>() { (__f)(x)? } else { Default::default() };
                let _t5: bool = Character::isLowSurrogate(_vdispatch4)?;
                let _vdispatch6: u16 = if let Some(_d) = seq.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.charAt((x).wrapping_sub(1i32))? } else if let Some(_d) = seq.0.as_any().downcast_ref::<CharBuffer>() { _d.charAt((x).wrapping_sub(1i32))? } else if let Some(_d) = seq.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.charAt((x).wrapping_sub(1i32))? } else if let Some(_d) = seq.0.as_any().downcast_ref::<String>() { _d.charAt((x).wrapping_sub(1i32))? } else if let Some(_d) = seq.0.as_any().downcast_ref::<StringBuilder>() { _d.charAt((x).wrapping_sub(1i32))? } else if let Some(_d) = seq.0.as_any().downcast_ref::<Object>() { _d.charAt((x).wrapping_sub(1i32))? } else if let Some(__f) = seq.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<u16>>>() { (__f)((x).wrapping_sub(1i32))? } else { Default::default() };
                let _t7: bool = Character::isHighSurrogate(_vdispatch6)?;
                if _t7 {
                    x = x.wrapping_sub(1i32);
                }
                i = i.wrapping_add(1i32);
            }
            Ok((index).wrapping_sub(x))
        }

        #[java_method(name = "countCodePoints", descriptor = "(Ljava/lang/CharSequence;)I", access = "private", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn countCodePoints(mut seq: Object) -> Result<i32> {
            let _vdispatch0: i32 = if let Some(_d) = seq.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<CharBuffer>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<String>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<StringBuilder>() { _d.length()? } else if let Some(_d) = seq.0.as_any().downcast_ref::<Object>() { _d.length()? } else if let Some(__f) = seq.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            let mut length: i32 = _vdispatch0;
            let mut n: i32 = 0i32;
            let mut i: i32 = 0i32;
            loop {
                if i >= length { break; }
                n = n.wrapping_add(1i32);
                i = i.wrapping_add(1i32);
                let _vdispatch1: u16 = if let Some(_d) = seq.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.charAt(i)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<CharBuffer>() { _d.charAt(i)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.charAt(i)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<String>() { _d.charAt(i)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<StringBuilder>() { _d.charAt(i)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<Object>() { _d.charAt(i)? } else if let Some(__f) = seq.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<u16>>>() { (__f)(i)? } else { Default::default() };
                let _t2: bool = Character::isHighSurrogate(_vdispatch1)?;
                let _vdispatch3: u16 = if let Some(_d) = seq.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.charAt(i)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<CharBuffer>() { _d.charAt(i)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.charAt(i)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<String>() { _d.charAt(i)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<StringBuilder>() { _d.charAt(i)? } else if let Some(_d) = seq.0.as_any().downcast_ref::<Object>() { _d.charAt(i)? } else if let Some(__f) = seq.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<u16>>>() { (__f)(i)? } else { Default::default() };
                let _t4: bool = Character::isLowSurrogate(_vdispatch3)?;
                i = i.wrapping_add(1i32);
            }
            Ok(n)
        }

        #[java_method(name = "newSlice", descriptor = "([IIZ)Ljava/util/regex/Pattern$Node;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newSlice(&self, mut buf: Rc<RefCell<Vec<i32>>>, mut count: i32, mut hasSupplementary: bool) -> Result<Pattern_Node> {
            let this = self;
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; count as usize]));
            let mut tmp: Rc<RefCell<Vec<i32>>> = _arr0;
            let _t1 = this.has(2i32)?;
            let _t2 = this.has(64i32)?;
            let mut i: i32 = 0i32;
            loop {
                if i >= count { break; }
                let _t3: i32 = Character::toUpperCase_i(buf.borrow()[i as usize])?;
                let _t4: i32 = Character::toLowerCase_i(_t3)?;
                tmp.borrow_mut()[i as usize] = _t4;
                i = i.wrapping_add(1i32);
            }
            return Ok(<_ as Into<Pattern_Node>>::into((if hasSupplementary { Pattern_SliceUS::new(Clone::clone(&tmp))? } else { Pattern_SliceU::new(Clone::clone(&tmp))? })));
            i = 0i32;
            loop {
                if i >= count { break; }
                let _t3: i32 = ASCII::toLower(buf.borrow()[i as usize])?;
                tmp.borrow_mut()[i as usize] = _t3;
                i = i.wrapping_add(1i32);
            }
            return Ok(<_ as Into<Pattern_Node>>::into((if hasSupplementary { Pattern_SliceIS::new(Clone::clone(&tmp))? } else { Pattern_SliceI::new(Clone::clone(&tmp))? })));
            i = 0i32;
            loop {
                if i >= count { break; }
                tmp.borrow_mut()[i as usize] = buf.borrow()[i as usize];
                i = i.wrapping_add(1i32);
            }
            Ok(<_ as Into<Pattern_Node>>::into((if hasSupplementary { Pattern_SliceS::new(Clone::clone(&tmp))? } else { Pattern_Slice::new(Clone::clone(&tmp))? })))
        }

        #[java_method(name = "hasBaseCharacter", descriptor = "(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasBaseCharacter(matcher: Matcher, i: i32, seq: Object) -> Result<bool> {
            panic!("stub: java/util/regex/Pattern.hasBaseCharacter:(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z")
        }

        #[java_method(name = "and", descriptor = "(Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Z)Ljava/util/regex/Pattern$CharPredicate;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn and(mut p1: Object, mut p2: Object, mut bmpChar: bool) -> Result<Object> {
            if bmpChar {
                let __lam_cap1114_0 = p2;
                let __lam_cap1114_1 = p1;
                let __lam_1114: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_and_1(__lam_cap1114_0.clone(), __lam_cap1114_1.clone(), _la0) });
                return Ok(Object::from_any(__lam_1114));
            }
            let __lam_cap1118_0 = p2;
            let __lam_cap1118_1 = p1;
            let __lam_1118: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_and_2(__lam_cap1118_0.clone(), __lam_cap1118_1.clone(), _la0) });
            Ok(Object::from_any(__lam_1118))
        }

        #[java_method(name = "union", descriptor = "(Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Z)Ljava/util/regex/Pattern$CharPredicate;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: union(Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Z)Ljava/util/regex/Pattern$CharPredicate;
        pub fn union_patter_patter_z(mut p1: Object, mut p2: Object, mut bmpChar: bool) -> Result<Object> {
            if bmpChar {
                let __lam_cap1121_0 = p2;
                let __lam_cap1121_1 = p1;
                let __lam_1121: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_union_3(__lam_cap1121_0.clone(), __lam_cap1121_1.clone(), _la0) });
                return Ok(Object::from_any(__lam_1121));
            }
            let __lam_cap1122_0 = p2;
            let __lam_cap1122_1 = p1;
            let __lam_1122: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_union_4(__lam_cap1122_0.clone(), __lam_cap1122_1.clone(), _la0) });
            Ok(Object::from_any(__lam_1122))
        }

        #[java_method(name = "union", descriptor = "(Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Z)Ljava/util/regex/Pattern$CharPredicate;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: union(Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Z)Ljava/util/regex/Pattern$CharPredicate;
        pub fn union_patter_patter_patter_z(mut p1: Object, mut p2: Object, mut p3: Object, mut bmpChar: bool) -> Result<Object> {
            if bmpChar {
                let __lam_cap1123_0 = p3;
                let __lam_cap1123_1 = p2;
                let __lam_cap1123_2 = p1;
                let __lam_1123: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_union_5(__lam_cap1123_0.clone(), __lam_cap1123_1.clone(), __lam_cap1123_2.clone(), _la0) });
                return Ok(Object::from_any(__lam_1123));
            }
            let __lam_cap1126_0 = p3;
            let __lam_cap1126_1 = p2;
            let __lam_cap1126_2 = p1;
            let __lam_1126: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_union_6(__lam_cap1126_0.clone(), __lam_cap1126_1.clone(), __lam_cap1126_2.clone(), _la0) });
            Ok(Object::from_any(__lam_1126))
        }

        #[java_method(name = "negate", descriptor = "(Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharPredicate;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn negate(mut p1: Object) -> Result<Object> {
            let __lam_cap1129_0 = p1;
            let __lam_1129: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_negate_7(__lam_cap1129_0.clone(), _la0) });
            Ok(Object::from_any(__lam_1129))
        }

        #[java_method(name = "VertWS", descriptor = "()Ljava/util/regex/Pattern$BmpCharPredicate;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn VertWS() -> Result<Object> {
            let __lam_1131: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_VertWS_8(_la0) });
            Ok(Object::from_any(__lam_1131))
        }

        #[java_method(name = "HorizWS", descriptor = "()Ljava/util/regex/Pattern$BmpCharPredicate;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn HorizWS() -> Result<Object> {
            let __lam_1133: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_HorizWS_9(_la0) });
            Ok(Object::from_any(__lam_1133))
        }

        #[java_method(name = "ALL", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ALL() -> Result<Object> {
            let __lam_1134: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_ALL_10(_la0) });
            Ok(Object::from_any(__lam_1134))
        }

        #[java_method(name = "DOT", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn DOT() -> Result<Object> {
            let __lam_1136: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_DOT_11(_la0) });
            Ok(Object::from_any(__lam_1136))
        }

        #[java_method(name = "UNIXDOT", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn UNIXDOT() -> Result<Object> {
            let __lam_1137: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_UNIXDOT_12(_la0) });
            Ok(Object::from_any(__lam_1137))
        }

        #[java_method(name = "SingleS", descriptor = "(I)Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn SingleS(mut c: i32) -> Result<Object> {
            let __lam_cap1138_0 = c;
            let __lam_1138: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_SingleS_13(__lam_cap1138_0.clone(), _la0) });
            Ok(Object::from_any(__lam_1138))
        }

        #[java_method(name = "Single", descriptor = "(I)Ljava/util/regex/Pattern$BmpCharPredicate;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn Single(mut c: i32) -> Result<Object> {
            let __lam_cap1140_0 = c;
            let __lam_1140: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_Single_14(__lam_cap1140_0.clone(), _la0) });
            Ok(Object::from_any(__lam_1140))
        }

        #[java_method(name = "SingleI", descriptor = "(II)Ljava/util/regex/Pattern$BmpCharPredicate;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn SingleI(mut lower: i32, mut upper: i32) -> Result<Object> {
            let __lam_cap1142_0 = upper;
            let __lam_cap1142_1 = lower;
            let __lam_1142: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_SingleI_15(__lam_cap1142_0.clone(), __lam_cap1142_1.clone(), _la0) });
            Ok(Object::from_any(__lam_1142))
        }

        #[java_method(name = "SingleU", descriptor = "(I)Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn SingleU(mut lower: i32) -> Result<Object> {
            let __lam_cap1144_0 = lower;
            let __lam_1144: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_SingleU_16(__lam_cap1144_0.clone(), _la0) });
            Ok(Object::from_any(__lam_1144))
        }

        #[java_method(name = "inRange", descriptor = "(III)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn inRange(lower: i32, ch: i32, upper: i32) -> Result<bool> {
            panic!("stub: java/util/regex/Pattern.inRange:(III)Z")
        }

        #[java_method(name = "Range", descriptor = "(II)Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn Range(mut lower: i32, mut upper: i32) -> Result<Object> {
            if upper < 490i32 {
                let __lam_cap1147_0 = upper;
                let __lam_cap1147_1 = lower;
                let __lam_1147: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_Range_17(__lam_cap1147_0.clone(), __lam_cap1147_1.clone(), _la0) });
                return Ok(Object::from_any(__lam_1147));
            }
            let __lam_cap1148_0 = upper;
            let __lam_cap1148_1 = lower;
            let __lam_1148: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_Range_18(__lam_cap1148_0.clone(), __lam_cap1148_1.clone(), _la0) });
            Ok(Object::from_any(__lam_1148))
        }

        #[java_method(name = "CIRange", descriptor = "(II)Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CIRange(mut lower: i32, mut upper: i32) -> Result<Object> {
            let __lam_cap1150_0 = upper;
            let __lam_cap1150_1 = lower;
            let __lam_1150: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_CIRange_19(__lam_cap1150_0.clone(), __lam_cap1150_1.clone(), _la0) });
            Ok(Object::from_any(__lam_1150))
        }

        #[java_method(name = "CIRangeU", descriptor = "(II)Ljava/util/regex/Pattern$CharPredicate;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CIRangeU(mut lower: i32, mut upper: i32) -> Result<Object> {
            let __lam_cap1151_0 = upper;
            let __lam_cap1151_1 = lower;
            let __lam_1151: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<bool>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<bool> { Pattern::lambda_CIRangeU_20(__lam_cap1151_0.clone(), __lam_cap1151_1.clone(), _la0) });
            Ok(Object::from_any(__lam_1151))
        }

        #[java_method(name = "asPredicate", descriptor = "()Ljava/util/function/Predicate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/function/Predicate<Ljava/lang/String;>;")]
        pub fn asPredicate(&self) -> Result<Object> {
            panic!("stub: java/util/regex/Pattern.asPredicate:()Ljava/util/function/Predicate;")
        }

        #[java_method(name = "asMatchPredicate", descriptor = "()Ljava/util/function/Predicate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/function/Predicate<Ljava/lang/String;>;")]
        pub fn asMatchPredicate(&self) -> Result<Object> {
            panic!("stub: java/util/regex/Pattern.asMatchPredicate:()Ljava/util/function/Predicate;")
        }

        #[java_method(name = "splitAsStream", descriptor = "(Ljava/lang/CharSequence;)Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/CharSequence;)Ljava/util/stream/Stream<Ljava/lang/String;>;")]
        pub fn splitAsStream(&self, input: Object) -> Result<Object> {
            panic!("stub: java/util/regex/Pattern.splitAsStream:(Ljava/lang/CharSequence;)Ljava/util/stream/Stream;")
        }
    }
}
