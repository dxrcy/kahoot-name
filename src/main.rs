#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use std::path::PathBuf;

/// Default name returned if no name given
const DEFAULT_NAME: &str = "https://bruh.news";

/// Get name, formatted, from optional raw name string
fn get_name(name: Option<&str>) -> String {
    let name = name.unwrap_or(DEFAULT_NAME).replace('^', "ㅤ");
    println!("Someone is named '{}'", name);
    format!(r#"{{"name": "{}"}}"#, name)
}

/// No query or path
#[get("/")]
fn index() -> String {
    get_name(None)
}

/// Query
#[get("/?<name>")]
fn name_query(name: String) -> String {
    get_name(Some(&name))
}

/// Path
#[get("/<name..>")]
fn name_path(name: PathBuf) -> String {
    get_name(name.to_str())
}

/// Get cors config
fn get_cors() -> Cors {
    CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_headers: AllowedHeaders::some(&[
            "Access-Control-Allow-Headers",
            "Origin, X-Requested-With, Content-Type, Accept",
        ]),
        ..Default::default()
    }
    .to_cors()
    .expect("Error creating CORS")
}

fn main() {
    // Start rocket server
    rocket::ignite()
        .mount("/", routes![index, name_query, name_path])
        .attach(get_cors())
        .launch();
}
