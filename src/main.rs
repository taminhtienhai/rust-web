use actix_web::{
    get, post,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
    middleware::{Logger},
};
use serde::{
    Serialize,
    Deserialize,
};
use serde_json;
use env_logger::Env;

#[derive(Serialize, Deserialize, Clone)]
struct Info {
    username: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: web::Json<Info>) -> impl Responder {
    let Ok(res) = serde_json::to_string(&req_body) else {
        return "abc".to_string();
    };
    res
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
