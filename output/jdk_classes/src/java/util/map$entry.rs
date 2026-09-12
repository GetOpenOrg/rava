#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Map$Entry",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "Map.java",
))]
pub struct Map_Entry<K, V>;

impl<K: Clone + 'static, V: Clone + 'static> Map_Entry<K, V> {
    #[cfg_attr(any(), java_native(name = "getKey", descriptor = "()Ljava/lang/Object;", access = "public abstract"))]
    pub fn getKey(&self) -> Result<Object> {
        todo!("abstract java/util/Map$Entry.getKey")
    }

    #[cfg_attr(any(), java_native(name = "getValue", descriptor = "()Ljava/lang/Object;", access = "public abstract"))]
    pub fn getValue(&self) -> Result<Object> {
        todo!("abstract java/util/Map$Entry.getValue")
    }

    #[cfg_attr(any(), java_native(name = "setValue", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public abstract"))]
    pub fn setValue(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/Map$Entry.setValue")
    }

    #[cfg_attr(any(), java_native(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public abstract"))]
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Map$Entry.equals")
    }

    #[cfg_attr(any(), java_native(name = "hashCode", descriptor = "()I", access = "public abstract"))]
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/Map$Entry.hashCode")
    }

    #[cfg_attr(any(), java_method(name = "comparingByKey", descriptor = "()Ljava/util/Comparator;", access = "public static"))]
    // java: comparingByKey()Ljava/util/Comparator;
    pub fn comparingByKey() -> Result<Object> {
        /* TODO: invokedynamic 1 */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "comparingByValue", descriptor = "()Ljava/util/Comparator;", access = "public static"))]
    // java: comparingByValue()Ljava/util/Comparator;
    pub fn comparingByValue() -> Result<Object> {
        /* TODO: invokedynamic 9 */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "comparingByKey", descriptor = "(Ljava/util/Comparator;)Ljava/util/Comparator;", access = "public static"))]
    // java: comparingByKey(Ljava/util/Comparator;)Ljava/util/Comparator;
    pub fn comparingByKey__compar(cmp: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(cmp)?;
        /* TODO: invokedynamic 16 */
        Ok(cmp)
    }

    #[cfg_attr(any(), java_method(name = "comparingByValue", descriptor = "(Ljava/util/Comparator;)Ljava/util/Comparator;", access = "public static"))]
    // java: comparingByValue(Ljava/util/Comparator;)Ljava/util/Comparator;
    pub fn comparingByValue__compar(cmp: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(cmp)?;
        /* TODO: invokedynamic 19 */
        Ok(cmp)
    }

    #[cfg_attr(any(), java_method(name = "copyOf", descriptor = "(Ljava/util/Map$Entry;)Ljava/util/Map$Entry;", access = "public static"))]
    pub fn copyOf(e: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(e)?;
        return Ok(e);
        let _t1 = e.getKey()?;
        let _t2 = e.getValue()?;
        let _t3: Object = Map::entry(_t1, _t2)?;
        Ok(_t3)
    }
}
