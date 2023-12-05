#[derive(Debug)]
pub enum TestType {
    None,
    Ground,
    Resistance,
    InsulationPos,
    InsulationNeg,
}

#[derive(Clone, Copy, Debug)]
pub enum ConnectorType {
    NONE,
    OJ10,
    OJ9,
    RTD,
    FULLTEST
}

#[derive(Clone, Copy, Debug)]
pub enum TestResult {
    PASS,
    FAIL,
    NONE,
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