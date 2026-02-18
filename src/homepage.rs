use std::collections::HashMap;
use yew::html::IntoHtmlResult;
use yew::prelude::*;
use yew::Callback;
use yew::Properties;

mod posts;

use posts::PostOne;

fn create_post(
    post_map: &mut HashMap<String, Html>,
    short_post_map: &mut HashMap<String, String>,
    title: String,
    desc: String,
    full_post: Html,
) {
    short_post_map.insert(title.clone(), desc);
    post_map.insert(title.clone(), full_post);
}

#[derive(Properties, PartialEq)]
pub struct TagProps {
    social_tag: String,
}

#[derive(Properties, PartialEq)]
pub struct PostGridProps {
    children: Html,
}

#[derive(Properties, PartialEq)]
pub struct FullPostProps {
    post_open: bool,
    inner: UseStateHandle<Html>,
}

#[derive(Properties, PartialEq)]
pub struct PostProps {
    on_click: Callback<Html>,
    title: String,
    close: Callback<bool>,
    post_html: Html,
    desc: String,
}

#[component]
pub fn Homepage() -> Html {
    let mut blog_posts = HashMap::<String, Html>::new();
    let mut blog_post_short = HashMap::<String, String>::new();

    //Post 1
    create_post(
        &mut blog_posts,
        &mut blog_post_short,
        PostOne::name(),
        PostOne::desc(),
        PostOne::inner(),
    );

    let post_is_open = use_state(|| false);

    let event_html: UseStateHandle<Html> = use_state(|| html! {});

    let onclick = {
        let post_is_open = post_is_open.clone();
        let event_html = event_html.clone();
        Callback::from(move |inner: Html| {
            post_is_open.set(true);
            event_html.set(inner);
        })
    };

    let close = {
        let post_is_open = post_is_open.clone();
        Callback::from(move |_| {
            post_is_open.set(false);
        })
    };

    html! {
        <div>
                <FullPost   post_open={*post_is_open} inner={event_html.clone()} />


            <div>
                <div class={classes!("centre")}>
                    <Title/>
                </div>
                <div class={classes!("centre")}>
                    <div class={classes!("social")}>
                        <Social social_tag="BlueSky - @lallybrochdev.bsky.social"/>
                    </div>

                    <div class={classes!("social")}>
                        <Social social_tag="Email - dev@frasercourtney.com"/>
                    </div>

                    <div class={classes!("social")}>
                        <Social social_tag="Youtube - @LallybrochDev"/>
                    </div>
                </div>
            </div>
            <PostGrid>
                <Post on_click={&onclick} close={&close} title={"About this Blog"} post_html={blog_posts.get("About this Blog").unwrap()} desc={blog_post_short.get("About this Blog").unwrap().to_string()}/>
            </PostGrid>
        </div>
    }
}

#[component]
fn Title() -> Html {
    html! {
        <h1 class={classes!("title", "centre")}>{"An Idiot's Brain Dump"}<div class={classes!("title-underscore")}>{"_"}</div></h1>
    }
}

#[component]
fn Social(TagProps { social_tag }: &TagProps) -> Html {
    html! {
        <div>{social_tag}</div>
    }
}

#[component]
fn PostGrid(props: &PostGridProps) -> Html {
    html! {
        <div class={classes!("post-grid", "centre")}>
            {props.children.clone()}

        </div>
    }
}

#[component]
fn Post(props: &PostProps) -> Html {
    let click = props.on_click.clone();
    let close = props.close.clone();
    let title = props.title.clone();
    let post = props.post_html.clone();
    let desc = props.desc.clone();
    let inner_html: Html = html! {
        <div class={classes!("full-post")}>
            <button class={classes!("full-post-close")} onclick={move |_| close.emit(false)}>{"X"}</button>
            <div class={classes!("full-post-inner")}>
            {post}
            </div>
        </div>

    };

    html! {
        <button class={classes!("post")} onclick={move |_|  click.emit(inner_html.clone())}>


                <div class={classes!("post-inner")}>
                    <h2 class={classes!("post-title")}>{title}</h2>
                    <p>
                        {desc}
                    </p>
            </div>
        </button>

    }
}

#[component]
fn FullPost(props: &FullPostProps) -> Html {
    let inner_html: Html = <Html as Clone>::clone(&props.inner)
        .into_html_result()
        .expect("error loading blog post");
    let post: Html = html! {


        <div>
            {inner_html}
        </div>

    };

    if props.post_open {
        return post;
    }

    return html! {};
}
