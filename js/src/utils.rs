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

// Get a [f64; 3] from a JavaScript value
pub fn to_vec3f(values: Box<[JsValue]>) -> [f64; 3] {
    [
        values.get(0).unwrap_or(&JsValue::NULL).as_f64().unwrap_or(0.0),
        values.get(1).unwrap_or(&JsValue::NULL).as_f64().unwrap_or(0.0),
        values.get(2).unwrap_or(&JsValue::NULL).as_f64().unwrap_or(0.0)
    ]
}

pub trait Native {
    type Output: Sized;

    fn into_native(self) -> Self::Output;
    fn as_native(&self) -> &Self::Output;
    fn as_native_mut(&mut self) -> &mut Self::Output;
}
