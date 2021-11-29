use actix_web::{delete, get, put, web, HttpResponse, Responder};
#[path = "service.rs"]
mod service;

#[get("/")]
async fn index() -> impl Responder {
    "Endpoints: /news ".to_string()
}

#[get("/news")]
pub async fn list_news() -> HttpResponse {
    let news = service::list_news().await;
    match news {
        Ok(n) => HttpResponse::Ok().json(n),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[get("/news/{id}")]
pub async fn get_news_by_id(info: web::Path<String>) -> HttpResponse {
    let id = String::from(info.as_str());
    let news = service::get_news_by_id(&id).await;
    match news {
        Ok(n) => HttpResponse::Ok().json(n),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[delete("/news/{id}")]
pub async fn delete_news_by_id(info: web::Path<String>) -> HttpResponse {
    let id = String::from(info.as_str());
    let news = service::delete_news_by_id(&id).await;
    match news {
        Ok(n) => HttpResponse::Ok().json(n),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[delete("/news")]
pub async fn delete_all_news() -> HttpResponse {
    let news = service::delete_all_news().await;
    match news {
        Ok(n) => HttpResponse::Ok().json(n),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[put("/news/{url}/{desc}")]
pub async fn insert_news(info: web::Path<(String, String)>) -> impl Responder {
    let (url, desc) = &info.into_inner();
    let new = service::insert_news(url, desc).await;
    match new {
        Ok(n) => HttpResponse::Ok().json(n),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
