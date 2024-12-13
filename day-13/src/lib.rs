use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "syntax.pest"]
pub struct InputParser;