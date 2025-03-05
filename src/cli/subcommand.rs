use clap::{Arg, ArgMatches, Command};
use tracing::info;

use crate::r#const::*;

pub struct CliEcho {}

impl CliEcho {
    pub fn command() -> Command {
        Command::new(CLI_CMD_ECHO).about("Echo").arg(
            Arg::new(CLI_ARG_MSG)
                .help(CLI_ARG_MSG_HELP)
                .default_value(CLI_ARG_MSG_DEFAULT)
                .required(true),
        )
    }
    pub fn execute(arg_matches: &ArgMatches) {
        let message_to_echo = arg_matches
            .get_one::<String>(CLI_ARG_MSG)
            .expect(format!("Can't get ARG `{}`", CLI_ARG_MSG).as_str());

        info!(message_to_echo);
    }
}
