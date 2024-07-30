-- Table User and Sessions
DEFINE TABLE user SCHEMAFULL
  PERMISSIONS FOR select, update, delete WHERE id = $auth.id;

DEFINE FIELD username ON user
  TYPE string;
DEFINE FIELD email ON user
  TYPE string
  ASSERT string::is::email($value);
DEFINE FIELD password ON user
  TYPE string;

DEFINE INDEX email ON user COLUMNS email UNIQUE;
DEFINE INDEX username ON user COLUMNS username UNIQUE;

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
