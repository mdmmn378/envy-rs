use indexmap::IndexMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error};

use anyhow::Result;

#[allow(dead_code)]
fn read_dotenv(path: &str) -> Result<IndexMap<String, String>, Error> {
    let mut env = IndexMap::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let line = remove_comments(&line);
        if line.is_empty() {
            continue;
        }
        let mut split = line.split('=');
        let key = split.next().unwrap().trim();
        let val = split.next().unwrap();
        env.insert(key.to_string(), clean_text(val));
    }
    Ok(env)
}

fn clean_text(text: &str) -> String {
    let mut text = text.to_string();
    text = text.replace("\"", "");
    text = text.replace("'", "");
    text
}

#[allow(dead_code)]
fn get_type(value: &str) -> &str {
    let value = clean_text(value).to_string().to_lowercase();

    if value.parse::<i32>().is_ok() {
        "int"
    } else if value.parse::<bool>().is_ok() {
        "bool"
    } else if value.parse::<f64>().is_ok() {
        "float"
    } else {
        "string"
    }
}

fn remove_comments(text: &str) -> String {
    let mut text = text.to_string();
    // From: # This is a comment, To: ""
    let hash_index = text.find('#').unwrap_or(text.len());
    let last_index = text.len();
    text.replace_range(hash_index..last_index, "");
    text = text.trim_end().to_string();
    text
}

#[allow(dead_code)]
fn generate_dotenv_string(env: IndexMap<String, String>) -> Result<String> {
    let mut env_string = String::new();
    for (key, val) in env {
        let val_type = get_type(&val);
        env_string.push_str(&format!("{}={}\n", key, val_type));
    }
    env_string = env_string.strip_suffix("\n").unwrap().to_string();
    Ok(env_string)
}

pub fn generate_dotenv_file(dry_run: bool, path: &str) -> Result<()> {
    let env = read_dotenv(path)?;
    let mut env_string = generate_dotenv_string(env)?;
    env_string.push_str("\n");
    let mut file = File::create(".env.example")?;
    if dry_run {
        println!("{}", env_string.strip_suffix("\n").unwrap());
        return Ok(());
    }
    file.write_all(env_string.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_env() {
        let env = read_dotenv("test.env").unwrap();
        assert_eq!(env.get("HELLO").unwrap(), "ADELE");
        assert_eq!(env.get("TAYLOR").unwrap(), "SWIFT");
    }

    #[test]
    fn test_get_type() {
        assert_eq!(get_type("1"), "int");
        assert_eq!(get_type("true"), "bool");
        assert_eq!(get_type("True"), "bool");
        assert_eq!(get_type("Frue"), "string");
        assert_eq!(get_type("False"), "bool");
        assert_eq!(get_type("1.0"), "float");
        assert_eq!(get_type("hello"), "string");
        assert_eq!(get_type("1hello"), "string");
    }

    #[test]
    fn test_generate_dotenv_string() {
        let mut env = IndexMap::new();
        env.insert("HELLO".to_string(), "ADELE".to_string());
        env.insert("WORLD".to_string(), "21".to_string());
        let env_string = generate_dotenv_string(env).unwrap();
        assert_eq!(env_string, "HELLO=string\nWORLD=int");
    }

    #[test]
    fn test_remove_comments() {
        let text = r#"# This is a comment\n"#;
        let text = remove_comments(text);
        assert_eq!(text, "");
        let text = r#"HELLO=WORLD # This is a comment\n"#;
        let text = remove_comments(text);
        assert_eq!(text, "HELLO=WORLD");
        let text = r#"#HELLO=WORLD # This is a comment\n"#;
        let text = remove_comments(text);
        assert_eq!(text, "");
    }

    #[test]
    fn test_generate_dotenv_file() {
        generate_dotenv_file(false, "test.env").unwrap();
        let mut file = File::open(".env.example").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(
            contents,
            "HELLO=string\nTAYLOR=string\nAGE=int\nSCORE=float\nACTIVE=bool\n"
        );
    }
}
