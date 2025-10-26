use crate::prelude::{PoolManager, Ron};
use actix_web::{
    delete, get, post, put,
    web::{Path, ServiceConfig, ThinData},
};
use audio_data::posts::{CreatePost, DeletePost, Post, UpdatePost};
use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use uuid::Uuid;

pub fn post_routes(service_config: &mut ServiceConfig) {
    service_config
        .service(get_all_posts)
        .service(get_post_by_id)
        .service(create_post)
        .service(update_post)
        .service(delete_post);
}

#[get("/posts")]
async fn get_all_posts(database: ThinData<PoolManager<PgConnection>>) -> Ron<Vec<Post>> {
    use audio_data::schema::posts::dsl::posts;
    let all_posts = posts
        .select(Post::as_select())
        .get_results(&mut database.0.get().unwrap())
        .unwrap();

    Ron(all_posts)
}

#[get("/posts/{post_id}")]
async fn get_post_by_id(
    post_id: Path<Uuid>,
    database: ThinData<PoolManager<PgConnection>>,
) -> Ron<Post> {
    use audio_data::schema::posts::dsl::posts;
    let post = posts
        .find(post_id.into_inner())
        .select(Post::as_select())
        .first(&mut database.0.get().unwrap())
        .unwrap();

    Ron(post)
}

#[post("/posts")]
async fn create_post(
    post: Ron<CreatePost>,
    database: ThinData<PoolManager<PgConnection>>,
) -> Ron<Post> {
    use audio_data::schema::posts::dsl::posts;
    let post = post
        .into_inner()
        .insert_into(posts)
        .get_result(&mut database.0.get().unwrap())
        .unwrap();

    Ron(post)
}

#[put("/posts/{post_id}")]
async fn update_post(
    post_id: Path<Uuid>,
    post: Ron<UpdatePost>,
    database: ThinData<PoolManager<PgConnection>>,
) -> Ron<Post> {
    use audio_data::schema::posts::{dsl::posts, id};

    let post = diesel::update(posts)
        .filter(id.eq(post_id.into_inner()))
        .set(post.into_inner())
        .get_result(&mut database.0.get().unwrap())
        .unwrap();

    Ron(post)
}

#[delete("/posts/{post_id}")]
async fn delete_post(
    post_id: Path<Uuid>,
    database: ThinData<PoolManager<PgConnection>>,
) -> Ron<Post> {
    let post: Post = diesel::delete(&DeletePost {
        id: post_id.into_inner(),
    })
    .get_result(&mut database.0.get().unwrap())
    .unwrap();

    Ron(post)
}
