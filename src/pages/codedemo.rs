use leptos::*;
use crate::components::code_block::CodeBlock;

#[component]
pub fn CodeDemo() -> impl IntoView {
    let rust_code = "fn main() {\n    println!(\"Hello, world!\");\n}";
    let python_code = "def greet(name):\n    print(f\"Hello, {name}!\")\n\ngreet(\"World\")";

    view! {
        <h1>"Code Demo"</h1>
        <h2>"Rust Example"</h2>
        <CodeBlock code=rust_code language="rust"/>
        <h2>"Python Example"</h2>
        <CodeBlock code=python_code language="python"/>
    }
}
