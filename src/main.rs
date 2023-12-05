use std::fs;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::io::Write;

// fn rename_json( old_name: &str, new_name: &str){
//     fs::rename(old_name, new_name)
//     .expect("Could not rename the file");
// }

fn search_by_string_json(file_path: &str, start_string: &str) -> Vec<String> {
    let file = fs::File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    let mut found_start_line = false;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let trimmed_line = line.trim();
        
        if trimmed_line.starts_with(start_string) {
            found_start_line = true;
        }

        if found_start_line {
            lines.push(String::from(trimmed_line));
            found_start_line = false;
        }
    }

    lines
}

fn save_block_json(file_path: &str, start_string: &str, end_string: &str) -> Vec<String> {
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


fn save_as_json(lines: Vec<String>, file_path: &str) {
    let json_content = lines.join("\n");

    let mut file = File::create(file_path).expect("Failed to create file");
    file.write_all(json_content.as_bytes())
        .expect("Failed to write to file");
}


fn main() {
    // rename_json("tests_export_copy", "tests_export_copy.json");

    let file_path = "tests_export_copy.json";

    // let lines = search_by_string_json(file_path, "\"messages\"");
    // for line in lines {
    //     println!("{}", line);
    // }

    let lines = save_block_json(file_path, "\"messages\"", "]");
    save_as_json(lines, "modified_messages.json")

}
