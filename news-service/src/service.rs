extern crate news_contract;
extern crate news_dao;

use news_contract::News;
use news_dao::DaoResult;

pub async fn get_news_by_id(id: &str) -> DaoResult<News> {
    news_dao::get_news_by_id(id).await
}

pub async fn delete_news_by_id(id: &str) -> DaoResult<bool> {
    news_dao::delete_news_by_id(id).await
}

pub async fn delete_all_news() -> DaoResult<bool> {
    news_dao::delete_all_news().await
}

pub async fn list_news() -> DaoResult<Vec<News>> {
    news_dao::list_news().await
}

pub async fn insert_news(url: &str, desc: &str) -> DaoResult<News> {
    news_dao::insert_news(url, desc).await
}
