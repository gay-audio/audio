use actix_web::{
    App, HttpServer, get,
    web::{Data, Path, ThinData},
};
use audio_data::prelude::*;
use backend::{config::Config, prelude::PoolManager, types::Serde};
use diesel::{
    PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
    r2d2::{ConnectionManager, Pool},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[get("/scripts/{content_id}")]
async fn get_script(
    content_id: Path<Uuid>,
    database: ThinData<PoolManager<PgConnection>>,
) -> Serde<Content<Script>> {
    use audio_data::schema::content::dsl::*;

    let result = content
        .find(content_id.into_inner())
        .select(Content::<Script>::as_select())
        .first(&mut database.get().unwrap())
        .expect("Error loading posts");

    Serde(result)
}

#[get("/audios/{id}/name")]
async fn get_audio(
    content_id: Path<Uuid>,
    database: ThinData<PoolManager<PgConnection>>,
) -> Serde<Content<Audio>> {
    use audio_data::schema::content::dsl::*;
    let result = content
        .find(content_id.into_inner())
        .select(Content::<Audio>::as_select())
        .first(&mut database.get().unwrap())
        .expect("Error loading posts");

    Serde(result)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Hello {
    hi: String,
}

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
            .service(get_script)
            .service(get_audio)
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}
