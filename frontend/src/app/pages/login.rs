use std::rc::Rc;

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, use_state, Callback, Html, InputEvent, SubmitEvent, TargetCast as _};
use yew_autoprops::autoprops;
use yew_hooks::{use_async, use_effect_once};
use yew_router::hooks::{use_location, use_navigator};

use crate::{app::{components::Title, utils::get_current_user, Route}, net::bodies};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super)struct LoginQuery {
    #[serde(default)]
    pub next: Option<Route>,
}

#[autoprops]
#[function_component]
fn LoginForm(#[prop_or_default] next: &Option<Route>) -> Html {
    // Use stuff
    let username_state = use_state(String::new);
    let password_state = use_state(String::new);
    let error_state = use_state(|| None::<String>);
    let navigator = use_navigator().expect("Navigator not found");

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

    // Create the onsubmit handler
    let on_submit = {
        // Clone stuff
        let username = (*username_state).clone();
        let password = (*password_state).clone();
        let error_state = error_state.clone();
        let navigator = navigator.clone();
        let next = next.clone();

        // Create the callback
        Callback::from(move |e: SubmitEvent| {
            // Prevent browser default form submission
            e.prevent_default();

            // Clone stuff
            let credentials = bodies::LoginBody {
                username: username.clone(),
                password: password.clone(),
            };
            let error_state = error_state.clone();
            let navigator = navigator.clone();
            let next = next.clone();

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
                let request = match Request::post("/backend/login")
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

                // Do an action based on the response status
                match response.status() {
                    200 => {
                        error_state.set(None);
                        if let Some(route) = next {
                            navigator.push(&route);
                        } else {
                            navigator.push(&Route::Landing);
                        }
                    }
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
                value="Login"
                class={ classes!("mb-5", "px-3", "py-2", "rounded", "border-3", "border-gray-300", "bg-amber-200", "active:bg-amber-300", "cursor-pointer") }
            />
        </form>
    }
}


#[function_component]
pub(in crate::app) fn LoginPage() -> Html {
    // Use stuff
    let location = use_location();
    let user_fetch = use_async(async { get_current_user().await.map_err(|err| Rc::new(err)) });

    // Attempt to parse the next url
    let next = location.and_then(|location| {
        location
            .query::<LoginQuery>()
            .ok()
            .and_then(|query| query.next)
    });

    // Fetch the current user
    {
        let user_fetch = user_fetch.clone();
        use_effect_once(move || {
            user_fetch.run();
            || ()
        })
    }

    html! {
        <>
            <Title>{ "Login" }</Title>
            {
                if user_fetch.loading {
                    html! {
                        <p>{ "Loading active user..." }</p>
                    }
                } else if let Some(err) = &user_fetch.error {
                    html! {
                        <p>{ format!("Error fetching the current user: {}", err.to_string()) }</p>
                    }
                } else if let Some(data) = &user_fetch.data {
                    if let Some(_user) = data {
                        html! {
                            <p>{ "You are already logged in!" }</p>
                        }
                    } else {
                        html! {
                            <LoginForm next={ next } />
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
