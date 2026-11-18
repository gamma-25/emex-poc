use std::fs::File;

use exe::VecPE;

use crate::engine::{Context, Pipeline};

pub mod engine;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("{} [pipeline] [input]", args[0]);
    }
    let pipeline_filename = &args[1];
    let input_filename = &args[2];

    let pipeline_yaml: serde_yaml_ng::Value = serde_yaml_ng::from_reader(
        File::open(pipeline_filename).unwrap()
    ).unwrap();

    let pipeline = Pipeline::from(pipeline_yaml.as_mapping().unwrap());

    let input_file = VecPE::from_disk_file(input_filename).unwrap();
    let context = Context { file: &input_file };

    let output = pipeline.run(&context);
    println!("{output}");
}
