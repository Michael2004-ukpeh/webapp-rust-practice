#[macro_use] extern crate diesel;
extern crate dotenv;

mod json_serialization;
mod processes;
mod state;
mod todo;
mod views;
mod jwt;
mod schema;
mod database;
mod models;
mod config;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_service::Service;
use actix_cors::Cors;
use serde_json::{value::Value, Map};
use state::read_file;
use std::env;

use todo::enums::TaskStatus;
use todo::todo_factory;

use crate::processes::process_input;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin()
        .allow_any_method()
        .allow_any_header();
        let app = App::new()
        .wrap_fn(|req, srv|{
            println!("{}-{}", req.method(), req.uri());
            let future =  srv.call(req);
            async {
                let result =  future.await?;
                Ok(result)
            }
        })
        .configure(views::views_factory).wrap(cors);
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await

    // let args: Vec<String> = env::args().collect();
    // let command: &String = &args[1];
    // let title: &String = &args[2];

    // let state: Map<String, Value> = read_file("./state.json");
    // let status: String;

    // match &state.get(*&title) {
    //     Some(result) => status = result.to_string().replace('\"', ""),
    //     None => {
    //         status = "pending".to_owned();
    //     }
    // }
    // let item = todo_factory(title, TaskStatus::from_string(status.to_uppercase()));
    // process_input(item, command.to_string(), &state);
}
