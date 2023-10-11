mod stores;
use stores::Store;

fn main() {
    let mut store1 = Store::<i32>::new(None, Some(32));

    let value = match store1.get() {
        Some(value) => value,
        None => &0,
    };

    println!("store1 value 1: {:?}", value);

    store1.update(Some(64));

    let value = match store1.get() {
        Some(value) => value,
        None => &0,
    };

    

    println!("store1 value 2: {:?}", value);
}
