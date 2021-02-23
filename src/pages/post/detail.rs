use yew::prelude::*;

pub struct PostDetail {
    link: ComponentLink<Self>,
    slug: String,
}

#[derive(yew::Properties, Clone)]
pub struct Props {
    pub slug: String,
}

impl Component for PostDetail {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link,
            slug: props.slug,
        }
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
            <h1>{"post list"} {self.slug.clone()}</h1>
        </div>
        }
    }
}
