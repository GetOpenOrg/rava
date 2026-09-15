#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/util/HashMap$TreeNode",
    super_class       = "java/util/LinkedHashMap$Entry",
    interfaces        = "",
    access            = "package",
    modifiers         = "final",
    generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/util/LinkedHashMap$Entry<TK;TV;>;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "HashMap.java",
    inner_classes     = "java/util/LinkedHashMap$Entry:java/util/LinkedHashMap:Entry:8;java/util/HashMap$Node:java/util/HashMap:Node:8;java/util/HashMap$TreeNode:java/util/HashMap:TreeNode:24",
    all_supertypes    = "java/lang/Object;java/util/HashMap$Node;java/util/HashMap$TreeNode;java/util/LinkedHashMap$Entry;java/util/Map$Entry",
)]
#[derive(Clone, Default, PartialEq)]
pub struct HashMap_TreeNode<K: Clone + Default + 'static, V: Clone + Default + 'static> {
    pub _super: LinkedHashMap_Entry<K, V>,
    #[cfg_attr(any(), java_field(name = "parent", descriptor = "Ljava/util/HashMap$TreeNode;", is_static = false, generic_signature = "Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
    pub parent: JField<HashMap_TreeNode<K, V>>,
    #[cfg_attr(any(), java_field(name = "left", descriptor = "Ljava/util/HashMap$TreeNode;", is_static = false, generic_signature = "Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
    pub left: JField<HashMap_TreeNode<K, V>>,
    #[cfg_attr(any(), java_field(name = "right", descriptor = "Ljava/util/HashMap$TreeNode;", is_static = false, generic_signature = "Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
    pub right: JField<HashMap_TreeNode<K, V>>,
    #[cfg_attr(any(), java_field(name = "prev", descriptor = "Ljava/util/HashMap$TreeNode;", is_static = false, generic_signature = "Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
    pub prev: JField<HashMap_TreeNode<K, V>>,
    #[cfg_attr(any(), java_field(name = "red", descriptor = "Z", is_static = false))]
    pub red: JField<bool>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> HashMap_TreeNode<K, V> {
    pub fn as_linked_hash_map_entry(&self) -> &LinkedHashMap_Entry<K, V> { &self._super }
    pub fn into_linked_hash_map_entry(self) -> LinkedHashMap_Entry<K, V> { self._super }
    pub fn as_hash_map_node(&self) -> &HashMap_Node<K, V> { &self._super._super }
    pub fn into_hash_map_node(self) -> HashMap_Node<K, V> { self._super._super }
}

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<HashMap_TreeNode<K, V>> for LinkedHashMap_Entry<K, V> {
    fn from(v: HashMap_TreeNode<K, V>) -> LinkedHashMap_Entry<K, V> { v._super }
}

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<HashMap_TreeNode<K, V>> for HashMap_Node<K, V> {
    fn from(v: HashMap_TreeNode<K, V>) -> HashMap_Node<K, V> { v._super._super }
}

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> HashMap_TreeNode<K, V> {
    #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
    // static field: $assertionsDisabled:Z
    pub fn _assertionsDisabled() -> bool {
        false
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;Ljava/util/HashMap$Node<TK;TV;>;)V"))]
    pub fn new(mut hash: i32, mut key: K, mut val: V, mut next: HashMap_Node<Object, Object>) -> Result<Self> {
        let mut this = Self { _super: Default::default(), parent: JField::new(Default::default()), left: JField::new(Default::default()), right: JField::new(Default::default()), prev: JField::new(Default::default()), red: JField::new(false), _phantom: std::marker::PhantomData, ..Default::default() };
        this._super = LinkedHashMap_Entry::new(hash, Clone::clone(&key), Clone::clone(&val), Clone::clone(&next))?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "root", descriptor = "()Ljava/util/HashMap$TreeNode;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
    pub fn root(&self) -> Result<HashMap_TreeNode<Object, Object>> {
        let this = self;
        let mut r = this.clone();
        loop {
            let mut p = r.parent.get();
            if _is_jnull(&r.parent.get()) {
                return Ok(Default::default());
            }
            let mut r: HashMap_TreeNode<Object, Object> = p;
        }
    }

    #[cfg_attr(any(), java_method(name = "moveRootToFront", descriptor = "([Ljava/util/HashMap$Node;Ljava/util/HashMap$TreeNode;)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>([Ljava/util/HashMap$Node<TK;TV;>;Ljava/util/HashMap$TreeNode<TK;TV;>;)V"))]
    pub fn moveRootToFront(mut tab: Rc<RefCell<Vec<HashMap_Node<Object, Object>>>>, mut root: HashMap_TreeNode<Object, Object>) -> Result<()> {
        let mut n = (tab.borrow().len() as i32);
        let mut index = ((n).wrapping_sub(1i32)&root._super._super.hash.get());
        let mut first: HashMap_TreeNode<Object, Object> = Default::default();
        tab.borrow_mut()[index as usize] = Clone::clone(&root);
        let mut rp = root.prev.get();
        let mut rn = root._super._super.next.get();
        if !_is_jnull(&root._super._super.next.get()) {
            Default::default().prev.set(Clone::clone(&rp));
        }
        if !_is_jnull(&rp) {
            rp._super._super.next.set(Clone::clone(&rn));
        }
        if !_is_jnull(&first) {
            first.prev.set(Clone::clone(&root));
        }
        root._super._super.next.set(Clone::clone(&first));
        root.prev.set(Default::default());
        let _t0: bool = HashMap_TreeNode::<Object, Object>::checkInvariants(Clone::clone(&root))?;
        if !(_t0) {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "find", descriptor = "(ILjava/lang/Object;Ljava/lang/Class;)Ljava/util/HashMap$TreeNode;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/lang/Object;Ljava/lang/Class<*>;)Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
    pub fn find(&self, mut h: i32, mut k: Object, mut kc: Object) -> Result<HashMap_TreeNode<Object, Object>> {
        let this = self;
        let mut p = this.clone();
        let mut p: HashMap_TreeNode<Object, Object> = Default::default();
        loop {
            let mut pl = p.left.get();
            let mut pr = p.right.get();
            let mut ph = p._super._super.hash.get();
            if p._super._super.hash.get() > h {
                p = pl;
            } else {
                if ph < h {
                    let mut p: HashMap_TreeNode<Object, Object> = pr;
                } else {
                    let mut pk = p._super._super.key.get();
                    let _t0 = k.equals(Clone::clone(&pk))?;
                    if _t0 {
                        return Ok(Default::default());
                    }
                    if _is_jnull(&pl) {
                        let mut p: HashMap_TreeNode<Object, Object> = pr;
                    } else {
                        if _is_jnull(&pr) {
                            let mut p: HashMap_TreeNode<Object, Object> = pl;
                        } else {
                            let _t1: Object = HashMap::<Object, Object>::comparableClassFor(Clone::clone(&k))?;
                            let mut kc = (_t1).downcast::<Class<Object>>();
                            if !_is_jnull(&kc) {
                                let _t2: i32 = HashMap::<Object, Object>::compareComparables(Clone::clone(&kc), Clone::clone(&k), Clone::clone(&pk))?;
                                let mut dir: i32 = _t2;
                                if (dir!=0) {
                                    let mut p = (if (dir<0) { pl } else { pr });
                                } else {
                                    let _t3 = pr.find(h, Clone::clone(&k), Clone::clone(&kc))?;
                                    let mut q: HashMap_TreeNode<Object, Object> = _t3;
                                    if !_is_jnull(&q) {
                                        return Ok(q);
                                    }
                                    let mut p: HashMap_TreeNode<Object, Object> = pl;
                                }
                            } else {
                                let _t2 = pr.find(h, Clone::clone(&k), Clone::clone(&kc))?;
                                let mut q: HashMap_TreeNode<Object, Object> = _t2;
                                if !_is_jnull(&q) {
                                    return Ok(q);
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

    #[cfg_attr(any(), java_method(name = "getTreeNode", descriptor = "(ILjava/lang/Object;)Ljava/util/HashMap$TreeNode;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/lang/Object;)Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
    pub fn getTreeNode(&self, mut h: i32, mut k: Object) -> Result<HashMap_TreeNode<Object, Object>> {
        let this = self;
        let mut _merged1: HashMap_TreeNode<Object, Object>;
        if !_is_jnull(&this.parent.get()) {
            let _t0 = this.root()?;
            _merged1 = _t0;
        } else {
            _merged1 = Clone::clone(this);
        }
        let _t2 = _merged1.find(h, Clone::clone(&k), Default::default())?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "tieBreakOrder", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn tieBreakOrder(mut a: Object, mut b: Object) -> Result<i32> {
        let _t0 = a.getClass()?;
        let _t1: String = Default::default();
        let _t2 = b.getClass()?;
        let _t3: String = Default::default();
        let _t4 = _t1.compareTo(Clone::clone(&_t3))?;
        let mut d: i32 = _t4;
        let _t5: i32 = System::identityHashCode(Clone::clone(&a))?;
        let _t6: i32 = System::identityHashCode(Clone::clone(&b))?;
        d = (_t5 > _t6) as i32;
        Ok(d)
    }

    #[cfg_attr(any(), java_method(name = "treeify", descriptor = "([Ljava/util/HashMap$Node;)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([Ljava/util/HashMap$Node<TK;TV;>;)V"))]
    pub fn treeify(&self, mut tab: Rc<RefCell<Vec<HashMap_Node<Object, Object>>>>) -> Result<()> {
        let this = self;
        let mut root: HashMap_TreeNode<K, V> = Default::default();
        let mut x = this.clone();
        let mut root: HashMap_TreeNode = Default::default();
        loop {
            if _is_jnull(&x) { break; }
            let mut next: HashMap_TreeNode<Object, Object> = Default::default();
            x.right.set(Default::default());
            x.left.set(Default::default());
    let mut dir: i32 = Default::default();
            if _is_jnull(&root) {
                x.parent.set(Default::default());
                x.red.set((0i32 != 0i32));
                root = x;
            } else {
                let mut k = x._super._super.key.get();
                let mut h = x._super._super.hash.get();
                let mut kc: Class<Object> = Default::default();
                let mut p: HashMap_TreeNode<K, V> = root;
                dir = Default::default();
                loop {
                    let mut pk = p._super._super.key.get();
                    let mut ph = p._super._super.hash.get();
                    if p._super._super.hash.get() > h {
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
                    let mut p = (if (dir<=0) { p.left.get() } else { p.right.get() });
                    if _is_jnull(&(if (dir<=0) { p.left.get() } else { p.right.get() })) {
                        x.parent.set(Clone::clone(&xp));
                        if (dir<=0) {
                            xp.left.set(Clone::clone(&x));
                        } else {
                            xp.right.set(Clone::clone(&x));
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
        HashMap_TreeNode::<Object, Object>::moveRootToFront(Default::default(), Clone::clone(&root))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "untreeify", descriptor = "(Ljava/util/HashMap;)Ljava/util/HashMap$Node;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap<TK;TV;>;)Ljava/util/HashMap$Node<TK;TV;>;"))]
    pub fn untreeify(&self, mut map: HashMap<Object, Object>) -> Result<HashMap_Node<Object, Object>> {
        let this = self;
        let mut hd: HashMap_Node<K, V> = Default::default();
        let mut tl: HashMap_Node<K, V> = Default::default();
        let mut q = this.clone();
        let mut hd: HashMap_Node<Object, Object> = Default::default();
        loop {
            if _is_jnull(&q) { break; }
            let _t0 = map.replacementNode(Clone::clone(&q).into(), Default::default())?;
            let mut p: HashMap_Node<Object, Object> = _t0;
            if _is_jnull(&tl) {
                hd = p;
            } else {
                tl.next.set(Clone::clone(&p));
            }
            let mut tl: HashMap_Node<Object, Object> = p;
            let mut q = q._super._super.next.get();
        }
        Ok(hd)
    }

    #[cfg_attr(any(), java_method(name = "putTreeVal", descriptor = "(Ljava/util/HashMap;[Ljava/util/HashMap$Node;ILjava/lang/Object;Ljava/lang/Object;)Ljava/util/HashMap$TreeNode;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap<TK;TV;>;[Ljava/util/HashMap$Node<TK;TV;>;ITK;TV;)Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
    pub fn putTreeVal(&self, mut map: HashMap<Object, Object>, mut tab: Rc<RefCell<Vec<HashMap_Node<Object, Object>>>>, mut h: i32, mut k: K, mut v: V) -> Result<HashMap_TreeNode<Object, Object>> {
        let this = self;
        let mut kc: Class<Object> = Default::default();
        let mut searched: i32 = 0i32;
        let mut _merged1: HashMap_TreeNode<Object, Object>;
        if !_is_jnull(&this.parent.get()) {
            let _t0 = this.root()?;
            _merged1 = _t0;
        } else {
            _merged1 = Clone::clone(this);
        }
        let mut root: HashMap_TreeNode<Object, Object> = _merged1;
        let mut p: HashMap_TreeNode<Object, Object> = root;
        let mut dir: i32 = Default::default();
        loop {
            let mut ph = p._super._super.hash.get();
            if p._super._super.hash.get() > h {
                dir = -1i32;
            } else {
                if ph < h {
                    let mut dir: i32 = 1i32;
                } else {
                    let mut pk = p._super._super.key.get();
                    let _t2 = k.equals(Clone::clone(&pk))?;
                    if _t2 {
                        return Ok(p);
                    }
                    let _t3: Object = HashMap::<Object, Object>::comparableClassFor(Clone::clone(&k))?;
                    kc = _t3;
                    let _t4: i32 = HashMap::<Object, Object>::compareComparables(Clone::clone(&kc), Clone::clone(&k), Clone::clone(&pk))?;
                    let mut dir: i32 = _t4;
                    searched = 1i32;
                    let mut ch = p.left.get();
                    let _t5 = ch.find(h, Clone::clone(&k), Clone::clone(&kc))?;
                    let mut q: HashMap_TreeNode<Object, Object> = _t5;
                    ch = p.right.get();
                    let _t6 = ch.find(h, Clone::clone(&k), Clone::clone(&kc))?;
                    q = _t6;
                    if !_is_jnull(&q) {
                        return Ok(q);
                    }
                    let _t7: i32 = HashMap_TreeNode::<Object, Object>::tieBreakOrder(Clone::clone(&k), Clone::clone(&pk))?;
                    dir = _t7;
                }
            }
            let mut q: HashMap_TreeNode<Object, Object> = p;
            p = (if (dir<=0) { p.left.get() } else { p.right.get() });
            let mut ch = q._super._super.next.get();
            let _t2 = map.newTreeNode(h, Clone::clone(&k), Clone::clone(&v), Clone::clone(&ch))?;
            let mut x: HashMap_TreeNode<Object, Object> = _t2;
            if (dir<=0) {
                q.left.set(Clone::clone(&x));
            } else {
                q.right.set(Clone::clone(&x));
            }
            q._super._super.next.set(Clone::clone(&x));
            x.prev.set(Clone::clone(&q));
            x.parent.set(Clone::clone(&q));
            if !_is_jnull(&ch) {
                Default::default().prev.set(Clone::clone(&x));
            }
            let _t3: HashMap_TreeNode<Object, Object> = HashMap_TreeNode::<Object, Object>::balanceInsertion(Clone::clone(&root), Clone::clone(&x))?;
            HashMap_TreeNode::<Object, Object>::moveRootToFront(Default::default(), Clone::clone(&_t3))?;
            return Ok(Default::default());
        }
    }

    #[cfg_attr(any(), java_method(name = "removeTreeNode", descriptor = "(Ljava/util/HashMap;[Ljava/util/HashMap$Node;Z)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap<TK;TV;>;[Ljava/util/HashMap$Node<TK;TV;>;Z)V"))]
    pub fn removeTreeNode(&self, map: HashMap<Object, Object>, tab: Rc<RefCell<Vec<HashMap_Node<Object, Object>>>>, movable: bool) -> Result<()> {
        panic!("stub: java/util/HashMap$TreeNode.removeTreeNode:(Ljava/util/HashMap;[Ljava/util/HashMap$Node;Z)V")
    }

    #[cfg_attr(any(), java_method(name = "split", descriptor = "(Ljava/util/HashMap;[Ljava/util/HashMap$Node;II)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashMap<TK;TV;>;[Ljava/util/HashMap$Node<TK;TV;>;II)V"))]
    pub fn split(&self, mut map: HashMap<Object, Object>, mut tab: Rc<RefCell<Vec<HashMap_Node<Object, Object>>>>, mut index: i32, mut bit: i32) -> Result<()> {
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
            e._super._super.next.set(Default::default());
            if ((e._super._super.hash.get()&bit)==0) {
                e.prev.set(Clone::clone(&loTail));
                if _is_jnull(&loTail) {
                    loHead = e;
                } else {
                    loTail._super._super.next.set(Clone::clone(&e));
                }
                let mut loTail: HashMap_TreeNode = e;
                lc = lc.wrapping_add(1i32);
            } else {
                e.prev.set(Clone::clone(&hiTail));
                if _is_jnull(&hiTail) {
                    hiHead = e;
                } else {
                    hiTail._super._super.next.set(Clone::clone(&e));
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
                loHead.treeify(Default::default())?;
            }
        }
        if hc <= 6i32 {
            let _t0 = hiHead.untreeify(Clone::clone(&map))?;
            tab.borrow_mut()[(index).wrapping_add(bit) as usize] = Clone::clone(&_t0);
        } else {
            tab.borrow_mut()[(index).wrapping_add(bit) as usize] = Clone::clone(&hiHead);
            if !_is_jnull(&loHead) {
                hiHead.treeify(Default::default())?;
            }
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "rotateLeft", descriptor = "(Ljava/util/HashMap$TreeNode;Ljava/util/HashMap$TreeNode;)Ljava/util/HashMap$TreeNode;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/HashMap$TreeNode<TK;TV;>;Ljava/util/HashMap$TreeNode<TK;TV;>;)Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
    pub fn rotateLeft(mut root: HashMap_TreeNode<Object, Object>, mut p: HashMap_TreeNode<Object, Object>) -> Result<HashMap_TreeNode<Object, Object>> {
        let mut r = p.right.get();
        p.right.set(Clone::clone(&r.left.get()));
        let mut rl = r.left.get();
        if !_is_jnull(&r.left.get()) {
            rl.parent.set(Clone::clone(&p));
        }
        r.parent.set(Clone::clone(&p.parent.get()));
        let mut pp = p.parent.get();
        if _is_jnull(&p.parent.get()) {
            root = r;
            root.red.set((0i32 != 0i32));
        } else {
            if Object::from_any(pp.left.get().clone()) == Object::from_any(p.clone()) {
                pp.left.set(Clone::clone(&r));
            } else {
                pp.right.set(Clone::clone(&r));
            }
        }
        r.left.set(Clone::clone(&p));
        p.parent.set(Clone::clone(&r));
        Ok(root)
    }

    #[cfg_attr(any(), java_method(name = "rotateRight", descriptor = "(Ljava/util/HashMap$TreeNode;Ljava/util/HashMap$TreeNode;)Ljava/util/HashMap$TreeNode;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/HashMap$TreeNode<TK;TV;>;Ljava/util/HashMap$TreeNode<TK;TV;>;)Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
    pub fn rotateRight(mut root: HashMap_TreeNode<Object, Object>, mut p: HashMap_TreeNode<Object, Object>) -> Result<HashMap_TreeNode<Object, Object>> {
        let mut l = p.left.get();
        p.left.set(Clone::clone(&l.right.get()));
        let mut lr = l.right.get();
        if !_is_jnull(&l.right.get()) {
            lr.parent.set(Clone::clone(&p));
        }
        l.parent.set(Clone::clone(&p.parent.get()));
        let mut pp = p.parent.get();
        if _is_jnull(&p.parent.get()) {
            root = l;
            root.red.set((0i32 != 0i32));
        } else {
            if Object::from_any(pp.right.get().clone()) == Object::from_any(p.clone()) {
                pp.right.set(Clone::clone(&l));
            } else {
                pp.left.set(Clone::clone(&l));
            }
        }
        l.right.set(Clone::clone(&p));
        p.parent.set(Clone::clone(&l));
        Ok(root)
    }

    #[cfg_attr(any(), java_method(name = "balanceInsertion", descriptor = "(Ljava/util/HashMap$TreeNode;Ljava/util/HashMap$TreeNode;)Ljava/util/HashMap$TreeNode;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/HashMap$TreeNode<TK;TV;>;Ljava/util/HashMap$TreeNode<TK;TV;>;)Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
    pub fn balanceInsertion(mut root: HashMap_TreeNode<Object, Object>, mut x: HashMap_TreeNode<Object, Object>) -> Result<HashMap_TreeNode<Object, Object>> {
        x.red.set((1i32 != 0i32));
        loop {
            let mut xp = x.parent.get();
            if _is_jnull(&x.parent.get()) {
                x.red.set((0i32 != 0i32));
                return Ok(x);
            }
            let mut xpp = xp.parent.get();
            if _is_jnull(&xp.parent.get()) {
                return Ok(root);
            }
            let mut xppl = xpp.left.get();
            let mut xppr = xpp.right.get();
            if xppr.red.get() {
                xppr.red.set((0i32 != 0i32));
                xp.red.set((0i32 != 0i32));
                xpp.red.set((1i32 != 0i32));
                x = xpp;
                continue;
            }
            x = xp;
            let _t0: HashMap_TreeNode<Object, Object> = HashMap_TreeNode::<Object, Object>::rotateLeft(Clone::clone(&root), Clone::clone(&x))?;
            root = _t0;
            xp = x.parent.get();
            let mut xpp = (if _is_jnull(&x.parent.get()) { Object::default() } else { Object::from_any(Clone::clone(&xp.parent.get())) });
            xp.red.set((0i32 != 0i32));
            xpp.red.set((1i32 != 0i32));
            let _t1: HashMap_TreeNode<Object, Object> = HashMap_TreeNode::<Object, Object>::rotateRight(Clone::clone(&root), Clone::clone(&xpp))?;
            root = _t1;
            continue;
            if xppl.red.get() {
                xppl.red.set((0i32 != 0i32));
                xp.red.set((0i32 != 0i32));
                xpp.red.set((1i32 != 0i32));
                let mut x: HashMap_TreeNode<K, V> = xpp;
                continue;
            }
            x = xp;
            let _t2: HashMap_TreeNode<Object, Object> = HashMap_TreeNode::<Object, Object>::rotateRight(Clone::clone(&root), Clone::clone(&x))?;
            root = _t2;
            xp = x.parent.get();
            xpp = (if _is_jnull(&x.parent.get()) { Object::default() } else { Object::from_any(Clone::clone(&xp.parent.get())) });
            xp.red.set((0i32 != 0i32));
            xpp.red.set((1i32 != 0i32));
            let _t3: HashMap_TreeNode<Object, Object> = HashMap_TreeNode::<Object, Object>::rotateLeft(Clone::clone(&root), Clone::clone(&xpp))?;
            root = _t3;
        }
    }

    #[cfg_attr(any(), java_method(name = "balanceDeletion", descriptor = "(Ljava/util/HashMap$TreeNode;Ljava/util/HashMap$TreeNode;)Ljava/util/HashMap$TreeNode;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/HashMap$TreeNode<TK;TV;>;Ljava/util/HashMap$TreeNode<TK;TV;>;)Ljava/util/HashMap$TreeNode<TK;TV;>;"))]
    pub fn balanceDeletion(root: HashMap_TreeNode<Object, Object>, x: HashMap_TreeNode<Object, Object>) -> Result<HashMap_TreeNode<Object, Object>> {
        panic!("stub: java/util/HashMap$TreeNode.balanceDeletion:(Ljava/util/HashMap$TreeNode;Ljava/util/HashMap$TreeNode;)Ljava/util/HashMap$TreeNode;")
    }

    #[cfg_attr(any(), java_method(name = "checkInvariants", descriptor = "(Ljava/util/HashMap$TreeNode;)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/HashMap$TreeNode<TK;TV;>;)Z"))]
    pub fn checkInvariants(mut t: HashMap_TreeNode<Object, Object>) -> Result<bool> {
        let mut tp = t.parent.get();
        let mut tl = t.left.get();
        let mut tr = t.right.get();
        let mut tb = t.prev.get();
        let mut tn: HashMap_TreeNode<Object, Object> = Default::default();
        if Object::from_any(tb._super._super.next.get().clone()) != Object::from_any(t.clone()) {
            return Ok((0i32 != 0i32));
        }
        if Object::from_any(tn.prev.get().clone()) != Object::from_any(t.clone()) {
            return Ok((0i32 != 0i32));
        }
        if Object::from_any(t.clone()) != Object::from_any(tp.right.get().clone()) {
            return Ok((0i32 != 0i32));
        }
        if tl._super._super.hash.get() > t._super._super.hash.get() {
            return Ok((0i32 != 0i32));
        }
        if tr._super._super.hash.get() < t._super._super.hash.get() {
            return Ok((0i32 != 0i32));
        }
        if tr.red.get() {
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
