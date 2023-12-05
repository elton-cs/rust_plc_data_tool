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

#[derive(Debug)]
pub struct TestSet {
    title: TestType,
    records_set: (ConnectorRecord, ConnectorRecord, ConnectorRecord),
    finalized_result: TestResult
}