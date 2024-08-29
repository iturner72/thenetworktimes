use leptos::*;
use leptos_router::A;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="flex justify-between items-center ib bg-salmon-500 px-6 py-4 bg-teal-800">
            <A href="/" class="text-2xl text-salmon-400 hover:text-salmon-600">"nwt"</A>
            <A href="/feed" class="text-2xl text-wenge-300 hover:text-mint-700">"feed"</A>
            <A href="/writersroom" class="text-2xl text-salmon-400 hover:text-salmon-600">"writersroom"</A>
            <A href="/settings" class="text-2xl text-salmon-400 hover:text-salmon-600">"settings"</A>
            <A href="/codedemo" class="text-2xl text-mint-700 hover:text-salmon-600">"highlightjs"</A>
        </div>
    }
}
