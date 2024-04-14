DEFINE FUNCTION fn::create_bucket($name: string, $color: string) {
  LET $bucket = CREATE ONLY bucket CONTENT {
    name: $name,
    color: $color
  };

  RETURN $bucket.id;
};

DEFINE FUNCTION fn::edit_bucket($bucket_id: record, $name: string, $color: string) {
  UPDATE ONLY $bucket_id SET name = $name, color = $color;
};

DEFINE FUNCTION fn::delete_bucket($bucket_id: record) {
  -- Move all accounts of this bucket to the "unsorted bucket" (no bucketid)
  DELETE is_sorted_in WHERE out=$bucket_id;

  DELETE $bucket_id;
};