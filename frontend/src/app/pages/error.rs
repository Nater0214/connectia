use yew::{Html, function_component, html};
use yew_autoprops::autoprops;

use crate::app::components::Title;

#[autoprops]
#[function_component]
pub(in crate::app) fn ErrorPage(
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
