use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogMeta {
    pub title: String,
    pub date: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPost {
    pub slug: String,
    pub meta: BlogMeta,
    pub content_html: String,
}
