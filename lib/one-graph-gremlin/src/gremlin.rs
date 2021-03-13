pub enum Step {
    AddV(String),
    V(Option<GValue>),
    Has(String, Predicate),
    AddE(String),
    As(String),
    From(String),
    Empty,
}

#[derive(Debug, PartialEq)]
pub enum GValue {
    Integer(i64),
    String(String),
    Boolean(bool),
}
#[derive(Debug, PartialEq)]
pub struct GList {
    pub values: Vec<GValue>,
}

#[derive(Debug, PartialEq)]
pub enum Predicate {
    Value(GValue),
    Within(GList),
}

pub struct Gremlin {
    pub request_id: String,
    pub steps: Vec<Step>,
}