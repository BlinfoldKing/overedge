use yew::prelude::*;
use yew_router::prelude::*;

#[path = "../router.rs"]
mod router;

use router::Routes;

pub fn navbar() -> Html {
    html! {
        <div class="navbar">
            <div class="nav-list">
                <RouterAnchor<Routes> route=Routes::Home>
                    <i class="fas fa-home"></i>
                </RouterAnchor<Routes>>
            </div>
       </div>
    }
}
