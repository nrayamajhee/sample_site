use axum::extract::State;
use maud::html;

use crate::{components::PageConfig, page::page, AppState, HtmlRes};

pub fn get_photo() {}

pub async fn upload_photo(State(state): State<AppState>) -> HtmlRes {
    let body = html! {
        p {"Hello"}
    };
    let page = page(PageConfig {
        body,
        ..Default::default()
    });
    HtmlRes(page)
}
