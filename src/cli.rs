use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("gee")
        .version("0.1.0")
        .author("Your Name")
        .about("GitHub binary installer")
        .subcommand(
            Command::new("install")
                .about("Install a binary from a GitHub repository")
                .arg(Arg::new("REPO").required(true).index(1)),
        )
}

