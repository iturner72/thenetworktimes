use leptos::*;

use crate::components::cast_list::CastList;
use crate::components::channels::Channels;

#[component]
pub fn Home() -> impl IntoView {
    let (channel, set_channel) = create_signal("networktimes".to_string());

    view! {
        <div class="home-feed-container flex flex-col md:flex-row justify-center pt-2 bg-gray-300 dark:bg-teal-900">
            <Channels set_active_channel=set_channel/>
            <CastList active_channel=channel/>
        </div>
    }
}
