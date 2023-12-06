#[derive(Debug)]
pub enum TestType {
    Empty,
    Ground,
    Resistance,
    InsulationPos,
    InsulationNeg,
}
impl TestType {

    fn get_text(self) -> String {
        match self {
            Self::Empty => String::from("Empty"),
            Self::Ground => String::from("Ground"),
            Self::Resistance => String::from("Resistance"),
            Self::InsulationPos => String::from("InsulationPos"),
            Self::InsulationNeg => String::from("InsulationNeg"),
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

    fn get_text(self) -> String {
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

    fn get_text(self) -> String {
        match self {
            Self::NONE => String::from("NONE"),      
            Self::PASS => String::from("PASS"),
            Self::FAIL => String::from("FAIL"),
        }
    }
}

#[derive(Debug)]
pub struct ConnectorRecord {
    pub connector: ConnectorType,
    pub test_value: f32,
    pub test_result: TestResult,
}

impl ConnectorRecord {
    pub fn new() -> Self {
        Self { connector: ConnectorType::NONE, test_value: 0.0, test_result: TestResult::NONE }
    }
}

#[derive(Debug)]
pub struct TestSet {
    pub title: TestType,
    pub records_set: [ConnectorRecord; 3],
    pub finalized_result: TestResult
}