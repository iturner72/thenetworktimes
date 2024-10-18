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
        %%{init: {'theme': 'dark', 'themeVariables': {
            'primaryColor': '#1a1b26',
            'secondaryColor': '#24283b',
            'tertiaryColor': '#414868',
            'primaryTextColor': '#a9b1d6',
            'lineColor': '#7aa2f7',
            'nodeBorder': '#7aa2f7'
        }}}%%
        
        flowchart TB
            %% Define colors
            classDef clientBg fill:#1a1b26,stroke:#7aa2f7,color:#a9b1d6;
            classDef configBg fill:#24283b,stroke:#7aa2f7,color:#a9b1d6;
            classDef blockchainBg fill:#414868,stroke:#7aa2f7,color:#a9b1d6;
            classDef serverBg fill:#1f2335,stroke:#7aa2f7,color:#a9b1d6;
            classDef argaNode fill:#FFC627,stroke:#7aa2f7,color:#1a1b26;
            classDef farcasterNode fill:#8A4FFF,stroke:#7aa2f7,color:#a9b1d6;
            classDef integrationNode fill:#2FCCB0,stroke:#7aa2f7,color:#1a1b26;
        
            subgraph Client ["Client (Browser)"]
                A[User Interface]
                B[Web3Modal]
                C[WagmiProvider]
                D[Wagmi Client]
                AA[Arga UI Components]:::argaNode
                AF[Farcaster SDK Client]:::farcasterNode
            end
        
            subgraph Configuration ["Wagmi Configuration"]
                E[wagmiConfig]
                F[WalletConnect]
                G[Coinbase Wallet]
                H[wagmiCoreConfig]
                I[HTTP Provider]
            end
        
            subgraph Blockchain ["Blockchain Interaction"]
                J[Optimism Chain]
                K[User's Wallet]
            end
        
            subgraph Server ["Server-Side"]
                L[API Routes]
                M[Smart Contracts]
                AS[Arga Server Logic]:::argaNode
                ADB[(Arga Database)]:::argaNode
                FS[Farcaster Server Integration]:::farcasterNode
            end
        
            %% Wagmi Flow
            A -->|1. Connect Wallet| B
            B -->|2. Trigger Connection| C
            C -->|3. Use Hooks| D
            E -->|4. Config| C
            E -->|5. Setup| F
            E -->|6. Setup| G
            H -->|7. Core Config| I
            J <-->|8. RPC| I
            K <-->|9. Sign| C
            L <-->|10. Interact| C
            M <-->|11. Interact| J
        
            %% Arga Flow
            A -->|12. Use Arga UI| AA:::argaNode
            AA -->|13. Arga Actions| AS:::argaNode
            AS <-->|14. Store/Retrieve| ADB:::argaNode
        
            %% Farcaster Flow
            A -->|15. Farcaster Login| AF:::farcasterNode
            AF -->|16. Auth Request| FS:::farcasterNode
            FS <-->|17. Verify| ADB:::argaNode
        
            %% Integration Points
            AS <-->|18. Link Accounts| FS:::integrationNode
            C <-->|19. Verify Wallet| FS:::integrationNode
        
            %% Apply styles
            class A,B,C,D clientBg;
            class E,F,G,H,I configBg;
            class J,K blockchainBg;
            class L,M serverBg;
        
            %% Define different colored arrows
            linkStyle 0,1,2 stroke:#FF6B6B,stroke-width:2px;
            linkStyle 3,4,5,6 stroke:#4ECDC4,stroke-width:2px;
            linkStyle 7,8,9,10 stroke:#45B7D1,stroke-width:2px;
            linkStyle 11,12,13 stroke:#FFC627,stroke-width:2px;
            linkStyle 14,15,16 stroke:#8A4FFF,stroke-width:2px;
            linkStyle 17,18 stroke:#2FCCB0,stroke-width:2px;
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
        <div class="flex flex-col items-center text-teal-700 dark:text-mint-300 bg-gray-100 dark:bg-teal-900 p-6 min-h-screen">
            <div class="w-11/12 md:w-10/12 max-w-4xl">
                <h2 class="text-3xl font-bold mb-6 text-seafoam-700 dark:text-aqua-400">"Mermaid Diagram"</h2>
                <div class="bg-white dark:bg-teal-800 p-4 rounded-lg shadow-md">
                    <div _ref=diagram_ref id="mermaid-diagram" class="mermaid">
                        {diagram}
                    </div>
                </div>
            </div>
            <div class="w-11/12 md:w-10/12 max-w-4xl mt-6">
                <p class="text-gray-700 dark:text-gray-300">"Render status: " <span class="font-semibold">{render_status}</span></p>
                <pre class="mt-4 p-4 bg-gray-200 dark:bg-teal-800 text-gray-800 dark:text-gray-200 text-left rounded-lg overflow-x-auto">
                    {diagram}
                </pre>
            </div>
        </div>
    }
}
