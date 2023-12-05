use crate::diesel;
use diesel::{prelude::*};
use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

use std::vec::Vec;

use crate::database::establish_connection;
use crate::models::item::item::Item;
use crate::schema::todo;

use crate::todo::structs::base::Base;
use crate::todo::ItemTypes;
use crate::todo::{enums::TaskStatus, todo_factory};

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending_array_buffer.push(packed.super_struct),
                ItemTypes::Done(packed) => done_array_buffer.push(packed.super_struct),
            }
        }
        let done_count: i8 = done_array_buffer.len() as i8;
        let pending_count: i8 = pending_array_buffer.len() as i8;

        return ToDoItems {
            pending_items: pending_array_buffer,
            done_items: done_array_buffer,
            pending_item_count: pending_count,
            done_item_count: done_count,
        };
    }

    pub fn get_state() -> ToDoItems {
        let mut connection =  establish_connection();
        // let state: Map<String, Value> = read_file("./state.json");
        let mut array_buffer = Vec::new();
        let items = todo::table.order(todo::columns::id.asc()).load::<Item>(&mut connection).unwrap();

        for item in items {
            let status = TaskStatus::from_string(item.status.as_str().to_string());
            let item = todo_factory(&item.title, status);
            array_buffer.push(item);
        }
        return ToDoItems::new(array_buffer);
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

