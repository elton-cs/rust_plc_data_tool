#[derive(Debug)]
pub enum TestType {
    None,
    Ground,
    Resistance,
    InsulationPos,
    InsulationNeg,
}

#[derive(Debug)]
pub enum ConnectorType {
    NONE,
    OJ10,
    OJ9,
    RTD,
    FULLTEST
}

pub enum TestResult {
    PASS,
    FAIL,
}

// pub struct ConnectorRecord {
//     connector: ConnectorType,
//     test_value: f32,
// }

// pub struct TestPack {
//     title: TestType,
//     results: Vec<ConnectorRecord>,
// }