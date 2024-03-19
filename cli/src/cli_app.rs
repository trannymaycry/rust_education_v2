use std::{error, io};
use std::fmt::Display;
use clap::{ArgMatches, SubCommand};
// use clap::Format::Error;
use {
    clap::{App, AppSettings, Arg},
    crate::{langs::*, dapp::*, dbg::*}
};
use udobno_help_utils::DisplayError;
use udobno_help_utils::errors::ProgramError;
// use udobno_help_utils::DisplayError;
use crate::go::GoSubCommands;
use crate::rust::RustSubCommands;
use crate::langs::*;
use crate::execute_help;

const GO_PROGRAM: &str = "go";
const RUST_PROGRAM: &str = "rust";
const GET_FUNCTIONS_FLAG: &str = "get_functions";
const DAPP_PROGRAM: &str = "dapp";
const GET_LIBS_FLAG: &str = "get_libs";
const GET_STRINGS_FLAG: &str = "get_strings";
const GET_INTERFACES_FLAG: &str = "get_interfaces";
const GET_DAPP_RESET_FLAG: &str = "reset";
const PATH: &str = "path-to-file-or-dir";

#[derive(Debug)]
pub struct DbgConf {}

// pub struct CliConfig {
//
// }
#[derive(Debug)]
pub enum CliCommand {
    Language {
        get_functions: bool,
        get_interfaces: bool,
        get_libs: bool,
        get_strings: bool,
        path: String,
    },
    Debugger {
        load_conf: DbgConf,
        args: Vec<String>,
        path: String,
    },
    Dapp {
        reset: bool,
        path: String,
    },
    Help {
        message: String,
    }
}

// #[derive(Debug)]
// pub struct CliConfig {
//     pub command: String,
//     pub use_flags: bool,
//     pub action: String,
//     pub usage: String,
//     // pub path: String,
// }

// impl Default for CliConfig {
//     fn default() -> CliConfig {
//         CliConfig {
//             command: String::from("Type `udobno <SUBCOMMAND> --help` for usage"),
//             use_flags: false,
//             action: String::from("get_help"),
//             usage: String::from("Type `udobno <SUBCOMMAND> --help` for usage"),
//         }
//     }
// }

impl Default for CliCommand {
    fn default() -> CliCommand {
        CliCommand::Help {
            message: String::from("Woow, you need some help :)")
        }
    }
}

pub fn get_cli_clap_app<'ab, 'v>(name: &str, about: &'ab str, version: &'v str) -> App<'ab, 'v> {
    App::new(name)
        .about(about)
        .version(version)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .rust_subcommands()
        .go_subcommands()
        .dapp_subcommands()
}

pub type ProcessResult = Result<String, Box<dyn std::error::Error>>;

pub fn execute_command(command: &CliCommand) -> ProcessResult {
    match &command {
        CliCommand::Language {
            get_functions: true,
            get_interfaces,
            get_libs,
            get_strings,
            path
        } => get_functions(),
        CliCommand::Language {
            get_functions,
            get_interfaces: true,
            get_libs,
            get_strings,
            path,
        } => get_interfaces(path.to_string()),
        CliCommand::Language {
            get_functions,
            get_interfaces,
            get_libs: true,
            get_strings,
            path,
        } => get_libs(),
        CliCommand::Language {
            get_functions,
            get_interfaces,
            get_libs,
            get_strings: true,
            path,
        } => get_strings(),
        CliCommand::Dapp {
            reset: true,
            path
        } => execute_dapp(),
        _ => execute_help(String::from("help")),
    }
}


