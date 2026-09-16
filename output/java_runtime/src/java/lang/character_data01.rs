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

impl From<CharacterData01> for CharacterData {
    fn from(v: CharacterData01) -> CharacterData { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/CharacterData01"]
    #[super_class       = "java/lang/CharacterData"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "CharacterData01.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "CharacterData"]
    #[all_supertypes    = "java/lang/CharacterData;java/lang/CharacterData01;java/lang/Object"]

    pub struct CharacterData01;

    impl CharacterData01 {
        #[cfg_attr(any(), java_field(name = "instance", descriptor = "Ljava/lang/CharacterData;", access = "package", modifiers = "static final", is_static = true))]
        // static field: instance:Ljava/lang/CharacterData;
        pub fn instance() -> CharacterData {
            panic!("stub: java/lang/CharacterData01.instance:Ljava/lang/CharacterData;")
        }

        #[cfg_attr(any(), java_field(name = "X", descriptor = "[C", access = "package", modifiers = "static final", is_static = true))]
        // static field: X:[C
        pub fn X() -> Rc<RefCell<Vec<u16>>> {
            panic!("stub: java/lang/CharacterData01.X:[C")
        }

        #[cfg_attr(any(), java_field(name = "Y", descriptor = "[C", access = "package", modifiers = "static final", is_static = true))]
        // static field: Y:[C
        pub fn Y() -> Rc<RefCell<Vec<u16>>> {
            panic!("stub: java/lang/CharacterData01.Y:[C")
        }

        #[cfg_attr(any(), java_field(name = "A", descriptor = "[I", access = "package", modifiers = "static final", is_static = true))]
        // static field: A:[I
        pub fn A() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/lang/CharacterData01.A:[I")
        }

        #[cfg_attr(any(), java_field(name = "A_DATA", descriptor = "Ljava/lang/String;", access = "package", modifiers = "static final", is_static = true, constant_value = "��瀅��瀅砀����瀅��瀅砀��砀��砀����\u{0018}栀\u{0018}��\u{0018}砀��砀����݋��݋��݋��݋��ѫ��֋��ࠋ��ࠋ��ࠋ砀����\u{001c}��\u{001c}��\u{001c}栀砊栀砊栀矪栀瑊栀瞪栀琪栀砊栀益栀睊栀砊栀砊栀癪栀甪栀甊栀瓪栀瓪栀瓊栀璪栀璊栀瓊栀畊栀甪栀甊栀瓪栀瓊栀眪栀砊栀癊栀砊栀ࠋ栀ࠋ栀ࠋ栀ࠋ栀\u{001c}栀\u{001c}栀\u{001c}栀ۋ栀ࠋ栀\u{001c}��\u{001c}��\u{001c}砀��栀\u{001c}砀����\u{001c}䀀〆䀀〆᠀Ћ᠀Ћ᠀Ћ᠀Ћ᠀ԫ᠀ً᠀ࠋ᠀ࠋ᠀ࠋ��Ы��ҋ��ԋ��ࠋ��瀅��砊��砊砀��䀀〆䀀〆䀀〆砀��砀����\u{0018}��\u{0018}��瘊��瘊��盪��琊��砊¢瀁¢瀁¡瀂¡瀂��㐉��㐉\u{009e}瀁\u{009e}瀁\u{009e}瀁砀��砀��\u{009d}瀂\u{009d}瀂\u{009d}瀂\u{009d}瀂砀����瀄��瀄��瀄��瀄��瀄��瀄砀����瀄��瀄砀��ࠀ瀅ࠀ瀅ࠀ瀅砀��砀��ࠀ瀅砀��ࠀ\u{0018}ࠀԫࠀԫࠀԫࠀ׫ࠀ܋ࠀࠋࠀࠋࠀࠋࠀ瀅ࠀ\u{001c}ࠀ\u{001c}ࠀԋࠀԋࠀԋࠀ֋ࠀګ砀��ࠀ݋ࠀ݋ࠀ݋ࠀ݋ࠀܫࠀܫࠀޫࠀӋࠀࠋ砀��ࠀӋࠀԫࠀ֫ࠀۋࠀࠋࠀիࠀ٫ࠀދࠀࠋ砀��栀\u{0018}ࠀЫࠀЫࠀՋࠀ٫ࠀ瀅䀀〆砀��䀀〆䀀〆䀀〆䀀〆砀��砀��䀀〆ࠀӋࠀ׫ࠀࠋ砀��ࠀ\u{0018}ࠀ\u{0018}ࠀ\u{0018}砀��ࠀ瀅ࠀҋࠀࠋࠀ\u{0018}ࠀ\u{001c}ࠀ瀅ࠀ瀅䀀〆砀��ࠀۋ栀\u{0018}栀\u{0018}ࠀ׋ࠀ۫砀��ࠀ܋ࠀ܋ࠀ܋ࠀ܋ࠀޫं瀁ं瀁ं瀁砀��ँ瀂ँ瀂ँ瀂砀��ࠀӫࠀՋࠀ׋ࠀࠋက瀅က瀅　㘉　㘉　Ы　Ы　Ջ　٫　ࠋ　ࠋ　ࠋ砀��䀀〆ࠀ\u{0014}ࠀҋࠀҋࠀԋࠀثࠀ݋ࠀࠋࠀࠋࠀ瀅䀀〆က؋က܋ကЫကࠋက\u{0018}က\u{0018}က\u{0018}ࠀ瀅ࠀދࠀދࠀދࠀދࠀЫࠀՋࠀࠋ��〈䀀〆��〈��瀅䀀〆��\u{0018}��\u{0018}��\u{0018}栀׫栀׫栀܋栀Ы��㝉��㝉䀀〆��瀅��瀅䀀〆䀀〆��瀅��〈��〈䀀〆��〈��〈䀀〆��\u{0018}��တ砀����တ��㘉��㘉䀀〆䀀〆��㕉��㕉��瀅��〈��瀅䀀〆��〈��瀅��瀅��\u{0018}��\u{0018}䀀〆砀����Ћ��Ћ��Ћ��Ћ��ԫ��ً��ࠋ��ࠋ砀��䀀〆��〈䀀〆䀀〆��〈砀��砀����〈��〈��〈��\u{0018}��瀅䀀〆��\u{0018}栀\u{0018}砀����〈䀀〆��؋��ܫ��\u{0018}��\u{001c}\u{0082}瀁\u{0082}瀁\u{0081}瀂\u{0081}瀂䀀〆��〆��〆䀀〆��〈��〆��ӫ��ӫ��〈��\u{0018}��ࠋ栀\u{001c}栀\u{001c}⠀怚⠀怚⠀怚⠀怚栀\u{001c}��瑊��瑊��睪��睪��睪��皪��皪��皪��皪��疊��疊��疊��瑪��瑪��瑪��矪��矪��矊��矊��矊��皪��皊��皊��皊��砊��砊��疪��疪��疪��疊��甪��甊��甊��瓪��瓊��璪��瓊��瓊��璪��璊��璊��瑪��瑪��瑊��琪��琊��眊��眊��眊��癊��癊��癊��癊��瘪��瘪��瘊��甪��甪��砊��睪��睪砀����တ��တ��瀄��瀄砀����׫��֫��֫��֫��\u{0018}䀀〆��瀄栀\u{0018}��瀄䀀〆砀����瀅��瀅��瀅砀��砀����瀄��瀄砀��䠀တ䠀တ��\u{001c}��〈��〈��〈��〈䠀တ䠀တ䀀〆䀀〆��\u{001c}��\u{001c}栀\u{001c}䀀〆栀\u{001c}��Ы��Ы��Ջ��٫��׫��׫��׫��Ջ��֫砀����瀁��瀁��瀂��瀂��瀂砀����瀁砀��砀����瀁砀����瀂��瀁��\u{0019}��瀂\u{0019}��瀁��瀂᠀㙉᠀㙉᠀㔉᠀㔉᠀㟉᠀㟉᠀㚉᠀㚉᠀㕉᠀㕉��\u{001c}��\u{0018}��瀅��瀂䀀〆��瀄��瀅��\u{001c}砀��⠀怚��瀅��瀄ࢊ瀁ࢊ瀁ࢉ瀂ࢉ瀂䀀〆ࠀ瀄ࠀ㘉ࠀ㘉砀��က؋က؋က؋က؋ကܫကыကࠋကࠋကࠋကࠋကߋကߋကߋက\u{001c}ကࠋက怚က؋က؋ကࠋကࠋ砀��砀��ကЋကЋကЋကЋကԫကًကࠋက\u{001c}က٫က٫က٫砀��က瀅က瀅砀��栀\u{0019}栀\u{0019}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}砀��砀��栀\u{001c}砀��砀��栀\u{001c}栀\u{001c}栀\u{001c}᠀Ћ᠀߫᠀߫᠀߫᠀߫栀ګ栀ڋ栀\u{001c}��\u{001c}栀\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}砀����\u{001c}��\u{001c}��\u{001c}砀��栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001b}栀\u{001b}栀\u{001b}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}砀��栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}砀��栀\u{001c}栀\u{001c}砀��᠀㘉᠀㘉"))]
        // static field: A_DATA:Ljava/lang/String;
        pub fn A_DATA() -> String {
            String::from("��瀅��瀅砀����瀅��瀅砀��砀��砀����\u{0018}栀\u{0018}��\u{0018}砀��砀����݋��݋��݋��݋��ѫ��֋��ࠋ��ࠋ��ࠋ砀����\u{001c}��\u{001c}��\u{001c}栀砊栀砊栀矪栀瑊栀瞪栀琪栀砊栀益栀睊栀砊栀砊栀癪栀甪栀甊栀瓪栀瓪栀瓊栀璪栀璊栀瓊栀畊栀甪栀甊栀瓪栀瓊栀眪栀砊栀癊栀砊栀ࠋ栀ࠋ栀ࠋ栀ࠋ栀\u{001c}栀\u{001c}栀\u{001c}栀ۋ栀ࠋ栀\u{001c}��\u{001c}��\u{001c}砀��栀\u{001c}砀����\u{001c}䀀〆䀀〆᠀Ћ᠀Ћ᠀Ћ᠀Ћ᠀ԫ᠀ً᠀ࠋ᠀ࠋ᠀ࠋ��Ы��ҋ��ԋ��ࠋ��瀅��砊��砊砀��䀀〆䀀〆䀀〆砀��砀����\u{0018}��\u{0018}��瘊��瘊��盪��琊��砊¢瀁¢瀁¡瀂¡瀂��㐉��㐉\u{009e}瀁\u{009e}瀁\u{009e}瀁砀��砀��\u{009d}瀂\u{009d}瀂\u{009d}瀂\u{009d}瀂砀����瀄��瀄��瀄��瀄��瀄��瀄砀����瀄��瀄砀��ࠀ瀅ࠀ瀅ࠀ瀅砀��砀��ࠀ瀅砀��ࠀ\u{0018}ࠀԫࠀԫࠀԫࠀ׫ࠀ܋ࠀࠋࠀࠋࠀࠋࠀ瀅ࠀ\u{001c}ࠀ\u{001c}ࠀԋࠀԋࠀԋࠀ֋ࠀګ砀��ࠀ݋ࠀ݋ࠀ݋ࠀ݋ࠀܫࠀܫࠀޫࠀӋࠀࠋ砀��ࠀӋࠀԫࠀ֫ࠀۋࠀࠋࠀիࠀ٫ࠀދࠀࠋ砀��栀\u{0018}ࠀЫࠀЫࠀՋࠀ٫ࠀ瀅䀀〆砀��䀀〆䀀〆䀀〆䀀〆砀��砀��䀀〆ࠀӋࠀ׫ࠀࠋ砀��ࠀ\u{0018}ࠀ\u{0018}ࠀ\u{0018}砀��ࠀ瀅ࠀҋࠀࠋࠀ\u{0018}ࠀ\u{001c}ࠀ瀅ࠀ瀅䀀〆砀��ࠀۋ栀\u{0018}栀\u{0018}ࠀ׋ࠀ۫砀��ࠀ܋ࠀ܋ࠀ܋ࠀ܋ࠀޫं瀁ं瀁ं瀁砀��ँ瀂ँ瀂ँ瀂砀��ࠀӫࠀՋࠀ׋ࠀࠋက瀅က瀅　㘉　㘉　Ы　Ы　Ջ　٫　ࠋ　ࠋ　ࠋ砀��䀀〆ࠀ\u{0014}ࠀҋࠀҋࠀԋࠀثࠀ݋ࠀࠋࠀࠋࠀ瀅䀀〆က؋က܋ကЫကࠋက\u{0018}က\u{0018}က\u{0018}ࠀ瀅ࠀދࠀދࠀދࠀދࠀЫࠀՋࠀࠋ��〈䀀〆��〈��瀅䀀〆��\u{0018}��\u{0018}��\u{0018}栀׫栀׫栀܋栀Ы��㝉��㝉䀀〆��瀅��瀅䀀〆䀀〆��瀅��〈��〈䀀〆��〈��〈䀀〆��\u{0018}��တ砀����တ��㘉��㘉䀀〆䀀〆��㕉��㕉��瀅��〈��瀅䀀〆��〈��瀅��瀅��\u{0018}��\u{0018}䀀〆砀����Ћ��Ћ��Ћ��Ћ��ԫ��ً��ࠋ��ࠋ砀��䀀〆��〈䀀〆䀀〆��〈砀��砀����〈��〈��〈��\u{0018}��瀅䀀〆��\u{0018}栀\u{0018}砀����〈䀀〆��؋��ܫ��\u{0018}��\u{001c}\u{0082}瀁\u{0082}瀁\u{0081}瀂\u{0081}瀂䀀〆��〆��〆䀀〆��〈��〆��ӫ��ӫ��〈��\u{0018}��ࠋ栀\u{001c}栀\u{001c}⠀怚⠀怚⠀怚⠀怚栀\u{001c}��瑊��瑊��睪��睪��睪��皪��皪��皪��皪��疊��疊��疊��瑪��瑪��瑪��矪��矪��矊��矊��矊��皪��皊��皊��皊��砊��砊��疪��疪��疪��疊��甪��甊��甊��瓪��瓊��璪��瓊��瓊��璪��璊��璊��瑪��瑪��瑊��琪��琊��眊��眊��眊��癊��癊��癊��癊��瘪��瘪��瘊��甪��甪��砊��睪��睪砀����တ��တ��瀄��瀄砀����׫��֫��֫��֫��\u{0018}䀀〆��瀄栀\u{0018}��瀄䀀〆砀����瀅��瀅��瀅砀��砀����瀄��瀄砀��䠀တ䠀တ��\u{001c}��〈��〈��〈��〈䠀တ䠀တ䀀〆䀀〆��\u{001c}��\u{001c}栀\u{001c}䀀〆栀\u{001c}��Ы��Ы��Ջ��٫��׫��׫��׫��Ջ��֫砀����瀁��瀁��瀂��瀂��瀂砀����瀁砀��砀����瀁砀����瀂��瀁��\u{0019}��瀂\u{0019}��瀁��瀂᠀㙉᠀㙉᠀㔉᠀㔉᠀㟉᠀㟉᠀㚉᠀㚉᠀㕉᠀㕉��\u{001c}��\u{0018}��瀅��瀂䀀〆��瀄��瀅��\u{001c}砀��⠀怚��瀅��瀄ࢊ瀁ࢊ瀁ࢉ瀂ࢉ瀂䀀〆ࠀ瀄ࠀ㘉ࠀ㘉砀��က؋က؋က؋က؋ကܫကыကࠋကࠋကࠋကࠋကߋကߋကߋက\u{001c}ကࠋက怚က؋က؋ကࠋကࠋ砀��砀��ကЋကЋကЋကЋကԫကًကࠋက\u{001c}က٫က٫က٫砀��က瀅က瀅砀��栀\u{0019}栀\u{0019}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}砀��砀��栀\u{001c}砀��砀��栀\u{001c}栀\u{001c}栀\u{001c}᠀Ћ᠀߫᠀߫᠀߫᠀߫栀ګ栀ڋ栀\u{001c}��\u{001c}栀\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}��\u{001c}砀����\u{001c}��\u{001c}��\u{001c}砀��栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001b}栀\u{001b}栀\u{001b}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}砀��栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}砀��栀\u{001c}栀\u{001c}砀��᠀㘉᠀㘉")
        }

        #[cfg_attr(any(), java_field(name = "B", descriptor = "[C", access = "package", modifiers = "static final", is_static = true))]
        // static field: B:[C
        pub fn B() -> Rc<RefCell<Vec<u16>>> {
            panic!("stub: java/lang/CharacterData01.B:[C")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "getProperties", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getProperties(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData01.getProperties:(I)I")
        }

        #[java_method(name = "getPropertiesEx", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPropertiesEx(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData01.getPropertiesEx:(I)I")
        }

        #[java_method(name = "getType", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getType(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData01.getType:(I)I")
        }

        #[java_method(name = "isOtherAlphabetic", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isOtherAlphabetic(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isOtherAlphabetic:(I)Z")
        }

