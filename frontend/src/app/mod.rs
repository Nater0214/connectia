use pages::*;
use yew::{Html, classes, function_component, html};
use yew_autoprops::autoprops;
use yew_router::{BrowserRouter, Routable, Switch};

mod forms;
mod pages;
mod state;

#[autoprops]
#[function_component]
fn Title(#[prop_or_default] children: &Html) -> Html {
    html! {
        <h1 class={ classes!("text-7xl", "text-center") }>{ children.clone() }</h1>
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Landing,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Landing => html! {
            <LandingPage />
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
