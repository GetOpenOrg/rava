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

impl From<CharacterData00> for CharacterData {
    fn from(v: CharacterData00) -> CharacterData { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/CharacterData00"]
    #[super_class       = "java/lang/CharacterData"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "CharacterData00.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "CharacterData"]
    #[all_supertypes    = "java/lang/CharacterData;java/lang/CharacterData00;java/lang/Object"]

    pub struct CharacterData00;

    impl CharacterData00 {
        #[cfg_attr(any(), java_field(name = "instance", descriptor = "Ljava/lang/CharacterData00;", access = "package", modifiers = "static final", is_static = true))]
        // static field: instance:Ljava/lang/CharacterData00;
        pub fn instance() -> CharacterData00 {
            panic!("stub: java/lang/CharacterData00.instance:Ljava/lang/CharacterData00;")
        }

        #[cfg_attr(any(), java_field(name = "charMap", descriptor = "[[[C", access = "package", modifiers = "static final", is_static = true))]
        // static field: charMap:[[[C
        pub fn charMap() -> Rc<RefCell<Vec<Rc<RefCell<Vec<Rc<RefCell<Vec<u16>>>>>>>>> {
            panic!("stub: java/lang/CharacterData00.charMap:[[[C")
        }

        #[cfg_attr(any(), java_field(name = "X", descriptor = "[C", access = "package", modifiers = "static final", is_static = true))]
        // static field: X:[C
        pub fn X() -> Rc<RefCell<Vec<u16>>> {
            panic!("stub: java/lang/CharacterData00.X:[C")
        }

        #[cfg_attr(any(), java_field(name = "Y", descriptor = "[C", access = "package", modifiers = "static final", is_static = true))]
        // static field: Y:[C
        pub fn Y() -> Rc<RefCell<Vec<u16>>> {
            panic!("stub: java/lang/CharacterData00.Y:[C")
        }

        #[cfg_attr(any(), java_field(name = "A", descriptor = "[I", access = "package", modifiers = "static final", is_static = true))]
        // static field: A:[I
        pub fn A() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/lang/CharacterData00.A:[I")
        }

        #[cfg_attr(any(), java_field(name = "A_DATA", descriptor = "Ljava/lang/String;", access = "package", modifiers = "static final", is_static = true, constant_value = "䠀ဏ䠀ဏ䠀ဏ堀䀏倀䀏堀䀏怀䀏倀䀏倀䀏倀䀏怀䀌栀\u{0018}栀\u{0018}⠀\u{0018}⠀怚⠀\u{0018}栀\u{0018}栀\u{0018}\u{0015}\u{0016}栀\u{0018} \u{0019}㠀\u{0018} \u{0014}㠀\u{0018}㠀\u{0018}᠀㘉᠀㘉㠀\u{0018}栀\u{0018}\u{0019}栀\u{0019}\u{0019}栀\u{0018}栀\u{0018}\u{0082}翡\u{0082}翡\u{0082}翡\u{0082}翡\u{0015}栀\u{0018}\u{0016}栀\u{001b}栀倗栀\u{001b}\u{0081}翢\u{0081}翢\u{0081}翢\u{0081}翢\u{0015}栀\u{0019}\u{0016}栀\u{0019}䠀ဏ䠀ဏ倀ဏ㠀\u{000c}栀\u{0018}⠀怚⠀怚栀\u{001c}栀\u{0018}栀\u{001b}栀\u{001c}��瀅\u{001d}栀\u{0019}䠀တ栀\u{001c}栀\u{001b}⠀\u{001c}⠀\u{0019}᠀؋᠀؋栀\u{001b}߽瀂栀\u{0018}栀\u{0018}栀\u{001b}᠀ԋ��瀅\u{001e}栀ࠋ栀ࠋ栀ࠋ栀\u{0018}\u{0082}瀁\u{0082}瀁\u{0082}瀁栀\u{0019}\u{0082}瀁߽瀂\u{0081}瀂\u{0081}瀂\u{0081}瀂栀\u{0019}\u{0081}瀂؝瀂\u{0006}瀁\u{0005}瀂߿Ρ瀂��瀂\u{0006}瀁\u{0005}瀂\u{0006}瀁\u{0005}瀂߽瀂؞瀁\u{0006}瀁ӵ瀂͊瀁̺瀁\u{0006}瀁\u{0005}瀂̶瀁̶瀁\u{0006}瀁\u{0005}瀂��瀂ľ瀁̪瀁̮瀁\u{0006}瀁̾瀁ٽ瀂͎瀁͆瀁յ瀂��瀂͎瀁͖瀁׹瀂͚瀁ͪ瀁\u{0006}瀁\u{0005}瀂ͪ瀁��瀂��瀂\u{0005}瀂ͦ瀁ͦ瀁\u{0006}瀁\u{0005}瀂ͮ瀁��瀂��瀅��瀂ܡ瀂��瀅��瀅\n\u{0007}\t\n\u{0007}\t\t\u{0006}瀁\u{0005}瀂Ľ瀂߽瀂\nپ瀁ܢ瀁׺瀁��瀂߾瀁\u{0006}瀁\u{0005}瀂ն瀁߾瀁߽瀂߽瀂\u{0006}瀁\u{0005}瀂Ӷ瀁Ė瀁Ğ瀁߽瀂߽瀂߽瀂͉瀂̹瀂��瀂̵瀂̵瀂��瀂̩瀂��瀂̭瀂߽瀂��瀂̵瀂߽瀂��瀂̽瀂��瀂߽瀂ͅ瀂͍瀂��瀂͍瀂͕瀂��瀂��瀂͙瀂ͩ瀂��瀂߽瀂ͩ瀂ͩ瀂ĕ瀂ͥ瀂ͥ瀂ĝ瀂��瀂ͭ瀂��瀂��瀅��瀂��瀄��瀄��瀄栀瀄栀瀄��瀄��瀄��瀄栀\u{001b}栀\u{001b}栀瀄栀瀄��瀄栀\u{001b}栀瀄栀\u{001b}��瀄栀\u{001b}䀀〆䀀〆䀀〆䚱〆砀��砀����瀄׹瀂׹瀂׹瀂栀\u{0018}ǒ瀁\u{009a}瀁栀\u{0018}\u{0096}瀁\u{0096}瀁\u{0096}瀁砀��Ă瀁砀��þ瀁þ瀁߽瀂\u{0082}瀁砀��\u{0082}瀁\u{0099}瀂\u{0095}瀂\u{0095}瀂\u{0095}瀂߽瀂\u{0081}瀂}瀂\u{0081}瀂ā瀂ý瀂ý瀂\"瀁ù瀂å瀂��瀁��瀁��瀁½瀂Ù瀂!瀂ř瀂Ł瀂ߥ瀂Ǒ瀂ܒ瀁Ɓ瀂栀\u{0019}\u{0006}瀁\u{0005}瀂ߦ瀁��瀂׺瀁׺瀁׺瀁ł瀁ł瀁Ł瀂Ł瀂��\u{001c}䀀〆䀀\u{0007}䀀\u{0007}>瀁\u{0006}瀁\u{0005}瀂=瀂砀��Â瀁Â瀁Â瀁Â瀁砀��砀����瀄��\u{0018}��\u{0018}��瀂Á瀂Á瀂Á瀂Á瀂߽瀂��瀂��\u{0018}栀\u{0014}砀��砀��栀\u{001c}栀\u{001c}⠀怚砀��䀀〆䀀〆䀀〆ࠀ\u{0014}䀀〆ࠀ\u{0018}䀀〆䀀〆ࠀ\u{0018}ࠀ瀅ࠀ瀅ࠀ瀅砀��砀��ࠀ瀅ࠀ瀅ࠀ\u{0018}ࠀ\u{0018}砀��　တ　တ栀\u{0019}栀\u{0019}က\u{0019}⠀\u{0018}⠀\u{0018}က怚㠀\u{0018}က\u{0018}栀\u{001c}栀\u{001c}䀀〆က\u{0018}ကတက\u{0018}က\u{0018}က\u{0018}က瀅က瀅က瀄က瀅က瀅䀀〆䀀〆䀀〆　㐉　㐉⠀\u{0018}　\u{0018}　\u{0018}က\u{0018}䀀〆က瀅က\u{0018}က瀅䀀〆　တ栀\u{001c}䀀〆䀀〆က瀄က瀄䀀〆䀀〆栀\u{001c}᠀㘉᠀㘉က瀅က\u{001c}က\u{001c}က瀅砀��ကတ䀀〆砀��砀��က瀅ࠀ㐉ࠀ㐉ࠀ瀅䀀〆ࠀ瀄ࠀ瀄ࠀ瀄砀��ࠀ怚ࠀ怚ࠀ瀄䀀〆䀀〆䀀〆ࠀ\u{0018}ࠀ\u{0018}က瀅砀��က\u{001b}က瀅က瀅က瀄　တ䀀〆䀀〆��〈䀀〆��瀅��〈��〈��〈䀀〆��〈䀀〆��瀅䀀〆��㝉��㝉��\u{0018}��瀄��瀅䀀〆砀����瀅��瀅砀��䀀〆砀��砀����〈��〈砀����ࠋ��ࠋ��ࠋ��۫��\u{001c}⠀怚��瀅��\u{0018}砀��䀀〆��\u{0018}砀����\u{0018}⠀怚��\u{001c}��瀅䀀〆��瀅��݋��ࠋ��ࠋ栀\u{001c}栀\u{001c}砀��砀����\u{0018}栀ԋ栀ԋ栀ҫ栀ҫ栀ҫ��\u{001c}��\u{0018}��瀅��〈��〆��〆��〈��瀅��〈��瀅��\u{001c}��ࠋ��瀅��ࠋ��\u{001c}砀��⠀怚��瀄䀀〆䀀〆��\u{0018}��㘉��㘉��瀄砀����\u{001c}��\u{001c}��\u{0018}��\u{001c}��㐉��㐉��〈��〈䀀〆��\u{001c}��\u{001c}砀����\u{001c}��\u{0018}䀀〆��〈��〈��瀅߾瀁߾瀁砀��߾瀁߽߽߽��\u{0018}��瀄߽��\u{0018}��܋��܋��܋��܋��Ы��Ջ��ࠋ��ࠋ砀��\"瀁\"瀁!瀂!瀂栀\u{0014}��瀅怀䀌��瀅��瀅\u{0015}\u{0016}砀����瑪��瑪��瑪��瀅䀀〆��〈��〈��\u{0018}栀؋栀؋栀\u{0014}栀\u{0018}栀\u{0018}䀀〆䠀တ䀀〆��瀅��瀄��瀅䀀〆䀀〆��瀅��ӫ砀��䀀\u{0007}䀀〆��〈��瀅��〈䀀〆߽瀂砀��߾瀁砀����瀅��〈��瀄��瀂��瀄߽瀂��瀂��瀄߽瀂í瀂߾瀁��瀂ߡ瀂ߡ瀂ߢ瀁ߢ瀁߽瀂ߡ瀂砀��ߢ瀁ۙ瀂ۙ瀂ک瀂ک瀂ٱ瀂ٱ瀂؁瀂؁瀂ف瀂ف瀂؉瀂؉瀂߿߿ۚ瀁ۚ瀁߿栀\u{001b}߽瀂栀\u{001b}ڪ瀁ڪ瀁ٲ瀁ٲ瀁砀��栀\u{001b}߽瀂ߥ瀂ق瀁ق瀁ߦ瀁栀\u{001b}؂瀁؂瀁؊瀁؊瀁栀\u{001b}砀��怀䀌怀䀌怀䀌怀\u{000c}怀䀌䠀တ䠀တ䠀တ��တࠀတ栀\u{0014}栀\u{0014}栀\u{001d}栀\u{001e}栀\u{0015}栀\u{001d}怀䀍倀䀎砀တ砀တ砀တ㠀\u{000c}⠀\u{0018}⠀\u{0018}⠀\u{0018}栀\u{0018}栀\u{0018}\u{001d}\u{001e}栀\u{0018}栀\u{0018}栀\u{0018}栀\u{0018}栀倗栀倗栀\u{0018}㠀\u{0019}\u{0015}\u{0016}栀\u{0018}栀\u{0018}栀\u{0018}栀\u{0019}栀\u{0018}栀\u{0018}怀䀌䠀တ䠀တ䠀တ砀��᠀؋��瀄 \u{0019} \u{0019}栀\u{0019}\u{0015}\u{0016}��瀄᠀Ћ᠀Ћ��瀄砀��⠀怚砀��䀀〆䀀\u{0007}䀀\u{0007}䀀〆䀀\u{0007}䀀\u{0007}��瀁栀\u{001c}栀\u{001c}��瀁��瀂��瀁��瀁��瀂栀\u{0019}��瀁栀\u{001c}栀\u{001c}߾瀁栀\u{001c}⠀\u{001c}��瀂r瀁��瀁��瀅��瀂栀\u{0019}��瀁栀\u{001c}栀\u{0019}q瀂��\u{001c}B琪B琪B砊B砊A瘪A瘪A砊A砊��砊��砊��砊\u{0006}瀁\u{0005}瀂��琪��砊栀۫栀\u{0019}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{0019}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{0019}\u{0019}\u{0019}\u{0019} \u{0019}⠀\u{0019}栀\u{001c}栀\u{001c}栀\u{001c}\u{0015}\u{0016}栀\u{001c}��\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}��\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀Ы栀Ы栀֫栀֫᠀ܫ᠀ܫj\u{001c}j\u{001c}j\u{001c}j\u{001c}i\u{001c}i\u{001c}栀ۋ栀Ћ栀Ћ栀Ћ栀Ћ栀֋栀֋栀֋栀֋栀Ы栀\u{001c}栀\u{0019}栀\u{0019}栀\u{0019}栀\u{0019}栀\u{0019}栀\u{0019}栀\u{0019}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{0019}��\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀ի栀ի栀۫栀۫\u{0019}\u{0015}\u{0016}栀\u{0019}栀\u{0019}栀\u{0019}\u{0016}\u{0015}\u{001c}栀\u{001c}\u{0005}瀂߾瀁��瀂栀\u{001c}栀\u{001c}\u{0006}瀁\u{0005}瀂䀀〆砀��栀\u{0018}栀\u{0018}栀ࠋ砀��߽瀂\u{001d}\u{001e}栀\u{0018}栀\u{0014}栀\u{0018}栀瀄栀\u{0015}栀\u{0018}栀\u{0018}\u{0015}\u{0016}栀\u{0014}栀\u{001c}��瀄��瀅��眪栀\u{0014}栀\u{0015}栀\u{0016}栀\u{0016}栀\u{001c}��琊��琊��琊栀\u{0014}��瀄��癊��睪��璊��瀄��瀅栀\u{0018}䀀〆栀\u{001b}栀\u{001b}��瀄��瀄��瀅��瀅栀\u{0018}��׫��׫��Ы��Ы��ы��ի��ڋ��ࠋ栀\u{001c}栀ҋ栀ҋ栀ҋ��\u{001c}��\u{001c}��\u{001c}栀ࠋ��瀅��瀅��瀄栀\u{0018}䀀\u{0007}栀\u{0018}栀\u{0018}栀瀄��睪��睪��睪��瘪栀\u{001b}栀瀄栀瀄��\u{001b}��\u{001b}\u{0006}瀁݁瀂��瀂݂瀁߾瀁\u{0005}瀂砀��砀����瀂��瀄\u{0006}瀁\u{0005}瀂��瀅⠀怚⠀\u{001c}��〈��〈��瀄��〈��瀂��\u{001b}��〈��\u{0018}��\u{0013}��\u{0013}��\u{0012}��\u{0012}��瀅��眅��瀅��盥��畅��瀅��病��瀅��瀅��皥��瀅��癥��瀅��疥䀀〆ࠀ瀅ࠀ瀅 \u{0019}က\u{001b}က\u{001b}က\u{001b}砀��栀\u{0016}栀\u{0015}က怚栀\u{001c}䀀〆䀀〆栀\u{0018}栀\u{0015}栀\u{0016}栀\u{0018}栀\u{0014}栀倗栀倗栀\u{0015}栀倗栀倗㠀\u{0018}砀��栀\u{0018}㠀\u{0018}栀\u{0014}\u{0015}\u{0016}⠀\u{0018} \u{0019} \u{0014}栀\u{0019}砀��栀\u{0018}⠀怚砀��䠀တ栀\u{0018}⠀\u{0018}栀\u{0018} \u{0019}栀\u{0019}栀\u{001b}砀��栀တ栀တ栀တ"))]
        // static field: A_DATA:Ljava/lang/String;
        pub fn A_DATA() -> String {
            String::from("䠀ဏ䠀ဏ䠀ဏ堀䀏倀䀏堀䀏怀䀏倀䀏倀䀏倀䀏怀䀌栀\u{0018}栀\u{0018}⠀\u{0018}⠀怚⠀\u{0018}栀\u{0018}栀\u{0018}\u{0015}\u{0016}栀\u{0018} \u{0019}㠀\u{0018} \u{0014}㠀\u{0018}㠀\u{0018}᠀㘉᠀㘉㠀\u{0018}栀\u{0018}\u{0019}栀\u{0019}\u{0019}栀\u{0018}栀\u{0018}\u{0082}翡\u{0082}翡\u{0082}翡\u{0082}翡\u{0015}栀\u{0018}\u{0016}栀\u{001b}栀倗栀\u{001b}\u{0081}翢\u{0081}翢\u{0081}翢\u{0081}翢\u{0015}栀\u{0019}\u{0016}栀\u{0019}䠀ဏ䠀ဏ倀ဏ㠀\u{000c}栀\u{0018}⠀怚⠀怚栀\u{001c}栀\u{0018}栀\u{001b}栀\u{001c}��瀅\u{001d}栀\u{0019}䠀တ栀\u{001c}栀\u{001b}⠀\u{001c}⠀\u{0019}᠀؋᠀؋栀\u{001b}߽瀂栀\u{0018}栀\u{0018}栀\u{001b}᠀ԋ��瀅\u{001e}栀ࠋ栀ࠋ栀ࠋ栀\u{0018}\u{0082}瀁\u{0082}瀁\u{0082}瀁栀\u{0019}\u{0082}瀁߽瀂\u{0081}瀂\u{0081}瀂\u{0081}瀂栀\u{0019}\u{0081}瀂؝瀂\u{0006}瀁\u{0005}瀂߿Ρ瀂��瀂\u{0006}瀁\u{0005}瀂\u{0006}瀁\u{0005}瀂߽瀂؞瀁\u{0006}瀁ӵ瀂͊瀁̺瀁\u{0006}瀁\u{0005}瀂̶瀁̶瀁\u{0006}瀁\u{0005}瀂��瀂ľ瀁̪瀁̮瀁\u{0006}瀁̾瀁ٽ瀂͎瀁͆瀁յ瀂��瀂͎瀁͖瀁׹瀂͚瀁ͪ瀁\u{0006}瀁\u{0005}瀂ͪ瀁��瀂��瀂\u{0005}瀂ͦ瀁ͦ瀁\u{0006}瀁\u{0005}瀂ͮ瀁��瀂��瀅��瀂ܡ瀂��瀅��瀅\n\u{0007}\t\n\u{0007}\t\t\u{0006}瀁\u{0005}瀂Ľ瀂߽瀂\nپ瀁ܢ瀁׺瀁��瀂߾瀁\u{0006}瀁\u{0005}瀂ն瀁߾瀁߽瀂߽瀂\u{0006}瀁\u{0005}瀂Ӷ瀁Ė瀁Ğ瀁߽瀂߽瀂߽瀂͉瀂̹瀂��瀂̵瀂̵瀂��瀂̩瀂��瀂̭瀂߽瀂��瀂̵瀂߽瀂��瀂̽瀂��瀂߽瀂ͅ瀂͍瀂��瀂͍瀂͕瀂��瀂��瀂͙瀂ͩ瀂��瀂߽瀂ͩ瀂ͩ瀂ĕ瀂ͥ瀂ͥ瀂ĝ瀂��瀂ͭ瀂��瀂��瀅��瀂��瀄��瀄��瀄栀瀄栀瀄��瀄��瀄��瀄栀\u{001b}栀\u{001b}栀瀄栀瀄��瀄栀\u{001b}栀瀄栀\u{001b}��瀄栀\u{001b}䀀〆䀀〆䀀〆䚱〆砀��砀����瀄׹瀂׹瀂׹瀂栀\u{0018}ǒ瀁\u{009a}瀁栀\u{0018}\u{0096}瀁\u{0096}瀁\u{0096}瀁砀��Ă瀁砀��þ瀁þ瀁߽瀂\u{0082}瀁砀��\u{0082}瀁\u{0099}瀂\u{0095}瀂\u{0095}瀂\u{0095}瀂߽瀂\u{0081}瀂}瀂\u{0081}瀂ā瀂ý瀂ý瀂\"瀁ù瀂å瀂��瀁��瀁��瀁½瀂Ù瀂!瀂ř瀂Ł瀂ߥ瀂Ǒ瀂ܒ瀁Ɓ瀂栀\u{0019}\u{0006}瀁\u{0005}瀂ߦ瀁��瀂׺瀁׺瀁׺瀁ł瀁ł瀁Ł瀂Ł瀂��\u{001c}䀀〆䀀\u{0007}䀀\u{0007}>瀁\u{0006}瀁\u{0005}瀂=瀂砀��Â瀁Â瀁Â瀁Â瀁砀��砀����瀄��\u{0018}��\u{0018}��瀂Á瀂Á瀂Á瀂Á瀂߽瀂��瀂��\u{0018}栀\u{0014}砀��砀��栀\u{001c}栀\u{001c}⠀怚砀��䀀〆䀀〆䀀〆ࠀ\u{0014}䀀〆ࠀ\u{0018}䀀〆䀀〆ࠀ\u{0018}ࠀ瀅ࠀ瀅ࠀ瀅砀��砀��ࠀ瀅ࠀ瀅ࠀ\u{0018}ࠀ\u{0018}砀��　တ　တ栀\u{0019}栀\u{0019}က\u{0019}⠀\u{0018}⠀\u{0018}က怚㠀\u{0018}က\u{0018}栀\u{001c}栀\u{001c}䀀〆က\u{0018}ကတက\u{0018}က\u{0018}က\u{0018}က瀅က瀅က瀄က瀅က瀅䀀〆䀀〆䀀〆　㐉　㐉⠀\u{0018}　\u{0018}　\u{0018}က\u{0018}䀀〆က瀅က\u{0018}က瀅䀀〆　တ栀\u{001c}䀀〆䀀〆က瀄က瀄䀀〆䀀〆栀\u{001c}᠀㘉᠀㘉က瀅က\u{001c}က\u{001c}က瀅砀��ကတ䀀〆砀��砀��က瀅ࠀ㐉ࠀ㐉ࠀ瀅䀀〆ࠀ瀄ࠀ瀄ࠀ瀄砀��ࠀ怚ࠀ怚ࠀ瀄䀀〆䀀〆䀀〆ࠀ\u{0018}ࠀ\u{0018}က瀅砀��က\u{001b}က瀅က瀅က瀄　တ䀀〆䀀〆��〈䀀〆��瀅��〈��〈��〈䀀〆��〈䀀〆��瀅䀀〆��㝉��㝉��\u{0018}��瀄��瀅䀀〆砀����瀅��瀅砀��䀀〆砀��砀����〈��〈砀����ࠋ��ࠋ��ࠋ��۫��\u{001c}⠀怚��瀅��\u{0018}砀��䀀〆��\u{0018}砀����\u{0018}⠀怚��\u{001c}��瀅䀀〆��瀅��݋��ࠋ��ࠋ栀\u{001c}栀\u{001c}砀��砀����\u{0018}栀ԋ栀ԋ栀ҫ栀ҫ栀ҫ��\u{001c}��\u{0018}��瀅��〈��〆��〆��〈��瀅��〈��瀅��\u{001c}��ࠋ��瀅��ࠋ��\u{001c}砀��⠀怚��瀄䀀〆䀀〆��\u{0018}��㘉��㘉��瀄砀����\u{001c}��\u{001c}��\u{0018}��\u{001c}��㐉��㐉��〈��〈䀀〆��\u{001c}��\u{001c}砀����\u{001c}��\u{0018}䀀〆��〈��〈��瀅߾瀁߾瀁砀��߾瀁߽߽߽��\u{0018}��瀄߽��\u{0018}��܋��܋��܋��܋��Ы��Ջ��ࠋ��ࠋ砀��\"瀁\"瀁!瀂!瀂栀\u{0014}��瀅怀䀌��瀅��瀅\u{0015}\u{0016}砀����瑪��瑪��瑪��瀅䀀〆��〈��〈��\u{0018}栀؋栀؋栀\u{0014}栀\u{0018}栀\u{0018}䀀〆䠀တ䀀〆��瀅��瀄��瀅䀀〆䀀〆��瀅��ӫ砀��䀀\u{0007}䀀〆��〈��瀅��〈䀀〆߽瀂砀��߾瀁砀����瀅��〈��瀄��瀂��瀄߽瀂��瀂��瀄߽瀂í瀂߾瀁��瀂ߡ瀂ߡ瀂ߢ瀁ߢ瀁߽瀂ߡ瀂砀��ߢ瀁ۙ瀂ۙ瀂ک瀂ک瀂ٱ瀂ٱ瀂؁瀂؁瀂ف瀂ف瀂؉瀂؉瀂߿߿ۚ瀁ۚ瀁߿栀\u{001b}߽瀂栀\u{001b}ڪ瀁ڪ瀁ٲ瀁ٲ瀁砀��栀\u{001b}߽瀂ߥ瀂ق瀁ق瀁ߦ瀁栀\u{001b}؂瀁؂瀁؊瀁؊瀁栀\u{001b}砀��怀䀌怀䀌怀䀌怀\u{000c}怀䀌䠀တ䠀တ䠀တ��တࠀတ栀\u{0014}栀\u{0014}栀\u{001d}栀\u{001e}栀\u{0015}栀\u{001d}怀䀍倀䀎砀တ砀တ砀တ㠀\u{000c}⠀\u{0018}⠀\u{0018}⠀\u{0018}栀\u{0018}栀\u{0018}\u{001d}\u{001e}栀\u{0018}栀\u{0018}栀\u{0018}栀\u{0018}栀倗栀倗栀\u{0018}㠀\u{0019}\u{0015}\u{0016}栀\u{0018}栀\u{0018}栀\u{0018}栀\u{0019}栀\u{0018}栀\u{0018}怀䀌䠀တ䠀တ䠀တ砀��᠀؋��瀄 \u{0019} \u{0019}栀\u{0019}\u{0015}\u{0016}��瀄᠀Ћ᠀Ћ��瀄砀��⠀怚砀��䀀〆䀀\u{0007}䀀\u{0007}䀀〆䀀\u{0007}䀀\u{0007}��瀁栀\u{001c}栀\u{001c}��瀁��瀂��瀁��瀁��瀂栀\u{0019}��瀁栀\u{001c}栀\u{001c}߾瀁栀\u{001c}⠀\u{001c}��瀂r瀁��瀁��瀅��瀂栀\u{0019}��瀁栀\u{001c}栀\u{0019}q瀂��\u{001c}B琪B琪B砊B砊A瘪A瘪A砊A砊��砊��砊��砊\u{0006}瀁\u{0005}瀂��琪��砊栀۫栀\u{0019}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{0019}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{0019}\u{0019}\u{0019}\u{0019} \u{0019}⠀\u{0019}栀\u{001c}栀\u{001c}栀\u{001c}\u{0015}\u{0016}栀\u{001c}��\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}��\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀Ы栀Ы栀֫栀֫᠀ܫ᠀ܫj\u{001c}j\u{001c}j\u{001c}j\u{001c}i\u{001c}i\u{001c}栀ۋ栀Ћ栀Ћ栀Ћ栀Ћ栀֋栀֋栀֋栀֋栀Ы栀\u{001c}栀\u{0019}栀\u{0019}栀\u{0019}栀\u{0019}栀\u{0019}栀\u{0019}栀\u{0019}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{0019}��\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀\u{001c}栀ի栀ի栀۫栀۫\u{0019}\u{0015}\u{0016}栀\u{0019}栀\u{0019}栀\u{0019}\u{0016}\u{0015}\u{001c}栀\u{001c}\u{0005}瀂߾瀁��瀂栀\u{001c}栀\u{001c}\u{0006}瀁\u{0005}瀂䀀〆砀��栀\u{0018}栀\u{0018}栀ࠋ砀��߽瀂\u{001d}\u{001e}栀\u{0018}栀\u{0014}栀\u{0018}栀瀄栀\u{0015}栀\u{0018}栀\u{0018}\u{0015}\u{0016}栀\u{0014}栀\u{001c}��瀄��瀅��眪栀\u{0014}栀\u{0015}栀\u{0016}栀\u{0016}栀\u{001c}��琊��琊��琊栀\u{0014}��瀄��癊��睪��璊��瀄��瀅栀\u{0018}䀀〆栀\u{001b}栀\u{001b}��瀄��瀄��瀅��瀅栀\u{0018}��׫��׫��Ы��Ы��ы��ի��ڋ��ࠋ栀\u{001c}栀ҋ栀ҋ栀ҋ��\u{001c}��\u{001c}��\u{001c}栀ࠋ��瀅��瀅��瀄栀\u{0018}䀀\u{0007}栀\u{0018}栀\u{0018}栀瀄��睪��睪��睪��瘪栀\u{001b}栀瀄栀瀄��\u{001b}��\u{001b}\u{0006}瀁݁瀂��瀂݂瀁߾瀁\u{0005}瀂砀��砀����瀂��瀄\u{0006}瀁\u{0005}瀂��瀅⠀怚⠀\u{001c}��〈��〈��瀄��〈��瀂��\u{001b}��〈��\u{0018}��\u{0013}��\u{0013}��\u{0012}��\u{0012}��瀅��眅��瀅��盥��畅��瀅��病��瀅��瀅��皥��瀅��癥��瀅��疥䀀〆ࠀ瀅ࠀ瀅 \u{0019}က\u{001b}က\u{001b}က\u{001b}砀��栀\u{0016}栀\u{0015}က怚栀\u{001c}䀀〆䀀〆栀\u{0018}栀\u{0015}栀\u{0016}栀\u{0018}栀\u{0014}栀倗栀倗栀\u{0015}栀倗栀倗㠀\u{0018}砀��栀\u{0018}㠀\u{0018}栀\u{0014}\u{0015}\u{0016}⠀\u{0018} \u{0019} \u{0014}栀\u{0019}砀��栀\u{0018}⠀怚砀��䠀တ栀\u{0018}⠀\u{0018}栀\u{0018} \u{0019}栀\u{0019}栀\u{001b}砀��栀တ栀တ栀တ")
        }

        #[cfg_attr(any(), java_field(name = "B", descriptor = "[C", access = "package", modifiers = "static final", is_static = true))]
        // static field: B:[C
        pub fn B() -> Rc<RefCell<Vec<u16>>> {
            panic!("stub: java/lang/CharacterData00.B:[C")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "getProperties", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getProperties(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData00.getProperties:(I)I")
        }

        #[java_method(name = "getPropertiesEx", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPropertiesEx(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData00.getPropertiesEx:(I)I")
        }

        #[java_method(name = "getType", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getType(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData00.getType:(I)I")
        }

        #[java_method(name = "isOtherAlphabetic", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isOtherAlphabetic(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isOtherAlphabetic:(I)Z")
        }

