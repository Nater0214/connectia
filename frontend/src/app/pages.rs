use std::rc::Rc;

use yew::{Html, classes, function_component, html, use_effect_with};
use yew_autoprops::autoprops;
use yew_hooks::{use_async, use_effect_once};
use yew_router::hooks::{use_location, use_navigator};

use super::{Route, Title, forms::*, queries::*, utils::get_current_user};

#[autoprops]
#[function_component]
pub(super) fn ErrorPage(
    #[prop_or_default] error_num: &Option<u16>,
    #[prop_or_default] error_message: &Option<String>,
) -> Html {
    html! {
        <>
            {
                if let Some(error_num) = error_num {
                    html! {
                        <Title>{ error_num }</Title>
                    }
                } else {
                    html! {}
                }
            }
            {
                if let Some(error_message) = error_message {
                    html! {
                        <p>{ error_message }</p>
                    }
                } else {
                    html! {}
                }
            }
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
    // Use stuff
    let location = use_location();

    // Attempt to parse the next url
    let next = location.and_then(|location| {
        location
            .query::<LoginQuery>()
            .ok()
            .and_then(|query| query.next)
    });

    html! {
        <>
            <Title>{ "Login" }</Title>
            <LoginForm next={ next } />
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
