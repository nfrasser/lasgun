use cfg_if::cfg_if;
use wasm_bindgen::JsValue;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// Get a [f64; 3] from a JavaScript value
pub fn to_vec3f(values: Box<[JsValue]>) -> [f64; 3] {
    [
        values.get(0).unwrap_or(&JsValue::NULL).as_f64().unwrap_or(0.0),
        values.get(1).unwrap_or(&JsValue::NULL).as_f64().unwrap_or(0.0),
        values.get(2).unwrap_or(&JsValue::NULL).as_f64().unwrap_or(0.0)
    ]
}

// Get a [u8; 3] from a JavaScript value
pub fn to_vec3u8(values: Box<[JsValue]>) -> [u8; 3] {
    [
        values.get(0).unwrap_or(&JsValue::NULL).as_f64().unwrap_or(0.0) as u8,
        values.get(1).unwrap_or(&JsValue::NULL).as_f64().unwrap_or(0.0) as u8,
        values.get(2).unwrap_or(&JsValue::NULL).as_f64().unwrap_or(0.0) as u8
    ]
}
