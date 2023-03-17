use material_yew::MatTopAppBarFixed;
use material_yew::top_app_bar::MatTopAppBarTitle;
use material_yew::top_app_bar::MatTopAppBar
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <MatTopAppBarFixed >
            <MatTopAppBarTitle>
                {
                    "Noah Lewis"
                }
            </MatTopAppBarTitle>
        </MatTopAppBarFixed>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
