use axum::{
    extract::{MatchedPath, Path, Query, State},
    http::{Request, StatusCode},
    response::{Html as AHtml, IntoResponse, Response},
    routing::{get, Router},
};
use maud::{html, Markup, PreEscaped};
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::{info_span, Level, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod components;
mod copy;
use components::{custom_page, gallery, page};
mod builder;
use clerk_rs::{clerk::Clerk, ClerkConfiguration};
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};

struct HtmlRes(Markup);

impl IntoResponse for HtmlRes {
    fn into_response(self) -> Response {
        (StatusCode::OK, AHtml(self.0.into_string())).into_response()
    }
}
use std::sync::Arc;

async fn home_page(State(state): State<AppState>) -> HtmlRes {
    let nav = builder::nav(&state.db).await;
    HtmlRes(page(
        nav,
        html! {
            article {
                h2 { "Section 1" }
                p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent a fermentum nisi, at ultricies orci. Maecenas maximus tincidunt velit, non lacinia sem porta ut. Integer ullamcorper neque quam, posuere efficitur purus rhoncus eu. Aliquam venenatis dui quis tempus egestas. Nulla malesuada ex velit. Phasellus ultrices aliquam accumsan. Praesent magna sem, dapibus sit amet luctus quis, ultricies in nisi. Aenean eu erat rhoncus, tincidunt mi eu, eleifend erat. Nam finibus congue iaculis. Morbi vel rutrum orci. Fusce mollis lectus non pretium interdum." }
            }
            (gallery(0))
        },
        &[],
    ))
}

struct PageContent {
    content: String,
}

fn script(script: &str) -> Markup {
    html! {
        script {
            (PreEscaped(script))
        }
    }
}

async fn single_page(Path(path): Path<String>, State(state): State<AppState>) -> HtmlRes {
    let page = page(
        builder::nav(&state.db).await,
        builder::page(&state.db, &path).await,
        &[],
    );
    HtmlRes(page)
}

#[derive(Deserialize)]
struct ImageQuery {
    index: Option<usize>,
}

async fn gallery_page(Query(ImageQuery { index }): Query<ImageQuery>) -> HtmlRes {
    HtmlRes(gallery(index.unwrap_or(0)))
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

#[derive(Serialize, Deserialize)]
struct JwkSession {
    sid: String,
}

#[derive(Clone)]
struct AppState {
    db: PgPool,
    auth: Arc<Clerk>,
}

async fn not_found() -> (StatusCode, AHtml<String>) {
    let page = custom_page(
        "404",
        html! {
            .text-center {
                p.huge { "Page not found" }
                a href="/" { "Your way home" }
            }
        },
    );
    (StatusCode::NOT_FOUND, AHtml(page.into_string()))
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
    let config = ClerkConfiguration::new(
        None,
        None,
        Some(std::env::var("CLERK_SECRET").unwrap()),
        None,
    );
    let auth = Arc::new(Clerk::new(config));
    let state = AppState { auth, db: pool };
    let router = Router::new()
        .route("/", get(home_page))
        .route("/page/:id", get(single_page))
        .route("/image", get(gallery_page))
        // .route("/logged_in", get(logged_in))
        .fallback(not_found)
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
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Can't bind to TCP");
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router)
        .await
        .expect("Can't start server");
}
