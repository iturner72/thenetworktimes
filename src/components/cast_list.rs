use leptos::*;
use crate::components::cast_entry::CastEntry;
use crate::models::farcaster::Cast;

#[component]
pub fn CastList(casts: Vec<Cast>) -> impl IntoView {
    view! {
        <div class="space-y-4">
            {casts.into_iter().map(|cast| view! { <CastEntry cast={cast} /> }).collect::<Vec<_>>()}
        </div>
    }
}
