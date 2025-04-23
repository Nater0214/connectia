use std::rc::Rc;

use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, use_effect_with, use_state, Callback, Html, InputEvent, SubmitEvent, TargetCast as _};
use yew_hooks::{use_async, use_effect_once};
use yew_router::hooks::use_navigator;

use crate::{app::{components::Title, utils::get_current_user, Route}, net::bodies};

use super::LoginQuery;

#[function_component]
pub(super) fn CreateUserForm() -> Html {
    // Use stuff
    let username_state = use_state(String::new);
    let password_state = use_state(String::new);
    let error_state = use_state(|| None::<String>);

    // Create the username input handler
    let handle_username_input = {
        let username_state = username_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_dyn_into().unwrap();
            username_state.set(input.value());
        })
    };

    // Create the password input handler
    let handle_password_input = {
        let password_state = password_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_dyn_into().unwrap();
            password_state.set(input.value());
        })
    };

    // Create the on submit handler
    let on_submit = {
        // Clone stuff
        let username = (*username_state).clone();
        let password = (*password_state).clone();
        let error_state = error_state.clone();

        // Create the callback
        Callback::from(move |e: SubmitEvent| {
            // Prevent the browser default form submission
            e.prevent_default();

            // Clone stuff
            let credentials = bodies::LoginBody {
                username: username.clone(),
                password: password.clone(),
            };
            let error_state = error_state.clone();

            // Spawn the task
            spawn_local(async move {
                // Serialize the credentials to json
                let credentials = match serde_json::to_string(&credentials) {
                    Ok(credentials) => credentials,
                    Err(error) => {
                        error_state.set(Some(error.to_string()));
                        return;
                    }
                };

                // Create a new request
                let request = match Request::post("/backend/create_user")
                    .header("Content-Type", "application/json")
                    .body(credentials)
                {
                    Ok(request) => request,
                    Err(_) => {
                        error_state.set(Some("Internal frontend error".to_string()));
                        return;
                    }
                };

                // Send the request and get a response
                let response = match request.send().await {
                    Ok(response) => response,
                    Err(_) => {
                        error_state.set(Some("Internal frontend error".to_string()));
                        return;
                    }
                };

                // If the response isn't ok then error
                if !response.ok() {
                    match response.status() {
                        401 => {
                            error_state.set(Some("Invalid credentials".to_string()));
                        }
                        500 => {
                            error_state.set(Some("Internal server error".to_string()));
                        }
                        _ => {
                            error_state.set(Some("Internal frontend error".to_string()));
                        }
                    }
                    return;
                }
            });
        })
    };

    // Return html for the form
    html! {
        <form onsubmit={ on_submit } novalidate=true>
            <div class={ classes!("mb-5") }>
                <label for="username">{ "Username:" }</label>
                <input
                    id="username"
                    class={ classes!("w-full", "mb-5", "px-3", "py-2", "rounded", "border-3", "border-gray-300", "bg-amber-200") }
                    type="text"
                    value={ (*username_state).clone() }
                    oninput={ handle_username_input }
                />
            </div>
            <div class={ classes!("mb-5") }>
                <label for="password">{ "Password:" }</label>
                <input
                    id="password"
                    class={ classes!("w-full", "mb-5", "px-3", "py-2", "rounded", "border-3", "border-gray-300", "bg-amber-200") }
                    type="password"
                    value={ (*password_state).clone() }
                    oninput={ handle_password_input }
                />
            </div>
            {
                if let Some(error) = &*error_state {
                    html! {
                        <p class={ classes!("text-red-500") }>{ error }</p>
                    }
                } else {
                    html! {}
                }
            }
            <input
                type="submit"
                value="Create User"
                class={ classes!("mb-5", "px-3", "py-2", "rounded", "border-3", "border-gray-300", "bg-amber-200", "active:bg-amber-300", "cursor-pointer") }
            />
        </form>
    }
}

#[function_component]
pub(in crate::app) fn AdminPage() -> Html {
    // Use stuff
    let user_fetch = use_async(async { get_current_user().await.map_err(|err| Rc::new(err)) });
    let navigator = use_navigator().expect("Navigator not found");

    // Fetch the current user
    {
        let user_fetch = user_fetch.clone();
        use_effect_once(move || {
            user_fetch.run();
            || ()
        })
    }

    // Effect to redirect if user is not logged in
    {
        let user_fetch = user_fetch.clone();
        let navigator = navigator.clone();
        use_effect_with(user_fetch, move |user_fetch| {
            if let Some(data) = &user_fetch.data {
                if data.is_none() {
                    let navigation_result = navigator.push_with_query(
                        &Route::Login,
                        &LoginQuery {
                            next: Some(Route::Admin),
                        },
                    );
                    if let Err(_err) = navigation_result {}
                }
            }
            || ()
        })
    }

    // Return html for this page
    html! {
        <>
            <Title>{ "Admin" }</Title>
            {
                if user_fetch.loading {
                    html! {
                        <p>{ "Loading admin panel..." }</p>
                    }
                } else if let Some(err) = &user_fetch.error {
                    html! {
                        <p>{ format!("Error fetching the current user: {}", err.to_string()) }</p>
                    }
                } else if let Some(data) = &user_fetch.data {
                    if let Some(user) = data {
                        if user.admin {
                            html! {
                            <div class={ classes!("w-1/2", "mx-auto") }>
                                <CreateUserForm />
                            </div>
                            }
                        } else {
                            html! {
                                <p>{ "You are not an admin!" }</p>
                            }
                        }
                    } else {
                        html! {
                            <p>{ "You are not logged in!" }</p>
                        }
                    }
                } else {
                    html! {
                        <p>{ "Initializing..." }</p>
                    }
                }
            }
        </>
    }
}
