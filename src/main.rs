use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

struct AppState {
    url_map: Mutex<HashMap<String, String>>,
}

async fn shorten_url(data: web::Data<AppState>, original_url: web::Json<String>) -> impl Responder {
    let short_id = Uuid::new_v4().to_string().chars().take(8).collect::<String>();
    let mut url_map = data.url_map.lock().unwrap();
    url_map.insert(short_id.clone(), original_url.into_inner());
    HttpResponse::Ok().body(format!("http://localhost:8080/{}", short_id))
}

async fn redirect(data: web::Data<AppState>, short_id: web::Path<String>) -> impl Responder {
    let short_id = short_id.into_inner();
    let url_map = data.url_map.lock().unwrap();
    if let Some(original_url) = url_map.get(&short_id) {
        HttpResponse::Found().append_header(("Location", original_url.clone())).finish()
    } else {
        HttpResponse::NotFound().body("URL not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        url_map: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/shorten", web::post().to(shorten_url))
            .route("/{short_id}", web::get().to(redirect))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}