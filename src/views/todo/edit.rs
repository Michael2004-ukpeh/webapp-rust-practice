use actix_web::{web,HttpResponse};
use serde_json::value::Value;
use serde_json::Map;
use crate::jwt::JwToken;
use crate::state::read_file;
use crate::todo::{todo_factory, enums::TaskStatus};

use crate::json_serialization::{todo_item::ToDoItem, todo_items::ToDoItems};
use crate::processes::process_input;

pub async fn edit(todo_item:web::Json<ToDoItem>, token:JwToken) -> HttpResponse{
    let state:Map<String, Value> =  read_file("./state.json");
    let status:TaskStatus;
    match &state.get(&todo_item.title){
        Some(result) =>{
            status = TaskStatus::from_string(result.as_str().unwrap().to_string());

        },
        None =>{
            return HttpResponse::NotFound().json(
                format!("{} not in state", &todo_item.title)
            )
        }

    }

    let existing_item =  todo_factory(todo_item.title.as_str(), status.clone());
    if &status.stringify() == &TaskStatus::from_string(todo_item.status.as_str().to_string()).stringify(){
            return HttpResponse::Ok().json(ToDoItems::get_state())
    }
    process_input(existing_item, "edit".to_owned(), &state);
    return HttpResponse::Ok().json(ToDoItems::get_state())
    
}