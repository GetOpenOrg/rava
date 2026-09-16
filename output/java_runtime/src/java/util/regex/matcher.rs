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
    #[binary_name       = "java/util/regex/Matcher"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/regex/MatchResult"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Matcher.java"]
    #[inner_classes     = "java/util/regex/Matcher$ImmutableMatchResult:java/util/regex/Matcher:ImmutableMatchResult:10;java/util/regex/Matcher$1MatchResultIterator::MatchResultIterator:0;java/util/regex/Pattern$Node:java/util/regex/Pattern:Node:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/regex/MatchResult;java/util/regex/Matcher"]
    #[has_to_string_method = true]

    pub struct Matcher {
        #[cfg_attr(any(), java_field(name = "parentPattern", descriptor = "Ljava/util/regex/Pattern;", is_static = false))]
        pub parentPattern: Pattern,
        #[cfg_attr(any(), java_field(name = "groups", descriptor = "[I", is_static = false))]
        pub groups: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "from", descriptor = "I", is_static = false))]
        pub from: i32,
        #[cfg_attr(any(), java_field(name = "to", descriptor = "I", is_static = false))]
        pub to: i32,
        #[cfg_attr(any(), java_field(name = "lookbehindTo", descriptor = "I", is_static = false))]
        pub lookbehindTo: i32,
        #[cfg_attr(any(), java_field(name = "text", descriptor = "Ljava/lang/CharSequence;", is_static = false))]
        pub text: Object,
        #[cfg_attr(any(), java_field(name = "acceptMode", descriptor = "I", is_static = false))]
        pub acceptMode: i32,
        #[cfg_attr(any(), java_field(name = "first", descriptor = "I", is_static = false))]
        pub first: i32,
        #[cfg_attr(any(), java_field(name = "last", descriptor = "I", is_static = false))]
        pub last: i32,
        #[cfg_attr(any(), java_field(name = "oldLast", descriptor = "I", is_static = false))]
        pub oldLast: i32,
        #[cfg_attr(any(), java_field(name = "lastAppendPosition", descriptor = "I", is_static = false))]
        pub lastAppendPosition: i32,
        #[cfg_attr(any(), java_field(name = "locals", descriptor = "[I", is_static = false))]
        pub locals: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "localsPos", descriptor = "[Ljava/util/regex/IntHashSet;", is_static = false))]
        pub localsPos: Rc<RefCell<Vec<IntHashSet>>>,
        #[cfg_attr(any(), java_field(name = "hitEnd", descriptor = "Z", is_static = false))]
        pub hitEnd: bool,
        #[cfg_attr(any(), java_field(name = "requireEnd", descriptor = "Z", is_static = false))]
        pub requireEnd: bool,
        #[cfg_attr(any(), java_field(name = "transparentBounds", descriptor = "Z", is_static = false))]
        pub transparentBounds: bool,
        #[cfg_attr(any(), java_field(name = "anchoringBounds", descriptor = "Z", is_static = false))]
        pub anchoringBounds: bool,
        #[cfg_attr(any(), java_field(name = "modCount", descriptor = "I", is_static = false))]
        pub modCount: i32,
        #[cfg_attr(any(), java_field(name = "namedGroups", descriptor = "Ljava/util/Map;", access = "private", modifiers = "", is_static = false, generic_signature = "Ljava/util/Map<Ljava/lang/String;Ljava/lang/Integer;>;"))]
        pub namedGroups: Object,
    }

    impl Matcher {
        #[cfg_attr(any(), java_field(name = "ENDANCHOR", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: ENDANCHOR:I
        pub fn ENDANCHOR() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "NOANCHOR", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: NOANCHOR:I
        pub fn NOANCHOR() -> i32 {
            0
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_acceptMode(0i32);
            this.__set_first(-1i32);
            this.__set_last(0i32);
            this.__set_oldLast(-1i32);
            this.__set_lastAppendPosition(0i32);
            this.__set_transparentBounds((0i32 != 0i32));
            this.__set_anchoringBounds((1i32 != 0i32));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/regex/Pattern;Ljava/lang/CharSequence;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/util/regex/Pattern;Ljava/lang/CharSequence;)V
        pub fn new_patter_seq(mut parent: Pattern, mut text: Object) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_acceptMode(0i32);
            this.__set_first(-1i32);
            this.__set_last(0i32);
            this.__set_oldLast(-1i32);
            this.__set_lastAppendPosition(0i32);
            this.__set_transparentBounds((0i32 != 0i32));
            this.__set_anchoringBounds((1i32 != 0i32));
            this.__set_parentPattern(Clone::clone(&parent));
            this.__set_text(Clone::clone(&text));
            let _t0: i32 = Math::max_i_i(parent.__get_capturingGroupCount(), 10i32)?;
            let mut parentGroupCount: i32 = _t0;
            let mut _arr1: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (parentGroupCount).wrapping_mul(2i32) as usize]));
            this.__set_groups(Clone::clone(&_arr1));
            let mut _arr2: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; parent.__get_localCount() as usize]));
            this.__set_locals(Clone::clone(&_arr2));
            let mut _arr3: Rc<RefCell<Vec<IntHashSet>>> = Rc::new(RefCell::new(vec![Default::default(); parent.__get_localTCNCount() as usize]));
            this.__set_localsPos(Clone::clone(&_arr3));
            let _t4 = this.reset()?;
            Ok(this)
        }

        #[java_method(name = "pattern", descriptor = "()Ljava/util/regex/Pattern;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn pattern(&self) -> Result<Pattern> {
            panic!("stub: java/util/regex/Matcher.pattern:()Ljava/util/regex/Pattern;")
        }

        #[java_method(name = "toMatchResult", descriptor = "()Ljava/util/regex/MatchResult;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toMatchResult(&self) -> Result<Object> {
            panic!("stub: java/util/regex/Matcher.toMatchResult:()Ljava/util/regex/MatchResult;")
        }

        #[java_method(name = "minStart", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minStart(&self) -> Result<i32> {
            panic!("stub: java/util/regex/Matcher.minStart:()I")
        }

        #[java_method(name = "maxEnd", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn maxEnd(&self) -> Result<i32> {
            panic!("stub: java/util/regex/Matcher.maxEnd:()I")
        }

        #[java_method(name = "usePattern", descriptor = "(Ljava/util/regex/Pattern;)Ljava/util/regex/Matcher;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn usePattern(&self, newPattern: Pattern) -> Result<Matcher> {
            panic!("stub: java/util/regex/Matcher.usePattern:(Ljava/util/regex/Pattern;)Ljava/util/regex/Matcher;")
        }

        #[java_method(name = "reset", descriptor = "()Ljava/util/regex/Matcher;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: reset()Ljava/util/regex/Matcher;
        pub fn reset(&self) -> Result<Matcher> {
            let this = self;
            this.__set_first(-1i32);
            this.__set_last(0i32);
            this.__set_oldLast(-1i32);
            let mut i: i32 = 0i32;
            loop {
                if i >= (this.__get_groups().borrow().len() as i32) { break; }
                this.__get_groups().borrow_mut()[i as usize] = -1i32;
                i = i.wrapping_add(1i32);
            }
            i = 0i32;
            loop {
                if i >= (this.__get_locals().borrow().len() as i32) { break; }
                this.__get_locals().borrow_mut()[i as usize] = -1i32;
                i = i.wrapping_add(1i32);
            }
            i = 0i32;
            loop {
                if i >= (this.__get_localsPos().borrow().len() as i32) { break; }
                if !_is_jnull(&Clone::clone(&this.__get_localsPos().borrow()[i as usize])) {
                    Clone::clone(&this.__get_localsPos().borrow()[i as usize]).clear()?;
                }
                i = i.wrapping_add(1i32);
            }
            this.__set_lastAppendPosition(0i32);
            this.__set_from(0i32);
            let _t0 = this.getTextLength()?;
            this.__set_to(_t0);
            this.__set_modCount((this.__get_modCount()).wrapping_add(1i32));
            Ok(Clone::clone(this))
        }

        #[java_method(name = "reset", descriptor = "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reset_seq(&self, input: Object) -> Result<Matcher> {
            panic!("stub: java/util/regex/Matcher.reset:(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;")
        }

        #[java_method(name = "start", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: start()I
        pub fn start(&self) -> Result<i32> {
            let this = self;
            this.checkMatch()?;
            Ok(this.__get_first())
        }

        #[java_method(name = "start", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: start(I)I
        pub fn start_i(&self, mut group: i32) -> Result<i32> {
            let this = self;
            this.checkMatch()?;
            this.checkGroup(group)?;
            Ok(this.__get_groups().borrow()[(group).wrapping_mul(2i32) as usize])
        }

        #[java_method(name = "start", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn start_str(&self, name: String) -> Result<i32> {
            panic!("stub: java/util/regex/Matcher.start:(Ljava/lang/String;)I")
        }

        #[java_method(name = "end", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: end()I
        pub fn end(&self) -> Result<i32> {
            let this = self;
            this.checkMatch()?;
            Ok(this.__get_last())
        }

        #[java_method(name = "end", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: end(I)I
        pub fn end_i(&self, mut group: i32) -> Result<i32> {
            let this = self;
            this.checkMatch()?;
            this.checkGroup(group)?;
            Ok(this.__get_groups().borrow()[((group).wrapping_mul(2i32)).wrapping_add(1i32) as usize])
        }

        #[java_method(name = "end", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn end_str(&self, name: String) -> Result<i32> {
            panic!("stub: java/util/regex/Matcher.end:(Ljava/lang/String;)I")
        }

        #[java_method(name = "group", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: group()Ljava/lang/String;
        pub fn group(&self) -> Result<String> {
            let this = self;
            let _t0 = this.group_i(0i32)?;
            Ok(_t0)
        }

        #[java_method(name = "group", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: group(I)Ljava/lang/String;
        pub fn group_i(&self, mut group: i32) -> Result<String> {
            let this = self;
            this.checkMatch()?;
            this.checkGroup(group)?;
            if this.__get_groups().borrow()[((group).wrapping_mul(2i32)).wrapping_add(1i32) as usize] == -1i32 {
                return Ok(Default::default());
            }
            let _t0 = this.getSubSequence(this.__get_groups().borrow()[(group).wrapping_mul(2i32) as usize], this.__get_groups().borrow()[((group).wrapping_mul(2i32)).wrapping_add(1i32) as usize])?;
            let _vdispatch1: String = if let Some(_d) = _t0.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.toString()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<CharBuffer>() { _d.toString()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.toString()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<String>() { _d.toString()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StringBuilder>() { _d.toString()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            Ok(_vdispatch1)
        }

        #[java_method(name = "group", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn group_str(&self, name: String) -> Result<String> {
            panic!("stub: java/util/regex/Matcher.group:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "groupCount", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn groupCount(&self) -> Result<i32> {
            let this = self;
            Ok((this.__get_parentPattern().__get_capturingGroupCount()).wrapping_sub(1i32))
        }

        #[java_method(name = "matches", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn matches(&self) -> Result<bool> {
            let this = self;
            let _t0 = this.match_(this.__get_from(), 1i32)?;
            Ok(_t0)
        }

        #[java_method(name = "find", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: find()Z
        pub fn find(&self) -> Result<bool> {
            let this = self;
            let mut nextSearchIndex = this.__get_last();
            if nextSearchIndex == this.__get_first() {
                nextSearchIndex = nextSearchIndex.wrapping_add(1i32);
            }
            if nextSearchIndex < this.__get_from() {
                nextSearchIndex = this.__get_from();
            }
            let mut i: i32 = 0i32;
            loop {
                if i >= (this.__get_groups().borrow().len() as i32) { break; }
                this.__get_groups().borrow_mut()[i as usize] = -1i32;
                i = i.wrapping_add(1i32);
            }
            return Ok((0i32 != 0i32));
            let _t0 = this.search(nextSearchIndex)?;
            Ok(_t0)
        }

        #[java_method(name = "find", descriptor = "(I)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: find(I)Z
        pub fn find_i(&self, mut start: i32) -> Result<bool> {
            let this = self;
            let _t0 = this.getTextLength()?;
            let mut limit: i32 = _t0;
            if start > limit {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t1 = this.reset()?;
            let _t2 = this.search(start)?;
            Ok(_t2)
        }

        #[java_method(name = "lookingAt", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lookingAt(&self) -> Result<bool> {
            panic!("stub: java/util/regex/Matcher.lookingAt:()Z")
        }

        #[java_method(name = "quoteReplacement", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn quoteReplacement(s: String) -> Result<String> {
            panic!("stub: java/util/regex/Matcher.quoteReplacement:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "appendReplacement", descriptor = "(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn appendReplacement_string_str(&self, sb: Object, replacement: String) -> Result<Matcher> {
            panic!("stub: java/util/regex/Matcher.appendReplacement:(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;")
        }

        #[java_method(name = "appendReplacement", descriptor = "(Ljava/lang/StringBuilder;Ljava/lang/String;)Ljava/util/regex/Matcher;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: appendReplacement(Ljava/lang/StringBuilder;Ljava/lang/String;)Ljava/util/regex/Matcher;
        pub fn appendReplacement_sb_str(&self, mut sb: StringBuilder, mut replacement: String) -> Result<Matcher> {
            let this = self;
            this.checkMatch()?;
            let _t0 = sb.__super().length()?;
            let mut curLen: i32 = _t0;
            let _t1 = sb.append_seq_i_i(Clone::clone(&this.__get_text()), this.__get_lastAppendPosition(), this.__get_first())?;
            this.appendExpandedReplacement(Object::from_any(sb.clone()), Clone::clone(&replacement))?;
            this.__set_lastAppendPosition(this.__get_last());
            this.__set_modCount((this.__get_modCount()).wrapping_add(1i32));
            Ok(Clone::clone(this))
        }

        #[java_method(name = "appendExpandedReplacement", descriptor = "(Ljava/lang/Appendable;Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn appendExpandedReplacement(&self, mut app: Object, mut replacement: String) -> Result<()> {
            let this = self;
            let mut cursor: i32 = 0i32;
            loop {
                let _t0 = replacement.length()?;
                if cursor >= _t0 { break; }
                let _t0 = replacement.charAt(cursor)?;
                let mut nextChar: u16 = _t0;
                if (nextChar as i32) == 92i32 {
                    cursor = cursor.wrapping_add(1i32);
                    let _t1 = replacement.length()?;
                    if cursor == _t1 {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                    let _t2 = replacement.charAt(cursor)?;
                    nextChar = _t2;
                    let _vdispatch3: Object = if let Some(_d) = app.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<Writer>() { _d.append_c(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<Object>() { _d.append(nextChar)? } else if let Some(__f) = app.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(nextChar)? } else { Default::default() };
                    cursor = cursor.wrapping_add(1i32);
                } else {
                    if (nextChar as i32) == 36i32 {
                        cursor = cursor.wrapping_add(1i32);
                        let _t1 = replacement.length()?;
                        if cursor == _t1 {
                            return Err(JvmError::Custom("athrow".to_owned()));
                        }
                        let _t2 = replacement.charAt(cursor)?;
                        nextChar = _t2;
                        let mut refNum: i32 = -1i32;
                        if (nextChar as i32) == 123i32 {
                            cursor = cursor.wrapping_add(1i32);
                            let mut begin: i32 = cursor;
                            loop {
                                let _t3 = replacement.length()?;
                                if cursor >= _t3 { break; }
                                let _t3 = replacement.charAt(cursor)?;
                                nextChar = _t3;
                                let _t4: bool = ASCII::isLower((nextChar as i32))?;
                                let _t5: bool = ASCII::isUpper((nextChar as i32))?;
                                let _t6: bool = ASCII::isDigit((nextChar as i32))?;
                                if _t6 {
                                    cursor = cursor.wrapping_add(1i32);
                                    continue;
                                }
                                break;
                            }
                            if begin == cursor {
                                return Err(JvmError::Custom("athrow".to_owned()));
                            }
                            if (nextChar as i32) != 125i32 {
                                return Err(JvmError::Custom("athrow".to_owned()));
                            }
                            let _t3 = replacement.substring_i_i(begin, cursor)?;
                            let mut gname: String = _t3;
                            let _t4 = gname.charAt(0i32)?;
                            let _t5: bool = ASCII::isDigit((_t4 as i32))?;
                            if _t5 {
                                let _t6 = StringBuilder::new()?.append_str(Clone::clone(&String::from("capturing group name {")))?;
                                let _t7 = _t6.append_str(Clone::clone(&gname))?;
                                let _t8 = _t7.append_str(Clone::clone(&String::from("} starts with digit character")))?;
                                let _t9 = _t8.toString()?;
                                return Err(JvmError::Custom("athrow".to_owned()));
                            }
                            let _t6 = this.namedGroups()?;
                            let _vdispatch7: Object = if let Some(_d) = _t6.0.as_any().downcast_ref::<ImmutableCollections_MapN<Object, Object>>() { _d.get(Object::from_any(gname.clone()))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<ImmutableCollections_Map1<Object, Object>>() { _d.get(Object::from_any(gname.clone()))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<Properties>() { _d.get(Object::from_any(gname.clone()))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<ConcurrentHashMap<Object, Object>>() { _d.get(Object::from_any(gname.clone()))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<ImmutableCollections_AbstractImmutableMap<Object, Object>>() { _d.get(Object::from_any(gname.clone()))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<TreeMap<Object, Object>>() { _d.get(Object::from_any(gname.clone()))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<LinkedHashMap<Object, Object>>() { _d.get(Object::from_any(gname.clone()))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<Hashtable<Object, Object>>() { _d.get(Object::from_any(gname.clone()))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(gname.clone()))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<AbstractMap<Object, Object>>() { _d.get(Object::from_any(gname.clone()))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<HashMap<Object, Object>>() { _d.get(Object::from_any(gname.clone()))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(gname.clone()))? } else if let Some(__f) = _t6.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(gname.clone()))? } else { Default::default() };
                            let mut number = (_vdispatch7).downcast::<i32>();
                            if _is_jnull(&number) {
                                let _t8 = StringBuilder::new()?.append_str(Clone::clone(&String::from("No group with name {")))?;
                                let _t9 = _t8.append_str(Clone::clone(&gname))?;
                                let _t10 = _t9.append_str(Clone::clone(&String::from("}")))?;
                                let _t11 = _t10.toString()?;
                                return Err(JvmError::Custom("athrow".to_owned()));
                            }
                            refNum = number;
                            cursor = cursor.wrapping_add(1i32);
                        } else {
                            refNum = ((nextChar as i32)).wrapping_sub(48i32);
                            if refNum > 9i32 {
                                return Err(JvmError::Custom("athrow".to_owned()));
                            }
                            cursor = cursor.wrapping_add(1i32);
                            let mut begin: i32 = 0i32;
                            loop {
                                if (begin!=0) { break; }
                                let _t3 = replacement.length()?;
                                if cursor >= _t3 {
                                    break;
                                }
                                let _t4 = replacement.charAt(cursor)?;
                                let mut gname = ((_t4 as i32)).wrapping_sub(48i32);
                                if gname > 9i32 {
                                    break;
                                }
                                let mut number = ((refNum).wrapping_mul(10i32)).wrapping_add(gname);
                                let _t5 = this.groupCount()?;
                                if _t5 < number {
                                    begin = 1i32;
                                } else {
                                    refNum = number;
                                    cursor = cursor.wrapping_add(1i32);
                                }
                            }
                        }
                        let _t3 = this.start_i(refNum)?;
                        let _t4 = this.end_i(refNum)?;
                        if _t4 != -1i32 {
                            let _t5 = this.start_i(refNum)?;
                            let _t6 = this.end_i(refNum)?;
                            let _vdispatch7: Object = if let Some(_d) = app.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_seq_i_i(Clone::clone(&this.__get_text()), _t5, _t6)? } else if let Some(_d) = app.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(Clone::clone(&this.__get_text()), _t5, _t6)? } else if let Some(_d) = app.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_seq_i_i(Clone::clone(&this.__get_text()), _t5, _t6)? } else if let Some(_d) = app.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(Clone::clone(&this.__get_text()), _t5, _t6)? } else if let Some(_d) = app.0.as_any().downcast_ref::<CharBuffer>() { _d.append_seq_i_i(Clone::clone(&this.__get_text()), _t5, _t6)? } else if let Some(_d) = app.0.as_any().downcast_ref::<Writer>() { _d.append_seq_i_i(Clone::clone(&this.__get_text()), _t5, _t6)? } else if let Some(_d) = app.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_seq_i_i(Clone::clone(&this.__get_text()), _t5, _t6)? } else if let Some(_d) = app.0.as_any().downcast_ref::<StringBuilder>() { _d.append_seq_i_i(Clone::clone(&this.__get_text()), _t5, _t6)? } else if let Some(_d) = app.0.as_any().downcast_ref::<PrintStream>() { _d.append_seq_i_i(Clone::clone(&this.__get_text()), _t5, _t6)? } else if let Some(_d) = app.0.as_any().downcast_ref::<Object>() { _d.append(Clone::clone(&this.__get_text()), _t5, _t6)? } else if let Some(__f) = app.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, i32, i32) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&this.__get_text()), _t5, _t6)? } else { Default::default() };
                        }
                    } else {
                        let _vdispatch1: Object = if let Some(_d) = app.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<Writer>() { _d.append_c(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(nextChar)? } else if let Some(_d) = app.0.as_any().downcast_ref::<Object>() { _d.append(nextChar)? } else if let Some(__f) = app.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(nextChar)? } else { Default::default() };
                        cursor = cursor.wrapping_add(1i32);
                    }
                }
            }
            Ok(())
        }

        #[java_method(name = "appendTail", descriptor = "(Ljava/lang/StringBuffer;)Ljava/lang/StringBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn appendTail_string(&self, sb: Object) -> Result<Object> {
            panic!("stub: java/util/regex/Matcher.appendTail:(Ljava/lang/StringBuffer;)Ljava/lang/StringBuffer;")
        }

        #[java_method(name = "appendTail", descriptor = "(Ljava/lang/StringBuilder;)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: appendTail(Ljava/lang/StringBuilder;)Ljava/lang/StringBuilder;
        pub fn appendTail_sb(&self, mut sb: StringBuilder) -> Result<StringBuilder> {
            let this = self;
            let _t0 = this.getTextLength()?;
            let _t1 = sb.append_seq_i_i(Clone::clone(&this.__get_text()), this.__get_lastAppendPosition(), _t0)?;
            Ok(sb)
        }

        #[java_method(name = "replaceAll", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: replaceAll(Ljava/lang/String;)Ljava/lang/String;
        pub fn replaceAll_str(&self, mut replacement: String) -> Result<String> {
            let this = self;
            let _t0 = this.reset()?;
            let _t1 = this.find()?;
            let mut result = (_t1) as i32;
            let mut sb = StringBuilder::new()?;
            loop {
                let _t2 = this.appendReplacement_sb_str(Clone::clone(&sb), Clone::clone(&replacement))?;
                let _t3 = this.find()?;
                result = (_t3) as i32;
                if (result==0) { break; }
            }
            let _t2 = this.appendTail_sb(Clone::clone(&sb))?;
            let _t3 = sb.toString()?;
            return Ok(_t3);
            let _vdispatch4: String = if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<HeapCharBuffer>() { _d.toString()? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<CharBuffer>() { _d.toString()? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.toString()? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<String>() { _d.toString()? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<StringBuilder>() { _d.toString()? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = this.__get_text().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            Ok(_vdispatch4)
        }

        #[java_method(name = "replaceAll", descriptor = "(Ljava/util/function/Function;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Function<Ljava/util/regex/MatchResult;Ljava/lang/String;>;)Ljava/lang/String;")]
        pub fn replaceAll_functi(&self, replacer: Object) -> Result<String> {
            panic!("stub: java/util/regex/Matcher.replaceAll:(Ljava/util/function/Function;)Ljava/lang/String;")
        }

        #[java_method(name = "results", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<Ljava/util/regex/MatchResult;>;")]
        pub fn results(&self) -> Result<Object> {
            panic!("stub: java/util/regex/Matcher.results:()Ljava/util/stream/Stream;")
        }

        #[java_method(name = "replaceFirst", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: replaceFirst(Ljava/lang/String;)Ljava/lang/String;
        pub fn replaceFirst_str(&self, mut replacement: String) -> Result<String> {
            let this = self;
            if _is_jnull(&replacement) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0 = this.reset()?;
            let _t1 = this.find()?;
            if !(_t1) {
                let _vdispatch2: String = if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<HeapCharBuffer>() { _d.toString()? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<CharBuffer>() { _d.toString()? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.toString()? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<String>() { _d.toString()? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<StringBuilder>() { _d.toString()? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = this.__get_text().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
                return Ok(_vdispatch2);
            }
            let mut sb = StringBuilder::new()?;
            let _t2 = this.appendReplacement_sb_str(Clone::clone(&sb), Clone::clone(&replacement))?;
            let _t3 = this.appendTail_sb(Clone::clone(&sb))?;
            let _t4 = sb.toString()?;
            Ok(_t4)
        }

        #[java_method(name = "replaceFirst", descriptor = "(Ljava/util/function/Function;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Function<Ljava/util/regex/MatchResult;Ljava/lang/String;>;)Ljava/lang/String;")]
        pub fn replaceFirst_functi(&self, replacer: Object) -> Result<String> {
            panic!("stub: java/util/regex/Matcher.replaceFirst:(Ljava/util/function/Function;)Ljava/lang/String;")
        }

        #[java_method(name = "region", descriptor = "(II)Ljava/util/regex/Matcher;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn region(&self, start: i32, end: i32) -> Result<Matcher> {
            panic!("stub: java/util/regex/Matcher.region:(II)Ljava/util/regex/Matcher;")
        }

        #[java_method(name = "regionStart", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn regionStart(&self) -> Result<i32> {
            panic!("stub: java/util/regex/Matcher.regionStart:()I")
        }

        #[java_method(name = "regionEnd", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn regionEnd(&self) -> Result<i32> {
            panic!("stub: java/util/regex/Matcher.regionEnd:()I")
        }

        #[java_method(name = "hasTransparentBounds", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasTransparentBounds(&self) -> Result<bool> {
            panic!("stub: java/util/regex/Matcher.hasTransparentBounds:()Z")
        }

        #[java_method(name = "useTransparentBounds", descriptor = "(Z)Ljava/util/regex/Matcher;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn useTransparentBounds(&self, b: bool) -> Result<Matcher> {
            panic!("stub: java/util/regex/Matcher.useTransparentBounds:(Z)Ljava/util/regex/Matcher;")
        }

        #[java_method(name = "hasAnchoringBounds", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasAnchoringBounds(&self) -> Result<bool> {
            panic!("stub: java/util/regex/Matcher.hasAnchoringBounds:()Z")
        }

        #[java_method(name = "useAnchoringBounds", descriptor = "(Z)Ljava/util/regex/Matcher;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn useAnchoringBounds(&self, b: bool) -> Result<Matcher> {
            panic!("stub: java/util/regex/Matcher.useAnchoringBounds:(Z)Ljava/util/regex/Matcher;")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "hitEnd", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hitEnd(&self) -> Result<bool> {
            panic!("stub: java/util/regex/Matcher.hitEnd:()Z")
        }

        #[java_method(name = "requireEnd", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn requireEnd(&self) -> Result<bool> {
            panic!("stub: java/util/regex/Matcher.requireEnd:()Z")
        }

        #[java_method(name = "search", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn search(&self, mut from: i32) -> Result<bool> {
            let this = self;
            this.__set_hitEnd((0i32 != 0i32));
            this.__set_requireEnd((0i32 != 0i32));
            from = (if (from<0) { 0i32 } else { from });
            this.__set_first(from);
            this.__set_oldLast((if (this.__get_oldLast()<0) { from } else { this.__get_oldLast() }));
            let mut i: i32 = 0i32;
            loop {
                if i >= (this.__get_groups().borrow().len() as i32) { break; }
                this.__get_groups().borrow_mut()[i as usize] = -1i32;
                i = i.wrapping_add(1i32);
            }
            i = 0i32;
            loop {
                if i >= (this.__get_localsPos().borrow().len() as i32) { break; }
                if !_is_jnull(&Clone::clone(&this.__get_localsPos().borrow()[i as usize])) {
                    Clone::clone(&this.__get_localsPos().borrow()[i as usize]).clear()?;
                }
                i = i.wrapping_add(1i32);
            }
            this.__set_acceptMode(0i32);
            let _t0 = this.__get_parentPattern().__get_root().match_(Clone::clone(this), from, Clone::clone(&this.__get_text()))?;
            i = (_t0) as i32;
            if (i==0) {
                this.__set_first(-1i32);
            }
            this.__set_oldLast(this.__get_last());
            this.__set_modCount((this.__get_modCount()).wrapping_add(1i32));
            Ok((i != 0i32))
        }

        #[java_method(name = "match", descriptor = "(II)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn match_(&self, mut from: i32, mut anchor: i32) -> Result<bool> {
            let this = self;
            this.__set_hitEnd((0i32 != 0i32));
            this.__set_requireEnd((0i32 != 0i32));
            from = (if (from<0) { 0i32 } else { from });
            this.__set_first(from);
            this.__set_oldLast((if (this.__get_oldLast()<0) { from } else { this.__get_oldLast() }));
            let mut i: i32 = 0i32;
            loop {
                if i >= (this.__get_groups().borrow().len() as i32) { break; }
                this.__get_groups().borrow_mut()[i as usize] = -1i32;
                i = i.wrapping_add(1i32);
            }
            i = 0i32;
            loop {
                if i >= (this.__get_localsPos().borrow().len() as i32) { break; }
                if !_is_jnull(&Clone::clone(&this.__get_localsPos().borrow()[i as usize])) {
                    Clone::clone(&this.__get_localsPos().borrow()[i as usize]).clear()?;
                }
                i = i.wrapping_add(1i32);
            }
            this.__set_acceptMode(anchor);
            let _t0 = this.__get_parentPattern().__get_matchRoot().match_(Clone::clone(this), from, Clone::clone(&this.__get_text()))?;
            i = (_t0) as i32;
            if (i==0) {
                this.__set_first(-1i32);
            }
            this.__set_oldLast(this.__get_last());
            this.__set_modCount((this.__get_modCount()).wrapping_add(1i32));
            Ok((i != 0i32))
        }

        #[java_method(name = "getTextLength", descriptor = "()I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTextLength(&self) -> Result<i32> {
            let this = self;
            let _vdispatch0: i32 = if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<HeapCharBuffer>() { _d.length()? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<CharBuffer>() { _d.length()? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.length()? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<String>() { _d.length()? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<StringBuilder>() { _d.length()? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<Object>() { _d.length()? } else if let Some(__f) = this.__get_text().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            Ok(_vdispatch0)
        }

        #[java_method(name = "getSubSequence", descriptor = "(II)Ljava/lang/CharSequence;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSubSequence(&self, mut beginIndex: i32, mut endIndex: i32) -> Result<Object> {
            let this = self;
            let _vdispatch0: Object = if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<HeapCharBuffer>() { _d.subSequence(beginIndex, endIndex)? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<CharBuffer>() { _d.subSequence(beginIndex, endIndex)? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.subSequence(beginIndex, endIndex)? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<String>() { _d.subSequence(beginIndex, endIndex)? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<StringBuilder>() { _d.subSequence(beginIndex, endIndex)? } else if let Some(_d) = this.__get_text().0.as_any().downcast_ref::<Object>() { _d.subSequence(beginIndex, endIndex)? } else if let Some(__f) = this.__get_text().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32, i32) -> crate::error::Result<Object>>>() { (__f)(beginIndex, endIndex)? } else { Default::default() };
            Ok(_vdispatch0)
        }

        #[java_method(name = "charAt", descriptor = "(I)C", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn charAt(&self, i: i32) -> Result<u16> {
            panic!("stub: java/util/regex/Matcher.charAt:(I)C")
        }

        #[java_method(name = "getMatchedGroupIndex", descriptor = "(Ljava/lang/String;)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMatchedGroupIndex(&self, name: String) -> Result<i32> {
            panic!("stub: java/util/regex/Matcher.getMatchedGroupIndex:(Ljava/lang/String;)I")
        }

        #[java_method(name = "checkGroup", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkGroup(&self, mut group: i32) -> Result<()> {
            let this = self;
            let _t0 = this.groupCount()?;
            if group > _t0 {
                let _t1 = StringBuilder::new()?.append_str(Clone::clone(&String::from("No group ")))?;
                let _t2 = _t1.append_i(group)?;
                let _t3 = _t2.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "checkMatch", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkMatch(&self) -> Result<()> {
            let this = self;
            let _t0 = this.hasMatch()?;
            if !(_t0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "namedGroups", descriptor = "()Ljava/util/Map;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Map<Ljava/lang/String;Ljava/lang/Integer;>;")]
        pub fn namedGroups(&self) -> Result<Object> {
            let this = self;
            if _is_jnull(&this.__get_namedGroups()) {
                let _t0 = this.__get_parentPattern().namedGroups()?;
                this.__set_namedGroups(Clone::clone(&_t0));
                return Ok(_t0);
            }
            Ok(this.__get_namedGroups())
        }

        #[java_method(name = "hasMatch", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasMatch(&self) -> Result<bool> {
            let this = self;
            Ok((this.__get_first()>=0))
        }
    }
}
