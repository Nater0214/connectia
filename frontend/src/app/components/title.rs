use yew::{Html, classes, function_component, html};
use yew_autoprops::autoprops;

#[autoprops]
#[function_component]
pub(in crate::app) fn Title(#[prop_or_default] children: &Html) -> Html {
    html! {
        <h1 class={ classes!("text-7xl", "text-center") }>{ children.clone() }</h1>
    }
}
