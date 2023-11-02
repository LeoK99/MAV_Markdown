use actix_web::http::header::HttpDate;
use serde::{Deserialize,Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User{
    pub name: String,
    pub username: String,
    pub email: String,
}

//Define Models for MongoDB
//The ID field will always exist

//I Want to have a tagging system to sort and group documents by
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Tag{
    pub tagname: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Document{
    pub name: String,
    pub tags: Vec<Tag>,
    pub create_date: DateTime<Utc>,
    pub mod_date:  DateTime<Utc>,
    pub content: String,  
}