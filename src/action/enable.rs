use colored::Colorize;
use dialoguer::Select;

use crate::{get_sites_available, get_sites_enabled};

pub fn run() {
    let sites_enabled = get_sites_enabled();
    let sites_available = get_sites_available();

    let disabled_sites = sites_available
        .iter()
        .filter(|site| !sites_enabled.contains(site))
        .map(|site| site.to_string())
        .collect::<Vec<String>>();

    if disabled_sites.is_empty() {
        println!("{}", "No sites to enable".yellow());
        return;
    }

    let selected_site = Select::new()
        .with_prompt("Select a site to enable")
        .items(&disabled_sites)
        .default(0)
        .interact()
        .unwrap();
    let site = &disabled_sites[selected_site];

    #[cfg(target_os = "linux")]
    {
        use crate::get_nginx_dir;
        use std::{os, path::PathBuf};

        let sites_available = PathBuf::from(get_nginx_dir())
            .join("sites-available")
            .join(site);
        let sites_enabled = PathBuf::from(get_nginx_dir())
            .join("sites-enabled")
            .join(site);
        os::unix::fs::symlink(sites_available, sites_enabled).expect("Failed to create symlink");
    }

    println!("{} {}", "Enabled".green(), site.to_string().green().bold());
}
