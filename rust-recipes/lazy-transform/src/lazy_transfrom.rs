use std::sync::Mutex;
struct LazyState<T, S> {
    source: Option<S>,
    value: Option<T>,
}

pub struct LazyTransform<T, S, FN> {
    transform_fn: FN,
    state: Mutex<LazyState<T, S>>,
}
impl<T: Clone, S, FN: Fn(S) -> Option<T>> LazyTransform<T, S, FN> {
    pub fn new(transform_fn: FN) -> LazyTransform<T, S, FN> {
        LazyTransform {
            transform_fn: transform_fn,
            state: Mutex::new(LazyState { source: None,value: None}),
        }
    }
    pub fn set_source(&self, source: S) {
        let mut state = self.state.lock().unwrap();
        state.source = Some(source);
    }
    pub fn get_transformed(&mut self) -> Option<T> {
        let mut state = self.state.lock().unwrap();
        if let Some(source) = state.source.take() {
            let newval = (self.transform_fn)(source);
            if newval.is_some() {
                state.value = newval;
            }
        }
        state.value.clone()
    }
}