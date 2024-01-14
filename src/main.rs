use axum::{
    extract::{MatchedPath, Path, Query, State},
    http::{Request, StatusCode},
    response::{Html as AHtml, IntoResponse, Response},
    routing::{get, Router},
};
use maud::{html, Markup, PreEscaped};
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::{info_span, Level, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod copy;
mod markup;
use dotenv::dotenv;
use markup::{build_gallery, page};
use sqlx::postgres::{PgPool, PgPoolOptions};

struct HtmlRes(Markup);

impl IntoResponse for HtmlRes {
    fn into_response(self) -> Response {
        (StatusCode::OK, AHtml(self.0.into_string())).into_response()
    }
}

async fn get_home(State(db): State<PgPool>) -> HtmlRes {
    let nav = build_nav(&db).await;
    HtmlRes(page(
        nav,
        html! {
        main {
            article {
                h2 { "Section 1" }
                p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent a fermentum nisi, at ultricies orci. Maecenas maximus tincidunt velit, non lacinia sem porta ut. Integer ullamcorper neque quam, posuere efficitur purus rhoncus eu. Aliquam venenatis dui quis tempus egestas. Nulla malesuada ex velit. Phasellus ultrices aliquam accumsan. Praesent magna sem, dapibus sit amet luctus quis, ultricies in nisi. Aenean eu erat rhoncus, tincidunt mi eu, eleifend erat. Nam finibus congue iaculis. Morbi vel rutrum orci. Fusce mollis lectus non pretium interdum." }
            }
            (build_gallery(0))
        }},
    ))
}

#[derive(Debug)]
struct Page {
    slug: String,
    title: String,
}

struct PageContent {
    content: String,
}

async fn build_page(db: &PgPool, slug: &str) -> Markup {
    let PageContent { content } = sqlx::query_as!(
        PageContent,
        "select content from pages where slug = $1",
        slug
    )
    .fetch_one(db)
    .await
    .unwrap();
    html! {
        div{(PreEscaped(markdown::to_html(&content)))}
    }
}

async fn get_page(Path(path): Path<String>, State(db): State<PgPool>) -> HtmlRes {
    let page = page(build_nav(&db).await, build_page(&db, &path).await);
    HtmlRes(page)
}

async fn build_nav(db: &PgPool) -> Markup {
    let pages = sqlx::query_as!(Page, "select slug, title from pages")
        .fetch_all(db)
        .await
        .unwrap();
    html! {
        nav {
            ul
            class="flex flex-row gap-4"
            {
                @for Page{slug, title} in pages {
                    li {
                        a href=(format!("/page/{}", slug)) { (title) }
                    }
                }
            }
        }
    }
}

#[derive(Deserialize)]
struct ImageQuery {
    index: Option<usize>,
}

async fn gallery(Query(ImageQuery { index }): Query<ImageQuery>) -> HtmlRes {
    HtmlRes(build_gallery(index.unwrap_or(0)))
}

fn span<T>(request: &Request<T>) -> Span {
    let matched_path = request
        .extensions()
        .get::<MatchedPath>()
        .map(MatchedPath::as_str);
    info_span!(
        "REQ",
        " {:?} {:?} ",
        request.method(),
        matched_path.unwrap_or("ERR"),
    )
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DB env var not set"))
        .await
        .expect("Can't connect to DB");

    let router = Router::new()
        .route("/", get(get_home))
        .route("/page/:id", get(get_page))
        .route("/image", get(gallery))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(span)
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .latency_unit(LatencyUnit::Micros),
                    ),
            ),
        )
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Can't bind to TCP");
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router)
        .await
        .expect("Can't start server");
}
