use std::{fs, path::PathBuf};

use colored::Colorize;
use dialoguer::Select;

use crate::{get_nginx_dir, get_sites_available, get_sites_enabled};

pub fn run() {
    let sites_enabled = get_sites_enabled();
    let sites_available = get_sites_available();

    let enabled_sites = sites_available
        .iter()
        .filter(|site| sites_enabled.contains(site))
        .map(|site| site.to_string())
        .collect::<Vec<String>>();

    let selected_site = Select::new()
        .with_prompt("Select a site to disable")
        .items(&enabled_sites)
        .default(0)
        .interact()
        .unwrap();
    let site = &sites_enabled[selected_site];

    let sites_enabled = PathBuf::from(get_nginx_dir())
        .join("sites-enabled")
        .join(site);
    fs::remove_file(sites_enabled).unwrap();

    println!("{} {}", "Disabled".red(), site.to_string().red().bold());
}
