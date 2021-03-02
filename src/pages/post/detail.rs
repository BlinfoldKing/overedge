use serde::Deserialize;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{
    format::{Json, Nothing},
    prelude::*,
};
use yew_router::prelude::*;

#[path = "../../router.rs"]
mod router;

#[path = "../../js.rs"]
mod js;

pub struct PostDetail {
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    slug: String,
    error: bool,
    post: Option<Post>,
    api_url: String,
    loading: bool,
    comment: bool,
}

#[derive(yew::Properties, Clone)]
pub struct Props {
    pub slug: String,
    pub api_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Metadata {
    slug: String,
    title: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Post {
    slug: String,
    title: String,
    categories: Vec<String>,
    author: String,
    date: String,
    body: String,
    subtitle: String,
    thumbnail: String,
    next: Option<Metadata>,
    prev: Option<Metadata>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PostResponse {
    ok: bool,
    data: Post,
}

#[derive(Debug)]
pub enum Msg {
    GetData,
    ReceiveResponse(Result<PostResponse, anyhow::Error>),
    ShowComment,
}

impl PostDetail {
    fn render_comment(&self) -> Html {
        if !self.comment {
            return html! {
                <div id="comment">
                    <a href="#comment" onclick=self.link.callback(|_| Msg::ShowComment)>{"Show Discussion"}</a>
                </div>
            };
        }
        html! {
            <div>
                <div id="disqus_thread"></div>
                <noscript>{"Please enable JavaScript to view the"} <a href="https://disqus.com/?ref_noscript">{"comments powered by Disqus."}</a></noscript>
            </div>
        }
    }
    fn render_body(&self) -> Html {
        web_sys::window()
            .and_then(|window| window.document())
            .map_or_else(
                || {
                    html! {"Failed to resolve document"}
                },
                |document| match document.create_element("div") {
                    Ok(div) => {
                        div.set_inner_html(&self.post.clone().unwrap().body);
                        yew::virtual_dom::VNode::VRef(div.into())
                    }
                    Err(e) => html! {<p>{format!("{:?}", &e)}</p>},
                },
            )
    }
    fn render_post(&self) -> Html {
        // parse_markdown("hello world".to_owned());
        match &self.post {
            None => {
                if !self.loading {
                    html! {
                        <div class="title-container">
                            <h1>{"Sorry, this post can't be loaded :<"}</h1>
                            <a href="#" onclick=self.link.callback(|_| Msg::GetData)>{"Reload"}</a>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
            Some(post) => html! {
                <div>
                    <div class="gradient-bg">
                        <img src=post.thumbnail/>
                    </div>
                    <div class="content">
                        <div class="gradient-vertical">
                            <div class="title-container">
                                <h1>{&post.title}</h1>
                            </div>
                        </div>
                        <div class="content-body">
                            <div class="container">
                                <div class="post-content">
                                    {self.render_body()}
                                </div>
                                <hr/>
                                <div class="recommendation">
                                    {
                                        match post.prev.clone() {
                                            None => html!{<div><br/></div>},
                                            Some(prev) => html!{
                                                <div class="prev">
                                                    <div>{"Prev"}</div>
                                                    <a href=format!("/post/{}", prev.slug)>
                                                        {prev.title.to_owned()}
                                                    </a>
                                                </div>
                                            }
                                        }
                                    }
                                    {
                                        match post.next.clone() {
                                            None => html!{<div><br/></div>},
                                            Some(next) => html!{
                                                <div class="next">
                                                    <div>{"Next"}</div>
                                                    <a href=format!("/post/{}", next.slug)>
                                                        {next.title.to_owned()}
                                                    </a>
                                                </div>
                                            }
                                        }
                                    }
                                </div>
                            </div>
                           <div class="container">
                                {self.render_comment()}
                            </div>
                        </div>
                    </div>
                </div>
            },
        }
    }
}

impl PostDetail {
    fn get_post(&mut self) {
        let request = Request::get(format! {"{}/{}", self.api_url, self.slug})
            .body(Nothing)
            .expect("could not build request");
        let callback = self.link.callback(
            |response: Response<Json<Result<PostResponse, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::ReceiveResponse(data)
            },
        );

        let task = FetchService::fetch(request, callback).expect("failed to start fetch");
        self.loading = true;
        self.fetch_task = Some(task);
    }
}

impl Component for PostDetail {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut res = Self {
            link: link,
            slug: props.slug,
            post: None,
            error: false,
            fetch_task: None,
            api_url: props.api_url,
            loading: true,
            comment: false,
        };
        res.get_post();
        res
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetData => {
                self.get_post();
                true
            }
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(data) => self.post = Some(data.data),
                    Err(_) => self.error = true,
                }
                self.fetch_task = None;
                self.loading = false;
                true
            }
            Msg::ShowComment => {
                self.comment = true;
                js::disqus_reset(self.slug.clone());
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
        <div class="post-detail">
            {self.render_post()}
        </div>
        }
    }
}
