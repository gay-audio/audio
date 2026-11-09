use std::fs;

use actix_files::NamedFile;
use actix_multipart::form::{MultipartForm, json::Json as MpJson, tempfile::TempFile, text::Text};
use actix_web::{
    Either, HttpResponse, Scope, delete, get, post, put,
    web::{Data, Json, Path, Query},
};
use anyhow::Context;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    IntoActiveValue, TransactionTrait,
    prelude::*,
};
use serde::Deserialize;

use crate::{
    entity::{content, content_part},
    model::ContentMetadata,
};

pub fn content_routes() -> Scope {
    Scope::new("/content")
        .service(get_all_content)
        .service(get_content_meta)
        .service(get_content)
        .service(create_content)
        .service(create_content_part)
        .service(update_content)
        .service(update_content_part)
        .service(delete_content)
        .service(delete_content_part)
}

#[derive(Deserialize)]
pub struct ContentQuery {
    by_name: Option<String>,
}

#[get("")]
async fn get_all_content(
    db: Data<DatabaseConnection>,
    Query(query): Query<ContentQuery>,
) -> HttpResponse {
    let mut base = content::Entity::load().with(content_part::Entity);

    if let Some(by_name) = query.by_name {
        base = base.filter(content::Column::Name.like(by_name));
    }

    let content = base.all(db.get_ref()).await.unwrap();

    HttpResponse::Ok().json(
        content
            .into_iter()
            .map(Into::into)
            .collect::<Vec<ContentMetadata>>(),
    )
}

#[get("/{id}")]
async fn get_content_meta(db: Data<DatabaseConnection>, content: Path<Uuid>) -> HttpResponse {
    let content = content.into_inner();

    let content = content::Entity::load()
        .with(content_part::Entity)
        .filter(content::Column::Id.eq(content))
        .one(db.get_ref())
        .await
        .unwrap();

    if let Some(model) = content {
        HttpResponse::Ok().json(ContentMetadata::from(model))
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/{id}/{part}")]
async fn get_content(
    db: Data<DatabaseConnection>,
    content: Path<(Uuid, String)>,
) -> Either<HttpResponse, NamedFile> {
    let (content, part) = content.into_inner();

    let content = content_part::Entity::find()
        .left_join(content::Entity)
        .filter(content_part::Column::PartType.eq(part))
        .filter(content::Column::Id.eq(content))
        .one(db.get_ref())
        .await
        .unwrap();

    if let Some(model) = content {
        Either::Right(
            NamedFile::open(format!("content/{}", model.part_id))
                .unwrap()
                .set_content_type(model.mime_type.parse().unwrap()),
        )
    } else {
        Either::Left(HttpResponse::NotFound().finish())
    }
}

#[derive(Deserialize)]
pub struct CreateContent {
    name: String,
}

#[post("")]
async fn create_content(
    db: Data<DatabaseConnection>,
    Json(create_content): Json<CreateContent>,
) -> HttpResponse {
    let model = content::ActiveModel {
        name: Set(create_content.name),
        id: NotSet,
        ..Default::default()
    };
    content::Entity::insert(model)
        .exec(db.get_ref())
        .await
        .unwrap();

    HttpResponse::Created().finish()
}

#[derive(MultipartForm)]
pub struct CreateContentPartForm {
    mime_type: Text<String>,
    file: TempFile,
}

#[post("/{id}/{part}")]
async fn create_content_part(
    db: Data<DatabaseConnection>,
    path: Path<(Uuid, String)>,
    MultipartForm(form): MultipartForm<CreateContentPartForm>,
) -> HttpResponse {
    let (content_id, part) = path.into_inner();
    let CreateContentPartForm {
        mime_type: Text(mime_type),
        file,
    } = form;
    let part_id = Uuid::new_v4();

    db.transaction(|transaction| {
        Box::pin(async move {
            let model = content_part::ActiveModel {
                content_id: Set(content_id),
                part_type: part.into_active_value(),
                mime_type: Set(mime_type),
                part_id: Set(part_id),
                ..Default::default()
            };

            content_part::Entity::insert(model)
                .exec(transaction)
                .await
                .context("Failed to create content part metadata")?;

            fs::copy(file.file.path(), format!("content/{part_id}"))
                .context("Failed to create content part")?;

            Ok::<(), anyhow::Error>(())
        })
    })
    .await
    .unwrap();

    HttpResponse::Created().finish()
}

#[derive(Deserialize)]
pub struct UpdateContent {
    name: Option<String>,
}

#[put("/{id}")]
async fn update_content(
    db: Data<DatabaseConnection>,
    id: Path<Uuid>,
    Json(update): Json<UpdateContent>,
) -> HttpResponse {
    let id = id.into_inner();
    let mut content: content::ActiveModel = content::Entity::find_by_id(id)
        .one(db.get_ref())
        .await
        .unwrap()
        .unwrap()
        .into();

    content.name = update.name.into_active_value();

    content.update(db.get_ref()).await.unwrap();

    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct UpdateContentPart {
    part_type: Option<String>,
    mime_type: Option<String>,
}

#[derive(MultipartForm)]
pub struct UpdateContentPartForm {
    meta: MpJson<UpdateContentPart>,
    file: Option<TempFile>,
}

#[put("/{id}/{part_type}")]
async fn update_content_part(
    db: Data<DatabaseConnection>,
    path: Path<(Uuid, String)>,
    MultipartForm(update): MultipartForm<UpdateContentPartForm>,
) -> HttpResponse {
    let (content_id, part_id) = path.into_inner();

    let meta = update.meta.into_inner();

    db.transaction(|transaction| {
        Box::pin(async move {
            let mut content_part: content_part::ActiveModel = content_part::Entity::find()
                .filter(content_part::Column::ContentId.eq(content_id))
                .one(transaction)
                .await?
                .unwrap()
                .into();

            content_part.part_type = meta.part_type.into_active_value();
            content_part.mime_type = meta.mime_type.into_active_value();

            content_part
                .update(transaction)
                .await
                .context("Failed to update content part metadata")?;

            if let Some(file) = update.file {
                fs::copy(file.file.path(), format!("content/{part_id}"))
                    .context("Failed to update content part")?;
            }
            Ok::<(), anyhow::Error>(())
        })
    })
    .await
    .unwrap();

    HttpResponse::Ok().finish()
}

#[delete("/{content_id}")]
async fn delete_content(db: Data<DatabaseConnection>, id: Path<Uuid>) -> HttpResponse {
    let id = id.into_inner();

    let Some(content) = content::Entity::load()
        .with(content_part::Entity)
        .filter(content::Column::Id.eq(id))
        .one(db.get_ref())
        .await
        .unwrap()
    else {
        return HttpResponse::NotFound().finish();
    };

    for part in content.content_parts.iter().cloned() {
        fs::remove_file(format!("content/{}", part.part_id)).unwrap();
    }

    content.delete(db.get_ref()).await.unwrap();

    HttpResponse::Ok().finish()
}

#[delete("/{content_id}/{part_type}")]
async fn delete_content_part(
    db: Data<DatabaseConnection>,
    path: Path<(Uuid, String)>,
) -> HttpResponse {
    let (content_id, part) = path.into_inner();
    let content_part: content_part::ActiveModel =
        content_part::Entity::find_by_id((content_id, part))
            .one(db.get_ref())
            .await
            .unwrap()
            .unwrap()
            .into();

    content_part.delete(db.get_ref()).await.unwrap();

    HttpResponse::Ok().finish()
}
