mod components;
mod route;

use route::switch;
use route::Route;
use yew::{function_component, html, Html};
use yew_router::switch::Switch;
use yew_router::BrowserRouter;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={ switch }/>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
