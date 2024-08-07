use crate::components::shared::svg::new_tab_svg::NewTabSVG;
use yew::{function_component, html, Html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <h2>
                {"Get in touch | "}
                <span> { "hanifanrizki@gmail.com" }</span>
            </h2>

            <div class="get-in-touch-frame">
                <div class="get-in-touch-container">
                    <ol>
                        <li>
                            <a
                                class="_text"
                                href="https://github.com/hanifanrn"
                                target="_blank"
                                >
                                {"Github "}
                                <NewTabSVG />
                            </a>
                        </li>
                        <li>
                            <a
                                class="_text"
                                href="https://linkedin.com/in/hanifanrn"
                                target="_blank"
                                >
                                {"Linkedin "}
                                <NewTabSVG />
                            </a>
                        </li>
                        <li>
                            <a
                                class="_text"
                                href="https://leetcode.com/hanifanrn/"
                                target="_blank"
                                >
                                {"Leetcode "}
                                <NewTabSVG />
                            </a>
                        </li>
                        <li>
                            <a
                                class="_text"
                                href="https://medium.com/@hanifanrn"
                                target="_blank"
                                >
                                {"Medium "}
                                <NewTabSVG />
                            </a>
                        </li>
                        <li>
                            <a
                                class="_text"
                                href="https://twitter.com/syntaxmaker"
                                target="_blank"
                                >
                                {"Twitter "}
                                <NewTabSVG />
                            </a>
                        </li>
                    </ol>
                </div>
            </div>
        </footer>
    }
}
