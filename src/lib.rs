use wasm_bindgen::prelude::*;

use std::{
    mem::forget,
    sync::{Arc, LazyLock, Mutex},
};

type Payload = Box<i32>;

static STORAGE: LazyLock<Arc<Payload>> = LazyLock::new(|| Arc::new(Box::new(1)));
static SPARE: Mutex<Option<Arc<Payload>>> = Mutex::new(None);

#[wasm_bindgen]
pub fn initialize() {
    *SPARE.lock().unwrap() = Some(Arc::clone(&STORAGE));
}

#[wasm_bindgen]
pub fn call_me() -> i32 {
    loop {
        let ref_count = Arc::strong_count(&STORAGE);
        if ref_count != 1 {
            // Increment the reference count by 1.
            // If this the reference count becomes too large, this will abort the rust program.
            // This results in an exception on the JS side.
            forget(Arc::clone(&STORAGE));
        } else {
            // We've successfully overflowed the reference count.
            // Now we cause a use-after-free.
            drop(SPARE.lock().unwrap().take());
            return ***STORAGE;
        }
    }
}
