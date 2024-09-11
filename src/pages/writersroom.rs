use leptos::*;

use crate::components::chat::Chat;
use crate::components::threadlist::ThreadList;
use crate::components::messagelist::MessageList;

#[component]
pub fn WritersRoom() -> impl IntoView {
    let (show_threads, set_show_threads) = create_signal(false);
    let (model, set_model) = create_signal("gpt-4o-mini".to_string());
    let (lab, set_lab) = create_signal("openai".to_string());
    let (thread_id, set_thread_id) = create_signal("0001".to_string());

    let handle_model_change = move |ev| {
        let value = event_target_value(&ev);
        set_model(value.clone());
    
        let new_lab = if value.contains("claude") {
            "anthropic"
        } else {
            "openai"
        };
        set_lab(new_lab.to_string());
    };

    view! {
        <div class="w-full flex flex-col justify-start pt-2 pl-2 pr-2 h-full">
            <div class="flex flex-row items-center justify-between">
                <button 
                    class="self-start ib text-xs md:text-sm text-gray-800 hover:text-gray-900 p-2 border-2 bg-teal-800 hover:bg-teal-900 border-gray-700 hover:border-gray-900" 
                    on:click=move |_| set_show_threads.update(|v| *v = !*v)
                >
                    {move || if show_threads.get() { "hide threads" } else { "show threads" }}
                </button>
                <select
                    class="self-start ib text-xs md:text-sm text-gray-800 hover:text-gray-900 p-2 border-2 bg-teal-800 hover:bg-teal-900 border-gray-700 hover:border-gray-900" 
                    on:change=handle_model_change
                >
                    <option value="claude-3-haiku-20240307">"claude-3-haiku"</option>
                    <option value="claude-3-sonnet-20240229">"claude-3-sonnet"</option>
                    <option value="claude-3-opus-20240229">"claude-3-opus"</option>
                    <option value="claude-3-5-sonnet-20240620">"claude-3-5-sonnet"</option>
                    <option value="gpt-4o-mini" selected="selected">"gpt-4o-mini"</option>
                    <option value="gpt-4o">"gpt-4o"</option>
                    <option value="gpt-4-turbo">"gpt-4-turbo"</option>
                </select>
            </div>
            <div class="flex flex-row items-start justify-between">
                <div class={move || if show_threads.get() { "block" } else { "hidden" }}>
                    <ThreadList
                        current_thread_id=thread_id
                        set_current_thread_id=set_thread_id
                        _lab=lab // might use later 

                    />
                </div>
                <div class="w-full flex flex-col content-end justify-between h-[calc(90vh-10px)]">
                    <MessageList
                        current_thread_id=thread_id
                    />
                    <Chat 
                        thread_id=thread_id 
                        model=model
                        lab=lab
                    />
                </div>
            </div>
        </div>
    }
}

