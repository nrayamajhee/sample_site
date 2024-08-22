use crate::components::{head, login_button};
use crate::copy::TITLE;
use crate::nav::nav;
use crate::script;
use crate::{components::PageConfig, AppState, HtmlRes, PageContent};
use axum::extract::{Path, State, Form};
use axum::response::Redirect;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use serde::Deserialize;
use sqlx::postgres::PgPool;

fn markdown(md: &str) -> PreEscaped<String> {
    PreEscaped(markdown::to_html(&md))
}

pub async fn content(db: &PgPool, slug: &str) -> Markup {
    let PageContent { content } = sqlx::query_as!(
        PageContent,
        "select content from pages where slug = $1",
        slug
    )
    .fetch_one(db)
    .await
    .unwrap();
    html! {
        div{
            a href=(format!("/{}/edit", slug)) { "Edit" }
            (markdown(&content))
        }
    }
}

pub fn page(config: PageConfig) -> Markup {
    html! {
        (DOCTYPE)
        html {
            (head(&config))
                body {
                (login_button())
                a.title href="/" {
                    h1 #head { (TITLE) }
                }
                .wrapper {
                    header {
                        (config.nav)
                    }
                    main {
                        (config.body)
                    }
                }
                }
        }
    }
}

pub async fn edit_page(db: &PgPool, slug: &str) -> Markup {
    let PageContent { content } = sqlx::query_as!(
        PageContent,
        "select content from pages where slug = $1",
        slug
    )
    .fetch_one(db)
    .await
    .unwrap();
    html! {
        h1 { (format!("Editing {}", slug)) }
        form action=(format!("/{}/edit",slug)) method="post" {
            textarea id="page-data" name="content" {}
            button { "Update" }
        }
        (script(&format!("
             document.addEventListener('DOMContentLoaded', () => {{
                 document.getElementById('page-data').value = '{}'
            }})
        ",
         content)))
    }
}

#[derive(Deserialize)]
pub struct PageUpdateForm {
    content: String,
}

pub async fn edit(Path(path): Path<String>, State(state): State<AppState>) -> HtmlRes {
    let page = page(PageConfig {
        nav: nav(&state.db).await,
        use_htmx: true,
        use_clerk: true,
        body: edit_page(&state.db, &path).await,
        ..Default::default()
    });
    HtmlRes(page)
}

pub async fn update(
    State(state): State<AppState>,
    Path(path): Path<String>,
    Form(form): Form<PageUpdateForm>,
) -> Redirect {
    sqlx::query!(
        "update pages set content = $1 where slug = $2",
        form.content,
        path
    )
    .execute(&state.db)
    .await
    .unwrap();
    Redirect::to(&format!("/{}", path))
}

pub async fn single(Path(path): Path<String>, State(state): State<AppState>) -> HtmlRes {
    let page = page(PageConfig {
        nav: nav(&state.db).await,
        use_htmx: true,
        use_clerk: true,
        body: content(&state.db, &path).await,
        ..Default::default()
    });
    HtmlRes(page)
}
