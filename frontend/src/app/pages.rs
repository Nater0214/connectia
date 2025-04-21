use yew::{AttrValue, Html, function_component, html};
use yew_autoprops::autoprops;

use crate::app::{Title, forms::LoginForm};

#[autoprops]
#[function_component]
pub fn ErrorPage(
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
pub fn LandingPage() -> Html {
    html! {
        <Title>{ "Welcome" }</Title>
    }
}

#[function_component]
pub fn LoginPage() -> Html {
    html! {
        <>
            <Title>{ "Login" }</Title>
            <LoginForm />
        </>
    }
}
