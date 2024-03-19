// parser tools here
use {
    clap::{App, SubCommand,Arg},
    crate::langs::*,
};
// const DEBUG_PARSER_MSG: &str = "[DEBUG][src/go/parser.rs] ";
// fn dbg_msg(msg: &str) {
//     println!("{}{} ", DEBUG_PARSER_MSG, msg)
// }
// pub fn get_interfaces() {
//     dbg_msg("get list of interfaces (entry fn get_interfaces)");
// }
// pub fn get_functions() {}
// pub fn get_strings() {}
// pub fn get_libs() {}


pub trait RustSubCommands {
    fn rust_subcommands(self) -> Self;
}

impl RustSubCommands for App<'_, '_> {
    fn rust_subcommands(self) -> Self {
        self.subcommand(SubCommand::with_name("rust")
            .about("interaction with rust code")
            .go_rust_flags_and_commands())
    }
}