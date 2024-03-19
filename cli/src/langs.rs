use std::fs::File;
use std::io;
use std::io::{Read};
use clap::{App, Arg, SubCommand, AppSettings};
use udobno_help_utils::DisplayError;
use crate::cli_app::ProcessResult;

pub trait GoRustFlagsAndCommands {
    fn go_rust_flags_and_commands(self) -> Self;
}

impl GoRustFlagsAndCommands for App<'_, '_> {
    fn go_rust_flags_and_commands(self) -> Self {
        self.arg(
            Arg::with_name("get_interfaces")
                .long("interfaces")
                .short("ifs")
                .help("get all interfaces in project/file"),
        )
            .arg(
                Arg::with_name("get_functions")
                    .long("functions")
                    .short("fn")
                    .help("get all functions in project/file"),
            )
            .arg(
                Arg::with_name("get_strings")
                    .long("strings")
                    .short("str")
                    .help("get all strings in project/file"),
            )
            .arg(
                Arg::with_name("get_libs_names")
                    .long("libs")
                    .help("get all libraries names in project/file"),
            )
            .subcommand(
                SubCommand::with_name("dbg")
                    .about("dive into debugger")
                    .arg(
                        Arg::with_name("set_config")
                            .long("config")
                            .short("cfg")
                            .value_name("CONFIG_FILE")
                            .takes_value(true)
                            .help("specify configuration file for debugger"),
                    )
                    .arg(
                        Arg::with_name("pass_arguments")
                            .long("arguments")
                            .short("arg")
                            .value_name("DEBUGGER_ARGUMENTS")
                            .takes_value(true)
                            .help("specify debugger arguments before start"),
                    )
            )
            .arg(
                Arg::with_name("path-to-file-or-dir")
                    .index(1)
                    .value_name("FILE or PROJECT_PATH")
                    .required(true),
            )

    }
}

pub fn execute_langs() -> ProcessResult {
    println!("{}", "execute_langs()");
    Ok(String::from("test"))
}
pub fn get_functions() -> ProcessResult {
    println!("{}", "get_functions()");
    Ok(String::from("test"))
}

fn open_file_path(path: String) -> Result<String, io::Error> {
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    Ok(content)
}
pub fn get_interfaces(path: String) -> ProcessResult {
    Ok(open_file_path(path)?)
}
pub fn get_libs() -> ProcessResult {
    println!("{}", "get_libs()");
    Ok(String::from("test"))
}
pub fn get_strings() -> ProcessResult {
    println!("{}", "get_strings()");
    Ok(String::from("test"))
}