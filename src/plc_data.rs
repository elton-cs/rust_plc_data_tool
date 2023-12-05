pub enum TestType {
    GROUND,
    RESISTANCE,
    INSULATION_POS,
    INSULATION_NEG,
}

pub enum ConnectorType {
    OJ10,
    OJ9,
    RTD,
}

pub enum TestResult {
    PASS,
    FAIL
}

pub struct ConnectorRecord {
    connector: ConnectorType,
    test_value: f32,
}

pub struct TestPack {
    title: TestType,
    results: Vec<ConnectorRecord>,
}