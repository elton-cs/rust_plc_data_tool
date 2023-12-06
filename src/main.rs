mod plc_data;
mod plc_fn;
mod plc_data_extract;

use plc_data::TestSet;
use plc_data_extract::extract_plc_data;
use plc_fn::just_a_pause;

//  -> Result<(), XlsxError>
fn main() {

    let test_set: Vec<TestSet> = extract_plc_data();

    just_a_pause();
}