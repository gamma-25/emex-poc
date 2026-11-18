use std::collections::HashMap;

use crate::engine::functions::FUNCTIONS;
use crate::engine::{Context, Value, globals::GLOBALS};
use crate::engine::pipeline::*;

struct State<'a> {
    ctx: &'a Context<'a>,
    vars: HashMap<String, Value> 
}

impl<'a> State<'a> {
    fn get_var(&self, name: &str) -> Option<Value> {
        if GLOBALS.contains_key(name) {
            return Some((GLOBALS[name])(self.ctx));
        }
        self.vars.get(name).cloned()
    }

    fn set_var(&mut self, name: &str, val: Value) {
        self.vars.insert(name.to_owned(), val);
    }
}

struct Invokation<'a> {
    state: &'a State<'a>,
    var: Option<(&'a str, &'a Value)>
}

impl<'a> Invokation<'a> {
    fn lookup(&self, name: &str) -> Option<Value> {
        if let Some((var_name, val)) = self.var {
            if var_name == name {
                return Some(val.clone());
            }
        }
        self.state.get_var(name)
    }
}

impl Pipeline {
    pub fn run(&self, ctx: &Context) -> Value {
        let mut state = State { ctx, vars: HashMap::new() };
        for step in &self.steps {
            let result = step.run(&state);
            state.set_var(&step.name, result);
        }
        state.get_var(&self.output_var).unwrap()
    }
}

impl Step {
    fn run(&self, state: &State) -> Value {
        let input = self.on_expr.run(&Invokation { state, var: None });
        let mut output = Vec::new();
        
        match self.kind {
            StepKind::Filter => {
                for x in input.iter() {
                    let v = self.do_expr.run(&Invokation { state, var: Some((&self.as_var, &x)) });
                    let Value::Bool(b) = v else { panic!("invalid step return type for {self:?}"); };
                    if b {
                        output.push(x);
                    }
                }
            }
        }

        Value::List(output.into())
    }
}

impl Expression {
    fn run(&self, inv: &Invokation) -> Value {
        match self {
            Expression::Literal(value) => value.clone(),
            Expression::Variable(name) => inv.lookup(name.as_str()).unwrap(),
            Expression::Call(name, args) => {
                let func = FUNCTIONS[name];
                let eval_args: Vec<Value> = args.iter().map(|x| x.run(inv)).collect();
                (func)(&inv.state.ctx, &eval_args)
            }
        }
    }
}
