use crate::prelude::*;
use super::list::List;

impl<E: Clone + Default + 'static> List<E> {
    pub fn stream(&self) -> Result<Object> {
        panic!("stub: java/util/List.stream:()Ljava/util/stream/Stream;")
    }
}
