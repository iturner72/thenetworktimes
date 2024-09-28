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
gitGraph
   commit id: "Last common commit"
   branch origin/siwf
   commit id: "62e0f06"
   commit id: "39feefb"
   commit id: "c8a8d50"
   commit id: "6cb4751"
   branch siwf
   commit id: "0f73e64"
   commit id: "8e12a5c"
   commit id: "1ad7c8b"
   commit id: "4ac1113"
   commit id: "70385e7"
   commit id: "9eda3e8"
   commit id: "1429ee3"
   commit id: "24ac2db"
   commit id: "70b1ef4"
   commit id: "d3b40c2"
   commit id: "afb615b"
   commit id: "c89ef6f"
   commit id: "849093c"
   branch siwf-backup
   commit id: "Backup point"
   branch proposed-solution
   checkout origin/siwf
   merge siwf id: "Reset to origin/siwf"
   commit id: "Cherry-pick 0f73e64"
   commit id: "Cherry-pick 8e12a5c"
   commit id: "..."
   commit id: "Cherry-pick 849093c"
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
