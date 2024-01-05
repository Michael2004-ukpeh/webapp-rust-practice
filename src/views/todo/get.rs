use crate::{json_serialization::todo_items::ToDoItems, jwt::JwToken};
use actix_web::Responder;

pub async fn get(token: JwToken) -> impl Responder {
    return ToDoItems::get_state(token.user_id);
}
