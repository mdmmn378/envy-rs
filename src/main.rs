mod common;
mod dotenv_handler;
mod toml_handler;
mod utils;

use clap::{ArgAction, Command};
use colored::*;
use common::SupportedFormats;
use dotenv_handler::generate_dotenv_file;
use toml_handler::generate_toml_file;
use utils::validate_file;

use anyhow::Result;

fn generate(val: &clap::ArgMatches) -> Result<()> {
    let path = val.get_one::<String>("path").unwrap();
    let file_format = validate_file(&path);
    let dry_run: bool = val.get_flag("dry-run");
    let output_name: String;
    match file_format {
        SupportedFormats::ENV => {
            output_name = generate_dotenv_file(dry_run, &path)?;
        }
        SupportedFormats::TOML => {
            output_name = generate_toml_file(dry_run, &path)?;
        }
        SupportedFormats::YAML => {
            println!(
                "{} {}",
                "❌  Error:".red(),
                "YAML file format is not supported yet".bold()
            );
            std::process::exit(1);
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
            format!("{} file", output_name).bold()
        );
    }
    Ok(())
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
                .help("Path to the config file")
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
        Some(("generate", val)) => {
            if let Err(e) = generate(val) {
                println!("{} {}", "❌  Error:".red(), e);
                std::process::exit(1);
            }
        }
        _ => println!("No subcommand"),
    }
}
