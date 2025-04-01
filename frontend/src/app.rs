use yew::{AttrValue, Html, classes, function_component, html};
use yew_autoprops::autoprops;
use yew_router::{BrowserRouter, Routable, Switch};

#[autoprops]
#[function_component]
fn Title(#[prop_or_default] children: &Html) -> Html {
    html! {
        <h1 class={ classes!("text-7xl", "text-center") }>{ children.clone() }</h1>
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

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <HomePage />
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
