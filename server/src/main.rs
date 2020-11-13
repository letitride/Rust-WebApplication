use actix_web::http::Method;
use actix_web::App;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate diesel;
mod schema;
mod model;
mod db;
mod handlers;

#[derive(Clone)]
pub struct Server {
    pool: Pool<ConnectionManager<PgConnection>>
}

impl Server {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("database url is not set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager).expect("Failed to create pool");
        Server{pool}
    }
}

pub fn app(server: Server) -> App<Server> {
    use crate::handlers::*;

    App::with_state(server)
        .route("/logs", Method::POST, handle_post_logs)
        .route("/csv", Method::POST, handle_post_csv)
        .route("/logs", Method::GET, handle_get_logs)
        .route("/csv", Method::GET, handle_get_csv)
}

fn main() {
    env_logger::init();
    let server = Server::new();
    actix_web::server::new(move || app(server.clone()))
        .bind("localhost:3000")
        .expect("Can not bind to port 3000")
        .run();
}
