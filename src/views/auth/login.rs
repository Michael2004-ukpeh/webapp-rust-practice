use crate::diesel;
use crate::json_serialization::login_response::LoginResponse;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder};

use crate::database::DB;
use crate::models::user::user::User;
use crate::json_serialization::login::Login;
use crate::schema::users::{self};
use crate::jwt::JwToken;
use std::collections::HashMap;
use std::os::macos::raw;

pub async fn login(credentials:web::Json<Login>, mut db: DB) -> HttpResponse{
let password = credentials.password.clone();
let users = users::table.filter(users::columns::username.eq(credentials.username.clone())).load::<User>(&mut db.connection).unwrap();
if users.len() == 0{
    return HttpResponse::NotFound().await.unwrap()
}else if users.len() > 1{
    return HttpResponse::Conflict().await.unwrap()
}
match  users[0].verify(password) {
    true =>{
        let token =   JwToken::new(users[0].id);
        let raw_token = token.encode();
        let response = LoginResponse{
            token:raw_token.clone()
        };
        let body =serde_json::to_string(&response).unwrap();
      
        HttpResponse::Ok().append_header(("token", raw_token)).json(body)
    },
    false =>{
        HttpResponse::Unauthorized().finish()
    }
    
}
}