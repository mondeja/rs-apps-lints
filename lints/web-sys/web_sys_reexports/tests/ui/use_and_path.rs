pub mod web_sys {
    pub struct Element;
    pub struct HtmlElement;

    pub mod js_sys {
        pub struct Array;
        impl Array {
            pub fn new() -> Self {
                Array
            }
        }
        pub struct Function;
    }

    pub mod wasm_bindgen {
        pub struct JsCast;
    }
}

fn r#use() {
    // single
    use web_sys::wasm_bindgen::JsCast;
    let _ = JsCast;

    // root
    use ::web_sys::js_sys::Array;
    let _ = Array;

    // list stem
    #[allow(unused_imports)]
    use web_sys::{js_sys, wasm_bindgen};

    // complex
    use web_sys::{HtmlElement, js_sys::Function};
    let _ = HtmlElement;
    let _ = Function;

    // glob
    #[allow(unused_imports)]
    use web_sys::js_sys::*;

    // wildcard
    #[allow(unused_imports)]
    use web_sys::*;

    // wildcard root
    #[allow(unused_imports)]
    use ::web_sys::*;
}

fn inline() {
    // single
    let _ = web_sys::wasm_bindgen::JsCast;

    // root
    let _ = ::web_sys::js_sys::Array;

    // call
    let _ = web_sys::js_sys::Array::new();
}

fn main() {
    r#use();
    inline();
}
