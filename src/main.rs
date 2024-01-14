use axum::extract::{MatchedPath, Query};
use axum::http::{Request, StatusCode};
use axum::response::{Html as AHtml, IntoResponse, Response};
use axum::{routing::get, Extension, Router};
use maud::{html, Markup, DOCTYPE};
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
// use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tracing::{info_span, Level};
// use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const TITLE: &'static str = "Sample site";

struct HtmlRes(Markup);

impl IntoResponse for HtmlRes {
    fn into_response(self) -> Response {
        (StatusCode::OK, AHtml(self.0.into_string())).into_response()
    }
}

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

async fn home() -> HtmlRes {
    HtmlRes(page(html! {
    h1 { (TITLE) }
    main {
        article {
            h2 { "Section 1" }
            p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent a fermentum nisi, at ultricies orci. Maecenas maximus tincidunt velit, non lacinia sem porta ut. Integer ullamcorper neque quam, posuere efficitur purus rhoncus eu. Aliquam venenatis dui quis tempus egestas. Nulla malesuada ex velit. Phasellus ultrices aliquam accumsan. Praesent magna sem, dapibus sit amet luctus quis, ultricies in nisi. Aenean eu erat rhoncus, tincidunt mi eu, eleifend erat. Nam finibus congue iaculis. Morbi vel rutrum orci. Fusce mollis lectus non pretium interdum." }
        }
        (get_gallery(0))
    }}))
}

#[derive(Deserialize)]
struct ImageQuery {
    index: Option<usize>,
}

async fn gallery(Query(ImageQuery { index }): Query<ImageQuery>) -> HtmlRes {
    HtmlRes(get_gallery(index.unwrap_or(0)))
}

#[derive(Clone)]
struct State {}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router = Router::new()
        .route("/", get(home))
        .route("/image", get(gallery))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &Request<_>| {
                            // Log the matched route's path (with placeholders not filled in).
                            // Use request.uri() or OriginalUri if you want the real path.
                            let matched_path = request
                                .extensions()
                                .get::<MatchedPath>()
                                .map(MatchedPath::as_str);

                            info_span!(
                                "http_request",
                                method = ?request.method(),
                                matched_path,
                                some_other_field = tracing::field::Empty,
                            )
                        })
                        .on_request(DefaultOnRequest::new().level(Level::INFO))
                        .on_response(
                            DefaultOnResponse::new()
                                .level(Level::INFO)
                                .latency_unit(LatencyUnit::Micros),
                        ),
                )
                .layer(Extension(State {})),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
