use axum::extract::Query;
use axum::{routing::get, Router, Server};
use maud::{html, Markup, DOCTYPE};
use serde::Deserialize;
use tower_http::services::ServeDir;

const TITLE: &'static str = "Sample site";

fn page(content: Markup) -> Markup {
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
            script
            src="https://unpkg.com/htmx.org@1.9.6"
            integrity="sha384-FhXw7b6AlE/jyjlZH5iHa/tTe9EpJ1Y55RjcgPbjeWMskSxZt1v9qkxLJWNJaGni"
            crossorigin="anonymous" {}
        }
    }
}

fn get_gallery(index: usize) -> Markup {
    html! {
        section #gallery  {
            ul {
                li {
                    img class=(if index == 0 {"shown"} else {""}) src="https://picsum.photos/seed/one/1500/1000" alt=(TITLE);
                }
                li {
                    img class=(if index == 1 {"shown"} else {""}) src="https://picsum.photos/seed/two/1500/1000" alt=(TITLE);
                }
                li {
                    img class=(if index == 2 {"shown"} else {""}) src="https://picsum.photos/seed/three/1500/1000" alt=(TITLE);
                }
            }
            .buttons {
                button hx-get="/image" hx-vals=(format!("js:{{index:{}}}", if index > 1 { index - 1} else {0}))   hx-target="#gallery" hx-swap="outerHTML" {"⟨"}
                button hx-get="/image" hx-vals=(format!("js:{{index:{}}}", if index < 2 { index + 1 } else {2} )) hx-target="#gallery" hx-swap="outerHTML" {"⟩"}
            }
        }
    }
}

async fn home() -> Markup {
    page(html! {
    h1 { (TITLE) }
    main {
        article {
            h2 { "Section 1" }
            p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent a fermentum nisi, at ultricies orci. Maecenas maximus tincidunt velit, non lacinia sem porta ut. Integer ullamcorper neque quam, posuere efficitur purus rhoncus eu. Aliquam venenatis dui quis tempus egestas. Nulla malesuada ex velit. Phasellus ultrices aliquam accumsan. Praesent magna sem, dapibus sit amet luctus quis, ultricies in nisi. Aenean eu erat rhoncus, tincidunt mi eu, eleifend erat. Nam finibus congue iaculis. Morbi vel rutrum orci. Fusce mollis lectus non pretium interdum." }
        }
        (get_gallery(0))
    }})
}

#[derive(Deserialize)]
struct ImageQuery {
    index: Option<usize>,
}

async fn gallery(Query(ImageQuery { index }): Query<ImageQuery>) -> Markup {
    get_gallery(index.unwrap_or(0))
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(home))
        .route("/image", get(gallery));
    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
