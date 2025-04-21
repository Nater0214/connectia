use std::rc::Rc;

use yew::{AttrValue, Html, classes, function_component, html, use_effect_with};
use yew_autoprops::autoprops;
use yew_hooks::{use_async, use_effect_once};
use yew_router::hooks::use_navigator;

use super::{Route, Title, forms::*, utils::get_current_user};

#[autoprops]
#[function_component]
pub(super) fn ErrorPage(
    #[prop_or_default] error_num: u16,
    #[prop_or(AttrValue::Static("error"))] error_message: &AttrValue,
) -> Html {
    html! {
        <>
            <Title>{ error_num }</Title>
            <h2>{ error_message }</h2>
        </>
    }
}

#[function_component]
pub(super) fn LandingPage() -> Html {
    html! {
        <>
            <Title>{ "Welcome" }</Title>
        </>
    }
}

#[function_component]
pub(super) fn LoginPage() -> Html {
    html! {
        <>
            <Title>{ "Login" }</Title>
            <LoginForm />
        </>
    }
}

#[function_component]
pub(super) fn AdminPage() -> Html {
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
                    navigator.push(&Route::Login);
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
