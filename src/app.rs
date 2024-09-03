use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::home::Home;
use crate::pages::hometest::HomeTest;
use crate::components::navbar::Navbar;
use crate::components::profile::Profile;
use crate::components::cache_provider::provide_client_cache;
use crate::pages::settings::Settings;
use crate::pages::writersroom::WritersRoom;
use crate::pages::codedemo::CodeDemo;
use crate::pages::mermaiddemo::MermaidDemo;

#[component]
pub fn App() -> impl IntoView {
    // provide the client cache through context
    provide_client_cache();
    
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/thenetworktimes.css"/>

        // sets the document title
        <Title text="thenetworktimes"/>

        <Script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js"/>
        <Script>
            "hljs.highlightAll();"
        </Script>

        <Script type_="module">
            r#"
            import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs';
            
            console.log('Initializing Mermaid');
            mermaid.initialize({ 
                startOnLoad: false,
                theme: 'default',
                securityLevel: 'loose',
                logLevel: 'debug'
            });

            async function render_mermaid(elementId, diagram) {
                console.log('Rendering Mermaid diagram', { elementId, diagram });
                const element = document.getElementById(elementId);
                try {
                    const { svg } = await mermaid.render('mermaid-svg-' + elementId, diagram.trim());
                    console.log('Mermaid render result', svg);
                    element.innerHTML = svg;
                } catch (error) {
                    console.error('Mermaid render error', error);
                    element.textContent = 'Error rendering diagram: ' + error.message;
                }
            }

            document.addEventListener('render-mermaid', async function(event) {
                console.log('Render Mermaid event received', event);
                const element = event.target;
                const diagram = event.detail;
                await render_mermaid(element.id, diagram);
            });

            window.render_mermaid = render_mermaid;

            // Use mermaid.run instead of mermaid.init
            mermaid.run().then(() => {
                console.log('Mermaid setup complete');
            }).catch((error) => {
                console.error('Mermaid setup error', error);
            });
            "#
        </Script>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <nav>
                <Navbar/>
            </nav>
            <main>
                <Routes>
                    <Route path="" view=HomeTest/>
                    <Route path="feed" view=Home/>
                    <Route path="writersroom" view=WritersRoom/>
                    <Route path="settings" view=Settings/>
                    <Route path="profile/:id" view=Profile/>
                    <Route path="codedemo" view=CodeDemo/>
                    <Route path="mermaiddemo" view=MermaidDemo/>
                </Routes>
            </main>
        </Router>
    }
}


