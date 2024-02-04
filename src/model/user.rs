use crate::error::{AppError, AppResult};

use super::get_conn;

use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, DeleteResult, UpdateResult};

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub username: String,
    pub password: String,
}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn insert_user(user: ActiveModel) -> AppResult<Model> {
    let conn = get_conn().await;
    user.insert(conn).await.map_err(|_| AppError::InternalError)
}

pub async fn get_user(username: &str) -> AppResult<Option<Model>> {
    let conn = get_conn().await;
    Entity::find()
        .filter(Column::Username.eq(username))
        .one(conn)
        .await
        .map_err(|_| AppError::InternalError)
}

pub async fn delete_user(username: &str) -> AppResult<DeleteResult> {
    let conn = get_conn().await;
    Entity::delete_many()
        .filter(Column::Username.eq(username))
        .exec(conn)
        .await
        .map_err(|_| AppError::InternalError)
}

pub async fn update_user(username: &str, password: &str) -> AppResult<UpdateResult> {
    let conn = get_conn().await;
    Entity::update_many()
        .filter(Column::Username.eq(username))
        .set(ActiveModel {
            password: ActiveValue::Set(password.into()),
            // 如果字段过多，可以使用 ..Default::default 表示不需要填写的字段
            ..Default::default()
        })
        .exec(conn)
        .await
        .map_err(|_| AppError::InternalError)
}
