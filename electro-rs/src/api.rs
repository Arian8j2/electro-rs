use anyhow::{ensure, Context};
use serde::de::DeserializeOwned;

const ELCDN_SERVERS_API: &str = "https://elcdn.ir/app/servers.json";

pub fn fetch_servers() -> anyhow::Result<Vec<Server>> {
    get_request(ELCDN_SERVERS_API)
}

fn get_request<T: DeserializeOwned>(url: &str) -> anyhow::Result<T> {
    let mut resp = ureq::get(url)
        .call()
        .with_context(|| format!("couldn't fetch '{url}'"))?;
    ensure!(
        resp.status().is_success(),
        "failure response ({})",
        resp.status()
    );
    let resp_body = resp.body_mut().read_to_string()?;
    let json: T = serde_json::from_str(&resp_body).with_context(|| "couldn't deserialize json")?;
    Ok(json)
}

pub type ServerName = String;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Server {
    pub name: ServerName,
    pub config_link: String,
    pub api_link: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct WgPublic {
    pub change_state: u32,
    pub publickey: String,
    pub endpoint: String,
    pub dns: String,
    pub routes: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct WgPrivate {
    pub ip: String,
    pub private_key: String,
    pub psk: String,
}

impl Server {
    pub fn fetch_wireguard_public_info(&self) -> anyhow::Result<WgPublic> {
        get_request(&self.config_link)
    }

    pub fn fetch_wireguard_private_info(&self) -> anyhow::Result<WgPrivate> {
        get_request(&self.api_link)
    }
}
