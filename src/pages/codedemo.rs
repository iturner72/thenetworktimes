use leptos::*;
use wasm_bindgen::prelude::*;
use std::time::Duration;

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

    /*
       lmaooo
    
       this happened on refresh while on codeblock route
    
       \textit{thread caused non-unwinding panic. aborting.}
    
       set_timeout impls FnOnce dummy 
    
       will fix later
    
       */

    set_timeout(
        {
            move || {
                for (i, _) in words.0.get().iter().enumerate() {
                    set_timeout(
                        move || {
                            words.1.update(|w| w[i].1 = true);
                        },
                        Duration::from_millis(100 * i as u64)
                    );
                }
            }
        },
        Duration::from_millis(100)
    );

    view! {
        <pre class="code-block-container flex flex-col items-start bg-wenge-900 text-left w-auto">
            <code class={format!("border-2 border-mint-800 language-{} text-sm", language)}>
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
