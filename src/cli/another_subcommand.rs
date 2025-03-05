use clap::{Arg, ArgMatches, Command};
use tracing::info;

use crate::{config, r#const::*};

pub struct CliReadConfig {}

impl CliReadConfig {
    pub fn command() -> Command {
        Command::new(CLI_CMD_READ_CONFIG)
            .about("Read the config file and print it out.")
            .arg(
                Arg::new(CLI_ARG_PATH)
                    .short(CLI_ARG_PATH_SHORT)
                    .help(CLI_ARG_PATH_HELP)
                    .default_value(CLI_CONFIG_PATH_DEFAULT)
                    .required(false),
            )
    }
    pub fn execute(arg_matches: &ArgMatches) {
        let config_path = arg_matches
            .get_one::<String>(CLI_ARG_PATH)
            .expect(format!("Can't get ARG `{}`", CLI_ARG_MSG).as_str());

        let cli_config = config::CliConfig::init(config_path);

        info!("Current debug mode? {}", cli_config.global.debug);
    }
}
