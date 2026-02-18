use yew::prelude::*;

pub struct PostOne {}

impl PostOne {
    pub fn name() -> String {
        "About this Blog".to_string()
    }
    pub fn desc() -> String {
        ("
            An Idiot’s Brain dump as the name suggests, is a place for me to write about random things I’ve been thinking about. This site is mostly going to be full of me writing about Programming, Politics, and Philosophy, however I’ll be more than willing to expand into other area’s if I’m interested...
        ")
        .to_string()
    }

    pub fn inner() -> Html {
        html! {
            <>
                <h1>{"About this blog, the process of making a website"}</h1><br/><br/>
                <h2>{"About this blog"}</h2>
                <p>{"
                        An Idiot’s Brain dump as the name suggests is a place for me to write about random things I’ve been thinking about. This site is mostly going to be full of me writing about Programming, Politics, and Philosophy, however I’ll be more than willing to expand into other area’s if I’m interested. My Goal is to publish a post weekly, holding myself to this standard is crucial, as I have a terrible habit of quitting projects early when losing interest. If I am able to write a post every week on a topic I personally am interested then I’ll hopefully avoid burn out and quitting. Before continuing please note that I am an awful writer, I often type faster than I think which results in typos at best and incoherent sentences at worst. Ill do my best to triple check all my posts before publishing but some mistakes are bound to slip through. 
                "} </p>
                <br/>
                <h2>{"Making a website the wrong way"}</h2>
                <p>
                {"
                    Before making this website I wasn’t a stranger to web development, I’ve dabbled in static HTML websites before, and even made a simple website in Svelte before. However my recent fascination has been with the Programming Language Rust has caused me to look into what I can create using this language. Well the first thing that came to mind was a website. I’ve always wanted a place to just write about me interests, without caring if anyone else would read them. Primarily to look back in the future and see how I thought, but also as a mental exercise. 
                "}
                </p>
                <p>
                {"
                    After some quick searching I found a tutorial on a blog called “Shesh’s blog”. This tutorial outlined how to use the Yew framework to create a website in Rust using Web-Assembly (It was an amazing tutorial I recommend checking it out if anyone is interested, link is at the bottom of the page). For anyone reading this who doesn’t know much about Web Development ill give you a quick and simple explination. (Feel free to skip to the next paragraph). Most websites a programmed in a language called “Javascript” this is a simple language however it has its faults. However there is a second language that most modern web browsers support, Web-Assembly. Web-Assembly is a special language because it isn't designed to be written, it is designed to be an intermediate language that other languages compile their code into for running on the web. This allows other non-javascript languages to be used to create websites.

                "}
                </p>
                <p>
                {"
                    After following Shesh’s tutorial and creating a website that displayed two images. I decided I would check out the Yew documentation and just dive straight into my own project. (In hindsight I should’ve follow the whole tutorial). I followed the setup guide and had my own project running in minutes. I would soon realise using Rust for this would make the whole project much harder than it needed to be.
                "}
                </p>
                <br/>
                <h2>{"The Development Process"}</h2>

                <p>
                {"
                    After displaying a title bar and some links to my socials, it was time to make my website look a little nicer. This is where my first issue came. For some reason importing my CSS stylesheet the regular way wasn’t working in my Yew app? After a bit of back and fourth between using different CSS frameworks, I managed to import my CSS stylesheet into my index.html file and decided to just use vanilla CSS. Now that I had CSS working I decided to learn how to make animations in CSS. This was extremely easy, and after a quick google search I had a flickering cursor indicator at the end of my title.
                "}
                </p>
                <p>
                {"
                    Once my website looked good enough I began work on the front-end logic. Without going into too much detail about the technical sides of the Rust programming, I quickly realised that this project is going to be harder than I thought. Unlike Javascript, Rust has strict rules for dealing with data. This made even the simplest thing fairly difficult as I a beginner when it comes to rust. However after a while of looking through documentation, looking up error messages, and reading outdated forum posts I managed to get my simple blog working.
                "}
                </p>
                <p>
                {"
                    Finally I just needed to host my website, easy right? Yeah.. No it wasn’t so easy. At first I thought I could just use the Yew build command and host it on Github Pages. Unfortunately no matter how hard I tried the code would never run on Github pages, every possible method I tried resulted in a blank website. After I kid you not, 4 hours, I found another Yew rust tutorial that also had a section on deploying your app. (Please check it out at the bottom of the page, I can’t express how much this tutorial helped me). This tutorial suggested using Netlify for hosting, so I followed their suggestion and tried. After another hour of trouble shooting, my website was up and running on Netlify servers. 
                "}
                </p>
                <p>
                {"
                    At this point it was about 1am and I called It a night. The final thing I needed to do was set up my custom domain. This was shockingly easy and in no time the address “frasercourtney.com” was pointing to my Netlify site. Unfortunately it didn’t work for me because I didn’t realise firefox caches you DNS register and frasercourtney.com was still pointing to its old website… it took me an embarrassingly long amount of time to realise this.
                "}
                </p>

                <br/>
                <h2>{"A Working website!"}</h2>
                <p>
                {"
                    After probably 10+ hours of work I had a crude yet working website built in what a lot of people consider the hardest programming language. Was it worth it? 100% the joy I felt seeing the site load for the first time after trouble shooting for 4 hours is impossible to explain. I defnatly could of chosen the tools I used to make this website better, something like React for Javascript would’ve been infinity easier to debug but I think using the more obscure and difficulte tools made the project much more enjoyable. This project perfectly encapsulates what I love about programming, the hours of problem solving that finally pays off with a working project, and the fact that I used more obscure and complex tools made this feeling so much sweeter (and the fact that I did not use any AI).
                "}
                </p>
                <p>
                {"
                    Thanks for reading this short post! Checkout these below for more information about making Websites with Rust and Yew.
                "}
                </p>

                <br/>
                <h2>{"Links!"}</h2>
                <ul>
                  <li><a href="https://yew.rs/" target="_blank">{"Yew Framework Website"}</a></li>
                  <li><a href="https://www.sheshbabu.com/posts/rust-wasm-yew-single-page-application/" target="_blank">{"Shesh's Blog"}</a></li>
                  <li><a href="https://www.djamware.com/post/build-a-web-app-in-rust-with-webassembly-and-yew" target="_blank">{"Djamware Deploying tutorial"}</a></li>
                </ul>
                <br/><br/>

            </>

        }
    }
}
