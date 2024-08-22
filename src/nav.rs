use maud::{html, Markup, PreEscaped};
use sqlx::PgPool;

use crate::components::Page;

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
