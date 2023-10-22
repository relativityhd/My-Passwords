-- Table Super Secure Accounts
DEFINE TABLE supersecure_account SCHEMAFULL
  PERMISSIONS FOR select, create, update, delete WHERE owner = $auth.id;

DEFINE FIELD industry ON supersecure_account
  TYPE string;
DEFINE FIELD identity ON supersecure_account
  TYPE string;
DEFINE FIELD pin ON supersecure_account
  TYPE int;
DEFINE FIELD min_length ON supersecure_account
  TYPE int;
DEFINE FIELD max_length ON supersecure_account
  TYPE int;

DEFINE FIELD owner ON supersecure_account
  TYPE record<user>
  DEFAULT $auth.id;

-- Relations
DEFINE TABLE is_supersecure SCHEMALESS
  PERMISSIONS FOR select, create, update, delete WHERE owner = $auth.id;

DEFINE FIELD in ON is_supersecure TYPE record<account>;
DEFINE FIELD out ON is_supersecure TYPE record<supersecure_account>;

DEFINE FIELD owner ON is_supersecure
  TYPE record<user>
  DEFAULT $auth.id;

DEFINE INDEX account_has_single_supersecure ON TABLE is_supersecure COLUMNS in UNIQUE;
DEFINE INDEX supersecure_has_single_account ON TABLE is_supersecure COLUMNS out UNIQUE;
