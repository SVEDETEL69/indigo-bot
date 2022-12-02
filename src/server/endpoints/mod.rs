pub mod byond;
mod delete_api_token;
mod get_identity;
mod post_connect_byond;
mod post_create_api_token;

pub mod get {
    pub use super::get_identity::get_identity as identity;
}

pub mod post {
    pub use super::post_connect_byond::post_connect_byond as connect_byond;
    pub use super::post_create_api_token::post_create_api_token as create_api_token;
}

pub mod delete {
    pub use super::delete_api_token::delete_api_token;
}
