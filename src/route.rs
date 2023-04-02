use crate::components;
use yew::{html, Html};
use yew_router::Routable;

// FIXME: move this and switch function to own module
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
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

pub fn switch(route: Route) -> Html {
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
