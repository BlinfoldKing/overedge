use yew::prelude::*;

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
        html! {
        <div>
            <h1>{"post list"}</h1>
        </div>
        }
    }
}
