use yew_router::prelude::*;

#[derive(Switch, Clone)]
pub enum Routes {
    #[to = "/post"]
    Post,
    #[to = "/not-found"]
    NotFound,
    #[to = "/"]
    Home,
}

pub type AppRouter = Router<Routes>;
