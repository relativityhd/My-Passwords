
DEFINE FUNCTION fn::create_secure_account(
  $identity: string,
  $industry: string,
  $institution: string,
  $recovery: option<string>,
  $website: option<string>,
  $alias: option<array<string>>,
  $bucket_id: option<string>,
  $twofactor_id: option<string>
) {
  LET $account = CREATE account CONTENT {
    institution: $institution,
    recovery: $recovery,
    account_type: "secure",
    website: $website,
    alias: IF $alias != NONE THEN $alias ELSE [] END,
  } RETURN id;

  LET $secure_account = CREATE secure_account CONTENT {
    identity: $identity,
    industry: $industry,
  };

  RELATE $account->is_secure->$secure_account;

  // Check if bucket exists
  IF $bucket_id {
    LET $bucket = SELECT * FROM type::thing("bucket", $bucket_id);
    IF !$bucket {
        THROW "Bucket does not exist"
    };
    RELATE $account->is_sorted_in->$bucket;
  };

  // Check if twofactor exists
  IF $twofactor_id {
    LET $twofactor = SELECT * FROM type::thing("twofactor", $twofactor_id);
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
  $bucket_id: option<string>,
  $twofactor_id: option<string>
) {
  LET $account = CREATE account CONTENT {
    institution: $institution,
    recovery: $recovery,
    account_type: "supersecure",
    website: $website,
    alias: IF $alias != NONE THEN $alias ELSE [] END,
  } RETURN id;

  LET $supersecure_account = CREATE supersecure_account CONTENT {
    identity: $identity,
    industry: $industry,
    pin: $pin,
    min_length: $min_length,
    max_length: $max_length,
  };

  RELATE $account->is_secure->$supersecure_account;

  // Check if bucket exists
  IF $bucket_id {
    LET $bucket = SELECT * FROM type::thing("bucket", $bucket_id);
    IF !$bucket {
        THROW "Bucket does not exist"
    };
    RELATE $account->is_sorted_in->$bucket;
  };

  // Check if twofactor exists
  IF $twofactor_id {
    LET $twofactor = SELECT * FROM type::thing("twofactor", $twofactor_id);
    IF !$twofactor {
        THROW "Two Factor does not exist"
    };
    RELATE $account->is_secured_by->$twofactor;
  };

  RETURN $account.id;
};


DEFINE FUNCTION fn::create_sso_account(
  $sso_account_id: string,
  $institution: string,
  $recovery: option<string>,
  $website: option<string>,
  $alias: option<array<string>>,
  $bucket_id: option<string>,
  $twofactor_id: option<string>
) {
  LET $account = CREATE account CONTENT {
    institution: $institution,
    recovery: $recovery,
    account_type: "supersecure",
    website: $website,
    alias: IF $alias != NONE THEN $alias ELSE [] END,
  } RETURN id;

  LET $sso_account = SELECT id FROM account where id = $sso_account_id;

  RELATE $account->use_sso_of->$sso_account;

  // Check if bucket exists
  IF $bucket_id {
    LET $bucket = SELECT * FROM type::thing("bucket", $bucket_id);
    IF !$bucket {
        THROW "Bucket does not exist"
    };
    RELATE $account->is_sorted_in->$bucket;
  };

  // Check if twofactor exists
  IF $twofactor_id {
    LET $twofactor = SELECT * FROM type::thing("twofactor", $twofactor_id);
    IF !$twofactor {
        THROW "Two Factor does not exist"
    };
    RELATE $account->is_secured_by->$twofactor;
  };

  RETURN $account.id;
};
