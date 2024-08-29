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

//        <Script src="/pkg/highlight.min.js"/>
//        <Script>
//            "hljs.highlightAll();"
//        </Script>

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
                </Routes>
            </main>
        </Router>
    }
}


