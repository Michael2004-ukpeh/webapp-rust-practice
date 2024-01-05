#[macro_use]
extern crate diesel;
extern crate dotenv;

mod config;
mod database;
mod json_serialization;
mod jwt;
mod models;
mod processes;
mod schema;
mod state;
mod todo;
mod views;
mod counter;

use actix_cors::Cors;
use actix_service::Service;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, middleware::Logger};
use futures::future::{ok, Either};
use serde_json::{value::Value, Map};
use state::read_file;
use std::env;

use todo::enums::TaskStatus;
use todo::todo_factory;

use crate::processes::process_input;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let site_counter =counter::Counter{
        count:0
    };
    site_counter.save();

    const ALLOWED_VERSION: &'static str = include_str!("./output_data.txt");
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        let app = App::new()
            .wrap_fn(|req, srv| {
                println!("{}-{}", req.method(), req.uri());
                let passed: bool;
                let mut site_counter = counter::Counter::load().unwrap();
                site_counter.count +=1;
                println!("{:?}", &site_counter);
                site_counter.save();
                if *&req.path().contains(&format!("/{}/", ALLOWED_VERSION)) {
                    passed = true;
                } else {
                    passed = false;
                }

                let end_result = match passed {
                    true => Either::Left(srv.call(req)),
                    false => {
                        let resp =
                            HttpResponse::NotImplemented().body("v1 API is no longer supported");
                        Either::Right(ok(req.into_response(resp).map_into_boxed_body()))
                    }
                };
                async move {
                    let result = end_result.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory)
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"));
        return app
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
