use std::rc::Rc;

use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::{Callback, Html, MouseEvent, classes, function_component, html, use_state};
use yew_hooks::{use_async, use_effect_once};
use yew_router::hooks::use_navigator;

use crate::app::{Route, components::Title, utils::get_current_user};

#[function_component]
pub(in crate::app) fn LogoutPage() -> Html {
    // Use stuff
    let error_state = use_state(|| None::<String>);
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

    // Create the on click handler
    let on_click = {
        // Clone stuff
        let error_state = error_state.clone();
        let navigator = navigator.clone();

        // Create the callback
        Callback::from(move |e: MouseEvent| {
            // Prevent browser default button press
            e.prevent_default();

            // Clone stuff
            let error_state = error_state.clone();
            let navigator = navigator.clone();

            // Spawn the task
            spawn_local(async move {
                // Create the logout request
                let request = Request::post("/backend/logout");

                // Send the logout request
                let response = request.send().await;

                match response {
                    Ok(response) => {
                        match response.status() {
                            200 => {
                                navigator.push(&Route::Landing);
                            }
                            403 => {
                                error_state.set(Some("You are not logged in!".to_string()));
                            }
                            500 => {
                                error_state.set(Some("Internal server error".to_string()));
                            }
                            _ => {
                                error_state.set(Some("Internal frontend error".to_string()));
                            }
                        };
                    }
                    Err(err) => {
                        error_state.set(Some(err.to_string()));
                    }
                };
            });
        })
    };

    html! {
        <>
            <Title>{ "Logout" }</Title>
            {
                if let Some(err) = (*error_state).clone() {
                    html! {
                        <p>{ err }</p>
                    }
                } else {
                    html! {}
                }
            }
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
                            <button class={ classes!("px-3", "py-2", "rounded", "border-3", "border-gray-300", "bg-amber-200", "active:bg-amber-300", "cursor-pointer") } onclick={ on_click }>{ "Logout" }</button>
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
