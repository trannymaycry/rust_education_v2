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


pub trait GoSubCommands {
    fn go_subcommands(self) -> Self;
}

impl GoSubCommands for App<'_, '_> {
    fn go_subcommands(self) -> Self {
        self.subcommand(SubCommand::with_name("go")
            .about("interaction with golang")
            .go_rust_flags_and_commands()
        )
    }
}