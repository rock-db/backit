use std::{fs, path::PathBuf};

pub fn config_dir() -> PathBuf {
    let config_dir = dirs_next::config_dir().unwrap().join("backit");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    }

    config_dir
}

pub fn update_config_dir() -> PathBuf {
    let config_dir = config_dir();

    let update_config_dir = config_dir.join("updates");

    if !update_config_dir.exists() {
        fs::create_dir_all(&update_config_dir).expect("Failed to create config directory");
    }

    update_config_dir
}

pub fn updates_dir() -> PathBuf {
    let config_dir = config_dir();

    let updates_dir = config_dir.join("updates_registry");

    if !updates_dir.exists() {
        fs::create_dir_all(&updates_dir).expect("Failed to create config directory");
    }

    updates_dir
}

pub fn packages_dir() -> PathBuf {
    let config_dir = config_dir();

    let packages_dir = config_dir.join("packages");

    if !packages_dir.exists() {
        fs::create_dir_all(&packages_dir).expect("Failed to create config directory");
    }

    packages_dir
}

pub fn cache_dir() -> PathBuf {
    let config_dir = config_dir();

    let cache_dir = config_dir.join("cache");

    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).expect("Failed to create config directory");
    }

    cache_dir
}
