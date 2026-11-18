use crate::engine::Value;

pub mod schema;
pub mod runtime;

#[derive(Debug)]
pub struct Pipeline {
    pub steps: Vec<Step>,
    pub output_var: String
}

#[derive(Debug)]
pub struct Step {
    pub name: String,
    pub kind: StepKind,
    pub on_expr: Expression,
    pub as_var: String,
    pub do_expr: Expression
}

#[derive(Debug)]
pub enum StepKind {
    Filter
}

#[derive(Debug)]
pub enum Expression {
    Literal(Value),
    Variable(String),
    Call(String, Vec<Expression>)
}

