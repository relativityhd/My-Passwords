-- Table Legacy Accounts
DEFINE TABLE legacy_account SCHEMAFULL
  PERMISSIONS FOR select, create, update, delete WHERE owner = $auth.id;

DEFINE FIELD industry ON legacy_account
  TYPE int;
DEFINE FIELD identity ON legacy_account
  TYPE string;

DEFINE FIELD owner ON legacy_account
  TYPE record<user>
  DEFAULT $auth.id;

-- Relations
DEFINE TABLE is_legacy SCHEMAFULL
  PERMISSIONS FOR select, create, update, delete WHERE owner = $auth.id;

DEFINE FIELD in ON is_legacy TYPE record<account>;
DEFINE FIELD out ON is_legacy TYPE record<legacy_account>;

DEFINE FIELD owner ON is_legacy
  TYPE record<user>
  DEFAULT $auth.id;

DEFINE INDEX account_has_single_legacy ON TABLE is_legacy COLUMNS in UNIQUE;
DEFINE INDEX legacy_has_single_account ON TABLE is_legacy COLUMNS out UNIQUE;