// pub fn configure_command(config: &CliConfig) -> CliCommand {
//     // let go_command = String::from("go");
//     let usage = &config.usage;
//     match config.command {
//         GO_PROGRAM | RUST_PROGRAM => {
//             if config.use_flags {
//                 match config.action.as_str() {
//                     GET_FUNCTIONS_FLAG =>
//                         CliCommand::Language {
//                         get_functions: true,
//                         get_interfaces: false,
//                         get_libs: false,
//                         get_strings: false,
//                     },
//                     GET_INTERFACES_FLAG =>
//                         CliCommand::Language {
//                         get_functions: false,
//                         get_interfaces: true,
//                         get_libs: false,
//                         get_strings: false,
//                     },
//                     GET_LIBS_FLAG =>
//                         CliCommand::Language {
//                             get_functions: false,
//                             get_interfaces: false,
//                             get_libs: true,
//                             get_strings: false,
//                         },
//                     GET_STRINGS_FLAG => CliCommand::Language {
//                             get_functions: false,
//                             get_interfaces: false,
//                             get_libs: false,
//                             get_strings: true,
//                         },
//                     _ => CliCommand::Help {
//                         message: usage.to_string()
//                     }
//                 }
//             } else {
//                 CliCommand::Help {
//                     message: usage.to_string()
//                 }
//             };
//             CliCommand::Language {
//                 get_functions: false,
//                 get_interfaces: false,
//                 get_libs: false,
//                 get_strings: false,
//             }
//         },
//         DAPP_PROGRAM => {
//             if config.use_flags {
//                 match config.action.as_str() {
//                     GET_DAPP_RESET_FLAG =>
//                         CliCommand::Dapp {
//                             reset: true,
//                         },
//                     _ => CliCommand::Help {
//                         message: usage.to_string(),
//                     }
//                 }
//             } else {
//                 CliCommand::Help {
//                     message: usage.to_string(),
//                 }
//             }
//         },
//         _ => CliCommand::default(),
//     }
// }

pub fn config_command_get_functions(path: String) -> CliCommand {
    CliCommand::Language {
        get_functions: true,
        get_interfaces: false,
        get_libs: false,
        get_strings: false,
        path,
    }
}

pub fn config_command_get_interfaces(path: String) -> CliCommand  {
    CliCommand::Language {
        get_functions: false,
        get_interfaces: true,
        get_libs: false,
        get_strings: false,
        path,
    }
}

pub fn config_command_get_libs(path: String) -> CliCommand  {
    CliCommand::Language {
        get_functions: false,
        get_interfaces: false,
        get_libs: true,
        get_strings: false,
        path,
    }
}

pub fn config_command_get_strings(path: String) -> CliCommand  {
    CliCommand::Language {
        get_functions: false,
        get_interfaces: false,
        get_libs: false,
        get_strings: true,
        path,
    }
}

pub fn config_command_dapp_reset(path: String) -> CliCommand  {
    CliCommand::Dapp {
        reset: true,
        path,
    }
}

pub fn configure_command(matches: &ArgMatches) -> Result<CliCommand, Box<dyn error::Error>> {
    println!("{}\n","pub fn parse_command(matches: ArgMatches)");
    // match matches.subcommand() {
    //     ("go", Some(matches)) => {
    //         if matches.is_present("get_strings") {
    //             println!("{}", "GET STRINGS")
    //         }
    //     }
    //     _ => ()
    // }
    // let command_name = matches.subcommand_name().ok_or(ProgramError::OhError)?;
    matches.subcommand();
    match matches.subcommand() {
        (GO_PROGRAM | RUST_PROGRAM, Some(matches)) => {
            let path = matches.value_of(PATH).unwrap();
            if matches.is_present(GET_FUNCTIONS_FLAG) {
                Ok(config_command_get_functions(path.to_string()))
            } else if matches.is_present(GET_INTERFACES_FLAG) {
                Ok(config_command_get_interfaces(path.to_string()))
            } else if matches.is_present(GET_LIBS_FLAG) {
                Ok(config_command_get_libs(path.to_string()))
            } else if matches.is_present(GET_STRINGS_FLAG) {
                Ok(config_command_get_strings(path.to_string()))
            } else {
                Ok(CliCommand::default())
            }
        },
        // Some(RUST_PROGRAM) => Ok(CliConfig {
        //     command: String::from(RUST_PROGRAM),
        //     use_flags: false,
        //     action: String::from(""),
        // }),
        (DAPP_PROGRAM , Some(matches)) => {
            let project_name = matches.value_of(PATH).unwrap();
            if matches.is_present(GET_DAPP_RESET_FLAG) {
                Ok(config_command_dapp_reset(project_name.to_string()))
            } else {
                Ok(CliCommand::default())
            }
        },
        _ => Ok(CliCommand::default())
    }
}