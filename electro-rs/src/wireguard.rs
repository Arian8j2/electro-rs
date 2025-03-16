use anyhow::Context;
use std::process::Command;

pub fn is_interface_up(if_name: &str) -> anyhow::Result<bool> {
    let output = run_command("ip", &["-br", "a"])?;
    let result = output
        .lines()
        .map(|line| line.split(' ').next().unwrap())
        .any(|name| name == if_name);
    Ok(result)
}

pub fn connect(config_path: &str) -> anyhow::Result<()> {
    // if path doesn't exit with .conf it will look up /etc/wireguard
    assert!(config_path.ends_with(".conf"));
    run_command("wg-quick", &["up", config_path]).map(|_| ())
}

pub fn disconnect(config_path: &str) -> anyhow::Result<()> {
    assert!(config_path.ends_with(".conf"));
    run_command("wg-quick", &["down", config_path]).map(|_| ())
}

fn run_command(command: &str, args: &[&str]) -> anyhow::Result<String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .with_context(|| format!("couldn't spawn `{command}` command"))?;
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr)?;
        let error = anyhow::anyhow!("{stderr}").context(format!(
            "command `{command}` with args `{}` exited with error",
            args.join(" ")
        ));
        return Err(error);
    }
    Ok(String::from_utf8(output.stdout)?)
}
