use yew::prelude::*

#[derive(Switch)]
pub enum AppRoute {
    #[at = "/post/{slug}"]
    Post(String),
    #[at = "/post"]
    Post,
    #[at = "/"]
    Home,
}
