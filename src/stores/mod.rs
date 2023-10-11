pub trait StoreFunctionality<T> {
    fn initialize_from_upstream_origins(&mut self);
    fn add_downstream(&mut self, store: Box<dyn StoreFunctionality<T>>);
    fn report_downstream(&self);
}

pub struct Store<T> {
    //State
    value: Option<T>,
    pub initializer: Option<Box<dyn Fn() -> Option<T>>>,
    subscriptions: Vec<Box<dyn Fn(&T) -> ()>>,

    //Derivation
    upstream_stores: Vec<Box<dyn StoreFunctionality<T>>>,
    downstream_stores: Vec<Box<dyn StoreFunctionality<T>>>,
}

impl<T> Store<T> {
    pub fn new(init_func: Option<Box<dyn Fn()-> Option<T>>>, val: Option<T>) -> Store<T> {
        Store {
            value: val,
            initializer: init_func,
            subscriptions: Vec::new(),
            upstream_stores: Vec::new(),
            downstream_stores: Vec::new(),
        }
    }

    pub fn get(&self) -> &Option<T> {
        &self.value
    }

    pub fn update(&mut self, new_value: Option<T>) {
        self.value = new_value;
        self.report_downstream();
    }
}

impl<T> StoreFunctionality<T> for Store<T> {
    //(re)initialize the store from upstream stores
    fn initialize_from_upstream_origins(&mut self) {
        match &self.initializer {
            Some(value) => {
                self.value = (value)();
            }
            None => {}
        };
    }

    //Add a downstream store (to be updated when this store is updated)
    fn add_downstream(&mut self, store: Box<dyn StoreFunctionality<T>>) {
        self.downstream_stores.push(store);
    }

    //Report value changes to all downstream stores
    fn report_downstream(&self) {
        for store in self.downstream_stores.iter() {
            store.report_downstream();
        }
    }
}
