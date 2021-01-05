pub struct LazyTransform<T, S, FN> {
    transform_fn: FN,
    source: Option<S>,
    value: Option<T>,
}
impl<T: Clone, S, FN: Fn(S) -> Option<T>> LazyTransform<T, S, FN> {
    pub fn new(transform_fn: FN) -> LazyTransform<T, S, FN> {
        LazyTransform {
            transform_fn: transform_fn,
            source: None,
            value: None,
        }
    }
    pub fn set_source(&mut self, source: S) {
       self.source = Some(source);
    }
    pub fn get_transformed(&mut self) -> Option<T> {
        if let Some(source) = self.source.take() {
            let newval = (self.transform_fn)(source);
            if newval.is_some() {
                self.value = newval;
            }
        }
        self.value.clone()
    }
}