#[derive(Clone, Copy, Debug)]
pub enum TestType {
    Empty,
    Ground,
    Resistance,
    InsulationPos,
    InsulationNeg,
}
impl TestType {

    pub fn get_text(self) -> String {
        match self {
            Self::Empty => String::from("Empty"),
            Self::Ground => String::from("Ground"),
            Self::Resistance => String::from("Resistance"),
            Self::InsulationPos => String::from("Insulation Positive"),
            Self::InsulationNeg => String::from("Insulation Negative"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ConnectorType {
    NONE,
    OJ10,
    OJ9,
    RTD,
    FULLTEST
}
impl ConnectorType {

    pub fn get_text(self) -> String {
        match self {
            Self::NONE => String::from("NONE"),
            Self::OJ10 => String::from("OJ10"),
            Self::OJ9 => String::from("OJ9"),
            Self::RTD => String::from("RTD"),
            Self::FULLTEST => String::from("FULLTEST"),         
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TestResult {
    NONE,
    PASS,
    FAIL,
}
impl TestResult {

    pub fn get_text(self) -> String {
        match self {
            Self::NONE => String::from("NONE"),      
            Self::PASS => String::from("PASS"),
            Self::FAIL => String::from("FAIL"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ConnectorRecord {
    pub connector: ConnectorType,
    pub test_value: f32,
    pub test_result: TestResult,
}
impl ConnectorRecord {

    pub fn new() -> Self {
        Self { connector: ConnectorType::NONE, test_value: 0.0, test_result: TestResult::NONE }
    }

    pub fn get_summary(self) -> String {
        let connector_string = self.connector.get_text();
        let test_value_string = self.test_value.to_string();
        let test_result_string = self.test_result.get_text();

        let summary = format!("{}: {} ({})", connector_string, test_value_string, test_result_string);

        summary
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TestSet {
    pub title: TestType,
    pub records_set: [ConnectorRecord; 3],
    pub finalized_result: TestResult
}
impl TestSet {

    pub fn print_test_summary(self) {
        let title_string = self.title.get_text();

        let record_set_string: Vec<String> = self.records_set
        .iter()
        .map(|each| {
            let summary = each.get_summary();
            summary
        }).collect();

        let final_result_string = self.finalized_result.get_text();

        println!("{} Test:", title_string);
        println!("{}", record_set_string[0]);
        println!("{}", record_set_string[1]);
        println!("{}", record_set_string[2]);
        println!("(TEST {}ED) \n", final_result_string);
    }

    pub fn _get_record_set(self) -> String{
        let record_set_string: Vec<String> = self.records_set
        .iter()
        .map(|each| {
            let summary = each.get_summary();
            summary
        }).collect();
        let _record_set_string = format!("{}\n{}\n{}", 
        record_set_string[0],
        record_set_string[1],
        record_set_string[2]);

        let test = format!("hello \\n world");

        test
    }
}