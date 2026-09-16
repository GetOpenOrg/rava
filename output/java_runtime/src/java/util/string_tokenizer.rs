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
    #[binary_name       = "java/util/StringTokenizer"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/Enumeration"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/lang/Object;Ljava/util/Enumeration<Ljava/lang/Object;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "StringTokenizer.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Enumeration;java/util/StringTokenizer"]

    pub struct StringTokenizer {
        #[cfg_attr(any(), java_field(name = "currentPosition", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub currentPosition: i32,
        #[cfg_attr(any(), java_field(name = "newPosition", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub newPosition: i32,
        #[cfg_attr(any(), java_field(name = "maxPosition", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub maxPosition: i32,
        #[cfg_attr(any(), java_field(name = "str", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub str: String,
        #[cfg_attr(any(), java_field(name = "delimiters", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub delimiters: String,
        #[cfg_attr(any(), java_field(name = "retDelims", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub retDelims: bool,
        #[cfg_attr(any(), java_field(name = "delimsChanged", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub delimsChanged: bool,
        #[cfg_attr(any(), java_field(name = "maxDelimCodePoint", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub maxDelimCodePoint: i32,
        #[cfg_attr(any(), java_field(name = "hasSurrogates", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub hasSurrogates: bool,
        #[cfg_attr(any(), java_field(name = "delimiterCodePoints", descriptor = "[I", access = "private", modifiers = "", is_static = false))]
        pub delimiterCodePoints: Rc<RefCell<Vec<i32>>>,
    }

    impl StringTokenizer {
        #[java_method(name = "setMaxDelimCodePoint", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMaxDelimCodePoint(&self) -> Result<()> {
            let this = self;
            if _is_jnull(&this.__get_delimiters()) {
                this.__set_maxDelimCodePoint(0i32);
                return Ok(());
            }
            let mut m: i32 = 0i32;
            let mut count: i32 = 0i32;
            let mut i: i32 = 0i32;
            let mut c: u16 = Default::default();
            loop {
                let _t0 = this.__get_delimiters().length()?;
                if i >= _t0 { break; }
                let _t0 = this.__get_delimiters().charAt(i)?;
                c = _t0;
                if (c as i32) <= 57343i32 {
                    let _t1 = this.__get_delimiters().codePointAt(i)?;
                    let mut c: i32 = _t1;
                    this.__set_hasSurrogates((1i32 != 0i32));
                }
                if m < c {
                    m = c;
                }
                count = count.wrapping_add(1i32);
                let _t1: i32 = Character::charCount(c)?;
                i = (i).wrapping_add(_t1);
            }
            this.__set_maxDelimCodePoint(m);
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; count as usize]));
            this.__set_delimiterCodePoints(Clone::clone(&_arr0));
            i = 0i32;
            let mut j: i32 = 0i32;
            loop {
                if i >= count { break; }
                let _t1 = this.__get_delimiters().codePointAt(j)?;
                let mut c = _t1;
                this.__get_delimiterCodePoints().borrow_mut()[i as usize] = c;
                i = i.wrapping_add(1i32);
                let _t2: i32 = Character::charCount(c)?;
                j = (j).wrapping_add(_t2);
            }
            Ok(())
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/String;Ljava/lang/String;Z)V
        pub fn new_str_str_z(mut str: String, mut delim: String, mut returnDelims: bool) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_hasSurrogates((0i32 != 0i32));
            this.__set_currentPosition(0i32);
            this.__set_newPosition(-1i32);
            this.__set_delimsChanged((0i32 != 0i32));
            this.__set_str(Clone::clone(&str));
            let _t0 = str.length()?;
            this.__set_maxPosition(_t0);
            this.__set_delimiters(Clone::clone(&delim));
            this.__set_retDelims(returnDelims);
            this.setMaxDelimCodePoint()?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/String;Ljava/lang/String;)V
        pub fn new_str_str(mut str: String, mut delim: String) -> Result<Self> {
            let mut this = Self::default();
            this = StringTokenizer::new_str_str_z(Clone::clone(&str), Clone::clone(&delim), (0i32 != 0i32))?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str(str: String) -> Result<Self> {
            panic!("stub: java/util/StringTokenizer.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "skipDelimiters", descriptor = "(I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn skipDelimiters(&self, mut startPos: i32) -> Result<i32> {
            let this = self;
            if _is_jnull(&this.__get_delimiters()) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut position: i32 = startPos;
            loop {
                if this.__get_retDelims() { break; }
                let _t0 = this.__get_str().charAt(position)?;
                let mut c: u16 = _t0;
                let _t1 = this.__get_delimiters().indexOf_i((c as i32))?;
                if (_t1<0) {
                    break;
                }
                position = position.wrapping_add(1i32);
                continue;
                let _t2 = this.__get_str().codePointAt(position)?;
                let mut c: i32 = _t2;
                let _t3 = this.isDelimiter(c)?;
                if !(_t3) {
                    break;
                }
                let _t4: i32 = Character::charCount(c)?;
                position = (position).wrapping_add(_t4);
            }
            Ok(position)
        }

        #[java_method(name = "scanToken", descriptor = "(I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn scanToken(&self, mut startPos: i32) -> Result<i32> {
            let this = self;
            let mut position: i32 = startPos;
            let mut c: u16 = Default::default();
            loop {
                if position >= this.__get_maxPosition() { break; }
                let _t0 = this.__get_str().charAt(position)?;
                c = _t0;
                if (c as i32) <= this.__get_maxDelimCodePoint() {
                    let _t1 = this.__get_delimiters().indexOf_i((c as i32))?;
                    if (_t1>=0) {
                        break;
                    }
                } else {
                    position = position.wrapping_add(1i32);
                    continue;
                    let _t1 = this.__get_str().codePointAt(position)?;
                    let mut c: i32 = _t1;
                    if c <= this.__get_maxDelimCodePoint() {
                        let _t2 = this.isDelimiter(c)?;
                        if _t2 {
                            break;
                        }
                    } else {
                        let _t2: i32 = Character::charCount(c)?;
                        position = (position).wrapping_add(_t2);
                        continue;
                    }
                }
            }
            if !(this.__get_hasSurrogates()) {
                let _t0 = this.__get_str().charAt(position)?;
                let mut c = _t0;
                let _t1 = this.__get_delimiters().indexOf_i((c as i32))?;
                if (_t1>=0) {
                    position = position.wrapping_add(1i32);
                }
            } else {
                let _t0 = this.__get_str().codePointAt(position)?;
                let mut c: i32 = _t0;
                let _t1 = this.isDelimiter(c)?;
                if _t1 {
                    let _t2: i32 = Character::charCount(c)?;
                    position = (position).wrapping_add(_t2);
                }
            }
            Ok(position)
        }

        #[java_method(name = "isDelimiter", descriptor = "(I)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDelimiter(&self, mut codePoint: i32) -> Result<bool> {
            let this = self;
            let mut local_2 = this.__get_delimiterCodePoints();
            let mut local_3 = (local_2.borrow().len() as i32);
            let mut local_4: i32 = 0i32;
            loop {
                if local_4 >= local_3 { break; }
                let mut delimiterCodePoint = local_2.borrow()[local_4 as usize];
                if delimiterCodePoint == codePoint {
                    return Ok((1i32 != 0i32));
                }
                local_4 = local_4.wrapping_add(1i32);
            }
            Ok((0i32 != 0i32))
        }

        #[java_method(name = "hasMoreTokens", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasMoreTokens(&self) -> Result<bool> {
            let this = self;
            let _t0 = this.skipDelimiters(this.__get_currentPosition())?;
            this.__set_newPosition(_t0);
            Ok(this.__get_newPosition() < this.__get_maxPosition())
        }

        #[java_method(name = "nextToken", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: nextToken()Ljava/lang/String;
        pub fn nextToken(&self) -> Result<String> {
            let this = self;
            let mut _merged2: i32;
            if (this.__get_newPosition()>=0) {
                let mut _merged1: i32;
                if !(this.__get_delimsChanged()) {
                    _merged1 = this.__get_newPosition();
                } else {
                    let _t0 = this.skipDelimiters(this.__get_currentPosition())?;
                    _merged1 = _t0;
                }
                _merged2 = _merged1;
            } else {
                let _t0 = this.skipDelimiters(this.__get_currentPosition())?;
                _merged2 = _t0;
            }
            this.__set_currentPosition(_merged2);
            this.__set_delimsChanged((0i32 != 0i32));
            this.__set_newPosition(-1i32);
            if this.__get_currentPosition() >= this.__get_maxPosition() {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut start = this.__get_currentPosition();
            let _t3 = this.scanToken(this.__get_currentPosition())?;
            this.__set_currentPosition(_t3);
            let _t4 = this.__get_str().substring_i_i(start, this.__get_currentPosition())?;
            Ok(_t4)
        }

        #[java_method(name = "nextToken", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextToken_str(&self, delim: String) -> Result<String> {
            panic!("stub: java/util/StringTokenizer.nextToken:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "hasMoreElements", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasMoreElements(&self) -> Result<bool> {
            let this = self;
            let _t0 = this.hasMoreTokens()?;
            Ok(_t0)
        }

        #[java_method(name = "nextElement", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextElement(&self) -> Result<Object> {
            panic!("stub: java/util/StringTokenizer.nextElement:()Ljava/lang/Object;")
        }

        #[java_method(name = "countTokens", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn countTokens(&self) -> Result<i32> {
            let this = self;
            let mut count: i32 = 0i32;
            let mut currpos = this.__get_currentPosition();
            loop {
                if currpos >= this.__get_maxPosition() { break; }
                let _t0 = this.skipDelimiters(currpos)?;
                currpos = _t0;
                if currpos >= this.__get_maxPosition() {
                    break;
                }
                let _t1 = this.scanToken(currpos)?;
                currpos = _t1;
                count = count.wrapping_add(1i32);
            }
            Ok(count)
        }
    }
}
