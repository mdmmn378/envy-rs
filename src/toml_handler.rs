use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use toml::Value;

#[allow(dead_code)]
fn read_toml(path: &str) -> Result<Value> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let value: Value = toml::from_str(
        &reader
            .lines()
            .collect::<Result<Vec<String>, _>>()?
            .join("\n"),
    )
    .unwrap();
    return Ok(value);
}

#[allow(dead_code)]
fn replace_leaves(value: &mut Value) {
    match value {
        Value::Table(table) => {
            for (_, val) in table {
                replace_leaves(val);
            }
        }
        Value::Array(array) => {
            for val in array {
                replace_leaves(val);
            }
        }
        Value::String(_) => {
            *value = Value::String("string".to_string());
        }
        Value::Integer(_) => {
            *value = Value::Integer(0);
        }
        Value::Float(_) => {
            *value = Value::Float(0.0);
        }
        Value::Boolean(_) => {
            *value = Value::Boolean(false);
        }
        _ => {}
    }
}

#[allow(dead_code)]
pub fn generate_toml_file(dry_run: bool, path: &str) -> Result<String> {
    let value = read_toml(path)?;
    let mut value = value.clone();
    replace_leaves(&mut value);
    let mut file = File::create("example.toml")?;
    if dry_run {
        println!("{}", value);
        return Ok(("").to_string());
    }
    file.write_all(toml::to_string(&value)?.as_bytes())?;

    Ok("example.toml".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_toml() {
        read_toml("tests/fixtures/test.toml").unwrap();
    }

    #[test]
    fn test_generate_toml_file() {
        generate_toml_file(false, "tests/fixtures/test.toml").unwrap();
    }
}
