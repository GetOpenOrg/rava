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
use crate::jdk::internal::misc::Unsafe;

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<ConcurrentHashMap_TreeBin<K, V>> for ConcurrentHashMap_Node<K, V> {
    fn from(v: ConcurrentHashMap_TreeBin<K, V>) -> ConcurrentHashMap_Node<K, V> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/concurrent/ConcurrentHashMap$TreeBin"]
    #[super_class       = "java/util/concurrent/ConcurrentHashMap$Node"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ConcurrentHashMap.java"]
    #[inner_classes     = "java/util/concurrent/ConcurrentHashMap$Node:java/util/concurrent/ConcurrentHashMap:Node:8;java/util/concurrent/ConcurrentHashMap$TreeBin:java/util/concurrent/ConcurrentHashMap:TreeBin:24;java/util/concurrent/ConcurrentHashMap$TreeNode:java/util/concurrent/ConcurrentHashMap:TreeNode:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "ConcurrentHashMap_Node<K, V>"]
    #[superclass_fields(hash: i32, key: K, val: V, next: ConcurrentHashMap_Node<K, V>)]
    #[all_supertypes    = "java/lang/Object;java/util/Map$Entry;java/util/concurrent/ConcurrentHashMap$Node;java/util/concurrent/ConcurrentHashMap$TreeBin"]

    pub struct ConcurrentHashMap_TreeBin<K: Clone + Default + 'static, V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "root", descriptor = "Ljava/util/concurrent/ConcurrentHashMap$TreeNode;", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;"))]
        pub root: ConcurrentHashMap_TreeNode<K, V>,
        #[cfg_attr(any(), java_field(name = "first", descriptor = "Ljava/util/concurrent/ConcurrentHashMap$TreeNode;", access = "package", modifiers = "volatile", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;"))]
        pub first: ConcurrentHashMap_TreeNode<K, V>,
        #[cfg_attr(any(), java_field(name = "waiter", descriptor = "Ljava/lang/Thread;", access = "package", modifiers = "volatile", is_static = false))]
        pub waiter: Thread,
        #[cfg_attr(any(), java_field(name = "lockState", descriptor = "I", access = "package", modifiers = "volatile", is_static = false))]
        pub lockState: i32,
    }

    impl<K, V> ConcurrentHashMap_TreeBin<K, V> {
        #[cfg_attr(any(), java_field(name = "WRITER", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: WRITER:I
        pub fn WRITER() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "WAITER", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: WAITER:I
        pub fn WAITER() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "READER", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: READER:I
        pub fn READER() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "LOCKSTATE", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: LOCKSTATE:J
        pub fn LOCKSTATE() -> i64 {
            panic!("stub: java/util/concurrent/ConcurrentHashMap$TreeBin.LOCKSTATE:J")
        }

        #[cfg_attr(any(), java_field(name = "WAITERTHREAD", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: WAITERTHREAD:J
        pub fn WAITERTHREAD() -> i64 {
            panic!("stub: java/util/concurrent/ConcurrentHashMap$TreeBin.WAITERTHREAD:J")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "tieBreakOrder", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tieBreakOrder(mut a: Object, mut b: Object) -> Result<i32> {
            let _vdispatch0: Object = if let Some(__f) = a.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let _vdispatch1: String = if let Some(__f) = _vdispatch0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            let _vdispatch2: Object = if let Some(__f) = b.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let _vdispatch3: String = if let Some(__f) = _vdispatch2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            let _t4 = _vdispatch1.compareTo(Clone::clone(&_vdispatch3))?;
            let mut d: i32 = _t4;
            let _t5: i32 = System::identityHashCode(Clone::clone(&a))?;
            let _t6: i32 = System::identityHashCode(Clone::clone(&b))?;
            d = (_t5 > _t6) as i32;
            Ok(d)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;)V")]
        pub fn new(mut b: ConcurrentHashMap_TreeNode<K, V>) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(ConcurrentHashMap_Node::new_i_obj_obj(-2i32, Default::default(), Default::default())?);
            this.__set_first(Clone::clone(&b));
            let mut r: ConcurrentHashMap_TreeNode<K, V> = Default::default();
            let mut x: ConcurrentHashMap_TreeNode<K, V> = b;
            loop {
                if _is_jnull(&x) { break; }
                let mut next: ConcurrentHashMap_TreeNode<Object, Object> = Default::default();
                x.__set_right(Default::default());
                x.__set_left(Default::default());
        let mut dir: i32 = Default::default();
                if _is_jnull(&r) {
                    x.__set_parent(Default::default());
                    x.__set_red((0i32 != 0i32));
                    r = x;
                } else {
                    let mut k = x.__get_key();
                    let mut h = x.__get_hash();
                    let mut kc: Class<Object> = Default::default();
                    let mut p: ConcurrentHashMap_TreeNode<K, V> = r;
                    dir = Default::default();
                    let mut r: ConcurrentHashMap_TreeNode<Object, Object> = Default::default();
                    loop {
                        let mut pk = p.__get_key();
                        let mut ph = p.__get_hash();
                        if p.__get_hash() > h {
                            dir = -1i32;
                        } else {
                            if ph < h {
                                dir = 1i32;
                            } else {
                                let _t0: Object = ConcurrentHashMap::<Object, Object>::comparableClassFor(Clone::clone(&k))?;
                                kc = _t0;
                                let _t1: i32 = ConcurrentHashMap::<Object, Object>::compareComparables(Clone::clone(&kc), Clone::clone(&k), Clone::clone(&pk))?;
                                dir = _t1;
                                if (dir==0) {
                                    let _t2: i32 = ConcurrentHashMap_TreeBin::<Object, Object>::tieBreakOrder(Clone::clone(&k), Clone::clone(&pk))?;
                                    dir = _t2;
                                }
                            }
                        }
                        let mut xp: ConcurrentHashMap_TreeNode<K, V> = p;
                        let mut p = (if (dir<=0) { p.__get_left() } else { p.__get_right() });
                        if _is_jnull(&(if (dir<=0) { p.__get_left() } else { p.__get_right() })) {
                            x.__set_parent(Clone::clone(&xp));
                            if (dir<=0) {
                                xp.__set_left(Clone::clone(&x));
                            } else {
                                xp.__set_right(Clone::clone(&x));
                            }
                            let _t0: ConcurrentHashMap_TreeNode<Object, Object> = ConcurrentHashMap_TreeBin::<Object, Object>::balanceInsertion(Clone::clone(&r), Clone::clone(&x))?;
                            r = _t0;
                        } else {
                            continue;
                        }
                    }
                }
                let mut x: ConcurrentHashMap_TreeNode<Object, Object> = next;
            }
            this.__set_root(Clone::clone(&r));
            let _t0: bool = ConcurrentHashMap_TreeBin::<Object, Object>::checkInvariants(Clone::clone(&this.__get_root()))?;
            if !(_t0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(this)
        }

        #[java_method(name = "lockRoot", descriptor = "()V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lockRoot(&self) -> Result<()> {
            let this = self;
            let _t0 = ConcurrentHashMap::<Object, Object>::U().compareAndSetInt(Object::from_any(Clone::clone(self)), ConcurrentHashMap_TreeBin::<Object, Object>::LOCKSTATE(), 0i32, 1i32)?;
            if !(_t0) {
                this.contendedLock()?;
            }
            Ok(())
        }

        #[java_method(name = "unlockRoot", descriptor = "()V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unlockRoot(&self) -> Result<()> {
            let this = self;
            this.__set_lockState(0i32);
            Ok(())
        }

        #[java_method(name = "contendedLock", descriptor = "()V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contendedLock(&self) -> Result<()> {
            let this = self;
            let _t0: Thread = Thread::currentThread()?;
            let mut current: Thread = _t0;
            loop {
                let mut s = this.__get_lockState();
                let _t1 = ConcurrentHashMap::<Object, Object>::U().compareAndSetInt(Object::from_any(Clone::clone(self)), ConcurrentHashMap_TreeBin::<Object, Object>::LOCKSTATE(), s, 1i32)?;
                if Object::from_any(this.__get_waiter().clone()) == Object::from_any(current.clone()) {
                    let _t2 = ConcurrentHashMap::<Object, Object>::U().compareAndSetReference(Object::from_any(Clone::clone(self)), ConcurrentHashMap_TreeBin::<Object, Object>::WAITERTHREAD(), Object::from_any(current.clone()), Clone::clone(&Object::default()))?;
                }
                return Ok(());
                let _t2 = ConcurrentHashMap::<Object, Object>::U().compareAndSetInt(Object::from_any(Clone::clone(self)), ConcurrentHashMap_TreeBin::<Object, Object>::LOCKSTATE(), s, (s|2i32))?;
                continue;
                let mut w = this.__get_waiter();
                let _t3 = ConcurrentHashMap::<Object, Object>::U().compareAndSetReference(Object::from_any(Clone::clone(self)), ConcurrentHashMap_TreeBin::<Object, Object>::WAITERTHREAD(), Clone::clone(&Object::default()), Object::from_any(current.clone()))?;
                continue;
                LockSupport::park_obj(Object::from_any(Clone::clone(self)))?;
            }
            Ok(())
        }

        #[java_method(name = "find", descriptor = "(ILjava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$Node;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;")]
        pub fn find(&self, h: i32, k: Object) -> Result<ConcurrentHashMap_Node<Object, Object>> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap$TreeBin.find:(ILjava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$Node;")
        }

        #[java_method(name = "putTreeVal", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;")]
        pub fn putTreeVal(&self, mut h: i32, mut k: K, mut v: V) -> Result<ConcurrentHashMap_TreeNode<K, V>> {
            let this = self;
            let mut kc: Class<Object> = Default::default();
            let mut searched: i32 = 0i32;
            let mut p = this.__get_root();
            let mut dir: i32 = Default::default();
            loop {
                if _is_jnull(&p) {
                    this.__set_root(ConcurrentHashMap_TreeNode::<Object, Object>::new(h, Clone::clone(&k), Clone::clone(&v), Default::default(), Default::default())?);
                    this.__set_first(ConcurrentHashMap_TreeNode::<Object, Object>::new(h, Clone::clone(&k), Clone::clone(&v), Default::default(), Default::default())?);
                } else {
                    let mut ph = p.__get_hash();
                    if p.__get_hash() > h {
                        dir = -1i32;
                    } else {
                        if ph < h {
                            let mut dir: i32 = 1i32;
                        } else {
                            let mut pk = p.__get_key();
                            let _t0 = k.equals(Clone::clone(&pk))?;
                            if _t0 {
                                return Ok(Default::default());
                            }
                            let _t1: Object = ConcurrentHashMap::<Object, Object>::comparableClassFor(Clone::clone(&k))?;
                            kc = _t1;
                            let _t2: i32 = ConcurrentHashMap::<Object, Object>::compareComparables(Clone::clone(&kc), Clone::clone(&k), Clone::clone(&pk))?;
                            let mut dir: i32 = _t2;
                            searched = 1i32;
                            let mut ch = p.__get_left();
                            let _t3 = ch.findTreeNode(h, Clone::clone(&k), Clone::clone(&kc))?;
                            let mut q: ConcurrentHashMap_TreeNode<Object, Object> = _t3;
                            ch = p.__get_right();
                            let _t4 = ch.findTreeNode(h, Clone::clone(&k), Clone::clone(&kc))?;
                            q = _t4;
                            if !_is_jnull(&q) {
                                return Ok(Default::default());
                            }
                            let _t5: i32 = ConcurrentHashMap_TreeBin::<Object, Object>::tieBreakOrder(Clone::clone(&k), Clone::clone(&pk))?;
                            dir = _t5;
                        }
                    }
                    let mut q: ConcurrentHashMap_TreeNode<Object, Object> = p;
                    p = (if (dir<=0) { p.__get_left() } else { p.__get_right() });
                    if _is_jnull(&(if (dir<=0) { p.__get_left() } else { p.__get_right() })) {
                        let mut f = this.__get_first();
                        let mut ch = ConcurrentHashMap_TreeNode::<Object, Object>::new(h, Clone::clone(&k), Clone::clone(&v), Clone::clone(&f).into(), Clone::clone(&q))?;
                        this.__set_first(ConcurrentHashMap_TreeNode::<Object, Object>::new(h, Clone::clone(&k), Clone::clone(&v), Clone::clone(&f).into(), Clone::clone(&q))?);
                        if !_is_jnull(&f) {
                            f.__set_prev(Clone::clone(&ch));
                        }
                        if (dir<=0) {
                            q.__set_left(Clone::clone(&ch));
                        } else {
                            q.__set_right(Clone::clone(&ch));
                        }
                        if !(q.__get_red()) {
                            ch.__set_red((1i32 != 0i32));
                        } else {
                            this.lockRoot()?;
                            let _t0: ConcurrentHashMap_TreeNode<Object, Object> = ConcurrentHashMap_TreeBin::<Object, Object>::balanceInsertion(Clone::clone(&this.__get_root()), Clone::clone(&ch))?;
                            this.__set_root(Clone::clone(&_t0));
                            this.unlockRoot()?;
                        }
                    } else {
                        continue;
                    }
                }
            }
            let _t0: bool = ConcurrentHashMap_TreeBin::<Object, Object>::checkInvariants(Clone::clone(&this.__get_root()))?;
            if !(_t0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(Default::default())
        }

        #[java_method(name = "removeTreeNode", descriptor = "(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)Z", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;)Z")]
        pub fn removeTreeNode(&self, p: ConcurrentHashMap_TreeNode<K, V>) -> Result<bool> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap$TreeBin.removeTreeNode:(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)Z")
        }

        #[java_method(name = "rotateLeft", descriptor = "(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;")]
        pub fn rotateLeft(mut root: ConcurrentHashMap_TreeNode<K, V>, mut p: ConcurrentHashMap_TreeNode<K, V>) -> Result<ConcurrentHashMap_TreeNode<K, V>> {
            let mut r = p.__get_right();
            p.__set_right(Clone::clone(&r.__get_left()));
            let mut rl = r.__get_left();
            if !_is_jnull(&r.__get_left()) {
                rl.__set_parent(Clone::clone(&p));
            }
            r.__set_parent(Clone::clone(&p.__get_parent()));
            let mut pp = p.__get_parent();
            if _is_jnull(&p.__get_parent()) {
                let mut root: ConcurrentHashMap_TreeNode<Object, Object> = r;
                root.__set_red((0i32 != 0i32));
            } else {
                if Object::from_any(pp.__get_left().clone()) == Object::from_any(p.clone()) {
                    pp.__set_left(Clone::clone(&r));
                } else {
                    pp.__set_right(Clone::clone(&r));
                }
            }
            r.__set_left(Clone::clone(&p));
            p.__set_parent(Clone::clone(&r));
            Ok(Default::default())
        }

        #[java_method(name = "rotateRight", descriptor = "(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;")]
        pub fn rotateRight(mut root: ConcurrentHashMap_TreeNode<K, V>, mut p: ConcurrentHashMap_TreeNode<K, V>) -> Result<ConcurrentHashMap_TreeNode<K, V>> {
            let mut l = p.__get_left();
            p.__set_left(Clone::clone(&l.__get_right()));
            let mut lr = l.__get_right();
            if !_is_jnull(&l.__get_right()) {
                lr.__set_parent(Clone::clone(&p));
            }
            l.__set_parent(Clone::clone(&p.__get_parent()));
            let mut pp = p.__get_parent();
            if _is_jnull(&p.__get_parent()) {
                let mut root: ConcurrentHashMap_TreeNode<Object, Object> = l;
                root.__set_red((0i32 != 0i32));
            } else {
                if Object::from_any(pp.__get_right().clone()) == Object::from_any(p.clone()) {
                    pp.__set_right(Clone::clone(&l));
                } else {
                    pp.__set_left(Clone::clone(&l));
                }
            }
            l.__set_right(Clone::clone(&p));
            p.__set_parent(Clone::clone(&l));
            Ok(Default::default())
        }

        #[java_method(name = "balanceInsertion", descriptor = "(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;")]
        pub fn balanceInsertion(mut root: ConcurrentHashMap_TreeNode<K, V>, mut x: ConcurrentHashMap_TreeNode<K, V>) -> Result<ConcurrentHashMap_TreeNode<K, V>> {
            x.__set_red((1i32 != 0i32));
            loop {
                let mut xp = x.__get_parent();
                if _is_jnull(&x.__get_parent()) {
                    x.__set_red((0i32 != 0i32));
                    return Ok(x);
                }
                let mut xpp = xp.__get_parent();
                if _is_jnull(&xp.__get_parent()) {
                    return Ok(root);
                }
                let mut xppl = xpp.__get_left();
                let mut xppr = xpp.__get_right();
                if xppr.__get_red() {
                    xppr.__set_red((0i32 != 0i32));
                    xp.__set_red((0i32 != 0i32));
                    xpp.__set_red((1i32 != 0i32));
                    let mut x: ConcurrentHashMap_TreeNode<Object, Object> = xpp;
                    continue;
                }
                let mut x: ConcurrentHashMap_TreeNode<Object, Object> = xp;
                let _t0: ConcurrentHashMap_TreeNode<Object, Object> = ConcurrentHashMap_TreeBin::<Object, Object>::rotateLeft(Clone::clone(&root), Clone::clone(&x))?;
                let mut root: ConcurrentHashMap_TreeNode<Object, Object> = _t0;
                xp = x.__get_parent();
                let mut xpp = (if _is_jnull(&x.__get_parent()) { Object::default() } else { Object::from_any(Clone::clone(&xp.__get_parent())) });
                xp.__set_red((0i32 != 0i32));
                xpp.__set_red((1i32 != 0i32));
                let _t1: ConcurrentHashMap_TreeNode<Object, Object> = ConcurrentHashMap_TreeBin::<Object, Object>::rotateRight(Clone::clone(&root), Clone::clone(&xpp))?;
                root = _t1;
                continue;
                if xppl.__get_red() {
                    xppl.__set_red((0i32 != 0i32));
                    xp.__set_red((0i32 != 0i32));
                    xpp.__set_red((1i32 != 0i32));
                    let mut x: ConcurrentHashMap_TreeNode<K, V> = xpp;
                    continue;
                }
                x = xp;
                let _t2: ConcurrentHashMap_TreeNode<Object, Object> = ConcurrentHashMap_TreeBin::<Object, Object>::rotateRight(Clone::clone(&root), Clone::clone(&x))?;
                root = _t2;
                xp = x.__get_parent();
                xpp = (if _is_jnull(&x.__get_parent()) { Object::default() } else { Object::from_any(Clone::clone(&xp.__get_parent())) });
                xp.__set_red((0i32 != 0i32));
                xpp.__set_red((1i32 != 0i32));
                let _t3: ConcurrentHashMap_TreeNode<Object, Object> = ConcurrentHashMap_TreeBin::<Object, Object>::rotateLeft(Clone::clone(&root), Clone::clone(&xpp))?;
                root = _t3;
            }
        }

        #[java_method(name = "balanceDeletion", descriptor = "(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;")]
        pub fn balanceDeletion(root: ConcurrentHashMap_TreeNode<K, V>, x: ConcurrentHashMap_TreeNode<K, V>) -> Result<ConcurrentHashMap_TreeNode<Object, Object>> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap$TreeBin.balanceDeletion:(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)Ljava/util/concurrent/ConcurrentHashMap$TreeNode;")
        }

        #[java_method(name = "checkInvariants", descriptor = "(Ljava/util/concurrent/ConcurrentHashMap$TreeNode;)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/concurrent/ConcurrentHashMap$TreeNode<TK;TV;>;)Z")]
        pub fn checkInvariants(mut t: ConcurrentHashMap_TreeNode<K, V>) -> Result<bool> {
            let mut tp = t.__get_parent();
            let mut tl = t.__get_left();
            let mut tr = t.__get_right();
            let mut tb = t.__get_prev();
            let mut tn: ConcurrentHashMap_TreeNode<Object, Object> = Default::default();
            if Object::from_any(tb.__get_next().clone()) != Object::from_any(t.clone()) {
                return Ok((0i32 != 0i32));
            }
            if Object::from_any(tn.__get_prev().clone()) != Object::from_any(t.clone()) {
                return Ok((0i32 != 0i32));
            }
            if Object::from_any(t.clone()) != Object::from_any(tp.__get_right().clone()) {
                return Ok((0i32 != 0i32));
            }
            if tl.__get_hash() > t.__get_hash() {
                return Ok((0i32 != 0i32));
            }
            if tr.__get_hash() < t.__get_hash() {
                return Ok((0i32 != 0i32));
            }
            if tr.__get_red() {
                return Ok((0i32 != 0i32));
            }
            let _t0: bool = ConcurrentHashMap_TreeBin::<Object, Object>::checkInvariants(Clone::clone(&tl))?;
            if !(_t0) {
                return Ok((0i32 != 0i32));
            }
            let _t1: bool = ConcurrentHashMap_TreeBin::<Object, Object>::checkInvariants(Clone::clone(&tr))?;
            if !(_t1) {
                return Ok((0i32 != 0i32));
            }
            Ok((1i32 != 0i32))
        }
    }
}
