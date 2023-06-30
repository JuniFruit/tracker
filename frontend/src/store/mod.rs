pub mod apps_store;
pub mod user_store;
/* This implementaion rejects the idea of pure functions. Instead of copying state we simply mutate it in place */
pub struct Store<T, M> {
    state: T,
    reducer: Box<dyn Fn(&mut T, M) -> ()>,
    middleware: Vec<Box<dyn Fn(&mut T, M) -> M>>,
}

unsafe impl<T, M> Send for Store<T, M> {}

impl<T, M> Store<T, M>
where
    T: Default,
    M: ReducerMsg + Clone,
{
    pub fn new(reducer: Box<dyn Fn(&mut T, M) -> ()>) -> Self {
        Store {
            state: T::default(),
            reducer,
            middleware: Vec::new(),
        }
    }

    pub fn dispatch(&mut self, msg: M) {
        (self.reducer)(&mut self.state, msg);
        self.middleware_strategy(msg);
    }
    /* Adds middileware */
    pub fn use_middleware(&mut self, mw: Box<dyn Fn(&mut T, M) -> M>) {
        self.middleware.push(mw);
    }

    pub fn selector(&mut self) -> &T {
        &self.state
    }

    fn middleware_strategy(&mut self, msg: M) {
        let mut results = Vec::new();
        for mw in self.middleware.iter() {
            results.push((mw)(&mut self.state, msg));
        }
        for m in results.iter() {
            self.dispatch(m.clone());
        }
    }
}

pub trait ReducerMsg {
    type Value;
}

// struct Msg {}

// impl ReducerMsg for Msg {
//     type Value = Messages;
// }

pub enum Messages {
    None,
}
