-- Default create
LET $accountid = fn::create_secure_account("tobiah01@gmail.com", "IT", "Google", none, none, none, none, none);
SELECT ->is_secure->secure_account.identity as name FROM account WHERE id = $accountid[0]; # Should be tobias01@gmail.com

-- Should fail
LET $accountid = fn::create_secure_account("tobiah01@gmail.com", "IT", "Google", none, none, none, "invalidbucket", none);

LET $accountid = fn::create_secure_account("tobiah01@gmail.com", "IT", "Google", none, none, none, none, "invalid2FA");

-- Extensive create
LET $bucket = CREATE bucket CONTENT {
  name: 'New Bucket',
  color: 'red'
};
LET $twofactor = CREATE twofactor CONTENT {
  name: 'MS Auth',
  device: 'A70'
};
LET $accountid = fn::create_secure_account("tobiah01@gmail.com", "IT", "Google", "recov", "www.google.de", ["Gog", "Go"], $bucket.id, $twofactor.id);
SELECT ->is_secure->secure_account.identity as name FROM account WHERE id = $accountid[0]; # Should be tobias01@gmail.com