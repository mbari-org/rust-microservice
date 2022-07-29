extern crate news_contract;
extern crate tokio_postgres;
extern crate uuid;

use std::error;

use tokio_postgres::NoTls;
use uuid::Uuid;

use news_contract::News;

pub type DaoResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub async fn connect() -> DaoResult<tokio_postgres::Client> {
    let (client, conn) = match tokio_postgres::connect(
        "host=127.0.0.1 user=postgres password=docker dbname=postgres port=5432",
        NoTls,
    )
    .await
    {
        Ok(x) => x,
        Err(e) => return Err(Box::new(e)),
    };
    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}

pub async fn get_news_by_id(id: &str) -> DaoResult<News> {
    let client = connect().await?;
    let rows = &client
        .query(
            "SELECT id::text,url,'desc' FROM news where id::text=$1",
            &[&id],
        )
        .await?;
    let row = rows.get(0).unwrap();
    let news = News {
        id: row.get(0),
        desc: row.get(2),
        url: row.get(1),
    };
    Ok(news)
}

pub async fn delete_news_by_id(id: &str) -> DaoResult<bool> {
    let client = connect().await?;
    let rows = &client
        .query("DELETE FROM news where id::text=$1", &[&id])
        .await?;
    Ok(!rows.is_empty())
}

pub async fn delete_all_news() -> DaoResult<bool> {
    let client = connect().await?;
    let rows = &client.query("DELETE FROM news", &[]).await?;
    Ok(!rows.is_empty())
}

pub async fn insert_news(url: &str, desc: &str) -> DaoResult<News> {
    let client = connect().await?;
    let row = client
        .query(
            "INSERT INTO news VALUES(uuid_in(md5(random()::text || clock_timestamp()::text)::cstring),$1,$2) RETURNING ID",
            &[&desc, &url],
        ).await?;

    println!("row = {:?}", row);

    let id = if row.is_empty() {
        String::from("0")
    } else {
        let uuid: Uuid = row.get(0).unwrap().get(0);
        uuid.to_string()
    };
    let news = News {
        id,
        desc: String::from(desc),
        url: String::from(url),
    };
    Ok(news)
}

pub async fn list_news() -> DaoResult<Vec<News>> {
    let client = connect().await?;
    let mut vec_news = Vec::new();
    let rows = &client
        .query("SELECT id::text, url, 'desc' FROM news", &[])
        .await?;
    for row in rows {
        let news = News {
            id: row.get(0),
            desc: row.get(1),
            url: row.get(2),
        };
        vec_news.push(news);
    }
    Ok(vec_news)
}

pub async fn mocked_list_news() -> Option<Vec<News>> {
    let mut vec_news = vec![News {
        id: String::from("1234"),
        desc: String::from("google"),
        url: String::from("google.com"),
    }];
    vec_news.push(News {
        id: String::from("1234"),
        desc: String::from("google"),
        url: String::from("google.com"),
    });
    Some(vec_news)
}
