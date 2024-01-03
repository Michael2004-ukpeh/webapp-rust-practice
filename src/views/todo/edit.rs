use crate::diesel;
use diesel::prelude::*;

use actix_web::{web,HttpResponse};
use crate::json_serialization::{todo_item::ToDoItem,todo_items::ToDoItems};
use crate::database::establish_connection;
use crate::schema::todo;
use crate::jwt::JwToken;
use crate::database::DB;

pub async fn edit(todo_item:web::Json<ToDoItem>, token:JwToken, mut db:DB) -> HttpResponse{
    // let mut connection =  establish_connection();
    let results =  todo::table.filter(todo::columns::title.eq(&todo_item.title)).filter(todo::columns::user_id.eq(&token.user_id));

    let _ = diesel::update(results).set(todo::columns::status.eq("DONE")).execute(&mut db.connection);

   
    return HttpResponse::Ok().json(ToDoItems::get_state(token.user_id))
    
}