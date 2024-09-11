use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[component]
pub fn MermaidDemo() -> impl IntoView {
    let diagram = r#"
graph TD
    subgraph Client1 ["Client 1"]
        A1[User 1 Input] --> B1[EventSource]
    end

    subgraph Client2 ["Client 2"]
        A2[User 2 Input] --> B2[EventSource]
    end

    subgraph Server ["Server (Leptos 0.6 or 0.7)"]
        C[Axum SSE Handler]
        D[Tokio Runtime]
        E1[Tokio Task 1]
        E2[Tokio Task 2]
        F1[MPSC Channel 1]
        F2[MPSC Channel 2]
        G1[SseStream 1]
        G2[SseStream 2]
        H[AI Service]
    end

    B1 --> C
    B2 --> C
    C --> D
    D --> E1
    D --> E2
    E1 --> F1
    E2 --> F2
    F1 --> G1
    F2 --> G2
    E1 --> H
    E2 --> H
    G1 --> B1
    G2 --> B2

    classDef client fill:#446784,stroke:#DCE9E6,stroke-width:2px,color:#DCE9E6;
    classDef server fill:#206D5F,stroke:#DCE9E6,stroke-width:2px,color:#DCE9E6;
    classDef channel fill:#715F58,stroke:#DCE9E6,stroke-width:2px,color:#DCE9E6;
    classDef aiService fill:#00AAA8,stroke:#DCE9E6,stroke-width:2px,color:#DCE9E6;

    class A1,A2,B1,B2 client;
    class C,D,E1,E2,G1,G2 server;
    class F1,F2 channel;
    class H aiService;
    "#;

    let diagram_ref = create_node_ref::<html::Div>();
    let (render_status, set_render_status) = create_signal("Not rendered yet".to_string());

    create_effect(move |_| {
        if let Some(element) = diagram_ref.get() {
            log("attempting to render Mermaid diagram");
            let event_init = web_sys::CustomEventInit::new();
            event_init.set_detail(&JsValue::from_str(diagram));
            
            let event = web_sys::CustomEvent::new_with_event_init_dict(
                "render-mermaid",
                &event_init,
            )
            .unwrap();
            element.dispatch_event(&event).unwrap();
            set_render_status.set("render event dispatched".to_string());
        }
    });

    view! {
        <div class="flex flex-col items-center text-purple-400 bg-black p-4">
            <div>
                <h2 class="ib text-3xl mb-4">"mermaid"</h2>
                <div _ref=diagram_ref id="mermaid-diagram" class="mermaid">
                    {diagram}
                </div>
            </div>
            <div class="w-10/12">
                <p class="mt-4">"render status: " {render_status}</p>
                <pre class="mt-4 p-4 bg-purple-900 text-blue-100 text-left rounded">{diagram}</pre>
            </div>
        </div>
    }
}
