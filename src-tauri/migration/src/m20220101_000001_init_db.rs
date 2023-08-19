use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::*;
use sea_query::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Industry
        manager
            .create_type(
                Type::create()
                    .as_enum(Industry::Table)
                    .values(Industry::iter().skip(1))
                    .to_owned(),
            )
            .await?;

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
                    .col(
                        ColumnDef::new(Institution::Alias)
                            .array(ColumnType::String(Some(64)))
                            .not_null(),
                    )
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
                    .col(ColumnDef::new(Account::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Account::AccountName).string().not_null())
                    .col(
                        ColumnDef::new(Account::TwoFactorAuth)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Account::Recovery).string())
                    .col(ColumnDef::new(Account::UserId).integer().not_null())
                    .col(ColumnDef::new(Account::BucketId).integer())
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

        // SecureAccount
        manager
            .create_table(
                Table::create()
                    .table(SecureAccount::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SecureAccount::Id)
                            .integer()
                            .not_null()
                            .unique_key()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SecureAccount::Industry)
                            .enumeration(Industry::Table, Industry::iter().skip(1))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SecureAccount::AccountId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_secure_account_account_id")
                            .from(SecureAccount::Table, SecureAccount::AccountId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // SuperSecureAccount
        manager
            .create_table(
                Table::create()
                    .table(SuperSecureAccount::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SuperSecureAccount::Id)
                            .integer()
                            .not_null()
                            .unique_key()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SuperSecureAccount::Pin).integer().not_null())
                    .col(
                        ColumnDef::new(SuperSecureAccount::MinLength)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SuperSecureAccount::MaxLength)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SuperSecureAccount::Industry)
                            .enumeration(Industry::Table, Industry::iter().skip(1))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SuperSecureAccount::AccountId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_super_secure_account_account_id")
                            .from(SecureAccount::Table, SecureAccount::AccountId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // SsoAccount
        manager
            .create_table(
                Table::create()
                    .table(SsoAccount::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SsoAccount::Id)
                            .integer()
                            .not_null()
                            .unique_key()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SsoAccount::AccountId).integer().not_null())
                    .col(ColumnDef::new(SsoAccount::SsoId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sso_account_account_id")
                            .from(SecureAccount::Table, SecureAccount::AccountId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sso_account_sso_id")
                            .from(SsoAccount::Table, SsoAccount::SsoId)
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
        manager
            .drop_table(Table::drop().table(SecureAccount::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(SuperSecureAccount::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(SsoAccount::Table).to_owned())
            .await?;

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
        manager
            .drop_type(Type::drop().name(Industry::Table).to_owned())
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
    TwoFactorAuth,
    Recovery,
    UserId,
    BucketId,
    InstitutionId,
}

#[derive(Iden)]
enum SecureAccount {
    Table,
    Id,
    Industry,
    AccountId,
}

#[derive(Iden)]
enum SuperSecureAccount {
    Table,
    Id,
    Pin,
    MinLength,
    MaxLength,
    Industry,
    AccountId,
}

#[derive(Iden)]
enum SsoAccount {
    Table,
    Id,
    AccountId,
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

#[derive(Iden, EnumIter)]
pub enum Industry {
    Table,
    #[iden = "Tech"]
    Tech,
    #[iden = "Games"]
    Games,
    #[iden = "Social"]
    Social,
    #[iden = "Finance"]
    Finance,
    #[iden = "Shopping"]
    Shopping,
    #[iden = "Science"]
    Science,
    #[iden = "Other"]
    Other,
}
