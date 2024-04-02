use crate::components::shared::svg::new_tab_svg::NewTabSVG;
use yew::{function_component, html, Html};

#[function_component(Resume)]
pub fn resume() -> Html {
    html! {
        // content
        <div class="frame resume-frame">
        // content box
            <div class="content-frame">
                <div class="content">
                    <h2>
                        {"About"}
                    </h2>
                    <p>
                        {"I am a Statistics graduate from Padjadjaran University.
                        Currently working as Lead Data and AI Services at IBM-JTI. I have 
                        worked on Data Warehouse, Backend Development, DevOps, 
                        Data Governance, and LLM Projects. Besides my main job, I also 
                        exploring Rust for web development and software 
                        development in general."}
                    </p>
                    <br/>
                    <h2>
                        {"Primary Toolkits"}
                    </h2>
                    <ol>
                        <li>
                            <p>
                                {"- Programming Language: Rust, Python"}
                            </p>
                        </li>
                        <li>
                            <p>
                                {"- Data Analytics: Pandas, Numpy, Scikit-learn, Tensorflow"}
                            </p>
                        </li>
                        <li>
                            <p>
                                {"- Web Dev: Axum, Yew, HTML5, CSS3"}
                            </p>
                        </li>
                        <li>
                            <p>
                                {"- Databases: PostgreSQL, MongoDB, CloudantDB, ELK"}
                            </p>
                        </li>
                        <li>
                            <p>
                                {"- Development Tools: Git, Docker, OKD/K8s, IBM Cloud"}
                            </p>
                        </li>
                    </ol>
                    <br />
                    <h2>
                        {"Currently Reading"}
                    </h2>
                    <p>
                        {"Software Engineering at Google."}
                    </p>
                    <br/>
                    <h2>
                        {"Download Resume "}
                        <span>
                            {"[ "}
                            <a
                                class="link-new-tab"
                                href="https://drive.google.com/file/d/1cFOMYojhK7rQpwDqr0TukBozEJ9jiIth/view?usp=sharing"
                                target="_blank"
                                aria-label="download resume"
                                >
                                {"link "}
                                <NewTabSVG />
                            </a>
                            {"]"}
                        </span>
                    </h2>
                    <blockquote>
                        <i>
                            {"Updated on 29 Mar 2024"}
                        </i>
                    </blockquote>
                    <br/>
                    <p>
                        {"If you would like a full CV or would like me to elaborate on any
                        points, please send me an email at hanifanrizki@gmail.com."}
                    </p>
                    <br/>
                </div>
            </div>
        </div>
    }
}
