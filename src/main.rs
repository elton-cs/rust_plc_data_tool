mod plc_fn;
mod plc_data;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a file path as a command line argument");
    }
    let file_path = &args[1].as_str();

    let start_string = "\"messages\"";
    let end_string = "]";
    let messages = plc_fn::json_to_vector(file_path, start_string, end_string);

    let start_string = "\"payload\"";
    let new_vec = plc_fn::string_search_in_vector(messages, start_string);
    println!("{:#?}", new_vec);

    let new_vec = plc_fn::format_data(&new_vec);
    println!("{:#?}", new_vec);

    let new_vec = plc_fn::remove_odd_indices(new_vec);
    println!("{:#?}", new_vec);

    // Working data extraction procedure:
    // let file_path = "test_data_final.json";
    let big_data = plc_fn::split_data_vector(new_vec, 3);

    let all_test_sets_vector: Vec<plc_data::TestSet> = big_data.iter().map(|single_test_set| {
        // In this map, [single_test_set] is the unmodified JSON of one test in the vector (contains 3 lines of raw data):


        // title of the test set
        let title: plc_data::TestType = plc_fn::get_test_type(&single_test_set[0]);

        // splitting the connector type from the result of each test
        let connector_and_result = plc_fn::split_connector_from_result(&single_test_set[2]);

        // connector type of each test
        let connectors_vec: Vec<plc_data::ConnectorType> = connector_and_result
        .iter()
        .map(|value| plc_fn::get_connector_type(value))
        .collect();

        // measured values of each test (voltage, current, ohms etc...)
        let value_vec = plc_fn::split_connector_from_result(&single_test_set[1]);
        let value_vec: Vec<f32> = value_vec
        .iter()
        .map(|value| plc_fn::get_test_value(value))
        .collect();

        // results of each test (PASS/FAIL)
        let results_vec: Vec<plc_data::TestResult> = connector_and_result
        .iter()
        .map(|value| plc_fn::get_test_result(value))
        .collect();

        // creation of one test set (Ground Test, Resistance Test etc...)
        let test_set_1 = plc_fn::create_test_set(
            title, 
            3,
            connectors_vec, 
            value_vec, 
            results_vec);

        test_set_1
    }).collect();

    println!("{:#?}", all_test_sets_vector);

}