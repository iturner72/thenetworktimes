use leptos::*;

#[component]
pub fn Settings() -> impl IntoView {
    view! {
        <div class="flex flex-col justify-center pt-8 bg-gray-200 dark:bg-teal-900 min-h-screen">
            <h1 class="text-3xl font-bold text-seafoam-700 dark:text-mint-400 mb-6 text-center">"Settings"</h1>
            <div class="max-w-2xl mx-auto bg-gray-100 dark:bg-teal-800 p-6 rounded-lg shadow-lg">
                <h2 class="text-2xl font-semibold text-teal-700 dark:text-aqua-300 mb-4">"General Settings"</h2>
                <div class="mb-4">
                    <label for="username" class="block text-gray-700 dark:text-gray-300 mb-2">"Username"</label>
                    <input type="text" id="username" class="w-full px-3 py-2 bg-white dark:bg-teal-700 text-gray-900 dark:text-gray-100 border border-gray-300 dark:border-teal-600 rounded-md focus:outline-none focus:ring-2 focus:ring-seafoam-500"/>
                </div>
                <div class="mb-4">
                    <label for="email" class="block text-gray-700 dark:text-gray-300 mb-2">"Email"</label>
                    <input type="email" id="email" class="w-full px-3 py-2 bg-white dark:bg-teal-700 text-gray-900 dark:text-gray-100 border border-gray-300 dark:border-teal-600 rounded-md focus:outline-none focus:ring-2 focus:ring-seafoam-500"/>
                </div>
                <div class="mb-6">
                    <label class="flex items-center">
                        <input type="checkbox" class="form-checkbox h-5 w-5 text-seafoam-600 dark:text-aqua-400"/>
                        <span class="ml-2 text-gray-700 dark:text-gray-300">"Receive notifications"</span>
                    </label>
                </div>
                <button class="w-full bg-seafoam-600 hover:bg-seafoam-700 dark:bg-aqua-600 dark:hover:bg-aqua-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition duration-300">
                    "Save Changes"
                </button>
            </div>
        </div>
    }
}
