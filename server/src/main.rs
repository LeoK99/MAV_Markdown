mod models;

use std::fmt::format;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use models::{User, Tag, MdDocument};
use mongodb::{bson::{doc, oid::ObjectId}, options::IndexOptions, Client as mongoClient, Collection, IndexModel};
use std::str::FromStr;


const DB_NAME: &str = "admin";
const COLL_NAME: &str = "system.users";
const TAG_COLL: &str = "tags";
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
async fn add_user(client: web::Data<mongoClient>, form: web::Form<User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user was added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[get("/user/{id}")]
async fn get_user(client: web::Data<mongoClient>, id: web::Path<String>) -> HttpResponse {
    let id: ObjectId = mongodb::bson::oid::ObjectId::from_str(id.into_inner().as_str()).unwrap();
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    match collection
        .find_one(doc! {"_id" : id} , None)
        .await{
            Ok(Some(user)) => HttpResponse::Ok().json(user),
            Ok(None) => {
                HttpResponse::NotFound().body(format!("No User found with id: {id}"))
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
}
#[get("/usertest")]
async fn usertest(client: web::Data<mongoClient>) -> HttpResponse{
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let user = User {
        name: "Horst".to_string(),
        username:"keksMaus35".to_string(),
       email:"pustKuchen44@gmx.de".to_string(),   
    };
    let result = collection.insert_one(user, None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
//A Simple get Document by Id Func
//TODO: ideally we would have a single "/document" URL wich handles all fetches based on wildecars
//i.e GET /document?id={id}
#[get("/document/{id}")]
async fn get_doc_by_id(client: web::Data<mongoClient>, id: web::Path<String>)-> HttpResponse {
    let id: ObjectId = mongodb::bson::oid::ObjectId::from_str(id.as_str()).unwrap();
    let collection: Collection<MdDocument> = client.database(DB_NAME).collection(COLL_NAME);
    match collection
        .find_one(doc! {"_id":id}, None)
        .await{
            Ok(Some(document)) => HttpResponse::Ok().json(document),
            Ok(None) => HttpResponse::NotFound().body(format!("No Document Found with id: {id}")),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
}
async fn create_username_index(client: &mongoClient) {
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
//Tags will be searched through quite often, so we create an Index to Speed up the Proccess
async fn create_tag_index(cleint: &mongoClient){
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! {"tagname":1})
        .options(options)
        .build();
    cleint
        .database(DB_NAME)
        .collection::<Tag>(TAG_COLL)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://root:example@mongo:27017".into());
    let client = mongoClient::with_uri_str(uri).await.expect("faild to connect");
    create_username_index(&client).await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(hello)
            .service(echo)
            .service(usertest)
            .service(get_user)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}