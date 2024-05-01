use std::{collections::HashMap, fmt::Display, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cert {
    pub domain: String,
    pub cert: PathBuf,
    pub key: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TinyCert {
    pub cert: PathBuf,
    pub key: PathBuf,
}

impl Display for Cert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.domain)
    }
}

type CertMap = HashMap<String, TinyCert>;

impl Cert {
    fn to_toml(certs: Vec<Cert>) -> CertMap {
        let mut map = HashMap::new();
        for cert in certs {
            map.insert(
                cert.domain,
                TinyCert {
                    cert: cert.cert,
                    key: cert.key,
                },
            );
        }
        map
    }

    fn from_toml(map: CertMap) -> Vec<Cert> {
        map.into_iter()
            .map(|(domain, tiny_cert)| Cert {
                domain,
                cert: tiny_cert.cert,
                key: tiny_cert.key,
            })
            .collect()
    }
}

const CERT_DATA: &str = "./certs.toml";
pub fn load_certs() -> Vec<Cert> {
    if !PathBuf::from(CERT_DATA).exists() {
        return vec![];
    }

    let data = fs::read_to_string(CERT_DATA).expect("Failed to read certs.toml");
    let certs: CertMap = toml::from_str(&data).expect("Failed to parse certs.toml");
    Cert::from_toml(certs)
}

pub fn save_certs(certs: Vec<Cert>) {
    let data = toml::to_string(&Cert::to_toml(certs)).expect("Failed to serialize certs.toml");
    fs::write(CERT_DATA, data).expect("Failed to write certs.toml");
}
