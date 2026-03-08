// frontend/src/components/version.rs

// dependencies
use crate::hooks::{FetchState, use_version};
use yew::prelude::*;

#[component]
pub fn Version() -> Html {
    let version = use_version();

    match (*version).clone() {
        FetchState::Loading => html! {},
        FetchState::Success(ver) => html! { <span>{ format!("v{ver}") }</span> },
        FetchState::Error(_) => html! {},
    }
}