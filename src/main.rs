#![feature(proc_macro_hygiene, decl_macro)]

use chatgpt::{chatgpt_api_call, ChatGPTResponse};
use dotenv::dotenv;
use std::env;

mod chatgpt;

use rocket_dyn_templates::Template;
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::{status, Responder};
use rocket::{get, routes, Rocket};
use serde::Deserialize;

#[get("/")]
fn index() -> Template {
    Template::render("index", ())
}

use rocket::post;

#[derive(Debug, Deserialize)]
struct ChatGPTRequest {
    prompt: String,
}

#[post("/api/chatgpt", format = "application/json", data = "<request>")]
async fn chatgpt_route(request: Json<ChatGPTRequest>) -> Result<String, status::Custom<String>> {
    let api_key = env::var("CHATGPT_API_KEY").map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            "CHATGPT_API_KEY environment variable is missing.".to_string(),
        )
    })?;

    let content = chatgpt_api_call(&request.prompt, &api_key).await.map_err(|err| {
        status::Custom(
            Status::InternalServerError,
            format!("Error calling ChatGPT API: {}", err),
        )
    })?;

    Ok(content)
}

#[rocket::main]
async fn main() {
    dotenv().ok();
    rocket().launch().await.unwrap();
}

use rocket::{Build};

fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, chatgpt_route])
}