use yew::{function_component, html, Html};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="frame projects-frame">
            <div class="content-frame">
                <div class="content">
                    <h2 class="title">
                        {"404 Page Not Found"}
                    </h2>
                </div>
            </div>
        </div>
    }
}
