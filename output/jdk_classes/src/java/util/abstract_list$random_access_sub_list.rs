#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/AbstractList$RandomAccessSubList",
    super_class = "java/util/AbstractList$SubList",
    interfaces  = "java/util/RandomAccess",
    access      = "",
    source      = "AbstractList.java",
))]
pub struct AbstractList_RandomAccessSubList<E>;

impl<E: Clone + 'static> AbstractList_RandomAccessSubList<E> {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/AbstractList;II)V"))]
    // java: <init>(Ljava/util/AbstractList;II)V
    pub fn new__abstra_i_i(root: Object, fromIndex: i32, toIndex: i32) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/util/AbstractList$SubList.<init>:(Ljava/util/AbstractList;II)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/AbstractList$RandomAccessSubList;II)V"))]
    // java: <init>(Ljava/util/AbstractList$RandomAccessSubList;II)V
    pub fn new__abstra_i_i(parent: Object, fromIndex: i32, toIndex: i32) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/util/AbstractList$SubList.<init>:(Ljava/util/AbstractList$SubList;II)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "subList", descriptor = "(II)Ljava/util/List;", access = "public"))]
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        let this = self;
        AbstractList$RandomAccessSubList::subListRangeCheck(fromIndex, toIndex, this.size.get())?;
        Ok(AbstractList_RandomAccessSubList::new(this, fromIndex, toIndex)?)
    }
}
