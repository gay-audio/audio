use crate::prelude::{PoolManager, Ron};
use actix_web::{
    delete, get, post, put,
    web::{Path, ServiceConfig, ThinData},
};
use audio_data::authors::{Author, CreateAuthor, DeleteAuthor, UpdateAuthor};
use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use uuid::Uuid;

pub fn author_routes(service_config: &mut ServiceConfig) {
    service_config
        .service(get_all_authors)
        .service(get_author_by_id)
        .service(create_author)
        .service(update_author)
        .service(delete_author);
}

#[get("/authors")]
async fn get_all_authors(database: ThinData<PoolManager<PgConnection>>) -> Ron<Vec<Author>> {
    use audio_data::schema::authors::dsl::authors;
    let all_posts = authors
        .select(Author::as_select())
        .get_results(&mut database.0.get().unwrap())
        .unwrap();

    Ron(all_posts)
}

#[get("/authors/{id}")]
async fn get_author_by_id(
    id: Path<Uuid>,
    database: ThinData<PoolManager<PgConnection>>,
) -> Ron<Author> {
    use audio_data::schema::authors::dsl::authors;
    let author = authors
        .find(id.into_inner())
        .select(Author::as_select())
        .first(&mut database.0.get().unwrap())
        .unwrap();

    Ron(author)
}

#[post("/authors")]
async fn create_author(
    author: Ron<CreateAuthor>,
    database: ThinData<PoolManager<PgConnection>>,
) -> Ron<Author> {
    use audio_data::schema::authors::dsl::authors;

    let author = author
        .into_inner()
        .insert_into(authors)
        .get_result(&mut database.0.get().unwrap())
        .unwrap();

    Ron(author)
}

#[put("/authors/{post_id}")]
async fn update_author(
    author_id: Path<Uuid>,
    author: Ron<UpdateAuthor>,
    database: ThinData<PoolManager<PgConnection>>,
) -> Ron<Author> {
    use audio_data::schema::authors::{dsl::authors, id};

    let post = diesel::update(authors)
        .filter(id.eq(author_id.into_inner()))
        .set(author.into_inner())
        .get_result(&mut database.0.get().unwrap())
        .unwrap();

    Ron(post)
}

#[delete("/authors/{id}")]
async fn delete_author(
    id: Path<Uuid>,
    database: ThinData<PoolManager<PgConnection>>,
) -> Ron<Author> {
    let post = diesel::delete(&DeleteAuthor {
        id: id.into_inner(),
    })
    .get_result(&mut database.0.get().unwrap())
    .unwrap();

    Ron(post)
}
