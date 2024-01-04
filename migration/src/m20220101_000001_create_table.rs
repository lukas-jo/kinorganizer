use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Event::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Event::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Event::Film).big_unsigned().not_null())
                    .col(ColumnDef::new(Event::Text).string().not_null())
                    .to_owned(),
            )
            .await?;

        let insert = Query::insert()
            .into_table(Event::Table)
            .columns([Event::Film, Event::Text])
            .values_panic([181886.into(), "Enemy".into()])
            .values_panic([115.into(), "Big Lebowski".into()])
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Event {
    Table,
    Id,
    Film,
    Text,
}
