mod models;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use models::User;
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

const DB_NAME: &str = "admin";
const COLL_NAME: &str = "system.users";
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[post("/add_user")]
async fn add_user(client: web::Data<Client>, form: web::Form<User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user was added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[get("/usertest")]
async fn usertest(client: web::Data<Client>) -> HttpResponse{
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let user = User {
        name: "Horst".to_string(),
        id: "0".to_string(),
        username:"keksMaus35".to_string(),
       email:"pustKuchen44@gmx.de".to_string(),   
    };
    let result = collection.insert_one(user, None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(usertest)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}