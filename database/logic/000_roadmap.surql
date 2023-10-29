/**
This file should contain all the busniess-logic in form of functions and events

Things to read:
- https://github.com/surrealdb/surrealdb/issues/1820#issuecomment-1539457546


Things to think about:
[x] Delete all the data from a user
[ ] At Bucket - delete: delete connection of existing accounts
[ ] At Account - edit: edit also the data from account type
[ ] At Account - delete: delete also the data from account type
[ ] At SSO - delete: think what should happen
[ ] At Two Factor - delete: think what should happen

Functions:
[ ] Create new account of type [secure, supersecure, sso, legacy] -> Use a transaction
[ ] Edit account of type [secure, supersecure, sso, legacy] -> Archive the whole account incl. type and create a new one
[ ] Delete account of type [secure, supersecure, sso, legacy] -> Just archive it
[ ] Create new bucket
[ ] Edit bucket
[ ] Delete bucket -> Move all accounts to "unsorted" (no bucket)
[ ] Create two factor
[ ] Edit two factor
[ ] Delete two factor
[ ] Edit Userdata
[ ] Delete User -> cascade

Events

*/
