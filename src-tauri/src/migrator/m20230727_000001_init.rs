use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        // Users
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .unique_key()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Pass).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Buckets
        manager
            .create_table(
                Table::create()
                    .table(Bucket::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Bucket::Id)
                            .integer()
                            .not_null()
                            .unique_key()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Bucket::Name).string().not_null())
                    .col(ColumnDef::new(Bucket::Color).string().not_null())
                    .col(ColumnDef::new(Bucket::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_bucket_user_id")
                            .from(Bucket::Table, Bucket::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Institutions
        manager
            .create_table(
                Table::create()
                    .table(Institution::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Institution::Id)
                            .integer()
                            .not_null()
                            .unique_key()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Institution::Name).string().not_null())
                    .col(
                        ColumnDef::new(Institution::IsSso)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Institution::Alias).string())
                    .col(ColumnDef::new(Institution::Website).string())
                    .col(ColumnDef::new(Institution::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_institution_user_id")
                            .from(Institution::Table, Institution::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Account
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Account::Id)
                            .integer()
                            .not_null()
                            .unique_key()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Account::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Account::AccountName).string().not_null())
                    .col(
                        ColumnDef::new(Account::Mode)
                            .string()
                            .not_null()
                            .default("secure"),
                    )
                    .col(ColumnDef::new(Account::Pin).integer())
                    .col(ColumnDef::new(Account::MinLength).integer())
                    .col(ColumnDef::new(Account::MaxLength).integer())
                    .col(ColumnDef::new(Account::Industry).string().not_null())
                    .col(
                        ColumnDef::new(Account::TwoFactorAuth)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Account::Recovery).string())
                    .col(ColumnDef::new(Account::UserId).integer().not_null())
                    .col(ColumnDef::new(Account::BucketId).integer().not_null())
                    .col(ColumnDef::new(Account::InstitutionId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_account_user_id")
                            .from(Account::Table, Account::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_account_bucket_id")
                            .from(Account::Table, Account::BucketId)
                            .to(Bucket::Table, Bucket::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_account_institution_id")
                            .from(Account::Table, Account::InstitutionId)
                            .to(Institution::Table, Institution::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Bucket::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Institution::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Account {
    Table,
    Id,
    CreatedAt,
    AccountName,
    Mode,
    Pin,
    MinLength,
    MaxLength,
    Industry,
    TwoFactorAuth,
    Recovery,
    UserId,
    BucketId,
    InstitutionId,
    SsoId,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Username,
    Pass,
}

#[derive(Iden)]
enum Bucket {
    Table,
    Id,
    Name,
    Color,
    UserId,
}

#[derive(Iden)]
enum Institution {
    Table,
    Id,
    Name,
    IsSso,
    Alias,
    Website,
    UserId,
}
