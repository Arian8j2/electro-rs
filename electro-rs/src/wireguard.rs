use super::utils;

pub fn is_interface_up(if_name: &str) -> anyhow::Result<bool> {
    let output = utils::run_command("ip", &["-br", "a"])?;
    let result = output
        .lines()
        .map(|line| line.split(' ').next().unwrap())
        .any(|name| name == if_name);
    Ok(result)
}

pub fn connect(config_path: &str) -> anyhow::Result<()> {
    // if path doesn't exit with .conf it will look up /etc/wireguard
    assert!(config_path.ends_with(".conf"));
    utils::run_command("sudo", &["wg-quick", "up", config_path]).map(|_| ())
}

pub fn disconnect(config_path: &str) -> anyhow::Result<()> {
    assert!(config_path.ends_with(".conf"));
    utils::run_command("sudo", &["wg-quick", "down", config_path]).map(|_| ())
}
