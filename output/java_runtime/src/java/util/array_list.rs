#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;
use crate::jdk::internal::util::ArraysSupport;

#[java_rta_macros::java_class(
    binary_name       = "java/util/ArrayList",
    super_class       = "java/util/AbstractList",
    interfaces        = "java/util/List,java/util/RandomAccess,java/lang/Cloneable,java/io/Serializable",
    access            = "public",
    modifiers         = "",
    generic_signature = "<E:Ljava/lang/Object;>Ljava/util/AbstractList<TE;>;Ljava/util/List<TE;>;Ljava/util/RandomAccess;Ljava/lang/Cloneable;Ljava/io/Serializable;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "ArrayList.java",
    inner_classes     = "java/util/ArrayList$ListItr:java/util/ArrayList:ListItr:2;java/util/ArrayList$Itr:java/util/ArrayList:Itr:2;java/util/ArrayList$SubList:java/util/ArrayList:SubList:10;java/util/ArrayList$ArrayListSpliterator:java/util/ArrayList:ArrayListSpliterator:16;java/util/ArrayList$SubList$2:::0;java/util/ArrayList$SubList$1:::0",
    all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Object;java/util/AbstractCollection;java/util/AbstractList;java/util/ArrayList;java/util/Collection;java/util/List;java/util/RandomAccess;java/util/SequencedCollection",
    has_hash_code_method = true,
)]
#[derive(Clone, Default, PartialEq)]
pub struct ArrayList<E: Clone + Default + 'static> {
    pub _super: AbstractList<E>,
    #[cfg_attr(any(), java_field(name = "elementData", descriptor = "[Ljava/lang/Object;", access = "package", modifiers = "transient", is_static = false))]
    pub elementData: JField<Rc<RefCell<Vec<Object>>>>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "private", modifiers = "", is_static = false))]
    pub size: JField<i32>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + Default + 'static> ArrayList<E> {
    pub fn as_abstract_list(&self) -> &AbstractList<E> { &self._super }
    pub fn into_abstract_list(self) -> AbstractList<E> { self._super }
    pub fn as_abstract_collection(&self) -> &AbstractCollection<E> { &self._super._super }
    pub fn into_abstract_collection(self) -> AbstractCollection<E> { self._super._super }
}

impl<E: Clone + Default + 'static> From<ArrayList<E>> for AbstractList<E> {
    fn from(v: ArrayList<E>) -> AbstractList<E> { v._super }
}

impl<E: Clone + Default + 'static> From<ArrayList<E>> for AbstractCollection<E> {
    fn from(v: ArrayList<E>) -> AbstractCollection<E> { v._super._super }
}

