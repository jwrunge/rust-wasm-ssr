mod stores;

use std::collections::HashMap;
use stores::Store;
use stores::StoreFunctionality;

fn main() {
    let mut store1 = Store::<i32>::new(32, None, None);

    let value = store1.get();
    println!("store1 value 1: {:?}", value);

    store1.update(64);
    let value = store1.get();
    println!("store1 value 2: {:?}", value);

    let store2 = Store::<i32>::new(
        0,
        Some(|store: Box<dyn StoreFunctionality>| {
            let d = store.get_upstream();
            let store1_value = match d.get("store1") {
                Some(store1) => {
                    *store1.get()
                },
                None => 0,
            };
            store1_value * 2
        }),
        Some({
            let stores = match store1.initializer {
                Some(func) => {
                    let mut map = HashMap::new();
                    map.insert("store1".to_string(), func);
                    map
                }
                None => HashMap::new(),
            };
            stores
        }),
    );
    let value2 = store2.get();
    println!("store2 value: {:?}", value2);

    store1.update(10);
    println!(
        "store1 value 3: {:?}; store2 value 2: {:?}",
        store1.get(),
        store2.get()
    );
}
