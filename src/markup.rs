use crate::copy::TITLE;
use maud::{html, Markup, DOCTYPE};

pub fn page(nav: Markup, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "Sample site" }
                link rel="stylesheet" href="/assets/css/main.css";
            }
            body {
                a.title href="/" { 
                    h1 { (TITLE) }
                }
                .wrapper {
                    header {
                        (nav)
                    }
                    main {
                        (content)
                    }
                }
                script
                src="https://unpkg.com/htmx.org@1.9.6"
                integrity="sha384-FhXw7b6AlE/jyjlZH5iHa/tTe9EpJ1Y55RjcgPbjeWMskSxZt1v9qkxLJWNJaGni"
                crossorigin="anonymous" {}
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

pub fn build_gallery(state: usize) -> Markup {
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
