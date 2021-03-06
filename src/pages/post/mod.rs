use serde::Deserialize;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{
    format::{Json, Nothing},
    prelude::*,
};
use yew_router::prelude::*;

#[path = "../../router.rs"]
mod router;

mod detail;

pub use detail::*;

pub struct PostList {
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    post_list: Vec<ListItem>,
    error: bool,
    active_post: usize,
    query: String,
    api_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListItem {
    slug: String,
    title: String,
    categories: Vec<String>,
    author: String,
    thumbnail: String,
    subtitle: String,
    date: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListResponse {
    ok: bool,
    data: Vec<ListItem>,
}

#[derive(Debug)]
pub enum Msg {
    GetData,
    SetActive(usize),
    SetQuery(String),
    ReceiveResponse(Result<ListResponse, anyhow::Error>),
}

#[derive(yew::Properties, Clone)]
pub struct Props {
    pub api_url: String,
}

impl PostList {
    fn get_post_list(&mut self) {
        let request = Request::get(format!("{}?query={}", self.api_url, self.query))
            .body(Nothing)
            .expect("could not build request");
        let callback = self.link.callback(
            |response: Response<Json<Result<ListResponse, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::ReceiveResponse(data)
            },
        );

        let task = FetchService::fetch(request, callback).expect("failed to start fetch");
        self.fetch_task = Some(task);
    }
}

impl Component for PostList {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut res = Self {
            fetch_task: None,
            link: link,
            post_list: Vec::new(),
            error: false,
            active_post: 0,
            query: "".to_owned(),
            api_url: props.api_url,
        };
        res.get_post_list();
        res
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetData => {
                self.get_post_list();
                true
            }
            Msg::SetQuery(query) => {
                self.query = query;
                true
            }
            Msg::SetActive(index) => {
                self.active_post = index;
                true
            }
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(data) => self.post_list = data.data,
                    Err(_) => self.error = true,
                }
                self.active_post = 0;
                self.fetch_task = None;
                true
            }
        }
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
         <div class="posts" onload=self.link.callback(|_|Msg::GetData)>
             <div class="gradient-bg">
                    <img src={
                        if self.post_list.len() > 0 {
                            &self.post_list[self.active_post].thumbnail
                        } else {
                            "/static/noimage.jpg"
                        }
                    }/>
                    <div class="gradient-horizontal"/>
                    <div class="dark-gradient-horizontal"/>
             </div>
             <div class="row">
                    <div class="six columns"><br/></div>
                    <div class="six columns">
                        <div class="container">
                            <div class="post-list">
                                <h1>
                                    {"Hi,"}
                                </h1>
                                <p>{"random thought, stories, and some useless knowledge"}</p>
                                <div class="row">
                                    <form onsubmit=self.link.callback(|_| Msg::GetData) action="#">
                                        <input
                                            oninput=self.link.callback(|input: InputData| {
                                                Msg::SetQuery(input.value)
                                            })
                                            class="u-full-width" type="text" placeholder="Type Something and Press Enter" id="searchInput"/>
                                    </form>
                                </div>
                                {
                                    self.post_list
                                        .iter()
                                        .enumerate()
                                        .map(|(index, item): (usize, &ListItem)|{
                                            if index == self.active_post {
                                                html!{
                                                    <div class="post-preview">
                                                        <h2 class="active post-item">{&item.title}</h2>
                                                        <div class="content">
                                                            <p>{&item.subtitle}</p>
                                                            <RouterAnchor<router::Routes> route=router::Routes::PostDetail(item.slug.clone())>
                                                                {"Read More"}
                                                            </RouterAnchor<router::Routes>>
                                                        </div>
                                                    </div>
                                                }
                                            } else {
                                                html!{<h2
                                                    onclick=self.link.callback(move |_| Msg::SetActive(index))
                                                    class="post-item">{&item.title}</h2>}
                                            }
                                        })
                                        .collect::<Html>()
                                }
                           </div>
                        </div>
                    </div>
             </div>
        </div>
        }
    }
}
