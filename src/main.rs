mod io_manager;

use clap::{ArgAction, Command};

fn generate(val: &clap::ArgMatches) {
    let path = val.get_one::<String>("path").unwrap();
    println!("Path: {}", path);
}

fn main() {
    let mut cmd = Command::new("envy").subcommand_required(true);
    let sub_generate = Command::new("generate")
        .about("Generate a new .env file")
        .arg(
            clap::Arg::new("path")
                .help("Path to the .env file to generate")
                .required(true)
                .action(ArgAction::Set),
        );
    cmd = cmd.subcommand(sub_generate);
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("generate", val)) => generate(val),
        _ => println!("No subcommand"),
    }
}
