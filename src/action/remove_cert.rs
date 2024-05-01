use colored::Colorize;
use dialoguer::Select;

use crate::cert::{load_certs, save_certs};

pub fn run() {
    let mut certs = load_certs();

    let selected_cert = Select::new()
        .with_prompt("Select a certificate to remove")
        .items(&certs)
        .default(0)
        .interact()
        .unwrap();
    let cert = &certs[selected_cert].clone();

    certs.remove(selected_cert);

    save_certs(certs);

    println!(
        "{} [{}]",
        "Certificate removed".red().bold(),
        cert.domain.red()
    );
}
