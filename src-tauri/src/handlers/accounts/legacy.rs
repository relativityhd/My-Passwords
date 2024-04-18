use crate::algorithm::gen_legacy_pw;
use crate::errors::AccountError;
use crate::handlers::accounts::add_call;
use crate::types::{
    extract_lc,
    handlers::{AccountMetadata, LegacyOverview},
    Industry, LocalCreds, DB, LC,
};
use serde::{Deserialize, Serialize};
use specta::Type;
use surrealdb::sql::Thing;

#[derive(Deserialize, Serialize)]
struct LegacyPasswordData {
    institution: String,
    industry: Industry,
}

#[tauri::command]
#[specta::specta]
pub async fn get_legacy_password(db: DB<'_>, lc: LC<'_>, id: &str) -> Result<String, AccountError> {
    let local_creds = extract_lc(&lc).await?;
    let secret = local_creds.secret;
    // Convert id to Record
    let sql = "SELECT institution,
        (->is_legacy->legacy_account.industry)[0] as industry
        FROM ONLY type::thing('account', $account);";
    let id = id.split(':').last().unwrap();
    let data = db
        .query(sql)
        .bind(("account", id))
        .await?
        .take::<Option<LegacyPasswordData>>(0)?
        .ok_or(AccountError::AccountNotFound(id.to_string()))?;
    let pw = gen_legacy_pw(&data.institution, &data.industry, &secret.to_string());
    add_call(db, id).await?;
    Ok(pw)
}

#[tauri::command]
#[specta::specta]
pub async fn get_legacy_overview(
    db: DB<'_>,
    lc: LC<'_>,
    id: &str,
) -> Result<(LegacyOverview, String), AccountError> {
    let local_creds = extract_lc(&lc).await?;
    let secret = local_creds.secret;

    let sql = "SELECT
        institution,
        recovery,
        website,
        alias,
        account_type as mode,
        type::string(created) as created,
        (->is_legacy->legacy_account.industry)[0] as industry,
        (SELECT
            type::string(id) as id,
            color,
            name,
            array::len(<-is_sorted_in<-account) as n
            FROM (->is_sorted_in->bucket))[0] as bucket,
        (SELECT type::string(id) as id, name, device FROM (->is_secured_by->twofactor))[0] as twofactor
        FROM ONLY type::thing('account', $account);";
    let id = id.split(':').last().unwrap();
    let data = db
        .query(sql)
        .bind(("account", id))
        .await?
        .take::<Option<LegacyOverview>>(0)?
        .ok_or(AccountError::AccountNotFound(id.to_string()))?;
    let pw = gen_legacy_pw(&data.institution, &data.industry, &secret.to_string());

    Ok((data, pw))
}

#[derive(Deserialize, Serialize, Type)]
pub struct LegacyData {
    pub institution: String,
    pub industry: u32,
}

#[derive(Deserialize, Serialize, Type)]
pub struct LegacySuperData {
    pub institution: String,
    pub industry: u32,
    pub idendity: String,
    pub seed: u32,
    pub min: u32,
    pub max: u32,
    pub specials: String,
}

#[derive(Debug, Serialize)]
struct NewLegacyAccount {
    account_type: String,
    institution: String,
}

#[derive(Debug, Serialize)]
struct NewLegacy {
    industry: Industry,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tauri::command]
#[specta::specta]
pub async fn load_from_json(
    db: DB<'_>,
    data: Vec<LegacyData>,
    superdata: Vec<LegacySuperData>,
) -> Result<(), AccountError> {
    for d in data {
        let industry = match d.industry {
            1 => Industry::Games,
            2 => Industry::Tech,
            3 => Industry::Social,
            4 => Industry::Finance,
            _ => Industry::Other,
        };
        let created: Vec<Record> = db
            .create("account")
            .content(NewLegacyAccount {
                account_type: "LegacySecure".to_string(),
                institution: d.institution,
            })
            .await?;
        let legacycreated: Vec<Record> = db
            .create("legacy_account")
            .content(NewLegacy { industry: industry })
            .await?;
        if created.len() != 1 || legacycreated.len() != 1 {
            dbg!(created);
            dbg!(legacycreated);
            return Err(AccountError::NoID);
        }
        let sql = "RELATE $account->is_legacy->$legacy_account;";
        db.query(sql)
            .bind(("account", &created[0].id))
            .bind(("legacy_account", &legacycreated[0].id))
            .await?;
    }

    for d in superdata {
        let industry = match d.industry {
            1 => Industry::Games,
            2 => Industry::Tech,
            3 => Industry::Social,
            4 => Industry::Finance,
            _ => Industry::Other,
        };
        let sql = "
            fn::create_supersecure_account(
                $identity,
                $industry,
                $specials,
                $seed,
                $min_length,
                $max_length,
                $institution,
                none,
                none,
                none,
                none,
                none
            );
        ";
        db.query(sql)
            .bind(("identity", d.idendity))
            .bind(("industry", industry))
            .bind(("specials", d.specials))
            .bind(("seed", d.seed))
            .bind(("min_length", d.min))
            .bind(("max_length", d.max))
            .bind(("institution", d.institution))
            .await?
            .take::<Option<Thing>>(0)?
            .ok_or(AccountError::NoID)?;
    }

    Ok(())
}
