mod update;
mod utils;

fn main() {
    println!("Cache_dir: {}", utils::paths::cache_dir().display());
    println!("Config_dir: {}", utils::paths::config_dir().display());
}
