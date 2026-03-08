// frontend/src/hooks/use_version.rs

// dependencies
use crate::hooks::FetchState;
use gloo_net::http::Request;
use wasm_drydock_dev_shared::StatusResponse;
use yew::prelude::*;

#[hook]
pub fn use_version() -> UseStateHandle<FetchState<String>> {
    let state = use_state(|| FetchState::Loading);

    let value = state.clone();
    use_effect_with((), move |_| {
        let state = value.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let result = Request::get("/api/status").send().await;
            match result {
                Ok(response) => match response.json::<StatusResponse>().await {
                    Ok(status) => state.set(FetchState::Success(status.version)),
                    Err(e) => state.set(FetchState::Error(format!("Parse error: {e}"))),
                },
                Err(e) => state.set(FetchState::Error(format!("Network error: {e}"))),
            }
        });
    });

    state
}