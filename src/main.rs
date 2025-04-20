pub mod commands;
pub mod detect;
pub mod recode;

use {
    clap::{Command, parser::ValuesRef},
    commands::{get_args_domain, get_command_domain},
    // detect::*,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let dgef = Command::new("dgef")
        .about("Ultimate encoding tool")
        .version(VERSION)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Michał Szmidt")
        .subcommand(get_command_domain().args(get_args_domain()))
        .get_matches();

    if let Some(("enoding", query_matches)) = dgef.subcommand() {
        let mut out = "./out.txt".to_string();
        let mut input = "./in.txt".to_string();
        let mut clrf = "yes".to_string();
        let mut outenc = "utf-8".to_string();

        if let Some(value_of_out) = query_matches.get_many::<String>("out") {
            out = get_param(value_of_out);
        }

        if let Some(value_of_in) = query_matches.get_many::<String>("input") {
            input = get_param(value_of_in);
        }

        if let Some(value_of_in) = query_matches.get_many::<String>("lf") {
            clrf = get_param(value_of_in);
        }

        if let Some(value_of_in) = query_matches.get_many::<String>("encoding") {
            outenc = get_param(value_of_in);
        }

        let clrf_b = match clrf.as_str() {
            "yes" => true,
            "no" => false,
            _ => return,
        };

        recode::convert_file_encoding(input.as_str(), out.as_str(), outenc.as_str(), clrf_b)
            .unwrap();
    }
}

fn get_param(valuesref: ValuesRef<String>) -> String {
    let x = valuesref.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
    return x;
}
