-- Table Secure Accounts
DEFINE TABLE secure_account SCHEMAFULL
  PERMISSIONS FOR select, create, update, delete WHERE owner = $auth.id;

DEFINE FIELD industry ON secure_account
  TYPE string;
DEFINE FIELD identity ON secure_account
  TYPE string;

DEFINE FIELD owner ON secure_account
  TYPE record<user>
  DEFAULT $auth.id;

-- Relations
DEFINE TABLE is_secure SCHEMAFULL TYPE RELATION IN account OUT secure_account
  PERMISSIONS FOR select, create, update, delete WHERE owner = $auth.id;

DEFINE FIELD owner ON is_secure
  TYPE record<user>
  DEFAULT $auth.id;

DEFINE INDEX account_has_single_secure ON TABLE is_secure COLUMNS in UNIQUE;
DEFINE INDEX secure_has_single_account ON TABLE is_secure COLUMNS out UNIQUE;
