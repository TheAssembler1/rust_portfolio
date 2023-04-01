use std::clone::Clone;
use std::cmp::PartialEq;
use yew::{function_component, html, Html};
use yew_router::prelude::use_navigator;
use yew_router::switch::Switch;
use yew_router::{BrowserRouter, Routable};

mod components;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Root,
    #[at("/blog")]
    Blog,
    #[at("/contact-me")]
    ContactMe,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    let core_html = match route {
        Route::Root => html! {
            <components::profile::Profile />
        },
        Route::Blog => html! {
            <components::blog::Blog />
        },
        Route::ContactMe => html! {
            <components::contact_me::ContactMe />
        },
        Route::NotFound => html! {
            <h1>{ "Not Found" }</h1>
        },
    };

    html! {
        <>
            <components::nav_bar::NavBar />
            <components::nav_tab::NavTab />
            { core_html }
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={ switch }/>
        </BrowserRouter>
    }
    /*html! {
        <div>
            <components::nav_bar::NavBar />
            <components::nav_tab::NavTab />
        </div>
    }*/
}

fn main() {
    yew::Renderer::<App>::new().render();
}
