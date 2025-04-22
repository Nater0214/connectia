use sea_orm_migration::{MigrationTrait, MigratorTrait};

use crate::db::users;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(users::Migration)]
    }
}
