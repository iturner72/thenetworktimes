use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[component]
pub fn MermaidDemo() -> impl IntoView {
    let diagram = r#"graph TD

        A[main.rs] --> B[app.rs]
        A --> C[handlers.rs]
        A --> D[state.rs]
        A --> E[fileserv.rs]
        
        B --> F[pages]
        B --> G[components]
        
        H[lib.rs] --> B
        H --> C
        H --> D
        H --> E
        H --> F
        H --> G
        H --> I[models]
        H --> J[schema]
        H --> K[services]
        
        L[chat.rs] --> M[database]
        L --> I
        L --> J
        
        C --> M
        C --> I
        
        subgraph Core
            A
            B
            H
        end
        
        subgraph Frontend
            F
            G
        end
        
        subgraph Backend
            C
            D
            E
            I
            J
            K
            M
        end
        
        subgraph Features
            L
        end

    "#;

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
        <div class="flex flex-row text-mint-700 bg-black p-4">
            <div>
                <h2 class="ib text-3xl mb-4">"Mermaid Diagram Demo"</h2>
                <div _ref=diagram_ref id="mermaid-diagram" class="mermaid">
                    {diagram}
                </div>
            </div>
            <div>
                <p class="mt-4">"Render status: " {render_status}</p>
                <pre class="mt-4 p-2 bg-purple-900 rounded">{diagram}</pre>
            </div>
        </div>
    }
}
