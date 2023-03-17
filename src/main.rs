use yew::{ Html, html, function_component };

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <components::nav_bar::NavBar />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
