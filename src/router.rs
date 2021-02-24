use yew_router::prelude::*;

#[derive(Switch, Clone)]
pub enum Routes {
    #[to = "/post/{string}"]
    PostDetail(String),
    #[to = "/post"]
    Post,
    #[to = "/not-found"]
    NotFound,
    #[to = "/contact"]
    Contact,
    #[to = "/login"]
    Login,
    #[to = "/dashboard"]
    Dashboard,
    #[to = "/!"]
    Home,
}

pub type AppRouter = Router<Routes>;
