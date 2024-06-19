use leptos::*;
use leptos_router::A;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="flex justify-between items-center ib bg-salmon-500 px-6 py-4 bg-teal-800">
            <A href="/" class="text-2xl text-salmon-400 hover:text-salmon-600">"nwtr"</A>
            <A href="/channels" class="text-2xl text-salmon-400 hover:text-salmon-600">"channels"</A>
            <A href="/settings" class="text-2xl text-salmon-400 hover:text-salmon-600">"settings"</A>
        </div>
    }
}
