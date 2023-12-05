mod plc_fn;
mod plc_data;

fn main() {

    let file_path = "tests_export_copy.json";
    let start_string = "\"messages\"";
    let end_string = "]";
    let lines = plc_fn::save_block_json(file_path, start_string, end_string);
    plc_fn::save_as_json(&lines, "modified_messages.json");

    let file_path = "modified_messages.json";
    let start_string = "\"payload\"";
    let payload = plc_fn::search_by_string_json(file_path, start_string);

    let payload = plc_fn::format_data(&payload);
    plc_fn::save_as_json(&payload, "payload.json");

    let payload = plc_fn::remove_odd_indices(payload);
    plc_fn::save_as_json(&payload, "payload_nodups.json");

    let file_path = "payload_nodups.json";
    let data = plc_fn::split_data_vector(file_path, 3);
    println!("{:#?}", data);

}