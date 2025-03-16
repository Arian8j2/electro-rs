use anyhow::ensure;
use clap::Parser;

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
            let servers = electro_rs::fetch_servers()?;
            let selected_server = match server {
                Some(server_name) => servers
                    .into_iter()
                    .find(|server| server.name == server_name)
                    .ok_or_else(|| {
                        anyhow::anyhow!("couldn't find server with name '{server_name}'")
                    })?,
                None => {
                    let servers_len = servers.len();
                    inquire::Select::new("Select electro server", servers)
                        .with_page_size(servers_len)
                        .prompt()?
                }
            };
            electro_rs::connect(&selected_server)?;
        }
        Command::Off => {
            ensure!(is_on, "electro is not on");
            electro_rs::disconnect()?;
        }
        Command::List => {
            let servers = electro_rs::fetch_servers()?
                .into_iter()
                .map(|server| server.name)
                .collect::<Vec<_>>();
            println!("{}", servers.join("\n"));
        }
    }
    Ok(())
}
