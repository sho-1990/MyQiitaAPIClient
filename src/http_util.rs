extern crate hyper;
extern crate hyper_native_tls;
extern crate params;
extern crate toml;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::time::Duration;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper::header::Headers;
use params::Map;
use params::Value;
use self::hyper_native_tls::NativeTlsClient;

header! { (Authorization, "Authorization") => [String] }

lazy_static! {
pub static ref CONFIG: Config = {
        read_config()
    };
}


#[derive(Deserialize)]
#[derive(Debug)]
pub struct Config {
    pub server: Server,
    pub api: Api,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Server {
    pub ip: String,
    pub port: String
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Api {
    pub origin: String,
    pub api_key: String

}


pub fn generate_client() -> Client {
    let tls = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(tls);
    let mut client = Client::with_connector(connector);

    client.set_read_timeout(Some(Duration::new(10, 0)));
    client.set_write_timeout(Some(Duration::new(10, 0)));
    client
}

pub fn safe_find<'a>(map: &'a Map, key: &String) -> Option<&'a String> {
    let some_v = map.0.get(key);
    let v = match some_v {
        Some(value) => {
            match *value {
                Value::String(ref sv) => { Some(sv) },
                _ => { None }
            }
        },
        None => { None }
    };
    v
}

pub fn qiita_header() -> Headers {
    let mut headers = Headers::new();
    // qiita api key
    headers.set(Authorization(format!("{}{}", "Bearer ", CONFIG.api.api_key).to_owned()));
    headers
}

fn read_config() -> Config {
    let path = Path::new("./config.toml");
    let mut f = File::open(&path).expect("plsease set config file");
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    toml::from_str(&s).unwrap()
}
