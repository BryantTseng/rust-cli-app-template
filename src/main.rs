mod cli;
mod config;
mod r#const;

fn main() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    //let cli_config = config::CliConfig::init(CLI_CONFIG_PATH_DEFAULT);

    let cli = cli::Cli::new();
    cli.execute();
}
