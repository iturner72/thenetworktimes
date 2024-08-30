use leptos::*;
use wasm_bindgen::prelude::*;
use std::time::Duration;

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
pub fn CodeDemo() -> impl IntoView {
    let rust_code = "fn main() {\n    println!(\"Hello, world!\");\n}";
    let python_code = "def greet(name):\n    print(f\"Hello, {name}!\")\n\ngreet(\"World\")";

    view! {
        <div class="flex justify-center items-start min-h-screen pt-6">
            <div class="code-demo-container bg-gray-900 border-2 border-gray-800 flex flex-col items-center justify-center text-wenge-300 p-2 w-auto max-w-4/12 space-y-2">
                <h1 class="ib text-2xl">"highlight js demo"</h1>
                <h2>"Rust Example"</h2>
                <CodeBlock code=rust_code language="rust"/>
                <h2>"Python Example"</h2>
                <CodeBlock code=python_code language="python"/>
            </div>
        </div>
    }
}

#[component]
pub fn CodeBlock(#[prop(into)] code: String, #[prop(into)] language: String) -> impl IntoView {
    let code_ref = create_node_ref::<html::Code>();
    let (displayed_code, set_displayed_code) = create_signal(String::new());

    create_effect(move |_| {
        let escaped_code = escape_html(&code);
        for (i, _) in escaped_code.char_indices() {
            let current_code = escaped_code[..=i].to_string();
            set_timeout(
                move || set_displayed_code.set(current_code),
                Duration::from_millis(20 * i as u64)
            );
        }

        // highlight after full code display
        set_timeout(
            move || {
                if let Some(element) = code_ref.get() {
                    highlightElement(&element);
                }
            },
            Duration::from_millis(20 * escaped_code.len() as u64 + 100)
        );
    });

    view! {
        <pre class="code-block-container flex flex-col items-start bg-gray-800 text-left w-auto">
            <code class={format!("border-2 border-mint-800 language-{} text-sm", language)} node_ref=code_ref inner_html={move || displayed_code.get()}>
            </code>
        </pre>
    }//
}
