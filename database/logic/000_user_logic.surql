DEFINE EVENT on_user_delete ON TABLE user WHEN $event = "DELETE" THEN {
  -- Delete all the relations
  FOR $table IN ['is_sorted_in', 'is_secured_by', 'is_secure', 'is_supersecure', 'is_legacy', 'use_sso_of'] {
    DELETE type::table($table) WHERE id IN (
      SELECT id FROM type::table($table) WHERE owner = $before.id
    );
  };

  -- Delete all user data
  FOR $table IN ['account', 'secure_account', 'supersecure_account', 'legacy_account', 'bucket', 'twofactor'] {
    DELETE type::table($table) WHERE id IN (
      SELECT id FROM type::table($table) WHERE owner = $before.id
    );
  };
};

DEFINE FUNCTION fn::change_email($new_email: string) {
  UPDATE user SET email = $new_email WHERE id = $auth.id;
};