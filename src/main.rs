use yew::prelude::*;

mod homepage;

use homepage::Homepage;

#[component]
fn App() -> Html {
    html! {
        <Homepage/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
