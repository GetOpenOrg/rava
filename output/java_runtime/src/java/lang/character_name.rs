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
use crate::jdk::internal::util::ArraysSupport;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/CharacterName"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "CharacterName.java"]
    #[inner_classes     = "java/lang/CharacterName$1:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/CharacterName;java/lang/Object"]

    pub struct CharacterName {
        #[cfg_attr(any(), java_field(name = "strPool", descriptor = "[B", access = "private", modifiers = "final", is_static = false))]
        pub strPool: Rc<RefCell<Vec<i8>>>,
        #[cfg_attr(any(), java_field(name = "lookup", descriptor = "[I", access = "private", modifiers = "final", is_static = false))]
        pub lookup: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "bkIndices", descriptor = "[I", access = "private", modifiers = "final", is_static = false))]
        pub bkIndices: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "cpEntries", descriptor = "[I", access = "private", modifiers = "final", is_static = false))]
        pub cpEntries: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "hsIndices", descriptor = "[I", access = "private", modifiers = "final", is_static = false))]
        pub hsIndices: Rc<RefCell<Vec<i32>>>,
    }

    impl CharacterName {
        #[cfg_attr(any(), java_field(name = "refCharName", descriptor = "Ljava/lang/ref/SoftReference;", access = "private", modifiers = "static", is_static = true, generic_signature = "Ljava/lang/ref/SoftReference<Ljava/lang/CharacterName;>;"))]
        // static field: refCharName:Ljava/lang/ref/SoftReference;
        pub fn refCharName() -> SoftReference<CharacterName> {
            panic!("stub: java/lang/CharacterName.refCharName:Ljava/lang/ref/SoftReference;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            let _t0: Object = AccessController::doPrivileged_privil(Clone::clone(&CharacterName_1::new(Clone::clone(this))?).into())?;
            let mut dis = DataInputStream::new(Clone::clone(&InflaterInputStream::new_inputs(Clone::clone(&(_t0).downcast::<InputStream>()))?).into())?;
            let _t1 = dis.readInt()?;
            let mut total: i32 = _t1;
            let _t2 = dis.readInt()?;
            let mut bkNum: i32 = _t2;
            let _t3 = dis.readInt()?;
            let mut cpNum: i32 = _t3;
            let _t4 = dis.readInt()?;
            let mut cpEnd: i32 = _t4;
            let mut _arr5: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; cpEnd as usize]));
            let mut ba: Rc<RefCell<Vec<i8>>> = _arr5;
            let mut _arr6: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (bkNum).wrapping_mul(256i32) as usize]));
            this.__set_lookup(Clone::clone(&_arr6));
            let mut _arr7: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 4352i32 as usize]));
            this.__set_bkIndices(Clone::clone(&_arr7));
            let mut _arr8: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; (total).wrapping_sub(cpEnd) as usize]));
            this.__set_strPool(Clone::clone(&_arr8));
            let mut _arr9: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (cpNum).wrapping_mul(3i32) as usize]));
            this.__set_cpEntries(Clone::clone(&_arr9));
            let mut _arr10: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; ((cpNum/2i32)|1i32) as usize]));
            this.__set_hsIndices(Clone::clone(&_arr10));
            Arrays::fill_arr_i_i(Clone::clone(&this.__get_bkIndices()), -1i32)?;
            Arrays::fill_arr_i_i(Clone::clone(&this.__get_hsIndices()), -1i32)?;
            dis.readFully_arr_b(Clone::clone(&ba))?;
            dis.readFully_arr_b(Clone::clone(&this.__get_strPool()))?;
            let mut nameOff: i32 = 0i32;
            let mut cpOff: i32 = 0i32;
            let mut cp: i32 = 0i32;
            let mut bk: i32 = -1i32;
            let mut prevBk: i32 = -1i32;
            let mut idx: i32 = 0i32;
            loop {
                cpOff = cpOff.wrapping_add(1i32);
                let mut len = ((ba.borrow()[cpOff as usize] as i32)&255i32);
                if (len==0) {
                    cpOff = cpOff.wrapping_add(1i32);
                    len = ((ba.borrow()[cpOff as usize] as i32)&255i32);
                    cpOff = cpOff.wrapping_add(1i32);
                    cpOff = cpOff.wrapping_add(1i32);
                    cpOff = cpOff.wrapping_add(1i32);
                    cp = (((((ba.borrow()[cpOff as usize] as i32)&255i32)<<(16i32&0x1f))|(((ba.borrow()[cpOff as usize] as i32)&255i32)<<(8i32&0x1f)))|((ba.borrow()[cpOff as usize] as i32)&255i32));
                } else {
                    cp = cp.wrapping_add(1i32);
                }
                let mut hi = (cp>>((8i32&0x1f)));
                if prevBk != hi {
                    bk = bk.wrapping_add(1i32);
                    this.__get_bkIndices().borrow_mut()[hi as usize] = bk;
                    prevBk = hi;
                }
                this.__get_lookup().borrow_mut()[((bk<<(8i32&0x1f))).wrapping_add((cp&255i32)) as usize] = ((nameOff<<(8i32&0x1f))|len);
                let _t11: i32 = CharacterName::hashN(Clone::clone(&this.__get_strPool()), nameOff, len)?;
                let mut hash: i32 = _t11;
                let mut hsh = ((hash&2147483647i32)%(this.__get_hsIndices().borrow().len() as i32));
                let mut next = this.__get_hsIndices().borrow()[hsh as usize];
                this.__get_hsIndices().borrow_mut()[hsh as usize] = idx;
                let _t12 = this.addCp(idx, hash, next, cp)?;
                idx = _t12;
                nameOff = (nameOff).wrapping_add(len);
                if cpOff >= cpEnd { break; }
            }
            dis.__super().close()?;
            Ok(this)
        }

        #[java_method(name = "hashN", descriptor = "([BII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashN(mut a: Rc<RefCell<Vec<i8>>>, mut off: i32, mut len: i32) -> Result<i32> {
            let _t0: i32 = ArraysSupport::vectorizedHashCode(Object::from_any(a.clone()), off, len, 1i32, 8i32)?;
            Ok(_t0)
        }

        #[java_method(name = "addCp", descriptor = "(IIII)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addCp(&self, mut idx: i32, mut hash: i32, mut next: i32, mut cp: i32) -> Result<i32> {
            let this = self;
            idx = idx.wrapping_add(1i32);
            this.__get_cpEntries().borrow_mut()[idx as usize] = hash;
            idx = idx.wrapping_add(1i32);
            this.__get_cpEntries().borrow_mut()[idx as usize] = next;
            idx = idx.wrapping_add(1i32);
            this.__get_cpEntries().borrow_mut()[idx as usize] = cp;
            Ok(idx)
        }

        #[java_method(name = "getCpHash", descriptor = "(I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCpHash(&self, mut idx: i32) -> Result<i32> {
            let this = self;
            Ok(this.__get_cpEntries().borrow()[idx as usize])
        }

        #[java_method(name = "getCpNext", descriptor = "(I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCpNext(&self, mut idx: i32) -> Result<i32> {
            let this = self;
            Ok(this.__get_cpEntries().borrow()[(idx).wrapping_add(1i32) as usize])
        }

        #[java_method(name = "getCp", descriptor = "(I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCp(&self, mut idx: i32) -> Result<i32> {
            let this = self;
            Ok(this.__get_cpEntries().borrow()[(idx).wrapping_add(2i32) as usize])
        }

        #[java_method(name = "getInstance", descriptor = "()Ljava/lang/CharacterName;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInstance() -> Result<CharacterName> {
            let mut ref_: SoftReference<Object> = CharacterName::refCharName();
            let _t0 = ref_.get()?;
            let mut cname = (_t0).downcast::<CharacterName>();
            if _is_jnull(&(_t0).downcast::<CharacterName>()) {
                cname = CharacterName::new()?;
                CharacterName::set_refCharName(SoftReference::<Object>::new_obj(Object::from_any(cname.clone()))?);
            }
            Ok(cname)
        }

        #[java_method(name = "getName", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getName(&self, mut cp: i32) -> Result<String> {
            let this = self;
            let mut bk = this.__get_bkIndices().borrow()[(cp>>((8i32&0x1f))) as usize];
            let mut off = this.__get_lookup().borrow()[((bk<<(8i32&0x1f))).wrapping_add((cp&255i32)) as usize];
            if (this.__get_lookup().borrow()[((bk<<(8i32&0x1f))).wrapping_add((cp&255i32)) as usize]==0) {
                return Ok(Default::default());
            }
            let mut result = String::new_arr_b_i_i_i(Clone::clone(&this.__get_strPool()), 0i32, ((off as u32>>(8i32&0x1f)) as i32), (off&255i32))?;
            Ok(result)
        }

        #[java_method(name = "getCodePoint", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCodePoint(&self, mut name: String) -> Result<i32> {
            let this = self;
            let _t0 = name.getBytes_charse(Clone::clone(&ISO_8859_1::INSTANCE()).into())?;
            let mut bname: Rc<RefCell<Vec<i8>>> = _t0;
            let _t1: i32 = CharacterName::hashN(Clone::clone(&bname), 0i32, (bname.borrow().len() as i32))?;
            let mut hsh: i32 = _t1;
            let mut idx = this.__get_hsIndices().borrow()[((hsh&2147483647i32)%(this.__get_hsIndices().borrow().len() as i32)) as usize];
            loop {
                if idx == -1i32 { break; }
                let _t2 = this.getCpHash(idx)?;
                let _t3 = this.getCp(idx)?;
                let mut cp: i32 = _t3;
                let mut bk = this.__get_bkIndices().borrow()[(cp>>((8i32&0x1f))) as usize];
                let mut off = this.__get_lookup().borrow()[((bk<<(8i32&0x1f))).wrapping_add((cp&255i32)) as usize];
                let mut len = (off&255i32);
                off = ((off as u32>>(8i32&0x1f)) as i32);
                let mut i: i32 = 0i32;
                loop {
                    if i >= len { break; }
                    off = off.wrapping_add(1i32);
                    if (bname.borrow()[i as usize] as i32) == (this.__get_strPool().borrow()[off as usize] as i32) {
                        i = i.wrapping_add(1i32);
                        continue;
                    }
                    break;
                }
                if i == len {
                    return Ok(cp);
                }
                let _t4 = this.getCpNext(idx)?;
                idx = _t4;
            }
            Ok(-1i32)
        }
    }
}
