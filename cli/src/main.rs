// aim this project is simpling research code by Go and Rust
use {
    std::{error,
          env::Args},
    clap::{crate_name, crate_description, crate_version, App, AppSettings, SubCommand, Arg, ArgMatches},
    udobno::{
        cli_app::get_cli_clap_app
    },
    udobno_help_utils::DisplayError,
    udobno::cli_app,
};
use udobno::cli_app::{execute_command, configure_command};
// use udobno::cli_app;
// use std::collections::HashMap;

// create cli app
// fn slut_main() {
//     let matches = App::new("udobno")
//         .version("0.0")
//         .author("slut")
//         .about("Start research Go and Rust code")
//         .setting(AppSettings::ArgRequiredElseHelp)
//         .subcommand(SubCommand::with_name("go")
//             .usage("udobno go [path_or_file] [args]")
//             .about("Go module")
//             .arg(Arg::with_name("ifces")
//                 .help("output all interface implementations"))
//             .arg(Arg::with_name("func")
//                 .help("output all functions"))
//             .arg(Arg::with_name("str")
//                 .help("output all strings"))
//             .subcommand(SubCommand::with_name("dbg")
//                 .usage("udobno go dbg [file] [args]")
//                 .about("dive into debugger")
//                 .arg(Arg::with_name("config")
//                     .help("set config dbg file")
//                     .short("-c"))
//                 .arg(Arg::with_name("argv")
//                     .short("-e")
//                     .help("pass arguments for dbg")))
//         )
//         .subcommand(SubCommand::with_name("rust")
//             .usage("udobno rust")
//             .about("Rust module"))
//         .subcommand(SubCommand::with_name("gru")
//             .usage("udobno gru")
//             .about("pentest module"))
//         .subcommand(SubCommand::with_name("dapp")
//             .usage("udobno dapp [args]")
//             .arg(Arg::with_name("reset")
//                 .help("reset dApp to genesis state")
//                 .short("r"))
//             .subcommand(SubCommand::with_name("script")
//                 .usage("udobno dapp script [script_name] [args]")
//                 .about("run custom script (deploy contracts, start dApp etc.)")
//                 .arg(Arg::with_name("path").short("p")
//                     .help("specify script path"))
//             )
//             .about("dApp interact module"))
//         .get_matches();
//
//     let test = matches.subcommand();
//     match matches.subcommand_name() {
//         Some("go") => {
//             get_interfaces();
//         }
//         None => println!("Something went wrong"),
//         _ => println!("Unsupported module")
//     }
// }

fn main() -> Result<(), Box<dyn error::Error>> {
    let matches = get_cli_clap_app(
        crate_name!(),
        crate_description!(),
        crate_version!(),
    )
        .get_matches();
    // println!("{:?}", matches);
    // Ok(())
    do_main(&matches).map_err(|err| DisplayError::new_as_boxed(err).into())
}

fn do_main(matches: &ArgMatches<'_>) -> Result<(), Box<dyn error::Error>> {
    // if parse_settings(matches)? {
    //     let mut wallet_manager = None;

    // let mut config = configure_command(matches)?;
        // config.signers = signers.iter().map(|s| s.as_ref()).collect();
    let command = configure_command(&matches)?;
    if let result = execute_command(&command)? {
        println!("{result}");
    };
    Ok(())
}