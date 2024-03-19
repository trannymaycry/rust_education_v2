use crate::cli_app::ProcessResult;
// use clap::{App, Arg, SubCommand};
//
// pub trait DbgSubCommands {
//     fn dbg_subcommands(self) -> Self;
// }
//
// impl DbgSubCommands for App<'_, '_> {
//     fn dbg_subcommands(self) -> Self {
//         self.about("dive into dbg")
//             .version(version)
//             .setting(AppSettings::SubcommandRequiredElseHelp)
//             .arg({
//                 let arg = Arg::with_name("config_file")
//                     .short("C")
//                     .long("config")
//                     .value_name("FILEPATH")
//                     .takes_value(true)
//                     .global(true)
//                     .help("Configuration file to use");
//                 if let Some(ref config_file) = *CONFIG_FILE {
//                     arg.default_value(config_file)
//                 } else {
//                     arg
//                 }
//             });
pub fn execute_dbg() -> ProcessResult {
    println!("{}", "execute_dbg()");
    Ok(String::from("test"))
}