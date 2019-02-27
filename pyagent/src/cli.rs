use crate::configuration::{Configuration, ConfigurationBuilder};
use crate::VERSION;
use clap::{App, Arg};

pub fn get_builder() -> ConfigurationBuilder {
    let matches = App::new("rollbar-agent")
                          .version(VERSION)
                          .author("rokob <andrew@rollbar.com>")
                          .about("Does agent things")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .default_value("rollbar-agent.conf")
                               .help("Path to configuration file."))
                          .arg(Arg::with_name("dry_run")
                               .long("dry_run")
                               .help("Dry run: processes log files, but does not save state or submit events to Rollbar. Exits after processing once."))
                          .arg(Arg::with_name("skip_to_end")
                               .long("skip_to_end")
                               .help("Go through existing log files and save them in the state without processing them, so you do not process existing log info next run. Exits after processing once."))
                          .arg(Arg::with_name("verbose")
                               .long("verbose")
                               .short("v")
                               .conflicts_with("quiet")
                               .help("Verbose output (uses log level DEBUG)"))
                          .arg(Arg::with_name("quiet")
                               .long("quiet")
                               .short("q")
                               .conflicts_with("verbose")
                               .help("Quiet output (uses log level WARNING)"))
                          .get_matches();

    let mut builder = Configuration::builder()
        .file(matches.value_of("config").unwrap())
        .dry_run(matches.is_present("dry_run"))
        .skip_to_end(matches.is_present("skip_to_end"));

    if matches.is_present("verbose") {
        builder.verbose();
    }
    if matches.is_present("quiet") {
        builder.quiet();
    }

    builder
}
