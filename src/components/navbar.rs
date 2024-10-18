use leptos::*;
use leptos_router::A;
use crate::components::dark_mode_toggle::DarkModeToggle;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="flex justify-between items-center ib bg-salmon-500 dark:bg-catppuccin-surface0 px-6 py-4">
            <A href="/" class="text-2xl text-salmon-400 dark:text-catppuccin-rosewater hover:text-salmon-600 dark:hover:text-catppuccin-flamingo">"nwt"</A>
            <A href="/feed" class="text-2xl text-wenge-300 dark:text-catppuccin-lavender hover:text-mint-700 dark:hover:text-catppuccin-blue">"feed"</A>
            <A href="/writersroom" class="text-2xl text-salmon-400 dark:text-catppuccin-peach hover:text-salmon-600 dark:hover:text-catppuccin-yellow">"yap"</A>
            <A href="/settings" class="text-2xl text-salmon-400 dark:text-catppuccin-green hover:text-salmon-600 dark:hover:text-catppuccin-teal">"advanced"</A>
            <A href="/codedemo" class="text-2xl text-gray-700 dark:text-catppuccin-mauve hover:text-salmon-600 dark:hover:text-catppuccin-pink">"hljs"</A>
            <A href="/mermaiddemo" class="text-2xl text-mint-700 dark:text-catppuccin-sky hover:text-salmon-600 dark:hover:text-catppuccin-sapphire">"mermaid"</A>
            <DarkModeToggle />
        </div>
    }
}
