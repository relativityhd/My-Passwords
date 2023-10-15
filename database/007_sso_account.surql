-- Table SSO Accounts (its a relation)
DEFINE TABLE use_sso_of SCHEMAFULL
  PERMISSIONS FOR select, create, update, delete WHERE owner = $auth.id;

DEFINE FIELD in ON use_sso_of TYPE record<account>; // The account (e.g. Figma)
DEFINE FIELD out ON use_sso_of TYPE record<account>; // The sso account (e.g. Google)

DEFINE FIELD owner ON use_sso_of
  TYPE record<user>
  DEFAULT $auth.id;

DEFINE INDEX account_has_single_sso ON TABLE use_sso_of COLUMNS in UNIQUE;
