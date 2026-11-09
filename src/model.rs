use std::collections::HashMap;

use serde::Serialize;
use uuid::Uuid;

use crate::entity::{content, content_part};

#[derive(Serialize)]
pub struct ContentMetadata {
    pub name: String,
    pub published: bool,
    pub tags: Vec<String>,
    pub parts: HashMap<String, ContentPartMetadata>,
}

impl From<content::ModelEx> for ContentMetadata {
    fn from(
        content::ModelEx {
            name,
            published,
            tags,
            content_parts,
            ..
        }: content::ModelEx,
    ) -> Self {
        Self {
            name,
            published,
            tags: tags.into_iter().map(|x| x.long).collect(),
            parts: content_parts.into_iter().collect(),
        }
    }
}

#[derive(Serialize)]
pub struct ContentPartMetadata {
    pub published: bool,
    pub mime_type: String,
    // pub tags: Vec<String>,
    pub part_id: Uuid,
}

impl FromIterator<content_part::ModelEx> for HashMap<String, ContentPartMetadata> {
    fn from_iter<T: IntoIterator<Item = content_part::ModelEx>>(iter: T) -> Self {
        iter.into_iter()
            .map(|content_part| {
                (
                    content_part.part_type,
                    ContentPartMetadata {
                        published: content_part.published,
                        mime_type: content_part.mime_type,
                        // tags: content_part.writeln
                        part_id: content_part.part_id,
                    },
                )
            })
            .collect()
    }
}
