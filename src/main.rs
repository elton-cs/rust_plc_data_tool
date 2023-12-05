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
    // println!("{:#?}", data);


    // unmodified JSON of unit test
    let test1_vec = &data[0];
    println!("{:#?}", test1_vec);

    // title of the test
    let unit_test_title: plc_data::TestType = plc_fn::get_test_type(&test1_vec[0]);
    println!("{:#?}", unit_test_title);

    // splitting the connector type from the result of the test
    let unit_test_struct = plc_fn::split_connector_from_result(&test1_vec[2]);
    // println!("{:#?}", test1_struct);

    // connector type of the test
    let unit_test_connectors: Vec<plc_data::ConnectorType> = unit_test_struct
        .iter()
        .map(|value| plc_fn::get_connector_type(value))
        .collect();

    println!("{:#?}", unit_test_connectors);

    // results of the connector test
    let unit_test_results: Vec<plc_data::TestResult> = unit_test_struct
        .iter()
        .map(|value| plc_fn::get_test_result(value))
        .collect();

        println!("{:#?}", unit_test_results);

    //  vector of connector measured float values
    let unit_test_value_vec = plc_fn::split_connector_from_result(&test1_vec[1]);
    let unit_test_value_vec: Vec<f32> = unit_test_value_vec
    .iter()
    .map(|value| plc_fn::get_test_value(value))
    .collect();

    println!("{:#?}", unit_test_value_vec);

    // creating a connector record:


}