use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{
    AttrValue, Callback, Html, InputEvent, SubmitEvent, TargetCast as _, classes,
    function_component, html, use_state,
};
use yew_autoprops::autoprops;
use yew_router::{BrowserRouter, Routable, Switch};
use yewdux::{Store, use_store};

use crate::{bodies, responses};

#[derive(Debug, Clone, Eq)]
struct User {
    username: String,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
    }
}

#[derive(Debug, Clone, PartialEq, Store)]
struct State {
    current_user: Option<User>,
}

impl Default for State {
    fn default() -> Self {
        Self { current_user: None }
    }
}

#[autoprops]
#[function_component]
fn Title(#[prop_or_default] children: &Html) -> Html {
    html! {
        <h1 class={ classes!("text-7xl", "text-center") }>{ children.clone() }</h1>
    }
}

#[function_component]
fn LoginForm() -> Html {
    // Create states
    let username_state = use_state(String::new);
    let password_state = use_state(String::new);
    let error_state = use_state(|| None::<String>);
    let (_store, dispatch) = use_store::<State>();

    // Create the username input callback
    let handle_username_input = {
        let username_state = username_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_dyn_into().unwrap();
            username_state.set(input.value());
        })
    };

    // Create the password input callback
    let handle_password_input = {
        let password_state = password_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_dyn_into().unwrap();
            password_state.set(input.value());
        })
    };

    // Create the onsubmit callback
    let on_submit = {
        // Clone stuff
        let username = (*username_state).clone();
        let password = (*password_state).clone();
        let error_state = error_state.clone();
        let dispatch = dispatch.clone();

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
            let dispatch = dispatch.clone();

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

                // Parse the response as json
                let response: responses::LoginResponse = match response.json().await {
                    Ok(response) => response,
                    Err(_) => {
                        error_state.set(Some("Internal frontend error".to_string()));
                        return;
                    }
                };

                dispatch.reduce_mut(|state| {
                    state.current_user = Some(User {
                        username: response.username,
                    })
                });
            });
        })
    };

    html! {
        <form onsubmit={ on_submit }>
            <label for="username">{ "Username:" }</label>
            <input id="username" class={ classes!("w-full", "mb-5", "px-3", "py-2", "rounded", "border-3", "border-gray-300", "bg-amber-200") } type="text" value={ (*username_state).clone() } oninput={ handle_username_input } />
            <br />
            <label for="password">{ "Password:" }</label>
            <input id="password" class={ classes!("w-full", "mb-5", "px-3", "py-2", "rounded", "border-3", "border-gray-300", "bg-amber-200") } type="password" value={ (*password_state).clone() } oninput={ handle_password_input } />
            <input type="submit" value="Login" class={ classes!("mb-5", "px-3", "py-2", "rounded", "border-3", "border-gray-300", "bg-amber-200") } />
            <p class={ classes!("text-red-500") }>{ (*error_state).clone() }</p>
        </form>
    }
}

#[autoprops]
#[function_component]
fn ErrorPage(
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
fn HomePage() -> Html {
    html! {
        <Title>{ "Home" }</Title>
    }
}

#[function_component]
fn LoginPage() -> Html {
    html! {
        <>
            <Title>{ "Login" }</Title>
            <LoginForm />
        </>
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <HomePage />
        },
        Route::Login => html! {
            <LoginPage />
        },
        Route::NotFound => html! {
            <ErrorPage error_num={ 404 } error_message={ "Page not found" } />
        },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
