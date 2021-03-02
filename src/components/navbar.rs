use yew::prelude::*;
use yew_router::prelude::*;

#[path = "../router.rs"]
mod router;

#[path = "../js.rs"]
mod js;

use router::Routes;

pub struct Navbar {
    link: ComponentLink<Self>,
    dark: bool,
    slug: Option<String>,
}

pub enum Message {
    ToggleMode,
}

#[derive(yew::Properties, Clone)]
pub struct Props {
    pub slug: String,
}

impl Component for Navbar {
    type Properties = Props;
    type Message = Message;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut res = Self {
            link: link,
            dark: !Self::get_cookie(),
            slug: if props.slug == "" {
                None
            } else {
                Some(props.slug.clone())
            },
        };

        res.toggle_mode();

        res
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

        if let Some(slug) = self.slug.clone() {
            js::disqus_reset(slug);
        }
        Self::set_cookie(self.dark)
    }

    fn get_cookie() -> bool {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        match storage.get_item("blinfoldking_theme").unwrap() {
            None => false,
            Some(theme) => theme == "dark",
        }
    }

    fn set_cookie(dark: bool) {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        storage
            .set_item("blinfoldking_theme", if dark { "dark" } else { "light" })
            .ok();
    }
}
