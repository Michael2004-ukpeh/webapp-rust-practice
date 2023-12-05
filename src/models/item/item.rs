
use crate::schema::todo;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable)]
#[table_name="todo"]
pub struct Item{
    pub id:i32,
    pub title:String,
    pub status:String,
    pub date:NaiveDateTime
}