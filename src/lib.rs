extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "sql.pest"]
pub struct SqlParser;

#[cfg(test)]
mod test;
