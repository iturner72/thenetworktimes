use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs)]
    fn highlightElement(element: &web_sys::Element);
}

#[component]
pub fn CodeBlock(
    #[prop(into)] code: String,
    #[prop(into)] language: String,
) -> impl IntoView {
    let code_ref = create_node_ref::<html::Code>();

    create_effect(move |_| {
        if let Some(element) = code_ref.get() {
            highlightElement(&element);
        }
    });

    view! {
        <pre>
            <code class={format!("language-{}", language)} node_ref=code_ref>
                {code}
            </code>
        </pre>
    }
}
