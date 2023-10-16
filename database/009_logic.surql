/**
This file should contain all the busniess-logic in form of functions and events

Things to read:
- https://github.com/surrealdb/surrealdb/issues/1820#issuecomment-1539457546


Things to think about:
- Delete all the data from a user
- At Bucket - delete: delete connection of existing accounts
- At Account - edit: edit also the data from account type
- At Account - delete: delete also the data from account type
- At SSO - delete: think what should happen
- At Two Factor - delete: think what should happen

Functions:
- Create new account of type [secure, supersecure, sso, legacy] -> Use a transaction
- Edit account of type [secure, supersecure, sso, legacy] -> Archive the whole account incl. type and create a new one
- Delete account of type [secure, supersecure, sso, legacy] -> Just archive it
- Create new bucket
- Edit bucket
- Delete bucket -> Move all accounts to "unsorted" (no bucket)
- Create two factor
- Edit two factor
- Delete two factor
- Edit Userdata
- Delete User -> cascade

Events

*/

DEFINE FUNCTION fn::create_secure_account(
  $identity: string,
  $industry: string,
  $institution: string,
  $recovery: option<string>,
  $website: option<string>,
  $alias: option<string>,
  $bucket_id: option<string>,
  // TODO: twofactor
) {
  BEGIN TRANSACTION;


  LET $account = CREATE account CONTENT {
    institution = $institution,
    recovery = $recovery,
    account_type = "secure",
    website = $website,
    alias = [$alias],
  } RETURN id;

  LET $secure_account = CREATE secure_account CONTENT {
    identity = $identity,
    industry = $industry,
  };

  RELATE $account->is_secure->$secure_account;

  // Check if bucket exists
  IF $bucket_id != NONE THEN
    LET $bucket = SELECT id FROM bucket WHERE id = $bucket_id;
    IF $bucket == NONE THEN
      ROLLBACK TRANSACTION;
      THROW "Bucket does not exist";
    END IF;

    RELATE $bucket->contains->$account;
  END IF;

  COMMIT TRANSACTION;
};
