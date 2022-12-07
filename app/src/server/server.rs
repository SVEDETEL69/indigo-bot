use std::net::ToSocketAddrs;

use actix_web::{App, HttpServer};

use super::endpoints;

pub struct Server;

impl Server {
    pub async fn run(addrs: impl ToSocketAddrs) {
        HttpServer::new(move || {
            App::new()
                // GET
                .service(endpoints::get::identity)
                // POST
                .service(endpoints::post::connect_byond)
                .service(endpoints::post::create_api_token)
                .service(endpoints::post::webhook)
                .service(endpoints::post::create_webhook)
                // DELETE
                .service(endpoints::delete::delete_api_token)
                .service(endpoints::delete::delete_webhook)
                // BYOND-friendly (retarded) API
                .service(endpoints::byond::get::connect_byond)
        })
        .bind(addrs)
        .unwrap()
        .run()
        .await
        .unwrap();
    }
}