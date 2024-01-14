use crate::copy::TITLE;
use maud::{html, Markup, DOCTYPE};

pub fn page(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "Sample site" }
                link rel="stylesheet" href="assets/css/main.css";
            }
            body {
                (content)
            }
            script src="https://cdn.tailwindcss.com" {}
            script
            src="https://unpkg.com/htmx.org@1.9.6"
            integrity="sha384-FhXw7b6AlE/jyjlZH5iHa/tTe9EpJ1Y55RjcgPbjeWMskSxZt1v9qkxLJWNJaGni"
            crossorigin="anonymous" {}
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
        button class="text-white text-md backdrop-blur-sm bg-[rgba(0,0,0,0.5)] p-2 rounded-full w-10"
        hx-get="/image" hx-vals=(format!("js:{{index:{}}}", val))   hx-target="#gallery" hx-swap="outerHTML" {(text)}
    }
}

pub fn get_gallery(state: usize) -> Markup {
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
            #buttons
            class="flex justify-between absolute top-[50%] left-0 w-full p-4"
            {
                (button(state, Button::Prev))
                (button(state, Button::Next))
            }
        }
    }
}
