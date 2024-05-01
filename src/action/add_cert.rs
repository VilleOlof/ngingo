use std::path::PathBuf;

use colored::Colorize;
use dialoguer::Input;

use crate::cert::{load_certs, save_certs, Cert};

pub fn run() {
    let cert: String = Input::new()
        .with_prompt("Enter the full path to the certificate")
        .interact_text()
        .expect("Failed to read certificate path");

    let key: String = Input::new()
        .with_prompt("Enter the full path to the key")
        .interact_text()
        .expect("Failed to read key path");

    let domain: String = Input::new()
        .with_prompt("Enter the domain")
        .interact_text()
        .expect("Failed to read domain");

    let mut certs = load_certs();
    let new_cert = Cert {
        domain,
        cert: PathBuf::from(cert),
        key: PathBuf::from(key),
    };
    certs.push(new_cert);

    save_certs(certs);

    println!("{}", "Certificate added".green().bold());
}
