use crate::json_serialization::todo_items::ToDoItems;
use actix_web::Responder;

pub async fn get() -> impl Responder {
    return ToDoItems::get_state();
}
