mod plc_fn;


fn main() {

    let file_path = "tests_export_copy.json";
    let lines = plc_fn::save_block_json(file_path, "\"messages\"", "]");
    plc_fn::save_as_json(&lines, "modified_messages.json");

    let file_path = "modified_messages.json";
    let payload = plc_fn::search_by_string_json(file_path, "\"payload\"");

    let payload = plc_fn::format_data(&payload);
    plc_fn::save_as_json(&payload, "payload.json");

    let payload = plc_fn::remove_odd_indices(payload);
    plc_fn::save_as_json(&payload, "payload_nodups.json");
}