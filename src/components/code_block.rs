use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs)]
    fn highlightElement(element: &web_sys::Element);
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&#39;")
}

#[component]
pub fn CodeBlock(
    #[prop(into)] code: String,
    #[prop(into)] language: String,
) -> impl IntoView {
    let code_ref = create_node_ref::<html::Code>();
    let escaped_code = escape_html(&code);

    create_effect(move |_| {
        if let Some(element) = code_ref.get() {
            highlightElement(&element);
        }
    });

    view! {
        <pre class="bg-gray-100 rounded-md p-4 overflow-x-auto">
            <code class={format!("language-{} text-sm", language)} node_ref=code_ref inner_html={escaped_code}>
            </code>
        </pre>
    }

}
