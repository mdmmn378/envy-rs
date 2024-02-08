use assert_cmd::Command;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let mut cmd = Command::cargo_bin("envy").unwrap();
        cmd.arg("generate").arg("test.env").assert().success();
    }

    #[test]
    fn test_help() {
        let mut cmd = Command::cargo_bin("envy").unwrap();
        cmd.arg("--help").assert().success();
    }

    #[test]
    fn test_args_required_else_help() {
        let mut cmd = Command::cargo_bin("envy").unwrap();
        cmd.assert().failure();
    }
}
