use leptos::*;
use leptos_router::A;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="flex justify-between items-center ib bg-rich-black-200 px-6 py-4">
            <A href="/" class="text-2xl text-celestial-blue-300 hover:text-celestial-blue-100">"nwt"</A>
            <A href="/feed" class="text-2xl text-tyrian-purple-300 hover:text-tyrian-purple-100">"feed"</A>
            <A href="/writersroom" class="text-2xl text-celestial-blue-300 hover:text-celestial-blue-100">"yap"</A>
            <A href="/settings" class="text-2xl text-celestial-blue-300 hover:text-celestial-blue-100">"advanced"</A>
            <A href="/codedemo" class="text-2xl text-dark-purple-300 hover:text-celestial-blue-100">"hljs"</A>
            <A href="/mermaiddemo" class="text-2xl text-ucla-blue-300 hover:text-celestial-blue-100">"mermaid"</A>
        </div>
    }
}
