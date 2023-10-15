/**
Init script for setting up the SurrealDB database.
*/

// TODO: Define Indices for all tables, depending on whats needed

-- Namespace and Database
DEFINE NAMESPACE accounts;
USE NAMESPACE accounts;
DEFINE DATABASE dev;
USE DB dev;

-- Table User and Sessions
DEFINE TABLE user SCHEMAFULL
  PERMISSIONS FOR select, update, delete WHERE id = $auth.id;

DEFINE FIELD username ON user TYPE string;
DEFINE FIELD email ON user TYPE string
  ASSERT string::is::email($value);
DEFINE FIELD password ON user TYPE string;

DEFINE INDEX email ON user FIELDS email UNIQUE;
DEFINE INDEX username ON user FIELDS username UNIQUE;

DEFINE SCOPE user SESSION 4w
  SIGNIN (
    SELECT *
    FROM user
    WHERE (email = $email or username = $username)
      AND crypto::argon2::compare(password, $password)
  )
  SIGNUP (
    CREATE user
    CONTENT {
      username: $username,
      email: $email,
      password: crypto::argon2::generate($password)
    }
  );

-- Table Institutions
DEFINE TABLE institution SCHEMAFULL
  PERMISSIONS FOR select, create, update, delete WHERE user = $auth.id;

DEFINE FIELD name ON institution TYPE string;
DEFINE FIELD is_sso ON institution TYPE bool DEFAULT false;
DEFINE FIELD alias ON institution TYPE array DEFAULT [];
DEFINE FIELD alias.* ON institution TYPE string;
DEFINE FIELD website ON institution TYPE option<string>;

DEFINE FIELD user ON institution TYPE record(user);

-- Table Buckets
DEFINE TABLE bucket SCHEMAFULL
  PERMISSIONS FOR select, create, update, delete WHERE user = $auth.id;

DEFINE FIELD name ON bucket TYPE string;
DEFINE FIELD color ON bucket TYPE string;

DEFINE FIELD user ON institution TYPE record(user);

-- Table TwoFactors
DEFINE TABLE two_factor SCHEMAFULL
  PERMISSIONS FOR select, create, update, delete WHERE user = $auth.id;

DEFINE FIELD name ON two_factor TYPE string;
DEFINE FIELD device ON two_factor TYPE string;

DEFINE FIELD user ON two_factor TYPE record(user);

-- Table Accounts
DEFINE TABLE account SCHEMAFULL
  PERMISSIONS FOR select, create, update, delete WHERE user = $auth.id;

DEFINE FIELD created_at ON account TYPE datetime DEFAULT time::now();
DEFINE FIELD recovery ON account TYPE option<string>;
DEFINE FIELD account_type ON account TYPE string;

DEFINE FIELD user ON account TYPE record(user);
DEFINE FIELD bucket ON account TYPE option<record(bucket)>;
DEFINE FIELD two_factor ON account TYPE option<record(two_factor)>;
DEFINE FIELD institution ON account TYPE record(institution);

-- Table Secure Accounts
DEFINE TABLE secure_account SCHEMAFULL
  PERMISSIONS FOR select, create, update, delete WHERE account.user = $auth.id;

DEFINE FIELD industry ON secure_account TYPE int;
DEFINE FIELD identity ON account TYPE string;

DEFINE FIELD account ON secure_account TYPE record(account);

-- Table Super Secure Accounts
DEFINE TABLE super_secure_account SCHEMAFULL
  PERMISSIONS FOR select, create, update, delete WHERE account.user = $auth.id;

DEFINE FIELD pin ON super_secure_account TYPE int;
DEFINE FIELD min_length ON super_secure_account TYPE int;
DEFINE FIELD max_length ON super_secure_account TYPE int;
DEFINE FIELD industry ON super_secure_account TYPE int;
DEFINE FIELD identity ON account TYPE string;

DEFINE FIELD account ON super_secure_account TYPE record(account);

-- Table SSO Accounts
DEFINE TABLE sso_account SCHEMAFULL
  PERMISSIONS FOR select, create, update, delete WHERE account.user = $auth.id;

DEFINE FIELD account ON sso_account TYPE record(account);
DEFINE FIELD sso ON sso_account TYPE record(institution);
