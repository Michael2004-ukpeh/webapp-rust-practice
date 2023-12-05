use actix_web::{HttpRequest, HttpResponse};
use crate::diesel;
use diesel::prelude::*;
use crate::json_serialization::todo_items::ToDoItems;
use crate::database::establish_connection;
use crate::models::item::new_item::NewItem;
use crate::models::item::item::Item;
use crate::schema::todo;


pub async fn create(req: HttpRequest) -> HttpResponse{
    // let state: Map<String, Value> = read_file("./state.json");

    let title: String = req.match_info().get("title").unwrap().to_string();
    let mut  connection =  establish_connection();
    let items =  todo::table.filter(todo::columns::title.eq(&title.as_str())).order(todo::columns::id.asc()).load::<Item>(&mut connection).unwrap();

   if items.len() == 0{
    let new_post  =  NewItem::new(title);
    let _ =  diesel::insert_into(todo::table).values(&new_post).execute(&mut connection);
   }
    return HttpResponse::Ok().json(ToDoItems::get_state())
}
