use actix_session::{
    config::CookieContentSecurity::Private, storage::CookieSessionStore, Session, SessionMiddleware,
};
use actix_web::{cookie::Key, web, App, HttpResponse, HttpServer, Responder};

async fn index(session: Session) -> impl Responder {
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter").unwrap() {
        counter = count + 1;
    }
    session.insert("counter", counter).unwrap();

    if let Some(_) = session.get::<String>("user_id").unwrap() {
        println!("User is logged in");
    } else {
        println!("User is not logged in");
        session.insert("user_id", "123456").unwrap();
    }

    HttpResponse::Ok().body(format!("Counter: {}", counter))
}

fn session_middleware(secret_key: Key) -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), secret_key)
        .cookie_content_security(Private)
        .cookie_secure(true)
        .build()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(session_middleware(secret_key.clone()))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
