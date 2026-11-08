use actix_web::Scope;

pub mod content;

pub fn routes() -> Scope {
    Scope::new("").service(content::content_routes())
}
