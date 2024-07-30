DEFINE FUNCTION fn::create_twofactor($name: string, $device: string) {
  LET $twofactor = CREATE ONLY twofactor CONTENT {
    name: $name,
    device: $device
  };

  RETURN $twofactor.id;
};

DEFINE FUNCTION fn::edit_twofactor($twofactor_id: record, $name: string, $device: string) {
  UPDATE ONLY $twofactor_id SET name = $name, device = $device;
};

DEFINE FUNCTION fn::delete_twofactor($twofactor_id: record) {
  DELETE is_secured_by WHERE out=$twofactor_id;

  DELETE $twofactor_id;
};