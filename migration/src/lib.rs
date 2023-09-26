pub use sea_orm_migration::prelude::*;

mod m20230925_123910_create_urls_table;
mod m20230925_124006_create_stats_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230925_123910_create_urls_table::Migration),
            Box::new(m20230925_124006_create_stats_table::Migration),
        ]
    }
}
