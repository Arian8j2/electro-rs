use anyhow::ensure;
use clap::Parser;
use electro_rs::Server;
use indicatif::ProgressBar;
use std::time::Duration;

#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, clap::Subcommand)]
enum Command {
    /// Turn on electro
    On {
        /// Use this specific server instead of prompting
        #[arg(short, long)]
        server: Option<String>,
    },
    /// Turn off electro
    Off,
    /// List current available electro servers
    List,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let is_on = electro_rs::is_on()?;
    match args.command {
        Command::On { server } => {
            ensure!(!is_on, "electro is already on");
            turn_on(server)?;
        }
        Command::Off => {
            ensure!(is_on, "electro is not on");
            turn_off()?;
        }
        Command::List => {
            let servers = fetch_server_with_loading()?
                .into_iter()
                .map(|server| server.name)
                .collect::<Vec<_>>();
            println!("{}", servers.join("\n"));
        }
    }
    Ok(())
}

fn turn_on(server: Option<String>) -> anyhow::Result<()> {
    let servers = fetch_server_with_loading()?;
    let selected_server = match server {
        Some(server_name) => servers
            .into_iter()
            .find(|server| server.name == server_name)
            .ok_or_else(|| anyhow::anyhow!("couldn't find server with name '{server_name}'"))?,
        None => {
            let servers_len = servers.len();
            inquire::Select::new("Select electro server", servers)
                .with_page_size(servers_len)
                .prompt()?
        }
    };
    let spinner = create_spinner("Connecting");
    electro_rs::connect(&selected_server)?;
    spinner.finish_with_message(console::style("Connected").green().bold().to_string());
    Ok(())
}

fn turn_off() -> anyhow::Result<()> {
    let spinner = create_spinner("Disconnecting");
    electro_rs::disconnect()?;
    spinner.finish_with_message(console::style("Disconnected").green().bold().to_string());
    Ok(())
}

fn fetch_server_with_loading() -> anyhow::Result<Vec<Server>> {
    let progress = create_spinner("Fetching servers");
    let servers = electro_rs::fetch_servers()?;
    progress.finish_and_clear();
    Ok(servers)
}

fn create_spinner(message: &str) -> ProgressBar {
    let colored_message = console::style(message).black().bright().to_string();
    let progress = ProgressBar::new_spinner().with_message(colored_message);
    progress.enable_steady_tick(Duration::from_millis(100));
    progress
}
