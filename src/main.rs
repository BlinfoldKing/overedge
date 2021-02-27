#![recursion_limit = "10000"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;
mod pages;
mod router;

use dotenv::dotenv;

struct Model {
    link: ComponentLink<Self>,
    api_url: String,
}

#[derive(yew::Properties, Clone)]
pub struct Props {
    pub api_url: String,
}

enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link,
            api_url: props.api_url,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let url = self.api_url.clone();
        html! {
        <div>
            <components::navbar::Navbar/>
            <router::AppRouter
                render=router::AppRouter::render(move |switch: router::Routes| {
                    let api_url = url.clone();
                    match switch {
                        router::Routes::PostDetail(slug) => {
                            html! { <pages::post::PostDetail slug=slug api_url=api_url/> }
                        }
                        router::Routes::Post => {
                            html! { <pages::post::PostList api_url=api_url/> }
                        }
                        router::Routes::Home => {
                            html! { <pages::post::PostList api_url=api_url/> }
                        }
                        router::Routes::Contact => {
                            html! { <pages::home::Home/> }
                        }
                        _ => {
                            html! { <pages::error::Error/> }
                        }
                    }
                })
                redirect=router::AppRouter::redirect(|route: yew_router::route::Route| {
                    router::Routes::NotFound
                })
           />
        </div>
        }
    }
}

pub fn main() {
    // dotenv().ok();
    wasm_logger::init(wasm_logger::Config::default());
    // use std::env;
    yew::start_app_with_props::<Model>(Props {
        api_url: "http://localhost:8000/api/posts".to_owned(),
    });
}
