use yew::prelude::*;

pub struct Error {}

impl Component for Error {
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
            <h1>{"404"}</h1>
            <p>{"Sorry, but the page you are searching are not of this dimension :<"}</p>
        </div>
        }
    }
}
