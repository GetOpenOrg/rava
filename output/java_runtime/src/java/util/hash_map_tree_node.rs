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

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<HashMap_TreeNode<K, V>> for LinkedHashMap_Entry<K, V> {
    fn from(v: HashMap_TreeNode<K, V>) -> LinkedHashMap_Entry<K, V> { v.__into_super() }
}

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<HashMap_TreeNode<K, V>> for HashMap_Node<K, V> {
    fn from(v: HashMap_TreeNode<K, V>) -> HashMap_Node<K, V> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/HashMap$TreeNode"]
    #[super_class       = "java/util/LinkedHashMap$Entry"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/util/LinkedHashMap$Entry<TK;TV;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "HashMap.java"]
    #[inner_classes     = "java/util/LinkedHashMap$Entry:java/util/LinkedHashMap:Entry:8;java/util/HashMap$Node:java/util/HashMap:Node:8;java/util/HashMap$TreeNode:java/util/HashMap:TreeNode:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "LinkedHashMap_Entry<K, V>"]
    #[superclass_fields(hash: i32, key: K, value: V, next: HashMap_Node<K, V>, before: LinkedHashMap_Entry<K, V>, after: LinkedHashMap_Entry<K, V>)]
    #[all_supertypes    = "java/lang/Object;java/util/HashMap$Node;java/util/HashMap$TreeNode;java/util/LinkedHashMap$Entry;java/util/Map$Entry"]

    pub struct HashMap_TreeNode<K: Clone + Default + 'static, V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "parent", descriptor = "Ljava/util/HashMap$TreeNode;", is_static = false, generic_signature = "Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
        pub parent: HashMap_TreeNode<K, V>,
        #[cfg_attr(any(), java_field(name = "left", descriptor = "Ljava/util/HashMap$TreeNode;", is_static = false, generic_signature = "Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
        pub left: HashMap_TreeNode<K, V>,
        #[cfg_attr(any(), java_field(name = "right", descriptor = "Ljava/util/HashMap$TreeNode;", is_static = false, generic_signature = "Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
        pub right: HashMap_TreeNode<K, V>,
        #[cfg_attr(any(), java_field(name = "prev", descriptor = "Ljava/util/HashMap$TreeNode;", is_static = false, generic_signature = "Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
        pub prev: HashMap_TreeNode<K, V>,
        #[cfg_attr(any(), java_field(name = "red", descriptor = "Z", is_static = false))]
        pub red: bool,
    }

    impl<K, V> HashMap_TreeNode<K, V> {
        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;Ljava/util/HashMap$Node<TK;TV;>;)V")]
        pub fn new(mut hash: i32, mut key: K, mut val: V, mut next: HashMap_Node<K, V>) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(LinkedHashMap_Entry::new(hash, Clone::clone(&key), Clone::clone(&val), Clone::clone(&next))?);
            Ok(this)
        }

        #[java_method(name = "root", descriptor = "()Ljava/util/HashMap$TreeNode;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/HashMap$TreeNode<TK;TV;>;")]
        pub fn root(&self) -> Result<HashMap_TreeNode<K, V>> {
            let this = self;
            let mut r = this.clone();
            loop {
                let mut p = r.__get_parent();
                if _is_jnull(&r.__get_parent()) {
                    return Ok(Default::default());
                }
                let mut r: HashMap_TreeNode<Object, Object> = p;
            }
        }

        #[java_method(name = "moveRootToFront", descriptor = "([Ljava/util/HashMap$Node;Ljava/util/HashMap$TreeNode;)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>([Ljava/util/HashMap$Node<TK;TV;>;Ljava/util/HashMap$TreeNode<TK;TV;>;)V")]
        pub fn moveRootToFront(mut tab: Rc<RefCell<Vec<HashMap_Node<K, V>>>>, mut root: HashMap_TreeNode<K, V>) -> Result<()> {
            let mut n = (tab.borrow().len() as i32);
            let mut index = ((n).wrapping_sub(1i32)&root.__get_hash());
            let mut first: HashMap_TreeNode<Object, Object> = Default::default();
            tab.borrow_mut()[index as usize] = Clone::clone(&root);
            let mut rp = root.__get_prev();
            let mut rn = root.__get_next();
            if !_is_jnull(&root.__get_next()) {
                Default::default().__set_prev(Clone::clone(&rp));
            }
            if !_is_jnull(&rp) {
                rp.__set_next(Clone::clone(&rn));
            }
            if !_is_jnull(&first) {
                first.__set_prev(Clone::clone(&root));
            }
            root.__set_next(Clone::clone(&<_ as Into<HashMap_Node<Object, Object>>>::into(first)));
            root.__set_prev(Default::default());
            let _t0: bool = HashMap_TreeNode::<Object, Object>::checkInvariants(Clone::clone(&root))?;
            if !(_t0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "find", descriptor = "(ILjava/lang/Object;Ljava/lang/Class;)Ljava/util/HashMap$TreeNode;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/lang/Object;Ljava/lang/Class<*>;)Ljava/util/HashMap$TreeNode<TK;TV;>;")]
        pub fn find(&self, mut h: i32, mut k: Object, mut kc: Class<Object>) -> Result<HashMap_TreeNode<K, V>> {
            let this = self;
            let mut p = this.clone();
            let mut p: HashMap_TreeNode<Object, Object> = Default::default();
            loop {
                let mut pl = p.__get_left();
                let mut pr = p.__get_right();
                let mut ph = p.__get_hash();
                if p.__get_hash() > h {
                    p = pl;
                } else {
                    if ph < h {
                        let mut p: HashMap_TreeNode<Object, Object> = pr;
                    } else {
                        let mut pk = p.__get_key();
                        let _vdispatch0: bool = if let Some(__f) = k.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Clone::clone(&pk))? } else { Default::default() };
                        if _vdispatch0 {
                            return Ok(Default::default());
                        }
                        if _is_jnull(&pl) {
                            let mut p: HashMap_TreeNode<Object, Object> = pr;
                        } else {
                            if _is_jnull(&pr) {
                                let mut p: HashMap_TreeNode<Object, Object> = pl;
                            } else {
                                let _t1: Object = HashMap::<Object, Object>::comparableClassFor(Clone::clone(&k))?;
                                kc = _t1;
                                if !_is_jnull(&kc) {
                                    let _t2: i32 = HashMap::<Object, Object>::compareComparables(Clone::clone(&kc), Clone::clone(&k), Clone::clone(&pk))?;
                                    let mut dir: i32 = _t2;
                                    if (dir!=0) {
                                        let mut p = (if (dir<0) { pl } else { pr });
                                    } else {
                                        let _t3 = pr.find(h, Clone::clone(&k), Clone::clone(&kc))?;
                                        let mut q: HashMap_TreeNode<Object, Object> = _t3;
                                        if !_is_jnull(&q) {
                                            return Ok(Default::default());
                                        }
                                        let mut p: HashMap_TreeNode<Object, Object> = pl;
                                    }
                                } else {
                                    let _t2 = pr.find(h, Clone::clone(&k), Clone::clone(&kc))?;
                                    let mut q: HashMap_TreeNode<Object, Object> = _t2;
                                    if !_is_jnull(&q) {
                                        return Ok(Default::default());
                                    }
                                    let mut p: HashMap_TreeNode<Object, Object> = pl;
                                }
                            }
                        }
                    }
                }
                if _is_jnull(&p) { break; }
            }
            Ok(Default::default())
        }

        #[java_method(name = "getTreeNode", descriptor = "(ILjava/lang/Object;)Ljava/util/HashMap$TreeNode;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/lang/Object;)Ljava/util/HashMap$TreeNode<TK;TV;>;")]
        pub fn getTreeNode(&self, mut h: i32, mut k: Object) -> Result<HashMap_TreeNode<K, V>> {
            let this = self;
            let mut _merged1: HashMap_TreeNode<Object, Object>;
            if !_is_jnull(&this.__get_parent()) {
                let _t0 = this.root()?;
                _merged1 = _t0;
            } else {
                _merged1 = Clone::clone(this);
            }
            let _t2 = _merged1.find(h, Clone::clone(&k), Default::default())?;
            Ok(Default::default())
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

        #[java_method(name = "treeify", descriptor = "([Ljava/util/HashMap$Node;)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([Ljava/util/HashMap$Node<TK;TV;>;)V")]
        pub fn treeify(&self, mut tab: Rc<RefCell<Vec<HashMap_Node<K, V>>>>) -> Result<()> {
            let this = self;
            let mut root: HashMap_TreeNode<K, V> = Default::default();
            let mut x = this.clone();
            let mut root: HashMap_TreeNode = Default::default();
            loop {
                if _is_jnull(&x) { break; }
                let mut next: HashMap_TreeNode<Object, Object> = Default::default();
                x.__set_right(Default::default());
                x.__set_left(Default::default());
        let mut dir: i32 = Default::default();
                if _is_jnull(&root) {
                    x.__set_parent(Default::default());
                    x.__set_red((0i32 != 0i32));
                    root = x;
                } else {
                    let mut k = x.__get_key();
                    let mut h = x.__get_hash();
                    let mut kc: Class<Object> = Default::default();
                    let mut p: HashMap_TreeNode<K, V> = root;
                    dir = Default::default();
                    loop {
                        let mut pk = p.__get_key();
                        let mut ph = p.__get_hash();
                        if p.__get_hash() > h {
                            dir = -1i32;
                        } else {
                            if ph < h {
                                dir = 1i32;
                            } else {
                                let _t0: Object = HashMap::<Object, Object>::comparableClassFor(Clone::clone(&k))?;
                                kc = _t0;
                                let _t1: i32 = HashMap::<Object, Object>::compareComparables(Clone::clone(&kc), Clone::clone(&k), Clone::clone(&pk))?;
                                dir = _t1;
                                if (dir==0) {
                                    let _t2: i32 = HashMap_TreeNode::<Object, Object>::tieBreakOrder(Clone::clone(&k), Clone::clone(&pk))?;
                                    dir = _t2;
                                }
                            }
                        }
                        let mut xp: HashMap_TreeNode<K, V> = p;
                        let mut p = (if (dir<=0) { p.__get_left() } else { p.__get_right() });
                        if _is_jnull(&(if (dir<=0) { p.__get_left() } else { p.__get_right() })) {
                            x.__set_parent(Clone::clone(&xp));
                            if (dir<=0) {
                                xp.__set_left(Clone::clone(&x));
                            } else {
                                xp.__set_right(Clone::clone(&x));
                            }
                            let _t0: HashMap_TreeNode<Object, Object> = HashMap_TreeNode::<Object, Object>::balanceInsertion(Clone::clone(&root), Clone::clone(&x))?;
                            let mut root: HashMap_TreeNode<Object, Object> = _t0;
                        } else {
                            continue;
                        }
                    }
                }
                let mut x: HashMap_TreeNode<Object, Object> = next;
            }
            HashMap_TreeNode::<Object, Object>::moveRootToFront(Clone::clone(&tab), Clone::clone(&root))?;
            Ok(())
        }

        #[java_method(name = "untreeify", descriptor = "(Ljava/util/HashMap;)Ljava/util/HashMap$Node;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap<TK;TV;>;)Ljava/util/HashMap$Node<TK;TV;>;")]
        pub fn untreeify(&self, mut map: HashMap<K, V>) -> Result<HashMap_Node<K, V>> {
            let this = self;
            let mut hd: HashMap_Node<K, V> = Default::default();
            let mut tl: HashMap_Node<K, V> = Default::default();
            let mut q = this.clone();
            loop {
                if _is_jnull(&q) { break; }
                let _t0 = map.replacementNode(Clone::clone(&q).into(), Default::default())?;
                let mut p: HashMap_Node<Object, Object> = _t0;
                if _is_jnull(&tl) {
                    let mut hd: HashMap_Node<Object, Object> = p;
                } else {
                    tl.__set_next(Clone::clone(&p));
                }
                let mut tl: HashMap_Node<Object, Object> = p;
                let mut q = q.__get_next();
            }
            Ok(Default::default())
        }

        #[java_method(name = "putTreeVal", descriptor = "(Ljava/util/HashMap;[Ljava/util/HashMap$Node;ILjava/lang/Object;Ljava/lang/Object;)Ljava/util/HashMap$TreeNode;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap<TK;TV;>;[Ljava/util/HashMap$Node<TK;TV;>;ITK;TV;)Ljava/util/HashMap$TreeNode<TK;TV;>;")]
        pub fn putTreeVal(&self, mut map: HashMap<K, V>, mut tab: Rc<RefCell<Vec<HashMap_Node<K, V>>>>, mut h: i32, mut k: K, mut v: V) -> Result<HashMap_TreeNode<K, V>> {
            let this = self;
            let mut kc: Class<Object> = Default::default();
            let mut searched: i32 = 0i32;
            let mut _merged1: HashMap_TreeNode<Object, Object>;
            if !_is_jnull(&this.__get_parent()) {
                let _t0 = this.root()?;
                _merged1 = _t0;
            } else {
                _merged1 = Clone::clone(this);
            }
            let mut root: HashMap_TreeNode<Object, Object> = _merged1;
            let mut p: HashMap_TreeNode<Object, Object> = root;
            let mut dir: i32 = Default::default();
            loop {
                let mut ph = p.__get_hash();
                if p.__get_hash() > h {
                    dir = -1i32;
                } else {
                    if ph < h {
                        let mut dir: i32 = 1i32;
                    } else {
                        let mut pk = p.__get_key();
                        let _t2 = k.equals(Clone::clone(&pk))?;
                        if _t2 {
                            return Ok(Default::default());
                        }
                        let _t3: Object = HashMap::<Object, Object>::comparableClassFor(Clone::clone(&k))?;
                        kc = _t3;
                        let _t4: i32 = HashMap::<Object, Object>::compareComparables(Clone::clone(&kc), Clone::clone(&k), Clone::clone(&pk))?;
                        let mut dir: i32 = _t4;
                        searched = 1i32;
                        let mut ch = p.__get_left();
                        let _t5 = ch.find(h, Clone::clone(&k), Clone::clone(&kc))?;
                        let mut q: HashMap_TreeNode<Object, Object> = _t5;
                        ch = p.__get_right();
                        let _t6 = ch.find(h, Clone::clone(&k), Clone::clone(&kc))?;
                        q = _t6;
                        if !_is_jnull(&q) {
                            return Ok(Default::default());
                        }
                        let _t7: i32 = HashMap_TreeNode::<Object, Object>::tieBreakOrder(Clone::clone(&k), Clone::clone(&pk))?;
                        dir = _t7;
                    }
                }
                let mut q: HashMap_TreeNode<Object, Object> = p;
                p = (if (dir<=0) { p.__get_left() } else { p.__get_right() });
                let mut ch = q.__get_next();
                let _t2 = map.newTreeNode(h, Clone::clone(&k), Clone::clone(&v), Clone::clone(&ch))?;
                let mut x: HashMap_TreeNode<Object, Object> = _t2;
                if (dir<=0) {
                    q.__set_left(Clone::clone(&x));
                } else {
                    q.__set_right(Clone::clone(&x));
                }
                q.__set_next(Clone::clone(&<_ as Into<HashMap_Node<Object, Object>>>::into(x)));
                x.__set_prev(Clone::clone(&q));
                x.__set_parent(Clone::clone(&q));
                if !_is_jnull(&ch) {
                    Default::default().__set_prev(Clone::clone(&x));
                }
                let _t3: HashMap_TreeNode<Object, Object> = HashMap_TreeNode::<Object, Object>::balanceInsertion(Clone::clone(&root), Clone::clone(&x))?;
                HashMap_TreeNode::<Object, Object>::moveRootToFront(Clone::clone(&tab), Clone::clone(&_t3))?;
                return Ok(Default::default());
            }
        }

        #[java_method(name = "removeTreeNode", descriptor = "(Ljava/util/HashMap;[Ljava/util/HashMap$Node;Z)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap<TK;TV;>;[Ljava/util/HashMap$Node<TK;TV;>;Z)V")]
        pub fn removeTreeNode(&self, map: HashMap<K, V>, tab: Rc<RefCell<Vec<HashMap_Node<K, V>>>>, movable: bool) -> Result<()> {
            panic!("stub: java/util/HashMap$TreeNode.removeTreeNode:(Ljava/util/HashMap;[Ljava/util/HashMap$Node;Z)V")
        }

        #[java_method(name = "split", descriptor = "(Ljava/util/HashMap;[Ljava/util/HashMap$Node;II)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap<TK;TV;>;[Ljava/util/HashMap$Node<TK;TV;>;II)V")]
        pub fn split(&self, mut map: HashMap<K, V>, mut tab: Rc<RefCell<Vec<HashMap_Node<K, V>>>>, mut index: i32, mut bit: i32) -> Result<()> {
            let this = self;
            let mut b = this.clone();
            let mut loHead: HashMap_TreeNode<K, V> = Default::default();
            let mut loTail: HashMap_TreeNode<K, V> = Default::default();
            let mut hiHead: HashMap_TreeNode<K, V> = Default::default();
            let mut hiTail: HashMap_TreeNode<K, V> = Default::default();
            let mut lc: i32 = 0i32;
            let mut hc: i32 = 0i32;
            let mut e: HashMap_TreeNode = b;
            let mut hiHead: HashMap_TreeNode = Default::default();
            let mut loHead: HashMap_TreeNode = Default::default();
            loop {
                if _is_jnull(&e) { break; }
                let mut next: HashMap_TreeNode<Object, Object> = Default::default();
                e.__set_next(Default::default());
                if ((e.__get_hash()&bit)==0) {
                    e.__set_prev(Clone::clone(&loTail));
                    if _is_jnull(&loTail) {
                        loHead = e;
                    } else {
                        loTail.__set_next(Clone::clone(&<_ as Into<HashMap_Node<Object, Object>>>::into(e)));
                    }
                    let mut loTail: HashMap_TreeNode = e;
                    lc = lc.wrapping_add(1i32);
                } else {
                    e.__set_prev(Clone::clone(&hiTail));
                    if _is_jnull(&hiTail) {
                        hiHead = e;
                    } else {
                        hiTail.__set_next(Clone::clone(&<_ as Into<HashMap_Node<Object, Object>>>::into(e)));
                    }
                    let mut hiTail: HashMap_TreeNode = e;
                    hc = hc.wrapping_add(1i32);
                }
                let mut e: HashMap_TreeNode<Object, Object> = next;
            }
            if lc <= 6i32 {
                let _t0 = loHead.untreeify(Clone::clone(&map))?;
                tab.borrow_mut()[index as usize] = Clone::clone(&_t0);
            } else {
                tab.borrow_mut()[index as usize] = Clone::clone(&loHead);
                if !_is_jnull(&hiHead) {
                    loHead.treeify(Clone::clone(&tab))?;
                }
            }
            if hc <= 6i32 {
                let _t0 = hiHead.untreeify(Clone::clone(&map))?;
                tab.borrow_mut()[(index).wrapping_add(bit) as usize] = Clone::clone(&_t0);
            } else {
                tab.borrow_mut()[(index).wrapping_add(bit) as usize] = Clone::clone(&hiHead);
                if !_is_jnull(&loHead) {
                    hiHead.treeify(Clone::clone(&tab))?;
                }
            }
            Ok(())
        }

        #[java_method(name = "rotateLeft", descriptor = "(Ljava/util/HashMap$TreeNode;Ljava/util/HashMap$TreeNode;)Ljava/util/HashMap$TreeNode;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/HashMap$TreeNode<TK;TV;>;Ljava/util/HashMap$TreeNode<TK;TV;>;)Ljava/util/HashMap$TreeNode<TK;TV;>;")]
        pub fn rotateLeft(mut root: HashMap_TreeNode<K, V>, mut p: HashMap_TreeNode<K, V>) -> Result<HashMap_TreeNode<K, V>> {
            let mut r = p.__get_right();
            p.__set_right(Clone::clone(&r.__get_left()));
            let mut rl = r.__get_left();
            if !_is_jnull(&r.__get_left()) {
                rl.__set_parent(Clone::clone(&p));
            }
            r.__set_parent(Clone::clone(&p.__get_parent()));
            let mut pp = p.__get_parent();
            if _is_jnull(&p.__get_parent()) {
                let mut root: HashMap_TreeNode<Object, Object> = r;
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

        #[java_method(name = "rotateRight", descriptor = "(Ljava/util/HashMap$TreeNode;Ljava/util/HashMap$TreeNode;)Ljava/util/HashMap$TreeNode;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/HashMap$TreeNode<TK;TV;>;Ljava/util/HashMap$TreeNode<TK;TV;>;)Ljava/util/HashMap$TreeNode<TK;TV;>;")]
        pub fn rotateRight(mut root: HashMap_TreeNode<K, V>, mut p: HashMap_TreeNode<K, V>) -> Result<HashMap_TreeNode<K, V>> {
            let mut l = p.__get_left();
            p.__set_left(Clone::clone(&l.__get_right()));
            let mut lr = l.__get_right();
            if !_is_jnull(&l.__get_right()) {
                lr.__set_parent(Clone::clone(&p));
            }
            l.__set_parent(Clone::clone(&p.__get_parent()));
            let mut pp = p.__get_parent();
            if _is_jnull(&p.__get_parent()) {
                let mut root: HashMap_TreeNode<Object, Object> = l;
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

        #[java_method(name = "balanceInsertion", descriptor = "(Ljava/util/HashMap$TreeNode;Ljava/util/HashMap$TreeNode;)Ljava/util/HashMap$TreeNode;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/HashMap$TreeNode<TK;TV;>;Ljava/util/HashMap$TreeNode<TK;TV;>;)Ljava/util/HashMap$TreeNode<TK;TV;>;")]
        pub fn balanceInsertion(mut root: HashMap_TreeNode<K, V>, mut x: HashMap_TreeNode<K, V>) -> Result<HashMap_TreeNode<K, V>> {
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
                    let mut x: HashMap_TreeNode<Object, Object> = xpp;
                    continue;
                }
                let mut x: HashMap_TreeNode<Object, Object> = xp;
                let _t0: HashMap_TreeNode<Object, Object> = HashMap_TreeNode::<Object, Object>::rotateLeft(Clone::clone(&root), Clone::clone(&x))?;
                let mut root: HashMap_TreeNode<Object, Object> = _t0;
                xp = x.__get_parent();
                let mut xpp = (if _is_jnull(&x.__get_parent()) { Object::default() } else { Object::from_any(Clone::clone(&xp.__get_parent())) });
                xp.__set_red((0i32 != 0i32));
                xpp.__set_red((1i32 != 0i32));
                let _t1: HashMap_TreeNode<Object, Object> = HashMap_TreeNode::<Object, Object>::rotateRight(Clone::clone(&root), Clone::clone(&xpp))?;
                root = _t1;
                continue;
                if xppl.__get_red() {
                    xppl.__set_red((0i32 != 0i32));
                    xp.__set_red((0i32 != 0i32));
                    xpp.__set_red((1i32 != 0i32));
                    let mut x: HashMap_TreeNode<K, V> = xpp;
                    continue;
                }
                x = xp;
                let _t2: HashMap_TreeNode<Object, Object> = HashMap_TreeNode::<Object, Object>::rotateRight(Clone::clone(&root), Clone::clone(&x))?;
                root = _t2;
                xp = x.__get_parent();
                xpp = (if _is_jnull(&x.__get_parent()) { Object::default() } else { Object::from_any(Clone::clone(&xp.__get_parent())) });
                xp.__set_red((0i32 != 0i32));
                xpp.__set_red((1i32 != 0i32));
                let _t3: HashMap_TreeNode<Object, Object> = HashMap_TreeNode::<Object, Object>::rotateLeft(Clone::clone(&root), Clone::clone(&xpp))?;
                root = _t3;
            }
        }

        #[java_method(name = "balanceDeletion", descriptor = "(Ljava/util/HashMap$TreeNode;Ljava/util/HashMap$TreeNode;)Ljava/util/HashMap$TreeNode;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/HashMap$TreeNode<TK;TV;>;Ljava/util/HashMap$TreeNode<TK;TV;>;)Ljava/util/HashMap$TreeNode<TK;TV;>;")]
        pub fn balanceDeletion(root: HashMap_TreeNode<K, V>, x: HashMap_TreeNode<K, V>) -> Result<HashMap_TreeNode<Object, Object>> {
            panic!("stub: java/util/HashMap$TreeNode.balanceDeletion:(Ljava/util/HashMap$TreeNode;Ljava/util/HashMap$TreeNode;)Ljava/util/HashMap$TreeNode;")
        }

        #[java_method(name = "checkInvariants", descriptor = "(Ljava/util/HashMap$TreeNode;)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/HashMap$TreeNode<TK;TV;>;)Z")]
        pub fn checkInvariants(mut t: HashMap_TreeNode<K, V>) -> Result<bool> {
            let mut tp = t.__get_parent();
            let mut tl = t.__get_left();
            let mut tr = t.__get_right();
            let mut tb = t.__get_prev();
            let mut tn: HashMap_TreeNode<Object, Object> = Default::default();
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
            let _t0: bool = HashMap_TreeNode::<Object, Object>::checkInvariants(Clone::clone(&tl))?;
            if !(_t0) {
                return Ok((0i32 != 0i32));
            }
            let _t1: bool = HashMap_TreeNode::<Object, Object>::checkInvariants(Clone::clone(&tr))?;
            if !(_t1) {
                return Ok((0i32 != 0i32));
            }
            Ok((1i32 != 0i32))
        }
    }
}
