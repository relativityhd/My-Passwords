/**
This file should contain all the busniess-logic in form of functions and events

Things to read:
- https://github.com/surrealdb/surrealdb/issues/1820#issuecomment-1539457546


Things to think about:
[x] Delete all the data from a user
[x] At Bucket - delete: delete connection of existing accounts
[x] At Account - edit: edit also the data from account type
[x] At Account - delete: delete also the data from account type
[x] At SSO - delete: think what should happen
[x] At Two Factor - delete: think what should happen

Functions:
[x] Create new account of type [secure, supersecure, sso, legacy] -> Use a transaction
[x] Edit account of type [secure, supersecure, sso, legacy] -> Archive the whole account incl. type and create a new one
[x] Delete account of type [secure, supersecure, sso, legacy] -> Just archive it
[x] Create new bucket
[x] Edit bucket
[x] Delete bucket -> Move all accounts to "unsorted" (no bucket)
[x] Create two factor
[x] Edit two factor
[x] Delete two factor
[x] Edit Userdata
[x] Delete User -> cascade

Events

*/
