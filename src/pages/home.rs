use leptos::*;

use crate::components::cast_list::CastList;
use crate::components::channels::Channels;

#[component]
pub fn Home() -> impl IntoView {
    let (channel, set_channel) = create_signal("networktimes".to_string());

    view! {
        <div class="home-feed-container flex flex-row justify-center pt-2">
            <Channels set_active_channel=set_channel/>
            <CastList active_channel=channel/>
        </div>
    }
}
