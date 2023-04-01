use material_yew::{MatTab, MatTabBar};
use yew::{function_component, html, Html};

#[function_component]
pub fn NavTab() -> Html {
    html! {
        <MatTabBar>
            <MatTab label="profile" />
            <MatTab label="blog" />
            <MatTab label="contact-me" />
        </MatTabBar>
    }
}
