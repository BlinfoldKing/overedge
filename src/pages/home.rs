use yew::prelude::*;

pub struct Home {}

impl Component for Home {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
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
            <div></div>
            <h1>{"Home"}</h1>
        </div>
        }
    }
}
