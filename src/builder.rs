use maud::{html, Markup, PreEscaped};
use sqlx::PgPool;

use crate::{components::Page, PageContent};

fn markdown(md: &str) -> PreEscaped<String> {
    PreEscaped(markdown::to_html(&md))
}

pub async fn page(db: &PgPool, slug: &str) -> Markup {
    let PageContent { content } = sqlx::query_as!(
        PageContent,
        "select content from pages where slug = $1",
        slug
    )
    .fetch_one(db)
    .await
    .unwrap();
    html! {
        div{(markdown(&content))}
    }
}

pub async fn nav(db: &PgPool) -> Markup {
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
