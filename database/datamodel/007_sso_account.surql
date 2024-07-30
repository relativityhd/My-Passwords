-- Table SSO Accounts (its a relation)
/**
In: The account (e.g. Figma)
Out: The sso account (e.g. Google)
*/

DEFINE TABLE use_sso_of SCHEMAFULL TYPE RELATION IN account OUT account
  PERMISSIONS FOR select, create, update, delete WHERE owner = $auth.id;

DEFINE FIELD owner ON use_sso_of
  TYPE record<user>
  DEFAULT $auth.id;

DEFINE INDEX account_has_single_sso ON TABLE use_sso_of COLUMNS in UNIQUE;
