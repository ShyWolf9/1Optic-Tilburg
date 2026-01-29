use actix_web::{get, post, put, web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Serialize, sqlx::FromRow)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
}

// For creating or updating a user
#[derive(Deserialize)]
struct UserInput {
    first_name: String,
    last_name: String,
}

#[get("/users")]
async fn users(db: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, User>(
        "SELECT id, first_name, last_name FROM users ORDER BY id"
    )
    .fetch_all(db.get_ref())
    .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("DB error: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/users")]
async fn add_user(db: web::Data<PgPool>, user: web::Json<UserInput>) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO users (first_name, last_name) VALUES ($1, $2) RETURNING id",
        user.first_name,
        user.last_name
    )
    .fetch_one(db.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Ok().json(record.id),
        Err(e) => {
            eprintln!("DB error: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[put("/users/{id}")]
async fn update_user(
    db: web::Data<PgPool>, 
    path: web::Path<i32>, 
    user: web::Json<UserInput>
) -> impl Responder {
    let id = path.into_inner();
    let result = sqlx::query!(
        "UPDATE users SET first_name = $1, last_name = $2 WHERE id = $3",
        user.first_name,
        user.last_name,
        id
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                HttpResponse::NotFound().body(format!("No user found with id {}", id))
            } else {
                HttpResponse::Ok().body(format!("User {} updated", id))
            }
        }
        Err(e) => {
            eprintln!("DB error: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .connect("postgres://admin:Baarle-Nassau@localhost:9112/users_db")
        .await
        .expect("Failed to connect to database");

    println!("Starting server at http://127.0.0.1:8080 ...");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(users)
            .service(add_user)
            .service(update_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

