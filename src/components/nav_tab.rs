use crate::route::Route;
use material_yew::{MatTab, MatTabBar};
use yew::{function_component, html, Html};
use yew_router::hooks::use_navigator;

#[function_component]
pub fn NavTab() -> Html {
    let navigator = use_navigator().unwrap();

    html! {
        <MatTabBar onactivated={move |index| {
            match index {
                0 => navigator.push(&Route::Root),
                1 => navigator.push(&Route::Blog),
                2 => navigator.push(&Route::ContactMe),
                _ => panic!("Invalid tab bar index"),
            }
        }}>
            <MatTab label="profile" />
            <MatTab label="blog" />
            <MatTab label="contact-me" />
        </MatTabBar>
    }
}
