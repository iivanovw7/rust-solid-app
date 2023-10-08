#[allow(unused_imports)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

use crate::server::server;

mod auth;
mod config;
mod database;
mod errors;
mod handlers;
mod helpers;
mod models;
mod routes;
mod schema;
mod server;
mod state;
mod swagger;
mod tests;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    server().await
}
