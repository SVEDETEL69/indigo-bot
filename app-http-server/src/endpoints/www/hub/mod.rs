use actix_web::Scope;

mod account;
mod auth;
mod index;

pub fn scope() -> Scope {
    Scope::new("/hub")
        .service(index::endpoint)
        .service(auth::endpoint)
        .service(account::endpoint)
}
