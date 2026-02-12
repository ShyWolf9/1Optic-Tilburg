use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use sqlx::{PgPool, FromRow};
use std::env;
use actix_cors::Cors;

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
}

#[get("/users")]
async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT id, first_name, last_name FROM users ORDER BY id")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Database query failed: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch users")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://admin:Baarle-Nassau@localhost:5432/user_db".to_string());

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Backend running on http://127.0.0.1:9112");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(pool.clone()))
            .service(get_users)
    })
    .bind(("127.0.0.1", 9112))?
    .run()
    .await
}
