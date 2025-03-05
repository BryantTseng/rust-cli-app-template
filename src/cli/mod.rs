pub mod subcommand;

use clap::{Command, crate_name, crate_version};

use crate::r#const::{CLI_ABOUT_MSG, CLI_CMD_ECHO};

use subcommand::CliEcho;
pub struct Cli {
    command: Command,
}

impl Cli {
    pub fn new() -> Self {
        let command = Command::new(crate_name!())
            .version(crate_version!())
            .subcommand(CliEcho::command())
            .about(CLI_ABOUT_MSG);

        Self { command }
    }
    pub fn execute(self) {
        let matches = self.command.get_matches();

        if let Some((subcommand, arg_matches)) = matches.subcommand() {
            match subcommand {
                CLI_CMD_ECHO => {
                    CliEcho::execute(arg_matches);
                }
                _ => {}
            }
        }
    }
}
