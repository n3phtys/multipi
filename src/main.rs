extern crate clap;
extern crate serde;
extern crate serde_json;

use clap::{App, Arg, SubCommand};
use serde_json::{Error, Value};

fn main() {
    //TODO: required file input, optional file output, else stdout
    let matches = App::new("Multipli")
        .version("1.0")
        .author("Christopher K. <christopher.kaag@gmail.com>")
        .about("Takes json with specific placeholder config values and multiplies existing array elements by randomly choosing values based on those config values.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();

    println!("Hello, world!");
}

pub fn multipi(json: &str) -> Value {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_works() {
        let json = minimal_input();
        let should = minimal_output();
        let is = multipi(&json);
        assert_eq!(serde_json::to_string_pretty(&is).unwrap(), should);
    }

    #[test]
    fn noop_works() {
        let json = noop_input();
        let should = noop_output();
        let is = multipi(&json);
        assert_eq!(serde_json::to_string_pretty(&is).unwrap(), should);
    }

    fn minimal_input() -> String {
        unimplemented!()
    }

    fn minimal_output() -> String {
        unimplemented!()
    }

    fn noop_input() -> String {
        return "{'a':2, 'b':'hello world'}".to_string();
    }

    fn noop_output() -> String {
        let a: Value = serde_json::from_str(&noop_input()).unwrap();
        serde_json::to_string_pretty(&a).unwrap()
    }

}
