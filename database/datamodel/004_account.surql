-- Table Accounts
DEFINE TABLE account SCHEMAFULL
  PERMISSIONS FOR select, create, update, delete WHERE owner = $auth.id;

DEFINE FIELD institution ON account
  TYPE string;
DEFINE FIELD created ON account
  TYPE datetime
  VALUE $before OR time::now()
  DEFAULT time::now();
DEFINE FIELD calls ON account
  TYPE int
  DEFAULT 0;
DEFINE FIELD recovery ON account
  TYPE option<string>;
DEFINE FIELD account_type ON account
  TYPE string;
DEFINE FIELD is_sso ON account
  TYPE bool
  DEFAULT false;
DEFINE FIELD website ON account
  TYPE option<string>;
DEFINE FIELD alias ON account
  TYPE array
  DEFAULT [];
DEFINE FIELD alias.* ON account
  TYPE string;
DEFINE FIELD archived ON account
  TYPE bool
  DEFAULT false;

DEFINE FIELD owner ON account
  TYPE record<user>
  DEFAULT $auth.id;

-- Relations
DEFINE TABLE is_sorted_in SCHEMAFULL TYPE RELATION IN account OUT bucket
  PERMISSIONS FOR select, create, update, delete WHERE owner = $auth.id;


DEFINE FIELD owner ON is_sorted_in
  TYPE record<user>
  DEFAULT $auth.id;

DEFINE INDEX account_has_single_bucket ON TABLE is_sorted_in COLUMNS in UNIQUE;


DEFINE TABLE is_secured_by SCHEMAFULL TYPE RELATION IN account OUT twofactor
  PERMISSIONS FOR select, create, update, delete WHERE owner = $auth.id;

DEFINE FIELD owner ON is_secured_by
  TYPE record<user>
  DEFAULT $auth.id;

DEFINE INDEX account_has_single_twofactor ON TABLE is_secured_by COLUMNS in UNIQUE;
