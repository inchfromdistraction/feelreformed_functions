use shopify_function::prelude::*;
use std::process;

pub mod checkout_validations_generate_run;

#[typegen("schema.graphql")]
pub mod schema {
    #[query("src/checkout_validations_generate_run.graphql")]
    pub mod checkout_validations_generate_run {}
}

fn main() {
    eprintln!("Please invoke a named export.");
    process::exit(1);
}