        #[java_method(name = "isIdeographic", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isIdeographic(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isIdeographic:(I)Z")
        }

        #[java_method(name = "isJavaIdentifierStart", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isJavaIdentifierStart(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isJavaIdentifierStart:(I)Z")
        }

        #[java_method(name = "isJavaIdentifierPart", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isJavaIdentifierPart(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isJavaIdentifierPart:(I)Z")
        }

        #[java_method(name = "isUnicodeIdentifierStart", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUnicodeIdentifierStart(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isUnicodeIdentifierStart:(I)Z")
        }

        #[java_method(name = "isUnicodeIdentifierPart", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUnicodeIdentifierPart(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isUnicodeIdentifierPart:(I)Z")
        }

        #[java_method(name = "isIdentifierIgnorable", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isIdentifierIgnorable(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isIdentifierIgnorable:(I)Z")
        }

        #[java_method(name = "isEmoji", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmoji(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isEmoji:(I)Z")
        }

        #[java_method(name = "isEmojiPresentation", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiPresentation(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isEmojiPresentation:(I)Z")
        }

        #[java_method(name = "isEmojiModifier", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiModifier(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isEmojiModifier:(I)Z")
        }

        #[java_method(name = "isEmojiModifierBase", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiModifierBase(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isEmojiModifierBase:(I)Z")
        }

        #[java_method(name = "isEmojiComponent", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiComponent(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isEmojiComponent:(I)Z")
        }

        #[java_method(name = "isExtendedPictographic", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isExtendedPictographic(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isExtendedPictographic:(I)Z")
        }

        #[java_method(name = "toLowerCase", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLowerCase(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData01.toLowerCase:(I)I")
        }

        #[java_method(name = "toUpperCase", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCase(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData01.toUpperCase:(I)I")
        }

        #[java_method(name = "toTitleCase", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toTitleCase(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData01.toTitleCase:(I)I")
        }

        #[java_method(name = "digit", descriptor = "(II)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn digit(&self, ch: i32, radix: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData01.digit:(II)I")
        }

        #[java_method(name = "getNumericValue", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNumericValue(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData01.getNumericValue:(I)I")
        }

        #[java_method(name = "isDigit", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDigit(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isDigit:(I)Z")
        }

        #[java_method(name = "isLowerCase", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLowerCase(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isLowerCase:(I)Z")
        }

        #[java_method(name = "isUpperCase", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUpperCase(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isUpperCase:(I)Z")
        }

        #[java_method(name = "isWhitespace", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isWhitespace(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isWhitespace:(I)Z")
        }

        #[java_method(name = "getDirectionality", descriptor = "(I)B", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDirectionality(&self, ch: i32) -> Result<i8> {
            panic!("stub: java/lang/CharacterData01.getDirectionality:(I)B")
        }

        #[java_method(name = "isMirrored", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isMirrored(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData01.isMirrored:(I)Z")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/CharacterData01.<init>:()V")
        }
    }
}
