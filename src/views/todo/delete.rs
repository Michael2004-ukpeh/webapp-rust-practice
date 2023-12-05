use actix_web::{web, HttpResponse};

use crate::diesel;
use crate::models::item::item::Item;
use diesel::{prelude::*};
use crate::database::establish_connection;
use crate::schema::todo;

use crate::json_serialization::{todo_item::ToDoItem, todo_items::ToDoItems};

use crate::jwt::JwToken;


pub async fn delete(todo_item:web::Json<ToDoItem>, token:JwToken) -> HttpResponse{
let connection = establish_connection();
let items = todo::table.filter(todo::columns::title.eq(&todo_item.title.as_str())).order(todo::columns::id.asc()).load::<Item>(&connection).unwrap();
 let _ =  diesel::delete(&items[0]).execute(&connection);   
    return HttpResponse::Ok().json(ToDoItems::get_state());
}