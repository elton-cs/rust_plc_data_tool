mod plc_data;
mod plc_fn;
mod plc_data_extract;

use std::io::Empty;

use plc_data::TestSet;
use plc_data_extract::extract_plc_data;
use plc_fn::just_a_pause;
use rust_xlsxwriter::Workbook;
use calamine::{open_workbook, Error, Xlsx, Reader, RangeDeserializerBuilder};

//  -> Result<(), XlsxError>
fn main() {

    // let test_set: Vec<TestSet> = extract_plc_data();

    let mut write_workbook = Workbook::new();
    let mut worksheet = write_workbook.add_worksheet().set_name("PG 3").expect("could not add worksheet");

    let mut read_workbook: Xlsx<_> = open_workbook("example.xlsx").unwrap();
    if let Some(Ok(all_rows)) = read_workbook.worksheet_range("PG 3") {
        let mut row_counter = 0;
        for row in all_rows.rows() {
            let mut column_counter = 0; 
            for cell in row {
                let value = cell.as_string();
                worksheet.write(row_counter, column_counter, value );
                println!("{:?}", cell);
                column_counter += 1;
            }
            row_counter += 1;
        }
    }

    write_workbook.save("demo.xlsx");

    // just_a_pause();
}

fn copy_excel_data() {
    let mut excel: Xlsx<_> = open_workbook("example.xlsx").unwrap();
    if let Some(Ok(all_rows)) = excel.worksheet_range("PG 3") {
        for row in all_rows.rows() {
            println!("{:?}", row);
        }
    }
}