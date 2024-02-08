#[derive(Debug, PartialEq)]
pub enum SupportedFormats {
    ENV,
    TOML,
    YAML,
    INVALID,
}

impl SupportedFormats {
    pub fn to_string(&self) -> String {
        match self {
            SupportedFormats::ENV => "env",
            SupportedFormats::TOML => "toml",
            SupportedFormats::YAML => "yaml",
            _ => "invalid",
        }
        .to_string()
    }

    pub fn supported_formats() -> Vec<String> {
        return vec![
            SupportedFormats::ENV.to_string(),
            SupportedFormats::TOML.to_string(),
            SupportedFormats::YAML.to_string(),
        ];
    }
}
