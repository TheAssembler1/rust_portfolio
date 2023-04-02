use yew::{function_component, html, Html};

#[function_component]
pub fn Profile() -> Html {
    html! {
        <div id="profile-container">
            <p class="title">{ "\u{1F44B} Hi, welcome to my portfolio..." }</p>
            <p class="subtitle">{ "Education" }</p>
            <p class="line-info">{ "Major: Computer Science - Cloud Computing and Networking" }</p>
            <p class="line-info">{ "Minor: Mathematics" }</p>
            <p class="subtitle">{ "Career" }</p>
            <p class="line-info">{ "Rural Sourcing Inc: C# Associate" }</p>
            <p clas="line-info">{ "Crimer: Backend Engineer" }</p>
            <p class="line-info">{ "LSU Research: Cloud Computing Engineer" }</p>
            <p class="subtitle">{ "Personal Interests" }</p>
            <p class="line-info">{ "I have a deep passion for the Rust programming language" }</p>
            <p class="line-info">{ "I love reading sci-fi novels such as Dune" }</p>
        </div>
    }
}
