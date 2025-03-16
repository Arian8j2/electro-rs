use anyhow::Context;
use cidr_utils::{cidr::Ipv4Cidr, combiner::Ipv4CidrCombiner};
use std::{process::Command, str::FromStr};

pub fn optimize_routes(routes: &str) -> anyhow::Result<String> {
    let routes = routes.split(',').collect::<Vec<_>>();
    let mut combiner = Ipv4CidrCombiner::with_capacity(routes.len());
    for route in routes {
        let cidr_route = Ipv4Cidr::from_str(route)
            .with_context(|| format!("'{route}' is invalid cidr route"))?;
        combiner.push(cidr_route);
    }
    let result = combiner
        .iter()
        .map(|route| route.to_string())
        .collect::<Vec<_>>()
        .join(",");
    Ok(result)
}

pub fn run_command(command: &str, args: &[&str]) -> anyhow::Result<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimize_route() {
        let routes =
            "192.168.56.0/26,192.168.56.64/26,192.168.56.128/26,192.168.56.192/26,10.0.0.0/24";
        assert_eq!(
            optimize_routes(routes).unwrap(),
            "10.0.0.0/24,192.168.56.0/24"
        );
    }
}
