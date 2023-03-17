use material_yew::MatButton;
use yew::html;
use yew::prelude::*;

#[function_component]
fn NavBar() -> Html {
    html! {
        <MatButton label="test" />    
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <NavBar />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
