use clap::{App, Arg, SubCommand, AppSettings};
use crate::cli_app::ProcessResult;
pub trait DappSubCommands {
    fn dapp_subcommands(self) -> Self;
}

impl DappSubCommands for App<'_, '_> {
    fn dapp_subcommands(self) -> Self {
        self.about("interact with decentralized app")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(SubCommand::with_name("dapp")
                .about("interaction with your decentralized app")
                .arg(
                    Arg::with_name("project-name")
                        .index(1)
                        .value_name("PROJECT_NAME")
                        .required(true),
                )
                .arg(Arg::with_name("reset-project")
                    .long("reset")
                    .short("r")
                    .help("reset inspecting project to start point (deploy and initialize)")
                )
            )
    }
}

pub fn execute_dapp() -> ProcessResult {
    println!("{}", "execute_dapp()");
    Ok(String::from("test"))
}


