use std::{fs, path::PathBuf};

use colored::Colorize;
use dialoguer::{Input, Select};

use crate::{
    cert::{load_certs, Cert},
    get_nginx_dir,
};

#[derive(Debug)]
struct Options {
    config_name: String,
    domains: Vec<String>,
    port: u16,
    cert: Cert,
}

pub fn run() {
    let config_name: String = Input::new()
        .with_prompt(format!("{}", "Configuration Name".bold()))
        .interact_text()
        .expect("Failed to read configuration name");

    let raw_domain_string: String = Input::new()
        .with_prompt(format!(
            "{} {}",
            "Domain".bold(),
            "(space separated)".bright_black().bold()
        ))
        .interact_text()
        .expect("Failed to read domain");
    let domains = raw_domain_string
        .split(" ")
        .map(|f| f.to_string())
        .collect::<Vec<String>>();

    let port: u16 = Input::new()
        .with_prompt(format!("{}", "Local Port".bold()))
        .interact()
        .expect("Failed to read port");
    let cert = get_cert();

    let options = Options {
        config_name,
        domains,
        port,
        cert,
    };

    if PathBuf::from(get_nginx_dir())
        .join("sites-available")
        .join(&options.config_name)
        .with_extension("conf")
        .exists()
    {
        println!("Configuration already exists");
        return;
    }

    create(options);
}

fn get_cert() -> Cert {
    let certs = load_certs();
    let cert_index = Select::new()
        .with_prompt(format!("{}", "Certificate".bold()))
        .items(&certs)
        .interact()
        .unwrap();
    let cert = &certs[cert_index];

    return cert.clone();
}

fn create(opts: Options) {
    let template_file = include_str!("../../template.conf");

    let config = template_file
        .replace("|DOMAIN|", &opts.domains.join(" "))
        .replace("|SSL_CERTIFICATE|", opts.cert.cert.to_str().unwrap())
        .replace("|SSL_CERTIFICATE_KEY|", opts.cert.key.to_str().unwrap())
        .replace("|PORT|", &opts.port.to_string());

    let config_path = PathBuf::from(get_nginx_dir())
        .join("sites-available")
        .join(&opts.config_name)
        .with_extension("conf");

    fs::write(&config_path, config).expect("Failed to write configuration file");

    #[cfg(target_os = "linux")]
    {
        use std::os;

        let symlink_path = PathBuf::from(get_nginx_dir())
            .join("sites-enabled")
            .join(&opts.config_name)
            .with_extension("conf");

        os::unix::fs::symlink(&config_path, &symlink_path).expect("Failed to create symlink");
    }

    println!("\n{}", "Configuration created successfully".green().bold());
}
