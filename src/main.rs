mod processes;
mod state;
mod todo;

use serde_json::{value::Value, Map};
use state::read_file;
use std::env;

use todo::enums::TaskStatus;
use todo::todo_factory;

use crate::processes::process_input;
fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];

    let state: Map<String, Value> = read_file("./state.json");
    let status: String;

    match &state.get(*&title) {
        Some(result) => status = result.to_string().replace('\"', ""),
        None => {
            status = "pending".to_owned();
        }
    }
    let item = todo_factory(title, TaskStatus::from_string(status.to_uppercase()));
    process_input(item, command.to_string(), &state);
}
