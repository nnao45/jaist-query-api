use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use actix_web::http::StatusCode;
use crate::infra::mysql::{DbPool, new_pool};

#[get("/exec")]
async fn exec(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user_count = web::block(move || crate::infra::repo::get_user_count(&conn))
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }).expect("fxxk");
    web::Json(user_count)
}

#[get("/hc")]
async fn hc() -> impl Responder {
    format!("OK")
}

#[actix_web::main]
pub async fn server_run() -> std::io::Result<()> {
    HttpServer::new(|| App::new().data(new_pool().clone())
        .service(exec)
        .service(hc))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}