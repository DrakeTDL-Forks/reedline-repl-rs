extern crate reedline_repl_rs;

use reedline_repl_rs::{Command, Parameter, Result, Value};
use reedline_repl_rs::{Convert, Repl};
use std::collections::HashMap;

/// Example using Repl without Context (or, more precisely, a Context of ())

// Add two numbers. Have to make this generic to be able to pass a Context of type ()
fn add<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
    let first: i32 = args["first"].convert()?;
    let second: i32 = args["second"].convert()?;

    Ok(Some((first + second).to_string()))
}

// Write "Hello"
fn hello<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
    Ok(Some(format!("Hello, {}", args["who"])))
}

fn main() -> Result<()> {
    let mut repl = Repl::new(())
        .with_name("MyApp")
        .with_version("v0.1.0")
        .with_description("My very cool app")
        .add_command(
            Command::new("add", add)
                .with_parameter(Parameter::new("first").set_required(true)?)?
                .with_parameter(Parameter::new("second").set_required(true)?)?
                .with_help("Add two numbers together"),
        )
        .add_command(
            Command::new("hello", hello)
                .with_parameter(Parameter::new("who").set_required(true)?)?
                .with_help("Greetings!"),
        );
    repl.run()
}
