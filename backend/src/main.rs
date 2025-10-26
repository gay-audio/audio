use actix_web::{
    App, HttpServer,
    web::{Data, ThinData},
};
use backend::{
    config::Config,
    routes::{authors, posts},
};
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load("Config.toml")?;

    let pool = Pool::builder().build(ConnectionManager::<PgConnection>::new(
        config.database_url(),
    ))?;

    let address = config.server.address;

    let config = Data::new(config);

    HttpServer::new(move || {
        App::new()
            .app_data(ThinData(pool.clone()))
            .app_data(config.clone())
            .configure(authors::author_routes)
            .configure(posts::post_routes)
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}
