use actix_web::{web, HttpResponse};

use crate::database::{establish_connection, DB};
use crate::diesel;
use crate::models::item::item::Item;
use crate::schema::todo;
use diesel::prelude::*;

use crate::json_serialization::{todo_item::ToDoItem, todo_items::ToDoItems};

use crate::jwt::JwToken;

pub async fn delete(todo_item: web::Json<ToDoItem>, token: JwToken, mut db: DB) -> HttpResponse {
    let items = todo::table
        .filter(todo::columns::title.eq(&todo_item.title.as_str()))
        .filter(todo::columns::user_id.eq(&token.user_id))
        .order(todo::columns::id.asc())
        .load::<Item>(&mut db.connection)
        .unwrap();
    let _ = diesel::delete(&items[0]).execute(&mut db.connection);
    return HttpResponse::Ok().json(ToDoItems::get_state(token.user_id));
}
