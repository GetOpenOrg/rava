use crate::prelude::*;
use super::list::List;
use crate::java::util::stream::Stream;

impl<E: Clone + Default + 'static> List<E> {
    pub fn stream(&self) -> Result<Stream<Object>> {
        panic!("stub: java/util/List.stream:()Ljava/util/stream/Stream;")
    }
}
