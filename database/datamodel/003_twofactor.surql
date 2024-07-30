-- Table TwoFactors
DEFINE TABLE twofactor SCHEMAFULL
  PERMISSIONS FOR select, create, update, delete WHERE owner = $auth.id;

DEFINE FIELD name ON twofactor
  TYPE string;
DEFINE FIELD device ON twofactor
  TYPE string;

DEFINE FIELD owner ON twofactor
  TYPE record<user>
  DEFAULT $auth.id;

DEFINE INDEX unique_twofactor_name_per_user ON twofactor COLUMNS owner, name UNIQUE;
