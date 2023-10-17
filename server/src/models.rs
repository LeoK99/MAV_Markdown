use serde::{Deserialize,Serialize};


#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]

pub struct User{
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
}