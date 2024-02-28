use crate::error::{AppError, AppResult};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, DeleteResult, Set, UpdateResult};

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: String,
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

pub async fn insert_user(
    conn: &DatabaseConnection,
    username: String,
    password: String,
) -> AppResult<Model> {
    let existing_user = get_user(conn, &username)
        .await
        .map_err(|_| AppError::InternalError)?;
    match existing_user {
        Some(_) => Err(AppError::ExistingSameUsername),
        None => {
            let user = ActiveModel {
                id: Set(uuid::Uuid::new_v4().to_string()),
                username: Set(username),
                password: Set(password),
            };

            user.insert(conn).await.map_err(|_| AppError::InternalError)
        }
    }
}

pub async fn get_user(conn: &DatabaseConnection, username: &str) -> AppResult<Option<Model>> {
    Entity::find()
        .filter(Column::Username.eq(username))
        .one(conn)
        .await
        .map_err(|_| AppError::InternalError)
}

pub async fn delete_user(conn: &DatabaseConnection, username: &str) -> AppResult<DeleteResult> {
    Entity::delete_many()
        .filter(Column::Username.eq(username))
        .exec(conn)
        .await
        .map_err(|_| AppError::InternalError)
}

pub async fn update_user(
    conn: &DatabaseConnection,
    username: &str,
    password: &str,
) -> AppResult<UpdateResult> {
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
