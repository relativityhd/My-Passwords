-- Default create
LET $accountid = fn::create_secure_account("tobiah01@gmail.com", "IT", "Google", none, none, none, none, none);
SELECT ->is_secure->secure_account.identity as name FROM account WHERE id = $accountid; # Should be tobias01@gmail.com

-- Should fail
LET $accountid = fn::create_secure_account("tobiah01@gmail.com", "IT", "Google", none, none, none, "invalidbucket", none);

LET $accountid = fn::create_secure_account("tobiah01@gmail.com", "IT", "Google", none, none, none, none, "invalid2FA");

-- Extensive create
LET $bucket = CREATE ONLY bucket CONTENT {
  name: 'New Bucket',
  color: 'red'
};
LET $twofactor = CREATE ONLY twofactor CONTENT {
  name: 'MS Auth',
  device: 'A70'
};
LET $accountid = fn::create_secure_account("tobiah01@gmail.com", "IT", "Google", "recov", "www.google.de", ["Gog", "Go"], $bucket.id, $twofactor.id);
SELECT ->is_secure->secure_account.identity as name FROM account WHERE id = $accountid; # Should be tobias01@gmail.com

-- Check SSO Delete
LET $accountid = fn::create_secure_account("tobiah01@gmail.com", "IT", "Google", none, none, none, none, none);
SELECT * FROM use_sso_of;
SELECT * FROM account;

LET $sso = fn::create_sso_account($accountid, "Pinterest", none, none, none, none, none);
SELECT * FROM use_sso_of;
SELECT * FROM account;

fn::delete_account($accountid);