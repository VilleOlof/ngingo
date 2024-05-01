use std::{fs, path::PathBuf};

use colored::Colorize;
use dialoguer::Select;

use crate::{get_nginx_dir, get_sites_available};

pub fn run() {
    let sites = get_sites_available();

    let selected_site = Select::new()
        .with_prompt("Select a site to remove")
        .items(&sites)
        .default(0)
        .interact()
        .unwrap();
    let site = &sites[selected_site];

    let sites_available = PathBuf::from(get_nginx_dir()).join("sites-available");
    let sites_enabled = PathBuf::from(get_nginx_dir()).join("sites-enabled");

    let site_available = sites_available.join(site);
    let site_enabled = sites_enabled.join(site);

    fs::remove_file(site_available).unwrap();
    fs::remove_file(site_enabled).unwrap();

    println!("{}", "Site removed".red().bold());
}
