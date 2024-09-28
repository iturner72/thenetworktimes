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
        %%{init: { 
          'theme': 'base',
          'themeVariables': {
            'git0': '#60608d',
            'git1': '#fb5367',
            'git2': '#00aaa8',
            'git3': '#206d5f',
            'git4': '#042f2e',
            'git5': '#021312',
            'gitBranchLabel0': '#ffffff',
            'gitBranchLabel1': '#ffffff',
            'gitBranchLabel2': '#ffffff',
            'gitBranchLabel3': '#ffffff',
            'gitBranchLabel4': '#ffffff',
            'gitBranchLabel5': '#ffffff'
          }
        } }%%
        
        gitGraph
           commit id: "Last common commit"
           branch origin/siwf
           commit id: "Initial SIWF setup"
           commit id: "Merge from main"
           commit id: "Working prototype"
           commit id: "Context provider fix"
           branch siwf
           commit id: "Context cleanup"
           commit id: "Add witnessing page"
           commit id: "UI improvements"
           commit id: "Add badges"
           commit id: "Styling updates"
           commit id: "Rename pages"
           commit id: "Add pool to declarations"
           commit id: "Add Coinbase wallet"
           commit id: "Improve readability"
           commit id: "Community declarations update"
           commit id: "Init SIWF"
           commit id: "Remove AuthKit provider"
           commit id: "Farcaster hooks implementation"
           branch siwf-backup
           commit id: "Backup point"
           branch proposed-solution
           checkout origin/siwf
           merge siwf id: "Reset to origin/siwf"
           commit id: "Cherry-pick: Context cleanup"
           commit id: "Cherry-pick: Add witnessing page"
           commit id: "..."
           commit id: "Cherry-pick: Farcaster hooks implementation"
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
            <div class="w-10/12">
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
