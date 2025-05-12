use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="min-h-screen bg-gradient-to-br from-indigo-500 to-purple-700 flex items-center justify-center p-4">
            <div class="max-w-md w-full bg-white rounded-xl shadow-2xl overflow-hidden transform transition-all hover:scale-105 duration-300">
                <div class="p-6 sm:p-8">
                    <div class="text-center">
                        // Chat icon
                        <svg class="h-16 w-16 text-indigo-600 mx-auto" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                        </svg>
                        <h2 class="mt-6 text-3xl font-extrabold text-gray-900">
                            {"Welcome to YewChat"}
                        </h2>
                        <p class="mt-2 text-sm text-gray-600">
                            {"Enter your username to start chatting"}
                        </p>
                    </div>

                    <div class="mt-8">
                        <div class="rounded-md shadow-sm">
                            <input
                                {oninput}
                                class="appearance-none rounded-lg relative block w-full px-4 py-3 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-lg transition duration-300"
                                placeholder="Username"
                                autocomplete="off"
                            />
                        </div>
                        <div class="mt-6">
                            <Link<Route> to={Route::Chat}>
                                <button
                                    {onclick}
                                    disabled={username.len() < 1}
                                    class="group relative w-full flex justify-center py-3 px-4 border border-transparent text-lg font-medium rounded-lg text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 transition duration-300 disabled:opacity-50 disabled:cursor-not-allowed"
                                >
                                    {"Start Chatting"}
                                </button>
                            </Link<Route>>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}