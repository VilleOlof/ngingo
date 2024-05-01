use std::{fs, path::PathBuf};

use colored::*;
use dialoguer::Select;

use crate::action::Action;

mod action;
mod cert;

pub static mut NGINX_DIR: &str = "/etc/nginx";

pub fn get_nginx_dir() -> &'static str {
    unsafe { NGINX_DIR }
}

pub fn get_sites_enabled() -> Vec<String> {
    let sites_enabled = PathBuf::from(get_nginx_dir()).join("sites-enabled");
    let mut sites = vec![];
    for entry in fs::read_dir(sites_enabled).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        sites.push(file_name);
    }
    sites
}

pub fn get_sites_available() -> Vec<String> {
    let sites_available = PathBuf::from(get_nginx_dir()).join("sites-available");
    let mut sites = vec![];
    for entry in fs::read_dir(sites_available).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        sites.push(file_name);
    }
    sites
}

pub fn restart_nginx() {
    #[cfg(target_os = "linux")]
    {
        let output = std::process::Command::new("sudo")
            .arg("systemctl")
            .arg("restart")
            .arg("nginx")
            .output()
            .expect("Failed to restart nginx");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn main() {
    #[cfg(debug_assertions)]
    {
        #[cfg(target_os = "windows")]
        {
            println!("Changed nginx config dir ./test-nginx");
            unsafe { NGINX_DIR = "./test-nginx" };
        }

        #[cfg(target_os = "linux")]
        {
            println!("Changed nginx config dir /tmp/test-nginx");
            unsafe { NGINX_DIR = "/tmp/test-nginx" };
        }
    }

    println!(
        "{}\n{}\n",
        "Ngingo".green().bold().underline(),
        "Manage Nginx Proxy Configurations".bright_black()
    );

    let action: Action = Select::new()
        .with_prompt("Action")
        .items(&[
            "Create",
            "Remove",
            "Enable",
            "Disable",
            "Add Certificate",
            "Remove Certificate",
        ])
        .default(0)
        .interact()
        .unwrap()
        .into();

    action.run();

    // Restart nginx if not adding or removing a certificate
    // Adding or removing a certificate does not require a restart
    if action != Action::AddCert || action != Action::RemoveCert {
        restart_nginx();
    }
}
