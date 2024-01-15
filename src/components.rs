use crate::{copy::TITLE, script};
use maud::{html, Markup, DOCTYPE};

#[derive(Debug)]
pub struct Page {
    pub slug: String,
    pub title: String,
}


pub fn head(htmx: bool, auth: bool, styles: &[&str]) -> Markup {
    html! {
        head {
            title { "Sample site" }
            link rel="stylesheet" href="/assets/css/main.css";
            link rel="stylesheet" href="/assets/css/input.css";
            @for site in styles {
                link rel="stylesheet" href=(site);
            }
            @if htmx {
                script
                src="https://unpkg.com/htmx.org@1.9.6"
                integrity="sha384-FhXw7b6AlE/jyjlZH5iHa/tTe9EpJ1Y55RjcgPbjeWMskSxZt1v9qkxLJWNJaGni"
                crossorigin="anonymous" {}
            }
            @if auth {
                script
                data-clerk-frontend-api=(std::env::var("CLERK_URL").unwrap())
                data-clerk-publishable-key=(std::env::var("CLERK_KEY").unwrap())
                src=(format!("https://{api}/npm/@clerk/clerk-js@latest/dist/clerk.browser.js", api=std::env::var("CLERK_URL").unwrap()))
                {}
            } 
        }
    }
}

pub fn custom_page(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            (head(false, false, &[]))
            h1 #head { (title) }
            .wrapper {
                main {
                    (body)
                }
            }
        }
    }
}

pub fn page(nav: Markup, main: Markup, style: &[&str]) -> Markup {
    html! {
        (DOCTYPE)
        html {
            (head(true, true, style))
            body {
                (login_button())
                a.title href="/" {
                    h1 #head { (TITLE) }
                }
                .wrapper {
                    header {
                        (nav)
                    }
                    main {
                        (main)
                    }
                }
            }
        }
    }
}

pub enum Button {
    Prev,
    Next,
}

pub fn button(index: usize, buton: Button) -> Markup {
    let (text, val) = match buton {
        Button::Prev => ("⟨", if index > 1 { index - 1 } else { 0 }),
        Button::Next => ("⟩", if index < 2 { index + 1 } else { 2 }),
    };
    html! {
        button hx-get="/image" hx-vals=(format!("js:{{index:{}}}", val))   hx-target="#gallery" hx-swap="outerHTML" {(text)}
    }
}

pub fn gallery(state: usize) -> Markup {
    let images = ["one", "two", "three"];
    html! {
        section #gallery  {
            ul {
                @for (i,img) in images.iter().enumerate() {
                    li {
                        img class=(if state == i {"shown"} else {""})
                        src=(format!("https://picsum.photos/seed/${}/1500/1000", img)) alt=(TITLE);
                    }
                }
            }
            .buttons {
                (button(state, Button::Prev))
                (button(state, Button::Next))
            }
        }
    }
}

pub fn login_button() -> Markup {
    html! {
        button #login-button type="button" {}
        (script("
            const clerk = window.Clerk
            const button = document.getElementById('login-button')
            clerk.load().then(() => {
                if (clerk?.user) {
                    button.classList.add('logged')
                }
                button.addEventListener('click', () => {
                    if (clerk?.user) {
                        clerk.signOut()
                        button.classList.remove('logged')
                    } else {
                        clerk.openSignIn()
                    }
                })
            })
        "))
    }
}
