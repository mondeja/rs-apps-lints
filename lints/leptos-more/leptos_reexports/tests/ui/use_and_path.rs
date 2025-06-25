pub mod leptos {
    pub mod serde {
        pub struct Serialize;
    }

    pub mod wasm_bindgen {
        pub struct JsCast;
    }

    pub mod web_sys {
        pub struct Element;
        pub struct HtmlElement;
    }

    pub mod prelude {
        pub struct Signal;
    }

    pub mod tracing {
        pub struct Instrument;
    }

    pub mod serde_json {
        pub struct Value;
    }
}

fn r#use() {
    // single
    use leptos::wasm_bindgen::JsCast;
    let _ = JsCast;

    // root
    use ::leptos::serde::Serialize;
    let _ = Serialize;

    // list stem
    #[allow(unused_imports)]
    use leptos::{wasm_bindgen, serde};
    #[allow(unused_imports)]
    use {leptos::serde_json, leptos::tracing};

    // complex
    use leptos::{prelude::*, web_sys::HtmlElement};
    let _ = HtmlElement;
    let _ = Signal;

    // glob
    use leptos::web_sys::*;
    let _ = Element;

    // wildcard
    #[allow(unused_imports)]
    use leptos::*;

    // wildcard root
    #[allow(unused_imports)]
    use ::leptos::*;
}

fn inline() {
    // single
    let _ = leptos::wasm_bindgen::JsCast;

    // root
    let _ = ::leptos::serde::Serialize;
}

fn main() {
    r#use();
    inline();
}
