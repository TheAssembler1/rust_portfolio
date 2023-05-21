use gloo_console::log;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures;
use yew::{function_component, html, use_effect_with_deps, use_state, Html, UseStateHandle};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct DbBlog {
    id: u64,
    title: String,
    html: String,
}

#[function_component]
pub fn Blog() -> Html {
    let blogs: UseStateHandle<Option<Vec<DbBlog>>> = use_state(|| None);

    use_effect_with_deps(
        |_| {
            wasm_bindgen_futures::spawn_local(async {
                let result: Result<Vec<DbBlog>, _> = Request::get("http://127.0.0.1:8080/blog")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await;
            });
        },
        (),
    );

    if blogs.is_none() {
        return html! {
            <h1>{ "Loading" }</h1>
        };
    }

    html! {
        <h1>{ "Blob" }</h1>
    }
}
