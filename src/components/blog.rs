use gloo_console::log;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures;
use yew::{
    function_component, html, use_effect_with_deps, use_state, AttrValue, Html, UseStateHandle,
};

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
    let blogs_clone = blogs.clone();

    use_effect_with_deps(
        move |_| {
            let blogs = blogs_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result: Result<Vec<DbBlog>, _> = Request::get("http://127.0.0.1:8079/blog")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await;

                blogs.set(result.ok());
            });
        },
        (),
    );

    if blogs.is_none() {
        return html! {
            <h1>{ "Loading" }</h1>
        };
    }

    let blogs = blogs.as_ref().unwrap();

    if blogs.len() <= 0 {
        return html! {
            <center>
                <h1>{ "Sorry... Currently there are no blogs available :(" }</h1>
            </center>
        };
    }

    blogs
        .into_iter()
        .map(|blog| {
            html! {
                <center>
                    <div key={ blog.id }>
                        <h1>{ blog.title.clone() }</h1>
                        { Html::from_html_unchecked(AttrValue::from(blog.html.clone())) }
                    </div>
                </center>
            }
        })
        .collect::<Html>()
}
