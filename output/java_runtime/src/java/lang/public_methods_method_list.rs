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
    #[binary_name       = "java/lang/PublicMethods$MethodList"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "PublicMethods.java"]
    #[inner_classes     = "java/lang/PublicMethods$MethodList:java/lang/PublicMethods:MethodList:24;java/lang/PublicMethods$Key:java/lang/PublicMethods:Key:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/lang/PublicMethods$MethodList"]

    pub struct PublicMethods_MethodList {
        #[cfg_attr(any(), java_field(name = "method", descriptor = "Ljava/lang/reflect/Method;", is_static = false))]
        pub method: Method,
        #[cfg_attr(any(), java_field(name = "next", descriptor = "Ljava/lang/PublicMethods$MethodList;", is_static = false))]
        pub next: PublicMethods_MethodList,
    }

    impl PublicMethods_MethodList {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/reflect/Method;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut method: Method) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_method(Clone::clone(&method));
            Ok(this)
        }

        #[java_method(name = "filter", descriptor = "([Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class;Z)Ljava/lang/PublicMethods$MethodList;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class<*>;Z)Ljava/lang/PublicMethods$MethodList;")]
        pub fn filter(mut methods: Rc<RefCell<Vec<Method>>>, mut name: String, mut ptypes: Rc<RefCell<Vec<Object>>>, mut includeStatic: bool) -> Result<PublicMethods_MethodList> {
            let mut head: Object = Object::default();
            let mut tail: Object = Object::default();
            let mut local_6: Rc<RefCell<Vec<Method>>> = methods;
            let mut local_7 = (local_6.borrow().len() as i32);
            let mut local_8: i32 = 0i32;
            let mut head = Default::default();
            loop {
                if local_8 >= local_7 { break; }
                let mut method = Clone::clone(&local_6.borrow()[local_8 as usize]);
                let _t0 = method.getModifiers()?;
                let _t1: bool = Modifier::isStatic(_t0)?;
                let _t2: bool = PublicMethods_Key::matches(Clone::clone(&method), Clone::clone(&name), Default::default())?;
                if _is_jnull(&tail) {
                    let mut tail = PublicMethods_MethodList::new(Clone::clone(&method))?;
                    head = PublicMethods_MethodList::new(Clone::clone(&method))?;
                } else {
                    tail.__set_next(PublicMethods_MethodList::new(Clone::clone(&method))?);
                    let mut tail = PublicMethods_MethodList::new(Clone::clone(&method))?;
                }
                local_8 = local_8.wrapping_add(1i32);
            }
            Ok(head)
        }

        #[java_method(name = "merge", descriptor = "(Ljava/lang/PublicMethods$MethodList;Ljava/lang/PublicMethods$MethodList;)Ljava/lang/PublicMethods$MethodList;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: merge(Ljava/lang/PublicMethods$MethodList;Ljava/lang/PublicMethods$MethodList;)Ljava/lang/PublicMethods$MethodList;
        pub fn merge_public_public(mut head: PublicMethods_MethodList, mut methodList: PublicMethods_MethodList) -> Result<PublicMethods_MethodList> {
            let mut ml: PublicMethods_MethodList = methodList;
            loop {
                if _is_jnull(&ml) { break; }
                let _t0: PublicMethods_MethodList = PublicMethods_MethodList::merge_public_method(Clone::clone(&head), Clone::clone(&ml.__get_method()))?;
                head = _t0;
                ml = ml.__get_next();
            }
            Ok(head)
        }

        #[java_method(name = "merge", descriptor = "(Ljava/lang/PublicMethods$MethodList;Ljava/lang/reflect/Method;)Ljava/lang/PublicMethods$MethodList;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: merge(Ljava/lang/PublicMethods$MethodList;Ljava/lang/reflect/Method;)Ljava/lang/PublicMethods$MethodList;
        pub fn merge_public_method(mut head: PublicMethods_MethodList, mut method: Method) -> Result<PublicMethods_MethodList> {
            let _t0 = method.getDeclaringClass()?;
            let mut dclass = (_t0).downcast::<Class<Object>>();
            let _t1 = method.getReturnType()?;
            let mut rtype = (_t1).downcast::<Class<Object>>();
            let mut prev: Object = Object::default();
            let mut l: PublicMethods_MethodList = head;
            let mut prev: PublicMethods_MethodList = Default::default();
            loop {
                if _is_jnull(&l) { break; }
                let mut xmethod = l.__get_method();
                let _t2 = xmethod.getReturnType()?;
                if Object::from_any(rtype.clone()) == _t2 {
                    let _t3 = xmethod.getDeclaringClass()?;
                    let mut xdclass = (_t3).downcast::<Class<Object>>();
                    let _t4 = dclass.isInterface()?;
                    let _t5 = xdclass.isInterface()?;
                    if _t4 == _t5 {
                        let _t6 = dclass.isAssignableFrom(Clone::clone(&xdclass))?;
                        if _t6 {
                            return Ok(head);
                        }
                        let _t7 = xdclass.isAssignableFrom(Clone::clone(&dclass))?;
                        if _t7 {
                            if !_is_jnull(&prev) {
                                prev.__set_next(Clone::clone(&l.__get_next()));
                            } else {
                                head = l.__get_next();
                            }
                        } else {
                            prev = l;
                        }
                    } else {
                        let _t6 = dclass.isInterface()?;
                        if _t6 {
                            return Ok(head);
                        }
                        if !_is_jnull(&prev) {
                            prev.__set_next(Clone::clone(&l.__get_next()));
                        } else {
                            head = l.__get_next();
                        }
                    }
                } else {
                    let mut prev: PublicMethods_MethodList = l;
                }
                l = l.__get_next();
            }
            if _is_jnull(&prev) {
                head = PublicMethods_MethodList::new(Clone::clone(&method))?;
            } else {
                prev.__set_next(PublicMethods_MethodList::new(Clone::clone(&method))?);
            }
            Ok(head)
        }

        #[java_method(name = "length", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn length(&self) -> Result<i32> {
            panic!("stub: java/lang/PublicMethods$MethodList.length:()I")
        }

        #[java_method(name = "getMostSpecific", descriptor = "()Ljava/lang/reflect/Method;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMostSpecific(&self) -> Result<Method> {
            let this = self;
            let mut m = this.__get_method();
            let _t0 = m.getReturnType()?;
            let mut rt = (_t0).downcast::<Class<Object>>();
            let mut ml = this.__get_next();
            loop {
                if _is_jnull(&ml) { break; }
                let mut m2 = ml.__get_method();
                let _t1 = m2.getReturnType()?;
                let mut rt2 = (_t1).downcast::<Class<Object>>();
                let _t2 = rt.isAssignableFrom(Clone::clone(&rt2))?;
                if _t2 {
                    m = m2;
                    rt = rt2;
                }
                ml = ml.__get_next();
            }
            Ok(m)
        }
    }
}
