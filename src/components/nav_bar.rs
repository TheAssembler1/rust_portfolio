use yew::{ Html, html, function_component, Callback, MouseEvent };
use material_yew::{ MatButton, MatTopAppBarFixed };
use material_yew::top_app_bar::{ MatTopAppBarNavigationIcon, MatTopAppBarActionItems };
use material_yew::top_app_bar_fixed::MatTopAppBarTitle;
use gloo_console::log;

#[function_component]
pub fn NavBar() -> Html {
    let onclick: Callback<MouseEvent> = Callback::from(move |_| {
        log!("Admin Login");
    });

    html! {
        <MatTopAppBarFixed >
            <MatTopAppBarTitle>{ "Noah Lewis" }</MatTopAppBarTitle>
            <MatTopAppBarActionItems>
                <MatTopAppBarNavigationIcon>
                    <span onclick={ onclick }>
                        <MatButton outlined=true label="Admin Login" />
                    </span>
                </MatTopAppBarNavigationIcon>
            </MatTopAppBarActionItems>
        </MatTopAppBarFixed>
    }
}
