use std::fs;
use yew::prelude::*;

mod detail;

pub use detail::*;

pub struct PostList {
    link: ComponentLink<Self>,
}

impl Component for PostList {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link: link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // let paths = fs::read_dir(".").unwrap();
        // let mut title: Vec<String> = Vec::new();
        // title.push("hello world".to_string());
        // for path in paths {
        //     title.push(path.unwrap().path().display().to_string());
        // }
        html! {
         <div class="posts">
             <div class="gradient-bg">
                    <img src="https://images7.alphacoders.com/695/695874.jpg"/>
                    <div class="gradient-horizontal"/>
             </div>
             <div class="row">
                    <div class="six columns">{"aa"}</div>
                    <div class="six columns">
                        <div class="container">
                            <div class="post-list">
                                <h1>
                                    {"Posts"}
                                </h1>
                                <p>{"random thought, stories, and some useless knowledge"}</p>
                                <div class="row">
                                    <div class="eleven columns">
                                        <input class="u-full-width" type="text" placeholder="looking for something" id="searchInput"/>
                                    </div>
                                    <div class="one columns">
                                        <button class="button-primary">
                                            <i class="fas fa-search" style="font-size: 18px"></i>
                                        </button>
                                    </div>
                                </div>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item active">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                                <h2 class="post-item">{"Hello World"}</h2>
                            </div>
                        </div>
                    </div>
             </div>
        </div>
        }
    }
}
