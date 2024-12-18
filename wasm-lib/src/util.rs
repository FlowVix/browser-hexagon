use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[doc(hidden)]
pub fn __log(s: &str) {
    log(s)
}

#[macro_export]
macro_rules! console_log {
    ($($arg:tt)*) => {
        $crate::util::__log(&format!($($arg)*))
    };
}

pub trait BoxPostfix {
    fn boxed(self) -> Box<Self>;
}
impl<T> BoxPostfix for T {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
