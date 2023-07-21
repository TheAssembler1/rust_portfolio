use yew::{function_component, html, Html};

#[function_component]
pub fn Profile() -> Html {
    html! {
        <div id="profile-container">
            <p class="title">{ "\u{1F44B} Hi, welcome to my portfolio..." }</p>
            <center>
                <div id="profile-inner-container">
                    <p class="subtitle">{ "Education" }</p>
                    <hr/>

                    <p class="line-info">{ " - Major: Computer Science - Cloud Computing and Networking"  }</p>
                    <p class="line-info-end">{ "Aug 2020 - May 2024" }</p>
                    <div style="clear: both;"></div>

                    <p class="line-info">{ " - Minor: Mathematics" }</p>
                    <p class="line-info-end">{ "Aug 2020 - May 2024" }</p>
                    <div style="clear: both;"></div>

                    <p class="subtitle">{ "Career" }</p>
                    <hr/>

                    <p class="line-info">{ " - Crimer: Backend Engineer"  }</p>
                    <p class="line-info-end">{ "Jan 2023 - Present" }</p>
                    <div style="clear: both;"></div>

                    <p class="line-info">{ " - LSU Research Project: Cloud Computing Engineer" }</p>
                    <p class="line-info-end">{ "Feb 2023 - Present" }</p>
                    <div style="clear: both;"></div>

                    <p class="line-info">{ " - Rural Sourcing Inc: C# Associate" }</p>
                    <p class="line-info-end">{ "Aug 2022 - Dec 2022" }</p>
                    <div style="clear: both;"></div>

                    <p class="subtitle">{ "Personal Interests" }</p>
                    <hr/>
                    <p class="line-info">{ " - I have a deep passion for the Rust programming language" }</p>
                    <div style="clear: both;"></div>
                    <p class="line-info">{ " - I love reading sci-fi novels such as Dune" }</p>
                    <div style="clear: both;"></div>
                </div>
            </center>
        </div>
    }
}
