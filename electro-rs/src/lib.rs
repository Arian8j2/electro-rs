mod api;
mod cache;
mod utils;
mod wireguard;

use anyhow::{ensure, Context};
pub use api::{fetch_servers, Server};
use api::{WgPrivate, WgPublic};
use cache::{Cache, CachedWgPrivate};

// name should match the config path
const WG_INTERFACE_NAME: &str = "electro-rs";

// path must end with .conf
const CONFIG_PATH: &str = "/tmp/electro-rs.conf";

pub fn connect(server: &Server) -> anyhow::Result<()> {
    ensure!(!is_on()?, "electro is already on");

    let mut wg_public = server.fetch_wireguard_public_info()?;
    let wg_private = get_wireguard_private_info(server, &wg_public)?;

    // electro has large allowed_ips routes, we try to optimize it to make
    // it fewer routes because `wg-quick` will run `ip route` command for each route
    wg_public.routes = utils::optimize_routes(&wg_public.routes)
        .with_context(|| "couldn't optimize allowed ips routes")?;

    let config = generate_config(wg_public, wg_private);
    std::fs::write(CONFIG_PATH, config)
        .with_context(|| format!("couldn't write config to '{CONFIG_PATH}'"))?;
    wireguard::connect(CONFIG_PATH)
}

pub fn disconnect() -> anyhow::Result<()> {
    ensure!(is_on()?, "electro is not currently on");
    wireguard::disconnect(CONFIG_PATH)
}

pub fn is_on() -> anyhow::Result<bool> {
    wireguard::is_interface_up(WG_INTERFACE_NAME)
}

fn get_wireguard_private_info(server: &Server, wg_public: &WgPublic) -> anyhow::Result<WgPrivate> {
    let mut cache = Cache::load().with_context(|| "couldn't load cache")?;
    if let Some(cached_wg_privatge) = cache.get(&server.name) {
        if cached_wg_privatge.state == wg_public.change_state {
            return Ok(cached_wg_privatge.wg_private.clone());
        }
    }

    let wg_private = server
        .fetch_wireguard_private_info()
        .with_context(|| "couldn't fetch new wireguard private info")?;
    let cached_wg_private = CachedWgPrivate {
        wg_private: wg_private.clone(),
        state: wg_public.change_state,
    };
    cache.insert(server.name.clone(), cached_wg_private);
    cache.save().with_context(|| "couldn't save cache")?;
    Ok(wg_private)
}

fn generate_config(wg_public: WgPublic, wg_private: WgPrivate) -> String {
    format!(
        "[Interface]\nPrivateKey = {}\nAddress = {}\n\n[Peer]\nPublicKey = {}\nPresharedKey = {}\nAllowedIPs = {}\nEndpoint = {}",
        wg_private.private_key,
        wg_private.ip,
        wg_public.publickey,
        wg_private.psk,
        wg_public.endpoint,
        wg_public.routes
    )
}
