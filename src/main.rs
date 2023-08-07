mod state;
mod todo;

use serde_json::{json, value::Value, Map};
use state::{read_file, write_to_file};
use std::env;

use todo::enums::TaskStatus;
use todo::todo_factory;

use todo::traits::{delete::Delete, edit::Edit, get::Get};
use todo::ItemTypes;
fn main() {
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];

    let mut state: Map<String, Value> = read_file("./state.json");
    println!("Before operation: {:?}", state);
    state.insert(title.to_string(), json!(status));
    println!("After operation: {:?}", state);
    write_to_file("./state.json", &mut state);
}
