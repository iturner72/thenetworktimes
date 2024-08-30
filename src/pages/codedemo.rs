use crate::cfg_if;
use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs)]
    fn highlightElement(element: &web_sys::Element);
}

// not using highlight js to test word highlighting on stream

fn _escape_html(s: &str) -> String {
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
    let words = create_signal(code.split_whitespace().map(|w| (w.to_string(), false)).collect::<Vec<_>>());
    let streaming_complete = create_signal(false);

    cfg_if! {
        if #[cfg(not(feature = "ssr"))] {
            use std::time::Duration;

            set_timeout(
                {
                    let words = words.clone();
                    let streaming_complete = streaming_complete.clone();
                    move || {
                        for (i, _) in words.0.get().iter().enumerate() {
                            set_timeout(
                                {
                                    let words = words.clone();
                                    let streaming_complete = streaming_complete.clone();
                                    move || {
                                        words.1.update(|w| w[i].1 = true);
                                        if i == words.0.get().len() - 1 {
                                            streaming_complete.1.set(true);
                                        }
                                    }
                                },
                                Duration::from_millis(50 * i as u64)
                            );
                        }
                    }
                },
                Duration::from_millis(50)
            );
        }
    }

    let code_ref = create_node_ref::<html::Code>();

    create_effect(move |_| {
        if streaming_complete.0() {
            if let Some(element) = code_ref.get() {
                highlightElement(&element);
            }
        }
    });

    view! {
        <pre class="code-block-container flex flex-col items-start bg-wenge-900 text-left w-auto">
            <code _ref=code_ref class={format!("border-2 border-mint-800 language-{} text-sm", language)}>
                {move || words.0().iter().map(|(word, highlighted)| {
                    view! {
                        <span class={if *highlighted { "highlighted" } else { "" }}>
                            {word} " "
                        </span>
                    }
                }).collect::<Vec<_>>()}
            </code>
        </pre>
    }
}