impl<E: Clone + Default + 'static> ArrayList<E> {
    #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "8683452581122892189"))]
    // static field: serialVersionUID:J
    pub fn serialVersionUID() -> i64 {
        8683452581122892189i64
    }

    #[cfg_attr(any(), java_field(name = "DEFAULT_CAPACITY", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "10"))]
    // static field: DEFAULT_CAPACITY:I
    pub fn DEFAULT_CAPACITY() -> i32 {
        10
    }

    #[cfg_attr(any(), java_field(name = "EMPTY_ELEMENTDATA", descriptor = "[Ljava/lang/Object;", access = "private", modifiers = "static final", is_static = true))]
    // static field: EMPTY_ELEMENTDATA:[Ljava/lang/Object;
    pub fn EMPTY_ELEMENTDATA() -> Rc<RefCell<Vec<Object>>> {
        Rc::new(RefCell::new(Vec::new()))
    }

    #[cfg_attr(any(), java_field(name = "DEFAULTCAPACITY_EMPTY_ELEMENTDATA", descriptor = "[Ljava/lang/Object;", access = "private", modifiers = "static final", is_static = true))]
    // static field: DEFAULTCAPACITY_EMPTY_ELEMENTDATA:[Ljava/lang/Object;
    pub fn DEFAULTCAPACITY_EMPTY_ELEMENTDATA() -> Rc<RefCell<Vec<Object>>> {
        Rc::new(RefCell::new(Vec::new()))
    }

    #[java_rta_macros::java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn new_i(initialCapacity: i32) -> Result<Self> {
        panic!("stub: java/util/ArrayList.<init>:(I)V")
    }

    #[java_rta_macros::java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let mut this = Self { _super: Default::default(), elementData: JField::new(Default::default()), size: JField::new(0), _phantom: std::marker::PhantomData, ..Default::default() };
        this._super = AbstractList::new()?;
        this.elementData.set(Clone::clone(&ArrayList::<Object>::DEFAULTCAPACITY_EMPTY_ELEMENTDATA()));
        Ok(this)
    }

    #[java_rta_macros::java_method(name = "<init>", descriptor = "(Ljava/util/Collection;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<+TE;>;)V")]
    pub fn new_coll(c: Object) -> Result<Self> {
        panic!("stub: java/util/ArrayList.<init>:(Ljava/util/Collection;)V")
    }

    #[java_rta_macros::java_method(name = "trimToSize", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn trimToSize(&self) -> Result<()> {
        panic!("stub: java/util/ArrayList.trimToSize:()V")
    }

    #[java_rta_macros::java_method(name = "ensureCapacity", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn ensureCapacity(&self, minCapacity: i32) -> Result<()> {
        panic!("stub: java/util/ArrayList.ensureCapacity:(I)V")
    }

    #[java_rta_macros::java_method(name = "grow", descriptor = "(I)[Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    // java: grow(I)[Ljava/lang/Object;
    pub fn grow_i(&self, mut minCapacity: i32) -> Result<Rc<RefCell<Vec<Object>>>> {
        let this = self;
        let mut oldCapacity = (this.elementData.get().borrow().len() as i32);
        if Object::from_any(this.elementData.get().clone()) != Object::from_any(ArrayList::<Object>::DEFAULTCAPACITY_EMPTY_ELEMENTDATA().clone()) {
            let _t0: i32 = ArraysSupport::newLength(oldCapacity, (minCapacity).wrapping_sub(oldCapacity), (oldCapacity>>((1i32&0x1f))))?;
            let mut newCapacity: i32 = _t0;
            let _t1: Rc<RefCell<Vec<Object>>> = Arrays::copyOf_arr_obj_i(Clone::clone(&this.elementData.get()), newCapacity)?;
            this.elementData.set(Clone::clone(&_t1));
            return Ok(_t1);
        }
        let _t0: i32 = Math::max_i_i(10i32, minCapacity)?;
        let mut _arr1: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); _t0 as usize]));
        this.elementData.set(Clone::clone(&_arr1));
        Ok(_arr1)
    }

    #[java_rta_macros::java_method(name = "grow", descriptor = "()[Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    // java: grow()[Ljava/lang/Object;
    pub fn grow(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        let this = self;
        let _t0 = this.grow_i((this.size.get()).wrapping_add(1i32))?;
        Ok(_t0)
    }

    #[java_rta_macros::java_method(name = "size", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok(this.size.get())
    }

    #[java_rta_macros::java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/ArrayList.isEmpty:()Z")
    }

    #[java_rta_macros::java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn contains(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/ArrayList.contains:(Ljava/lang/Object;)Z")
    }

    #[java_rta_macros::java_method(name = "indexOf", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/ArrayList.indexOf:(Ljava/lang/Object;)I")
    }

    #[java_rta_macros::java_method(name = "indexOfRange", descriptor = "(Ljava/lang/Object;II)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn indexOfRange(&self, o: Object, start: i32, end: i32) -> Result<i32> {
        panic!("stub: java/util/ArrayList.indexOfRange:(Ljava/lang/Object;II)I")
    }

    #[java_rta_macros::java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/ArrayList.lastIndexOf:(Ljava/lang/Object;)I")
    }

    #[java_rta_macros::java_method(name = "lastIndexOfRange", descriptor = "(Ljava/lang/Object;II)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn lastIndexOfRange(&self, o: Object, start: i32, end: i32) -> Result<i32> {
        panic!("stub: java/util/ArrayList.lastIndexOfRange:(Ljava/lang/Object;II)I")
    }

    #[java_rta_macros::java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn clone(&self) -> Result<Object> {
        panic!("stub: java/util/ArrayList.clone:()Ljava/lang/Object;")
    }

    #[java_rta_macros::java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn toArray(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/ArrayList.toArray:()[Ljava/lang/Object;")
    }

    #[java_rta_macros::java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;")]
    pub fn toArray_arr_obj(&self, a: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/ArrayList.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    #[java_rta_macros::java_method(name = "elementAt", descriptor = "([Ljava/lang/Object;I)Ljava/lang/Object;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>([Ljava/lang/Object;I)TE;")]
    pub fn elementAt(es: Rc<RefCell<Vec<Object>>>, index: i32) -> Result<Object> {
        panic!("stub: java/util/ArrayList.elementAt:([Ljava/lang/Object;I)Ljava/lang/Object;")
    }

    #[java_rta_macros::java_method(name = "getFirst", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
    pub fn getFirst(&self) -> Result<Object> {
        panic!("stub: java/util/ArrayList.getFirst:()Ljava/lang/Object;")
    }

    #[java_rta_macros::java_method(name = "getLast", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
    pub fn getLast(&self) -> Result<Object> {
        panic!("stub: java/util/ArrayList.getLast:()Ljava/lang/Object;")
    }

    #[java_rta_macros::java_method(name = "set", descriptor = "(ILjava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITE;)TE;")]
    pub fn set(&self, index: i32, element: E) -> Result<Object> {
        panic!("stub: java/util/ArrayList.set:(ILjava/lang/Object;)Ljava/lang/Object;")
    }

    #[java_rta_macros::java_method(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)Z")]
    // java: add(Ljava/lang/Object;)Z
    pub fn add_obj(&self, mut e: E) -> Result<bool> {
        let this = self;
        this._super.modCount.set((this._super.modCount.get()).wrapping_add(1i32));
        this.add_obj_arr_obj_i(Clone::clone(&e), Clone::clone(&this.elementData.get()), this.size.get())?;
        Ok((1i32 != 0i32))
    }

    #[java_rta_macros::java_method(name = "add", descriptor = "(ILjava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITE;)V")]
    pub fn add_i_obj(&self, index: i32, element: E) -> Result<()> {
        panic!("stub: java/util/ArrayList.add:(ILjava/lang/Object;)V")
    }

    #[java_rta_macros::java_method(name = "addFirst", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V")]
    pub fn addFirst(&self, element: E) -> Result<()> {
        panic!("stub: java/util/ArrayList.addFirst:(Ljava/lang/Object;)V")
    }

    #[java_rta_macros::java_method(name = "addLast", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V")]
    pub fn addLast(&self, element: E) -> Result<()> {
        panic!("stub: java/util/ArrayList.addLast:(Ljava/lang/Object;)V")
    }

    #[java_rta_macros::java_method(name = "remove", descriptor = "(I)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)TE;")]
    pub fn remove_i(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/ArrayList.remove:(I)Ljava/lang/Object;")
    }

    #[java_rta_macros::java_method(name = "removeFirst", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
    pub fn removeFirst(&self) -> Result<Object> {
        panic!("stub: java/util/ArrayList.removeFirst:()Ljava/lang/Object;")
    }

    #[java_rta_macros::java_method(name = "removeLast", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
    pub fn removeLast(&self) -> Result<Object> {
        panic!("stub: java/util/ArrayList.removeLast:()Ljava/lang/Object;")
    }

    #[java_rta_macros::java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn equals(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/ArrayList.equals:(Ljava/lang/Object;)Z")
    }

    #[java_rta_macros::java_method(name = "equalsRange", descriptor = "(Ljava/util/List;II)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<*>;II)Z")]
    pub fn equalsRange(&self, other: List<Object>, from: i32, to: i32) -> Result<bool> {
        panic!("stub: java/util/ArrayList.equalsRange:(Ljava/util/List;II)Z")
    }

    #[java_rta_macros::java_method(name = "equalsArrayList", descriptor = "(Ljava/util/ArrayList;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/ArrayList<*>;)Z")]
    pub fn equalsArrayList(&self, other: ArrayList<Object>) -> Result<bool> {
        panic!("stub: java/util/ArrayList.equalsArrayList:(Ljava/util/ArrayList;)Z")
    }

    #[java_rta_macros::java_method(name = "checkForComodification", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn checkForComodification(&self, expectedModCount: i32) -> Result<()> {
        panic!("stub: java/util/ArrayList.checkForComodification:(I)V")
    }

    #[java_rta_macros::java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn hashCode(&self) -> Result<i32> {
        Ok(0)
    }

    #[java_rta_macros::java_method(name = "hashCodeRange", descriptor = "(II)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn hashCodeRange(&self, from: i32, to: i32) -> Result<i32> {
        panic!("stub: java/util/ArrayList.hashCodeRange:(II)I")
    }

    #[java_rta_macros::java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn remove_obj(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/ArrayList.remove:(Ljava/lang/Object;)Z")
    }

    #[java_rta_macros::java_method(name = "fastRemove", descriptor = "([Ljava/lang/Object;I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn fastRemove(&self, es: Rc<RefCell<Vec<Object>>>, i: i32) -> Result<()> {
        panic!("stub: java/util/ArrayList.fastRemove:([Ljava/lang/Object;I)V")
    }

    #[java_rta_macros::java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/ArrayList.clear:()V")
    }

    #[java_rta_macros::java_method(name = "addAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<+TE;>;)Z")]
    pub fn addAll_coll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/ArrayList.addAll:(Ljava/util/Collection;)Z")
    }

    #[java_rta_macros::java_method(name = "addAll", descriptor = "(ILjava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/util/Collection<+TE;>;)Z")]
    pub fn addAll_i_coll(&self, index: i32, c: Object) -> Result<bool> {
        panic!("stub: java/util/ArrayList.addAll:(ILjava/util/Collection;)Z")
    }

    #[java_rta_macros::java_method(name = "removeRange", descriptor = "(II)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn removeRange(&self, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/ArrayList.removeRange:(II)V")
    }

    #[java_rta_macros::java_method(name = "shiftTailOverGap", descriptor = "([Ljava/lang/Object;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn shiftTailOverGap(&self, es: Rc<RefCell<Vec<Object>>>, lo: i32, hi: i32) -> Result<()> {
        panic!("stub: java/util/ArrayList.shiftTailOverGap:([Ljava/lang/Object;II)V")
    }

    #[java_rta_macros::java_method(name = "rangeCheckForAdd", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn rangeCheckForAdd(&self, index: i32) -> Result<()> {
        panic!("stub: java/util/ArrayList.rangeCheckForAdd:(I)V")
    }

    #[java_rta_macros::java_method(name = "outOfBoundsMsg", descriptor = "(I)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn outOfBoundsMsg_i(&self, index: i32) -> Result<String> {
        panic!("stub: java/util/ArrayList.outOfBoundsMsg:(I)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "outOfBoundsMsg", descriptor = "(II)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn outOfBoundsMsg_i_i(fromIndex: i32, toIndex: i32) -> Result<String> {
        panic!("stub: java/util/ArrayList.outOfBoundsMsg:(II)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "removeAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z")]
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/ArrayList.removeAll:(Ljava/util/Collection;)Z")
    }

    #[java_rta_macros::java_method(name = "retainAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z")]
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/ArrayList.retainAll:(Ljava/util/Collection;)Z")
    }

    #[java_rta_macros::java_method(name = "batchRemove", descriptor = "(Ljava/util/Collection;ZII)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;ZII)Z")]
    pub fn batchRemove(&self, c: Object, complement: bool, from: i32, end: i32) -> Result<bool> {
        panic!("stub: java/util/ArrayList.batchRemove:(Ljava/util/Collection;ZII)Z")
    }

    #[java_rta_macros::java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
    pub fn writeObject(&self, s: Object) -> Result<()> {
        panic!("stub: java/util/ArrayList.writeObject:(Ljava/io/ObjectOutputStream;)V")
    }

    #[java_rta_macros::java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
    pub fn readObject(&self, s: Object) -> Result<()> {
        panic!("stub: java/util/ArrayList.readObject:(Ljava/io/ObjectInputStream;)V")
    }

    #[java_rta_macros::java_method(name = "listIterator", descriptor = "(I)Ljava/util/ListIterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)Ljava/util/ListIterator<TE;>;")]
    pub fn listIterator_i(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/ArrayList.listIterator:(I)Ljava/util/ListIterator;")
    }

    #[java_rta_macros::java_method(name = "listIterator", descriptor = "()Ljava/util/ListIterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/ListIterator<TE;>;")]
    pub fn listIterator(&self) -> Result<Object> {
        panic!("stub: java/util/ArrayList.listIterator:()Ljava/util/ListIterator;")
    }

    #[java_rta_macros::java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TE;>;")]
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/ArrayList.iterator:()Ljava/util/Iterator;")
    }

    #[java_rta_macros::java_method(name = "subList", descriptor = "(II)Ljava/util/List;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(II)Ljava/util/List<TE;>;")]
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        panic!("stub: java/util/ArrayList.subList:(II)Ljava/util/List;")
    }

    #[java_rta_macros::java_method(name = "forEach", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TE;>;)V")]
    pub fn forEach(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/ArrayList.forEach:(Ljava/util/function/Consumer;)V")
    }

    #[java_rta_macros::java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TE;>;")]
    pub fn spliterator(&self) -> Result<Object> {
        panic!("stub: java/util/ArrayList.spliterator:()Ljava/util/Spliterator;")
    }

    #[java_rta_macros::java_method(name = "nBits", descriptor = "(I)[J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn nBits(n: i32) -> Result<Rc<RefCell<Vec<i64>>>> {
        panic!("stub: java/util/ArrayList.nBits:(I)[J")
    }

    #[java_rta_macros::java_method(name = "setBit", descriptor = "([JI)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn setBit(bits: Rc<RefCell<Vec<i64>>>, i: i32) -> Result<()> {
        panic!("stub: java/util/ArrayList.setBit:([JI)V")
    }

    #[java_rta_macros::java_method(name = "isClear", descriptor = "([JI)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn isClear(bits: Rc<RefCell<Vec<i64>>>, i: i32) -> Result<bool> {
        panic!("stub: java/util/ArrayList.isClear:([JI)Z")
    }

    #[java_rta_macros::java_method(name = "removeIf", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TE;>;)Z")]
    pub fn removeIf_predic(&self, filter: Object) -> Result<bool> {
        panic!("stub: java/util/ArrayList.removeIf:(Ljava/util/function/Predicate;)Z")
    }

    #[java_rta_macros::java_method(name = "removeIf", descriptor = "(Ljava/util/function/Predicate;II)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TE;>;II)Z")]
    pub fn removeIf_predic_i_i(&self, filter: Object, i: i32, end: i32) -> Result<bool> {
        panic!("stub: java/util/ArrayList.removeIf:(Ljava/util/function/Predicate;II)Z")
    }

    #[java_rta_macros::java_method(name = "replaceAll", descriptor = "(Ljava/util/function/UnaryOperator;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/UnaryOperator<TE;>;)V")]
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        panic!("stub: java/util/ArrayList.replaceAll:(Ljava/util/function/UnaryOperator;)V")
    }

    #[java_rta_macros::java_method(name = "replaceAllRange", descriptor = "(Ljava/util/function/UnaryOperator;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/UnaryOperator<TE;>;II)V")]
    pub fn replaceAllRange(&self, operator: Object, i: i32, end: i32) -> Result<()> {
        panic!("stub: java/util/ArrayList.replaceAllRange:(Ljava/util/function/UnaryOperator;II)V")
    }

    #[java_rta_macros::java_method(name = "sort", descriptor = "(Ljava/util/Comparator;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Comparator<-TE;>;)V")]
    pub fn sort(&self, c: Object) -> Result<()> {
        panic!("stub: java/util/ArrayList.sort:(Ljava/util/Comparator;)V")
    }

    #[java_rta_macros::java_method(name = "checkInvariants", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn checkInvariants(&self) -> Result<()> {
        panic!("stub: java/util/ArrayList.checkInvariants:()V")
    }

    #[java_rta_macros::java_method(name = "reversed", descriptor = "()Ljava/util/List;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/List<TE;>;")]
    pub fn reversed(&self) -> Result<Object> {
        panic!("stub: java/util/ArrayList.reversed:()Ljava/util/List;")
    }
}
