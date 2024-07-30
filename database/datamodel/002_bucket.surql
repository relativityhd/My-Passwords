-- Table Buckets
DEFINE TABLE bucket SCHEMAFULL
  PERMISSIONS FOR select, create, update, delete WHERE owner = $auth.id;

DEFINE FIELD name ON bucket
  TYPE string;
DEFINE FIELD color ON bucket
  TYPE string;

DEFINE FIELD owner ON bucket
  TYPE record<user>
  DEFAULT $auth.id;

DEFINE INDEX unique_bucket_name_per_user ON bucket COLUMNS owner, name UNIQUE;
