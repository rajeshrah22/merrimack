use clap::value_parser;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

use merrimack_common::Config;

const CLI_NAME: &str = "merrimackctl";
const SOCK_PATH: &str = "/tmp/merrimack-config";

fn main() {
    let cli = clap::Command::new(CLI_NAME)
        .author("Rahul Rajesh <rajeshrah22@gmail.com>")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Command Line Interface for merrimack")
        .subcommand_required(true)
        .subcommand(
            clap::Command::new("set")
                .about("Set break schedule")
                .alias("s")
                .arg(
                    clap::Arg::new("INTERVAL_MINUTES")
                        .help("Amount of time in between breaks in minutes")
                        .required(true)
                        .value_parser(value_parser!(i64)),
                )
                .arg(
                    clap::Arg::new("DURATION_SECONDS")
                        .help("Duration of the break in seconds")
                        .required(true)
                        .value_parser(value_parser!(i64)),
                ),
        );

    let matches = cli.get_matches();

    if let Some(sub_matches) = matches.subcommand_matches("set") {
        let interval_minutes: i64 = *sub_matches
            .get_one("INTERVAL_MINUTES")
            .expect("Interval minutes required");

        let duration_seconds: i64 = *sub_matches
            .get_one("DURATION_SECONDS")
            .expect("Duration seconds required");

        println!(
            "interval_minutes: {}, duration_seconds: {}",
            interval_minutes, duration_seconds
        );

        // now send over socket
        let mut stream = UnixStream::connect(SOCK_PATH).unwrap();
        let new_config = Config {
            interval_minutes,
            duration_seconds,
        };
        let config_json = serde_json::to_string(&new_config).unwrap();
        stream.write_all(config_json.as_bytes()).unwrap();
    }
}
