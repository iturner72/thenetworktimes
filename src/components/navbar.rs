use leptos::*;
use leptos_router::A;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="flex justify-between items-center ib bg-salmon-500 px-6 py-4 bg-teal-800">
            <A href="/" class="text-2xl text-salmon-400 hover:text-salmon-600">"nwt"</A>
            <A href="/feed" class="text-2xl text-wenge-300 hover:text-mint-700">"feed"</A>
            <A href="/writersroom" class="text-2xl text-salmon-400 hover:text-salmon-600">"yap"</A>
            <A href="/settings" class="text-2xl text-salmon-400 hover:text-salmon-600">"advanced"</A>
            <A href="/codedemo" class="text-2xl text-gray-700 hover:text-salmon-600">"hljs"</A>
            <A href="/mermaiddemo" class="text-2xl text-mint-700 hover:text-salmon-600">"mermaid"</A>
        </div>
    }
}
