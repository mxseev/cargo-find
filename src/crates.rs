use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json;
use std::io::Read;

use error::Error;


pub fn find_crates(query: String) -> Result<Vec<Krate>, Error> {
    let ssl = NativeTlsClient::new()?;
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    let url = format!("https://crates.io/api/v1/crates?q={}", query);
    let mut resp = client.get(&url).send()?;
    let mut buffer = String::new();
    resp.read_to_string(&mut buffer).unwrap();

    let resp: CratesIoResponse = serde_json::from_str(&buffer)?;
    Ok(resp.crates)
}

#[derive(Deserialize)]
struct CratesIoResponse {
    crates: Vec<Krate>,
}
#[derive(Deserialize)]
pub struct Krate {
    pub name: String,
    pub max_version: String,
    pub description: String,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
