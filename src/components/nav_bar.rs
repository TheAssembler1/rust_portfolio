use material_yew::top_app_bar_fixed::MatTopAppBarTitle;
use material_yew::MatTopAppBarFixed;
use yew::{function_component, html, Html};

#[function_component]
pub fn NavBar() -> Html {
    html! {
        <MatTopAppBarFixed >
            <MatTopAppBarTitle>{ "Noah Lewis" }</MatTopAppBarTitle>
        </MatTopAppBarFixed>
    }
}
