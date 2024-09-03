use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[component]
pub fn MermaidDemo() -> impl IntoView {
    let diagram = "graph TD\nA[Client] --> B[Server]\nB --> C[Database]\nC --> D[Blockchain]";

    let diagram_ref = create_node_ref::<html::Div>();
    let (render_status, set_render_status) = create_signal("Not rendered yet".to_string());

    create_effect(move |_| {
        if let Some(element) = diagram_ref.get() {
            log("Attempting to render Mermaid diagram");
            let event_init = web_sys::CustomEventInit::new();
            event_init.set_detail(&JsValue::from_str(diagram));
            
            let event = web_sys::CustomEvent::new_with_event_init_dict(
                "render-mermaid",
                &event_init,
            )
            .unwrap();
            element.dispatch_event(&event).unwrap();
            set_render_status.set("Render event dispatched".to_string());
        }
    });

    view! {
        <div class="text-purple-300 p-4">
            <h2 class="ib text-2xl mb-4">"Mermaid Diagram Demo"</h2>
            <div _ref=diagram_ref id="mermaid-diagram" class="mermaid">
                {diagram}
            </div>
            <p class="ir mt-4">"Render status: " {render_status}</p>
            <pre class="ir mt-4 p-2 bg-gray-700 rounded">{diagram}</pre>
        </div>
    }
}
