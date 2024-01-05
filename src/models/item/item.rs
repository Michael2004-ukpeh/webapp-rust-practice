use super::super::user::user::User;
use crate::schema::todo;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(User)]
#[table_name = "todo"]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub date: NaiveDateTime,
    pub user_id: i32,
}
