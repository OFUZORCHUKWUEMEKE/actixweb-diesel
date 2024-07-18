use crate::actix::Addr;
use crate::actors::db::DbActor;
use crate::schema::articles;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct AppState {
    pub db: Addr<DbActor>,
}
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
pub struct Article {
    uuid: Uuid,
    title: String,
    body: String,
    published: bool,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "articles"]
pub struct NewArticle {
    pub uuid: Uuid,
    pub title: String,
    pub body: String,
}
