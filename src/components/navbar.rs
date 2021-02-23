use yew::prelude::*;
use yew_router::prelude::*;

#[path = "../router.rs"]
mod router;

use router::Routes;

pub fn navbar() -> Html {
    html! {
        <div class="navbar">
            <div class="container">
                <div class="nav-list">
                    <RouterAnchor<Routes> route=Routes::Home>
                        {"Home"}
                    </RouterAnchor<Routes>>
                    <RouterAnchor<Routes> route=Routes::Post>
                        {"Post"}
                    </RouterAnchor<Routes>>
                    <RouterAnchor<Routes> route=Routes::Post>
                        {"About"}
                    </RouterAnchor<Routes>>
                </div>
           </div>
       </div>
    }
}
