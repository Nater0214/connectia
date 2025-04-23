use yew::{Html, function_component, html};

use crate::app::components::Title;

#[function_component]
pub(in crate::app) fn LandingPage() -> Html {
    html! {
        <>
            <Title>{ "Welcome" }</Title>
        </>
    }
}
