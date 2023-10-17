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
async fn create_username_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "localhost:27017".into());
    let client = Client::with_uri_str(uri).await.expect("faild to connect");
    create_username_index(&client).await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(hello)
            .service(echo)
            .service(usertest)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}