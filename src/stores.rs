use std::collections::HashMap;
use std::any::Any;

pub trait StoreFunctionality {
    fn get(&self)-> &dyn Any;
    fn initialize_from_upstream_origins(&mut self);
    fn add_downstream(&mut self, key: String, store: Box<dyn StoreFunctionality>);
    fn get_upstream(&self) -> &HashMap<String, Box<dyn StoreFunctionality>>;
    fn report_downstream(&self);
}

pub struct Store<T> {
    //State
    value: T,
    pub initializer: Option<fn(&Self) -> T>,
    subscriptions: Vec<Box<dyn Fn(&T) -> ()>>,

    //Derivation
    upstream_stores: HashMap<String, Box<dyn StoreFunctionality>>,
    downstream_stores: HashMap<String, Box<dyn StoreFunctionality>>
}

impl<T> Store<T> {
    pub fn new(
        val: T,
        initializer: Option<fn(&Self) -> T>,
        upstream_stores: Option<HashMap<String, Box<dyn StoreFunctionality>>>,
    ) -> Store<T> {
        let ustores = match upstream_stores {
            Some(stores) => stores,
            None => HashMap::new(),
        };
        match initializer {
            Some(_) => {
                let mut store = Store {
                    value: val,
                    initializer,
                    subscriptions: Vec::new(),
                    upstream_stores: ustores,
                    downstream_stores: HashMap::new(),
                };
                store.initialize_from_upstream_origins();
                store
            }
            None => Store {
                value: val,
                initializer,
                subscriptions: Vec::new(),
                upstream_stores: ustores,
                downstream_stores: HashMap::new(),
            },
        }
    }

    pub fn get(&self) -> T {
        self.value
    }

    pub fn update(&mut self, new_value: T) {
        self.value = new_value;
        self.report_downstream();
    }
}

impl<T> StoreFunctionality for Store<T> {
    fn get(&self) -> &dyn Any {
        &self.value
    }

    //(re)initialize the store from upstream stores
    fn initialize_from_upstream_origins(&mut self) {
        match self.initializer {
            Some(value) => {
                let new_value = (value)(&self);
                self.update(new_value);
            }
            None => {}
        };
    }

    //Add a downstream store (to be updated when this store is updated)
    fn add_downstream(&mut self, key: String, store: Box<dyn StoreFunctionality>) {
        self.downstream_stores.insert(key, store);
    }

    fn get_upstream(&self) -> &HashMap<String, Box<dyn StoreFunctionality>> {
        &self.upstream_stores
    }

    //Report value changes to all downstream stores
    fn report_downstream(&self) {
        for (_, store) in self.downstream_stores.iter() {
            store.initialize_from_upstream_origins();
            store.report_downstream();
        }
        for subscription in self.subscriptions.iter() {
            subscription(&self.value);
        }
    }
}
