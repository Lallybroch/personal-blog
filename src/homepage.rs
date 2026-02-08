use yew::prelude::*;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct Props {
    social_tag: String,
}

#[component]
pub fn Homepage() -> Html {
    html! {
        <div>
            <div class={classes!("centre")}>
                <Title/>
            </div>
            <div>
                <Social social_tag="Test"/>
                <Social social_tag="Test2"/>
                <Social social_tag="Test3"/>
            </div>
        </div>
    }
}

#[component]
fn Title() -> Html {
    html! {
        <h1 class={classes!("title", "centre")}>{"Fraser's Blog"}<div class={classes!("title-underscore")}>{"_"}</div></h1>
    }
}

#[component]
fn Social(Props { social_tag }: &Props) -> Html {
    html! {
        <div>{"@"}{social_tag}</div>
    }
}
