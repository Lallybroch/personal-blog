use yew::prelude::*;

pub struct Post1 {}

impl Post1 {
    pub fn name() -> String {
        "Post One".to_string()
    }
    pub fn desc() -> String {
        ("
            First Post for testing.
        ")
        .to_string()
    }

    pub fn inner() -> Html {
        html! {
            <>
                <h1>{"Post One"}</h1>
                <p>{"
                    Pretend this is useful information.
                    "}
                </p>
            </>

        }
    }
}
