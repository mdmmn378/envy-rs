use indexmap::IndexMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[allow(dead_code)]
fn read_dot_env(path: &str) -> Result<IndexMap<String, String>, Error> {
    let mut env = IndexMap::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let mut split = line.split('=');
        let key = split.next().unwrap();
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

#[allow(dead_code)]
fn generate_dot_env_string(env: IndexMap<String, String>) -> String {
    let mut env_string = String::new();
    for (key, val) in env {
        let val_type = get_type(&val);
        env_string.push_str(&format!("{}={}\n", key, val_type));
    }
    env_string = env_string.strip_suffix("\n").unwrap().to_string();
    env_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_env() {
        let env = read_dot_env(".env.example").unwrap();
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
    fn test_generate_dot_env_string() {
        let mut env = IndexMap::new();
        env.insert("HELLO".to_string(), "ADELE".to_string());
        env.insert("WORLD".to_string(), "21".to_string());
        let env_string = generate_dot_env_string(env);
        assert_eq!(env_string, "HELLO=string\nWORLD=int");
    }
}
