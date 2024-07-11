use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::home::Home;
use crate::pages::settings::Settings;
use crate::components::threadlist::ThreadList;
use crate::components::navbar::Navbar;
use crate::components::chat::Chat;
use crate::components::profile::Profile;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/thenetworktimes.css"/>

        // sets the document title
        <Title text="thenetworktimes"/>

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
                    <Route path="" view=Chat/>
                    <Route path="home" view=Home/>
                    <Route path="threadlist" view=ThreadList/>
                    <Route path="settings" view=Settings/>
                    <Route path="profile" view=Profile/>
                </Routes>
            </main>
        </Router>
    }
}