        #[java_method(name = "isIdeographic", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isIdeographic(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isIdeographic:(I)Z")
        }

        #[java_method(name = "isJavaIdentifierStart", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isJavaIdentifierStart(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isJavaIdentifierStart:(I)Z")
        }

        #[java_method(name = "isJavaIdentifierPart", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isJavaIdentifierPart(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isJavaIdentifierPart:(I)Z")
        }

        #[java_method(name = "isUnicodeIdentifierStart", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUnicodeIdentifierStart(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isUnicodeIdentifierStart:(I)Z")
        }

        #[java_method(name = "isUnicodeIdentifierPart", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUnicodeIdentifierPart(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isUnicodeIdentifierPart:(I)Z")
        }

        #[java_method(name = "isIdentifierIgnorable", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isIdentifierIgnorable(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isIdentifierIgnorable:(I)Z")
        }

        #[java_method(name = "isEmoji", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmoji(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isEmoji:(I)Z")
        }

        #[java_method(name = "isEmojiPresentation", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiPresentation(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isEmojiPresentation:(I)Z")
        }

        #[java_method(name = "isEmojiModifier", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiModifier(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isEmojiModifier:(I)Z")
        }

        #[java_method(name = "isEmojiModifierBase", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiModifierBase(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isEmojiModifierBase:(I)Z")
        }

        #[java_method(name = "isEmojiComponent", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiComponent(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isEmojiComponent:(I)Z")
        }

        #[java_method(name = "isExtendedPictographic", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isExtendedPictographic(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isExtendedPictographic:(I)Z")
        }

        #[java_method(name = "toLowerCase", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLowerCase(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData00.toLowerCase:(I)I")
        }

        #[java_method(name = "toUpperCase", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCase(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData00.toUpperCase:(I)I")
        }

        #[java_method(name = "toTitleCase", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toTitleCase(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData00.toTitleCase:(I)I")
        }

        #[java_method(name = "digit", descriptor = "(II)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn digit(&self, ch: i32, radix: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData00.digit:(II)I")
        }

        #[java_method(name = "getNumericValue", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNumericValue(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData00.getNumericValue:(I)I")
        }

        #[java_method(name = "isDigit", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDigit(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isDigit:(I)Z")
        }

        #[java_method(name = "isLowerCase", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLowerCase(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isLowerCase:(I)Z")
        }

        #[java_method(name = "isUpperCase", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUpperCase(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isUpperCase:(I)Z")
        }

        #[java_method(name = "isWhitespace", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isWhitespace(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isWhitespace:(I)Z")
        }

        #[java_method(name = "getDirectionality", descriptor = "(I)B", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDirectionality(&self, ch: i32) -> Result<i8> {
            panic!("stub: java/lang/CharacterData00.getDirectionality:(I)B")
        }

        #[java_method(name = "isMirrored", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isMirrored(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData00.isMirrored:(I)Z")
        }

        #[java_method(name = "toUpperCaseEx", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCaseEx(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData00.toUpperCaseEx:(I)I")
        }

        #[java_method(name = "toUpperCaseCharArray", descriptor = "(I)[C", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCaseCharArray(&self, ch: i32) -> Result<Rc<RefCell<Vec<u16>>>> {
            panic!("stub: java/lang/CharacterData00.toUpperCaseCharArray:(I)[C")
        }

        #[java_method(name = "findInCharMap", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn findInCharMap(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData00.findInCharMap:(I)I")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/CharacterData00.<init>:()V")
        }
    }
}
