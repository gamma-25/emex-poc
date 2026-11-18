use serde_yaml_ng::{
    Value as YamlValue, 
    Mapping as YamlMapping
};

use crate::engine::pipeline::*;

impl From<&YamlMapping> for Pipeline {
    fn from(value: &YamlMapping) -> Self {
        Pipeline {
            steps: value["steps"].as_sequence().unwrap().iter()
                .map(|v| Step::from(v.as_mapping().unwrap()))
                .collect(),
            output_var: value["output"].as_str().unwrap().to_owned()
        }
    }
}

impl From<&YamlMapping> for Step {
    fn from(value: &YamlMapping) -> Self {
        Step { 
            name: value["name"].as_str().unwrap().to_owned(), 
            kind: StepKind::from(value["kind"].as_str().unwrap()),
            on_expr: Expression::from(&value["on"]),
            as_var: value["as"].as_str().unwrap().to_owned(),
            do_expr: Expression::from(&value["do"])
        }
    }
}

impl From<&str> for StepKind {
    fn from(value: &str) -> Self {
        match value {
            "filter" => StepKind::Filter,
            _ => panic!("unknown kind {value}")
        }
    }
}

impl From<&YamlValue> for Expression {
    fn from(value: &YamlValue) -> Self {
        match value {
            YamlValue::Bool(t) => Expression::Literal(Value::Bool(*t)),
            YamlValue::Number(t) => Expression::Literal(Value::Address(t.as_u64().unwrap())),
            YamlValue::String(t) => {
                if !t.starts_with('#') {
                    Expression::Variable(t.to_owned())
                } else {
                    let s = &t[1..];
                    Expression::Literal(Value::Bytes(s.as_bytes().into()))
                }
            },
            YamlValue::Mapping(t) => {
                if t.len() != 1 { panic!("expr ambiguous {value:?}") }
                Expression::Call(
                    t.keys().next().unwrap().as_str().unwrap().to_owned(),
                    t.values().next().unwrap().as_sequence().unwrap().iter()
                        .map(|v| Expression::from(v))
                        .collect()
                )
            },
            _ => panic!("unable to parse expr {value:?}")
        }
    }
}
