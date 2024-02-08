use crate::common::SupportedFormats;
use colored::*;

fn _extract_file_format(path: &str) -> SupportedFormats {
    let file_extension = path.split('.').last().unwrap();
    match file_extension {
        "env" => SupportedFormats::ENV,
        "toml" => SupportedFormats::TOML,
        "yaml" => SupportedFormats::YAML,
        _ => SupportedFormats::INVALID,
    }
}

fn _check_file_format(path: &str) -> SupportedFormats {
    let format = _extract_file_format(path);
    if format == SupportedFormats::INVALID {
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
    return format;
}

fn _check_path_exists(path: &str) {
    if std::path::Path::new(path).exists() == false {
        println!("{} {}", "❌  Error:".red(), "File not found".bold());
        std::process::exit(1);
    }
}

pub fn validate_file(path: &str) -> SupportedFormats {
    _check_path_exists(path);
    _check_file_format(path)
}
