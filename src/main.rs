#![recursion_limit = "10000"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;
mod pages;
mod router;

struct Model {
    link: ComponentLink<Self>,
}

enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
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
        html! {
        <div>
            {components::navbar::navbar()}
            <router::AppRouter
                render=router::AppRouter::render(Self::switch)
                redirect=router::AppRouter::redirect(|route: yew_router::route::Route| {
                    router::Routes::NotFound
                })
           />
        </div>
        }
    }
}

impl Model {
    fn switch(switch: router::Routes) -> Html {
        match switch {
            router::Routes::PostDetail(slug) => {
                html! { <pages::post::PostDetail slug=slug/> }
            }
            router::Routes::Post => {
                html! { <pages::post::PostList/> }
            }
            router::Routes::Home => {
                html! { <pages::home::Home/> }
            }
            router::Routes::Contact => {
                html! { <pages::home::Home/> }
            }
            _ => {
                html! { <pages::error::Error/> }
            }
        }
    }
}

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
