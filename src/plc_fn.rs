use std::fs;
use std::io::{BufRead, BufReader, Write};
use crate::plc_data::*;

pub fn create_single_connector_record( 
    connector: ConnectorType, 
    test_value: f32, 
    test_result: TestResult) -> ConnectorRecord {
    
    let single_record = ConnectorRecord{
        connector,
        test_value,
        test_result,
    };

    single_record
}

pub fn create_test_set (
    title: TestType, 
    num_of_connectors: usize,
    connector: Vec<ConnectorType>, 
    test_value: Vec<f32>, 
    test_result: Vec<TestResult>) -> TestSet {

    let mut records_set = [ConnectorRecord::new(), ConnectorRecord::new(), ConnectorRecord::new()];
    let finalized_result = test_result[3];
    
    for i in 0..num_of_connectors {
        records_set[i] = create_single_connector_record(
            connector[i], 
            test_value[i], 
            test_result[i] );
    }

    let test_set = TestSet {
        title,
        records_set,
        finalized_result,
    };

    test_set
}

pub fn format_data(lines: &Vec<String>) -> Vec<String> {
    lines.
    iter()
    .map(|s| 
        String::from(
            s.as_str()
            .strip_prefix("\"payload\": ")
            .unwrap()
            .trim_matches(',')
            .trim_matches('\"')
        )
    )
    .collect()
}

pub fn get_test_type(name: &String) -> TestType {
    match  name.as_str() {
        "Ground Test Result:"       => TestType::Ground,
        "Resistance Test Result:"   => TestType::Resistance,
        "InsulationP Test Result:"  => TestType::InsulationPos,
        "InsulationN Test Result:"  => TestType::InsulationNeg,
        _                           => TestType::Empty,
    }
}

pub fn get_connector_type(name: &String) -> ConnectorType {
    let slice = name.split_whitespace().next().unwrap();
    match slice {
        "OJ10"  => ConnectorType::OJ10,
        "OJ9"   => ConnectorType::OJ9,
        "RTD"   => ConnectorType::RTD,
        "TEST"  => ConnectorType::FULLTEST,
        _       => ConnectorType::NONE,
    }
}

pub fn get_test_result(name: &String) -> TestResult {
    let mut words = name.split_whitespace();
    words.next();
    let result = words.next().unwrap();
    match result {
        "FAIL"  => TestResult::FAIL,
        "FAILED"  => TestResult::FAIL,
        "PASS"  => TestResult::PASS,
        "PASSED"  => TestResult::PASS,
        _       => TestResult::NONE,
    }
}

pub fn get_test_value(name: &String) -> f32 {

    let name = name.parse::<f32>().unwrap();

    name
}

pub fn remove_odd_indices<T>(vec: Vec<T>) -> Vec<T> {
    let mut result = Vec::new();

    for (index, value) in vec.into_iter().enumerate() {
        if index % 2 == 0 {
            result.push(value);
        }
    }

    result
}

pub fn _rename_json( old_name: &str, new_name: &str){
    fs::rename(old_name, new_name)
    .expect("Could not rename the file");
}


pub fn _save_as_json(lines: &Vec<String>, file_path: &str) {
    let json_content = lines.join("\n");

    let mut file = fs::File::create(file_path).expect("Failed to create file");
    file.write_all(json_content.as_bytes())
        .expect("Failed to write to file");
}

pub fn json_to_vector(file_path: &str, start_string: &str, end_string: &str) -> Vec<String> {
    let file = fs::File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    let mut found_start_line = false;

    lines.push(String::from("{"));
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let trimmed_line = line.trim();
        
        if trimmed_line.starts_with(start_string) {
            found_start_line = true;
            
        } else if trimmed_line.starts_with(end_string) {
            lines.push(String::from(end_string));
            break;
        }

        if found_start_line {
            lines.push(String::from(trimmed_line));
        }

    }
    lines.push(String::from("}"));

    lines
}

pub fn string_search_in_vector(original_string_vector: Vec<String>, start_string: &str) -> Vec<String> {

    let mut mod_string_vector: Vec<String> = Vec::new();
    for line in original_string_vector {
        if line.starts_with(start_string){
            mod_string_vector.push(line);
        }
    }
    mod_string_vector

}

pub fn split_connector_from_result(name: &String) -> Vec<String> {
    let name = name.as_str();

    let connector: Vec<&str> = name.split_terminator('\\').collect();

    let new_connectors: Vec<String> = connector.iter().map(|s| {
        s.trim_matches('n').to_string()
    }).collect();

    new_connectors
}

// pub fn split_data_vector(file_path: &str, lines_per_slice: u64 ) -> Vec<Vec<String>> {
//     let file = fs::File::open(file_path).expect("Failed to open file");
//     let reader = BufReader::new(file);

//     let mut single_test_vec: Vec<String> = Vec::new();
//     let mut tests_vec: Vec<Vec<String>> = Vec::new();
//     let mut counter = 0;

//     for line in reader.lines() {
//         let line = line.expect("Failed to read line");
//         single_test_vec.push(line);
//         counter += 1;
//         if counter == lines_per_slice {
//             tests_vec.push(single_test_vec.clone());
//             single_test_vec = Vec::new();
//             counter = 0;
//         }
//     }

//     tests_vec
// }

pub fn split_data_vector(original_string_vector: Vec<String>, lines_per_slice: u64 ) -> Vec<Vec<String>> {

    let mut single_test_vec: Vec<String> = Vec::new();
    let mut tests_vec: Vec<Vec<String>> = Vec::new();
    let mut counter = 0;

    for line in original_string_vector {
        single_test_vec.push(line);
        counter += 1;
        if counter == lines_per_slice {
            tests_vec.push(single_test_vec.clone());
            single_test_vec = Vec::new();
            counter = 0;
        }
    };

    // for line in reader.lines() {
    //     let line = line.expect("Failed to read line");
    //     single_test_vec.push(line);
    //     counter += 1;
    //     if counter == lines_per_slice {
    //         tests_vec.push(single_test_vec.clone());
    //         single_test_vec = Vec::new();
    //         counter = 0;
    //     }
    // }

    tests_vec
}