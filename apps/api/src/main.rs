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
mod files;
mod handlers;
mod helpers;
mod middleware;
mod models;
mod routes;
mod schema;
mod server;
mod state;
mod swagger;
mod tests;
mod validate;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    server().await
}
