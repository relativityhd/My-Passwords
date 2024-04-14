-- Default create
LET $bucketid = fn::create_bucket("work", "#bacdef");
SELECT * FROM bucket WHERE id = $bucketid;
fn::edit_bucket($bucketid, "personal", "#bacdef");
SELECT * FROM bucket WHERE id = $bucketid;

-- Default create
LET $accountid = fn::create_secure_account("tobiah01@gmail.com", "IT", "Google", none, none, none, $bucketid, none);
SELECT <-is_sorted_in<-account.institution as name FROM bucket WHERE id = $bucketid; # Should be Google

-- Delete bucket
fn::delete_bucket($bucketid);
SELECT bucket_id FROM account WHERE id = $accountid; # Should none
