use leptos::*;

#[component]
pub fn Toast(
    message: ReadSignal<String>,
    visible: ReadSignal<bool>,
    #[prop(into)] on_close: Callback<()>,
) -> impl IntoView {
    let opacity_class = move || {
        if visible.get() {
            "opacity-100"
        } else {
            "opacity-0"
        }
    };

    view! {
        <div
            class="fixed bottom-4 right-4 bg-rich-black-500 text-dark-purple-800 px-4 py-2 rounded shadow-lg transition-opacity duration-300"
            class=opacity_class
        >
            {message}
            <button
                class="ml-2 text-dark-purple-700 hover:text-dark-purple-800"
                on:click=move |_| leptos::Callable::call(&on_close, ())
            >
                "×"
            </button>
        </div>
    }
}
