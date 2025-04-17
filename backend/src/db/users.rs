use async_trait::async_trait;
use sea_orm::{
    ActiveModelBehavior, DbErr, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EnumIter,
    PrimaryKeyTrait,
    sea_query::{ColumnDef, Table},
};
use sea_orm_migration::{MigrationName, MigrationTrait, SchemaManager};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users", rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "users"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Column::Username).string().not_null())
                    .col(ColumnDef::new(Column::PasswordHash).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await
    }
}
