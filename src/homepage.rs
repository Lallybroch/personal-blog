use std::collections::HashMap;
use yew::html::IntoHtmlResult;
use yew::prelude::*;
use yew::Callback;
use yew::Properties;

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

    create_post(
        &mut blog_posts,
        &mut blog_post_short,
        "Post 1".to_string(),
        "Lorum Ipslum test2".to_string(),
        html! {
                                <div>
                                    <h2>{"hello world"}</h2>
                                    <p>
                                        {"

                                        

                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur egestas vitae magna quis consequat. Donec mattis non eros sit amet elementum. Nulla at tincidunt diam. Phasellus bibendum quam sit amet nisi consequat, at malesuada quam maximus. Morbi pulvinar eros dignissim, dignissim elit quis, ultrices quam. Sed id cursus nibh, in mollis sem. Donec tristique elementum nunc, a varius nisl dignissim in. Etiam blandit suscipit mauris eget auctor. Donec vitae bibendum ligula. Donec eget sapien quis ipsum imperdiet elementum. Aliquam erat volutpat. Nullam vestibulum risus ac nunc pulvinar vulputate. Mauris nec turpis ornare, placerat augue mollis, mattis massa.
                            
                            Nullam dignissim augue neque. Pellentesque placerat risus vitae iaculis placerat. Aenean quis ex vel ligula dictum volutpat at at nisl. Curabitur et elit purus. Etiam leo tortor, finibus ac facilisis sit amet, fermentum in magna. Aenean porttitor tellus sem, eget consectetur mi posuere eu. Morbi vitae enim quis ex volutpat interdum. Vestibulum felis felis, facilisis sed mi vitae, fermentum aliquam dolor. Sed quis lacus pulvinar orci fringilla hendrerit. Nam cursus, risus quis aliquet posuere, sem nisi tincidunt nulla, posuere elementum elit eros id augue. Morbi vitae varius arcu.

                            Curabitur ultrices tellus in porta pellentesque. Sed cursus interdum ullamcorper. Aliquam quis lacinia purus, nec consequat dolor. Mauris in felis dolor. Nam lorem magna, condimentum at arcu ac, luctus elementum velit. Ut at sapien a erat consectetur tincidunt blandit vitae enim. Sed et odio auctor, fermentum neque pretium, aliquet dui. Ut ac ex ac sem tristique sollicitudin at vel nisi. Proin tempor arcu vitae porttitor convallis. Suspendisse faucibus posuere magna, vitae elementum massa feugiat eget.

                            Proin in eros iaculis, mollis ante suscipit, consectetur lectus. Maecenas augue arcu, sagittis eget hendrerit sit amet, viverra nec sapien. Quisque venenatis erat orci, at eleifend mi lacinia quis. Aliquam eu lorem ac massa posuere elementum elementum in odio. Praesent dictum, mauris vel maximus volutpat, nisl nisi luctus nulla, id consectetur urna ex consectetur tellus. Integer congue eros ac tincidunt scelerisque. Aliquam commodo dapibus varius. Nullam et turpis nec libero rhoncus fermentum. Cras at felis sed augue ornare fringilla in vitae elit. In vitae ex id leo elementum viverra. Nullam eros dui, finibus at tincidunt sed, placerat ac metus. Proin auctor enim vitae eros consectetur, in luctus dolor egestas. Phasellus sit amet turpis in sem rhoncus semper sed in lectus. Donec dignissim justo at feugiat porta.

                            Suspendisse at feugiat nisi. Quisque id malesuada nisl, at sodales tortor. Maecenas sed mauris consequat tellus consectetur ullamcorper sit amet in sapien. Phasellus sollicitudin auctor magna. Nam et dui arcu. Suspendisse ac metus congue, venenatis ligula eget, malesuada sem. Mauris lobortis sapien sed fermentum accumsan. Vivamus venenatis mauris at velit lacinia pharetra. 


                        "}<br/><br/>
                {"

                                        

                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur egestas vitae magna quis consequat. Donec mattis non eros sit amet elementum. Nulla at tincidunt diam. Phasellus bibendum quam sit amet nisi consequat, at malesuada quam maximus. Morbi pulvinar eros dignissim, dignissim elit quis, ultrices quam. Sed id cursus nibh, in mollis sem. Donec tristique elementum nunc, a varius nisl dignissim in. Etiam blandit suscipit mauris eget auctor. Donec vitae bibendum ligula. Donec eget sapien quis ipsum imperdiet elementum. Aliquam erat volutpat. Nullam vestibulum risus ac nunc pulvinar vulputate. Mauris nec turpis ornare, placerat augue mollis, mattis massa.
                            
                            Nullam dignissim augue neque. Pellentesque placerat risus vitae iaculis placerat. Aenean quis ex vel ligula dictum volutpat at at nisl. Curabitur et elit purus. Etiam leo tortor, finibus ac facilisis sit amet, fermentum in magna. Aenean porttitor tellus sem, eget consectetur mi posuere eu. Morbi vitae enim quis ex volutpat interdum. Vestibulum felis felis, facilisis sed mi vitae, fermentum aliquam dolor. Sed quis lacus pulvinar orci fringilla hendrerit. Nam cursus, risus quis aliquet posuere, sem nisi tincidunt nulla, posuere elementum elit eros id augue. Morbi vitae varius arcu.

                            Curabitur ultrices tellus in porta pellentesque. Sed cursus interdum ullamcorper. Aliquam quis lacinia purus, nec consequat dolor. Mauris in felis dolor. Nam lorem magna, condimentum at arcu ac, luctus elementum velit. Ut at sapien a erat consectetur tincidunt blandit vitae enim. Sed et odio auctor, fermentum neque pretium, aliquet dui. Ut ac ex ac sem tristique sollicitudin at vel nisi. Proin tempor arcu vitae porttitor convallis. Suspendisse faucibus posuere magna, vitae elementum massa feugiat eget.

                            Proin in eros iaculis, mollis ante suscipit, consectetur lectus. Maecenas augue arcu, sagittis eget hendrerit sit amet, viverra nec sapien. Quisque venenatis erat orci, at eleifend mi lacinia quis. Aliquam eu lorem ac massa posuere elementum elementum in odio. Praesent dictum, mauris vel maximus volutpat, nisl nisi luctus nulla, id consectetur urna ex consectetur tellus. Integer congue eros ac tincidunt scelerisque. Aliquam commodo dapibus varius. Nullam et turpis nec libero rhoncus fermentum. Cras at felis sed augue ornare fringilla in vitae elit. In vitae ex id leo elementum viverra. Nullam eros dui, finibus at tincidunt sed, placerat ac metus. Proin auctor enim vitae eros consectetur, in luctus dolor egestas. Phasellus sit amet turpis in sem rhoncus semper sed in lectus. Donec dignissim justo at feugiat porta.

                            Suspendisse at feugiat nisi. Quisque id malesuada nisl, at sodales tortor. Maecenas sed mauris consequat tellus consectetur ullamcorper sit amet in sapien. Phasellus sollicitudin auctor magna. Nam et dui arcu. Suspendisse ac metus congue, venenatis ligula eget, malesuada sem. Mauris lobortis sapien sed fermentum accumsan. Vivamus venenatis mauris at velit lacinia pharetra. 


                        "}<br/><br/>
        {"

                                        

                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur egestas vitae magna quis consequat. Donec mattis non eros sit amet elementum. Nulla at tincidunt diam. Phasellus bibendum quam sit amet nisi consequat, at malesuada quam maximus. Morbi pulvinar eros dignissim, dignissim elit quis, ultrices quam. Sed id cursus nibh, in mollis sem. Donec tristique elementum nunc, a varius nisl dignissim in. Etiam blandit suscipit mauris eget auctor. Donec vitae bibendum ligula. Donec eget sapien quis ipsum imperdiet elementum. Aliquam erat volutpat. Nullam vestibulum risus ac nunc pulvinar vulputate. Mauris nec turpis ornare, placerat augue mollis, mattis massa.
                            
                            Nullam dignissim augue neque. Pellentesque placerat risus vitae iaculis placerat. Aenean quis ex vel ligula dictum volutpat at at nisl. Curabitur et elit purus. Etiam leo tortor, finibus ac facilisis sit amet, fermentum in magna. Aenean porttitor tellus sem, eget consectetur mi posuere eu. Morbi vitae enim quis ex volutpat interdum. Vestibulum felis felis, facilisis sed mi vitae, fermentum aliquam dolor. Sed quis lacus pulvinar orci fringilla hendrerit. Nam cursus, risus quis aliquet posuere, sem nisi tincidunt nulla, posuere elementum elit eros id augue. Morbi vitae varius arcu.

                            Curabitur ultrices tellus in porta pellentesque. Sed cursus interdum ullamcorper. Aliquam quis lacinia purus, nec consequat dolor. Mauris in felis dolor. Nam lorem magna, condimentum at arcu ac, luctus elementum velit. Ut at sapien a erat consectetur tincidunt blandit vitae enim. Sed et odio auctor, fermentum neque pretium, aliquet dui. Ut ac ex ac sem tristique sollicitudin at vel nisi. Proin tempor arcu vitae porttitor convallis. Suspendisse faucibus posuere magna, vitae elementum massa feugiat eget.

                            Proin in eros iaculis, mollis ante suscipit, consectetur lectus. Maecenas augue arcu, sagittis eget hendrerit sit amet, viverra nec sapien. Quisque venenatis erat orci, at eleifend mi lacinia quis. Aliquam eu lorem ac massa posuere elementum elementum in odio. Praesent dictum, mauris vel maximus volutpat, nisl nisi luctus nulla, id consectetur urna ex consectetur tellus. Integer congue eros ac tincidunt scelerisque. Aliquam commodo dapibus varius. Nullam et turpis nec libero rhoncus fermentum. Cras at felis sed augue ornare fringilla in vitae elit. In vitae ex id leo elementum viverra. Nullam eros dui, finibus at tincidunt sed, placerat ac metus. Proin auctor enim vitae eros consectetur, in luctus dolor egestas. Phasellus sit amet turpis in sem rhoncus semper sed in lectus. Donec dignissim justo at feugiat porta.

                            Suspendisse at feugiat nisi. Quisque id malesuada nisl, at sodales tortor. Maecenas sed mauris consequat tellus consectetur ullamcorper sit amet in sapien. Phasellus sollicitudin auctor magna. Nam et dui arcu. Suspendisse ac metus congue, venenatis ligula eget, malesuada sem. Mauris lobortis sapien sed fermentum accumsan. Vivamus venenatis mauris at velit lacinia pharetra. 


                        "}<br/><br/>
                                    </p>

                                </div>
                                },
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
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
                <Post on_click={&onclick} close={&close} title={"Post 1"} post_html={blog_posts.get("Post 1").unwrap()} desc={blog_post_short.get("Post 1").unwrap().to_string()}/>
            </PostGrid>
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

            <div>

                <div class={classes!("post-inner")}>
                    <h2 class={classes!("post-title")}>{title}</h2>
                    <p>
                        {desc}
                    </p>
                </div>
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
