mod common;
mod dotenv_handler;
mod utils;

use clap::{ArgAction, Command};
use colored::*;
use common::SupportedFormats;
use dotenv_handler::generate_dotenv_file;
use utils::validate_file;

fn generate(val: &clap::ArgMatches) {
    let path = val.get_one::<String>("path").unwrap();
    let format = validate_file(&path);
    let dry_run: bool = val.get_flag("dry-run");
    match format {
        SupportedFormats::ENV => {
            let _ = generate_dotenv_file(dry_run, &path);
        }
        _ => {
            println!(
                "{} {}",
                "❌  Error:".red(),
                "Unsupported file format".bold()
            );
            println!(
                "{} {}",
                "Supported formats:".bold(),
                SupportedFormats::supported_formats().join(", ").bold()
            );
            std::process::exit(1);
        }
    }
    if !dry_run {
        println!(
            "{} {}",
            "✨  Successfully generated".green(),
            ".env.example".bold()
        );
    }
}

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let mut cmd = Command::new("envy")
        .version(version)
        .arg_required_else_help(true);
    let sub_generate = Command::new("generate")
        .about("Generate a new masked config file")
        .arg(
            clap::Arg::new("path")
                .help("Path to the .env file to generate")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            clap::Arg::new("dry-run")
                .long("dry-run")
                .action(ArgAction::SetTrue),
        );
    cmd = cmd.subcommand(sub_generate);
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("generate", val)) => generate(val),
        _ => println!("No subcommand"),
    }
}
