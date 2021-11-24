extern crate news_contract;
extern crate news_dao;

use news_contract::News;

pub async fn get_news_by_id(id: &str) -> Option<News> {
    if id.is_empty() {
        None
    } else {
        news_dao::get_news_by_id(id).await
    }
}

pub async fn delete_news_by_id(id: &str) -> Option<bool> {
    if id.is_empty() {
        None
    } else {
        news_dao::delete_news_by_id(id).await
    }
}

pub async fn list_news() -> Option<Vec<News>> {
    news_dao::list_news().await
}

pub async fn insert_news(url: &str, desc: &str) -> Option<News> {
    if url.is_empty() || desc.is_empty() {
        None
    } else {
        news_dao::insert_news(&url, &desc).await
    }
}
