use pages::*;
use serde::{Deserialize, Serialize};
use yew::{Html, classes, function_component, html};
use yew_autoprops::autoprops;
use yew_router::{BrowserRouter, Routable, Switch};

pub(self) mod forms;
pub(self) mod pages;
pub(self) mod queries;
pub(self) mod state;
pub(self) mod utils;

#[derive(Debug, Clone, Routable, PartialEq, Serialize, Deserialize)]
pub(self) enum Route {
    #[at("/")]
    Landing,
    #[at("/login")]
    Login,
    #[at("/logout")]
    Logout,
    #[at("/admin")]
    Admin,
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
        Route::Logout => html! {
            <LogoutPage />
        },
        Route::Admin => html! {
            <AdminPage />
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
