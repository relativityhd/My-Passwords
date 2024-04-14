
-- Create Functions

DEFINE FUNCTION fn::create_secure_account(
  $identity: string,
  $industry: string,
  $institution: string,
  $recovery: option<string>,
  $website: option<string>,
  $alias: option<array<string>>,
  $bucket_id: option<record>,
  $twofactor_id: option<record>
) {
  LET $account = CREATE ONLY account CONTENT {
    institution: $institution,
    recovery: $recovery,
    account_type: "Secure",
    website: $website,
    alias: IF $alias != NONE THEN $alias ELSE [] END
  } RETURN id;

  LET $secure_account = CREATE ONLY secure_account CONTENT {
    identity: $identity,
    industry: $industry
  };

  RELATE $account->is_secure->$secure_account;

  // Check if bucket exists
  IF $bucket_id {
    LET $bucket = SELECT * FROM ONLY $bucket_id;
    IF !$bucket {
        THROW "Bucket does not exist"
    };
    RELATE $account->is_sorted_in->$bucket;
  };

  // Check if twofactor exists
  IF $twofactor_id {
    LET $twofactor = SELECT * FROM ONLY $twofactor_id;
    IF !$twofactor {
        THROW "Two Factor does not exist"
    };
    RELATE $account->is_secured_by->$twofactor;
  };

  RETURN $account.id;
};

DEFINE FUNCTION fn::create_supersecure_account(
  $identity: string,
  $industry: string,
  $pin: int,
  $min_length: int,
  $max_length: int,
  $institution: string,
  $recovery: option<string>,
  $website: option<string>,
  $alias: option<array<string>>,
  $bucket_id: option<record>,
  $twofactor_id: option<record>
) {
  LET $account = CREATE ONLY account CONTENT {
    institution: $institution,
    recovery: $recovery,
    account_type: "SuperSecure",
    website: $website,
    alias: IF $alias != NONE THEN $alias ELSE [] END
  } RETURN id;

  LET $supersecure_account = CREATE ONLY supersecure_account CONTENT {
    identity: $identity,
    industry: $industry,
    pin: $pin,
    min_length: $min_length,
    max_length: $max_length
  };

  RELATE $account->is_supersecure->$supersecure_account;

  // Check if bucket exists
  IF $bucket_id {
    LET $bucket = SELECT * FROM ONLY $bucket_id;
    IF !$bucket {
        THROW "Bucket does not exist"
    };
    RELATE $account->is_sorted_in->$bucket;
  };

  // Check if twofactor exists
  IF $twofactor_id {
    LET $twofactor = SELECT * FROM ONLY $twofactor_id;
    IF !$twofactor {
        THROW "Two Factor does not exist"
    };
    RELATE $account->is_secured_by->$twofactor;
  };

  RETURN $account.id;
};


DEFINE FUNCTION fn::create_sso_account(
  $sso_account_id: record,
  $institution: string,
  $recovery: option<string>,
  $website: option<string>,
  $alias: option<array<string>>,
  $bucket_id: option<record>,
  $twofactor_id: option<record>
) {
  LET $account = CREATE ONLY account CONTENT {
    institution: $institution,
    recovery: $recovery,
    account_type: "Sso",
    website: $website,
    alias: IF $alias != NONE THEN $alias ELSE [] END
  } RETURN id;

  LET $sso_account = SELECT id FROM ONLY $sso_account_id;

  RELATE $account->use_sso_of->$sso_account;

  // Check if bucket exists
  IF $bucket_id {
    LET $bucket = SELECT * FROM ONLY $bucket_id;
    IF !$bucket {
        THROW "Bucket does not exist"
    };
    RELATE $account->is_sorted_in->$bucket;
  };

  // Check if twofactor exists
  IF $twofactor_id {
    LET $twofactor = SELECT * FROM ONLY $twofactor_id;
    IF !$twofactor {
        THROW "Two Factor does not exist"
    };
    RELATE $account->is_secured_by->$twofactor;
  };

  RETURN $account.id;
};


-- Edit Functions
DEFINE FUNCTION fn::edit_secure_account(
  $account_id: record,
  $identity: string,
  $industry: string,
  $institution: string,
  $recovery: option<string>,
  $website: option<string>,
  $alias: option<array<string>>,
  $bucket_id: option<record>,
  $twofactor_id: option<record>
) {
  LET $created_at = SELECT created_at FROM ONLY $account_id;
  LET $new_account_id = fn::create_secure_account($identity, $industry, $institution, $recovery, $website, $alias, $bucket_id, $twofactor_id);

  -- Set created_at to the old value
  UPDATE ONLY $new_account_id SET created_at = $created_at;

  -- Delete old account
  fn::delete_account($account_id);
  RETURN $new_account_id;
};


DEFINE FUNCTION fn::edit_super_secure_account(
  $account_id: record,
  $identity: string,
  $industry: string,
  $pin: int,
  $min_length: int,
  $max_length: int,
  $institution: string,
  $recovery: option<string>,
  $website: option<string>,
  $alias: option<array<string>>,
  $bucket_id: option<record>,
  $twofactor_id: option<record>
) {
  LET $created_at = SELECT created_at FROM ONLY $account_id;
  LET $new_account_id = fn::create_supersecure_account($identity, $industry, $pin, $min_length, $max_length, $institution, $recovery, $website, $alias, $bucket_id, $twofactor_id);

  -- Set created_at to the old value
  UPDATE ONLY $new_account_id SET created_at = $created_at;

  -- Delete old account
  fn::delete_account($account_id);
  RETURN $new_account_id;
};


DEFINE FUNCTION fn::edit_sso_account(
  $account_id: record,
  $sso_account_id: record,
  $institution: string,
  $recovery: option<string>,
  $website: option<string>,
  $alias: option<array<string>>,
  $bucket_id: option<record>,
  $twofactor_id: option<record>
) {
  LET $created_at = SELECT created_at FROM ONLY $account_id;
  LET $new_account_id = fn::create_sso_account($sso_account_id, $institution, $recovery, $website, $alias, $bucket_id, $twofactor_id);

  -- Set created_at to the old value
  UPDATE ONLY $new_account_id SET created_at = $created_at;

  -- Delete old account
  fn::delete_account($account_id);
  RETURN $new_account_id;
};

-- Delete Functions
DEFINE FUNCTION fn::delete_account(
  $account_id: record,
) {
  -- Check for sso account
  LET $secured_accounts = SELECT in FROM use_sso_of WHERE out = $account_id;
  LET $n = array::len($secured_accounts);
  IF $n > 0 {
    THROW string::concat('Account is used by ', type::string($n), ' other accounts');
  };

  UPDATE ONLY $account_id SET updated = time::NOW(), archived = true;
};
