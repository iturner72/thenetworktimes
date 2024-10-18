use leptos::*;
use leptos_router::A;
use crate::components::dark_mode_toggle::DarkModeToggle;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="flex justify-between items-center bg-gray-300 dark:bg-teal-800 px-6 py-4">
            <A href="/" class="text-2xl text-teal-600 dark:text-mint-400 hover:text-teal-800 dark:hover:text-mint-300">"nwt"</A>
            <A href="/feed" class="text-2xl text-seafoam-600 dark:text-aqua-400 hover:text-seafoam-800 dark:hover:text-aqua-300">"feed"</A>
            <A href="/writersroom" class="text-2xl text-teal-600 dark:text-mint-400 hover:text-teal-800 dark:hover:text-mint-300">"yap"</A>
            <A href="/settings" class="text-2xl text-seafoam-600 dark:text-aqua-400 hover:text-seafoam-800 dark:hover:text-aqua-300">"advanced"</A>
            <A href="/codedemo" class="text-2xl text-teal-600 dark:text-mint-400 hover:text-teal-800 dark:hover:text-mint-300">"hljs"</A>
            <A href="/mermaiddemo" class="text-2xl text-seafoam-600 dark:text-aqua-400 hover:text-seafoam-800 dark:hover:text-aqua-300">"mermaid"</A>
            <DarkModeToggle />
        </div>
    }
}
