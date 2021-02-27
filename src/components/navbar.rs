use yew::prelude::*;
use yew_router::prelude::*;

#[path = "../router.rs"]
mod router;

use router::Routes;

pub struct Navbar {
    link: ComponentLink<Self>,
    dark: bool,
}

pub enum Message {
    ToggleMode,
}

impl Component for Navbar {
    type Properties = ();
    type Message = Message;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link,
            dark: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::ToggleMode => self.toggle_mode(),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
         <div class="navbar">
             <div class="nav-list">
                 <a onclick=self.link.callback(|_| Message::ToggleMode)>
                        {
                            if !self.dark {
                                html!{
                                    <i class="fas fa-sun"></i>
                                }
                            } else {
                                html!{
                                    <i class="fas fa-moon"></i>
                                }
                            }
                        }
                 </a>
                 <RouterAnchor<Routes> route=Routes::Home>
                     <i class="fas fa-home"></i>
                 </RouterAnchor<Routes>>
             </div>
        </div>
         }
    }
}

impl Navbar {
    fn toggle_mode(&mut self) {
        let dom = web_sys::window().unwrap().document().unwrap();
        let html = dom.get_elements_by_tag_name("html").item(0).unwrap();
        if !self.dark {
            html.set_attribute("data-theme", "dark").ok();
            self.dark = true
        } else {
            html.set_attribute("data-theme", "light").ok();
            self.dark = false
        }
    }
}
